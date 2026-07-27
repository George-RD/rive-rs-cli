# Bundled assets

Runtime and fixture assets shipped with `rive-cli`.

| Path | Purpose | Licence |
|---|---|---|
| `rive.js`, `rive.wasm` | Vendored `@rive-app/canvas` 2.39.1 runtime used by `rive-cli render` and the Playwright suites | MIT (Rive) |
| `render-harness.html` | Deterministic frame stepper driven over CDP by `rive-cli render` | This repository |
| `fonts/Inter-Bold-Subset.ttf` | Inter, instanced to `wght=700 opsz=28` and subset to U+0020–U+007E for embedded-font fixtures and showcases | SIL Open Font License 1.1 — see `fonts/OFL.txt` |
| `fonts/OFL.txt` | Inter licence text, copied verbatim from the upstream Google Fonts distribution | SIL Open Font License 1.1 |
| `textures/aurora.png` | 256x256 RGBA gradient texture authored for this repository by script | This repository |

## Referencing assets from a scene

`font_asset` and `image_asset` accept a `source` path. It is resolved relative to the
directory containing the scene JSON, so a fixture in `tests/fixtures/` reaches this
directory with `../../assets/fonts/Inter-Bold-Subset.ttf`. The referenced bytes are
embedded in the generated `.riv`.

Two rules keep `source` honest. Absolute paths are rejected, so scenes stay portable.
The resolved path is canonicalised and must stay inside the project that owns the scene
— the nearest ancestor holding `Cargo.toml`, `.git` or `package.json` — so neither `..`
nor a symlink can pull an arbitrary file into a `.riv`. Embedding is refused outright on
surfaces with no scene file on disk, such as the MCP server.
