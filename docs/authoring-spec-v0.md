# AuthoringSpec v0

`AuthoringSpec` is the strict AI-facing and programmatic authoring frontend for `rive-cli`. It lowers into the existing canonical `SceneSpec`; it does not write `.riv` bytes directly and does not replace the builder or encoder.

## Versioning

- `authoring_format_version` is required and must be `0`; the generated schema constrains the field to that single value.
- Unknown fields are rejected at every typed authoring layer.
- A breaking field, semantic, unit, naming, or lowering change requires a new authoring format version.
- Additive compiler capability may be introduced within v0 only when existing v0 documents lower to the same canonical `SceneSpec` and source map.
- `scene_format_version` remains independently versioned. v0 currently lowers to `SceneSpec` version `1`.

`stacking`, motion `continuity` and `waypoint`, state `blend`, and statechart `regions` are optional fields whose defaults (`runtime`, `per_keyframe`, `auto`, and absent `blend` and `regions`) leave the canonical `SceneSpec` and source map unchanged. The `number` and `trigger` input kinds, the comparison and trigger transition conditions, and the `number_change` and `trigger_change` listener actions are new variants of the input, condition, and listener-action unions. A document that uses none of them lowers as it did before, so `authoring_format_version` stays `0`; `tests/showcase_artifact.rs` recompiles each committed showcase and compares the bytes against the checked-in `.riv`.

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
- `behavior`: typed boolean view models and bindings, `bool`, `number`, and `trigger` state-machine inputs, named Rive events, typed listeners, named states that play one motion track or blend at least two, parallel regions, and binding, boolean, comparison, and trigger transitions plus `raw_state_machines` for canonical expert escapes.

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

Typed behavior stays on the same compiler-owned `SceneSpec` draft as visual and motion lowering and keeps authored interaction free of runtime indices. A behavior model may currently declare boolean properties, and bindings select a model and property by authored ID. A statechart declares named states, an authored initial state, and named transitions whose `from` and `to` fields reference state IDs.

Statechart `inputs` are typed by `kind`. A `bool` input carries a boolean `value`, a `number` input carries a scalar expression, and a `trigger` input carries no value:

```json
"inputs": [
  { "kind": "number", "id": "load", "value": { "kind": "literal", "value": 0, "unit": "scalar" } },
  { "kind": "bool", "id": "armed", "value": false },
  { "kind": "trigger", "id": "reset" }
]
```

Statecharts also declare named Rive `events` and typed `listeners`. A listener targets either an authored visual ID for pointer interaction or an authored event ID when `listener_type` is `event`; the compiler resolves that semantic target to the generated runtime object name. The supported listener types are `enter`, `exit`, `down`, `up`, `move`, `event`, and `click`. The typed actions are `bool_change`, whose `value` defaults to `true`, `number_change`, which sets a number input from a scalar expression, and `trigger_change`, which fires a trigger. An action kind that does not match the declared kind of the input it names fails with `invalid_listener_input` at `$.behavior.statecharts[i].listeners[j].actions[k].input`.

Transition conditions are an untagged union of four forms:

- `{"binding": ..., "equals": <bool>}` checks a view-model binding.
- `{"input": ..., "equals": <bool>}` checks a boolean input.
- `{"input": ..., "compare": ..., "value": <scalar expression>}` checks a number input, where `compare` is `equal`, `not_equal`, `greater`, `greater_or_equal`, `less`, or `less_or_equal`.
- `{"trigger": ...}` fires on a trigger input.

The condition form and the input kind must agree. `unknown_behavior_input` fires when the named input is not declared and `invalid_condition_input` when the kinds differ, both at `$.behavior.statecharts[i].transitions[j].when.input`, or `.when.trigger` for the trigger form.

The compiler lowers model properties to Rive view models, explicit inputs to named state-machine inputs, events to named artboard event objects, listeners to canonical state-machine listeners, and conditions to runtime input names. Source-map entries preserve the authored model, property, binding, input, event, listener, statechart, states, and transitions. Unknown input, event, listener-target, and listener-action references fail at their authored JSON paths.

The canonical builder validates the merged graph. Behavior validation drops asset `source` fields from its copy of the lowered scene the same way the visual path does, so a document may declare `font_assets` or `image_assets` and a statechart together. Runtime contracts prove both interaction paths: changing a bound view-model boolean through the official web runtime changes state, while the compiled typed interaction fixture is also driven through the public `rive-cli render` interface with both `--input` and `--pointer`, and both must converge on the same visible state.

## Blend states

A behavior state declares exactly one of `motion` and `blend`. `motion` names an authored motion track. `blend` maps a number input onto at least two motion tracks, each with the input value at which that track is fully applied:

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

The state lowers to a `blend_state_1d` whose `input` is the lowered input name and whose children are `blend_animation_1d` entries naming the lowered animations. A state with neither field fails with `missing_state_motion` and a state with both fails with `ambiguous_state_motion`, both at the state path. `unknown_behavior_input` and `invalid_blend_input` fire at `$.behavior.statecharts[i].states[j].blend.input` when the named input is absent or is not a number input, and `unknown_behavior_motion` at `$.behavior.statecharts[i].states[j].blend.stops[k].motion` for a track that is not defined.

A blend needs between 2 and 1000 stops; a count outside that range fails with `invalid_blend_stops` at `$.behavior.statecharts[i].states[j].blend.stops`. Stop values must strictly increase. Rive's `BlendState1DInstance` binary-searches its children as ascending thresholds, so an out-of-order or repeated value would leave a stop unreachable; it fails with `invalid_blend_stop_order` at `$.behavior.statecharts[i].states[j].blend.stops[k].value`. Both checks run on the typed Rust path as well as through the published JSON schema.

Rive mixes the two neighbouring stop animations in sequence rather than as a weighted average, so an input between two stops does not render at the arithmetic midpoint. `tests/authoring_behavior_blend_runtime.rs` drives `examples/authoring/blend-meter.v0.json` at `load` 0, 50, and 100: the stops hold the needle within 3px of x 40 and x 200, and 50 renders it near x 146 rather than at x 120. The mapping stays monotonic; the test asserts that ordering rather than the exact middle position.

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

Additive blend states, direct blend states, transition duration and exit time, and view-model number and trigger properties remain outside the current typed subset and continue under the behavior roadmap. `raw_state_machines` remains available for canonical behavior that is not yet represented by the typed frontend.

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
