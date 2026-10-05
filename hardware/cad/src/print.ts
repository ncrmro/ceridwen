import type {Shape3D} from 'replicad';
/** STEP stays in assembly coordinates; STL sits on Z=0 for slicer import. */
export function printSolid(id:string,shape:Shape3D):Shape3D {
 let solid=shape.clone();if(id==='CASE_COVER'||id.startsWith('CAP')||id.startsWith('PUSH_PIN'))solid=solid.rotate(180,[0,0,0],[1,0,0]);
 const [min]=solid.boundingBox.bounds;return solid.translate([-min[0],-min[1],-min[2]]);
}
