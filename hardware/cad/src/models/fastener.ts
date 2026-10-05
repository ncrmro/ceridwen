import {pin,box,body,type Model} from './common.ts';
export const defaults={diameter:2,headDiameter:3.8,headHeight:1.6};
export function build(length:number):Model {
 const d=defaults;const r=d.headDiameter/2;
 const solid=pin(r,r,0,d.diameter/2,length).fuse(pin(r,r,length,r,d.headHeight)).cut(box([r-.3,0,length+.8],[.6,d.headDiameter,1]));
 return {size:[d.headDiameter,d.headDiameter,length+d.headHeight],mounts:[],anchors:{head:[r,r,length]},pieces:[body(`M2 × ${length} screw`,'#a8b3c0',solid)]};
}
