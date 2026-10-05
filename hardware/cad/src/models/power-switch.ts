import {box,body,type Model} from './common.ts';
export const defaults={width:10,depth:5,height:5,leverHeight:3};
export type Dimensions=typeof defaults;
export function build(d:Dimensions):Model{return {size:[d.width,d.depth+d.leverHeight,d.height],mounts:[],anchors:{lever:[d.width/2,d.depth+d.leverHeight,d.height/2]},pieces:[body('power switch housing','#9aa6b0',box([0,0,0],[d.width,d.depth,d.height])),body('power switch lever','#242c35',box([d.width/2-1.5,d.depth,d.height/2-1.5],[3,d.leverHeight,3]))]};}
