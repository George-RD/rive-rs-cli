# AuthoringSpec CLI

`rive-cli` exposes two explicit JSON input levels.

- `authoring compile` accepts the high-level `AuthoringSpec` contract. It lowers semantic visual and motion declarations deterministically, validates the resulting canonical `SceneSpec`, and compiles through the shared SceneSpec compilation seam.
- `generate` accepts raw `SceneSpec` JSON. Use it when direct control of the runtime object graph is required.

The CLI does not guess the input format.

## Compile AuthoringSpec

```bash
rive-cli authoring compile authoring.json -o output.riv
rive-cli authoring compile authoring.json -o output.riv --file-id 42 --json
```

Relative font and image asset paths are resolved from the AuthoringSpec file's directory. `--file-id` is written to the Rive header exactly as it is for `generate`.

Successful JSON output includes:

- `ok`;
- `bytes_written`;
- `output_path`;
- the authored `source_map` connecting AuthoringSpec paths and IDs to lowered SceneSpec paths and runtime names.

Lowering failures return one JSON envelope with `code: "lowering-failed"` and the complete ordered `diagnostics` array. Final canonical compilation failures, including asset-loading errors, preserve the same diagnostics shape at `$.lowered_scene` and retain the stable compilation error code. Each diagnostic contains `path`, `code`, and `message`.

## Discover the schema

```bash
rive-cli authoring schema
rive-cli authoring schema --compact
```

The checked-in schema remains available at `docs/authoring.schema.v0.json` for editors and generators.

## Raw SceneSpec path

```bash
rive-cli generate scene.json -o output.riv
rive-cli schema
```

`generate` and the top-level `schema` command remain the raw SceneSpec interface. Unsupported high-level concepts may also use AuthoringSpec's explicit raw escape hatches; they are never inserted implicitly.
