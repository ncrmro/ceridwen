import init from 'replicad-opencascadejs/src/replicad_single.js';
import wasmUrl from 'replicad-opencascadejs/src/replicad_single.wasm?url';
import {setOC} from 'replicad';
import {assemblySTEP} from './step.ts';
import type {OpenCascadeInstance} from 'replicad-opencascadejs';
import {physical} from './models/common.ts';
import * as pushPin from './models/push-pin.ts';
import {printSolid} from './print.ts';
import {buildDesign} from './build-design.ts';
import type {Parameters} from './parameters.ts';
let design:ReturnType<typeof buildDesign>|undefined;
const kernel=(init as unknown as (options:Record<string,unknown>)=>Promise<OpenCascadeInstance>)({locateFile:()=>wasmUrl}).then(oc=>{setOC(oc);return oc;});
self.onmessage=async(event:MessageEvent<{type:string;parameters?:Parameters;id?:string}>)=>{
 try{
  if(event.data.type==='build'){
   design=buildDesign(event.data.parameters!,await kernel);
   self.postMessage({type:'built',result:{assembly:design.assembly,meshes:design.meshes,report:design.report}});
  }else if(event.data.type==='stl'&&design){
   const entity=design.entities.find(e=>(e.category==='printed'||e.category==='fastener')&&e.id===event.data.id);
   if(!entity)throw new Error('Unknown printed part');
   const clip=design.pushPins.find(p=>p.id===entity.id);
   const printable=clip?printSolid('PUSH_PIN',physical(pushPin.build(design.layout.params.pushFit,clip.length))):printSolid(entity.id,entity.shape);
   self.postMessage({type:'download',name:entity.id+'.stl',blob:printable.blobSTL()});
  }else if(event.data.type==='step'&&design){
   self.postMessage({type:'download',name:'ceridwen-assembled.step',blob:assemblySTEP(design.entities.map(e=>e.shape))});
  }
 }catch(error){self.postMessage({type:'error',error:String(error)});}
};
