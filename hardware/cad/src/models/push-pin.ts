import {pin,box,body,type Model} from './common.ts';
export const defaults={diameter:2,headDiameter:3.8,headHeight:1.6,slotWidth:.5,radialInterference:.05};
/** Split friction pin, no threads. The slot allows elastic compression in its socket. */
export function build(d:typeof defaults,length:number):Model {
 const r=d.headDiameter/2;
 let shaft=pin(r,r,0,d.diameter/2,length);
 // Lead-in at the insertion end; leave a solid neck below the thumb head.
 shaft=shaft.chamfer(.25,e=>e.inPlane('XY',0));
 const solid=shaft.fuse(pin(r,r,length,r,d.headHeight))
  .cut(box([r-d.slotWidth/2,0,-.1],[d.slotWidth,d.headDiameter,length-1.1]));
 return {size:[d.headDiameter,d.headDiameter,length+d.headHeight],mounts:[],anchors:{head:[r,r,length]},pieces:[body(`Printed split push-pin ${length} mm`,'#e0a264',solid)]};
}
