import {derive} from './layout.ts';
import {defaultParameters,type Parameters} from './parameters.ts';
/** Millimetres; right-handed coordinates: X right, Y toward back, Z up. */
export type Vec3 = [number, number, number];
export type Evidence = 'provisional' | 'drawing' | 'measured';
export interface Part {
  id: string; name: string; size: Vec3; position: Vec3; color: string;
  evidence: Evidence; source: string; missing: string[];
  kind: 'component' | 'clearance';
}
export interface Assembly {
  schema: 1; parameters?: Parameters; name: string; minimumGap: number; parts: Part[];
  /** Only explicitly named mating pairs can touch/overlap. */
  mates: [string, string][];
  unresolved: string[];
}
export const initialAssembly: Assembly = derive(defaultParameters).assembly;

export function validateAssembly(value: unknown): asserts value is Assembly {
  const a = value as Assembly;
  if (!a || a.schema !== 1 || typeof a.name !== 'string' ||
      !Number.isFinite(a.minimumGap) || a.minimumGap < 0 || !Array.isArray(a.parts) ||
      !a.parts.length || !Array.isArray(a.mates) || !Array.isArray(a.unresolved) ||
      !a.unresolved.every(x => typeof x === 'string')) throw new Error('Invalid assembly schema');
  const ids = new Set<string>();
  for (const p of a.parts) {
    if (!p || !/^[A-Z][A-Z0-9_-]*$/.test(p.id) || ids.has(p.id) ||
        typeof p.name !== 'string' || typeof p.source !== 'string' ||
        !/^#[0-9a-f]{6}$/i.test(p.color) ||
        !['provisional', 'drawing', 'measured'].includes(p.evidence) ||
        !['component', 'clearance'].includes(p.kind) || !Array.isArray(p.missing) ||
        !p.missing.every(x => typeof x === 'string') ||
        ![p.size, p.position].every(v => Array.isArray(v) && v.length === 3 && v.every(Number.isFinite)) ||
        p.size.some(n => n <= 0 || n > 1000) || p.position.some(n => Math.abs(n) > 1000))
      throw new Error(`Invalid part: ${p?.id}`);
    ids.add(p.id);
  }
  for (const pair of a.mates) {
    if (!Array.isArray(pair) || pair.length !== 2 || pair[0] === pair[1] || !pair.every(id => ids.has(id)))
      throw new Error('Invalid mating pair');
  }
}
