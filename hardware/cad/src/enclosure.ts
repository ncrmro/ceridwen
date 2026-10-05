import {makeCylinder,type Shape3D} from 'replicad';
import {box,pin} from './models/common.ts';
import * as capModel from './models/cap.ts';
import type {Assembly,Part,Vec3} from './assembly.ts';
import {derive,type Layout} from './layout.ts';
import {defaultParameters} from './parameters.ts';
export interface PrintedPart {id:string;name:string;color:string;shape:Shape3D;lift:number}
export const envelopeSolid=(p:Part)=>box(p.position,p.size);
export function capSolid(p:Part){return capModel.build(defaultParameters.cap).pieces[0].shape.translate(p.position);}
export interface Screw {id:string;center:[number,number];headZ:number;length:number;up?:boolean;owner:string}
export function enclosure(assembly:Assembly){return buildEnclosure(derive(assembly.parameters??defaultParameters));}
export function buildEnclosure(l:Layout){
 const {params:p}=l,b=p.breadboard,c=p.case;const get=(id:string)=>l.assembly.parts.find(q=>q.id===id)!;
 const x0=-c.gap-c.screwRail,x1=b.length+c.gap+c.screwRail,y0=-c.gap-c.wall,y1=b.width+c.gap+c.wall;
 const lowerTop=l.carrier+c.carrierThickness,deckTop=l.deck+c.deckThickness;
 const rounded=(min:Vec3,size:Vec3)=>box(min,size).fillet(c.cornerRadius,e=>e.inDirection('Z'));
 let base=rounded([x0,y0,-c.floor],[x1-x0,y1-y0,l.carrier+c.floor]).cut(box([-c.gap,-c.gap,0],[b.length+2*c.gap,b.width+2*c.gap,l.carrier+1]));
 let carrier=rounded([x0,y0,l.carrier],[x1-x0,y1-y0,c.carrierThickness]);
 let cover=rounded([x0,y0,lowerTop],[x1-x0,y1-y0,l.lid+c.roof-lowerTop]).cut(box([-c.gap,-c.gap,lowerTop-1],[b.length+2*c.gap,b.width+2*c.gap,l.lid-lowerTop+1]));
 let deck=box([.5,.5,l.deck],[b.length-1,b.width-1,c.deckThickness]);
 const screws:Screw[]=[];
 const m=get('U1'),o=get('DISP1'),bat=get('BAT1'),power=get('PWR2'),usb=get('USB1');
 const mcuOpening=box([m.position[0]-.5,m.position[1]-.5,l.carrier-1],[m.size[0]+1,m.size[1]+1,deckTop-l.carrier+2]);
 carrier=carrier.cut(mcuOpening);deck=deck.cut(mcuOpening);
 // USB approaches the short edge of the MCU, aligned with the header/grid orientation.
 const usbCut=box([x0-1,usb.position[1]-.5,usb.position[2]-.5],[m.position[0]-x0+2,usb.size[1]+1,usb.size[2]+1]);
 base=base.cut(usbCut);carrier=carrier.cut(usbCut);cover=cover.cut(usbCut);
 // Through-carrier service slots alongside, never outside, the breadboard.
 for(const [x,y,w,d] of [[m.position[0]+m.size[0]+1,4,7,11],[m.position[0]+m.size[0]+1,b.width-21,7,10]]){
  const tool=box([x,y,l.carrier-1],[w,d,c.carrierThickness+2]);carrier=carrier.cut(tool);
 }
 // Capturing the breadboard body between tray and carrier avoids adhesive-only retention.
 for(const [x,y] of [[8,1],[b.length-10,1],[8,b.width-3],[b.length-10,b.width-3]])carrier=carrier.fuse(box([x,y,b.height],[2,2,l.carrier-b.height]));
 // Carrier-to-cover and base-to-carrier have independent screw positions.
 for(const x of [-c.gap-c.screwRail/2,b.length+c.gap+c.screwRail/2]){
  for(const y of [6,b.width-6]){
   base=base.cut(pin(x,y,l.carrier-7,.8,8));carrier=carrier.cut(pin(x,y,l.carrier-1,1.15,c.carrierThickness+2));
   cover=cover.cut(pin(x,y,lowerTop,2.2,l.lid+c.roof-lowerTop+1));
   screws.push({id:`BASE_${x}_${y}`,center:[x,y],headZ:lowerTop,length:8,owner:'CASE_BASE'});
  }
  for(const y of [14,b.width-14]){
   base=base.cut(pin(x,y,l.carrier-1.8,2.2,2.8));
   carrier=carrier.cut(pin(x,y,l.carrier-1,1.15,c.carrierThickness+2));cover=cover.cut(pin(x,y,lowerTop-.1,.8,7.1));
   screws.push({id:`COVER_${x}_${y}`,center:[x,y],headZ:l.carrier,length:8,up:true,owner:'CASE_COVER'});
  }
 }
 // A removable upper deck transfers button forces into the carrier and case.
 for(const [x,y] of [[2.5,2.5],[b.length-2.5,2.5],[2.5,b.width-2.5],[b.length-2.5,b.width-2.5]]){
  deck=deck.fuse(pin(x,y,lowerTop,2,l.deck-lowerTop)).cut(pin(x,y,lowerTop-.1,.8,7.1));
  carrier=carrier.cut(pin(x,y,l.carrier-1,1.15,c.carrierThickness+2));
  screws.push({id:`DECK_${x}_${y}`,center:[x,y],headZ:l.carrier,length:8,up:true,owner:'CASE_DECK'});
 }
 // Protected battery pocket: lateral clearance, lead notch and loose overhead restraint.
 const [bx,by,bz]=bat.position,[bw,bd,bh]=bat.size;const bc=c.batteryClearance,fw=1.2;
 let fence=box([bx-bc-fw,by-bc-fw,lowerTop],[bw+2*(bc+fw),bd+2*(bc+fw),3])
 .cut(box([bx-bc,by-bc,lowerTop-1],[bw+2*bc,bd+2*bc,5]));
 const notch=2*p.battery.leadSpace+4;
 fence=fence.cut(box([bx+bw/2-notch/2,by+bd,lowerTop-1],[notch,bc+fw+1,5]));carrier=carrier.fuse(fence).fuse(box([bx,by,lowerTop],[bw,bd,bz-lowerTop]));
 const retainerZ=bz+bh+bc;
 for(const xx of [bx+3,bx+bw-5]){
  deck=deck.fuse(box([xx,by-bc-fw,retainerZ],[2,fw,l.deck-retainerZ]))
   .fuse(box([xx,by+bd+bc,retainerZ],[2,fw,l.deck-retainerZ]))
   .fuse(box([xx,by-bc-fw,retainerZ],[2,bd+2*(bc+fw),1.2]));
 }
 // Power-module standoffs use its own editable mounting-hole datum.
 for(const xx of [p.power.holeInset,p.power.width-p.power.holeInset])for(const yy of [p.power.holeInset,p.power.depth-p.power.holeInset]){
  const x=power.position[0]+xx,y=power.position[1]+yy;
  carrier=carrier.fuse(pin(x,y,lowerTop,1.8,power.position[2]-lowerTop)).cut(pin(x,y,l.carrier-.1,.8,3));
  screws.push({id:`POWER_${x}_${y}`,center:[x,y],headZ:power.position[2]+p.power.pcbThickness,length:4,owner:'CASE_CARRIER'});
 }
 // OLED: PCB standoffs, header relief and a glass/viewport-derived bezel.
 for(const xx of [p.oled.holeInset,p.oled.width-p.oled.holeInset])for(const yy of [p.oled.holeInset,p.oled.depth-p.oled.holeInset]){
  const x=o.position[0]+xx,y=o.position[1]+yy;
  deck=deck.fuse(pin(x,y,deckTop,1.8,l.displayPcb-deckTop)).cut(pin(x,y,deckTop-1,.8,l.displayPcb-deckTop+2));
  screws.push({id:`OLED_${x}_${y}`,center:[x,y],headZ:l.displayPcb+p.oled.pcbThickness,length:4,owner:'CASE_DECK'});
 }
 const headerX=o.position[0]+p.oled.width/2;
 deck=deck.cut(box([headerX-5.5,o.position[1]+p.oled.depth-4.5,l.deck-1],[11,3,c.deckThickness+2]));
 const vx=o.position[0]+p.oled.glassX+(p.oled.glassWidth-p.oled.viewWidth)/2;
 const vy=o.position[1]+p.oled.glassY+(p.oled.glassDepth-p.oled.viewDepth)/2;
 cover=cover.cut(box([vx-.6,vy-.6,l.lid-1],[p.oled.viewWidth+1.2,p.oled.viewDepth+1.2,c.roof+2]).fillet(.5,e=>e.inDirection('Z')));
 // Switch saddles leave the entire lead corridor open. Separate stop plates capture caps.
 const stops:PrintedPart[]=[];
 for(const id of ['1','2']){
  const sw=get('SW'+id),cap=get('CAP'+id),cx=sw.position[0]+sw.size[0]/2,cy=sw.position[1]+sw.size[1]/2;
  const w=p.button.bodyWidth,d=p.button.bodyDepth;
  for(const y of [cy-d/2,cy+d/2-1])deck=deck.fuse(box([cx-w/2,y,deckTop],[w,1,l.switchSeat-deckTop]));
  let saddle=box([cx-w/2-1.5,cy-d/2-1.5,deckTop],[w+3,d+3,l.switchSeat+1.5-deckTop])
   .cut(box([cx-w/2-.3,cy-d/2-.3,deckTop-.1],[w+.6,d+.6,l.switchSeat+2-deckTop]));
  saddle=saddle.cut(box([cx-w/2-2,cy-.7,deckTop-1],[w+4,1.4,l.switchSeat-deckTop+1.1]));deck=deck.fuse(saddle);
  deck=deck.cut(box([cx-p.button.leadSpacing/2-.6,cy-.7,l.deck-1],[p.button.leadSpacing+1.2,1.4,c.deckThickness+2]));
  const plateBottom=l.stopTop-1.2;
  let stop=box([cx-p.cap.width/2-5,cy-p.cap.depth/2-2,plateBottom],[p.cap.width+10,p.cap.depth+4,1.2]);
  const opening=Math.max(p.button.actuatorDiameter+.4,p.cap.stemWidth+.6);
  stop=stop.cut(box([cx-opening/2,cy-opening/2,plateBottom-1],[opening,opening,3.2]));
  for(const x of [cx-p.cap.width/2-3,cx+p.cap.width/2+3]){
   deck=deck.fuse(pin(x,cy,deckTop,2.2,plateBottom-deckTop)).cut(pin(x,cy,plateBottom-6,.8,7));
   stop=stop.cut(pin(x,cy,plateBottom-1,1.15,3.2));
   screws.push({id:`STOP_${id}_${x}`,center:[x,cy],headZ:l.stopTop,length:6,owner:'CASE_DECK'});
  }
  stops.push({id:'CASE_STOP'+id,name:'Button '+id+' retainer and travel stop',color:'#9cb1b8',shape:stop,lift:35});
  cover=cover.cut(box([cap.position[0]+2-p.cap.sideGap,cap.position[1]+2-p.cap.sideGap,l.lid-1],[p.cap.width-4+2*p.cap.sideGap,p.cap.depth-4+2*p.cap.sideGap,c.roof+2]));
 }
 // Connector cradle and upper-deck retainer provide a detachable, supported connection.
 const j=get('J1'),[jx,jy,jz]=j.position,[jw,jd,jh]=j.size;
 let cradle=box([jx-1.6,jy-1.6,lowerTop],[jw+3.2,jd+3.2,4])
  .cut(box([jx-.4,jy-.4,lowerTop-.1],[jw+.8,jd+.8,4.2]))
  .cut(box([jx+jw/2-2,jy-2,lowerTop+1.5],[4,jd+4,3]));
 carrier=carrier.fuse(cradle).fuse(box([jx,jy,lowerTop],[jw,jd,jz-lowerTop]));
 const jrz=jz+jh+.6;
 deck=deck.fuse(box([jx-1.6,jy+2,jrz],[1.2,2,l.deck-jrz]))
  .fuse(box([jx+jw+.4,jy+2,jrz],[1.2,2,l.deck-jrz]))
  .fuse(box([jx-1.6,jy+2,jrz],[jw+3.2,2,1.2]));
 // Side-actuated power switch is located by a removable pocket at the rear wall.
 const ps=get('PWR1');
 let pocket=box([ps.position[0]-1,ps.position[1]-1,lowerTop],[ps.size[0]+2,p.powerSwitch.depth+1,ps.size[2]+.5])
  .cut(box([ps.position[0]-.25,ps.position[1]-.25,lowerTop+.5],[ps.size[0]+.5,p.powerSwitch.depth+1.5,ps.size[2]+1]));
 carrier=carrier.fuse(pocket);
 cover=cover.cut(box([ps.position[0]-1,ps.position[1]+p.powerSwitch.depth,ps.position[2]-1],[ps.size[0]+2,y1-ps.position[1]+1,ps.size[2]+2]));
 const parts:PrintedPart[]=[
 {id:'CASE_BASE',name:'Breadboard tray',color:'#355b6e',shape:base,lift:-20},
 {id:'CASE_CARRIER',name:'Battery and power carrier',color:'#789d94',shape:carrier,lift:10},
 {id:'CASE_DECK',name:'Display and switch mounting deck',color:'#7293a1',shape:deck,lift:24},
 ...stops,{id:'CASE_COVER',name:'Bezel and guided-button cover',color:'#cedadd',shape:cover,lift:65},
 ...['CAP1','CAP2'].map(id=>({id,name:id==='CAP1'?'Left button':'Right button',color:'#ecab5e',shape:capModel.build(p.cap).pieces[0].shape.translate(get(id).position),lift:45})),
 ];
 return {parts,screws,bounds:{min:[x0,y0,-c.floor],max:[x1,y1,l.lid+c.roof]},status:'complete parametric mechanical concept',blockers:[
 'Dimensions are editable design assumptions; physical fit and print tolerances are unverified.',
 'Power module and power switch represent mechanical interfaces; battery electronics still require selection and validation.',
 'Wiring paths represent a routed mechanical harness; actual connector polarity and breadboard continuity need checking.',
 ]};
}
