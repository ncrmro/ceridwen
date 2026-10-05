import * as THREE from 'three';
import {OrbitControls} from 'three/addons/controls/OrbitControls.js';
import {defaultParameters,parameterLabels,validateParameters,type Parameters,type ParameterGroup} from './parameters.ts';
import type {buildDesign} from './build-design.ts';
import './style.css';
type Result=Pick<ReturnType<typeof buildDesign>,'assembly'|'meshes'|'report'>;
let parameters:Parameters=structuredClone(defaultParameters),result:Result|undefined,worker:Worker|undefined,buildTimer:ReturnType<typeof setTimeout>|undefined;
const el=<T extends HTMLElement>(id:string)=>document.getElementById(id) as T;
const viewport=el('viewport'),scene=new THREE.Scene();scene.background=new THREE.Color('#101a22');
const camera=new THREE.PerspectiveCamera(40,1,.1,2000);camera.up.set(0,0,1);
const renderer=new THREE.WebGLRenderer({antialias:true});renderer.setPixelRatio(Math.min(devicePixelRatio,2));viewport.append(renderer.domElement);
const controls=new OrbitControls(camera,renderer.domElement);controls.enableDamping=true;
scene.add(new THREE.HemisphereLight(0xe1f6ff,0x425368,2.5));const light=new THREE.DirectionalLight(0xffffff,3);light.position.set(40,20,150);scene.add(light);
const grid=new THREE.GridHelper(200,20,0x476271,0x263e49);grid.rotateX(Math.PI/2);grid.position.set(40,25,-3);scene.add(grid);
const group=new THREE.Group();scene.add(group);
new ResizeObserver(()=>{camera.aspect=viewport.clientWidth/viewport.clientHeight;camera.updateProjectionMatrix();renderer.setSize(viewport.clientWidth,viewport.clientHeight);}).observe(viewport);
renderer.setAnimationLoop(()=>{controls.update();renderer.render(scene,camera);});
function resetView(){
 const bounds=new THREE.Box3().setFromObject(group),center=bounds.isEmpty()?new THREE.Vector3(40,25,15):bounds.getCenter(new THREE.Vector3());
 const radius=bounds.isEmpty()?60:bounds.getSize(new THREE.Vector3()).length()/2;
 const distance=radius/Math.sin(THREE.MathUtils.degToRad(20))*1.05/Math.min(1,camera.aspect);
 camera.position.copy(center).add(new THREE.Vector3(1.1,1.35,1.05).normalize().multiplyScalar(distance));controls.target.copy(center);controls.update();
}
resetView();
function redraw(){
 for(const child of [...group.children]){const mesh=child as THREE.Mesh;mesh.geometry.dispose();(mesh.material as THREE.Material).dispose();group.remove(mesh);}
 if(!result)return;
 for(const p of result.meshes){
  if(p.category==='printed'&&p.id.startsWith('CASE')&&!el<HTMLInputElement>('case').checked)continue;
  if(p.category==='wire'&&!el<HTMLInputElement>('wires').checked)continue;
  if(p.category==='fastener'&&!el<HTMLInputElement>('fasteners').checked)continue;
  const geometry=new THREE.BufferGeometry();geometry.setAttribute('position',new THREE.Float32BufferAttribute(p.vertices,3));geometry.setAttribute('normal',new THREE.Float32BufferAttribute(p.normals,3));geometry.setIndex(p.triangles);
  const transparent=p.id.startsWith('CASE')&&el<HTMLInputElement>('xray').checked;
  const mesh=new THREE.Mesh(geometry,new THREE.MeshStandardMaterial({color:p.color,roughness:.65,metalness:.05,transparent,opacity:transparent?.18:1,depthWrite:!transparent,side:THREE.DoubleSide}));
  if(el<HTMLInputElement>('explode').checked)mesh.position.z=p.lift;
  group.add(mesh);
 }
}
function status(text:string){el('status').textContent=text;}
function report(){
 if(!result)return;const r=result.report;el('report').replaceChildren();
 for(const text of [`Digital assembly: ${r.interferencePass&&r.footprintPass&&r.motionPass&&r.headerGridPass?'PASS':'CHECK FIT'}`,`Button travel: ${r.motionPass?'PASS':'CHECK FIT'}`,`Header grid: ${r.headerGridPass?'PASS':'CHECK ALIGNMENT'}`,`Breadboard footprint: ${r.footprintPass?'PASS':'FAIL'}`,`Case: ${r.bounds.max.map((v,i)=>(v-r.bounds.min[i]).toFixed(1)).join(' × ')} mm`,`${r.printedCount} printed parts · ${r.pushPinCount} printed push-pins · ${r.wireCount} routed wires`,...r.intersections.map(v=>`${v.a} / ${v.b}: ${v.volume.toFixed(2)} mm³ interference`)]){const p=document.createElement('p');p.textContent=text;el('report').append(p);}
 el('blockers').replaceChildren(...r.blockers.map(text=>{const li=document.createElement('li');li.textContent=text;return li;}));
 const select=el<HTMLSelectElement>('print-part');select.replaceChildren(...[...new Map(result.meshes.filter(m=>m.category==='printed'||m.category==='fastener').map(m=>[m.id,m])).values()].map(m=>{const option=document.createElement('option');option.value=m.id;option.textContent=m.name;return option;}));
}
function download(name:string,blob:Blob){const a=document.createElement('a'),url=URL.createObjectURL(blob);a.href=url;a.download=name;a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}
function rebuild(){
 clearTimeout(buildTimer);worker?.terminate();el('report').replaceChildren();status('Rebuilding components, mounts, case and wiring…');el('error').textContent='';
 for(const id of ['stl','step'])el<HTMLButtonElement>(id).disabled=true;
 worker=new Worker(new URL('./design.worker.ts',import.meta.url),{type:'module'});
 const active=worker;
 worker.onmessage=event=>{
  if(worker!==active)return;
  if(event.data.type==='built'){
   result=event.data.result;redraw();report();resetView();status('Current dimensions applied · design assumptions remain editable');
   for(const id of ['stl','step'])el<HTMLButtonElement>(id).disabled=false;
  }else if(event.data.type==='download')download(event.data.name,event.data.blob);
  else{el('error').textContent=event.data.error;status('Could not build these dimensions; adjust the highlighted assumptions.');}
 };
 worker.onerror=event=>{el('error').textContent=event.message;status('CAD worker failed');};
 worker.postMessage({type:'build',parameters});
}
function editor(){
 const key=el<HTMLSelectElement>('part').value as ParameterGroup,values=parameters[key] as Record<string,number>;
 el('fields').replaceChildren();el('source').textContent='Millimetres unless named as a count. These are editable component-level dimensions.';
 for(const [name,value] of Object.entries(values)){
  const label=document.createElement('label');label.className='parameter';const span=document.createElement('span');span.textContent=name.replace(/([A-Z])/g,' $1');
  const input=document.createElement('input');input.type='number';input.step=name==='rows'||name==='pins'?'1':'.1';input.value=String(value);input.setAttribute('aria-label',`${key}.${name}`);
  input.addEventListener('change',()=>{
   const old=values[name];values[name]=input.valueAsNumber;
   try{validateParameters(parameters);el('error').textContent='';status('Dimensions changed; preparing rebuild…');clearTimeout(buildTimer);buildTimer=setTimeout(rebuild,450);}
   catch(error){values[name]=old;input.value=String(old);el('error').textContent=String(error);}
  });label.append(span,input);el('fields').append(label);
 }
}
const select=el<HTMLSelectElement>('part');for(const [key,label]of Object.entries(parameterLabels)){const o=document.createElement('option');o.value=key;o.textContent=label;select.append(o);}editor();
select.addEventListener('change',editor);
for(const id of ['case','xray','wires','fasteners'])el(id).addEventListener('change',redraw);
el('explode').addEventListener('change',()=>{redraw();resetView();});el('reset-view').addEventListener('click',resetView);
el('reset').addEventListener('click',()=>{parameters=structuredClone(defaultParameters);editor();rebuild();});
el('save').addEventListener('click',()=>download('ceridwen-parameters.json',new Blob([JSON.stringify({schema:2,parameters},null,2)],{type:'application/json'})));
el<HTMLInputElement>('load').addEventListener('change',async event=>{const file=(event.target as HTMLInputElement).files?.[0];if(!file)return;try{const data=JSON.parse(await file.text()),p=data.parameters??data;validateParameters(p);parameters=p;editor();rebuild();}catch(error){el('error').textContent=String(error);}});
el('stl').addEventListener('click',()=>worker?.postMessage({type:'stl',id:el<HTMLSelectElement>('print-part').value}));el('step').addEventListener('click',()=>worker?.postMessage({type:'step'}));
rebuild();

// A matching generated snapshot gives an immediate first view while the worker initializes.
fetch('/generated/case-preview.json').then(r=>r.json()).then((saved:Result)=>{
 if(!result&&JSON.stringify(saved.assembly.parameters)===JSON.stringify(parameters)){result=saved;redraw();resetView();}
}).catch(()=>{/* live generation is authoritative */});
