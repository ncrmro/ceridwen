import {box,body,type Model} from './common.ts';
import type {Parameters} from '../parameters.ts';
export function build(d:Parameters['cap']):Model {
 const w=d.width,h=d.depth,z=d.stemHeight,fh=d.flangeThickness;
 const solid=box([w/2-d.stemWidth/2,h/2-d.stemWidth/2,0],[d.stemWidth,d.stemWidth,z])
 .fuse(box([0,0,z],[w,h,fh])).fuse(box([2,2,z+fh],[w-4,h-4,d.fingerHeight]).fillet(.65,e=>e.inDirection('Z')));
 return {size:[w,h,z+fh+d.fingerHeight],mounts:[],anchors:{contact:[w/2,h/2,0],flangeBottom:[0,0,z]},pieces:[body('printed button','#ecab5e',solid)]};
}
