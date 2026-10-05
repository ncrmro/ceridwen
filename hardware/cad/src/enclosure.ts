import {type Shape3D} from 'replicad';
import {box} from './models/common.ts';
import * as capModel from './models/cap.ts';
import type {Assembly,Part,Vec3} from './assembly.ts';
import {derive,type Layout} from './layout.ts';
import {defaultParameters} from './parameters.ts';
export interface PrintedPart {id:string;name:string;color:string;shape:Shape3D;lift:number}
export const envelopeSolid=(p:Part)=>box(p.position,p.size);
export function capSolid(p:Part){return capModel.build(defaultParameters.cap).pieces[0].shape.translate(p.position);}
export interface SnapFeature {id:string;owner:string;retains:string;engagement:number;clearance:number;release:string}
export function enclosure(assembly:Assembly){return buildEnclosure(derive(assembly.parameters??defaultParameters));}
export function buildEnclosure(l:Layout){
 const {params:p}=l,b=p.breadboard,c=p.case,s=p.snap;const get=(id:string)=>l.assembly.parts.find(q=>q.id===id)!;
 const x0=-c.gap-c.wall,x1=b.length+c.gap+c.wall,y0=-c.gap-c.wall,y1=b.width+c.gap+c.wall;
 const seam=l.carrier+c.carrierThickness;
 const rounded=(min:Vec3,size:Vec3)=>box(min,size).fillet(c.cornerRadius,e=>e.inDirection('Z'));
 let base=rounded([x0,y0,-c.floor],[x1-x0,y1-y0,seam+c.floor])
  .cut(box([-c.gap,-c.gap,0],[b.length+2*c.gap,b.width+2*c.gap,seam+1]));
 let carrier=box([-.5,-.5,l.carrier],[b.length+1,b.width+1,c.carrierThickness]);
 let cover=rounded([x0,y0,seam],[x1-x0,y1-y0,l.lid+c.roof-seam])
  .cut(box([-c.gap,-c.gap,seam-1],[b.length+2*c.gap,b.width+2*c.gap,l.lid-seam+1]));
 const snaps:SnapFeature[]=[];
 const record=(id:string,owner:string,retains:string,release:string)=>snaps.push({id,owner,retains,engagement:s.engagement,clearance:s.clearance,release});
 const m=get('U1'),o=get('DISP1'),bat=get('BAT1'),power=get('PWR2'),usb=get('USB1');
 carrier=carrier.cut(box([m.position[0]-.5,m.position[1]-.5,l.carrier-1],[m.size[0]+1,m.size[1]+1,c.carrierThickness+2]));
 const usbCut=box([x0-1,usb.position[1]-.5,usb.position[2]-.5],[m.position[0]-x0+2,usb.size[1]+1,usb.size[2]+1]);
 base=base.cut(usbCut);carrier=carrier.cut(usbCut);cover=cover.cut(usbCut);
 for(const [x,y,w,d] of [[m.position[0]+m.size[0]+1,4,7,11],[m.position[0]+m.size[0]+1,b.width-21,7,10]])carrier=carrier.cut(box([x,y,l.carrier-1],[w,d,c.carrierThickness+2]));
 // Narrow ledges sit outside the board outline, so the breadboard drops straight in.
 // The lid captures the tray; it needs no clips or separate fasteners of its own.
 for(const x of [10,b.length-16])for(const rear of [false,true]){
  const y=rear?b.width:-c.gap;
  base=base.fuse(box([x,y,l.carrier-1.5],[6,c.gap,1.5]));
  const stopY=rear?b.width-1:-c.gap;
  cover=cover.fuse(box([x,stopY,seam],[6,c.gap+1,2]));
 }
 for(const [x,y] of [[8,1],[b.length-10,1],[8,b.width-3],[b.length-10,b.width-3]])carrier=carrier.fuse(box([x,y,b.height],[2,2,l.carrier-b.height]));
 // Four integral downward cantilevers latch into side windows in the base.
 // The inset arms have relief behind them; push inward through a window to release.
 for(const right of [false,true])for(const y of [8,b.width-12]){
  const inner=right?b.length+c.gap:-c.gap,dir=right?1:-1;
  const armX=right?inner+s.clearance:inner-s.clearance-s.armThickness;
  const z=seam-s.armLength,arm=box([armX,y,z],[s.armThickness,s.width,s.armLength+.3]);
  const toothX=right?armX:armX-s.engagement;
  const tooth=box([toothX,y,z],[s.armThickness+s.engagement,s.width,s.hookHeight]).chamfer(.25,e=>e.inPlane('XY',z));
  cover=cover.fuse(arm).fuse(tooth);
  const reliefX=right?inner-.05:armX-s.clearance;
  base=base.cut(box([reliefX,y-s.clearance,z-s.clearance],[s.armThickness+2*s.clearance+.05,s.width+2*s.clearance,s.armLength+s.clearance+1]));
  const winX=right?inner-.1:x0-.1;
  base=base.cut(box([winX,y-s.clearance,z-s.clearance],[c.wall+.2,s.width+2*s.clearance,s.hookHeight+2*s.clearance]));
  record(`SHELL_${right?'R':'L'}_${y}`,'CASE_COVER','CASE_BASE','Press inward through side window; lift lid while holding released.');
 }
 // Roof-mounted clip, inserting its retained item upwards from the open underside.
 // axis 0 = left/right edges, axis 1 = front/rear edges; hook is below the item.
 const downClip=(cx:number,cy:number,edge:number,positive:boolean,axis:0|1,bottom:number,width:number,id:string,retains:string)=>{
  const t=s.armThickness,g=s.clearance,e=s.engagement;
  const lateral=positive?edge+g:edge-g-t;
  const min:Vec3=axis===0?[lateral,cy-width/2,bottom-s.hookHeight-g]:[cx-width/2,lateral,bottom-s.hookHeight-g];
  const size:Vec3=axis===0?[t,width,l.lid-min[2]]:[width,t,l.lid-min[2]];
  const start=positive?edge-e:edge-g-t;
  const hookMin:Vec3=axis===0?[start,cy-width/2,min[2]]:[cx-width/2,start,min[2]];
  const hookSize:Vec3=axis===0?[t+g+e,width,s.hookHeight]:[width,t+g+e,s.hookHeight];
  cover=cover.fuse(box(min,size)).fuse(box(hookMin,hookSize).chamfer(.2,e=>e.inPlane('XY',min[2])));
  record(id,'CASE_COVER',retains,retains.startsWith('CAP')?'Pinch the cap ears from the underside and withdraw the cap upward.':'With lid removed, flex exposed clip outward and withdraw from underside.');
 };
 // OLED locates directly against four roof bosses; edge clips replace the deck and screws.
 for(const xx of [3,p.oled.width-6])for(const yy of [1,p.oled.depth-3]){
  cover=cover.fuse(box([o.position[0]+xx,o.position[1]+yy,l.displayPcb+p.oled.pcbThickness],[3,2,l.lid-l.displayPcb-p.oled.pcbThickness]));
 }
 for(const xx of [5,p.oled.width-5])for(const rear of [false,true])downClip(o.position[0]+xx,0,o.position[1]+(rear?p.oled.depth:0),rear,1,l.displayPcb,3,`OLED_${xx}_${rear}`,'DISP1');
 const vx=o.position[0]+p.oled.glassX+(p.oled.glassWidth-p.oled.viewWidth)/2,vy=o.position[1]+p.oled.glassY+(p.oled.glassDepth-p.oled.viewDepth)/2;
 cover=cover.cut(box([vx-.6,vy-.6,l.lid-1],[p.oled.viewWidth+1.2,p.oled.viewDepth+1.2,c.roof+2]).fillet(.5,e=>e.inDirection('Z')));
 // Switches install from underneath; button caps snap through the top opening afterwards.
 for(const id of ['1','2']){
  const sw=get('SW'+id),cap=get('CAP'+id),cx=sw.position[0]+sw.size[0]/2,cy=sw.position[1]+sw.size[1]/2;
  for(const right of [false,true])downClip(cx,cap.position[1]+p.cap.depth-3,cap.position[0]+(right?p.cap.width-2+p.cap.latchProjection:2-p.cap.latchProjection),right,0,l.stopTop+s.clearance,2,`CAP_${id}_${right}`,'CAP'+id);
  for(const rear of [false,true]){
   const sign=rear?1:-1,outer=cy+sign*(p.cap.depth/2+s.clearance),edge=cy+sign*p.button.bodyDepth/2;
   const armY=rear?outer:outer-s.armThickness,hookY=rear?edge-s.engagement:armY;
   const bottom=l.switchSeat-s.clearance-s.hookHeight;
   cover=cover.fuse(box([cx-1.5,armY,bottom],[3,s.armThickness,l.lid-bottom]));
   cover=cover.fuse(box([cx-1.5,hookY,bottom],[3,Math.abs(outer-edge)+s.armThickness+s.engagement,s.hookHeight]).chamfer(.2,e=>e.inPlane('XY',bottom)));
   const stopY=rear?edge-1:armY;
   cover=cover.fuse(box([cx-1.5,stopY,l.switchSeat+p.button.bodyHeight],[3,Math.abs(outer-edge)+s.armThickness+1,1]));
   record(`SW_${id}_${rear}`,'CASE_COVER','SW'+id,'Flex the front/rear cradle arms outward and withdraw the switch downwards.');
  }
  cover=cover.cut(box([cap.position[0]+2-p.cap.sideGap,cap.position[1]+2-p.cap.sideGap,l.lid-1],[p.cap.width-4+2*p.cap.sideGap,p.cap.depth-4+2*p.cap.sideGap,c.roof+2]));
 }
 // Loose battery pocket. A single roof-hung bridge restrains it without pressure.
 const [bx,by,bz]=bat.position,[bw,bd,bh]=bat.size,bc=c.batteryClearance,fw=1.2;
 let fence=box([bx-bc-fw,by-bc-fw,seam],[bw+2*(bc+fw),bd+2*(bc+fw),3]).cut(box([bx-bc,by-bc,seam-1],[bw+2*bc,bd+2*bc,5]));
 const notch=2*p.battery.leadSpace+4;
 fence=fence.cut(box([bx+bw/2-notch/2,by+bd,seam-1],[notch,bc+fw+1,5]));carrier=carrier.fuse(fence).fuse(box([bx,by,seam],[bw,bd,bz-seam]));
 const rx=bx+bw-5,rz=bz+bh+bc;
 cover=cover.fuse(box([rx,by-bc-fw,rz],[2,fw,l.lid-rz])).fuse(box([rx,by+bd+bc,rz],[2,fw,l.lid-rz])).fuse(box([rx,by-bc-fw,rz],[2,bd+2*(bc+fw),1.2]));
 // Tray-mounted upstanding clips: power board and connector drop in from above.
 const upClip=(cx:number,cy:number,edge:number,positive:boolean,top:number,width:number,id:string,retains:string)=>{
  const t=s.armThickness,g=s.clearance,e=s.engagement;
  const x=positive?edge+g:edge-g-t;
  carrier=carrier.fuse(box([x,cy-width/2,seam],[t,width,top+g+s.hookHeight-seam]));
  const hx=positive?edge-e:edge-g-t;
  carrier=carrier.fuse(box([hx,cy-width/2,top+g],[t+g+e,width,s.hookHeight]).chamfer(.2,e=>e.inPlane('XY',top+g+s.hookHeight)));
  record(id,'CASE_CARRIER',retains,'With lid removed, flex side clip outward and lift part.');
 };
 const [px,py,pz]=power.position;
 for(const xx of [2,p.power.width-4])carrier=carrier.fuse(box([px+xx,py+2,seam],[2,p.power.depth-4,pz-seam]));
 for(const right of [false,true])upClip(0,py+p.power.depth/2,px+(right?p.power.width:0),right,pz+p.power.pcbThickness,4,`POWER_${right}`,'PWR2');
 const j=get('J1'),[jx,jy,jz]=j.position,[jw,jd,jh]=j.size;
 carrier=carrier.fuse(box([jx,jy,seam],[jw,jd,jz-seam]));
 for(const right of [false,true])upClip(0,jy+jd/2,jx+(right?jw:0),right,jz+jh,2,`CONNECTOR_${right}`,'J1');
 const ps=get('PWR1');
 carrier=carrier.fuse(box([ps.position[0]-1,ps.position[1]-1,seam],[ps.size[0]+2,p.powerSwitch.depth+1,ps.size[2]+.5]).cut(box([ps.position[0]-.25,ps.position[1]-.25,seam+.5],[ps.size[0]+.5,p.powerSwitch.depth+1.5,ps.size[2]+1])));
 cover=cover.cut(box([ps.position[0]-1,ps.position[1]+p.powerSwitch.depth,ps.position[2]-1],[ps.size[0]+2,y1-ps.position[1]+1,ps.size[2]+2]));
 const parts:PrintedPart[]=[
  {id:'CASE_BASE',name:'Snap base and breadboard tray',color:'#355b6e',shape:base,lift:-20},
  {id:'CASE_CARRIER',name:'Captured electronics tray',color:'#789d94',shape:carrier,lift:10},
  {id:'CASE_COVER',name:'Snap lid with integral component clips',color:'#cedadd',shape:cover,lift:65},
  ...['CAP1','CAP2'].map(id=>({id,name:id==='CAP1'?'Left button':'Right button',color:'#ecab5e',shape:capModel.build(p.cap).pieces[0].shape.translate(get(id).position),lift:45})),
 ];
 return {parts,snaps,bounds:{min:[x0,y0,-c.floor],max:[x1,y1,l.lid+c.roof]},status:'five-piece integral-snap mechanical concept',blockers:[
  'Snap deflection, insertion force, fatigue and release need physical print validation; this is seated geometry, not a material simulation.',
  'Dimensions remain editable assumptions. Battery restraint has clearance and must not squeeze the pouch.',
  'Power module and power switch are mechanical studies; the battery circuit is not selected or validated.',
 ]};
}
