import { makeBox, makeCylinder, makeCompound, type Shape3D } from 'replicad';
import type { Vec3 } from '../assembly.ts';
export type Numbers = Record<string, number>;
export interface Piece { name: string; color: string; shape: Shape3D; cosmetic?: boolean }
export interface Model { size: Vec3; pieces: Piece[]; anchors: Record<string, Vec3>; mounts: Vec3[] }
export const box = (p: Vec3, s: Vec3) => makeBox(p, p.map((v, i) => v + s[i]) as Vec3);
export const pin = (x: number, y: number, z: number, r: number, h: number) => makeCylinder(r, h, [x, y, z]);
export const compound = (shapes: Shape3D[]) => makeCompound(shapes) as Shape3D;
export const body = (name: string, color: string, shape: Shape3D, cosmetic = false): Piece => ({name, color, shape, cosmetic});
export function physical(model: Model) { return compound(model.pieces.filter(p => !p.cosmetic).map(p => p.shape.clone())); }
