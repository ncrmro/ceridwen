import {test} from 'node:test';import assert from 'node:assert/strict';
import {derive} from './layout.ts';import {defaultParameters,validateParameters} from './parameters.ts';
test('a thicker battery raises the deck, screen, switches and lid together',()=>{
 const p=structuredClone(defaultParameters),a=derive(p);p.battery.thickness=8.3;const b=derive(p);
 assert.ok(b.deck>a.deck);assert.ok(Math.abs((b.displayPcb-a.displayPcb)-(b.deck-a.deck))<1e-8);assert.ok(Math.abs((b.lid-a.lid)-(b.deck-a.deck))<1e-8);
 const position=(l:ReturnType<typeof derive>,id:string)=>l.assembly.parts.find(p=>p.id===id)!.position[2];
 assert.ok(Math.abs((position(b,'SW1')-position(a,'SW1'))-(b.deck-a.deck))<1e-8);
 assert.equal(position(b,'BAT1'),position(a,'BAT1'));
});
test('OLED PCB and viewport are independent parameters with bounds validation',()=>{
 const p=structuredClone(defaultParameters);p.oled.width=29;p.oled.glassX=2.5;validateParameters(p);
 assert.equal(derive(p).assembly.parts.find(x=>x.id==='DISP1')!.size[0],29);
 p.oled.viewWidth=30;assert.throws(()=>validateParameters(p),/viewport/);
});
test('invalid counts and undersized structural rails are rejected',()=>{
 const p=structuredClone(defaultParameters);p.breadboard.rows=30.5;assert.throws(()=>validateParameters(p));
 p.breadboard.rows=30;p.case.screwRail=2;assert.throws(()=>validateParameters(p),/structural/);
});
