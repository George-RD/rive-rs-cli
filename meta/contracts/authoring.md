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
- constraints that reference direct typed siblings by stable authored ID, preserve component parameter and instance override semantics, bound each group to 100 declarations, and report invalid IDs, conflicts, bounded dependency depth, or cycles at authored paths;
- semantic font asset IDs that text can reference without runtime indices;
- semantic image asset IDs that static image nodes can reference without runtime indices;
- deterministic file-scope asset ordering and collision-checked runtime names;
- preservation of asset sources in lowered `SceneSpec`, with actual file embedding
  performed only when the canonical builder receives an explicit base directory;
- poses with transform, opacity, and parametric shape-dimension properties, compact motion tracks, shared easing definitions, and named statecharts;
- view-model-first data bindings and events;
- a raw SceneSpec escape hatch for unsupported advanced Rive objects;
- validation at each lowering stage and no direct binary encoding path.

The current motion subset supports named transform, opacity, and positive pixel-valued
parametric shape-dimension poses, compact pose tracks, and shared cubic Bézier easing
definitions with authored visual targets,
scalar-expression frame timing and control points, `hold` or `linear`
interpolation, and `oneshot`, `loop`, or `pingpong` loop behavior. Easing time-axis
control points remain within zero and one, while value-axis control points may
overshoot. Authored opacity expressions resolve to scalar ratios in the inclusive
zero-to-one range. Width and height expressions resolve to positive pixels and select
the first compatible runtime object represented by an authored visual node, while
transform properties remain on its root transform object. A keyframe may reference
one named easing unless it uses `hold`;
each referencing animation receives the same stable local declaration required by
SceneSpec validation, the canonical builder deduplicates those declarations into one
runtime interpolator, and the authored source-map entry records every declaration.
Every pose used by one track declares the same target/property shape. Frame and
duration expressions must resolve to non-negative whole numbers. Bounded
floating-point round-off around a whole number is normalized deterministically
through a capped multi-ULP window while representable spacing remains below half a
frame. Once one ULP reaches half a frame, exact whole-frame equality is required;
magnitudes where one ULP reaches a whole frame are rejected because authored
half-frame intent can no longer be represented. Material fractional values are
rejected. The complete typed-motion document may expand to at most 10,000 canonical
property-keyframe values, preventing individually valid poses and tracks from
creating an unbounded Cartesian expansion. The validator reports this aggregate
limit once, at the first track that causes the document to cross the budget, while
continuing to validate later tracks for unrelated authored errors. Tracks lower
through the canonical builder, retain deterministic runtime names, and map errors
and runtime objects back to `$.motion.tracks` and `$.motion.poses`. Visual motion
targets are indexed once from the authored source map as ordered runtime object
candidates. Every authored property selects the first candidate accepted by the
canonical builder's animatable-property registry before keyframe emission. A raw SceneSpec
target that resolves to a fill, stroke, or other incompatible object fails with
`unsupported_motion_property` at the exact authored property path rather than
surfacing later as `builder_rejected_scene`. Failed invariants return structured
authored diagnostics rather than panicking. Raw state-machine escapes may reference
generated track runtime names in the same document because final canonical
validation occurs only after typed animations are present. Semantic motion helpers,
color and further non-transform property tracks, and typed statecharts remain
separate roadmap slices.

The first version stays JSON. Its constraints align or derive direct-child `x` and
`y` transform anchors; they are not a rendered-bounds or general CAD solver. A
custom textual DSL or broader constraint system requires separate evidence and an
accepted decision.
