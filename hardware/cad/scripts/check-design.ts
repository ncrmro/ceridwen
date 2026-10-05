import {kernel} from './kernel.ts';import {buildDesign} from '../src/build-design.ts';import {defaultParameters} from '../src/parameters.ts';
const design=buildDesign(defaultParameters,await kernel());
console.log(JSON.stringify(design.report,null,2));
process.exitCode=design.report.interferencePass&&design.report.footprintPass&&design.report.motionPass&&design.report.headerGridPass?0:1;
