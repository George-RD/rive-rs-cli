# AuthoringSpec v0

`AuthoringSpec` is the strict AI-facing and programmatic authoring frontend for `rive-cli`. It lowers into the existing canonical `SceneSpec`; it does not write `.riv` bytes directly and does not replace the builder or encoder.

## Versioning

- `authoring_format_version` is required and must be `0`; the generated schema constrains the field to that single value.
- Unknown fields are rejected at every typed authoring layer.
- A breaking field, semantic, unit, naming, or lowering change requires a new authoring format version.
- Additive compiler capability may be introduced within v0 only when existing v0 documents lower to the same canonical `SceneSpec` and source map.
- `scene_format_version` remains independently versioned. v0 currently lowers to `SceneSpec` version `1`.

`stacking`, motion `continuity` and `waypoint`, state `blend` and `direct_blend`, and statechart `regions` are optional fields whose defaults (`runtime`, `per_keyframe`, `auto`, and absent `blend`, `direct_blend`, and `regions`) leave the canonical `SceneSpec` and source map unchanged. The `number` and `trigger` input kinds, the comparison and trigger transition conditions, and the `number_change` and `trigger_change` listener actions are new variants of the input, condition, and listener-action unions. A document that uses none of them lowers as it did before, so `authoring_format_version` stays `0`; `tests/showcase_artifact.rs` recompiles each committed showcase and compares the bytes against the checked-in `.riv`.

`exit_time_ms` is an optional outgoing-animation gate; its contract is described under Transition exit time. `duration_ms` is an optional transition field. Omitting it preserves existing canonical scenes, source maps, and compiled artifacts; explicit zero remains instantaneous. This additive capability keeps `authoring_format_version` at `0`.

The generated JSON Schema is available through `authoring::authoring_schema()` and uses this stable identifier:

```text
https://github.com/George-RD/rive-rs-cli/docs/authoring.schema.v0.json
```

## Document model

A v0 document has four explicit graphs plus a deterministic file-scope asset registry:

- `font_assets`: semantic font IDs mapped to file sources.
- `image_assets`: semantic image IDs mapped to file sources.
- `components`: reusable authored visual definitions with typed parameter defaults and an optional `stacking` order.
- `visual`: the root visual graph, with an optional `stacking` order.
- `motion`: typed poses and tracks, per-track `continuity` and per-keyframe `waypoint`, plus `raw_animations` for canonical expert escapes.
- `behavior`: typed boolean and numeric view models and bindings, `bool`, `number`, and `trigger` state-machine inputs, named Rive events, typed listeners, named states that play one motion track, use a one-dimensional blend, or mix independent motion weights, parallel regions, and binding, boolean, comparison, and trigger transitions plus `raw_state_machines` for canonical expert escapes.

The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, triangles, polygons, stars, literal text, static images, groups, component instances, deterministic grid, radial, mirror, distribute, and along-path patterns, group-scoped transform-anchor constraints, semantic font and image assets, and raw `SceneSpec` objects. Shapes and text share one solid/linear/radial paint contract; stroke width is a positive pixel expression, and strokes may include a typed trim path. Polygon and star point counts must be at least three; star inner radius is a scalar ratio from zero to one. Motion and behavior remain deliberately incremental: v0 exposes only compiler-proven typed subsets and retains raw canonical escapes for unsupported features.

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

## Group constraints

A `group` may declare an optional `constraints` array. Constraints reference direct children by stable authored `id` and resolve their typed `x` and `y` transform anchors before ordinary node lowering:

```json
"constraints": [
  {
    "kind": "align",
    "id": "align-label",
    "subject": "label",
    "target": "icon",
    "axis": "y"
  },
  {
    "kind": "center",
    "id": "center-label",
    "subject": "label",
    "start": "left-edge",
    "end": "right-edge",
    "axis": "x"
  },
  {
    "kind": "offset",
    "id": "place-badge",
    "subject": "badge",
    "target": "label",
    "x": { "kind": "literal", "value": 16, "unit": "px" },
    "y": { "kind": "literal", "value": -8, "unit": "px" }
  },
  {
    "kind": "spacing",
    "id": "space-actions",
    "items": ["action-a", "action-b", "action-c"],
    "axis": "x",
    "gap": { "kind": "parameter", "name": "action-gap" }
  }
]
```

`align` copies one sibling anchor on one axis. `center` places an anchor at the midpoint between two sibling anchors. `offset` derives both axes from one sibling plus pixel expressions. `spacing` preserves the first item's authored anchor and places each later item the evaluated pixel gap after the previous item on the selected axis; the perpendicular authored coordinate is unchanged. Constraint expressions use the normal component parameter scope, so instance overrides remain deterministic.

Constraints are intentionally group-local and anchor-based. They do not inspect rendered bounds, infer edges, or act as a general CAD solver. Raw `SceneSpec` nodes cannot participate because they have no typed authoring transform. A group may declare at most 100 constraints. Each constraint `id` must be non-empty after trimming, must not contain `/`, and must be unique within its group. Dependency chains are bounded to 100 assignments. Unknown siblings, oversized constraint lists, invalid or duplicate constraint IDs, duplicate spacing entries, conflicting assignments, invalid units, excessive dependency depth, and dependency cycles return authored-path diagnostics such as `unknown_constraint_node`, `invalid_constraint_count`, `invalid_constraint_id`, `duplicate_constraint_id`, `constraint_conflict`, `constraint_resolution_depth_limit`, and `constraint_cycle`. Cycle messages include the stable authored anchor chain.

## Stacking order

Rive paints the first child of a list on top of the children that follow it. The optional `stacking` field states which reading of the authored array is intended. `runtime` is the default and leaves the order untouched; `back_to_front` reverses the emitted children, so the last authored sibling paints on top.

`stacking` is accepted on the `visual` section, on each entry of `components`, and on a `group` node. Each list is reversed on its own: a `back_to_front` group inside a `runtime` root reverses only that group's children.

```json
"visual": {
  "stacking": "back_to_front",
  "nodes": [
    {
      "kind": "group",
      "id": "card",
      "stacking": "back_to_front",
      "transform": {
        "x": { "kind": "literal", "value": 64, "unit": "px" },
        "y": { "kind": "literal", "value": 64, "unit": "px" }
      },
      "children": [
        {
          "kind": "rectangle",
          "id": "surface",
          "width": { "kind": "literal", "value": 128, "unit": "px" },
          "height": { "kind": "literal", "value": 128, "unit": "px" },
          "fill": "#C2410C"
        },
        {
          "kind": "rectangle",
          "id": "cue",
          "width": { "kind": "literal", "value": 32, "unit": "px" },
          "height": { "kind": "literal", "value": 32, "unit": "px" },
          "fill": "#22C55E"
        }
      ]
    }
  ]
}
```

Only the emitted child order and the source-map `scene_paths` change. Authored paths, component definition paths, diagnostic paths, and source-map entry order stay in authored order. The graph above is the `visual` section of `examples/authoring/stacking-card.v0.json`: `cue` keeps authored path `$.visual.nodes[0].children[1]` and receives scene path `/artboard/children/0/children/0`, so the 32px cue covers the centre of the 128px surface. A bad unit on that same child is still reported at `$.visual.nodes[0].children[1].width`. `tests/authoring_stacking_runtime.rs` renders that fixture at 128x128 and reads the cue colour `#22C55E` at the artboard centre; with both `stacking` fields set to `runtime` the surface colour `#C2410C` is there instead.

Raw `SceneSpec` input through `generate` is unaffected and keeps native runtime ordering.

## Motion continuity and waypoints

Rive attaches an interpolator to the keyframe that starts a segment, so an easing that settles at its end brings the target to a stop at every keyframe it governs, including keyframes the author intended as pass-through points. `continuity` sets the track's reading of its keyframes; `waypoint` overrides that reading for one keyframe.

`continuity` is `per_keyframe`, the default, or `through`. Under `through`, each segment that arrives at an interior waypoint is emitted as `linear` with no interpolator, so the target keeps its speed across that waypoint. The segment arriving at the last keyframe keeps its authored easing, so the motion still settles at the destination. A `hold` segment is never rewritten, and an interpolator that governs no remaining segment after the rewrite is not emitted.

`waypoint` is `auto`, the default, `transit`, or `settle`. `transit` forces the rewrite for one keyframe inside a `per_keyframe` track; `settle` suppresses it for one keyframe inside a `through` track. Both are valid only on a keyframe that is neither first nor last once the track's keyframes are sorted by frame; otherwise lowering fails with `waypoint_not_interior` at `$.motion.tracks[i].keyframes[j].waypoint`.

```json
{
  "id": "transit",
  "fps": 60,
  "duration_frames": { "kind": "literal", "value": 60, "unit": "scalar" },
  "continuity": "through",
  "keyframes": [
    { "frame": { "kind": "literal", "value": 0, "unit": "scalar" }, "pose": "start", "easing": "settle" },
    { "frame": { "kind": "literal", "value": 30, "unit": "scalar" }, "pose": "mid", "easing": "settle" },
    { "frame": { "kind": "literal", "value": 60, "unit": "scalar" }, "pose": "arrive", "easing": "settle" }
  ]
}
```

The track above is from `examples/authoring/waypoint-transit.v0.json`, which moves a 24px token to x 40, 160, and 280 at frames 0, 30, and 60 with the same ease-out cubic `(0.23, 1, 0.32, 1)` on every keyframe. `tests/authoring_motion_continuity_runtime.rs` renders it through the official runtime and measures the token's horizontal centre at frames 26 and 30: under `through` the token travels at least 12px into the waypoint; with `continuity` deleted it travels at most 2px, because it is already stopping into the `mid` pose.

Lowering also returns non-fatal warnings in `LoweredAuthoring.warnings`, a `Vec<AuthoringDiagnostic>` with the same `path`, `code`, and `message` fields as a failure diagnostic. `waypoint_stop_start` is reported at `$.motion.tracks[i].keyframes[j]` when the track leaves an interior keyframe as a stopping point. All of these must hold: the keyframe's `waypoint` is `auto`, the track continuity is `per_keyframe`, the keyframe is entered and left on the same easing, that easing's end tangent is flat (`y2` is 1 and `x2` is below 1), and at least one animated property keeps moving in the same direction through the keyframe. Deleting `"continuity": "through"` from the track above produces:

```text
warning: $.motion.tracks[0].keyframes[1] [waypoint_stop_start]: waypoint 'mid' at frame 30 enters and leaves on easing 'settle', which stops the motion and starts it again; mark this keyframe as a transit waypoint or set the track continuity to 'through' to move through it
```

## Typed behavior interaction

Typed behavior stays on the same compiler-owned `SceneSpec` draft as visual and motion lowering and keeps authored interaction free of runtime indices. A behavior model may declare boolean or numeric properties, and bindings select a model and property by authored ID. A statechart declares named states, an authored initial state, and named transitions whose `from` and `to` fields reference state IDs.

Statechart `inputs` are typed by `kind`. A `bool` input carries a boolean `value`, a `number` input carries a scalar expression, and a `trigger` input carries no value:

```json
"inputs": [
  { "kind": "number", "id": "load", "value": { "kind": "literal", "value": 0, "unit": "scalar" } },
  { "kind": "bool", "id": "armed", "value": false },
  { "kind": "trigger", "id": "reset" }
]
```

Statecharts also declare named Rive `events` and typed `listeners`. A listener targets either an authored visual ID for pointer interaction or an authored event ID when `listener_type` is `event`; the compiler resolves that semantic target to the generated runtime object name. The supported listener types are `enter`, `exit`, `down`, `up`, `move`, `event`, and `click`. The typed actions are `bool_change`, whose `value` defaults to `true`, `number_change`, which sets a number input from a scalar expression, and `trigger_change`, which fires a trigger. An action kind that does not match the declared kind of the input it names fails with `invalid_listener_input` at `$.behavior.statecharts[i].listeners[j].actions[k].input`.

Transition conditions are an untagged union of five forms:

- `{"binding": ..., "equals": <bool>}` checks a boolean view-model binding.
- `{"binding": ..., "compare": ..., "value": <scalar expression>}` checks a numeric view-model binding with the same six comparisons as a number input.
- `{"input": ..., "equals": <bool>}` checks a boolean input.
- `{"input": ..., "compare": ..., "value": <scalar expression>}` checks a number input, where `compare` is `equal`, `not_equal`, `greater`, `greater_or_equal`, `less`, or `less_or_equal`.
- `{"trigger": ...}` fires on a trigger input.

The condition form and the input kind must agree. `unknown_behavior_input` fires when the named input is not declared and `invalid_condition_input` when the kinds differ, both at `$.behavior.statecharts[i].transitions[j].when.input`, or `.when.trigger` for the trigger form.

The compiler lowers model properties to Rive view models, explicit inputs to named state-machine inputs, events to named artboard event objects, listeners to canonical state-machine listeners, and conditions to runtime input names. Source-map entries preserve the authored model, property, binding, input, event, listener, statechart, states, and transitions. Unknown input, event, listener-target, and listener-action references fail at their authored JSON paths.

The canonical builder validates the merged graph. Behavior validation drops asset `source` fields from its copy of the lowered scene the same way the visual path does, so a document may declare `font_assets` or `image_assets` and a statechart together. Runtime contracts prove both interaction paths: changing a bound view-model boolean through the official web runtime changes state, while the compiled typed interaction fixture is also driven through the public `rive-cli render` interface with both `--input` and `--pointer`, and both must converge on the same visible state.

## Combined transition conditions

Keep a single condition in `when`, or require several conditions together with
`when.all`. For example, this transition requires both an armed input and a load
of at least 60:

```json
{
  "id": "engage",
  "from": "resting",
  "to": "engaged",
  "when": {
    "all": [
      { "input": "armed", "equals": true },
      {
        "input": "load",
        "compare": "greater_or_equal",
        "value": { "kind": "literal", "value": 60, "unit": "scalar" }
      }
    ]
  }
}
```

An `all` group contains 1–1000 of the existing five condition forms. Conditions
are emitted in authored order and must all hold in the same runtime update.
Boolean and numeric model bindings, explicit inputs, and triggers can be mixed.
A trigger is momentary: a trigger fired while another condition is false is not
queued until that condition becomes true. Fire it again when the other conditions
are satisfied.

The group is flat. Nested groups, `any`, negation, and an `all` object containing
additional condition fields are rejected. An empty or oversized group reports
`invalid_behavior_collection_count` at `.when.all` through both the JSON and typed
Rust entry points. Leaf errors retain their existing codes at indexed paths such
as `$.behavior.statecharts[0].transitions[0].when.all[1].input`; numeric expression
errors can point further into `.value`. Regions use the corresponding
`.regions[i].transitions[j].when.all[k]` paths.

Existing single-condition documents keep their SceneSpec, source-map and binary
output. Wrapping one condition in `all` gives the same output. Groups do not add
runtime inputs beyond the bindings already required by their leaves; bindings are
shared with other transitions and blend consumers in the same chart. Model leaves
still read the bound model instance, not a mirrored machine input.

`duration_ms` and `exit_time_ms` work independently of the guard. The exit gate
cannot bypass a false member of `all`. The public CLI and official-runtime
contracts retain mixed model/input/trigger truth cases for root and region
transitions, blocked-trigger/re-fire evidence, and timed frames in the
`typed-behavior-runtime-evidence` artifact under `transition-guards`.

## Numeric view-model bindings

A model property accepts `{"kind": "number", "id": "load", "value": <scalar expression>}`.
Its initial value and a bound comparison threshold use document parameters and
scalar units. Unknown parameters, incompatible units, non-finite numbers, and
values that overflow or underflow the runtime's f32 representation fail at the
authored expression path, including unused model properties.

```json
"when": {
  "binding": "gate-load",
  "compare": "greater_or_equal",
  "value": { "kind": "parameter", "name": "threshold" }
}
```

`examples/authoring/number-binding.v0.json` provides the complete model, binding,
parameters, motion and reverse transition. The binding must name a numeric property
for this condition, or a boolean property for `equals`; a mismatch reports
`invalid_condition_binding` at `.when.binding`. An unknown binding reports
`unknown_behavior_binding` there. Root transitions and parallel regions share this
validation and lowering path.

**A bound condition reads the model instance, not the synthesized machine input.**
The property's authored value initializes that input, following the existing boolean
convention. It does not serialize a default view-model instance or continuously
synchronize the input. The host must create, initialize, and bind its model instance.
Resolve model and property runtime names from the compile report's source map:

```javascript
const instance = runtime.viewModelByName(modelRuntimeName).instance();
const load = instance.number(propertyRuntimeName);
load.value = 25;
runtime.bindViewModelInstance(instance);
load.value = 60;
```

Changing only a synthesized input, including through `render --input`, does not
change the model condition. Direct blends may also select numeric bindings as
described below, as may one-dimensional blends. Listener actions still target
explicitly declared machine inputs. Converters, string/enum/trigger model properties,
and listener writes to model properties remain outside the typed subset.

The official-runtime test compiles this example through the public CLI, proves that
input-only mutation does not transition, then mutates the model through 59, 60, 90
and 59 against a threshold of 60. The typed-behavior CI artifact retains authored
JSON, the compile report/source map, binary, PNGs, measured positions and hashes.
Both schema versions remain unchanged; published schemas include the new forms.

## Transition duration

A transition may set `duration_ms` to blend from its source state to its destination over a fixed number of milliseconds:

```json
{
  "id": "engage",
  "from": "resting",
  "to": "engaged",
  "when": { "input": "pressed", "equals": true },
  "duration_ms": { "kind": "literal", "value": 250, "unit": "scalar" }
}
```

The expression uses scalar units and the document's parameter scope. For example, `{"kind": "parameter", "name": "crossfade-ms"}` reads a scalar parameter declared under `parameters`; arithmetic expressions are also supported. The evaluated value must be a finite whole number from 0 through 4294967295, matching the runtime's unsigned 32-bit millisecond field. Negative, fractional, and oversized results report `invalid_transition_duration` at the authored `.duration_ms` path. Expression errors retain their own codes and paths, including `unit_mismatch`, `unknown_parameter`, `division_by_zero`, and `non_finite` for programmatic non-finite values.

The same contract applies to transitions inside parallel regions. The compiler emits canonical `duration` only when the field is supplied, keeps the transition's source-map identity unchanged, and leaves the generated entry transition instantaneous. Duration controls the blend after its condition is satisfied; it is not an exit-time gate or a delay before starting the transition. Percentage timing and transition easing remain outside this typed field.

`tests/playwright/authoring-behavior-runtime.js` compiles a 1000ms transition through the public CLI, schedules its input at frame 1, and checks distinct intermediate poses at frames 16, 31, and 46 at 60fps before the destination at frame 91. The same render path supplies control and instantaneous comparisons. Source, compiled output, frame PNGs, and hashes are retained in the typed-behavior runtime CI artifact.

## Transition exit time

`exit_time_ms` prevents a transition from leaving its named motion state before the outgoing animation reaches the specified point. The `when` condition must also be satisfied. It is animation time, not a delay started by an input:

```json
{
  "id": "engage",
  "from": "resting",
  "to": "engaged",
  "when": { "input": "pressed", "equals": true },
  "exit_time_ms": { "kind": "literal", "value": 1000, "unit": "scalar" },
  "duration_ms": { "kind": "literal", "value": 250, "unit": "scalar" }
}
```

With a one-shot outgoing track, an input that becomes true early waits for 1000ms of animation time; an input that becomes true after that point can transition immediately. Once allowed, the optional `duration_ms` controls the blend independently. For looping animations, the official runtime repeats gates within the animation's duration on subsequent cycles; gates beyond one cycle use accumulated animation time. This field does not change that runtime behavior.

The scalar expression uses document parameters and must resolve to a finite integer from 0 through 4294967295 milliseconds. Negative, fractional, and oversized results report `invalid_transition_exit_time` at the authored `.exit_time_ms` path; expression errors retain their existing codes and paths. Parallel-region transitions use the same validation and lowering. The source must name a motion track: a blend source reports `unsupported_transition_exit_source` at `.exit_time_ms`, because an ordinary runtime transition cannot enforce that gate on a blend. A blend destination is allowed.

Omitted or null gates preserve the previous canonical scene, source map, and compiled bytes. Explicit zero writes canonical `exit_time: 0` and enables the runtime gate flag, so its binary representation differs from omission. Entry transitions remain ungated. Canonical `TransitionSpec.exit_time` accepts an unsigned 32-bit millisecond value and rejects entry, exit, any, and blend source states. Percentage timing, pause-on-exit, early exit, and blend-source animation selection remain outside this field. Both format versions are unchanged.

`tests/authoring_transition_exit_time_contract.rs` verifies the public compiler and encoded properties. `tests/playwright/authoring-behavior-runtime.js` retains authored source, compiled binaries, representative PNGs, and hashes under `typed-interaction/exit-time` in the typed-behavior runtime CI artifact. Comparisons cover early input, late input, unmet conditions, looping, and exit time combined with duration.

## Blend states

A behavior state declares exactly one of `motion`, `blend`, and `direct_blend`. `motion` names an authored motion track. `blend` maps either a number input or numeric model binding onto at least two motion tracks, each with the input value at which that track is fully applied:

```json
{
  "id": "reading",
  "blend": {
    "input": "load",
    "stops": [
      { "motion": "calm-track", "value": { "kind": "literal", "value": 0, "unit": "scalar" } },
      { "motion": "surge-track", "value": { "kind": "literal", "value": 100, "unit": "scalar" } }
    ]
  }
}
```

The state lowers to a `blend_state_1d` whose `input` is the lowered input name and whose children are `blend_animation_1d` entries naming the lowered animations. A state with no motion source fails with `missing_state_motion` and a state with multiple sources fails with `ambiguous_state_motion`, both at the state path. `unknown_behavior_input` and `invalid_blend_input` fire at `$.behavior.statecharts[i].states[j].blend.input` when the named input is absent or is not a number input, and `unknown_behavior_motion` at `$.behavior.statecharts[i].states[j].blend.stops[k].motion` for a track that is not defined.

A blend needs between 2 and 1000 stops; a count outside that range fails with `invalid_blend_stops` at `$.behavior.statecharts[i].states[j].blend.stops`. Stop values must strictly increase. Rive's `BlendState1DInstance` binary-searches its children as ascending thresholds, so an out-of-order or repeated value would leave a stop unreachable; it fails with `invalid_blend_stop_order` at `$.behavior.statecharts[i].states[j].blend.stops[k].value`. Each value is narrowed to a 32-bit float before the comparison, because that is the width the emitted `blend_animation_1d` carries and the encoder writes, so two thresholds that differ only below `f32` precision are rejected instead of arriving at the runtime as duplicates. Both checks run on the typed Rust path. The published JSON schema also records the stop-count bounds; ordering is checked by the compiler, not JSON Schema.

Rive mixes the two neighbouring stop animations in sequence rather than as a weighted average, so an input between two stops does not render at the arithmetic midpoint. `tests/authoring_behavior_blend_runtime.rs` drives `examples/authoring/blend-meter.v0.json` at `load` 0, 50, and 100: the stops hold the needle within 3px of x 40 and x 200, and 50 renders it near x 146 rather than at x 120. The mapping stays monotonic; the test asserts that ordering rather than the exact middle position.

### Model-bound one-dimensional blends

Replace `input` with `binding` to drive the same ordered stops from a numeric
model property:

```json
{
  "id": "reading",
  "blend": {
    "binding": "model-load",
    "stops": [
      { "motion": "calm-track", "value": { "kind": "literal", "value": 0, "unit": "scalar" } },
      { "motion": "surge-track", "value": { "kind": "literal", "value": 100, "unit": "scalar" } }
    ]
  }
}
```

Exactly one source is required. Neither, both, null sources and runtime indices are
rejected by the strict source union. `unknown_behavior_binding` and
`invalid_blend_binding` report an absent binding or non-number model property at
`.blend.binding`; invalid model/property declarations retain their own paths.
The same 2..=1000 stop, scalar expression and emitted-float ordering checks apply.
Stop values are thresholds, not percentage weights; 0..100 is only this example's
range. Values outside the outer stops select the nearest endpoint.

A binding used only by a blend is still emitted. Root states, parallel regions,
direct blends and transition conditions share chart-scoped binding resolution and
source-map entries. The canonical scene remains a named `blend_state_1d` input
reference with `view_model_binding`. The builder emits a native
`BlendState1DViewModel` with its own preceding bindable number/context, rather than
an input-driven state. No second compiler pass or host animation simulation is used.

The host creates, initializes and binds the model instance as shown in the numeric
binding example. Changing its number property drives the blend; changing only the
synthesized input, including through `render --input`, does not. The authored model
value initializes the synthetic input, not a serialized default model instance.

`examples/authoring/model-blend-1d-panel.v0.json` provides two independent three-stop
blends, one in a parallel region. Run
`node tests/playwright/authoring-direct-blend-runtime.js --model-bound --one-dimensional`
for public-CLI/bundled-runtime verification. Evidence is retained under
`target/playwright-behavior/model-blend-1d`: exact stop positions, intermediate
values, clamping, reversal, input-only non-effects and timed exit/resume after a
model change while inactive. The normal typed-behavior CI artifact retains this
mode alongside the direct blend proofs. Existing input-only documents preserve
their JSON and bytes; `authoring_format_version` remains 0.

## Direct blend states

`direct_blend` gives each named motion its own number input, numeric model binding or fixed scalar weight instead of selecting neighbouring stops from a shared input:

```json
{
  "id": "blending",
  "direct_blend": {
    "motions": [
      { "motion": "rest-track", "input": "foundation" },
      { "motion": "left-track", "input": "left-weight" },
      { "motion": "right-track", "input": "right-weight" }
    ]
  }
}
```

Each input is a percentage weight. The official runtime clamps values below 0 to no contribution and above 100 to full contribution. These are not relative weights normalized to a total. Children are applied in authored order, so later motions can modify properties already changed by earlier motions. The compiler preserves that order.

`examples/authoring/direct-blend-panel.v0.json` uses a first rest motion with its `foundation` number input initialized to 100. That reestablishes both panels' baseline before two independent inputs contribute motion. With that baseline, a weight of 50 places its panel halfway from x 40 to x 200; reversing either weight returns that panel towards rest without changing the other panel. This example does not claim that every arbitrary combination of overlapping tracks has the same midpoint semantics.

The collection contains 1 through 1000 children. `invalid_direct_blend_motions` reports an empty or oversized collection at `.direct_blend.motions`. Each `motion` must name a typed motion track; each `input` must name a number input declared in the same chart. Unknown references report `unknown_behavior_motion` or `unknown_behavior_input` at the child's `.motion` or `.input`; non-number inputs report `invalid_blend_input` at `.input`. A binding ID is not an `input` alias: use the exclusive `binding` form below. Raw-animation IDs are not typed motion references. Unknown fields, including runtime indices, are rejected.

The same form works in parallel regions. The compiler resolves animation indices from its existing lowered scene and input indices from the emitted chart inputs, including offsets from binding-generated inputs. It lowers to existing `blend_state_direct` and `blend_animation_direct` objects without a new encoder path. Source-map identities remain attached to named states and regions. Transitions may enter or leave direct states and use `duration_ms`; `exit_time_ms` on a direct-blend source is rejected with `unsupported_transition_exit_source`, just as for a one-dimensional blend.

The runtime contract `tests/playwright/authoring-direct-blend-runtime.js` compiles the source through the public CLI and retains the source, binary, compile report, measured positions, PNGs and hashes. Cases cover independent 0/50/100 inputs, runtime clamping, reversal, leaving the direct state, resuming it and returning both inputs to zero. Fixed expression weights are described below; additive states remain outside the typed subset. Omission or null leaves existing canonical output and bytes unchanged; the authoring format stays at version 0.

### Model-bound direct weights

A child may use `binding` instead of `input`:

```json
{
  "id": "blending",
  "direct_blend": {
    "motions": [
      { "motion": "rest-track", "input": "foundation" },
      { "motion": "left-track", "binding": "left-model" },
      { "motion": "right-track", "binding": "right-model" }
    ]
  }
}
```

Each binding names a numeric model property through `behavior.bindings`. A child selects exactly one of `input`, `binding` and `weight`. Multiple source
fields, missing sources, null sources and unknown fields are rejected. An unknown
binding reports `unknown_behavior_binding` at the child's `.binding`; a boolean
property reports `invalid_blend_binding` there. Invalid model/property references
keep their declaration paths. The same rules apply in parallel regions.

The compiler emits a binding even when only a blend uses it. Transition, root-state
and region uses share one synthesized input per binding per chart. Actual emitted
indices and source-map paths remain chart-local. The canonical builder emits the
native numeric bindable property and data-binding context before its direct-blend
consumer, using the existing model/property index resolver. It does not continuously
copy model values into machine inputs or simulate motion on the host.

`examples/authoring/model-blend-panel.v0.json` retains a full-weight rest motion and
two independently bound contributions. The host creates, initializes and binds the
model instance, then changes its `number(...)` properties. Resolve model/property
runtime names from the compile report, as in the numeric binding example above.
Changing a synthesized input, including with `render --input`, does not change a
model-controlled weight. The percentages, clamping and authored-order semantics are
the same as for input-driven direct blends.

Run `node tests/playwright/authoring-direct-blend-runtime.js --model-bound` for the
public-CLI/official-runtime proof. It retains evidence under
`target/playwright-behavior/model-blend`, separately from the existing input proof.
It checks model-only changes, input-only non-effects, partial/full weights,
independence, clamping, reversal and timed exit/resume. The normal typed-behavior CI
artifact retains both modes. Existing input-driven documents keep their bytes;
`authoring_format_version` stays 0.

## Parallel regions

A statechart may declare `regions` so that independent behavior runs at the same time. The statechart's own `states` and `transitions` remain layer 0 of the lowered state machine, and each region becomes an additional layer with its own `initial`, `states`, and `transitions`. Regions share the statechart's inputs, events, and listeners; they do not declare their own.

```json
"regions": [
  {
    "id": "alert",
    "initial": "calm",
    "states": [
      { "id": "calm", "motion": "lamp-calm-track" },
      { "id": "busy", "motion": "lamp-busy-track" }
    ],
    "transitions": [
      {
        "id": "escalate",
        "from": "calm",
        "to": "busy",
        "when": { "input": "load", "compare": "greater_or_equal", "value": { "kind": "literal", "value": 60, "unit": "scalar" } }
      },
      {
        "id": "settle",
        "from": "busy",
        "to": "calm",
        "when": { "input": "load", "compare": "less", "value": { "kind": "literal", "value": 60, "unit": "scalar" } }
      }
    ]
  }
]
```

Every layer is emitted with an entry state at index 0, the authored states from index 1 in authored order, an exit state last, and a transition from the entry state to the authored `initial` state. Scene paths are `/artboard/state_machines/{m}/layers/{n}/states/{i}`. Source-map authored ids inside a region are `statechart/region/state`, while ids on layer 0 stay `statechart/state`.

Region ids are unique within a statechart; a repeat fails with `duplicate_behavior_region` at `$.behavior.statecharts[i].regions[j].id`. A region id may not match any other id the statechart scopes either. States, transitions, inputs, events, listeners and regions all claim the source-map identity `{statechart}/{id}`, and consumers resolve an entry by first match, so a collision makes that lookup ambiguous; it fails with `behavior_region_id_collision` at the same path. Every state and transition diagnostic listed above applies inside a region under the same `.regions[j]` prefix. The region above is from `examples/authoring/interactive-console.v0.json`, whose other region, `stream`, carries a token across the artboard while layer 0 is still in `standby`. Regions do not require inputs: `examples/authoring/signal-weave.v0.json` declares three layers with no inputs and no transitions between authored states, so each layer plays its own track.

Additive blend states, advanced exit timing, and view-model properties beyond boolean and number remain outside the current typed subset and continue under the behavior roadmap. `raw_state_machines` remains available for canonical behavior that is not yet represented by the typed frontend.

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

### Fixed direct weights

Use `weight` for a contribution that is known at compile time. A child selects
exactly one of `input`, `binding` and `weight`; runtime indices remain unavailable.

```json
{
  "id": "blending",
  "direct_blend": {
    "motions": [
      { "motion": "rest-track", "weight": { "kind": "literal", "value": 100, "unit": "scalar" } },
      { "motion": "left-track", "weight": { "kind": "parameter", "name": "contribution" } },
      { "motion": "right-track", "input": "right-weight" }
    ]
  }
}
```

Declare `contribution` in document `parameters`, for example
`{"value": 25.5, "unit": "scalar"}`. The normal scalar-expression operators are
supported. Fixed values must be finite, representable scene scalars between 0 and
100 inclusive; fractions are valid percentages. `invalid_blend_weight` reports a
range violation at the child's `.weight` before narrowing to the runtime float.
Unknown parameters, wrong units, division by zero and numeric representation
errors preserve their existing expression diagnostics and nested authored paths.
Unlike dynamic controls, invalid authored constants are rejected, not clamped.

The compiler emits a native constant without an input or binding object. Input,
model-bound and fixed contributions can be mixed, including in parallel regions.
They retain authored order and are not normalized: a full-weight rest motion
last can overwrite motion that earlier children applied. Changing a fixed value
requires recompilation; use `input` or `binding` for runtime changes.

`examples/authoring/fixed-blend-panel.v0.json` replaces the original panel's
artificial `foundation` input with a fixed 100-percent rest contribution. Its
left/right controls remain independent. Run the shared runtime harness with
`--fixed-weights` for this example plus parameterized zero, fractional, half,
full, reordered and reset/resume cases with no numeric inputs at all. Add
`--model-bound` to prove a fixed rest contribution composes with model-driven
weights without input mirroring. Sources, binaries, compile reports, measurements,
PNGs and hashes are retained under `target/playwright-behavior/fixed-blend` and
`model-fixed-blend`, alongside the unchanged existing runtime modes.
