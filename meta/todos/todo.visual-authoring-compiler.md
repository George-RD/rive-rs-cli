---
node: rive-cli.intelligence.authoring
status: in_progress
created: 2026-07-29
---

# P1 — Implement the visual/component lowering slice

Lower inline geometry, paint, trim, text, assets, transforms, parameters,
components, instances, and bounded patterns into explicit SceneSpec objects.

## Progress

- Triangle, polygon, and star authoring nodes are implemented in PR #136 through
  the existing expression, component, source-map, SceneSpec, builder, and encoder
  path.
- Polygon and star point counts are validated at the authored path; star inner
  radius is a typed scalar ratio.
- Optional typed solid strokes are implemented in PR #137 through components and
  parameter overrides, with stroke width diagnostics mapped back to the authored
  definition path.
- Shape validation and lowering now share one shape descriptor, and JSON
  authoring input is parsed once before validation and lowering.
- Typed linear and radial gradient fills are implemented in PR #138 while
  preserving the compact solid-colour string form. Endpoint and stop expressions
  flow through component parameters, and every generated stop is source-mapped.
- Typed linear and radial gradient strokes are implemented in PR #140 through the
  shared `PaintSpec` and paint-lowering helper. `stroke.paint` is canonical while
  the previous solid `stroke.color` form remains accepted as a parser alias.
- Typed trim paths on strokes are implemented in PR #141. Start, end, and optional
  offset expressions flow through component parameters and overrides; generated trim
  objects, runtime names, SceneSpec paths, and diagnostics remain source-mapped.
- Literal text nodes are implemented in PR #143 with parameterized numeric styling,
  semantic alignment and overflow, derived sizing, shared paints, deterministic
  runtime names, and complete source maps.
- The current text slice proves canonical structure and encoding. Official-runtime
  glyph rendering remains gated on the separate font-asset embedding slice.
- The authored visual model is split from the core document/error model in PR #144.
  The generated schema remains byte-identical while `spec.rs` and `visual.rs` are
  each below Cairn's module-size guideline.
- The authored lowering pipeline is split by node, text, shape, and paint responsibility
  in PR #145. The parent lowerer is 466 lines; its focused modules are 225–294 lines.
  Schema, validation, generated SceneSpec, runtime names, and source maps remain fixed.
- Deterministic bounded grid patterns are implemented in PR #146. Row-major expansion,
  component parameter overrides, stable generated IDs, complete source maps, and a global
  nested-pattern cell budget are pinned by the authoring grid contract suite.
- Review hardening for PR #146 mechanically synchronizes the published AuthoringSpec schema and preserves component-definition diagnostic paths through nested grid items.
- Remaining work includes font and image assets, radial/mirror/distribute/along-path patterns,
  constraints, and
  a complex static showcase without raw escapes.

## Acceptance criteria

- Common shapes do not require hand-authored shape/geometry/paint scaffolding.
- Components can be instantiated with parameter overrides and stable IDs.
- Grid, radial, mirror, distribute, and along-path patterns are deterministic.
- Simple align, center, offset, and spacing constraints produce actionable cycle errors.
- Source maps identify every expanded SceneSpec object produced by an authored concept.
- A complex static showcase is reproduced without raw escapes for supported concepts.

## Dependency

Depends on `todo.authoring-spec-v0.md`.
