const { chromium } = require("playwright");
const fs = require("node:fs");
const path = require("node:path");
const zlib = require("node:zlib");
const {
  ROOT,
  FIXTURES,
  buildFixtures,
  startServer,
  waitForServer,
  cleanupFixtures,
  openFixturePage,
} = require("./shared");

const CURRENT_DIR = path.join(ROOT, "target", "playwright-visual");
const BASELINE_DIR = path.join(ROOT, "tests", "playwright", "baselines");
const PORT = Number(process.env.PLAYWRIGHT_PORT || 8766);
const THRESHOLD_PERCENT = Number(process.env.VISUAL_DIFF_THRESHOLD || "1.0");

const ARTBOARD_PLANS = {
  icon_set: ["Home", "Settings", "Profile"],
  multi_artboard: ["Screen A", "Screen B"],
};

const SHOT_FRAMES = {
  animation: [0, 30, 60],
  cubic_easing: [0, 15, 30, 45, 60],
  multi_artboard: [0, 30],
  color_animation: [0, 30, 60],
  loop_animation: [0, 30],
  game_hud: [0, 60, 120],
  mascot: [0, 30, 60],
};

// Per-fixture or per-fixture+frame threshold overrides (animation non-determinism)
const THRESHOLD_OVERRIDES = {
  "multi_artboard@f30": 5.0,
  "color_animation@f30": 20.0,
  "color_animation@f60": 20.0,
  "loop_animation": 5.0,
};

function readUInt32BE(buffer, offset) {
  return (
    (buffer[offset] << 24) |
    (buffer[offset + 1] << 16) |
    (buffer[offset + 2] << 8) |
    buffer[offset + 3]
  ) >>> 0;
}

function parsePng(buffer) {
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  if (!buffer.subarray(0, 8).equals(signature)) {
    throw new Error("invalid PNG signature");
  }

  let offset = 8;
  let width = 0;
  let height = 0;
  let bitDepth = 0;
  let colorType = 0;
  let interlaceMethod = 0;
  const idatParts = [];

  while (offset < buffer.length) {
    const length = readUInt32BE(buffer, offset);
    offset += 4;
    const chunkType = buffer.subarray(offset, offset + 4).toString("ascii");
    offset += 4;
    const chunkData = buffer.subarray(offset, offset + length);
    offset += length;
    offset += 4;

    if (chunkType === "IHDR") {
      width = readUInt32BE(chunkData, 0);
      height = readUInt32BE(chunkData, 4);
      bitDepth = chunkData[8];
      colorType = chunkData[9];
      interlaceMethod = chunkData[12];
    } else if (chunkType === "IDAT") {
      idatParts.push(chunkData);
    } else if (chunkType === "IEND") {
      break;
    }
  }

  if (!width || !height) {
    throw new Error("missing PNG dimensions");
  }
  if (bitDepth !== 8 || (colorType !== 6 && colorType !== 2)) {
    throw new Error(`unsupported PNG format (bitDepth=${bitDepth}, colorType=${colorType})`);
  }
  if (interlaceMethod !== 0) {
    throw new Error("unsupported interlaced PNG");
  }

  const compressed = Buffer.concat(idatParts);
  const raw = zlib.inflateSync(compressed);
  const bytesPerPixel = colorType === 6 ? 4 : 3;
  const stride = width * bytesPerPixel;
  const expectedLength = height * (1 + stride);
  if (raw.length !== expectedLength) {
    throw new Error(`unexpected PNG data length ${raw.length} != ${expectedLength}`);
  }

  const decoded = Buffer.alloc(width * height * bytesPerPixel);
  let src = 0;
  let dst = 0;

  for (let y = 0; y < height; y += 1) {
    const filterType = raw[src];
    src += 1;

    if (filterType === 0) {
      for (let x = 0; x < stride; x += 1) {
        decoded[dst + x] = raw[src + x];
      }
    } else if (filterType === 1) {
      for (let x = 0; x < stride; x += 1) {
        const left = x >= bytesPerPixel ? decoded[dst + x - bytesPerPixel] : 0;
        decoded[dst + x] = (raw[src + x] + left) & 0xff;
      }
    } else if (filterType === 2) {
      for (let x = 0; x < stride; x += 1) {
        const up = y > 0 ? decoded[dst + x - stride] : 0;
        decoded[dst + x] = (raw[src + x] + up) & 0xff;
      }
    } else if (filterType === 3) {
      for (let x = 0; x < stride; x += 1) {
        const left = x >= bytesPerPixel ? decoded[dst + x - bytesPerPixel] : 0;
        const up = y > 0 ? decoded[dst + x - stride] : 0;
        decoded[dst + x] = (raw[src + x] + ((left + up) >> 1)) & 0xff;
      }
    } else if (filterType === 4) {
      for (let x = 0; x < stride; x += 1) {
        const left = x >= bytesPerPixel ? decoded[dst + x - bytesPerPixel] : 0;
        const up = y > 0 ? decoded[dst + x - stride] : 0;
        const upLeft = y > 0 && x >= bytesPerPixel ? decoded[dst + x - stride - bytesPerPixel] : 0;
        const p = left + up - upLeft;
        const pa = Math.abs(p - left);
        const pb = Math.abs(p - up);
        const pc = Math.abs(p - upLeft);
        const predictor = pa <= pb && pa <= pc ? left : pb <= pc ? up : upLeft;
        decoded[dst + x] = (raw[src + x] + predictor) & 0xff;
      }
    } else {
      throw new Error(`unsupported PNG filter type ${filterType}`);
    }

    src += stride;
    dst += stride;
  }

  if (colorType === 6) {
    return { width, height, data: decoded };
  }

  const rgba = Buffer.alloc(width * height * 4);
  for (let i = 0, j = 0; i < decoded.length; i += 3, j += 4) {
    rgba[j] = decoded[i];
    rgba[j + 1] = decoded[i + 1];
    rgba[j + 2] = decoded[i + 2];
    rgba[j + 3] = 255;
  }
  return { width, height, data: rgba };
}

function comparePngPixels(actualPath, baselinePath) {
  const actual = parsePng(fs.readFileSync(actualPath));
  const baseline = parsePng(fs.readFileSync(baselinePath));
  if (actual.width !== baseline.width || actual.height !== baseline.height) {
    throw new Error(
      `dimension mismatch ${actual.width}x${actual.height} vs ${baseline.width}x${baseline.height}`
    );
  }

  let differentPixels = 0;
  const pixels = actual.width * actual.height;
  for (let i = 0; i < pixels; i += 1) {
    const base = i * 4;
    if (
      actual.data[base] !== baseline.data[base] ||
      actual.data[base + 1] !== baseline.data[base + 1] ||
      actual.data[base + 2] !== baseline.data[base + 2] ||
      actual.data[base + 3] !== baseline.data[base + 3]
    ) {
      differentPixels += 1;
    }
  }

  return (differentPixels / pixels) * 100;
}

function printSummary(rows) {
  const nameWidth = Math.max("fixture".length, ...rows.map((row) => row.name.length));
  const statusWidth = Math.max("status".length, ...rows.map((row) => row.status.length));
  const diffWidth = Math.max("diff %".length, ...rows.map((row) => row.diffText.length));

  const header =
    `${"fixture".padEnd(nameWidth)} | ${"status".padEnd(statusWidth)} | ${"diff %".padStart(diffWidth)}`;
  const divider = `${"-".repeat(nameWidth)}-+-${"-".repeat(statusWidth)}-+-${"-".repeat(diffWidth)}`;
  console.log(header);
  console.log(divider);
  for (const row of rows) {
    console.log(`${row.name.padEnd(nameWidth)} | ${row.status.padEnd(statusWidth)} | ${row.diffText.padStart(diffWidth)}`);
  }
}

function shotFramesForFixture(fixture) {
  return SHOT_FRAMES[fixture] || [0];
}

function thresholdForShot(fixture, frame) {
  return THRESHOLD_OVERRIDES[`${fixture}@f${frame}`]
    ?? THRESHOLD_OVERRIDES[fixture]
    ?? THRESHOLD_PERCENT;
}

async function advanceFrames(page, frames) {
  if (frames <= 0) {
    return;
  }
  await page.evaluate((count) => {
    return new Promise((resolve) => {
      let remaining = count;
      const tick = () => {
        if (remaining <= 0) {
          resolve(true);
          return;
        }
        remaining -= 1;
        window.requestAnimationFrame(tick);
      };
      window.requestAnimationFrame(tick);
    });
  }, frames);
}

function sanitizeArtboardName(name) {
  return name.toLowerCase().replace(/\s+/g, "_");
}

async function mountControlledRive(page, fixture, artboard) {
  const result = await page.evaluate(async ({ file, artboard: ab }) => {
    const originalCanvas = document.getElementById("canvas");
    if (!originalCanvas) {
      return { ok: false, error: "missing canvas" };
    }
    originalCanvas.style.display = "none";

    const controlledCanvas = document.createElement("canvas");
    controlledCanvas.id = "canvas-controlled";
    controlledCanvas.style.width = "100%";
    controlledCanvas.style.height = "100%";
    controlledCanvas.width = originalCanvas.width;
    controlledCanvas.height = originalCanvas.height;
    originalCanvas.parentElement.appendChild(controlledCanvas);

    try {
      await new Promise((resolve, reject) => {
        const opts = {
          src: file,
          canvas: controlledCanvas,
          autoplay: false,
          onLoad: resolve,
          onLoadError: (error) => reject(new Error(String(error || "rive load error"))),
        };
        if (ab) {
          opts.artboard = ab;
        }
        window.__VISUAL_RIVE = new rive.Rive(opts);
      });
      return { ok: true, error: "" };
    } catch (error) {
      return { ok: false, error: String(error || "unknown error") };
    }
  }, { file: `${fixture}.riv`, artboard: artboard || null });

  if (!result.ok) {
    throw new Error(`${fixture}.riv failed to mount controlled runtime: ${result.error}`);
  }
}

function baselineName(fixture, frame, artboard) {
  if (artboard) {
    return `${fixture}-${sanitizeArtboardName(artboard)}-f${frame}.png`;
  }
  return `${fixture}-f${frame}.png`;
}

function artboardPlansForFixture(fixture) {
  return ARTBOARD_PLANS[fixture] || [null];
}

async function main() {
  const update = process.argv.includes("--update");
  fs.mkdirSync(CURRENT_DIR, { recursive: true });
  fs.mkdirSync(BASELINE_DIR, { recursive: true });

  let server;
  let browser;
  const rows = [];
  let hasFailures = false;
  let hasNewBaselines = false;

  try {
    buildFixtures();
    server = startServer(PORT);
    await waitForServer(PORT);
    browser = await chromium.launch({
      headless: true,
      args: ["--disable-gpu", "--deterministic-mode", "--run-all-compositor-stages-before-draw"],
    });

    for (const fixture of FIXTURES) {
      for (const artboard of artboardPlansForFixture(fixture)) {
        for (const frame of shotFramesForFixture(fixture)) {
          const page = await openFixturePage(browser, PORT, fixture, {
            artboard,
            pageOptions: { viewport: { width: 512, height: 512 }, deviceScaleFactor: 2 },
          });

          await mountControlledRive(page, fixture, artboard);

          if (frame > 0) {
            await page.evaluate(() => {
              if (!window.__VISUAL_RIVE || typeof window.__VISUAL_RIVE.play !== "function") {
                return false;
              }
              window.__VISUAL_RIVE.play();
              return true;
            });
          }

          await advanceFrames(page, frame);
          const name = baselineName(fixture, frame, artboard);
          const label = artboard ? `${fixture}[${artboard}]@f${frame}` : `${fixture}@f${frame}`;
          const currentPath = path.join(CURRENT_DIR, name);
          const baselinePath = path.join(BASELINE_DIR, name);
          await page.screenshot({ path: currentPath });

          if (update) {
            fs.copyFileSync(currentPath, baselinePath);
            rows.push({ name: label, status: "updated", diffText: "0.0000" });
            await page.close();
            continue;
          }

          if (!fs.existsSync(baselinePath)) {
            rows.push({ name: label, status: "missing", diffText: "-" });
            hasNewBaselines = true;
            await page.close();
            continue;
          }

          const diffPercent = comparePngPixels(currentPath, baselinePath);
          const pass = diffPercent <= thresholdForShot(fixture, frame);
          rows.push({
            name: label,
            status: pass ? "pass" : "fail",
            diffText: diffPercent.toFixed(4),
          });
          if (!pass) {
            hasFailures = true;
          }

          await page.close();
        }
      }
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

  printSummary(rows);
  console.log(`threshold: ${THRESHOLD_PERCENT.toFixed(4)}%`);
  if (hasNewBaselines) {
    console.log("created new baselines; manual review required before relying on comparisons");
  }

  if (hasFailures || hasNewBaselines) {
    process.exit(1);
  }
}

main().catch((err) => {
  console.error(err.message || err);
  process.exit(1);
});
