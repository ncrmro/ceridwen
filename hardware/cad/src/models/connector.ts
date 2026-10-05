import {box,body,type Model} from './common.ts';
export const defaults={width:8,depth:6,height:5};
export type Dimensions=typeof defaults;
export function build(d:Dimensions):Model{return {size:[d.width,d.depth,d.height],mounts:[],anchors:{input:[d.width/2,0,d.height/2],output:[d.width/2,d.depth,d.height/2]},pieces:[body('mated battery connector','#eee7d4',box([0,0,0],[d.width,d.depth,d.height]).cut(box([1,d.depth/2-.2,d.height-.7],[d.width-2,.4,1])))]};}
