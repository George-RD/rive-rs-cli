(function (global) {
  const CAPTURE_FPS = 60;
  const CLOCK_ORIGIN_MS = 1000;
  let runtimeConfigured = false;

  function configureRuntime() {
    if (runtimeConfigured) return;
    if (!global.rive || !global.rive.Rive) {
      throw new Error("the Rive runtime script did not load");
    }
    global.rive.RuntimeLoader.setWasmUrl("assets/rive.wasm");
    runtimeConfigured = true;
  }

  function nextPaint() {
    return new Promise((resolve) => {
      global.requestAnimationFrame(() => global.requestAnimationFrame(resolve));
    });
  }

  function describeError(error) {
    if (!error) return "unknown Rive load error";
    if (typeof error === "string") return error;
    if (error.message) return error.message;
    if (error.data) {
      return typeof error.data === "string" ? error.data : JSON.stringify(error.data);
    }
    try {
      return JSON.stringify(error);
    } catch (_failure) {
      return String(error);
    }
  }

  async function loadSceneBuffer(src) {
    const response = await fetch(src);
    if (!response.ok) {
      throw new Error(`could not load ${src}: HTTP ${response.status}`);
    }
    return response.arrayBuffer();
  }

  function createControlledRive(canvas, src, options = {}) {
    configureRuntime();

    const fps = options.fps || CAPTURE_FPS;
    const stateMachine = options.stateMachine || null;
    const animation = options.animation || null;
    const mode = stateMachine ? "stateMachine" : animation ? "animation" : "static";
    const sceneBuffer = loadSceneBuffer(src);
    const stepMs = 1000 / fps;

    let instance = null;
    let stepsAdvanced = 0;
    let destroyed = false;

    function detachScheduledFrame() {
      if (instance && instance.frameRequestId) {
        global.cancelAnimationFrame(instance.frameRequestId);
        instance.frameRequestId = null;
      }
    }

    function stepTo(timeMs) {
      instance.draw(timeMs);
      if (
        instance.runtime &&
        typeof instance.runtime.resolveAnimationFrame === "function"
      ) {
        instance.runtime.resolveAnimationFrame();
      }
      detachScheduledFrame();
    }

    async function build() {
      const buffer = await sceneBuffer;
      return new Promise((resolve, reject) => {
        let built = null;
        let loaded = false;
        const params = {
          buffer,
          canvas,
          autoplay: false,
          fit: global.rive.Fit.contain,
          alignment: global.rive.Alignment.center,
          onLoad() {
            loaded = true;
            if (built) resolve(built);
          },
          onLoadError(error) {
            reject(new Error(`${src}: ${describeError(error)}`));
          },
        };
        if (stateMachine) {
          params.stateMachines = [stateMachine];
        } else if (animation) {
          params.animations = [animation];
        }
        built = new global.rive.Rive(params);
        if (loaded) resolve(built);
      });
    }

    function startStateMachine() {
      instance.play([stateMachine]);
      detachScheduledFrame();
      instance.lastRenderTime = 0;
      stepsAdvanced = 0;
      stepTo(CLOCK_ORIGIN_MS);
    }

    async function initialize() {
      instance = await build();
      if (destroyed) {
        instance.cleanup();
        return;
      }
      instance.resizeDrawingSurfaceToCanvas();
      if (mode === "stateMachine") {
        startStateMachine();
      } else if (mode === "animation") {
        instance.pause();
        instance.scrub([animation], 0);
      } else {
        instance.drawFrame();
      }
      detachScheduledFrame();
    }

    const ready = initialize();

    async function rebuildStateMachine() {
      if (instance) instance.cleanup();
      instance = await build();
      if (destroyed) {
        instance.cleanup();
        return;
      }
      instance.resizeDrawingSurfaceToCanvas();
      startStateMachine();
    }

    async function seekToFrame(frame) {
      await ready;
      if (destroyed) return;
      const target = Math.max(0, Math.round(frame));

      if (mode === "animation") {
        instance.pause();
        instance.scrub([animation], target / fps);
      } else if (mode === "stateMachine") {
        if (target < stepsAdvanced) {
          await rebuildStateMachine();
        }
        while (stepsAdvanced < target) {
          stepsAdvanced += 1;
          stepTo(CLOCK_ORIGIN_MS + stepsAdvanced * stepMs);
        }
      } else {
        instance.drawFrame();
      }

      await nextPaint();
      detachScheduledFrame();
    }

    function resize() {
      if (instance && !destroyed) instance.resizeDrawingSurfaceToCanvas();
    }

    function destroy() {
      if (destroyed) return;
      destroyed = true;
      detachScheduledFrame();
      if (instance) instance.cleanup();
      instance = null;
    }

    return { ready, seekToFrame, resize, destroy };
  }

  function createPairedTimeline(options) {
    const fps = options.fps || CAPTURE_FPS;
    const left = createControlledRive(options.left.canvas, options.left.src, {
      fps,
      stateMachine: options.left.stateMachine || options.stateMachine,
      animation: options.left.animation || options.animation,
    });
    const right = createControlledRive(options.right.canvas, options.right.src, {
      fps,
      stateMachine: options.right.stateMachine || options.stateMachine,
      animation: options.right.animation || options.animation,
    });

    let logicalFrame = 0;
    let playing = false;
    let destroyed = false;
    let frameRequestId = null;
    let wallOrigin = 0;
    let frameOrigin = 0;
    let seekChain = Promise.resolve();

    function reportFrame() {
      if (typeof options.onFrame === "function") options.onFrame(logicalFrame);
    }

    function reportPlaying() {
      if (typeof options.onPlayingChange === "function") {
        options.onPlayingChange(playing);
      }
    }

    function enqueueSeek(frame) {
      const target = Math.max(0, Math.round(frame));
      seekChain = seekChain.then(async () => {
        if (destroyed) return;
        await Promise.all([left.seekToFrame(target), right.seekToFrame(target)]);
        logicalFrame = target;
        reportFrame();
      });
      return seekChain;
    }

    const ready = Promise.all([left.ready, right.ready]).then(() => {
      logicalFrame = 0;
      reportFrame();
    });

    async function tick(now) {
      if (!playing || destroyed) return;
      const target = frameOrigin + Math.max(0, Math.floor(((now - wallOrigin) * fps) / 1000));
      if (target > logicalFrame) {
        await enqueueSeek(target);
      }
      if (playing && !destroyed) {
        frameRequestId = global.requestAnimationFrame(tick);
      }
    }

    async function play() {
      await ready;
      await seekChain;
      if (destroyed || playing) return;
      playing = true;
      frameOrigin = logicalFrame;
      wallOrigin = global.performance.now();
      reportPlaying();
      frameRequestId = global.requestAnimationFrame(tick);
    }

    async function pause() {
      if (!playing) return seekChain;
      playing = false;
      if (frameRequestId !== null) {
        global.cancelAnimationFrame(frameRequestId);
        frameRequestId = null;
      }
      reportPlaying();
      return seekChain;
    }

    async function seekToFrame(frame) {
      await ready;
      await enqueueSeek(frame);
      if (playing) {
        frameOrigin = logicalFrame;
        wallOrigin = global.performance.now();
      }
    }

    function resize() {
      left.resize();
      right.resize();
    }

    function destroy() {
      if (destroyed) return;
      destroyed = true;
      playing = false;
      if (frameRequestId !== null) global.cancelAnimationFrame(frameRequestId);
      frameRequestId = null;
      left.destroy();
      right.destroy();
      reportPlaying();
    }

    return {
      ready,
      play,
      pause,
      seekToFrame,
      resize,
      destroy,
      get currentFrame() {
        return logicalFrame;
      },
      get isPlaying() {
        return playing;
      },
    };
  }

  global.RivePlayback = {
    CAPTURE_FPS,
    createControlledRive,
    createPairedTimeline,
  };
})(window);
