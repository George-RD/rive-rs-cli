const { chromium } = require("playwright");
const fs = require("node:fs");
const path = require("node:path");
const {
  ROOT,
  FIXTURES,
  buildFixtures,
  startServer,
  waitForServer,
  cleanupFixtures,
  openFixturePage,
} = require("./shared");

const SCREENSHOT_DIR = path.join(ROOT, "target", "playwright-snapshots");
const PORT = Number(process.env.PLAYWRIGHT_PORT || 8765);

async function main() {
  fs.mkdirSync(SCREENSHOT_DIR, { recursive: true });

  let server;
  let browser;

  try {
    buildFixtures();
    server = startServer(PORT);
    await waitForServer(PORT);
    browser = await chromium.launch({ headless: true });

    for (const fixture of FIXTURES) {
      const page = await openFixturePage(browser, PORT, fixture);
      await page.screenshot({ path: path.join(SCREENSHOT_DIR, `${fixture}.png`) });
      await page.close();
    }
  } finally {
    if (browser) {
      await browser.close();
    }
    if (server) {
      server.kill("SIGTERM");
    }
    cleanupFixtures();
  }
}

main().catch((err) => {
  console.error(err.message || err);
  process.exit(1);
});
