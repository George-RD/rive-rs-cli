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
`{"kind": "trigger", "id": ...}`. Transition conditions are an untagged union of five
forms: `{"binding": ..., "equals": ...}`, `{"input": ..., "equals": ...}`,
`{"input": ..., "compare": ..., "value": <scalar expression>}` where `compare` is
one of `equal`, `not_equal`, `greater`, `greater_or_equal`, `less`, or
`less_or_equal`, and `{"trigger": ...}`. The three input forms require a `bool`,
`number`, and `trigger` input respectively: a mismatch returns
`invalid_condition_input` and an undeclared id returns `unknown_behavior_input`, both
at `$....transitions[i].when.input` or `$....transitions[i].when.trigger`. The
`binding` forms instead resolve against `bindings`: `equals` requires a boolean
property, while `{"binding": ..., "compare": ..., "value": <scalar expression>}`
requires a number property. A mismatch returns `invalid_condition_binding` and an
unknown id returns `unknown_behavior_binding`, both at the authored `.when.binding`.
Numeric model values and thresholds use document parameters, scalar units and the
existing finite/f32-survival checks, including unused properties. Each using chart
gets one synthesized input per binding, shared across its regions. The condition
reads a host-initialized bound model instance, not a synchronized machine input.
The authored property value initializes the synthesized input only. Listener actions are
`bool_change`, `number_change` with a scalar `value`, and `trigger_change`; an action
whose kind does not match the declared input kind returns `invalid_listener_input` at
that action's `input` path.

A transition may declare `duration_ms` as a scalar expression evaluated in the
document parameter scope. It must resolve to a finite whole number from 0 through
`u32::MAX` milliseconds; negative, fractional, and oversized results return
`invalid_transition_duration` at its authored `.duration_ms` path. Expression errors
keep their existing codes and paths. Region transitions follow the same rule. Only a
supplied duration emits canonical `duration`; omission preserves the previous scene
and source map, and explicit zero preserves instantaneous binary behavior. Duration
controls the blend after a condition fires, not an exit-time gate or percentage.

A transition may also declare `exit_time_ms`, evaluated in document parameter scope
as whole milliseconds from 0 through `u32::MAX`. Invalid numeric values report
`invalid_transition_exit_time`; expression errors retain their own codes and paths.
Only named motion sources are supported: a blend source reports
`unsupported_transition_exit_source` at the authored `.exit_time_ms` path. Blend
destinations remain valid. Root and region transitions use the same compiler path.
The gate requires outgoing animation time and the existing condition, not elapsed
time since the input. Duration controls blending independently; runtime loop timing
is unchanged. Omission/null preserves output and source-map identity; explicit zero
enables a zero-time gate and therefore changes encoded flags. Canonical `exit_time`
is an unsigned 32-bit value wired through the existing transition object; no second
lowering pass or encoder is introduced.

A behavior state declares exactly one of `motion`, `blend`, and `direct_blend`.
None returns `missing_state_motion` and multiple sources return
`ambiguous_state_motion`, each at the state path. `blend` is `{"input": <number input id>, "stops": [{"motion": <track id>,
"value": <scalar expression>}, ...]}` or the same stops with a numeric `binding`
instead of `input`. It lowers to a `blend_state_1d` whose children are
`blend_animation_1d` entries naming the lowered animations.
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
alongside a statechart. Additive blend states, advanced exit timing, and view-model properties beyond `bool` and `number` are not
exposed by this frontend.
`stacking`, `continuity`, `waypoint`, `blend`, `direct_blend`, `regions`, `duration_ms`, and `exit_time_ms` are optional and
their defaults reproduce the previous lowered output, so `authoring_format_version`
remains 0.

The first version stays JSON. Its constraints align or derive direct-child `x` and
`y` transform anchors; they are not a rendered-bounds or general CAD solver. A
custom textual DSL or broader constraint system requires separate evidence and an
accepted decision.

## Input-driven direct blends (#239)

A state chooses exactly one of `motion`, `blend`, and `direct_blend`. Direct blends
contain 1..=1000 ordered `{motion, input}` children naming typed motion tracks and
chart-local number inputs. Canonical animation indices come from the existing
lowered scene; input indices include any preceding binding-generated inputs.
Root charts and regions share this lowering. The canonical builder and encoder
remain the only binary path. Source-map state identities and omitted-field output
remain stable. Invalid references, kinds, counts and competing sources are rejected
at authored paths. Removing a referenced motion is atomic.

Runtime weights are clamped percentages and applied sequentially, not normalized
relative shares. The panel example deliberately puts a full-weight rest motion
first, followed by separate motion contributions. The browser contract checks
partial/full/zero weights, independence, reversal, clamping and state transitions.
Duration is supported; exit-time gates on direct sources are rejected. Fixed weight expressions are exposed by #245 below; additive states are not exposed.

## Model-bound direct weights (#241)

Each direct-blend child selects exactly one of `{motion, input}`,
`{motion, binding}` and, after #245, `{motion, weight}`. Only a numeric model property is a valid bound weight. Unknown
binding and wrong-kind diagnostics use the child's authored `.binding` path;
model/property declaration errors retain their own paths. The strict typed union
and published schema reject ambiguous, missing, null and unknown fields.

Blend-only uses participate in the same used-binding collection as transitions.
Shared root/region/transition uses emit one bound input per chart, in authored
binding order. Canonical input indices include all preceding bound inputs; a shared
binding's source map retains every chart-local input path. Root and region states
reuse the same lowering and atomic-operation validation. Existing unbound documents
and source identities remain unchanged.

The canonical builder consumes bound number inputs as native direct-blend data
bindings. Model mutation controls the weight independently of the synthesized
machine input; initialization and binding of the model instance belong to the host.
No parallel compiler, host-side input mirroring or direct binary encoding is added.
The same browser harness verifies input-driven and model-bound modes separately.

## Numeric model-bound one-dimensional blends (#243)

`blend` has exactly one source: a chart-local number `input` or a document numeric
`binding`. Both/neither/null forms and runtime fields fail strict deserialization.
Unknown bindings and non-number properties report `unknown_behavior_binding` and
`invalid_blend_binding` at `.blend.binding`. Invalid model/property declarations
retain their existing authored paths. Stops retain scalar expressions, 2..=1000
cardinality and strict emitted-f32 ordering, including in parallel regions.

Blend-only uses participate in shared binding discovery. Root/region one-dimensional
and direct blends and transitions deduplicate bindings per chart, retain actual
input offsets, and map to the existing compiler-owned scene without a second pass.
The canonical named-input reference carries a view-model binding; the builder emits
native model-bound state objects, not synthetic-input synchronization. Each chart
resolves independently. Model initialization and instance binding remain host work.
Model changes affect active blends and are observed on re-entry after inactive
changes; input-only writes do not drive them. Existing input-only bytes remain stable.

Public contracts cover source exclusivity, validation paths, float stop ordering,
region-only discovery, shared/multi-chart uses, nonzero/multibyte indices,
deterministic maps/bytes, native encoding and atomic-edit rejection. The three-stop
panel example and existing browser harness's one-dimensional mode retain CLI/source,
binary, measured renders and hashes in the typed-behavior runtime artifact.

## Fixed direct-blend weights (#245)

`{motion, weight}` accepts a scalar expression evaluated against document
parameters. The result must be finite and representable in the runtime float,
within 0..=100 percent; fractions are valid. Out-of-range constants report
`invalid_blend_weight` at `.weight` before float narrowing. Existing expression
errors retain their specific codes and authored paths. The typed union and
published schema reject multiple sources, null, missing and runtime-only fields.

Root and parallel-region lowering emit `blend_source: 1` and `mix_value` through
the existing canonical direct-blend child. Fixed weights allocate no inputs or
model contexts. Mixed sources retain authored child order and actual chart-local
input indices. Old input/model-driven JSON, binary output and source-map identities
remain unchanged. Atomic operations reuse these validations and roll back failure.

The public fixed-blend contracts cover encoding, scalar boundaries/arithmetic,
strict schema forms, diagnostic paths, non-finite programmatic parameters, regions,
mixed source offsets across charts, ordering, rollback and legacy model binaries.
The existing input-binary regression remains in the direct/model contracts.
The shared browser harness adds fixed/input and fixed/model modes without replacing
existing cases. Pure-constant cases contain only reset/resume trigger inputs and
must render zero, fractional, half and full weights, authored-order overwrites and
timed exit/re-entry. Evidence includes exact sources, binaries and rendered pixels.
