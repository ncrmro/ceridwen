import { test } from 'node:test';
import assert from 'node:assert/strict';
import { initialAssembly, validateAssembly } from './assembly.ts';
import { fitReport } from './fit.ts';

test('planning geometry can clear while manufacturing release stays blocked', () => {
  const report = fitReport(initialAssembly);
  assert.equal(report.geometryPass, true, JSON.stringify(report.violations));
  assert.equal(report.releaseReady, false);
  assert.ok(report.blockers.some(s => s.includes('Power module')));
});
test('moving a component into the battery reports physical collision', () => {
  const a = structuredClone(initialAssembly);
  a.parts.find(p => p.id === 'PWR2')!.position = [...a.parts.find(p => p.id === 'BAT1')!.position];
  const report = fitReport(a);
  assert.ok(report.violations.some(v => v.a === 'BAT1' && v.b === 'PWR2' && v.volume > 0));
});
test('a clear part body can still obstruct USB access', () => {
  const a = structuredClone(initialAssembly);
  a.parts.find(p => p.id === 'PWR2')!.position = [...a.parts.find(p => p.id === 'USB1')!.position];
  assert.ok(fitReport(a).violations.some(v => v.kind === 'blocked clearance' && v.a === 'PWR2' && v.b === 'USB1'));
});
test('invalid imported geometry is rejected before generating solids', () => {
  const a = structuredClone(initialAssembly);
  a.parts[0].size[0] = -1;
  assert.throws(() => validateAssembly(a));
});
test('changing evidence labels alone cannot bypass unresolved work', () => {
  const a = structuredClone(initialAssembly);
  a.parts.forEach(p => p.evidence = 'measured');
  a.parts.forEach(p => p.missing = []);
  a.unresolved = [];
  assert.equal(fitReport(a).releaseReady, false);
});

test('an isolated part outside the breadboard still fails fit', () => {
  const a = structuredClone(initialAssembly);
  a.parts.find(p => p.id === 'BAT1')!.position = [100, 5, 14.5];
  const r = fitReport(a);
  assert.equal(r.violations.length, 0);
  assert.equal(r.geometryPass, false);
  assert.deepEqual(r.footprint.violations.map(v => [v.id, v.axis]), [['BAT1', 'X']]);
});
test('negative edge and board datum changes are checked', () => {
  const a = structuredClone(initialAssembly);
  a.parts.find(p => p.id === 'BAT1')!.position[0] = -0.1;
  assert.equal(fitReport(a).footprint.pass, false);
  a.parts.find(p => p.id === 'BAT1')!.position[0] = 0;
  assert.equal(fitReport(a).footprint.pass, true);
  a.parts.find(p => p.id === 'BB1')!.size[0] = 70;
  assert.equal(fitReport(a).footprint.pass, false);
});
