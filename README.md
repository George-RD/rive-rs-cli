# rive-cli

`rive-cli` creates, validates, examines, and renders Rive (`.riv`) animations from JSON SceneSpec files. It is a Rust CLI for the write side of the Rive binary format; generated files are intended to load in Rive runtimes without requiring the Rive editor.

## Workflow

Start from a known-good scene, make the scene your own, compile it, validate the binary, then render it with the real Rive canvas runtime:

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

### Other commands

```bash
rive-cli --list-presets
rive-cli ai generate --template spinner -o output.riv
rive-cli ai lab --suite evals/suites/prompt_lab.v1.json
```

`ai generate` accepts either `--prompt` or `--template`; `ai lab` runs a suite given by `--suite`. The optional MCP server is built with `--features mcp`.

## For AI agents

Read [`skills/rive-animation/SKILL.md`](skills/rive-animation/SKILL.md) before authoring a scene. It describes the scaffold → discover → generate → validate → render loop and the runtime constraints that structural validation alone cannot catch.

The ten scenes in [`showcase/`](showcase/) are a gallery of working examples: six basics authored end to end by a fresh-context agent, and four advanced scenes covering embedded fonts, path morphing, embedded imagery and pointer-driven state machines. Use `rive-cli describe <type>` rather than guessing fields or animation properties.

## Site and promo

[`site/`](site/) is a dependency-free page that plays the committed `showcase/*.riv` live in the vendored Rive runtime — the animations on it are the tool's own output, not recordings. Preview it with `node site/serve.js`; `.github/workflows/pages.yml` publishes it, and `node tests/playwright/site-validation.js` asserts every scene paints, the interactive controls change the render, and the console is clean.

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
