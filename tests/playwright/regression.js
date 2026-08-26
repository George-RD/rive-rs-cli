const { chromium } = require("playwright");
const fs = require("node:fs");
const path = require("node:path");
const {
  ROOT,
  HARNESS_DIR,
  FIXTURES,
  KNOWN_RUNTIME_GAPS,
  run,
  buildFixtures,
  startServer,
  waitForServer,
  cleanupFixtures,
  openFixturePage,
} = require("./shared");

const SCREENSHOT_DIR = path.join(ROOT, "target", "playwright-snapshots");
const PORT = Number(process.env.PLAYWRIGHT_PORT || 8765);
const AUTHORING_FIXTURE = "authoring_typed_motion";
const RUNTIME_FIXTURES = [...FIXTURES, AUTHORING_FIXTURE];

function buildAuthoringFixture() {
  run("cargo", [
    "run",
    "--quiet",
    "--",
    "authoring",
    "compile",
    path.join(ROOT, "examples", "authoring", "typed-motion.v0.json"),
    "-o",
    path.join(HARNESS_DIR, `${AUTHORING_FIXTURE}.riv`),
  ]);
}

async function main() {
  fs.mkdirSync(SCREENSHOT_DIR, { recursive: true });

  let server;
  let browser;

  try {
    buildFixtures();
    buildAuthoringFixture();
    server = startServer(PORT);
    await waitForServer(PORT);
    browser = await chromium.launch({ headless: true });

    for (const fixture of RUNTIME_FIXTURES) {
      if (KNOWN_RUNTIME_GAPS.has(fixture)) {
        console.log(`SKIP ${fixture} (known runtime gap)`);
        continue;
      }
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
    cleanupFixtures(RUNTIME_FIXTURES);
  }
}

main().catch((err) => {
  console.error(err.message || err);
  process.exit(1);
});
