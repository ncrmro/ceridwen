import { box, body, compound, type Model } from './common.ts';
export const defaults={ length:22.5,width:18,pcbThickness:1.6,pitch:2.54,rowSpacing:15.24,pins:8,pcbBase:7,usbWidth:9,usbHeight:3.2 };
export type Dimensions=typeof defaults;
export function anchors(d:Dimensions):Record<string,[number,number,number]>{
 const pin=(index:number,far=false):[number,number,number]=>[(d.length-(d.pins-1)*d.pitch)/2+index*d.pitch,(d.width+(far?1:-1)*d.rowSpacing)/2,d.pcbBase+d.pcbThickness+1];
 return {usb:[0,d.width/2,d.pcbBase+d.pcbThickness+d.usbHeight/2],vcc:pin(2),gnd:pin(1),sda:pin(3),scl:pin(0,true),left:pin(7),right:pin(6)};
}
export function build(d:Dimensions):Model {
 const pins=[];const plastics=[];
 for(const y of [(d.width-d.rowSpacing)/2,(d.width+d.rowSpacing)/2]) {
   plastics.push(box([(d.length-d.pins*d.pitch)/2,y-1.27,3],[d.pins*d.pitch,2.54,2.5]));
   for(let i=0;i<d.pins;i++) pins.push(box([(d.length-(d.pins-1)*d.pitch)/2+i*d.pitch-.32,y-.32,0],[.64,.64,d.pcbBase+d.pcbThickness+1]));
 }
 const top=d.pcbBase+d.pcbThickness;
 return {size:[d.length,d.width,top+Math.max(d.usbHeight,2)],mounts:[],anchors:anchors(d),pieces:[
 body('ESP32-C3 PCB','#155158',box([0,0,d.pcbBase],[d.length,d.width,d.pcbThickness])),
 body('headers','#202731',compound(plastics)),body('pins','#c1bd91',compound(pins)),
 body('USB-C shell','#b8c4cf',box([0,(d.width-d.usbWidth)/2,top],[6,d.usbWidth,d.usbHeight]).cut(box([-.1,(d.width-d.usbWidth)/2+.6,top+.5],[5.5,d.usbWidth-1.2,d.usbHeight-1]))),
 body('ESP32 package','#252b33',box([8,5,top],[8,8,1.4])),body('antenna','#c29b66',box([d.length-4,1,top],[3,4,1])),
 ]};
}
