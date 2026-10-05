import type {OpenCascadeInstance} from 'replicad-opencascadejs';
import {readFile} from 'node:fs/promises';
import {fileURLToPath} from 'node:url';import {dirname} from 'node:path';import {createRequire} from 'node:module';
import {setOC} from 'replicad';
export async function kernel():Promise<OpenCascadeInstance>{
 const modulePath=fileURLToPath(import.meta.resolve('replicad-opencascadejs/src/replicad_single.js'));
 const source=await readFile(modulePath,'utf8');
 const factory=new Function('require','__dirname','__filename',source.replace('export default Module;','return Module;'))(createRequire(import.meta.url),dirname(modulePath),modulePath);
 const wasmPath=fileURLToPath(import.meta.resolve('replicad-opencascadejs/src/replicad_single.wasm'));
 const oc=await factory({wasmBinary:await readFile(wasmPath),locateFile:()=>wasmPath});setOC(oc);return oc;
}
