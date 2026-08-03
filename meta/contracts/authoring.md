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
- poses, compact motion tracks, shared easing definitions, and named statecharts;
- view-model-first data bindings and events;
- a raw SceneSpec escape hatch for unsupported advanced Rive objects;
- validation at each lowering stage and no direct binary encoding path.

The current motion subset supports named transform poses and compact pose tracks
with authored visual targets, scalar-expression frame timing, `hold` or `linear`
interpolation, and `oneshot`, `loop`, or `pingpong` loop behavior. Every pose used
by one track declares the same target/property shape. Frame and duration
expressions must resolve to non-negative whole numbers; bounded floating-point
round-off around a whole number is normalized deterministically through a capped
multi-ULP window that never becomes narrower than one representable step at the
evaluated magnitude, while material fractional values are rejected. The complete
typed-motion document may expand to at most 10,000 canonical property-keyframe
values, preventing individually valid poses and tracks from creating an unbounded
Cartesian expansion. Tracks lower through the canonical builder, retain
deterministic runtime names, and map errors and runtime objects back to
`$.motion.tracks` and `$.motion.poses`. Visual motion targets are indexed once from
the authored source map, and failed invariants return structured authored
diagnostics rather than panicking. Raw state-machine escapes may reference
generated track runtime names in the same document because final canonical
validation occurs only after typed animations are present. Shared easing
definitions, semantic motion helpers, non-transform property tracks, and typed
statecharts remain separate roadmap slices.

The first version stays JSON. Its constraints align or derive direct-child `x` and
`y` transform anchors; they are not a rendered-bounds or general CAD solver. A
custom textual DSL or broader constraint system requires separate evidence and an
accepted decision.
