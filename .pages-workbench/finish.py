from pathlib import Path


def replace(name, before, after):
    p = Path(name)
    s = p.read_text()
    assert before in s, name
    p.write_text(s.replace(before, after))


replace('site/page.js', "          if (entry.action === 'replay') { await timeline.pause(); await timeline.seekToFrame(0); await syncMotion(); announcement.textContent = 'Motion reset.'; }", """          if (entry.action === 'replay') {
            const current = timeline;
            await current.pause();
            if (current !== timeline || disposed) return;
            await current.setInput(layout.inputs.morph, morph);
            await current.seekToFrame(0);
            if (current !== timeline || disposed) return;
            await syncMotion();
            announcement.textContent = 'Motion reset.';
          }""")
replace('site/authoring/page.js', "'Drag to morph'", "'Drag the slider'")
replace('tests/playwright/rive-page-validation.js', "document.querySelector('#page-surface').dataset.layout === expected, width <", "document.querySelector('#page-surface').dataset.layout === expected && document.querySelector('#page-surface').dataset.state === 'ready', width <")
replace('tests/playwright/rive-page-validation.js', "    await page.getByRole('button', { name: 'Play motion' }).click();\n    await page.emulateMedia", """    await page.locator('[data-control="playback"]').evaluate(element => { for (let i = 0; i < 10; i += 1) element.click(); });
    await delay(200);
    assert.equal(await page.locator('#page-surface').getAttribute('data-playing'), 'false', 'rapid playback toggles left stale playback work');
    await page.getByRole('button', { name: 'Play motion' }).click();
    await page.emulateMedia""")
Path(__file__).unlink()
