import type {OpenCascadeInstance} from 'replicad-opencascadejs';
import {makeCompound,measureVolume,measureDistanceBetween,type Shape3D} from 'replicad';
import {derive} from './layout.ts';
import {buildEnclosure} from './enclosure.ts';
import {component} from './models/index.ts';
import {physical,type Piece} from './models/common.ts';
import * as fastener from './models/fastener.ts';
import {harness} from './harness.ts';
import type {Parameters} from './parameters.ts';
import type {Vec3} from './assembly.ts';
import {fitReport} from './fit.ts';
export interface Entity {id:string;name:string;category:'component'|'printed'|'fastener'|'wire';shape:Shape3D;pieces:Piece[];lift:number;owner?:string;net?:string;terminals?:string[]}
export function buildDesign(parameters:Parameters,oc:OpenCascadeInstance){
 const l=derive(parameters),caseModel=buildEnclosure(l),entities:Entity[]=[];
 const add=(e:Entity)=>entities.push(e);
 for(const part of l.assembly.parts.filter(p=>p.kind==='component'&&!p.id.startsWith('CAP'))){
  const m=component(parameters,part);for(const piece of m.pieces)piece.shape=piece.shape.translate(part.position);
  add({id:part.id,name:part.name,category:'component',shape:physical(m),pieces:m.pieces,lift:part.id==='BB1'?0:part.id==='U1'?10:part.id==='DISP1'||part.id.startsWith('SW')?35:20});
 }
 for(const part of caseModel.parts)add({id:part.id,name:part.name,category:'printed',shape:part.shape,pieces:[{name:part.name,shape:part.shape,color:part.color}],lift:part.lift});
 for(const screw of caseModel.screws){
  const model=fastener.build(screw.length);const r=fastener.defaults.headDiameter/2;
  for(const piece of model.pieces){if(screw.up)piece.shape=piece.shape.rotate(180,[r,r,screw.length],[1,0,0]);piece.shape=piece.shape.translate([screw.center[0]-r,screw.center[1]-r,screw.headZ-screw.length]);}
  add({id:screw.id,name:`M2 × ${screw.length} ${screw.owner}`,category:'fastener',shape:physical(model),pieces:model.pieces,owner:screw.owner,lift:screw.headZ>l.deck?40:8});
 }
 const routes=harness(l);
 for(const w of routes)add({id:w.id,name:w.name,category:'wire',shape:w.shape,pieces:[{name:w.name,shape:w.shape,color:w.color}],lift:20,net:w.net,terminals:w.terminals});
 const topology=[];
 for(const e of entities.filter(e=>e.category==='printed')){
  const analyzer=new oc.BRepCheck_Analyzer(e.shape.wrapped,true,false),valid=analyzer.IsValid_2();analyzer.delete();
  const ex=new oc.TopExp_Explorer_2(e.shape.wrapped,oc.TopAbs_ShapeEnum.TopAbs_SOLID as unknown as import('replicad-opencascadejs').TopAbs_ShapeEnum,oc.TopAbs_ShapeEnum.TopAbs_SHAPE as unknown as import('replicad-opencascadejs').TopAbs_ShapeEnum);let count=0;while(ex.More()){count++;ex.Next();}ex.delete();
  topology.push({id:e.id,valid,solidCount:count,volume:measureVolume(e.shape)});
  if(!valid||count!==1)throw new Error(`${e.id}: expected one valid connected solid, got ${count}`);
 }
 const violations:{a:string;b:string;volume:number}[]=[],contacts:{a:string;b:string;distance:number}[]=[];
 const bounds=new Map(entities.map(e=>[e.id,e.shape.boundingBox.bounds]));
 const permitted:{a:string;b:string;reason:string;volume:number}[]=[];
 for(let i=0;i<entities.length;i++)for(const b of entities.slice(i+1)){
  const a=entities[i],aa=bounds.get(a.id)!,bb=bounds.get(b.id)!;
  const gaps=[0,1,2].map(j=>Math.max(bb[0][j]-aa[1][j],aa[0][j]-bb[1][j],0));
  if(Math.hypot(...gaps)>.5)continue;
  const intersection=a.shape.intersect(b.shape),volume=Math.abs(measureVolume(intersection as Shape3D));intersection.delete();
  const pair=[a.id,b.id];
  let reason='';
  if(pair.includes('BB1')&&pair.includes('U1')&&volume<=parameters.esp32.pins*2*.64*.64*3+.01)reason='Header pins inserted 3 mm into breadboard sockets';
  if(a.category==='fastener'&&a.owner===b.id||b.category==='fastener'&&b.owner===a.id)reason='M2 thread engagement in named pilot bore';
  if(a.category==='wire'&&b.category==='wire'&&a.net===b.net)reason='Common electrical net junction';
  if(a.category==='wire'&&a.terminals?.includes(b.id)||b.category==='wire'&&b.terminals?.includes(a.id))reason='Named wire termination';
  if(volume>1e-4){if(reason)permitted.push({a:a.id,b:b.id,reason,volume});else violations.push({a:a.id,b:b.id,volume});}
  else contacts.push({a:a.id,b:b.id,distance:measureDistanceBetween(a.shape,b.shape)});
 }
 const motionChecks=[];
 for(const id of ['CAP1','CAP2']){
  const cap=entities.find(e=>e.id===id)!,travel=parameters.cap.contactGap+parameters.button.travel;
  const sweep=cap.shape.fuse(cap.shape.clone().translateZ(-travel)).fuse(cap.shape.clone().translateZ(parameters.cap.returnGap));
  const hits=[];
  for(const e of entities.filter(e=>e.category==='printed'&&e.id!==id)){
   const intersection=sweep.intersect(e.shape),volume=Math.abs(measureVolume(intersection as Shape3D));intersection.delete();
   if(volume>1e-4)hits.push({id:e.id,volume});
  }
  motionChecks.push({id,downTravel:travel,upTravel:parameters.cap.returnGap,actuatorTravel:parameters.button.travel,printedPartIntersections:hits});sweep.delete();
 }
 const componentFootprint=entities.filter(e=>e.category==='component').flatMap(e=>{
  const b=bounds.get(e.id)!;return b[0][0]<-.001||b[0][1]<-.001||b[1][0]>parameters.breadboard.length+.001||b[1][1]>parameters.breadboard.width+.001?[e.id]:[];
 });
 const board=parameters.breadboard,mcu=parameters.esp32,u=l.assembly.parts.find(p=>p.id==='U1')!;
 const gridPoints:Vec3[]=[];
 for(let row=0;row<board.rows;row++)for(const bank of [-1,1])for(let column=0;column<5;column++)gridPoints.push([(board.length-(board.rows-1)*board.pitch)/2+row*board.pitch,board.width/2+bank*(board.channel/2+column*board.pitch),board.height]);
 let headerGridError=0;
 for(let n=0;n<mcu.pins;n++)for(const bank of [-1,1]){
  const x=u.position[0]+(mcu.length-(mcu.pins-1)*mcu.pitch)/2+n*mcu.pitch,y=u.position[1]+(mcu.width+bank*mcu.rowSpacing)/2;
  headerGridError=Math.max(headerGridError,Math.min(...gridPoints.map(p=>Math.hypot(p[0]-x,p[1]-y))));
 }
 const envelope=fitReport(l.assembly);
 const wireFootprint=entities.filter(e=>e.category==='wire').flatMap(e=>{
  const b=bounds.get(e.id)!;return b[0][0]<-.001||b[0][1]<-.001||b[1][0]>parameters.breadboard.length+.001||b[1][1]>parameters.breadboard.width+.001?[e.id]:[];
 });
 const report={headerGridPass:headerGridError<.1,headerGridError,status:caseModel.status,bounds:caseModel.bounds,topology,intersections:violations,contacts,permittedContacts:permitted,
  interferencePass:violations.length===0,footprintPass:envelope.footprint.pass&&wireFootprint.length===0&&componentFootprint.length===0,wireFootprintViolations:wireFootprint,componentFootprintViolations:componentFootprint,
  motionChecks,motionPass:motionChecks.every(c=>c.printedPartIntersections.length===0),
  componentCount:l.assembly.parts.filter(p=>p.kind==='component').length,printedCount:topology.length,screwCount:caseModel.screws.length,wireCount:entities.filter(e=>e.category==='wire').length,
  minimumClearanceAccepted:false,manufacturingReady:false,blockers:caseModel.blockers};
 const meshes=entities.flatMap(e=>e.pieces.map(piece=>({id:e.id,name:piece.name,category:e.category,color:piece.color,lift:e.lift,...piece.shape.mesh({tolerance:.12,angularTolerance:.2})})));
 return {layout:l,entities,report,meshes,assembly:l.assembly,fit:envelope,fasteners:caseModel.screws,routes};
}
