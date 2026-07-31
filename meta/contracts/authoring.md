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
- reusable components, instances, bounded deterministic grid, radial, mirror, distribute, and along-path patterns, and simple constraints;
- semantic font asset IDs that text can reference without runtime indices;
- semantic image asset IDs that static image nodes can reference without runtime indices;
- deterministic file-scope asset ordering and collision-checked runtime names;
- preservation of asset sources in lowered `SceneSpec`, with actual file embedding
  performed only when the canonical builder receives an explicit base directory;
- poses, compact motion tracks, shared easing definitions, and named statecharts;
- view-model-first data bindings and events;
- a raw SceneSpec escape hatch for unsupported advanced Rive objects;
- validation at each lowering stage and no direct binary encoding path.

The first version stays JSON. A custom textual DSL or general CAD constraint
solver requires separate evidence and an accepted decision.
