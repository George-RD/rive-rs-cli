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
    let animation = options.animation || null;
    let mode = stateMachine ? "stateMachine" : animation ? "animation" : "static";
    const sceneBuffer = loadSceneBuffer(src);
    const stepMs = 1000 / fps;

    let instance = null;
    let stepsAdvanced = 0;
    let destroyed = false;
    const retainedInputs = new Map(Object.entries(options.inputs || {}));

    function inputByName(name) {
      if (!instance || !stateMachine) return null;
      const inputs = instance.stateMachineInputs(stateMachine) || [];
      return inputs.find((input) => input.name === name) || null;
    }

    function applyRetainedInputs() {
      for (const [name, value] of retainedInputs) {
        const input = inputByName(name);
        if (input) input.value = value;
      }
    }

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
      applyRetainedInputs();
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
      if (!stateMachine && !animation && instance.animationNames?.length > 0) {
        animation = instance.animationNames[0];
        mode = "animation";
      }
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

    async function setInput(name, value) {
      await ready;
      if (destroyed) return;
      retainedInputs.set(name, value);
      const input = inputByName(name);
      if (input) input.value = value;
    }

    async function fireTrigger(name) {
      await ready;
      if (destroyed) return;
      const input = inputByName(name);
      if (input && typeof input.fire === "function") input.fire();
    }

    function advanceOneStep() {
      if (!instance || destroyed || mode !== "stateMachine") return;
      stepsAdvanced += 1;
      stepTo(CLOCK_ORIGIN_MS + stepsAdvanced * stepMs);
    }

    function readInputs() {
      if (!instance || !stateMachine) return {};
      const values = {};
      for (const input of instance.stateMachineInputs(stateMachine) || []) {
        values[input.name] = input.value;
      }
      return values;
    }

    function destroy() {
      if (destroyed) return;
      destroyed = true;
      detachScheduledFrame();
      if (instance) instance.cleanup();
      instance = null;
    }

    return {
      ready,
      seekToFrame,
      resize,
      setInput,
      fireTrigger,
      advanceOneStep,
      readInputs,
      destroy,
    };
  }

  function createLogicalTimeline(controllers, options = {}) {
    const fps = options.fps || CAPTURE_FPS;
    const endFrame =
      typeof options.endFrame === "number" && Number.isFinite(options.endFrame)
        ? Math.max(0, Math.round(options.endFrame))
        : null;
    const loops = endFrame !== null && options.loop === true;
    const reducedMotion = Boolean(
      global.matchMedia && global.matchMedia("(prefers-reduced-motion: reduce)").matches
    );

    let logicalFrame = 0;
    let playing = false;
    let destroyed = false;
    let frameRequestId = null;
    let wallOrigin = 0;
    let frameOrigin = 0;
    let seekChain = Promise.resolve();
    let settleQueued = false;

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
        await Promise.all(controllers.map((controller) => controller.seekToFrame(target)));
        logicalFrame = target;
        reportFrame();
      });
      return seekChain;
    }

    function startPlaying() {
      if (destroyed) return;
      if (playing || (endFrame !== null && !loops && logicalFrame >= endFrame)) {
        reportPlaying();
        return;
      }
      playing = true;
      frameOrigin = logicalFrame;
      wallOrigin = global.performance.now();
      reportPlaying();
      frameRequestId = global.requestAnimationFrame(tick);
    }

    const ready = Promise.all(controllers.map((controller) => controller.ready)).then(() => {
      logicalFrame = 0;
      reportFrame();
      reportPlaying();
      if (options.autoplay === true && !reducedMotion) startPlaying();
    });

    async function tick(now) {
      if (!playing || destroyed) return;
      const elapsedTarget =
        frameOrigin + Math.max(0, Math.floor(((now - wallOrigin) * fps) / 1000));
      const target = endFrame === null ? elapsedTarget : Math.min(elapsedTarget, endFrame);
      if (target > logicalFrame) {
        await enqueueSeek(target);
      }
      if (endFrame !== null && elapsedTarget >= endFrame) {
        if (loops) {
          await enqueueSeek(0);
          if (playing && !destroyed) {
            frameOrigin = 0;
            wallOrigin = global.performance.now();
          }
        } else {
          await pause();
        }
      }
      if (playing && !destroyed) {
        frameRequestId = global.requestAnimationFrame(tick);
      }
    }

    async function play() {
      await ready;
      await seekChain;
      startPlaying();
    }

    async function pause() {
      if (!playing) {
        reportPlaying();
        return seekChain;
      }
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
      const target = endFrame === null ? frame : Math.min(frame, endFrame);
      await enqueueSeek(target);
      if (playing) {
        frameOrigin = logicalFrame;
        wallOrigin = global.performance.now();
      }
    }

    function resize() {
      for (const controller of controllers) controller.resize();
    }

    async function setInput(name, value) {
      await ready;
      await Promise.all(controllers.map((controller) => controller.setInput(name, value)));
      await settleControlChange();
    }

    async function fireTrigger(name) {
      await ready;
      await Promise.all(controllers.map((controller) => controller.fireTrigger(name)));
      await settleControlChange();
    }

    function settleControlChange() {
      if (playing || destroyed || settleQueued) return seekChain;
      settleQueued = true;
      const settled = seekChain.then(async () => {
        settleQueued = false;
        if (playing || destroyed) return;
        for (const controller of controllers) controller.advanceOneStep();
      });
      seekChain = settled.catch(() => {});
      return settled;
    }

    function readInputs() {
      return controllers[0] ? controllers[0].readInputs() : {};
    }

    function destroy() {
      if (destroyed) return;
      destroyed = true;
      playing = false;
      if (frameRequestId !== null) global.cancelAnimationFrame(frameRequestId);
      frameRequestId = null;
      for (const controller of controllers) controller.destroy();
      reportPlaying();
    }

    return {
      ready,
      play,
      pause,
      seekToFrame,
      resize,
      setInput,
      fireTrigger,
      readInputs,
      destroy,
      get currentFrame() {
        return logicalFrame;
      },
      get isPlaying() {
        return playing;
      },
      get reducedMotion() {
        return reducedMotion;
      },
    };
  }

  function createTimeline(options) {
    const fps = options.fps || CAPTURE_FPS;
    const scene = createControlledRive(options.canvas, options.src, {
      fps,
      stateMachine: options.stateMachine,
      animation: options.animation,
      inputs: options.inputs,
    });
    return createLogicalTimeline([scene], {
      ...options,
      fps,
      autoplay: options.autoplay === true,
    });
  }

  function createPairedTimeline(options) {
    const fps = options.fps || CAPTURE_FPS;
    const left = createControlledRive(options.left.canvas, options.left.src, {
      fps,
      stateMachine: options.left.stateMachine || options.stateMachine,
      animation: options.left.animation || options.animation,
      inputs: options.left.inputs || options.inputs,
    });
    const right = createControlledRive(options.right.canvas, options.right.src, {
      fps,
      stateMachine: options.right.stateMachine || options.stateMachine,
      animation: options.right.animation || options.animation,
      inputs: options.right.inputs || options.inputs,
    });
    return createLogicalTimeline([left, right], {
      ...options,
      fps,
      autoplay: options.autoplay === true,
    });
  }

  global.RivePlayback = {
    CAPTURE_FPS,
    createControlledRive,
    createTimeline,
    createPairedTimeline,
  };
})(window);
