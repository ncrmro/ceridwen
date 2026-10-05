import { box,body,type Model } from './common.ts';
export const defaults={length:32,width:20.5,thickness:5.3,leadSpace:2};
export type Dimensions=typeof defaults;
export function build(d:Dimensions):Model {
 return {size:[d.length,d.width,d.thickness],mounts:[],anchors:{leadExit:[d.length/2,d.width,d.thickness/2]},pieces:[
 body('protected LiPo pouch','#c6cbd0',box([0,0,0],[d.length,d.width,d.thickness]).fillet(.65,e=>e.inDirection('Z'))),
 body('pack label','#eff0d7',box([3,3,d.thickness],[d.length-6,d.width-6,.03]),true),
 body('insulating tab wrap','#c9a33f',box([0,d.width-2,0],[d.length,2,d.thickness])),
 ]};
}
