import { box, pin, body, compound, type Model } from './common.ts';
export const defaults = { length:82, width:55, height:9, pitch:2.54, rows:30, channel:7.62 };
export type Dimensions = typeof defaults;
export function grid(d: Dimensions, row: number, bank: number, column: number): [number,number] {
  return [(d.length-(d.rows-1)*d.pitch)/2+row*d.pitch, d.width/2 + bank*(d.channel/2+column*d.pitch)];
}
export function build(d: Dimensions): Model {
  const shell=box([0,0,0],[d.length,d.width,d.height]).cut(box([2,d.width/2-1, d.height-1],[d.length-4,2,2]));
  const holes=[];
  for(let r=0;r<d.rows;r++) for(const bank of [-1,1]) for(let c=0;c<5;c++) {
    const [x,y]=grid(d,r,bank,c); holes.push(pin(x,y,d.height,0.45,0.02));
  }
  for(let r=0;r<25;r++) for(const y of [4,6.54,d.width-6.54,d.width-4]) holes.push(pin((d.length-24*d.pitch)/2+r*d.pitch,y,d.height,0.45,0.02));
  return {size:[d.length,d.width,d.height], mounts:[], anchors:{top:[0,0,d.height]},pieces:[
    body('breadboard body','#eee8d8',shell),body('socket pattern','#36424b',compound(holes),true),
    body('positive rails','#b94442',compound([box([4,2,d.height+.02],[d.length-8,.35,.02]),box([4,d.width-2,d.height+.02],[d.length-8,.35,.02])]),true),
  ]};
}
