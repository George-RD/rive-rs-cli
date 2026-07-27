const { chromium } = require('playwright');

const { spawn } = require('node:child_process');
const http = require('node:http');
const path = require('node:path');

const ROOT = path.resolve(__dirname, '..', '..');
const PORT = 3021;

function waitForServer(server, timeoutMs = 120000) {
  const deadline = Date.now() + timeoutMs;

  return new Promise((resolve, reject) => {
    const check = () => {
      if (server.exitCode !== null) {
        reject(new Error(`demo server exited with code ${server.exitCode}`));
        return;
      }
      if (Date.now() >= deadline) {
        reject(new Error(`demo server did not start on port ${PORT}`));
        return;
      }

      const request = http.get({ hostname: '127.0.0.1', port: PORT, path: '/', timeout: 2000 }, (response) => {
        response.resume();
        if (response.statusCode === 200) {
          resolve();
        } else {
          setTimeout(check, 100);
        }
      });
      request.on('error', () => setTimeout(check, 100));
      request.on('timeout', () => request.destroy());
    };
    check();
  });
}

(async () => {
function stopServer(server) {
  if (!server || server.exitCode !== null) {
    return Promise.resolve();
  }

  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      if (server.exitCode !== null) {
        resolve();
        return;
      }
      server.kill('SIGKILL');
      setTimeout(() => {
        if (server.exitCode === null) {
          reject(new Error('demo server did not exit after SIGKILL'));
        }
      }, 1000);
    }, 5000);
    server.once('exit', () => {
      clearTimeout(timer);
      resolve();
    });
    server.kill('SIGTERM');
  });
}

  const errors = [];
  let browser;
  let server;

  try {
    server = spawn('node', ['demo/serve.js'], {
      cwd: ROOT,
      stdio: 'inherit',
    });
    await waitForServer(server);

    browser = await chromium.launch();
    const page = await browser.newPage();

    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });
    page.on('pageerror', err => {
      errors.push(err.message);
    });

    await page.goto(`http://127.0.0.1:${PORT}/`, { waitUntil: 'networkidle', timeout: 60000 });
    await page.waitForTimeout(10000);

    if (errors.length > 0) {
      throw new Error(`Demo validation failed: ${errors.length} console error(s) found:\n${[...new Set(errors)].map(error => `  - ${error.substring(0, 400)}`).join('\n')}`);
    }

    console.log('Demo validation passed: 0 console errors');
  } finally {
    if (browser) {
      await browser.close();
    }
    if (server) {
      await stopServer(server);
    }
  }
})().catch(err => {
  console.error(err.message || err);
  process.exit(1);
});
