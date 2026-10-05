import {makeCylinder,makeSphere,type Shape3D} from 'replicad';
import {anchors} from './models/esp32.ts';
import {compound} from './models/common.ts';
import type {Layout} from './layout.ts';
import type {Vec3} from './assembly.ts';
export interface HarnessWire {id:string;net:string;name:string;color:string;points:Vec3[];shape:Shape3D;terminals:string[]}
export function harness(l:Layout):HarnessWire[]{
 const p=l.params,m=p.esp32;const get=(id:string)=>l.assembly.parts.find(q=>q.id===id)!;
 const u=get('U1'),d=get('DISP1');const wires:HarnessWire[]=[];
 const ports=anchors(m),port=(name:string):Vec3=>ports[name].map((v,i)=>v+u.position[i]) as Vec3;
 const vcc=port('vcc'),ground=port('gnd');
 const add=(id:string,net:string,name:string,color:string,points:Vec3[],terminals:string[])=>{
  const solids:Shape3D[]=[];for(let i=1;i<points.length;i++){
   const a=points[i-1],b=points[i],delta=b.map((v,j)=>v-a[j]) as Vec3,len=Math.hypot(...delta);
   if(len>1e-6)solids.push(makeCylinder(.3,len,a,delta));
  }
  for(const point of points)solids.push(makeSphere(.3).translate(point));
  wires.push({id,net,name,color,points,terminals,shape:compound(solids)});
 };
 const starts=[vcc,ground,port('sda'),port('scl')];
 const net=['3V3','GND','SDA','SCL'],colors=['#ee635f','#303a47','#69a7eb','#f0c650'];
 for(let i=0;i<4;i++){
  const a=starts[i],z=l.deck-3.2+i*.8,y=15-i*1.2;
  const end:Vec3=[d.position[0]+p.oled.width/2-3.81+i*2.54,d.position[1]+p.oled.depth-3,d.position[2]];
  add('W0'+(i+1),net[i],`OLED ${net[i]}`,colors[i],[a,[a[0],a[1],z],[a[0],y,z],[end[0],y,z],[end[0],end[1],z],end],['U1','DISP1']);
 }
 for(let i=0;i<2;i++){
  const sw=get('SW'+(i+1)),cx=sw.position[0]+sw.size[0]/2,cy=sw.position[1]+sw.size[1]/2;
  for(let j=0;j<2;j++){
   const a=j===0?port(i===0?'left':'right'):ground;
   const z=l.deck-(i===0?(j===0?2.2:3):(j===0?.6:1.4)),y=p.breadboard.width-(j===0?15.5:14.5);
   const end:Vec3=[cx+(j===0?-1:1)*p.button.leadSpacing/2,cy,sw.position[2]];
   add('W0'+(5+i*2+j),j===0?'GPIO'+i:'GND',`Button ${i+1} ${j===0?'input':'ground'}`,j===0?(i===0?'#69c38c':'#bf8ce7'):'#303a47',[a,[a[0],a[1],z],[a[0],y,z],[end[0],y,z],[end[0],end[1],z],end],['U1',sw.id]);
  }
 }
 // Battery pigtail is a mechanical route only; power-board electrical selection is outstanding.
 const bat=get('BAT1'),j=get('J1');
 for(let i=0;i<2;i++){
  const offset=i===0?-.5:.5;
  const a:Vec3=[bat.position[0]+bat.size[0]/2+offset,bat.position[1]+bat.size[1],bat.position[2]+bat.size[2]/2];
  const e:Vec3=[j.position[0]+j.size[0]/2+offset,j.position[1],j.position[2]+j.size[2]/2];
  add('BAT_W'+i,'BAT'+i,'Battery connector lead',i===0?'#ee635f':'#303a47',[a,[a[0],a[1]+p.battery.leadSpace+1+i,a[2]],[e[0],a[1]+p.battery.leadSpace+1+i,a[2]],[e[0],e[1],a[2]],e],['BAT1','J1']);
 }
 return wires;
}
