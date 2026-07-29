# AuthoringSpec v0

`AuthoringSpec` is the strict AI-facing and programmatic authoring frontend for `rive-cli`. It lowers into the existing canonical `SceneSpec`; it does not write `.riv` bytes directly and does not replace the builder or encoder.

## Versioning

- `authoring_format_version` is required and must be `0`.
- Unknown fields are rejected at every typed authoring layer.
- A breaking field, semantic, unit, naming, or lowering change requires a new authoring format version.
- Additive compiler capability may be introduced within v0 only when existing v0 documents lower to the same canonical `SceneSpec` and source map.
- `scene_format_version` remains independently versioned. v0 currently lowers to `SceneSpec` version `1`.

The generated JSON Schema is available through `authoring::authoring_schema()` and uses this stable identifier:

```text
https://github.com/George-RD/rive-rs-cli/docs/authoring.schema.v0.json
```

## Document model

A v0 document has four explicit graphs:

- `components`: reusable authored visual definitions with typed parameter defaults.
- `visual`: the root visual graph.
- `motion`: raw canonical animation escapes until the dedicated motion compiler lands.
- `behavior`: raw canonical state-machine escapes until the dedicated behavior compiler lands.

The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, groups, component instances, and raw `SceneSpec` objects. Broader primitives, bounded patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.

## Stable identity and runtime names

Every authored artboard, component, node, and raw fragment has an explicit stable `id`. Generated Rive runtime names are derived deterministically from the authored expansion path, including instance paths. The encoding is collision-resistant for distinct UTF-8 ids and does not depend on hash-map iteration or process state.

Lowering returns an `AuthoringSourceMap`. Each entry links:

- the authored id and JSON path;
- the component definition path when an instance was expanded;
- generated or declared runtime names;
- canonical `SceneSpec` JSON-pointer paths.

Raw escapes preserve expert-authored runtime names. Duplicate declared or generated object names are rejected before canonical builder validation.

## Units and expressions

Literal quantities are typed as `px`, `scalar`, `percent`, `degrees`, or `radians`. Expressions are data-only AST nodes; executable strings are not accepted.

Supported expression nodes are:

- `literal`
- `parameter`
- `add`
- `subtract`
- `multiply`
- `divide`

Addition and subtraction require compatible units. Degrees are normalized to radians. Transform position and dimensions require pixels; scale requires scalar values; rotation requires an angle. Non-finite values and division by zero are rejected with authored JSON paths.

## Components and instances

Components define typed parameter defaults and a visual node list. Instances may override only declared component parameters. Runtime names include the full instance expansion path, so repeated component contents remain unique and deterministic. Recursive component expansion is rejected with a `component_cycle` diagnostic.

## Raw canonical escapes

The escape hatches are intentionally explicit:

- visual nodes use `kind: "raw_scene_object"` with an `object` value;
- motion uses `raw_animations` entries;
- behavior uses `raw_state_machines` entries.

Each raw value must be a JSON object and still passes through `SceneSpec` deserialization and the canonical builder. Raw escapes therefore extend authoring coverage without creating a second encoder path.

## Diagnostics

`lower_authoring_json()` returns `AuthoringError` with one or more structured diagnostics:

```json
{
  "path": "$.visual.nodes[0].width.right",
  "code": "unit_mismatch",
  "message": "cannot combine Px with Scalar; operands must have compatible units"
}
```

Semantic diagnostics point to authored paths. JSON syntax and unknown-field errors use the root path plus Serde line and column information. Lowered `SceneSpec` and builder failures are reported at `$.lowered_scene`.

## Minimal example

```json
{
  "authoring_format_version": 0,
  "artboard": {
    "id": "stage",
    "width": { "value": 320, "unit": "px" },
    "height": { "value": 240, "unit": "px" }
  },
  "components": [
    {
      "id": "badge",
      "parameters": {
        "diameter": { "value": 64, "unit": "px" }
      },
      "visual": [
        {
          "kind": "ellipse",
          "id": "disc",
          "width": { "kind": "parameter", "name": "diameter" },
          "height": { "kind": "parameter", "name": "diameter" },
          "fill": "#246BFD"
        }
      ]
    }
  ],
  "visual": {
    "nodes": [
      {
        "kind": "instance",
        "id": "badge-one",
        "component": "badge",
        "transform": {
          "x": { "kind": "literal", "value": 160, "unit": "px" },
          "y": { "kind": "literal", "value": 120, "unit": "px" }
        }
      }
    ]
  },
  "motion": {},
  "behavior": {}
}
```
