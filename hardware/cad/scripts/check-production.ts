import {preview} from 'vite';
import {chromium} from '@playwright/test';
const server=await preview({configFile:false,preview:{host:'127.0.0.1',port:0,strictPort:false}});
const address=server.httpServer.address();if(!address||typeof address==='string')throw new Error('No preview listener');
const browser=await chromium.launch({executablePath:process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH,args:['--enable-unsafe-swiftshader']});
try{
 const page=await browser.newPage({viewport:{width:1400,height:900}});const errors:string[]=[];page.on('pageerror',e=>errors.push(e.message));
 await page.goto(`http://127.0.0.1:${address.port}`);
 await page.waitForFunction(()=>document.querySelector('#report')?.textContent?.includes('Digital assembly: PASS'),{},{timeout:90000});
 await page.screenshot({path:'out/production-closed.png'});
 const step=page.waitForEvent('download',{timeout:60000});await page.getByRole('button',{name:'Assembly STEP',exact:true}).click();
 const file=await step;if(await file.failure())throw new Error('STEP download failed');
 await file.saveAs('out/browser-export.step');
 if(errors.length)throw new Error(errors.join('\n'));
 console.log('Production worker, assembled view and STEP download passed.');
}catch(error){console.error(error);process.exitCode=1;}finally{await browser.close();await new Promise<void>((resolve,reject)=>server.httpServer.close(e=>e?reject(e):resolve()));}
