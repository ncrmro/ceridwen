import type {Parameters} from '../parameters.ts';
import type {Part} from '../assembly.ts';
import type {Model} from './common.ts';
import * as breadboard from './breadboard.ts';import * as esp32 from './esp32.ts';import * as oled from './oled.ts';import * as button from './switch.ts';import * as battery from './battery.ts';import * as power from './power.ts';import * as connector from './connector.ts';import * as powerSwitch from './power-switch.ts';import * as cap from './cap.ts';
export function component(p:Parameters,part:Part):Model {
 switch(part.id){case 'BB1':return breadboard.build(p.breadboard);case 'U1':return esp32.build(p.esp32);case 'DISP1':return oled.build(p.oled);case 'SW1':case 'SW2':return button.build(p.button);case 'BAT1':return battery.build(p.battery);case 'PWR2':return power.build(p.power);case 'J1':return connector.build(p.connector);case 'PWR1':return powerSwitch.build(p.powerSwitch);case 'CAP1':case 'CAP2':return cap.build(p.cap);default:throw new Error(`No component model: ${part.id}`);}
}
