const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { captureCanvasPng, visualBrowserLaunchOptions } = require("./shared");

async function main() {
  assert.deepEqual(visualBrowserLaunchOptions(), {
    headless: true,
    args: ["--disable-gpu"],
  });

  const root = fs.mkdtempSync(path.join(os.tmpdir(), "rive-visual-contract-"));
  const output = path.join(root, "frame.png");
  const expected = Buffer.from("deterministic-png-bytes");
  const calls = [];
  const session = {
    send: async (method, options) => {
      calls.push({ type: "send", method, options });
      if (method === "Page.captureScreenshot") {
        return { data: expected.toString("base64") };
      }
      return {};
    },
    detach: async () => calls.push({ type: "detach" }),
  };
  const page = {
    evaluate: async (fn, options) => {
      calls.push({ type: "evaluate", fn, options });
      return "";
    },
    locator: (selector) => ({
      boundingBox: async () => {
        calls.push({ type: "boundingBox", selector });
        return { x: 12, y: 18, width: 512, height: 512 };
      },
    }),
    context: () => ({
      newCDPSession: async (target) => {
        calls.push({ type: "newCDPSession", target });
        return session;
      },
    }),
  };

  try {
    await captureCanvasPng(page, output, { scale: 2 });
    assert.deepEqual(fs.readFileSync(output), expected);
    assert.equal(calls.filter((call) => call.type === "newCDPSession").length, 1);
    assert.equal(calls.filter((call) => call.type === "boundingBox").length, 1);
    assert.equal(calls.filter((call) => call.type === "detach").length, 1);
    const sends = calls.filter((call) => call.type === "send");
    assert.deepEqual(sends.map((call) => call.method), [
      "Page.enable",
      "Page.captureScreenshot",
    ]);
    assert.deepEqual(sends[0].options, undefined);
    assert.deepEqual(sends[1].options, {
      format: "png",
      captureBeyondViewport: false,
      clip: { x: 12, y: 18, width: 512, height: 512, scale: 2 },
    });
    const evaluations = calls.filter((call) => call.type === "evaluate");
    assert.equal(evaluations.length, 2);
    assert.deepEqual(evaluations[0].options, {
      selector: "#canvas-controlled",
      background: "#0f172a",
    });
  } finally {
    fs.rmSync(root, { recursive: true, force: true });
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
