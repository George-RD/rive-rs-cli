from pathlib import Path


def replace(name, before, after):
    p = Path(name)
    s = p.read_text()
    assert before in s, name
    p.write_text(s.replace(before, after))


replace('site/playback.js', '''    function resize() {
      if (instance && !destroyed) instance.resizeDrawingSurfaceToCanvas();
    }''', '''    function resize() {
      if (!instance || destroyed) return;
      if (mode !== "stateMachine") {
        instance.resizeDrawingSurfaceToCanvas();
        detachScheduledFrame();
        return;
      }
      instance.pause([stateMachine]);
      detachScheduledFrame();
      try {
        instance.resizeDrawingSurfaceToCanvas();
      } finally {
        instance.play([stateMachine]);
        detachScheduledFrame();
        instance.lastRenderTime = CLOCK_ORIGIN_MS + stepsAdvanced * stepMs;
      }
      settleAtCurrentFrame();
    }''')
replace('tests/playwright/site-playback-contract.js', '''    resizeDrawingSurfaceToCanvas() {}
    play() {}
    pause() {}''', '''    resizeDrawingSurfaceToCanvas() {
      if (options.resizeDrawsWallClock) this.draw(20000);
    }
    play() { this.playing = true; }
    pause() { this.playing = false; }''')
replace('tests/playwright/site-playback-contract.js', 'if (this.lastRenderTime !== 0) this.elapsedMs += timeMs - this.lastRenderTime;', 'if (this.playing && this.lastRenderTime !== 0) this.elapsedMs += timeMs - this.lastRenderTime;')
p = Path('tests/playwright/site-playback-contract.js')
p.write_text(p.read_text() + '''
for (const fps of [30, 60]) {
  test(`resize cannot advance a paused state machine or contaminate its next tick at ${fps}fps`, async (t) => {
    const harness = playbackHarness({ fps, resizeDrawsWallClock: true });
    t.after(() => harness.cleanup());
    const { timeline, canvases } = harness;
    await timeline.ready;
    await timeline.seekToFrame(30);
    const elapsed = canvases[0].rendered.elapsedMs;
    timeline.resize();
    assert.equal(canvases[0].rendered.elapsedMs, elapsed);
    assert.equal(timeline.currentFrame, 30);
    assert.equal(timeline.isPlaying, false);
    await timeline.seekToFrame(31);
    assert.ok(Math.abs(canvases[0].rendered.elapsedMs - elapsed - 1000 / fps) < 0.0001);
  });
}
''')
replace('site/page.js', "    surface.dataset.state = 'error';", "    surface.style.removeProperty('display');\n    surface.dataset.state = 'error';")
replace('site/page.css', '.page-fallback { padding:', '.page-fallback { position: absolute; top: 0; padding:')
replace('site/page.js', '  let resizeFrame = null;', '  let resizeFrame = null, motionGeneration = 0;')
replace('site/page.js', '''    const current = timeline;
    if (!current) return;
    const playing = wantsMotion && !suspended;
    await current.pause();
    await current.setInput(layout.inputs.paused, !wantsMotion);
    if (playing) await current.play();
    updateSemantics();''', '''    const token = ++motionGeneration;
    const current = timeline;
    if (!current) return;
    const playing = wantsMotion && !suspended;
    await current.pause();
    if (token !== motionGeneration || current !== timeline || disposed) return;
    await current.setInput(layout.inputs.paused, !wantsMotion);
    if (token !== motionGeneration || current !== timeline || disposed) return;
    if (playing) await current.play();
    updateSemantics();''')
replace('site/page.js', '''      mountControls();
      surface.dataset.state = 'ready';''', '''      mountControls();
      await syncMotion();
      if (disposed || token !== generation) { current.destroy(); return; }
      surface.dataset.state = 'ready';''')
replace('site/page.js', '''      fallback.hidden = true;
      await syncMotion();''', '''      fallback.hidden = true;''')
replace('site/page.js', 'else timeline.resize();', 'else guarded(() => timeline?.resize());')
replace('tests/playwright/rive-page-validation.js', '''      const frozen = await page.locator('#rive-page').screenshot();
      await delay(200);
      assert.deepEqual(await page.locator('#rive-page').screenshot(), frozen, `${name}: reduced motion moved`);''', '''      const frozen = await page.locator('#rive-page').screenshot();
      await delay(200);
      assert.deepEqual(await page.locator('#rive-page').screenshot(), frozen, `${name}: reduced motion moved`);
      const pixelsBeforeResize = await page.locator('#rive-page').evaluate(canvas => canvas.toDataURL());
      await page.setViewportSize({ width, height: height + 80 });
      await delay(150);
      assert.equal(await page.locator('#rive-page').evaluate(canvas => canvas.toDataURL()), pixelsBeforeResize, `${name}: resizing advanced paused artwork`);
      await page.setViewportSize({ width, height });
      await delay(150);''')
replace('tests/playwright/rive-page-validation.js', '''    assert.ok(await failed.locator('#page-fallback a[href="text.html"]').isVisible());
    await failed.close();''', '''    const link = failed.locator('#page-fallback a[href="text.html"]');
    assert.ok(await link.isVisible());
    assert.ok((await link.boundingBox()).y < 844, 'failure navigation is below the fold');
    await screenshot(failed, blocked.includes('wasm') ? 'failed-wasm' : blocked.includes('manifest') ? 'failed-manifest' : 'failed-artifact');
    await failed.close();''')
Path(__file__).unlink()
