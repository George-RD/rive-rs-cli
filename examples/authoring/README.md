# AuthoringSpec v0 examples

These fixtures exercise the typed authoring frontend before it lowers to the
canonical `SceneSpec` object graph.

| Fixture | Purpose |
|---|---|
| `component-badges.v0.json` | Reusable components, parameters, and instance overrides |
| `text-label.v0.json` | Typed literal text and semantic text styling |
| `typed-motion.v0.json` | Typed visual composition, relative image asset, poses, and timeline motion |
| `raw-pulse.v0.json` | The explicit raw SceneSpec escape hatch for unsupported concepts |
| `complex-static-showcase.v0.json` | A complex static composition built without raw scene, motion, or behavior escapes |

The complex static showcase combines reusable components, expression-backed
parameters, linear and radial gradients, trimmed strokes, text, grid, radial,
mirror, distribute, and along-path patterns, plus align, center, offset, and
spacing constraints.

Compile the high-level typed-motion fixture directly:

```bash
cargo run -- authoring compile examples/authoring/typed-motion.v0.json -o typed-motion.riv
cargo run -- authoring compile examples/authoring/typed-motion.v0.json -o typed-motion.riv --file-id 42 --json
```

Print the high-level AuthoringSpec schema with:

```bash
cargo run -- authoring schema
```

`authoring compile` is the typed, semantic input path. `generate` remains the
explicit expert path for raw `SceneSpec` JSON:

```bash
cargo run -- generate scene.json -o output.riv
```

Run the durable example and CLI contracts with:

```bash
cargo test --test authoring_examples
cargo test --test authoring_cli
```

The example contract lowers every fixture twice to prove deterministic
SceneSpec and source-map output, then builds the canonical object graph. The
CLI contract compiles typed visual and motion content through the shared
SceneSpec compilation seam, validates the resulting `.riv`, preserves file IDs
and input-relative assets, and checks full source-mapped JSON diagnostics.
