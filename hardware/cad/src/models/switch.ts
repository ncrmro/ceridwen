import { box,pin,body,compound,type Model } from './common.ts';
export const defaults={bodyWidth:6,bodyDepth:6,bodyHeight:3.5,actuatorHeight:1.5,actuatorDiameter:3.4,leadLength:3,leadSpacing:6.5,travel:.25};
export type Dimensions=typeof defaults;
export function build(d:Dimensions):Model {
 const w=Math.max(d.bodyWidth,d.leadSpacing+.5),cx=w/2,cy=d.bodyDepth/2;
 return {size:[w,d.bodyDepth,d.leadLength+d.bodyHeight+d.actuatorHeight],mounts:[],anchors:{seat:[cx,cy,d.leadLength],actuator:[cx,cy,d.leadLength+d.bodyHeight+d.actuatorHeight]},pieces:[
 body('switch body','#323942',box([(w-d.bodyWidth)/2,0,d.leadLength],[d.bodyWidth,d.bodyDepth,d.bodyHeight-.25])),
 body('metal lid','#aab5c0',box([(w-d.bodyWidth)/2,0,d.leadLength+d.bodyHeight-.25],[d.bodyWidth,d.bodyDepth,.25])),
 body('actuator','#4a5660',pin(cx,cy,d.leadLength+d.bodyHeight,d.actuatorDiameter/2,d.actuatorHeight)),
 body('two terminals','#c0b88e',compound([-1,1].map(s=>box([cx+s*d.leadSpacing/2-.25,cy-.25,0],[.5,.5,d.leadLength])))),
 ]};
}
