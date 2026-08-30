const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");

const ROOT = path.resolve(__dirname, "..", "..");
const source = fs.readFileSync(path.join(ROOT, "site", "landing.js"), "utf8");
const showcaseSource = fs.readFileSync(path.join(ROOT, "site", "showcase.js"), "utf8");

async function main() {
  const listeners = new Map();
  let destroyCalls = 0;
  let resizeCalls = 0;

  const parcel = { dataset: {} };
  const canvas = {};
  const timeline = {
    ready: Promise.resolve(),
    resize() {
      resizeCalls += 1;
    },
    destroy() {
      destroyCalls += 1;
    },
  };
  const playback = {
    createTimeline(options) {
      assert.equal(options.canvas, canvas);
      assert.equal(options.src, "examples/authoring/complex-animated-showcase.v0.riv");
      return timeline;
    },
  };
  const window = {
    RivePlayback: playback,
    addEventListener(name, handler) {
      listeners.set(name, handler);
    },
  };
  const document = {
    querySelector(selector) {
      if (selector === ".proof-parcel") return parcel;
      if (selector === ".hero-scene") return canvas;
      return null;
    },
  };

  vm.runInNewContext(source, {
    window,
    document,
    RivePlayback: playback,
    console,
  });
  await Promise.resolve();

  assert.equal(parcel.dataset.playbackReady, "true");
  assert.equal(typeof listeners.get("pagehide"), "function");
  assert.equal(typeof listeners.get("resize"), "function");

  listeners.get("pagehide")({ persisted: true });
  assert.equal(destroyCalls, 0, "bfcache pagehide must preserve the live timeline");

  listeners.get("resize")();
  assert.equal(resizeCalls, 1, "preserved timeline must still respond after bfcache pagehide");

  listeners.get("pagehide")({ persisted: false });
  assert.equal(destroyCalls, 1, "normal page exit must clean up the timeline");

  const showcaseHandler = showcaseSource.match(
    /window\.addEventListener\("pagehide", \(event\) => \{([\s\S]*?)\n  \}\);/
  );
  assert.ok(showcaseHandler, "showcase must register a reusable pagehide handler");
  assert.match(
    showcaseHandler[1],
    /if \(event\.persisted\) return;/,
    "showcase must preserve timelines when entering bfcache"
  );
  assert.match(
    showcaseHandler[1],
    /timeline\.destroy\(\)/,
    "showcase must still destroy timelines on a real page exit"
  );
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
