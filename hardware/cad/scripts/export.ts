import {mkdir,readFile,writeFile,rm} from 'node:fs/promises';
import {assemblySTEP} from '../src/step.ts';
import {kernel} from './kernel.ts';
import {buildDesign} from '../src/build-design.ts';
import {defaultParameters,validateParameters} from '../src/parameters.ts';
import {component} from '../src/models/index.ts';
import {physical} from '../src/models/common.ts';
import {fitCoupons} from '../src/fit-coupons.ts';
import {printSolid} from '../src/print.ts';
try{
 await rm('out/fasteners.json',{force:true});await rm('out/push-pins.json',{force:true});
 const index=process.argv.indexOf('--config');let parameters=structuredClone(defaultParameters);
 if(index>=0){if(!process.argv[index+1])throw new Error('--config requires a JSON path');const input=JSON.parse(await readFile(process.argv[index+1],'utf8'));parameters=input.parameters??input;}
 validateParameters(parameters);const design=buildDesign(parameters,await kernel());
 for(const folder of ['out/components','out/enclosure','out/coupons','public/generated']){await mkdir(folder,{recursive:true});}
 // Remove only generator-owned model directories, avoiding stale models in packages.
 for(const folder of ['out/components','out/enclosure']){await rm(folder,{recursive:true});await mkdir(folder,{recursive:true});}
 await writeFile('out/parameters.json',JSON.stringify({schema:2,parameters},null,2));
 await writeFile('out/assembly.json',JSON.stringify(design.assembly,null,2));
 await writeFile('out/fit-report.json',JSON.stringify(design.fit,null,2));
 await writeFile('out/enclosure-report.json',JSON.stringify(design.report,null,2));
 await writeFile('public/generated/case-preview.json',JSON.stringify({assembly:design.assembly,meshes:design.meshes,report:design.report}));
 const metadata=[];
 for(const part of design.assembly.parts.filter(p=>p.kind==='component')){
  const model=component(parameters,part),solid=physical(model);
  await writeFile(`out/components/${part.id}.step`,Buffer.from(await solid.blobSTEP().arrayBuffer()));
  await writeFile(`out/components/${part.id}.stl`,Buffer.from(await solid.blobSTL().arrayBuffer()));
  metadata.push({id:part.id,name:part.name,localOrigin:[0,0,0],position:part.position,size:model.size,anchors:model.anchors,mounts:model.mounts,source:part.source,evidence:part.evidence});
 }
 await writeFile('out/snaps.json',JSON.stringify(design.snaps,null,2));
 await writeFile('out/harness.json',JSON.stringify(design.routes.map(({shape,...r})=>({...r,length:r.points.slice(1).reduce((n,p,i)=>n+Math.hypot(...p.map((v,j)=>v-r.points[i][j])),0)})),null,2));
 await writeFile('out/component-metadata.json',JSON.stringify({units:'mm',components:metadata},null,2));
 for(const e of design.entities.filter(e=>e.category==='printed')){
  await writeFile(`out/enclosure/${e.id}.step`,Buffer.from(await e.shape.blobSTEP().arrayBuffer()));
  await writeFile(`out/enclosure/${e.id}.stl`,Buffer.from(await printSolid(e.id,e.shape).blobSTL().arrayBuffer()));
 }
 const couponRecords=[];
 for(const c of fitCoupons()){
  await writeFile(`out/coupons/${c.id}.stl`,Buffer.from(await c.shape.blobSTL().arrayBuffer()));await writeFile(`out/coupons/${c.id}.step`,Buffer.from(await c.shape.blobSTEP().arrayBuffer()));
  couponRecords.push({id:c.id,description:c.description,openings:c.openings});
 }
 await writeFile('out/coupons/README.json',JSON.stringify({units:'mm',coupons:couponRecords},null,2));
 await writeFile('out/assembly-with-case.step',Buffer.from(await assemblySTEP(design.entities.map(e=>e.shape)).arrayBuffer()));
 await writeFile('out/assembly.step',Buffer.from(await assemblySTEP(design.entities.filter(e=>e.category==='component').map(e=>e.shape)).arrayBuffer()));
 const quote=(s:unknown)=>`"${String(s).replaceAll('"','""')}"`;
 await writeFile('out/bom.csv',[['ID','Name','Category'],...design.entities.map(e=>[e.id,e.name,e.category])].map(row=>row.map(quote).join(',')).join('\n')+'\n');
 await writeFile('out/enclosure/README.txt','PARAMETRIC MECHANICAL CONCEPT\nSTL files are oriented on Z=0; STEP files retain assembly coordinates.\nIntegral clips and battery retainers may require slicer supports. Verify with fit coupons.\nDimensions are editable assumptions, not measured acceptance. Battery circuit remains unselected.\n');
 await writeFile('out/README.txt','PARAMETRIC DESIGN PACKAGE\nComponent STEP/STL files use independent local origins. component-metadata.json records assembly transforms and mounting datums.\nEnclosure STEP files use assembly coordinates; enclosure STL files sit on the print plane.\nparameters.json drives all components, supports, case and harness.\nSee enclosure-report.json and docs/hardware/parametric-design.md for verification limits.\n');
 console.log(JSON.stringify({printed:design.report.printedCount,components:design.report.componentCount,looseFasteners:0,snaps:design.report.snapFeatureCount,wires:design.report.wireCount,interferencePass:design.report.interferencePass,footprintPass:design.report.footprintPass,motionPass:design.report.motionPass,headerGridPass:design.report.headerGridPass,intersections:design.report.intersections}));
 if(!design.report.interferencePass||!design.report.footprintPass||!design.report.motionPass||!design.report.headerGridPass)process.exitCode=1;
 if(process.argv.includes('--release')){console.error('Manufacturing release requires physical measurements and a validated battery circuit.');process.exitCode=1;}
}catch(error){console.error(error instanceof Error?error.message:String(error));process.exitCode=1;}
