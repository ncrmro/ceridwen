import { defineConfig } from 'vite';
import { writeFileSync } from 'node:fs';

export default defineConfig({
  server: { allowedHosts: true, host: '0.0.0.0', port: 4310 },
  plugins: [{
    name: 'record-actual-port',
    configureServer(server) {
      server.httpServer?.once('listening', () => {
        const address = server.httpServer?.address();
        if (address && typeof address !== 'string') {
          writeFileSync('.dev-server.json', JSON.stringify({
            url: `http://localhost:${address.port}`, pid: process.pid, cwd: process.cwd(),
          }, null, 2));
        }
      });
    },
  }],
});
