const { chromium } = require("playwright");
const fs = require("node:fs");
const path = require("node:path");
const {
  ROOT,
  buildFixtures,
  startServer,
  waitForServer,
  cleanupFixtures,
  openFixturePage,
} = require("./shared");

const SCREENSHOT_DIR = path.join(ROOT, "target", "playwright-vision");
const REFERENCE_DIR = path.join(ROOT, "demo", "riv", "reference");
const HARNESS_DIR = path.join(ROOT, "tests", "playwright");
const PORT = Number(process.env.PLAYWRIGHT_PORT || 8767);

const COMPARISON_FIXTURES = [
  "comparison_trim",
  "comparison_quantize_test",
  "comparison_clip_tests",
  "comparison_official_test",
];

async function renderReference(browser, port, fixture) {
  const refName = fixture.replace("comparison_", "");
  const refPath = path.join(REFERENCE_DIR, `${refName}.riv`);
  if (!fs.existsSync(refPath)) {
    return null;
  }

  const harnessRefPath = path.join(HARNESS_DIR, `ref_${refName}.riv`);
  fs.copyFileSync(refPath, harnessRefPath);

  const page = await openFixturePage(browser, port, `ref_${refName}`, {
    pageOptions: {
      viewport: { width: 512, height: 512 },
      deviceScaleFactor: 2,
    },
  });

  const screenshotPath = path.join(SCREENSHOT_DIR, `${fixture}-reference.png`);
  await page.screenshot({ path: screenshotPath });
  await page.close();
  return screenshotPath;
}

async function renderGenerated(browser, port, fixture) {
  const page = await openFixturePage(browser, port, fixture, {
    pageOptions: {
      viewport: { width: 512, height: 512 },
      deviceScaleFactor: 2,
    },
  });

  const screenshotPath = path.join(SCREENSHOT_DIR, `${fixture}-generated.png`);
  await page.screenshot({ path: screenshotPath });
  await page.close();
  return screenshotPath;
}

function comparePixels(a, b) {
  const bufA = fs.readFileSync(a);
  const bufB = fs.readFileSync(b);
  if (bufA.length !== bufB.length) return 100.0;

  let diff = 0;
  for (let i = 0; i < bufA.length; i++) {
    if (bufA[i] !== bufB[i]) diff++;
  }
  return (diff / bufA.length) * 100;
}

async function main() {
  fs.mkdirSync(SCREENSHOT_DIR, { recursive: true });

  let server;
  let browser;
  const rows = [];

  try {
    buildFixtures(COMPARISON_FIXTURES);
    server = startServer(PORT);
    await waitForServer(PORT);
    browser = await chromium.launch({ headless: true });

    for (const fixture of COMPARISON_FIXTURES) {
      const refPath = await renderReference(browser, PORT, fixture);
      const genPath = await renderGenerated(browser, PORT, fixture);

      if (!refPath || !genPath) {
        rows.push({ fixture, status: "missing", diff: "N/A" });
        continue;
      }

      const diffPercent = comparePixels(refPath, genPath);
      rows.push({ fixture, status: diffPercent < 1.0 ? "pass" : "fail", diff: diffPercent.toFixed(4) });
    }
  } finally {
    if (browser) await browser.close();
    if (server) server.kill("SIGTERM");
    cleanupFixtures(COMPARISON_FIXTURES);
    for (const fixture of COMPARISON_FIXTURES) {
      const refName = fixture.replace("comparison_", "");
      fs.rmSync(path.join(HARNESS_DIR, `ref_${refName}.riv`), { force: true });
    }
  }

  console.log("\nVision Comparison Results");
  console.log("=========================\n");
  console.log("fixture                    | status | diff %");
  console.log("---------------------------+--------+--------");
  for (const row of rows) {
    const pad = " ".repeat(27 - row.fixture.length);
    console.log(`${row.fixture}${pad}| ${row.status.padEnd(6)} | ${row.diff}`);
  }

  console.log("\nNote: A vision model API (e.g. GPT-4V) can be used to");
  console.log("determine semantic likeness for fixtures with non-zero diff.");
  console.log(`Screenshots saved to: ${SCREENSHOT_DIR}`);
}

main().catch((err) => {
  console.error(err.message || err);
  process.exit(1);
});
