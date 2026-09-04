---
node: rive-cli.intelligence.authoring
status: done
created: 2026-07-29
completed: 2026-08-01
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
- Semantic font assets are implemented in PR #148. A deterministic `font_assets`
  registry lowers before visual nodes, text references assets by authored ID, and
  source-map offsets remain complete for both root and component-expanded text.
- PR #148 also proves actual vendored TTF byte embedding through the canonical
  builder with an explicit base directory. Pathless compiler validation preserves
  the returned asset source while avoiding filesystem-dependent lowering.
- Semantic image assets and transformable static image nodes are implemented in PR
  #149. Fonts lower before images, both registries are sorted by authored ID, and
  named references flow through the canonical global asset-ordinal resolver.
- PR #149 proves actual vendored PNG byte embedding and complete source maps for
  root and component-expanded image nodes without exposing runtime indices.
- Font and image declarations share one strict validator and deterministic lowering
  loop instead of maintaining duplicate asset-specific helpers. Parameter keys use
  the same accepted authored-key rule.
- The published AuthoringSpec schema has a tested regeneration path and now exposes
  both semantic asset registries and the static image node contract.
- The authored visual model is split from the core document/error model in PR #144.
  The generated schema remains byte-identical while `spec.rs` and `visual.rs` are
  each below Cairn's module-size guideline.
- The authored lowering pipeline is split by node, text, shape, and paint responsibility
  in PR #145. The parent lowerer is 466 lines; its focused modules are 225–294 lines.
  Schema, validation, generated SceneSpec, runtime names, and source maps remain fixed.
- Deterministic bounded grid patterns are implemented in PR #146. Row-major expansion,
  component parameter overrides, stable generated IDs, complete source maps, and a global
  nested-pattern cell budget are pinned by the authoring grid contract suite.
- Review hardening for PR #146 mechanically synchronizes the published AuthoringSpec
  schema and preserves component-definition diagnostic paths through nested grid items.
- Deterministic bounded radial patterns are implemented in PR #147. They share pattern
  traversal, expansion budgets, source-map lowering, and authored-path rewriting with
  grids; radius and angular expressions flow through component overrides.
- A Linux/macOS/Windows bit audit found platform-dependent standard-library trig output;
  authoring math now pins pure-Rust `libm` 0.2.16 and exact coordinate bits so radial
  SceneSpec output remains reproducible across supported build hosts.
- Pattern expansion accounting now charges every recursively generated node at inherited
  multiplicity, preventing grouped or component-backed radial items from bypassing the
  shared 10,000-node budget.
- Generated gradient stops now share the inherited pattern and component expansion budget.
- Raw SceneSpec escapes remain valid as single nodes but are rejected when a pattern would
  repeat them, because arbitrary embedded names and references cannot be safely namespaced.
- Deterministic semantic mirror patterns are implemented in PR #150. Vertical and
  horizontal axes lower to exactly two named cells through the shared repeated-pattern
  pipeline, including component definition paths, source maps, runtime-name collision
  checks, raw-scene repetition safety, canonical builder validation, and the inherited
  10,000-node expansion budget.
- Grid, radial, and mirror placement lowering now share explicit position, rotation,
  and scale metadata instead of duplicating pattern-specific wrapper construction.
- PR #151 hardens the merged mirror slice after delayed review by extracting the
  axis-to-placement mapping into one pure helper and pinning it with an inline
  `#[cfg(test)]` unit contract while retaining the public end-to-end contract suite.
- Review remediation keeps Rust source comment-free under the repository compliance
  policy; durable explanation remains in this Cairn work item and the public contracts.
- Endpoint-inclusive distribute patterns are implemented in PR #153. Two to 100
  copies lower at equal intervals along a typed straight segment, including both authored
  endpoints, component overrides, definition paths, source maps, runtime-name collision
  checks, canonical builder validation, and the inherited generated-node budget.
- Distribute lowering reuses the shared placement and repeated-pattern pipeline. Pattern
  count validation now accepts primitive-specific minimums instead of duplicating a
  separate bound check for the new node.
- Deterministic along-path patterns are implemented in PR #154. Two to 100 copies
  are spaced by total arc length across a typed polyline with two to 100 points, including
  both endpoints, optional tangent rotation, component overrides, stable source maps,
  canonical builder validation, and the inherited generated-node budget.
- Pure polyline sampling lives in a focused lowering helper with pinned `libm` distance
  and tangent math. Shared bounded-count validation now covers both pattern copies and
  path-point counts without duplicating range logic.
- PR #155 implements deterministic group-scoped align, center, offset, and ordered
  spacing constraints over direct-child `x` and `y` transform anchors. Component
  parameters and instance overrides flow through the same typed expression scope.
- Constraint assignments share one stable dependency graph and emit authored-path
  diagnostics for unknown or raw siblings, invalid or duplicate constraint IDs,
  duplicate entries, conflicting writes, invalid units, malformed spacing lists, and
  cycles with the authored anchor chain.
- PR #155 review hardening independently bounds dependency resolution, spacing lists,
  and each group's constraint declarations to 100, keeps those limits separate from
  pattern and path bounds, avoids cloning unconstrained groups, and pins align, center,
  and spacing behavior on both axes.
- Final PR #155 remediation adds a RED/GREEN contract for memoized-prefix traversal,
  validates dependency depth before memoized value evaluation, and centralizes immutable
  and mutable typed-node transform access behind one variant mapping.
- PR #156 completes the slice with `complex-static-showcase.v0.json`, a typed static
  composition combining reusable components, parameter overrides, expression math,
  linear and radial gradients, trimmed strokes, text, grid, radial, mirror, distribute,
  along-path patterns, and align, center, offset, and spacing constraints without raw
  SceneSpec, motion, or behavior escapes.
- The showcase contract proves deterministic lowering and source maps, expanded authored
  IDs, canonical SceneSpec construction, `.riv` encoding, and structural validation. Its
  test helper also removes duplicated deterministic-lowering setup from the existing
  authoring examples.
- TDD run `30691630011` passed formatting, Clippy, all 614 library tests, and every prior
  integration suite before failing only because the showcase fixture did not yet exist.
  GREEN run `30691846841` then passed the full Rust, browser, Cairn, official-runtime,
  demo, site, Playwright, and visual-regression matrix.

## Acceptance criteria

- Common shapes do not require hand-authored shape/geometry/paint scaffolding.
- Components can be instantiated with parameter overrides and stable IDs.
- Grid, radial, mirror, distribute, and along-path patterns are deterministic.
- Simple align, center, offset, and spacing constraints produce actionable cycle errors.
- Source maps identify every expanded SceneSpec object produced by an authored concept.
- A complex static showcase is reproduced without raw escapes for supported concepts.

## Stacking hardening inside the completed slice

Issue #193 adds explicit sibling stacking order. Exact head
`02d5621` passed CI run `33836991510` across formatting, Clippy, the Rust 1.88 minimum,
browser contracts, Cairn architecture validation, official-runtime evidence, demo, site,
Playwright, and visual regression, and PR #221 merged to `main` as `73598d5` on
2026-09-04. It hardens the slice PR #156 completed rather than reopening it: the progress
and acceptance criteria above are unchanged, and the status stays `done`.

- `StackingSpec` in `src/authoring/spec.rs` accepts `runtime` (default) or
  `back_to_front`. The optional `stacking` field sits on the `visual` section, on each
  entry of `components`, and on the `group` visual node.
- Rive paints the first sibling on top. `runtime` leaves the emitted child order
  unchanged; `back_to_front` reverses the emitted SceneSpec children, so the last
  authored sibling paints on top.
- Authored paths, component definition paths, diagnostic paths, and source-map entry
  order stay in authored order. Only `scene_paths` and the emitted child order change.
- Raw SceneSpec input is not reordered and keeps native runtime ordering.
- `tests/authoring_stacking_contract.rs` holds four contracts: `back_to_front` on the
  `visual` section lowered twice to the same scene and source map, on a group, on a
  component definition, and diagnostics under authored child indexes. Under
  `back_to_front` on a group, the second authored child keeps authored path
  `$.visual.nodes[0].children[1]` and receives scene path
  `/artboard/children/0/children/0`; a scalar-unit width on that child still reports
  `unit_mismatch` at `$.visual.nodes[0].children[1].width`.
- `tests/authoring_stacking_runtime.rs` holds two official-runtime contracts. Both
  compile `examples/authoring/stacking-card.v0.json` and render frame 0 at 128x128
  through the browser runtime. Under `back_to_front` the centre pixel is the cue
  `#22C55E` while the corner at (8, 8) is the surface `#C2410C`; under `runtime` the
  centre pixel is the surface, because the first authored sibling covers the cue.
- `stacking` is optional and `runtime` reproduces the previous output. The committed
  `examples/authoring/complex-animated-showcase.v0.riv` is byte-identical after the
  change, and `authoring_format_version` stays 0.

## Dependency

Depends on `todo.authoring-spec-v0.md`.
