# Changelog

All notable changes to this project are documented in this file.

The format is based on Keep a Changelog, and this project follows Semantic Versioning.

## [Unreleased]

### Added

- `decompile` command that renders a `.riv` file as structured JSON.
- `ai generate` and `ai lab` commands for prompt-driven scene generation and prompt-lab evaluation runs.
- Optional MCP stdio server behind `--features mcp`, exposing generate/validate/inspect/decompile as JSON-RPC tools.
- Artboard size presets and the `--list-presets` flag.
- Global `--json` flag for `--list-presets`, plus artboard-aware `inspect` filters (`--artboard-index`, `--artboard-name`, `--local-index`, `--type-key`, `--type-name`, `--object-index`, `--property-key`).
- E2E coverage for every fixture in `tests/fixtures/`, including the three `invalid_*` fixtures, which now assert specific error text.

### Fixed

- **Table of Contents backing-code packing.** The encoder wrote, and the parser read, 16 two-bit backing codes per `u32`, but the runtime reloads a fresh `u32` every 4 codes (`rive-runtime/include/rive/runtime_header.hpp:87-104`). Every file with more than 4 ToC keys was misaligned by 4 bytes per extra word. `rive-cli validate demo/riv/reference/official_test.riv` previously failed with `first object should be Backboard (type 23), got type 0`; it now reports 407 valid objects. The committed reference decompiles were regenerated.
- **The Table of Contents was always empty.** `encode_riv` selected only keys whose backing type was unknown, while `encode_toc` panicked on exactly those keys, so no ToC was ever emitted. The encoder now declares every property key the file writes, which is what lets a runtime skip object types it does not recognise instead of abandoning the object stream. This alone made `transition_comparators` load in the WASM runtime.
- **`Mesh` omitted its required `triangleIndexBytes` property** (key 223). The runtime allocates its index buffer from that property; without it the buffer stays null and import returns `InvalidObject`. `Mesh` now always emits it.
- **`NSlicer` carried property keys 697-700.** `initialWidth`, `initialHeight`, `width`, and `height` belong to `NSlicedNode`; `NSlicerBase` declares no fields and derives its dimensions from the parent `Image`. The four properties moved to `NSlicedNode` across the object model, `SceneSpec`, and the builder.
- **`tests/fixtures/follow_path_constraint.json`** parented the constraint to the artboard while targeting a sibling, which the runtime rejected as `Dependency cycle!`. The constraint is now a child of the shape it constrains.
- **`tests/fixtures/nslicer.json`** nested an `Image`/`NSlicer` inside an `NSlicedNode`. These are two mutually exclusive constructs and the combination stalled the runtime. The fixture now exercises each in its own artboard.

### Changed

- `docs/scene.schema.v1.json` is now generated from the Rust `SceneSpec` types and covers all 202 object types instead of a hand-written subset of 14. A unit test fails if the committed file drifts; regenerate with `UPDATE_SCENE_SCHEMA=1 cargo test scene_schema_file_is_in_sync`. The MCP resource `schema://scene/v1` serves the generated schema.
- The curated 14-type schema moved to `docs/ai/scene-prompt-schema.json` and remains the contract embedded in the `ai generate` system prompt, which now states the type list is closed.
- The binary consumes the `rive_cli` library instead of re-declaring its modules, so the crate is compiled and unit-tested once instead of twice.
- CI lints and tests all targets and all features, so the `mcp` module and test code are covered.

### Removed

- The `target` field on `draggable_constraint`, `scroll_constraint`, and `scroll_bar_constraint`. These rive-runtime types do not derive from `TargetedConstraint`, so the field was accepted and discarded.
- Agent-orchestration scaffolding unrelated to the product: `.sauron/`, `.claude/commands/`, `.sisyphus/`, `docs/orchestration/`, `docs/history/`, `specs/`, both stale implementation plans, the multi-model race harness (`scripts/run_race.sh`, `data/`), and two partial duplicates of `scripts/vision_gate_orchestrator.py`.

## [0.1.0] - 2026-02-24

### Added

- Initial feature-complete CLI with `generate`, `validate`, and `inspect` commands.
- Encoder support for drawing, animation, state machines, rigging, constraints, text, assets, layout, and data binding objects.
- Validator and inspect tooling with JSON and filtered diagnostics output.
- Runtime compatibility checks via Playwright regression harness.
- Release automation and cookbook documentation.
