import {box,pin,body,type Model} from './common.ts';
/** Mechanical study for an unselected power module, not a wiring prescription. */
export const defaults={width:26,depth:18,height:7,pcbThickness:1.6,holeInset:2};
export type Dimensions=typeof defaults;
export function build(d:Dimensions):Model {
 let pcb=box([0,0,0],[d.width,d.depth,d.pcbThickness]);const mounts:[number,number,number][]=[];
 for(const x of [d.holeInset,d.width-d.holeInset])for(const y of [d.holeInset,d.depth-d.holeInset]){mounts.push([x,y,0]);pcb=pcb.cut(pin(x,y,-1,1.15,d.pcbThickness+2));}
 return {size:[d.width,d.depth,d.height],mounts,anchors:{input:[2,d.depth/2,d.pcbThickness],output:[d.width-2,d.depth/2,d.pcbThickness]},pieces:[body('power module study PCB','#755e93',pcb),body('inductor space','#3d4148',box([d.width/2-4,d.depth/2-4,d.pcbThickness],[8,8,d.height-d.pcbThickness])),body('controller space','#272c35',box([3,d.depth/2-2,d.pcbThickness],[4,4,1.5]))]};
}
