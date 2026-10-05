import { validateAssembly, type Assembly, type Part, type Vec3 } from './assembly.ts';

export function boxGap(a: Part, b: Part): number {
  const separation = a.size.map((size, i) => Math.max(
    b.position[i] - (a.position[i] + size), a.position[i] - (b.position[i] + b.size[i]), 0));
  return Math.hypot(...separation);
}
export function overlapVolume(a: Part, b: Part): number {
  return a.size.reduce((v, size, i) => v * Math.max(0,
    Math.min(a.position[i] + size, b.position[i] + b.size[i]) - Math.max(a.position[i], b.position[i])), 1);
}
export function fitReport(assembly: Assembly) {
  validateAssembly(assembly);
  const violations: { a: string; b: string; gap: number; volume: number; kind: string }[] = [];
  for (let i = 0; i < assembly.parts.length; i++) {
    for (const b of assembly.parts.slice(i + 1)) {
      const a = assembly.parts[i];
      if (a.kind === 'clearance' && b.kind === 'clearance') continue;
      if (assembly.mates.some(pair => pair.includes(a.id) && pair.includes(b.id))) continue;
      const gap = boxGap(a, b), volume = overlapVolume(a, b);
      if (volume > 1e-6 || gap + 1e-6 < assembly.minimumGap)
        violations.push({ a: a.id, b: b.id, gap, volume,
          kind: a.kind === 'clearance' || b.kind === 'clearance' ? 'blocked clearance' : 'physical interference' });
    }
  }
  const components = assembly.parts.filter(p => p.kind === 'component');
  // The external USB cable may leave the board's footprint; installed bodies may not.
  const board = components.find(p => p.id === 'BB1');
  if (!board) throw new Error('BB1 breadboard is required as the footprint datum');
  const footprintViolations = components.filter(p => p.id !== 'BB1').flatMap(p =>
    [0, 1].filter(axis => p.position[axis] < board.position[axis] - 1e-6 ||
      p.position[axis] + p.size[axis] > board.position[axis] + board.size[axis] + 1e-6)
      .map(axis => ({ id: p.id, axis: ['X', 'Y'][axis],
        minimum: p.position[axis], maximum: p.position[axis] + p.size[axis] })));
  const geometryPass = violations.length === 0 && footprintViolations.length === 0;
  const min = [0, 1, 2].map(i => Math.min(...components.map(p => p.position[i]))) as Vec3;
  const max = [0, 1, 2].map(i => Math.max(...components.map(p => p.position[i] + p.size[i]))) as Vec3;
  const blockers = ['This report checks conservative component envelopes. See enclosure-report.json for solid, motion and header-grid checks; physical fit remains unverified.', ...assembly.unresolved, ...assembly.parts.flatMap(p => [
    ...(p.evidence === 'provisional' ? [`${p.id}: provisional dimensions`] : []),
    ...p.missing.map(item => `${p.id}: ${item}`),
  ])];
  return {
    schema: 1, units: 'mm', method: 'Axis-aligned envelope clearance, not detailed part/assembly acceptance',
    geometryPass, releaseReady: geometryPass && blockers.length === 0,
    footprint: { datum: board.id, size: board.size.slice(0, 2), pass: footprintViolations.length === 0, violations: footprintViolations },
    minimumGap: assembly.minimumGap, envelope: { min, max, size: max.map((n, i) => n - min[i]) },
    violations, blockers, allowedMates: assembly.mates,
  };
}
