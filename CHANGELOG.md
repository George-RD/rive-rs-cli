# Changelog

All notable changes to this project are documented in this file.

The format is based on Keep a Changelog, and this project follows Semantic Versioning.

## [Unreleased]

### Added

- `render` renders deterministic animation or state-machine frames through embedded `assets/rive.js` and `assets/rive.wasm`, driving headless Chromium over CDP from Rust. It supports frame lists/ranges, `--fps`, `--animation`, `--state-machine`, repeatable typed `--input`, `--artboard`, `--width`, `--height`, `--scale`, `--background`, `--contact-sheet`, `--preview`, `--browser`, and `--json`, and writes PNGs plus `manifest.json` and optional `preview.txt`.
- `schema`, `types`, and `describe` expose the SceneSpec schema, object-type catalogue, valid parents, fields, enum values, and per-type animatable properties.
- `new` scaffolds the `shape`, `animated`, `gradient`, `spinner`, `button`, and `multi` starter scenes.
- Added the primary AI-agent authoring skill at `skills/rive-animation/SKILL.md` and six authored SceneSpec examples in `showcase/`.

### Fixed

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

- Superseded `skills/opencode/rive-animation.md` guidance, whose animation table contradicted the authoritative discovery resolver (including `stroke.thickness` and trim `start`/`end`/`offset`). Claude Code command aliases remain available.

## [0.1.0] - 2026-02-24

### Added

- Initial feature-complete CLI with `generate`, `validate`, and `inspect` commands.
- Encoder support for drawing, animation, state machines, rigging, constraints, text, assets, layout, and data binding objects.
- Validator and inspect tooling with JSON and filtered diagnostics output.
- Runtime compatibility checks via Playwright regression harness.
- Release automation and cookbook documentation.
