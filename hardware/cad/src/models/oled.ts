import { box,pin,body,compound,type Model } from './common.ts';
export const defaults={width:27,depth:27,pcbThickness:1.6,pcbBase:4,glassWidth:24,glassDepth:14,glassThickness:2,glassX:1.5,glassY:6,holeInset:2,holeDiameter:2.3,viewWidth:21.7,viewDepth:10.9};
export type Dimensions=typeof defaults;
export function build(d:Dimensions):Model {
 const mounts:[number,number,number][]=[];let pcb=box([0,0,d.pcbBase],[d.width,d.depth,d.pcbThickness]);
 for(const x of [d.holeInset,d.width-d.holeInset]) for(const y of [d.holeInset,d.depth-d.holeInset]) {mounts.push([x,y,d.pcbBase]);pcb=pcb.cut(pin(x,y,d.pcbBase-1,d.holeDiameter/2,d.pcbThickness+2));}
 const glassTop=d.pcbBase+d.pcbThickness+d.glassThickness;
 const pieces=[body('OLED PCB','#1d687c',pcb),body('glass','#111921',box([d.glassX,d.glassY,d.pcbBase+d.pcbThickness],[d.glassWidth,d.glassDepth,d.glassThickness])),
 body('active display','#050a0f',box([d.glassX+(d.glassWidth-d.viewWidth)/2,d.glassY+(d.glassDepth-d.viewDepth)/2,glassTop],[d.viewWidth,d.viewDepth,.02]),true)];
 const header=[];for(let i=0;i<4;i++)header.push(box([d.width/2-3.81+i*2.54-.32,d.depth-3.32,0],[.64,.64,d.pcbBase]));
 pieces.push(body('OLED header','#c0b886',compound(header)));
 const dots=[];for(const x of [-2.2,2.2])for(const y of [-3,0,3])dots.push(pin(d.glassX+d.glassWidth/2+x,d.glassY+d.glassDepth/2+y,glassTop+.04,.65,.02));
 pieces.push(body('lesson preview dots','#e8ffff',compound(dots),true));
 return {size:[d.width,d.depth,glassTop],pieces,mounts,anchors:{glass:[d.glassX,d.glassY,d.pcbBase+d.pcbThickness],header:[d.width/2,d.depth-3,d.pcbBase],viewport:[d.glassX+(d.glassWidth-d.viewWidth)/2,d.glassY+(d.glassDepth-d.viewDepth)/2,glassTop]}};
}
