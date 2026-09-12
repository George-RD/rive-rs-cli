from pathlib import Path


def replace(name, before, after):
    p = Path(name)
    s = p.read_text()
    assert before in s, name
    p.write_text(s.replace(before, after))


replace('site/page.js', "    const response = await fetch('scenes/manifest.json',", "    global.rive.RuntimeLoader.setWasmFallbackUrl(null);\n    const response = await fetch('scenes/manifest.json',")
replace('site/authoring/page.js', 'const { value, scalar,', 'const { scalar,')
replace('site/authoring/page.js', ", pale: '#E8E0CB', white: '#FBF8EF'", '')
replace('site/authoring/page.js', 'const [cx, cy, cw, ch]', 'const [cx, cy, cw]')
replace('docs/site/rive-page.md', 'No-JavaScript or failed WASM/file loads retain ordinary navigation links.', 'No-JavaScript or failed WASM/file loads retain ordinary navigation links. The\npage disables the runtime CDN fallback so a failed vendored WASM load is handled\nlocally, rather than silently fetching a different binary from a third party.')
Path(__file__).unlink()
