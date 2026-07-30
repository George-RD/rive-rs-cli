# AuthoringSpec v0

`AuthoringSpec` is the strict AI-facing and programmatic authoring frontend for `rive-cli`. It lowers into the existing canonical `SceneSpec`; it does not write `.riv` bytes directly and does not replace the builder or encoder.

## Versioning

- `authoring_format_version` is required and must be `0`; the generated schema constrains the field to that single value.
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

The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, triangles, polygons, stars, literal text, groups, component instances, and raw `SceneSpec` objects. Shapes and text share one solid/linear/radial paint contract; stroke width is a positive pixel expression, and strokes may include a typed trim path. Polygon and star point counts must be at least three; star inner radius is a scalar ratio from zero to one. Font and image assets, bounded patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.

## Stable identity and runtime names

Every authored artboard, component, node, and raw fragment has an explicit stable `id`. The `/` character is reserved as the source-map expansion separator and is rejected in authored ids. Generated Rive runtime names are derived deterministically from the authored expansion path, including instance paths. The encoding is collision-resistant for distinct accepted ids and does not depend on hash-map iteration or process state.

Parameter names must contain only ASCII letters, digits, `_`, or `-`. This keeps parameter references and diagnostic paths unambiguous.

Lowering returns an `AuthoringSourceMap`. Each entry links:

- the authored id and JSON path;
- the component definition path when an instance was expanded;
- generated or declared runtime names;
- canonical `SceneSpec` JSON-pointer paths.

Raw escapes preserve expert-authored runtime names. Generated names and names declared by visual objects, animations, and state machines share a collision registry; duplicates are rejected before a result is returned.

## Units and expressions

Literal quantities are typed as `px`, `scalar`, `degrees`, or `radians`. Expressions are data-only AST nodes; executable strings are not accepted.

Supported expression nodes are:

- `literal`
- `parameter`
- `add`
- `subtract`
- `multiply`
- `divide`

Addition and subtraction require compatible units. Degrees are normalized to radians. Transform position and dimensions require pixels; scale requires scalar values; rotation requires an angle. Non-finite values, values that overflow or underflow the canonical `f32` scene representation, and division by zero are rejected with authored JSON paths. Canonicalized values are checked again after unit conversion, so conversion cannot silently turn a non-zero authored value into zero.

## Paints

A solid fill remains the compact string form used by existing v0 documents:

```json
"fill": "#246BFD"
```

Linear and radial gradients use the same typed expression model as geometry and components:

```json
"fill": {
  "kind": "linear_gradient",
  "start_x": { "kind": "literal", "value": 0, "unit": "px" },
  "start_y": { "kind": "literal", "value": 0, "unit": "px" },
  "end_x": { "kind": "parameter", "name": "gradient-width" },
  "end_y": { "kind": "literal", "value": 80, "unit": "px" },
  "stops": [
    {
      "color": "#F59E0B",
      "position": { "kind": "literal", "value": 0, "unit": "scalar" }
    },
    {
      "color": "#7C3AED",
      "position": { "kind": "literal", "value": 1, "unit": "scalar" }
    }
  ]
}
```

Gradient endpoints require pixel expressions. Stop positions require scalar expressions from zero to one, at least two stops are required, and evaluated positions must be in non-decreasing order. Equal positions are allowed for hard colour transitions. Every generated gradient and stop receives a deterministic runtime name and source-map path.

Strokes use the same paint contract under `paint`, plus a positive pixel `width`:

```json
"stroke": {
  "paint": {
    "kind": "radial_gradient",
    "start_x": { "kind": "literal", "value": 0, "unit": "px" },
    "start_y": { "kind": "literal", "value": 0, "unit": "px" },
    "end_x": { "kind": "literal", "value": 80, "unit": "px" },
    "end_y": { "kind": "literal", "value": 80, "unit": "px" },
    "stops": [
      {
        "color": "#0F172A",
        "position": { "kind": "literal", "value": 0, "unit": "scalar" }
      },
      {
        "color": "#F8FAFC",
        "position": { "kind": "literal", "value": 1, "unit": "scalar" }
      }
    ]
  },
  "width": { "kind": "literal", "value": 4, "unit": "px" }
}
```

The previous `color` field remains accepted as a parser compatibility alias for `paint`, but `paint` is the canonical v0 schema field.

A stroke may optionally add a typed trim path after its paint child:

```json
"trim": {
  "start": { "kind": "literal", "value": 0.1, "unit": "scalar" },
  "end": { "kind": "parameter", "name": "trim-end" },
  "offset": { "kind": "literal", "value": 0, "unit": "scalar" },
  "mode": "sequential"
}
```

`start` and `end` are normalized scalar expressions from zero to one. `offset` is an optional scalar expression that defaults to zero and is intentionally not clamped, allowing complete-cycle wrapping. `mode` is either `sequential` or `synchronized`. The generated trim object receives a deterministic runtime name and source-map path.

## Text

A `text` visual node lowers to a deterministic Rive text hierarchy: a transform anchor, text object, one text style with a fill, and one literal value run. Numeric styling uses the same typed expressions and component parameters as shapes:

```json
{
  "kind": "text",
  "id": "headline",
  "text": "Rive from data",
  "font_size": { "kind": "parameter", "name": "headline-size" },
  "fill": "#F8FAFC",
  "width": { "kind": "literal", "value": 280, "unit": "px" },
  "line_height": { "kind": "literal", "value": 1.2, "unit": "scalar" },
  "align": "center",
  "overflow": "visible"
}
```

Font size and optional width, height, letter spacing, and paragraph spacing are pixel expressions. Line height is a positive scalar expression. Optional `origin_x` and `origin_y` are normalized scalar expressions from zero to one. Alignment is `left`, `right`, or `center`; overflow is `visible`, `hidden`, `clipped`, `ellipsis`, `fit`, or `fit_font_size`.

Sizing is derived rather than exposed as a low-level numeric switch: no dimensions produce auto-width text, width alone produces auto-height wrapping, and width plus height produces a fixed box. A height without a width is rejected. Literal content is intentionally separate from future string parameters and view-model bindings. Font asset embedding is the next asset-focused slice.

## Components and instances

Components define typed parameter defaults and a visual node list. A component body can reference only parameters declared by that component. Document-level parameters remain available to the root visual graph and instance transforms but do not leak into reusable component definitions. Instances may override only declared component parameters. Runtime names include the full instance expansion path, so repeated component contents remain unique and deterministic. Recursive component expansion is rejected with a `component_cycle` diagnostic.

Expansion is preflighted iteratively before recursive lowering. An active component chain is limited to 64 definitions, and each component-validation or root-document traversal may generate at most 10,000 component nodes. The limits return `component_expansion_depth_limit` or `component_expansion_node_limit` diagnostics at the authored instance path instead of risking stack or memory exhaustion.

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
          "fill": "#246BFD",
          "stroke": {
            "paint": "#0F172A",
            "width": { "kind": "literal", "value": 3, "unit": "px" }
          }
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
