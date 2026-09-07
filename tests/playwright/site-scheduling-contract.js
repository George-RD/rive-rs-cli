const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");
const vm = require("node:vm");

const source = fs.readFileSync(path.join(__dirname, "../../site/playback.js"), "utf8");

function frameQueue() {
  let nextId = 0;
  const callbacks = new Map();
  return {
    requestAnimationFrame(callback) {
      const id = ++nextId;
      callbacks.set(id, callback);
      return id;
    },
    cancelAnimationFrame(id) {
      callbacks.delete(id);
    },
    flush(timeMs) {
      const pending = [...callbacks.values()];
      callbacks.clear();
      for (const callback of pending) callback(timeMs);
    },
  };
}

function schedulingHarness(runtimeScheduling = true) {
  const browser = frameQueue();
  const runtime = runtimeScheduling ? frameQueue() : {};
  const scheduler = runtimeScheduling ? runtime : browser;
  runtime.resolveAnimationFrame = () => {};
  const window = {
    ...browser,
    performance: { now: () => 0 },
    matchMedia: () => ({ matches: false }),
  };

  class ScheduledRive {
    constructor(params) {
      this.canvas = params.canvas;
      this.runtime = runtime;
      this.lastRenderTime = 0;
      queueMicrotask(params.onLoad);
    }
    stateMachineInputs() { return []; }
    resizeDrawingSurfaceToCanvas() {}
    play() { this.schedule(); }
    cleanup() { scheduler.cancelAnimationFrame(this.frameRequestId); }
    schedule() {
      this.frameRequestId = scheduler.requestAnimationFrame((time) => this.draw(time));
    }
    draw(time) {
      this.frameRequestId = null;
      this.canvas.drawnAt = time;
      this.schedule();
    }
  }

  window.rive = {
    Rive: ScheduledRive,
    RuntimeLoader: { setWasmUrl() {} },
    Fit: { contain: "contain" },
    Alignment: { center: "center" },
  };
  vm.runInNewContext(source, {
    window,
    fetch: async () => ({ ok: true, arrayBuffer: async () => new ArrayBuffer(0) }),
  }, { filename: "site/playback.js" });

  return { browser, scheduler, playback: window.RivePlayback };
}

for (const runtimeScheduling of [true, false]) {
  const provider = runtimeScheduling ? "Rive" : "browser fallback";

  test(`${provider} cancellation preserves unrelated browser frames`, async (t) => {
    const { browser, playback } = schedulingHarness(runtimeScheduling);
    let painted = false;
    browser.requestAnimationFrame(() => { painted = true; });
    const controller = playback.createControlledRive({}, "scene.riv", {
      stateMachine: "console",
    });
    t.after(() => controller.destroy());
    await controller.ready;
    browser.flush(16);
    assert.equal(painted, true, "controlled playback must not cancel another browser frame");
  });

  test(`${provider} cannot advance a controlled scene outside its logical clock`, async (t) => {
    const { scheduler, playback } = schedulingHarness(runtimeScheduling);
    const canvas = {};
    const controller = playback.createControlledRive(canvas, "scene.riv", {
      stateMachine: "console",
    });
    t.after(() => controller.destroy());
    await controller.ready;
    const controlledFrame = canvas.drawnAt;
    assert.equal(controlledFrame, 1000);
    scheduler.flush(16);
    scheduler.flush(32);
    assert.equal(canvas.drawnAt, controlledFrame, "automatic runtime draws must be detached");
  });
}
