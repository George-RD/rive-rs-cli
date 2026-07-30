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
- Optional typed solid strokes lower through components and parameter overrides,
  with stroke width diagnostics mapped back to the authored definition path.
- Shape validation and lowering now share one shape descriptor, and JSON
  authoring input is parsed once before validation and lowering.
- Remaining work includes richer paint and trim, text and assets, bounded
  patterns, constraints, and a complex static showcase without raw escapes.

## Acceptance criteria

- Common shapes do not require hand-authored shape/geometry/paint scaffolding.
- Components can be instantiated with parameter overrides and stable IDs.
- Grid, radial, mirror, distribute, and along-path patterns are deterministic.
- Simple align, center, offset, and spacing constraints produce actionable cycle errors.
- Source maps identify every expanded SceneSpec object produced by an authored concept.
- A complex static showcase is reproduced without raw escapes for supported concepts.

## Dependency

Depends on `todo.authoring-spec-v0.md`.
