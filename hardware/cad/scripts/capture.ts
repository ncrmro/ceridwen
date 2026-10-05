/** Capture the built revision with one consistent default parameter set. */
import {preview} from 'vite';
import {chromium} from '@playwright/test';
import {mkdir,readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const server=await preview({configFile:false,preview:{host:'127.0.0.1',port:0,strictPort:false}});
const address=server.httpServer.address();if(!address||typeof address==='string')throw new Error('No preview listener');
const browser=await chromium.launch({executablePath:process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH,args:['--enable-unsafe-swiftshader']});
try {
 await mkdir('out/captures',{recursive:true});
 const page=await browser.newPage({viewport:{width:1440,height:1000}}),errors:string[]=[];
 page.on('pageerror',e=>errors.push(e.message));
 page.on('requestfailed',r=>errors.push(`${r.url()}: ${r.failure()?.errorText}`));
 await page.goto(`http://127.0.0.1:${address.port}`);
 await page.waitForFunction(()=>document.querySelector('#report')?.textContent?.includes('Digital assembly: PASS'),{},{timeout:120000});
 if(await page.evaluate(()=>getComputedStyle(document.querySelector('main')!).display)!=='grid')throw new Error('Styles did not load');
 const captures:{file:string;capturedAt:string;viewport:{width:number;height:number};mode:string}[]=[];
 const shot=async(file:string,mode:string)=>{
  await page.screenshot({path:`out/captures/${file}`,fullPage:true});
  captures.push({file,capturedAt:new Date().toISOString(),viewport:page.viewportSize()!,mode});
 };
 await shot('desktop.png','closed');
 await page.locator('#xray').check();await page.locator('#explode').check();await page.getByRole('button',{name:'Fit view'}).click();
 await page.waitForTimeout(250);await shot('exploded.png','transparent exploded');
 await page.locator('#explode').uncheck();await page.locator('#xray').uncheck();
 await page.setViewportSize({width:390,height:844});await page.getByRole('button',{name:'Fit view'}).click();
 await page.waitForTimeout(250);await shot('mobile.png','closed');
 const report=await page.locator('#report').textContent();
 if(errors.length)throw new Error(errors.join('\n'));
 const parameters=await readFile('out/parameters.json');
 await writeFile('out/captures/capture.json',JSON.stringify({captures,parameterSha256:createHash('sha256').update(parameters).digest('hex'),report,browser:browser.version()},null,2));
 console.log('Captured desktop, exploded and mobile views from one default parameter set.');
} finally {await browser.close();await new Promise<void>((resolve,reject)=>server.httpServer.close(e=>e?reject(e):resolve()));}
