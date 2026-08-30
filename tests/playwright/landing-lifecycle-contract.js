const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");

const ROOT = path.resolve(__dirname, "..", "..");
const source = fs.readFileSync(path.join(ROOT, "site", "landing.js"), "utf8");

async function main() {
  const listeners = new Map();
  let destroyCalls = 0;
  let pauseCalls = 0;
  let playCalls = 0;
  let resizeCalls = 0;
  let playing = true;

  const parcel = { dataset: {} };
  const canvas = {};
  const timeline = {
    ready: Promise.resolve(),
    pause() {
      pauseCalls += 1;
      playing = false;
      return Promise.resolve();
    },
    play() {
      playCalls += 1;
      playing = true;
      return Promise.resolve();
    },
    resize() {
      resizeCalls += 1;
    },
    destroy() {
      destroyCalls += 1;
      playing = false;
    },
    get isPlaying() {
      return playing;
    },
  };
  const playback = {
    createTimeline(options) {
      assert.equal(options.canvas, canvas);
      assert.equal(options.src, "examples/authoring/complex-animated-showcase.v0.riv");
      assert.equal(options.endFrame, 120);
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
  assert.equal(typeof listeners.get("pageshow"), "function");
  assert.equal(typeof listeners.get("resize"), "function");

  listeners.get("pagehide")({ persisted: true });
  assert.equal(pauseCalls, 1, "bfcache pagehide must pause the live timeline");
  assert.equal(destroyCalls, 0, "bfcache pagehide must not destroy the timeline");

  listeners.get("pageshow")({ persisted: true });
  assert.equal(playCalls, 1, "bfcache pageshow must resume a timeline that was playing");

  listeners.get("resize")();
  assert.equal(resizeCalls, 1, "restored timeline must still respond after bfcache pageshow");

  playing = false;
  listeners.get("pagehide")({ persisted: true });
  listeners.get("pageshow")({ persisted: true });
  assert.equal(pauseCalls, 1, "manually paused timelines must stay paused on bfcache entry");
  assert.equal(playCalls, 1, "manually paused timelines must not resume on bfcache restoration");

  listeners.get("pagehide")({ persisted: false });
  assert.equal(destroyCalls, 1, "normal page exit must clean up the timeline");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
