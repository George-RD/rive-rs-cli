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

A v0 document has four explicit graphs plus a deterministic file-scope asset registry:

- `font_assets`: semantic font IDs mapped to file sources.
- `image_assets`: semantic image IDs mapped to file sources.
- `components`: reusable authored visual definitions with typed parameter defaults.
- `visual`: the root visual graph.
- `motion`: raw canonical animation escapes until the dedicated motion compiler lands.
- `behavior`: raw canonical state-machine escapes until the dedicated behavior compiler lands.

The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, triangles, polygons, stars, literal text, static images, groups, component instances, deterministic grid, radial, mirror, distribute, and along-path patterns, semantic font and image assets, and raw `SceneSpec` objects. Shapes and text share one solid/linear/radial paint contract; stroke width is a positive pixel expression, and strokes may include a typed trim path. Polygon and star point counts must be at least three; star inner radius is a scalar ratio from zero to one. Constraints, motion helpers, and statechart authoring remain separate roadmap items.

## Stable identity and runtime names

Every authored artboard, component, node, and raw fragment has an explicit stable `id`. The `/` character is reserved as the source-map expansion separator and is rejected in authored ids. Generated Rive runtime names are derived deterministically from the authored expansion path, including instance paths. The encoding is collision-resistant for distinct accepted ids and does not depend on hash-map iteration or process state.

Parameter names and font or image asset IDs must contain only ASCII letters, digits, `_`, or `-`. This keeps semantic references and diagnostic paths unambiguous.

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

## Font assets

A document declares fonts by semantic ID rather than exposing a Rive runtime index:

```json
"font_assets": {
  "inter": "assets/fonts/Inter-Bold-Subset.ttf"
}
```

Font assets lower in sorted ID order before visual nodes. Each asset receives a deterministic runtime name and its own source-map entry. Text may reference the semantic ID through `font`; unknown IDs fail at the authored text path. Lowering preserves the source in returned `SceneSpec` while keeping compiler validation independent of the filesystem. The canonical builder embeds the file bytes when its caller supplies an explicit base directory.

## Image assets

A document declares images by semantic ID and references them from transformable static image nodes:

```json
"image_assets": {
  "aurora": "assets/textures/aurora.png"
}
```

```json
{
  "kind": "image",
  "id": "backdrop",
  "asset": "aurora",
  "transform": {
    "x": { "kind": "literal", "value": 160, "unit": "px" },
    "y": { "kind": "literal", "value": 120, "unit": "px" }
  }
}
```

Font assets lower first, followed by image assets, with each registry sorted by authored ID. Image nodes reference the generated asset name rather than a runtime ordinal, and unknown IDs fail at the authored `asset` path. The returned `SceneSpec` keeps the source; the canonical builder resolves the global image ordinal and embeds bytes when given an explicit base directory.

## Text

A `text` visual node lowers to a deterministic Rive text hierarchy: a transform anchor, text object, one text style with a fill, and one literal value run. Numeric styling uses the same typed expressions and component parameters as shapes:

```json
{
  "kind": "text",
  "id": "headline",
  "text": "Rive from data",
  "font": "inter",
  "font_size": { "kind": "parameter", "name": "headline-size" },
  "fill": "#F8FAFC",
  "width": { "kind": "literal", "value": 280, "unit": "px" },
  "line_height": { "kind": "literal", "value": 1.2, "unit": "scalar" },
  "align": "center",
  "overflow": "visible"
}
```

Font size and optional width, height, letter spacing, and paragraph spacing are pixel expressions. Line height is a positive scalar expression. Optional `origin_x` and `origin_y` are normalized scalar expressions from zero to one. Alignment is `left`, `right`, or `center`; overflow is `visible`, `hidden`, `clipped`, `ellipsis`, `fit`, or `fit_font_size`.

Sizing is derived rather than exposed as a low-level numeric switch: no dimensions produce auto-width text, width alone produces auto-height wrapping, and width plus height produces a fixed box. A height without a width is rejected. Literal content is intentionally separate from future string parameters and view-model bindings. The optional `font` field must reference `font_assets`; omitting it preserves the previous structure-only text behavior.

## Components and instances

Components define typed parameter defaults and a visual node list. A component body can reference only parameters declared by that component. Document-level parameters remain available to the root visual graph and instance transforms but do not leak into reusable component definitions. Instances may override only declared component parameters. Runtime names include the full instance expansion path, so repeated component contents remain unique and deterministic. Recursive component expansion is rejected with a `component_cycle` diagnostic.

Expansion is preflighted iteratively before recursive lowering. An active component chain is limited to 64 definitions, and each component-validation or root-document traversal may generate at most 10,000 component nodes. The limits return `component_expansion_depth_limit` or `component_expansion_node_limit` diagnostics at the authored instance path instead of risking stack or memory exhaustion.

## Mirror patterns

A `mirror` node emits exactly two deterministic cells: `original` and `mirrored`. A vertical axis reflects the second cell through `scale_x: -1`; a horizontal axis reflects it through `scale_y: -1`. The pattern's transform wraps both cells, while the authored item keeps its own transform inside each cell.

```json
{
  "kind": "mirror",
  "id": "wings",
  "axis": "vertical",
  "item": {
    "kind": "triangle",
    "id": "wing",
    "width": { "kind": "literal", "value": 48, "unit": "px" },
    "height": { "kind": "literal", "value": 72, "unit": "px" },
    "fill": "#2563EB",
    "transform": {
      "x": { "kind": "literal", "value": 28, "unit": "px" }
    }
  }
}
```

Mirror items use the same component expansion, generated-node budget, runtime-name registry, source-map rewriting, and canonical builder path as grid and radial patterns. Nested repeat-safe authored nodes are supported. Raw `SceneSpec` objects are rejected when mirrored because embedded names and references cannot be safely namespaced across repeated copies.

## Distribute patterns

A `distribute` node places between two and 100 copies at equal intervals along a straight authored segment. Both endpoints are included. The four endpoint expressions use pixel units and may reference component parameters.

```json
{
  "kind": "distribute",
  "id": "steps",
  "copies": 4,
  "start_x": { "kind": "literal", "value": 0, "unit": "px" },
  "start_y": { "kind": "literal", "value": 0, "unit": "px" },
  "end_x": { "kind": "literal", "value": 120, "unit": "px" },
  "end_y": { "kind": "literal", "value": 60, "unit": "px" },
  "item": {
    "kind": "ellipse",
    "id": "dot",
    "width": { "kind": "literal", "value": 16, "unit": "px" },
    "height": { "kind": "literal", "value": 16, "unit": "px" },
    "fill": "#2563EB"
  }
}
```

This example emits cells at `(0, 0)`, `(40, 20)`, `(80, 40)`, and `(120, 60)`. The pattern transform wraps the complete distribution, while the item keeps its own transform inside every cell. Distribution uses the same component expansion, runtime-name registry, source maps, raw-scene repetition safety, generated-node budget, and canonical builder path as the other bounded patterns.

## Along-path patterns

An `along_path` node places between two and 100 copies at equal distances along a polyline with between two and 100 authored points. Both path endpoints are included. Point coordinates use pixel expressions and may reference component parameters.

```json
{
  "kind": "along_path",
  "id": "route",
  "copies": 5,
  "points": [
    {
      "x": { "kind": "literal", "value": 0, "unit": "px" },
      "y": { "kind": "literal", "value": 0, "unit": "px" }
    },
    {
      "x": { "kind": "literal", "value": 80, "unit": "px" },
      "y": { "kind": "literal", "value": 0, "unit": "px" }
    },
    {
      "x": { "kind": "literal", "value": 80, "unit": "px" },
      "y": { "kind": "literal", "value": 60, "unit": "px" }
    }
  ],
  "rotate_items": true,
  "item": {
    "kind": "triangle",
    "id": "marker",
    "width": { "kind": "literal", "value": 18, "unit": "px" },
    "height": { "kind": "literal", "value": 12, "unit": "px" },
    "fill": "#2563EB"
  }
}
```

Spacing is measured across the complete polyline rather than independently per segment. When `rotate_items` is true, each cell follows the active segment tangent; an item exactly on an interior vertex uses the outgoing segment. The final item uses the last segment tangent. Consecutive duplicate points are rejected because they do not define a tangent. v0 intentionally models polylines only and does not infer or fit curves.

Along-path patterns use the same component expansion, runtime-name registry, source maps, raw-scene repetition safety, generated-node budget, and canonical builder path as the other bounded patterns.

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
