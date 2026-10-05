import {defaults as pushFit} from './models/push-pin.ts';
import {defaults as breadboard} from './models/breadboard.ts';
import {defaults as esp32} from './models/esp32.ts';
import {defaults as oled} from './models/oled.ts';
import {defaults as button} from './models/switch.ts';
import {defaults as battery} from './models/battery.ts';
import {defaults as power} from './models/power.ts';
import {defaults as connector} from './models/connector.ts';
import {defaults as powerSwitch} from './models/power-switch.ts';
export const defaultParameters={breadboard,esp32,oled,button,battery,power,connector,powerSwitch,pushFit,
 case:{wall:2,gap:1,mountRail:5,floor:2,carrierThickness:2,wireGap:3,layerGap:2,deckThickness:2,standoff:3,roof:2,cornerRadius:2.5,screenGap:.35,batteryClearance:.6},
 cap:{width:14,depth:12,flangeThickness:1.2,stemHeight:.6,stemWidth:3,fingerHeight:4,sideGap:.3,contactGap:.15,returnGap:.2},
};
export type Parameters=typeof defaultParameters;
export type ParameterGroup=keyof Parameters;
export const parameterLabels:Record<ParameterGroup,string>={breadboard:'ELEGOO breadboard',esp32:'Teyleten ESP32-C3',oled:'Hosyond OLED',button:'Chanzon switches',battery:'EEMB battery',power:'Power module study',connector:'Battery connector',powerSwitch:'Power switch study',pushFit:'Printed push-fit pins',case:'Case and mounting clearances',cap:'Printed buttons'};
export function validateParameters(value:unknown):asserts value is Parameters {
 const p=value as Parameters;
 if(!p||typeof p!=='object')throw new Error('Expected parameter groups');
 for(const group of Object.keys(defaultParameters) as ParameterGroup[]) {
  const values=p[group] as Record<string,number>;
  if(!values||typeof values!=='object')throw new Error(`Missing ${group}`);
  for(const key of Object.keys(defaultParameters[group])) if(!Number.isFinite(values[key])||values[key]<=0||values[key]>300)throw new Error(`${group}.${key} must be positive and no larger than 300 mm`);
 }
 if(!Number.isInteger(p.breadboard.rows)||p.breadboard.rows<8||p.breadboard.rows>40)throw new Error('Breadboard rows must be 8–40');
 if(!Number.isInteger(p.esp32.pins)||p.esp32.pins<8||p.esp32.pins>12)throw new Error('ESP32 pins per row must be 8–12');
 if((p.breadboard.rows-1)*p.breadboard.pitch>p.breadboard.length-4)throw new Error('Breadboard grid exceeds its body');
 if(p.esp32.rowSpacing>=p.esp32.width||p.esp32.pins*p.esp32.pitch>p.esp32.length)throw new Error('ESP32 headers exceed PCB');
 const o=p.oled;
 if(o.glassX+o.glassWidth>o.width||o.glassY+o.glassDepth>o.depth||o.viewWidth>o.glassWidth||o.viewDepth>o.glassDepth)throw new Error('OLED glass or viewport exceeds its supporting surface');
 if(o.holeInset*2>=Math.min(o.width,o.depth)||o.holeDiameter>=o.holeInset*2)throw new Error('OLED hole pattern exceeds PCB');
 if(p.cap.stemWidth>=p.button.actuatorDiameter||p.cap.width<10||p.cap.depth<10)throw new Error('Cap stem must fit actuator; flange must be at least 10 mm');
 if(p.case.mountRail<5||p.case.wall<1.5||p.case.carrierThickness<2||p.case.deckThickness<2)throw new Error('Mounting rails and plates below structural minimum');
 if(p.power.width<14||p.power.depth<12||p.power.height<=p.power.pcbThickness||p.connector.width<5||p.connector.height<2||p.esp32.usbWidth>p.esp32.width)throw new Error('Component dimensions do not contain their modeled features');
 if(p.pushFit.diameter>=2.3||p.pushFit.diameter<=1.5||p.pushFit.headDiameter>4||p.pushFit.headDiameter<3||p.pushFit.headHeight>1.6||p.pushFit.slotWidth>=p.pushFit.diameter/2||p.pushFit.radialInterference>=.15||p.pushFit.radialInterference>=p.pushFit.slotWidth/2)throw new Error('Push-fit pin dimensions exceed mounting interfaces');
 if(p.button.bodyHeight<=.25||p.battery.thickness<1.5)throw new Error('Component dimensions below modeled feature sizes');
}
