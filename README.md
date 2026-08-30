# rive-cli

[![CI](https://github.com/George-RD/rive-rs-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/George-RD/rive-rs-cli/actions/workflows/ci.yml)
[![Pages](https://github.com/George-RD/rive-rs-cli/actions/workflows/pages.yml/badge.svg)](https://github.com/George-RD/rive-rs-cli/actions/workflows/pages.yml)
[![Live demo](https://img.shields.io/badge/live-showcase-2f6df6)](https://george-rd.github.io/rive-rs-cli/)
[![Rust](https://img.shields.io/badge/rust-edition%202024-b7410e?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-black)](LICENSE)

**Write Rive animations as JSON. Compile them to real `.riv` files. Prove they work before you ship.**

[Live showcase and verification lab](https://george-rd.github.io/rive-rs-cli/) · [Skill file for agents](skills/rive-animation/SKILL.md) · [Scene schema](docs/scene.schema.v1.json)

An AI agent can write an animation scene in seconds. It cannot tell you whether a state is missing,
a property sits on the wrong object, or the file quietly breaks in the real Rive player. `rive-cli`
turns those unknowns into checks you can run again, in a terminal or in CI.

The landing page now leads with an original AuthoringSpec animation built by rive-cli. The separate
[Made with rive-cli showcase](https://george-rd.github.io/rive-rs-cli/showcase.html) keeps original
source and artifacts inspectable and includes a clearly labelled Horaxon production-consumer proof.
The [Verification Lab](https://george-rd.github.io/rive-rs-cli/lab.html) keeps upstream reproduction
fidelity separate, including the coffee-loader comparison and its committed measured evidence.

## What you get

- **A file format you can edit.** Scenes are plain JSON in your repo, so they diff, review, and merge.
- **A binary the runtime accepts.** `generate` writes the `.riv` format the Rive players read, with no editor in the loop.
- **Answers a tool can act on.** Object counts, missing types, pixel differences, content bounds, and a text coverage map, all available as JSON.
- **A self-describing contract.** `schema`, `types`, and `describe` list every field, parent, and animatable property, so an agent never has to guess.

## Workflow

Start from a known-good scene, make it your own, compile it, validate the binary, then render it
with the real Rive canvas runtime:

```bash
rive-cli new spinner -o scene.json
# Edit scene.json.
rive-cli generate scene.json -o out.riv
rive-cli validate out.riv
rive-cli render out.riv --frames 0,15,30,45 --preview -o frames/
```

`render --preview` prints an ASCII coverage map, dominant-colour percentage, and non-background bounds for each frame. It also writes `preview.txt` and `manifest.json`, alongside the PNG files, so a non-visual workflow can still inspect what was rendered.

## Commands

### Generate, validate, and examine files

```bash
rive-cli generate scene.json -o output.riv
rive-cli validate output.riv
rive-cli inspect output.riv --type-name Shape
rive-cli decompile output.riv --json
```

- `generate INPUT` accepts `-o, --output`, `--file-id`, and `--json`.
- `validate FILE` accepts `--json`.
- `inspect FILE` accepts `--json`, `--artboard-index`, `--artboard-name`, `--local-index`, `--type-key`, `--type-name`, `--object-index`, and `--property-key`.
- `decompile FILE` accepts `--json`.

`--json` is also available globally and on each command that produces structured output. Errors in JSON mode use the stable envelope `{ok, command, code, message}`.

### Discover the authoring contract

```bash
rive-cli schema
rive-cli schema --compact
rive-cli types --category paint
rive-cli describe ellipse --json
```

- `schema` prints the complete SceneSpec JSON schema; `--compact` removes indentation.
- `types` lists usable object types; `--category` filters the list.
- `describe TYPE` reports the type's fields, enum values, valid parents, and animatable properties. Its animation-property resolver is the same one `generate` uses.

### Scaffold scenes

```bash
rive-cli new --list
rive-cli new animated -o scene.json
```

`new TEMPLATE` writes a known-good SceneSpec to standard output or, with `-o, --output`, to a file. The available templates are `shape`, `animated`, `gradient`, `spinner`, `button`, and `multi`.

### Render PNG frames

```bash
rive-cli render output.riv
rive-cli render output.riv --frames 0,15,30,45 -o frames/
rive-cli render output.riv --frames 0..120:10 --width 800 --height 600
rive-cli render output.riv --animation spin --contact-sheet
```

`render FILE` drives headless Chromium directly over CDP from Rust; it does not use Node or Playwright. A Chrome or Chromium executable is therefore required. For a non-standard browser location, set `$RIVE_CHROME` or pass `--browser /path/to/chromium`.

Render options:

| Option | Purpose |
|---|---|
| `-o, --output DIR` | PNG and manifest output directory (default `renders`) |
| `--frames LIST_OR_RANGE` | Frame list such as `0,15,30`, or range such as `0..120:10` |
| `--fps FPS` | Frames per second used to convert indices to time |
| `--animation NAME` | Linear animation to scrub |
| `--state-machine NAME` | State machine to advance instead of an animation |
| `--input NAME=VALUE[@FRAME]` | Repeatable state-machine bool, number, or `trigger` input. `@FRAME` applies it when the stepper reaches that frame |
| `--pointer EVENT:X,Y@FRAME` | Repeatable pointer event (`down`, `up`, `move`, `enter`, `exit`) in artboard coordinates, dispatched through Rive's own listener handling |
| `--artboard NAME` | Artboard to render |
| `--width PX`, `--height PX`, `--scale RATIO` | Logical dimensions and device-pixel multiplier |
| `--background COLOR` | Background behind the artboard, for example `#202024` |
| `--contact-sheet` | Write a horizontal filmstrip in addition to individual frames |
| `--preview` | Print and write text coverage previews |
| `--browser PATH` | Override browser discovery |
| `--json` | Emit the render manifest as JSON |

Each frame in `manifest.json` records its PNG path, distinct-colour count, and `blank` flag, and the manifest also records the inputs and pointer events that were applied. Identical inputs produce byte-identical PNGs.

`--input` and `--pointer` both require `--state-machine`. Interaction is proved the same way animation is: render the same frames with and without the flag and require the frames before the scheduled frame to be byte-identical.

### Compare against a reference file

```bash
rive-cli compare official.riv ours.riv \
  --frames 0,15,30,45 \
  --reference-state-machine 'State Machine 1' \
  --candidate-state-machine 'State Machine 1' \
  --max-pixel-diff 5
```

`compare REFERENCE CANDIDATE` decompiles both files, renders both, and prints a per-type object
delta table plus a pixel difference for each frame. It exits non-zero only when you pass
`--max-pixel-diff PCT` and the worst frame goes over it, so it drops into CI as a gate. Frame,
size, background, animation, and state-machine flags mirror `render`, with `--reference-` and
`--candidate-` prefixes where the two files differ.

### Other commands

```bash
rive-cli --list-presets
rive-cli ai generate --template spinner -o output.riv
rive-cli ai lab --suite evals/suites/prompt_lab.v1.json
```

`ai generate` accepts either `--prompt` or `--template`; `ai lab` runs a suite given by `--suite`. The optional MCP server is built with `--features mcp`.

## For AI agents

Read [`skills/rive-animation/SKILL.md`](skills/rive-animation/SKILL.md) before authoring a scene. It describes the scaffold → discover → generate → validate → render loop and the runtime constraints that structural validation alone cannot catch.

The SceneSpec examples in [`showcase/`](showcase/) cover basics and advanced scenes including embedded fonts, path morphing, embedded imagery, and pointer-driven state machines. The public showcase also promotes a high-level AuthoringSpec example and retains the exact Horaxon artifact consumed in production plus its generating AuthoringSpec source with separate provenance. Use `rive-cli describe <type>` rather than guessing fields or animation properties.

## Site and parity lab

[`site/`](site/) is a dependency-free page published at
[george-rd.github.io/rive-rs-cli](https://george-rd.github.io/rive-rs-cli/). Everything on it plays
live in the vendored Rive runtime, so nothing is a recording. The landing hero uses the original
`examples/authoring/complex-animated-showcase.v0.riv` through the same browser playback seam as the
showcase and Verification Lab. The [showcase](https://george-rd.github.io/rive-rs-cli/showcase.html)
answers “what can it create?” with original work and explicitly labelled production-consumer evidence.
The [Verification Lab](https://george-rd.github.io/rive-rs-cli/lab.html) separately answers “how do we
know it is correct?” by putting official Rive files beside the copies `rive-cli` generated, with the
committed measured gap underneath.

Preview it with `node site/serve.js`. `.github/workflows/pages.yml` publishes it. Browser CI covers
landing/showcase/lab runtime paint, manifest staging, shared playback behavior, responsive layout,
reduced motion, provenance links, and parity figures sourced from `parity/results.json`.

[`parity/`](parity/) holds the official files, the JSON that reproduces them, and
`results.json`, the numbers the lab displays. `parity/fetch-official.sh` re-fetches the upstream
files and checks them against a pinned manifest.

[`promo/`](promo/) is a Remotion composition assembled from PNG sequences that `rive-cli render` produced, so every frame in the video is a frame the test suite verifies. See [`promo/README.md`](promo/README.md) to rebuild it.

## SceneSpec

Scene specs require `scene_format_version: 1`:

```json
{
  "scene_format_version": 1,
  "artboard": {
    "name": "Main",
    "width": 500,
    "height": 500,
    "children": []
  }
}
```

The complete generated schema is [`docs/scene.schema.v1.json`](docs/scene.schema.v1.json). Format and runtime-compatibility constraints are recorded in [`docs/format-spec.md`](docs/format-spec.md).

## Installation

Build from source:

```bash
cargo build --release
./target/release/rive-cli --help
```

Prebuilt binaries and platform packages are documented in [`docs/install.md`](docs/install.md).

## Testing

```bash
cargo test
npx -y -p playwright node tests/playwright/regression.js
npx -y -p playwright node tests/playwright/visual-regression.js
```

The Rust tests cover generation and structural validation. The runtime and visual regressions load generated files in the official Rive canvas runtime and compare actual renders against PNG baselines.

## Reference material

- [Rive binary format specification](https://rive.app/docs/runtimes/advanced-topic/format)
- [rive-runtime C++](https://github.com/rive-app/rive-runtime)
- [rive-rs Rust runtime](https://github.com/rive-app/rive-rs)

## License

MIT
