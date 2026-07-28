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
  const page = {
    evaluate: async (fn, options) => {
      calls.push({ fn, options });
      return `data:image/png;base64,${expected.toString("base64")}`;
    },
  };

  try {
    await captureCanvasPng(page, output);
    assert.equal(calls.length, 1);
    assert.equal(typeof calls[0].fn, "function");
    assert.deepEqual(calls[0].options, {
      selector: "#canvas-controlled",
      background: "#0f172a",
    });
    assert.deepEqual(fs.readFileSync(output), expected);
  } finally {
    fs.rmSync(root, { recursive: true, force: true });
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
