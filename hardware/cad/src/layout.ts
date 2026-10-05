import type {Assembly,Part,Vec3} from './assembly.ts';
import {defaultParameters,validateParameters,type Parameters} from './parameters.ts';
export interface Layout { assembly:Assembly; carrier:number; deck:number; displayPcb:number; switchSeat:number; lid:number; capBottom:number; stopTop:number; params:Parameters }
export function derive(p:Parameters=defaultParameters):Layout {
 validateParameters(p);
 const b=p.breadboard,m=p.esp32,o=p.oled,s=p.button,c=p.case;
 const carrier=b.height+c.wireGap;
 const lowerTop=carrier+c.carrierThickness+.5+Math.max(p.battery.thickness,p.power.height,p.connector.height,p.powerSwitch.height);
 const deck=lowerTop+c.layerGap;
 const switchReferencePcb=deck+c.deckThickness+c.standoff;
 const screenTop=switchReferencePcb+o.pcbThickness+o.glassThickness;
 const switchSeat=screenTop-s.bodyHeight-s.actuatorHeight;
 const capBottom=screenTop+p.cap.contactGap;
 const stopTop=capBottom+p.cap.stemHeight-p.cap.contactGap-s.travel;
 const lid=capBottom+p.cap.stemHeight+p.cap.flangeThickness+p.cap.returnGap;
 const displayPcb=lid-c.screenGap-o.pcbThickness-o.glassThickness;
 const mcuX=(b.length-(b.rows-1)*b.pitch)/2+b.pitch-(m.length-(m.pins-1)*m.pitch)/2;
 const mcuY=b.width/2-b.channel/2-b.pitch-(m.width-m.rowSpacing)/2;
 const parts:Part[]=[];
 function add(id:string,name:string,size:Vec3,position:Vec3,color:string,source:string){parts.push({id,name,size,position,color,source,evidence:'provisional',missing:['Replace assumed dimensions with selected-part measurements'],kind:'component'});}
 add('BB1','ELEGOO 400-point breadboard',[b.length,b.width,b.height],[0,0,0],'#eee8d8','ELEGOO; assumed outline, parametric 2.54 mm grid');
 add('U1','Teyleten ESP32-C3 with headers',[m.length,m.width,m.pcbBase+m.pcbThickness+Math.max(m.usbHeight,2)],[mcuX,mcuY,b.height-3],'#155158','Teyleten Supermini; header and USB parameters editable');
 add('DISP1','Hosyond SSD1306 OLED',[o.width,o.depth,o.pcbBase+o.pcbThickness+o.glassThickness],[(b.length-o.width)/2,5,displayPcb-o.pcbBase],'#1d687c','Hosyond; assumed PCB/glass/hole datums, editable separately');
 const swWidth=Math.max(s.bodyWidth,s.leadSpacing+.5),swHeight=s.leadLength+s.bodyHeight+s.actuatorHeight;
 for(const [id,cx] of [['1',b.length/3],['2',b.length*2/3]] as const){
  add('SW'+id,'Chanzon '+(id==='1'?'left':'right')+' switch',[swWidth,s.bodyDepth,swHeight],[cx-swWidth/2,b.width-11-s.bodyDepth/2,switchSeat-s.leadLength],'#aab5c0','Chanzon 6 × 6 × 5; assumed lead spacing/travel');
  add('CAP'+id,(id==='1'?'Left':'Right')+' printed button',[p.cap.width,p.cap.depth,p.cap.stemHeight+p.cap.flangeThickness+p.cap.fingerHeight],[cx-p.cap.width/2,b.width-11-p.cap.depth/2,capBottom],'#ecab5e','Derived from switch actuator datum and cap parameters');
 }
 const lowerZ=carrier+c.carrierThickness+.5;
 add('BAT1','EEMB protected 250mAh battery',[p.battery.length,p.battery.width,p.battery.thickness],[b.length-p.battery.length-7,5,lowerZ],'#c6cbd0','EEMB store nominal 32 × 20.5 × 5.3 mm; installed pack assumptions editable');
 add('PWR2','Power module mechanical study',[p.power.width,p.power.depth,p.power.height],[b.length-p.power.width-9,b.width-p.power.depth-6,lowerZ],'#755e93','Generic packaging study; charger/regulator circuit not selected');
 add('J1','Battery connector',[p.connector.width,p.connector.depth,p.connector.height],[mcuX+m.length+3,b.width-p.connector.depth-6,lowerZ],'#eee7d4','JST 2.0 mm selected; mated body and lead exits assumed');
 add('PWR1','Power switch mechanical study',[p.powerSwitch.width,p.powerSwitch.depth+p.powerSwitch.leverHeight,p.powerSwitch.height],[6,b.width-p.powerSwitch.depth-p.powerSwitch.leverHeight,lowerZ],'#9aa6b0','Generic side-actuated switch; electrical part not selected');
 parts.push({id:'USB1',name:'External USB plug approach',size:[20,12,8],position:[mcuX-20,mcuY+m.width/2-6,b.height-3+m.pcbBase+m.pcbThickness-2],color:'#77b7ee',source:'Editable cable access allowance, external to board footprint',evidence:'provisional',missing:['Actual cable dimensions'],kind:'clearance'});
 return {params:p,carrier,deck,displayPcb,switchSeat,lid,capBottom,stopTop,assembly:{schema:1,name:'Ceridwen · parametric assembled handheld',parameters:p,minimumGap:.3,parts,mates:[['BB1','U1'],['U1','USB1'],['CAP1','SW1'],['CAP2','SW2']],unresolved:['Assumed dimensions require physical verification before manufacturing.','Power module and switch are mechanical studies, not a completed battery circuit.']}};
}
