import {box,body,type Model} from './common.ts';
import type {Parameters} from '../parameters.ts';
/** Top-inserted cap: two compliant ears clear the lid opening, then retain below it. */
export function build(d:Parameters['cap']):Model {
 const w=d.width,h=d.depth,z=d.stemHeight,fh=d.flangeThickness,t=d.flexThickness,g=d.flexGap;
 let solid=box([w/2-d.stemWidth/2,h/2-d.stemWidth/2,0],[d.stemWidth,d.stemWidth,z])
  .fuse(box([2,2,z],[w-4,h-4,fh]))
  .fuse(box([2,2,z+fh],[w-4,h-4,d.fingerHeight]).fillet(.65,e=>e.inDirection('Z')));
 for(const right of [false,true]){
  const x=right?w-2-t-g:2;
  // Free each arm on its side and above; retain a 1 mm root at the front edge.
  solid=solid.cut(box([x,3,z-.1],[t+g,h-4+.1,fh+.7]));
  const ax=right?w-2-t:2;
  solid=solid.fuse(box([ax,2,z],[t,h-4,fh]));
  const ex=right?w-2-t:2-d.latchProjection;
  solid=solid.fuse(box([ex,h-4,z],[t+d.latchProjection,2,fh]).chamfer(.2,e=>e.inPlane('XY',z)));
 }
 return {size:[w,h,z+fh+d.fingerHeight],mounts:[],anchors:{contact:[w/2,h/2,0],flangeBottom:[2-d.latchProjection,h-4,z]},pieces:[body('Snap-in button cap','#ecab5e',solid)]};
}
