(function (global) {
  const LOAD_TIMEOUT_MS = 15000;
  const names = ['Orbit', 'Weave', 'Stack'];
  const chooseLayout = (layouts, width) => [...layouts].reverse().find(layout => width >= layout.minWidth) || layouts[0];
  const clamp = value => Math.max(0, Math.min(2, value));
  const describeBlend = value => Number.isInteger(value) ? names[value] : `${names[Math.floor(value)]} to ${names[Math.ceil(value)]}, ${Math.round((value % 1) * 100)} percent`;
  const pointerValue = (clientX, bounds, rail, layoutWidth) => Math.round(clamp(((clientX - bounds.left) / bounds.width * layoutWidth - rail.x) / rail.width * 2) * 100) / 100;

  if (typeof module !== 'undefined') module.exports = { chooseLayout, clamp, describeBlend, pointerValue };
  if (!global.document) return;

  const surface = document.getElementById('page-surface');
  const canvas = document.getElementById('rive-page');
  const controls = document.getElementById('page-controls');
  const fallback = document.getElementById('page-fallback');
  const status = document.getElementById('page-status');
  const announcement = document.getElementById('interaction-status');
  const reduced = global.matchMedia('(prefers-reduced-motion: reduce)');
  let manifest, layout, timeline, generation = 0, disposed = false;
  let morph = 0, wantsMotion = !reduced.matches, suspended = document.hidden;
  let controlElements = new Map();
  let resizeFrame = null, motionGeneration = 0;

  function fail(error) {
    if (disposed) return;
    generation += 1;
    timeline?.destroy();
    timeline = null;
    surface.style.removeProperty('display');
    surface.dataset.state = 'error';
    surface.setAttribute('aria-busy', 'false');
    controls.hidden = true;
    fallback.hidden = false;
    status.textContent = 'The Rive interface could not load. The text version, showcase and verification lab are still available.';
    console.error('Rive page:', error);
  }

  function guarded(operation) {
    Promise.resolve().then(operation).catch(fail);
  }

  function updateSemantics() {
    const slider = controlElements.get('morph');
    if (slider) {
      slider.setAttribute('aria-valuenow', String(Number(morph.toFixed(2))));
      slider.setAttribute('aria-valuetext', describeBlend(morph));
    }
    ['orbit', 'weave', 'stack'].forEach((id, index) => controlElements.get(id)?.setAttribute('aria-pressed', String(Math.abs(index - morph) < 0.005)));
    const transport = controlElements.get('playback');
    if (transport) transport.setAttribute('aria-label', wantsMotion ? 'Pause motion' : 'Play motion');
    surface.dataset.morph = String(morph);
    surface.dataset.playing = String(Boolean(timeline?.isPlaying));
  }

  async function setMorph(value, announce = false) {
    morph = clamp(value);
    updateSemantics();
    if (timeline) await timeline.setInput(layout.inputs.morph, morph);
    if (announce) announcement.textContent = describeBlend(morph);
  }

  async function syncMotion() {
    const token = ++motionGeneration;
    const current = timeline;
    if (!current) return;
    const playing = wantsMotion && !suspended;
    await current.pause();
    if (token !== motionGeneration || current !== timeline || disposed) return;
    await current.setInput(layout.inputs.paused, !wantsMotion);
    if (token !== motionGeneration || current !== timeline || disposed) return;
    if (playing) await current.play();
    updateSemantics();
  }

  function bindSlider(element, entry) {
    const move = event => guarded(() => setMorph(pointerValue(event.clientX, surface.getBoundingClientRect(), entry.rail, layout.width)));
    element.addEventListener('pointerdown', event => {
      if (!event.isPrimary || event.button !== 0) return;
      element.focus({ preventScroll: true });
      element.setPointerCapture(event.pointerId);
      move(event);
    });
    element.addEventListener('pointermove', event => { if (element.hasPointerCapture(event.pointerId)) move(event); });
    element.addEventListener('pointerup', event => {
      if (element.hasPointerCapture(event.pointerId)) {
        move(event);
        element.releasePointerCapture(event.pointerId);
        announcement.textContent = describeBlend(morph);
      }
    });
    element.addEventListener('keydown', event => {
      const actions = { ArrowRight: morph + 0.01, ArrowUp: morph + 0.01, ArrowLeft: morph - 0.01, ArrowDown: morph - 0.01, PageUp: morph + 0.1, PageDown: morph - 0.1, Home: 0, End: 2 };
      if (!(event.key in actions)) return;
      event.preventDefault();
      guarded(() => setMorph(actions[event.key]));
    });
  }

  function mountControls() {
    const focused = document.activeElement?.dataset.control;
    controls.replaceChildren();
    controlElements = new Map();
    for (const entry of layout.controls) {
      const isLink = ['link', 'source', 'download'].includes(entry.action);
      const element = document.createElement(isLink ? 'a' : entry.action === 'range' ? 'div' : 'button');
      element.className = 'page-control';
      element.dataset.control = entry.id;
      element.setAttribute('aria-label', entry.label);
      element.style.left = `${entry.x / layout.width * 100}%`;
      element.style.top = `${entry.y / layout.height * 100}%`;
      element.style.width = `${entry.width / layout.width * 100}%`;
      element.style.height = `${entry.height / layout.height * 100}%`;
      if (isLink) {
        element.href = entry.action === 'source' ? layout.source : entry.action === 'download' ? layout.artifact : entry.href;
        if (entry.action === 'download') element.download = `rive-cli-page-${layout.id}.riv`;
      } else if (entry.action === 'range') {
        element.tabIndex = 0;
        element.setAttribute('role', 'slider');
        element.setAttribute('aria-valuemin', '0');
        element.setAttribute('aria-valuemax', '2');
        bindSlider(element, entry);
      } else {
        element.type = 'button';
        element.addEventListener('click', () => guarded(async () => {
          if (entry.action === 'preset') await setMorph(entry.value, true);
          if (entry.action === 'playback') { wantsMotion = !wantsMotion; await syncMotion(); }
          if (entry.action === 'replay') { await timeline.pause(); await timeline.seekToFrame(0); await syncMotion(); announcement.textContent = 'Motion reset.'; }
        }));
      }
      controlElements.set(entry.id, element);
      controls.append(element);
    }
    updateSemantics();
    if (focused) controlElements.get(focused)?.focus({ preventScroll: true });
  }

  async function mount(next) {
    const token = ++generation;
    timeline?.destroy();
    layout = next;
    surface.style.aspectRatio = `${layout.width} / ${layout.height}`;
    surface.dataset.layout = layout.id;
    surface.dataset.state = 'loading';
    surface.setAttribute('aria-busy', 'true');
    controls.hidden = true;
    fallback.hidden = false;
    status.textContent = 'Loading the Rive interface.';
    surface.style.display = 'block';
    canvas.style.visibility = 'hidden';
    const current = global.RivePlayback.createTimeline({
      canvas, src: layout.artifact, stateMachine: layout.stateMachine,
      inputs: { [layout.inputs.morph]: morph, [layout.inputs.paused]: !wantsMotion },
      autoplay: false,
      onFrame(frame) {
        if (token !== generation) return;
        surface.dataset.frame = String(frame);
        const value = current?.readInputs()[layout.inputs.morph];
        if (typeof value === 'number' && Math.abs(value - morph) > 0.0001) { morph = value; updateSemantics(); }
      },
      onPlayingChange(playing) { if (token === generation) surface.dataset.playing = String(playing); },
    });
    timeline = current;
    let timer;
    try {
      await Promise.race([current.ready, new Promise((_, reject) => { timer = setTimeout(() => reject(new Error('interface load timed out')), LOAD_TIMEOUT_MS); })]);
      if (disposed || token !== generation) { current.destroy(); return; }
      await current.setInput(layout.inputs.morph, morph);
      await current.setInput(layout.inputs.paused, !wantsMotion);
      if (!Object.hasOwn(current.readInputs(), layout.inputs.morph)) throw new Error('generated state machine did not load');
      mountControls();
      await syncMotion();
      if (disposed || token !== generation) { current.destroy(); return; }
      surface.dataset.state = 'ready';
      surface.style.removeProperty('display');
      canvas.style.removeProperty('visibility');
      surface.setAttribute('aria-busy', 'false');
      controls.hidden = false;
      fallback.hidden = true;
    } catch (error) {
      current.destroy();
      if (token === generation) throw error;
    } finally { clearTimeout(timer); }
  }

  async function start() {
    if (!global.RivePlayback || !global.rive) throw new Error('animation runtime unavailable');
    const response = await fetch('scenes/manifest.json', { signal: AbortSignal.timeout(LOAD_TIMEOUT_MS) });
    if (!response.ok) throw new Error(`manifest HTTP ${response.status}`);
    manifest = await response.json();
    if (manifest.format !== 1 || !Array.isArray(manifest.layouts) || manifest.layouts.length !== 3) throw new Error('invalid page manifest');
    await mount(chooseLayout(manifest.layouts, surface.parentElement.clientWidth));
  }

  global.addEventListener('resize', () => {
    if (resizeFrame !== null) cancelAnimationFrame(resizeFrame);
    resizeFrame = requestAnimationFrame(() => {
      resizeFrame = null;
      if (!manifest || disposed || !timeline) return;
      const next = chooseLayout(manifest.layouts, surface.parentElement.clientWidth);
      if (next.id !== layout.id) guarded(() => mount(next));
      else guarded(() => timeline?.resize());
    });
  });
  reduced.addEventListener('change', () => {
    if (!reduced.matches) return;
    wantsMotion = false;
    guarded(syncMotion);
  });
  document.addEventListener('visibilitychange', () => { suspended = document.hidden; guarded(syncMotion); });
  global.addEventListener('pagehide', event => {
    if (event.persisted) { suspended = true; guarded(syncMotion); }
    else { disposed = true; generation += 1; timeline?.destroy(); if (resizeFrame !== null) cancelAnimationFrame(resizeFrame); }
  });
  global.addEventListener('pageshow', event => { if (event.persisted) { suspended = document.hidden; guarded(syncMotion); } });
  guarded(start);
})(typeof window === 'undefined' ? globalThis : window);
