const { chromium } = require("playwright");
const { spawnSync, spawn } = require("node:child_process");
const fs = require("node:fs");
const http = require("node:http");
const path = require("node:path");
const zlib = require("node:zlib");

const ROOT = path.resolve(__dirname, "..", "..");
const HARNESS_DIR = path.join(ROOT, "tests", "playwright");
const OUT_DIR = path.join(ROOT, "target", "playwright-riv");
const CURRENT_DIR = path.join(ROOT, "target", "playwright-visual");
const BASELINE_DIR = path.join(ROOT, "tests", "playwright", "baselines");
const FIXTURES = ["minimal", "shapes", "animation", "state_machine", "path", "cubic_easing", "trim_path"];
const PORT = Number(process.env.PLAYWRIGHT_PORT || 8766);
const THRESHOLD_PERCENT = Number(process.env.VISUAL_DIFF_THRESHOLD || "0.1");

function run(command, args, cwd = ROOT) {
  const result = spawnSync(command, args, { cwd, stdio: "inherit" });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(" ")} failed with exit code ${result.status}`);
  }
}

function wait(delayMs) {
  return new Promise((resolve) => setTimeout(resolve, delayMs));
}

async function waitForServer(port, timeoutMs = 5000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      await new Promise((resolve, reject) => {
        const request = http.get(
          { hostname: "127.0.0.1", port, path: "/harness.html" },
          (response) => {
            response.resume();
            if (response.statusCode === 200) {
              resolve();
            } else {
              reject(new Error(`server returned status ${response.statusCode}`));
            }
          }
        );
        request.on("error", reject);
      });
      return;
    } catch {
      await wait(100);
    }
  }
  throw new Error(`server did not start on port ${port}`);
}

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

function shotPlanForFixture(fixture) {
  if (fixture === "animation") {
    return [
      { frame: 0, waitFrames: 0 },
      { frame: 30, waitFrames: 30 },
      { frame: 60, waitFrames: 60 },
    ];
  }
  return [{ frame: 0, waitFrames: 0 }];
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

async function mountControlledRive(page, fixture) {
  const result = await page.evaluate(async (file) => {
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
        window.__VISUAL_RIVE = new rive.Rive({
          src: file,
          canvas: controlledCanvas,
          autoplay: false,
          onLoad: resolve,
          onLoadError: (error) => reject(new Error(String(error || "rive load error"))),
        });
      });
      return { ok: true, error: "" };
    } catch (error) {
      return { ok: false, error: String(error || "unknown error") };
    }
  }, `${fixture}.riv`);

  if (!result.ok) {
    throw new Error(`${fixture}.riv failed to mount controlled runtime: ${result.error}`);
  }
}

function baselineName(fixture, frame) {
  return `${fixture}-f${frame}.png`;
}

async function main() {
  const update = process.argv.includes("--update");
  fs.mkdirSync(OUT_DIR, { recursive: true });
  fs.mkdirSync(CURRENT_DIR, { recursive: true });
  fs.mkdirSync(BASELINE_DIR, { recursive: true });

  for (const fixture of FIXTURES) {
    const input = path.join(ROOT, "tests", "fixtures", `${fixture}.json`);
    const output = path.join(OUT_DIR, `${fixture}.riv`);
    run("cargo", ["run", "--quiet", "--", "generate", input, "-o", output]);
    fs.copyFileSync(output, path.join(HARNESS_DIR, `${fixture}.riv`));
  }

  let server;
  let browser;
  const rows = [];
  let hasFailures = false;
  let hasNewBaselines = false;

  try {
    server = spawn("python3", ["-m", "http.server", String(PORT), "--bind", "127.0.0.1"], {
      cwd: HARNESS_DIR,
      stdio: "ignore",
    });

    await waitForServer(PORT);
    browser = await chromium.launch({
      headless: true,
      args: ["--disable-gpu", "--deterministic-mode", "--run-all-compositor-stages-before-draw"],
    });

    for (const fixture of FIXTURES) {
      for (const shot of shotPlanForFixture(fixture)) {
        const page = await browser.newPage({ viewport: { width: 512, height: 512 } });
        const runtimeErrors = [];
        page.on("pageerror", (err) => runtimeErrors.push(String(err)));
        page.on("console", (msg) => {
          if (msg.type() === "error") {
            runtimeErrors.push(msg.text());
          }
        });

        await page.goto(`http://127.0.0.1:${PORT}/harness.html?file=${fixture}.riv`, {
          waitUntil: "domcontentloaded",
        });
        await page.waitForFunction(() => window.__RIVE_OK || window.__RIVE_ERROR, {
          timeout: 15000,
        });

        const state = await page.evaluate(() => ({
          ok: window.__RIVE_OK,
          error: window.__RIVE_ERROR,
        }));

        if (runtimeErrors.length > 0) {
          throw new Error(`${fixture}.riv runtime errors: ${runtimeErrors.join(" | ")}`);
        }
        if (!state.ok || state.error) {
          throw new Error(`${fixture}.riv failed to load: ${state.error || "unknown error"}`);
        }

        await mountControlledRive(page, fixture);

        if (shot.waitFrames > 0) {
          await page.evaluate(() => {
            if (!window.__VISUAL_RIVE || typeof window.__VISUAL_RIVE.play !== "function") {
              return false;
            }
            window.__VISUAL_RIVE.play();
            return true;
          });
        }

        await advanceFrames(page, shot.waitFrames);
        const name = baselineName(fixture, shot.frame);
        const currentPath = path.join(CURRENT_DIR, name);
        const baselinePath = path.join(BASELINE_DIR, name);
        await page.screenshot({ path: currentPath });

        if (update || !fs.existsSync(baselinePath)) {
          fs.copyFileSync(currentPath, baselinePath);
          const status = update ? "updated" : "new";
          rows.push({ name: `${fixture}@f${shot.frame}`, status, diffText: "0.0000" });
          if (!update) {
            hasNewBaselines = true;
          }
          await page.close();
          continue;
        }

        const diffPercent = comparePngPixels(currentPath, baselinePath);
        const pass = diffPercent <= THRESHOLD_PERCENT;
        rows.push({
          name: `${fixture}@f${shot.frame}`,
          status: pass ? "pass" : "fail",
          diffText: diffPercent.toFixed(4),
        });
        if (!pass) {
          hasFailures = true;
        }

        await page.close();
      }
    }
  } finally {
    if (browser) {
      await browser.close();
    }
    if (server) {
      server.kill("SIGTERM");
    }
    for (const fixture of FIXTURES) {
      const filePath = path.join(HARNESS_DIR, `${fixture}.riv`);
      if (fs.existsSync(filePath)) {
        fs.unlinkSync(filePath);
      }
    }
  }

  printSummary(rows);
  console.log(`threshold: ${THRESHOLD_PERCENT.toFixed(4)}%`);
  if (hasNewBaselines) {
    console.log("created new baselines; manual review required before relying on comparisons");
  }

  if (hasFailures) {
    process.exit(1);
  }
  process.exit(0);
}

main().catch((err) => {
  console.error(err.message || err);
  process.exit(1);
});
