---
id: res.ai-authoring-format
nodes:
  - rive-cli.intelligence.ai
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
sources:
  - src.scene-spec
  - src.scene-prompt-subset
  - src.control-panel-showcase
  - src.ai-authoring-discussion
  - src.openscad-modules
  - src.lottie-layers
  - src.dotlottie-v2
  - src.rive-data-binding
date: 2026-07-29
method: primary-and-comparative
---

# AI authoring format for complex Rive files

## Finding

The current JSON `SceneSpec` is a strong canonical intermediate representation
and a poor direct authoring language for complex AI-generated Rive files. It is
explicit, deterministic, close to the runtime object graph, and easy to validate.
Those same properties force a model to perform compiler work while also making
design decisions.

## Sources of avoidable model complexity

- A simple visual concept expands into runtime containment objects: shape,
  geometry, fill or stroke, paint, gradient stops, trim paths, and generated names.
- Names simultaneously act as labels, identities, animation targets, constraint
  references, listener targets, and blend references.
- Static poses are encoded as constant animations because state machines consume
  animation states.
- Timelines repeat fps, durations, easing definitions, target/property pairs, and
  frame objects.
- State transitions use array indices, so inserting a state can invalidate later
  transitions.
- Raw numbers carry hidden units and runtime enums: frames, seconds, radians,
  percentages, normalized coordinates, and integer codes.
- The full object union is too broad for reliable model generation. The existing
  prompt schema already exposes a deliberately smaller subset.

The `control_panel` showcase demonstrates all of these at once: repeated visual
object scaffolding, duplicated easing declarations, constant low/high poses
encoded as animations, and indexed state-machine layers.

## Comparison with Lottie and dotLottie

Classic Lottie has useful concepts—assets, precompositions, transforms, markers,
and a consistent static-versus-keyframed property model—but its raw JSON is more
cryptic and index-heavy than SceneSpec. Copying its short-key representation would
make AI authoring worse.

dotLottie v2 adds named state machines and interactions around packaged Lottie
animations. Its named behavior syntax is useful precedent, but Rive behavior is
more deeply integrated with artboard objects, blend states, view models, and data
bindings. The authoring frontend should preserve that richer model rather than
flattening Rive into a Lottie-shaped timeline format.

## Concepts worth borrowing from parametric CAD

Borrow the bounded concepts that reduce repetition and preserve intent:

- named parameters and typed derived expressions;
- reusable components and instances;
- compositional transforms;
- grid, radial, path, mirror, distribute, and stagger patterns;
- simple explicit layout and geometric constraints;
- stable feature identities that survive expansion.

Do not begin with a general simultaneous constraint solver, arbitrary scripts,
CSG, or a CAD feature-history tree. Those add opaque failure modes before the
common 2D animation cases are solved.

## Recommended model

Treat a complex animation as three connected authored graphs:

1. **Visual graph** — geometry, hierarchy, styles, assets, rigs, layout,
   components, parameters, patterns, and constraints.
2. **Motion graph** — named poses, clips, compact property tracks, easing,
   procedural helpers, and blend definitions.
3. **Behavior graph** — typed view models, bindings, events, named statecharts,
   transitions, and parallel behavior regions.

Compile these through a strict `AuthoringSpec` frontend:

```text
AuthoringSpec
  -> deterministic lowering + source map
SceneSpec
  -> existing builder and validator
Rive object graph
  -> existing encoder
.riv
  -> official-runtime and semantic evaluation
```

The compiler owns generated names, containment objects, index assignment, unit
normalization, keyframe expansion, and mechanism selection. An authored pose may
lower to a constant animation, blend endpoint, or binding depending on supported
Rive semantics.

The persisted format can remain JSON. For model interaction, prefer typed
incremental operations—define component, instantiate, define pose, add track,
bind property, add state, connect transition—so each operation can compile and
validate before the next one.

## Conclusion

Do not expand the current plan into a family of skills that directly generate
large SceneSpec documents. Keep the existing skill as a bounded expert/raw
workflow for simple scenes. Build the authoring frontend and evidence gates first;
then build skills around that smaller, intent-preserving language.
