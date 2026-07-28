const assert = require("node:assert/strict");
const { waitForRiveReady } = require("./shared");

async function main() {
  const calls = [];
  const page = {
    waitForFunction: async (...args) => {
      calls.push(args);
    },
  };

  await waitForRiveReady(page, 15_000);

  assert.equal(calls.length, 1);
  assert.equal(typeof calls[0][0], "function");
  assert.equal(calls[0][1], undefined);
  assert.deepEqual(calls[0][2], { timeout: 15_000 });
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
