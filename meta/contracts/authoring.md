---
node: rive-cli.intelligence.authoring
---
# AI-facing authoring frontend contract

The Authoring frontend is a strict, versioned JSON model that compiles
deterministically to canonical `SceneSpec`.

It must provide:

- stable author IDs and generated runtime names;
- a source map from authored concepts to expanded SceneSpec objects;
- typed units and safe expression trees, not arbitrary executable strings;
- reusable components, instances, bounded deterministic grid, radial, mirror, distribute, and along-path patterns, and group-scoped transform-anchor constraints;
- explicit sibling stacking order on the `visual` section, a component, or a group;
- constraints that reference direct typed siblings by stable authored ID, preserve component parameter and instance override semantics, bound each group to 100 declarations, and report invalid IDs, conflicts, bounded dependency depth, or cycles at authored paths;
- semantic font asset IDs that text can reference without runtime indices;
- semantic image asset IDs that static image nodes can reference without runtime indices;
- deterministic file-scope asset ordering and collision-checked runtime names;
- preservation of asset sources in lowered `SceneSpec`, with actual file embedding
  performed only when the canonical builder receives an explicit base directory;
- poses with transform, opacity, and parametric shape-dimension properties, compact motion tracks, shared easing definitions, and named statecharts;
- motion tracks whose interior waypoints keep velocity instead of settling at every keyframe;
- view-model-first data bindings and events;
- typed bool, number, and trigger statechart inputs, comparison and trigger transition
  conditions, one-dimensional blend states, and parallel regions;
- a non-fatal warning channel beside the fatal authored diagnostics;
- a raw SceneSpec escape hatch for unsupported advanced Rive objects;
- validation at each lowering stage and no direct binary encoding path.

Rive paints the first sibling on top. `stacking` sets that order and takes `runtime`
(default) or `back_to_front`. `runtime` emits children in authored order;
`back_to_front` reverses the emitted SceneSpec children, so the last authored sibling
paints on top. Authored paths, component definition paths, diagnostic paths, and
source-map entry order stay in authored order; only `scene_paths` and the emitted
child order change. Under `back_to_front` on a group, the second authored child keeps
authored path `$.visual.nodes[0].children[1]`, receives scene path
`/artboard/children/0/children/0`, and a scalar-unit width on it still reports
`unit_mismatch` at `$.visual.nodes[0].children[1].width`. Raw SceneSpec input is not
reordered and keeps native runtime ordering.

Incremental authoring operations target stable authored IDs and never runtime names,
SceneSpec paths, generated array indices, or binary indices. The shared operation
envelope supports replace, insert, move, and remove over authored visual concepts,
components, typed motion concepts, typed behavior concepts, and raw motion or behavior
fragments. Visual inserts can target the root visual list or an authored group;
ordered concepts can also be placed before or after an authored same-domain anchor.
For list-backed domains, insertion into the matching domain appends to authored order,
while before and after placements resolve one authored anchor in that domain. A move
removes one authored entity and inserts that same entity at the requested placement;
it never recreates the entity from lowered runtime state.

Visual targets use the same ancestor-scoped identity as the visual source map, for
example `frame/panel`; a local leaf ID such as `panel` is not an alias for that nested
concept. Pattern containers and component instances remain targetable as visual nodes,
but repeated pattern-item definitions, component definitions, and expanded instance
children are not independent visual-tree targets. Those concepts can expand to
multiple source-map identities or live outside the root visual tree. Non-visual
top-level concepts resolve by their stable authored IDs inside their typed domain. A
target or anchor must resolve exactly once: no match returns `unknown_authored_id`,
while multiple matches return `ambiguous_authored_id`. Placements cannot cross typed
authoring domains and return `invalid_operation_placement` when the entity, anchor,
or container types do not agree.

Every operation applies to a cloned `AuthoringSpec` and lowers the complete candidate
through the normal AuthoringCompiler and canonical SceneSpec validation path before a
changed document can be returned. `apply_operations` applies the same rule after each
step in a sequence, so every intermediate authored document is valid; a later failure
returns no partially mutated result. Reference and dependency failures therefore use
the existing authored-path diagnostics, such as `unknown_motion_target` or
`unknown_behavior_motion`, rather than silently deleting, rebinding, or retargeting a
dependent concept. The caller's input remains unchanged on every failure. A successful
operation returns both the changed `AuthoringSpec` and its lowered result;
deterministic lowering must preserve source-map entries and runtime bindings for
unaffected authored IDs unless the edited dependency genuinely requires a change.

The original `ReplaceVisualNode { target_id, node: VisualNode }` public operation shape
remains source-compatible with the first incremental slice; insert, move, remove, and
multi-operation application extend that contract rather than replacing it.

The current motion subset supports named transform, opacity, and positive pixel-valued
parametric shape-dimension poses, compact pose tracks, and shared cubic Bézier easing
definitions with authored visual targets,
scalar-expression frame timing and control points, `hold` or `linear`
interpolation, and `oneshot`, `loop`, or `pingpong` loop behavior. Easing time-axis
control points remain within zero and one, while value-axis control points may
overshoot. Authored opacity expressions resolve to scalar ratios in the inclusive
zero-to-one range. Width and height expressions resolve to positive pixels. Transform
and opacity properties target the authored node's primary transform binding, while
width and height target its parametric geometry bindings. The canonical builder's
animatable-property registry remains the final compatibility authority within each
role. Exactly one compatible runtime binding is required: no match returns
`unsupported_motion_property`, while multiple matches return
`ambiguous_motion_property_target` at the exact authored property path. A raw
compound target with two compatible geometry children is therefore rejected rather
than being routed according to incidental child order.

A keyframe may reference one named easing unless it uses `hold`; each referencing
animation receives the same stable local declaration required by SceneSpec
validation, the canonical builder deduplicates those declarations into one runtime
interpolator, and the authored source-map entry records every declaration. Every pose
used by one track declares the same target/property shape. Frame and duration
expressions must resolve to non-negative whole numbers. Bounded floating-point
round-off around a whole number is normalized deterministically through a capped
multi-ULP window while representable spacing remains below half a frame. Once one
ULP reaches half a frame, exact whole-frame equality is required; magnitudes where
one ULP reaches a whole frame are rejected because authored half-frame intent can no
longer be represented. Material fractional values are rejected. The complete
typed-motion document may expand to at most 10,000 canonical property-keyframe
values, preventing individually valid poses and tracks from creating an unbounded
Cartesian expansion. The validator reports this aggregate limit once, at the first
track that causes the document to cross the budget, while continuing to validate
later tracks for unrelated authored errors.

A track declares `continuity`, either `per_keyframe` (default) or `through`. Rive
attaches an interpolator to the keyframe that starts a segment, so an easing that
flattens at its end stops the target at every keyframe it governs. Under `through`,
every segment arriving at an interior keyframe is emitted as `linear` with no
interpolator and the target keeps its speed across that keyframe, while the segment
arriving at the last keyframe keeps its authored easing so the motion still settles at
the destination. A `hold` segment is never rewritten. One keyframe overrides its track
through `waypoint`: `transit` forces the rewrite inside a `per_keyframe` track, `settle`
suppresses it inside a `through` track, and `auto` (default) follows the track. Neither
`transit` nor `settle` is valid on the first or last keyframe once the track's keyframes
are sorted by frame; that returns `waypoint_not_interior` at
`$.motion.tracks[i].keyframes[j].waypoint`. An interpolator that governs no remaining
segment after the rewrite is not emitted.

`LoweredAuthoring.warnings` carries non-fatal `AuthoringDiagnostic` values and lowering
still succeeds. `waypoint_stop_start` is reported at `$.motion.tracks[i].keyframes[j]`
when an interior keyframe with `waypoint: auto` is entered and left on the same easing
whose end tangent is flat, meaning `y2` equals 1 while `x2` stays below 1, and at least
one animated property continues in the same direction through that keyframe. Without
`--json`, `rive-cli authoring compile` prints each warning to stderr as
`warning: {path} [{code}]: {message}` before the byte count; with `--json` it instead
carries them in a `warnings` array beside `bytes_written`, `output_path`, and
`source_map` in the success envelope.

Tracks lower through the canonical builder, retain deterministic runtime names, and
map errors and runtime objects back to `$.motion.tracks` and `$.motion.poses`. Visual
motion targets are indexed once from the authored source map as ordered runtime
bindings. Before indexing, every named runtime object must have a scene path at the
same position; mismatched cardinality or a path that does not resolve to a typed
scene object returns `invalid_source_map_binding`. The existing source-map form for
an unnamed raw object may retain one root scene path without a runtime binding. Each
valid binding is internally paired and assigned a semantic role before property
routing; downstream motion code does not zip unchecked parallel vectors or silently
drop malformed bindings.

Raw state-machine escapes may reference generated track runtime names in the same
document because final canonical validation occurs only after typed animations are
present. Failed invariants return structured authored diagnostics rather than
panicking. Semantic motion helpers and color or other property tracks beyond transform,
opacity, width, and height remain separate roadmap slices.

A statechart declares typed inputs as `{"kind": "bool", "id": ..., "value": ...}`,
`{"kind": "number", "id": ..., "value": <scalar expression>}`, or
`{"kind": "trigger", "id": ...}`. Transition conditions are an untagged union of four
forms: `{"binding": ..., "equals": ...}`, `{"input": ..., "equals": ...}`,
`{"input": ..., "compare": ..., "value": <scalar expression>}` where `compare` is
one of `equal`, `not_equal`, `greater`, `greater_or_equal`, `less`, or
`less_or_equal`, and `{"trigger": ...}`. The three input forms require a `bool`,
`number`, and `trigger` input respectively: a mismatch returns
`invalid_condition_input` and an undeclared id returns `unknown_behavior_input`, both
at `$....transitions[i].when.input` or `$....transitions[i].when.trigger`. The
`binding` form instead resolves against `bindings` and returns
`unknown_behavior_binding` at `$....transitions[i].when.binding`. Listener actions are
`bool_change`, `number_change` with a scalar `value`, and `trigger_change`; an action
whose kind does not match the declared input kind returns `invalid_listener_input` at
that action's `input` path.

A behavior state declares exactly one of `motion` and `blend`. Neither returns
`missing_state_motion` and both return `ambiguous_state_motion`, each at the state
path. `blend` is `{"input": <number input id>, "stops": [{"motion": <track id>,
"value": <scalar expression>}, ...]}` and lowers to a `blend_state_1d` whose children
are `blend_animation_1d` entries naming the lowered animations.
`docs/authoring.schema.v0.json` records `minItems` 2 and `maxItems` 1000 on `stops`,
and the compiler enforces the same bound on the typed path, where no JSON schema runs:
a count outside it returns `invalid_blend_stops` at `$....states[i].blend.stops`. Stop
values must strictly increase, because `BlendState1DInstance::animationIndex` binary-
searches its children as ascending thresholds; a value that does not exceed its
predecessor returns `invalid_blend_stop_order` at
`$....states[i].blend.stops[j].value`. The comparison narrows each value to `f32`
first, the width `BlendState1DChildSpec::BlendAnimation1D` carries and the encoder
writes, so two thresholds that differ only below `f32` precision are rejected rather
than reaching the runtime as duplicates. A `blend.input` that is not a number input
returns `invalid_blend_input` and one that does not exist returns
`unknown_behavior_input`, both at `$....states[i].blend.input`; an unknown stop track
returns `unknown_behavior_motion` at `$....states[i].blend.stops[j].motion`. Rive
mixes the two neighbouring stop animations sequentially rather than averaging them, so
stops at 0 and 100 driving positions 40px and 200px place input 50 near 146px rather
than 120px. The mapping stays monotonic.

`regions` adds parallel layers to a statechart, each `{"id", "initial", "states",
"transitions"}`. The statechart's own states remain layer 0, and each region becomes one
further state-machine layer with its own entry state, exit state, and entry transition.
Region ids are unique within a statechart; a repeat returns `duplicate_behavior_region`
at `$....regions[i].id`. A region id may not alias any other id the statechart scopes
either. States, transitions, inputs, events, listeners and regions all take the
source-map identity `{statechart}/{id}`, and consumers resolve an entry by first
match, so a collision would make the lookup ambiguous; it returns
`behavior_region_id_collision` at the same path. Source-map authored
ids inside a region are
`statechart/region/state` and their scene paths are
`/artboard/state_machines/{m}/layers/{n}/states/{i}`.

Typed behavior validates its lowered scene with file-asset `source` fields removed, the
same way the visual path does, so a document may declare `font_assets` or `image_assets`
alongside a statechart. Additive blend states, direct blend states, transition duration
and exit time, and view-model properties other than `bool` are not exposed by this
frontend. `stacking`, `continuity`, `waypoint`, `blend`, and `regions` are optional and
their defaults reproduce the previous lowered output, so `authoring_format_version`
remains 0.

The first version stays JSON. Its constraints align or derive direct-child `x` and
`y` transform anchors; they are not a rendered-bounds or general CAD solver. A
custom textual DSL or broader constraint system requires separate evidence and an
accepted decision.
