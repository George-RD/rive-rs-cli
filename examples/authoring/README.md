# AuthoringSpec v0 examples

These fixtures exercise the typed authoring frontend before it lowers to the
canonical `SceneSpec` object graph.

| Fixture | Purpose |
|---|---|
| `component-badges.v0.json` | Reusable components, parameters, and instance overrides |
| `text-label.v0.json` | Typed literal text and semantic text styling |
| `raw-pulse.v0.json` | The explicit raw SceneSpec escape hatch for unsupported concepts |
| `complex-static-showcase.v0.json` | A complex static composition built without raw scene, motion, or behavior escapes |

The complex static showcase combines reusable components, expression-backed
parameters, linear and radial gradients, trimmed strokes, text, grid, radial,
mirror, distribute, and along-path patterns, plus align, center, offset, and
spacing constraints.

Run the durable example contract with:

```bash
cargo test --test authoring_examples
```

The contract lowers every fixture twice to prove deterministic SceneSpec and
source-map output, then builds the canonical object graph. The complex showcase
also encodes a `.riv` byte stream and passes it through the structural validator.
