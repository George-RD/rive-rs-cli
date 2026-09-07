const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { setImmediate: nextTask } = require("node:timers/promises");
const { test } = require("node:test");
const vm = require("node:vm");

const source = fs.readFileSync(path.join(__dirname, "../../site/playback.js"), "utf8");

function playbackHarness(options = {}) {
  const instances = [];
  const frames = new Set();
  const window = {
    performance: { now: () => 0 },
    matchMedia: () => ({ matches: Boolean(options.reducedMotion) }),
    requestAnimationFrame(callback) {
      const handle = setImmediate(() => {
        frames.delete(handle);
        callback(0);
      });
      frames.add(handle);
      return handle;
    },
    cancelAnimationFrame(handle) {
      clearImmediate(handle);
      frames.delete(handle);
    },
  };

  class FakeRive {
    constructor(params) {
      this.canvas = params.canvas;
      this.animationNames = [];
      this.lastRenderTime = 0;
      this.elapsedMs = 0;
      this.triggerCount = 0;
      this.pendingTriggers = 0;
      this.failNextDraw = false;
      this.inputs = [
        { name: "load", value: 0 },
        { name: "armed", value: false },
        { name: "reset", fire: () => { this.pendingTriggers += 1; } },
      ];
      this.runtime = { resolveAnimationFrame() {} };
      instances.push(this);
      queueMicrotask(params.onLoad);
    }

    stateMachineInputs() { return this.inputs; }
    resizeDrawingSurfaceToCanvas() {}
    play() {}
    pause() {}
    scrub() {}
    drawFrame() {}
    cleanup() { this.cleaned = true; }

    draw(timeMs) {
      assert.ok(!this.cleaned, "a destroyed runtime must not draw");
      if (this.failNextDraw) {
        this.failNextDraw = false;
        throw new Error("draw failed");
      }
      if (this.lastRenderTime !== 0) this.elapsedMs += timeMs - this.lastRenderTime;
      this.lastRenderTime = timeMs;
      this.triggerCount += this.pendingTriggers;
      this.pendingTriggers = 0;
      this.canvas.rendered = {
        elapsedMs: this.elapsedMs,
        load: this.inputs[0].value,
        armed: this.inputs[1].value,
        triggerCount: this.triggerCount,
      };
    }
  }

  window.rive = {
    Rive: FakeRive,
    RuntimeLoader: { setWasmUrl() {} },
    Fit: { contain: "contain" },
    Alignment: { center: "center" },
  };
  vm.runInNewContext(source, {
    window,
    fetch: async () => ({ ok: true, arrayBuffer: async () => new ArrayBuffer(0) }),
  }, { filename: "site/playback.js" });

  const canvases = options.paired ? [{}, {}] : [{}];
  const config = { stateMachine: "console", fps: options.fps || 60, autoplay: false };
  const timeline = options.paired
    ? window.RivePlayback.createPairedTimeline({
      ...config,
      left: { canvas: canvases[0], src: "left.riv" },
      right: { canvas: canvases[1], src: "right.riv" },
    })
    : window.RivePlayback.createTimeline({ ...config, canvas: canvases[0], src: "scene.riv" });

  return {
    timeline,
    canvases,
    instances,
    cleanup() {
      timeline.destroy();
      for (const handle of frames) clearImmediate(handle);
    },
  };
}

for (const fps of [30, 60]) {
  test(`paused inputs and triggers refresh without elapsed time at ${fps}fps`, async (t) => {
    const harness = playbackHarness({ fps });
    t.after(() => harness.cleanup());
    const { timeline, canvases } = harness;
    await timeline.ready;
    await timeline.seekToFrame(30);
    const elapsedMs = canvases[0].rendered.elapsedMs;

    for (let value = 1; value <= 20; value += 1) {
      await nextTask();
      await timeline.setInput("load", value);
      assert.equal(canvases[0].rendered.load, value);
      assert.equal(canvases[0].rendered.elapsedMs, elapsedMs);
    }
    await timeline.setInput("armed", true);
    await timeline.fireTrigger("reset");
    assert.deepEqual(canvases[0].rendered, {
      elapsedMs, load: 20, armed: true, triggerCount: 1,
    });
    assert.equal(timeline.currentFrame, 30);
    assert.equal(timeline.isPlaying, false);

    await timeline.seekToFrame(30);
    assert.equal(canvases[0].rendered.triggerCount, 1);
    await timeline.seekToFrame(31);
    assert.ok(Math.abs(canvases[0].rendered.elapsedMs - (31 * 1000) / fps) < 1e-9);
    assert.equal(canvases[0].rendered.triggerCount, 1);
  });
}

test("paired controls share frozen time across concurrent and sequential updates", async (t) => {
  const harness = playbackHarness({ paired: true });
  t.after(() => harness.cleanup());
  const { timeline, canvases } = harness;
  await timeline.ready;
  await timeline.seekToFrame(15);
  const elapsedMs = canvases[0].rendered.elapsedMs;
  await Promise.all([
    timeline.setInput("load", 25),
    timeline.setInput("armed", true),
  ]);
  await nextTask();
  await timeline.fireTrigger("reset");
  for (const canvas of canvases) {
    assert.deepEqual(canvas.rendered, {
      elapsedMs, load: 25, armed: true, triggerCount: 1,
    });
  }
  assert.equal(timeline.currentFrame, 15);
});

test("backward seeks retain assigned inputs but do not replay a trigger", async (t) => {
  const harness = playbackHarness();
  t.after(() => harness.cleanup());
  const { timeline, canvases } = harness;
  await timeline.ready;
  await timeline.seekToFrame(30);
  await timeline.setInput("load", 75);
  await timeline.fireTrigger("reset");
  await timeline.seekToFrame(0);
  assert.deepEqual(canvases[0].rendered, {
    elapsedMs: 0, load: 75, armed: false, triggerCount: 0,
  });
  await timeline.setInput("armed", true);
  assert.equal(canvases[0].rendered.elapsedMs, 0);
  assert.equal(canvases[0].rendered.armed, true);
});

test("a failed paused refresh rejects its caller without poisoning subsequent controls", async (t) => {
  const harness = playbackHarness();
  t.after(() => harness.cleanup());
  const { timeline, canvases, instances } = harness;
  await timeline.ready;
  await timeline.seekToFrame(15);
  instances[0].failNextDraw = true;
  await assert.rejects(timeline.setInput("load", 10), /draw failed/);
  await timeline.setInput("load", 20);
  assert.equal(canvases[0].rendered.elapsedMs, 250);
  assert.equal(canvases[0].rendered.load, 20);
  await timeline.seekToFrame(30);
  assert.equal(canvases[0].rendered.elapsedMs, 500);
});

test("playing controls wait for playback rather than injecting time", async (t) => {
  const harness = playbackHarness();
  t.after(() => harness.cleanup());
  const { timeline, canvases } = harness;
  await timeline.ready;
  await timeline.play();
  await timeline.setInput("load", 90);
  await timeline.fireTrigger("reset");
  await timeline.pause();
  assert.equal(canvases[0].rendered.elapsedMs, 0);
  await timeline.seekToFrame(1);
  assert.equal(canvases[0].rendered.load, 90);
  assert.equal(canvases[0].rendered.triggerCount, 1);
});

test("reduced-motion controls work at frame zero and destruction cancels pending work", async (t) => {
  const harness = playbackHarness({ reducedMotion: true });
  t.after(() => harness.cleanup());
  const { timeline, canvases } = harness;
  await timeline.ready;
  assert.equal(timeline.reducedMotion, true);
  await timeline.setInput("armed", true);
  assert.equal(canvases[0].rendered.armed, true);
  assert.equal(canvases[0].rendered.elapsedMs, 0);
  const rendered = { ...canvases[0].rendered };
  const pending = timeline.setInput("load", 100);
  timeline.destroy();
  await pending;
  assert.deepEqual(canvases[0].rendered, rendered);
});
