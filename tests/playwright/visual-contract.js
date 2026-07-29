const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { captureCanvasPng } = require("./shared");

async function main() {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "rive-visual-contract-"));
  const output = path.join(root, "frame.png");
  const expected = Buffer.from("deterministic-png-bytes");
  const calls = [];
  const session = {
    send: async (method, options) => {
      calls.push({ type: "send", method, options });
      return { data: expected.toString("base64") };
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
    await captureCanvasPng(page, output);
    assert.deepEqual(fs.readFileSync(output), expected);
    assert.equal(calls.filter((call) => call.type === "newCDPSession").length, 1);
    assert.equal(calls.filter((call) => call.type === "boundingBox").length, 1);
    assert.equal(calls.filter((call) => call.type === "detach").length, 1);
    const capture = calls.find((call) => call.type === "send");
    assert.equal(capture.method, "Page.captureScreenshot");
    assert.deepEqual(capture.options, {
      format: "png",
      fromSurface: true,
      clip: { x: 12, y: 18, width: 512, height: 512, scale: 1 },
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
