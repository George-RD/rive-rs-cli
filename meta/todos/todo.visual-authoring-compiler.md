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
- Remaining work includes text and assets, bounded patterns, constraints, and a
  complex static showcase without raw escapes.

## Acceptance criteria

- Common shapes do not require hand-authored shape/geometry/paint scaffolding.
- Components can be instantiated with parameter overrides and stable IDs.
- Grid, radial, mirror, distribute, and along-path patterns are deterministic.
- Simple align, center, offset, and spacing constraints produce actionable cycle errors.
- Source maps identify every expanded SceneSpec object produced by an authored concept.
- A complex static showcase is reproduced without raw escapes for supported concepts.

## Dependency

Depends on `todo.authoring-spec-v0.md`.
