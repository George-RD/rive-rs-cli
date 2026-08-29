# Changelog

All notable changes to this project are documented in this file.

The format is based on Keep a Changelog, and this project follows Semantic Versioning.

## [Unreleased]

### Added

- **AuthoringSpec-first AI generation.** Prompt-based `ai generate` now asks providers for the live high-level AuthoringSpec contract, focuses context on the relevant static/animated/interactive schema slice, includes the closest checked-in showcase plus authored-to-runtime source-map evidence, lowers through the canonical Authoring compiler, and retains SceneSpec as the explicit expert/template escape path.
- **Incremental AuthoringSpec AI repair.** Typed lowering failures can request one stable-ID insert/move/remove/replace operation at a time and apply it atomically through the existing Authoring operation seam instead of regenerating the whole authored document. Prompt eval cases retain AuthoringSpec, repair attempts, source maps, runtime evidence, and semantic evidence separately.
- **AuthoringSpec over MCP.** The MCP server exposes the live `schema://authoring/v0` resource and a `generate_authoring` tool that compiles through the typed frontend and returns source-map evidence alongside the output.
- `render` renders deterministic animation or state-machine frames through embedded `assets/rive.js` and `assets/rive.wasm`, driving headless Chromium over CDP from Rust. It supports frame lists/ranges, `--fps`, `--animation`, `--state-machine`, repeatable typed `--input`, `--artboard`, `--width`, `--height`, `--scale`, `--background`, `--contact-sheet`, `--preview`, `--browser`, and `--json`, and writes PNGs plus `manifest.json` and optional `preview.txt`.
- `schema`, `types`, and `describe` expose the SceneSpec schema, object-type catalogue, valid parents, fields, enum values, and per-type animatable properties.
- `new` scaffolds the `shape`, `animated`, `gradient`, `spinner`, `button`, and `multi` starter scenes.
- Added the primary AI-agent authoring skill at `skills/rive-animation/SKILL.md` and six authored SceneSpec examples in `showcase/`.
- **Path morphing.** Vertex objects are animatable: `straight_vertex` exposes `x`/`y`/`radius`, the cubic vertices additionally expose their handle rotations and distances. `x`/`y` on a vertex now resolve to `VERTEX_X`/`VERTEX_Y` instead of silently falling back to the node transform keys, so keyframing a `points_path` deforms the silhouette.
- **Asset embedding.** `font_asset` and `image_asset` accept a `source` path, resolved relative to the scene file's directory, and the referenced bytes are embedded in the `.riv` as a `FileAssetContents` object. Adds `PropertyValue::Bytes`, `BinaryWriter::write_byte_array`, and byte-aware parsing in the validator.
- **Name-based references.** `image.asset`, `text_style.font_asset`, `text_value_run.style`, `blend_state1d.input` and `blend_animation1_d.animation` accept names instead of hand-computed indices; supplying both a name and an index is an error. `state_machine_listener.listener_type` accepts `enter`/`exit`/`down`/`up`/`move`/`event`/`click`.
- **Scheduled and pointer-driven interaction.** `render --input NAME=VALUE@FRAME` applies an input when the stepper reaches that frame, and `render --pointer EVENT:X,Y@FRAME` dispatches a real pointer event in artboard coordinates so Rive's own listener handling runs. Both require `--state-machine` and are recorded in `manifest.json`.
- **Four advanced showcases** in `showcase/`: `wordmark` (embedded font), `liquid_loader` (path morphing), `textured_scene` (embedded PNG) and `control_panel` (pointer events plus a 1D blend state). Each carries a measured capability proof in `showcase/README.md`.
- **A licensed asset set** at `assets/fonts/` (Inter subset, SIL OFL) and `assets/textures/`, with provenance recorded in `assets/README.md`.
- **A site** at `site/`, published to GitHub Pages by `.github/workflows/pages.yml`. Each card plays an official Rive-authored `.riv` beside our reproduction of it, with the measured pixel difference and object counts beneath, so the page shows a measured gap rather than asserting one. `tests/playwright/site-validation.js` asserts every canvas paints and that the figures on the page match `parity/results.json`; it runs as a CI job.
- **A promo** at `promo/`: a Remotion composition built from PNG sequences that `rive-cli render` produced, so every frame in the video is a frame the test suite verifies.
- **`compare` measures one `.riv` against another.** It decompiles both, renders both through headless Chromium at identical geometry, and reports a table of every Rive type whose count differs plus the share of pixels that differ per frame. Separate `--reference-animation`/`--candidate-animation` and `--reference-state-machine`/`--candidate-state-machine` flags because a reproduction need not use the official file's internal names. It exits non-zero only when `--max-pixel-diff` is supplied and exceeded; there is deliberately no default threshold.
- **An official corpus with provenance** at `parity/official/`, replacing `demo/riv/reference/`. Fetched entries in `parity/official/manifest.json` pin an upstream repository, commit SHA and path; legacy `in-repo` entries are checksum-pinned and verified locally. `parity/fetch-official.sh` re-downloads fetched entries and verifies every entry's recorded checksum and size.
- **A reproduction ladder** at `parity/reproductions/`, authored from the official decompiles and measured with `compare`: `button.riv` (64 objects, embedded variable font, text, state machine) at **0.0000%** across frames 0/15/30/45, and `coffee_loader.riv` (250 objects, five state-machine layers, a 1D blend state, ninety keyframes) at **0.2833%**. `parity/collate-results.sh` refreshes `parity/results.json`, enforcing the 5% gate and rejecting missing type names. Full findings in `docs/parity.md`.
- Static transform coverage is explicit: `text` accepts `x` and `y`; `node`, `shape`, `ellipse`, `rectangle`, `triangle`, `polygon` and `star` accept `x`, `y`, `rotation`, `scale_x` and `scale_y`. `shape` also accepts `hidden`, and `stroke` accepts `transform_affects_stroke`.

### Changed

- AI/agent guidance now routes complex work through `rive-cli authoring schema` and `authoring compile`; the retained SceneSpec skill is explicitly a bounded expert escape-hatch reference and relies on live CLI discovery instead of copied schema tables. The legacy OpenCode validation entry now redirects to this Authoring-first verification contract.
- Prompt `ai lab` cases now generate and retain AuthoringSpec/source-map evidence before canonical SceneSpec compilation, so the existing static, animated, interactive, runtime, reproducibility, and drift gates evaluate the same authored representation used by the AI generation path.
- The site's landing hero now plays `parity/reproductions/coffee_loader.riv`, the file this tool generated, rather than the official one. The page's headline animation is now the tool's own output. `site/stage.js` also scans `landing.js` for referenced scenes, so a future hero swap cannot publish a missing file.
- README rewritten for users rather than for the repository: what the tool is for, badges, and a link to the published site and verification lab. Adds the `compare` reference section, which was previously undocumented.

### Known gaps

- A pair of transitions between two animation states conditioned on the same bool (`A -> B` when true, `B -> A` when false) makes the runtime fall back to `A` a few frames after reaching `B`. Author one-way transitions, or drive the return edge from a second input. Recorded in `docs/parity.md`.
- Giving both edges of a transition cycle `duration: 0` makes the layer log `exceeded max iterations` and stop transitioning. Give condition-driven transitions a non-zero duration.
- `image` has static `x`/`y` but no static `scale_x`/`scale_y`/`rotation`; hold a constant value with a single-value keyframe pair until it gains static fields.
- `feather` is ignored by the vendored canvas runtime, so feathered edges do not render.

### Fixed

- **`demo/serve.js` returned 500 for `/`.** The root path resolved to the `demo/` directory and was handed to `fs.readFile`, which fails with `EISDIR`. Directory paths now resolve to `index.html`, so `tests/playwright/demo-validation.js` reaches the page instead of polling until its 120-second timeout.
- **File assets were nested inside the artboard.** Assets are now hoisted to file scope between the Backboard and the first artboard, where Rive's own exporter puts them. Previously every `parentId` after an asset was off by one against the runtime's index space, and a scene containing an asset plus any drawable made `@rive-app/canvas` hang indefinitely rather than reporting an error.
- **`text_style` emitted the abstract `TextStyle` (573) rather than `TextStylePaint` (137).** Only the subclass implements `ShapePaintContainer`, so text never drew regardless of font or fill; every committed text baseline was a flat colour. `text_style` now emits 137 and accepts `fill`/`stroke` children.
- **`blend_state1d` emitted the abstract `BlendState1D` (527) alongside `BlendState1DInput` (76).** The layer entered the input-less base state, so 1D blends produced nothing. A single `BlendState1DInput` is now emitted and the blend interpolates across its input range.
- **Unscheduled `render --input` was discarded.** Inputs were applied before `instance.play()`, which rebuilds the state machine and resets them; they are now applied immediately after the first advance, through the same path as scheduled inputs.
- **Cross-kind asset references.** An `image` naming or indexing a `font_asset` (or a `text_style` pointing at an `image_asset`) generated a file the runtime could not resolve. Named and numeric references are now checked against the declared asset kind.
- **`blend_state1d.input` accepted a bool or trigger.** The named path bypassed the number-input check the numeric path already had, emitting a blend the runtime cannot evaluate.
- **Scheduled input failures were swallowed.** An unknown input name at a frame above zero left the render succeeding, capturing frames without the input and recording it as applied. The harness now fails the render.
- **Asset `source` could escape the project.** The resolved path is canonicalised and must stay inside the nearest ancestor holding `Cargo.toml`, `.git` or `package.json`, so neither `..` nor a symlink reaches an arbitrary file.
- **Keyframes could not target objects nested under a `node`.** Type-key collection now recurses through `node` and `image` children. A keyframe naming an object that appears more than once in an artboard is now a clear error instead of silently binding to whichever was registered last.
- Playwright deterministic seeking now passes the mounted animation name to `scrub([name], seconds)`; `scrub(undefined, t)` was a silent no-op in `@rive-app/canvas` 2.39.1.
- Playwright captures now set the canvas backing-store dimensions instead of upscaling the browser's default 300×150 store to 1024×1024.
- State-machine manual stepping now resolves the WASM animation frame after each stepped draw.
- `generate` rejects state-machine layers missing an entry or exit state; the runtime rejects the whole file even though `validate` accepts it.
- `generate` rejects keyframes for properties the target object does not own, including `width`/`height` on a `shape`, `opacity` on a fill/stroke, and `stroke.thickness`; the error lists the target type and allowed properties.
- `describe` no longer advertises transform properties on types that do not own them (for example `linear_gradient` now reports `start_x`/`start_y`/`end_x`/`end_y`/`opacity`), so discovery and `generate` cannot disagree.
- `describe` examples are validated through `build_scene` before being emitted, so every published example compiles; types needing extra scene context say so instead of emitting an invalid snippet.
- `render` output is transparent by default again; screenshots previously composited onto opaque white because no CDP transparent background override was set.
- `render --preview --json` keeps stdout parseable by writing the human-readable coverage grid to stderr.
- `render` rejects non-finite or non-positive `--fps`, guards frame-range arithmetic against `u32` overflow, and surfaces seek failures instead of silently capturing the wrong frame.
- `render --input` validates the requested kind against the state machine input's real runtime type, so a trigger value against a bool input is an error rather than a silent no-op.
- Chromium auto-discovery probes current Playwright cache layouts, including the architecture-qualified `chrome-mac-arm64` and `chrome-headless-shell-*` directories.
- Every failing command honours the documented `{ok, command, code, message}` JSON envelope under `--json`, including `render`, the discovery commands, and clap usage errors.

### Removed

- Stale duplicate `.opencode/skills/rive-animation`, `rive-scene-schema`, and `rive-anti-patterns` skill files whose copied low-level rules contradicted the authoritative resolver/runtime contract. The remaining OpenCode validation entry is intentionally a thin redirect to live CLI discovery and canonical Authoring-first guidance.
- Superseded `skills/opencode/rive-animation.md` guidance, whose animation table contradicted the authoritative discovery resolver (including `stroke.thickness` and trim `start`/`end`/`offset`). Claude Code command aliases remain available.

## [0.1.0] - 2026-02-24

### Added

- Initial feature-complete CLI with `generate`, `validate`, and `inspect` commands.
- Encoder support for drawing, animation, state machines, rigging, constraints, text, assets, layout, and data binding objects.
- Validator and inspect tooling with JSON and filtered diagnostics output.
- Runtime compatibility checks via Playwright regression harness.
- Release automation and cookbook documentation.
