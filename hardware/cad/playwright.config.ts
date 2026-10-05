import { defineConfig } from '@playwright/test';
import { readFileSync } from 'node:fs';
async function serverURL(): Promise<string> {
  if (process.env.CAD_URL) return process.env.CAD_URL;
  const deadline = Date.now() + 15000;
  while (Date.now() < deadline) {
    try {
      const record = JSON.parse(readFileSync('.dev-server.json', 'utf8'));
      if (record.cwd === process.cwd() && (await fetch(record.url)).ok) return record.url;
    } catch { /* devenv may still be starting the server */ }
    await new Promise(resolve => setTimeout(resolve, 200));
  }
  throw new Error('CAD server is not ready; run devenv up cad');
}
const baseURL = await serverURL();
export default defineConfig({
  testDir: './tests',
  use: { baseURL, viewport: { width: 1400, height: 900 },
    launchOptions: { executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH,
      args: ['--enable-unsafe-swiftshader'] } },
});
