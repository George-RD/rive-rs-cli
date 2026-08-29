---
node: rive-cli.intelligence.authoring
status: done
created: 2026-08-03
completed: 2026-08-29
---

# P3 — Add incremental typed authoring operations

Let agents change an existing `AuthoringSpec` through small, validated operations
instead of regenerating the whole document. Operations target authored concepts
and semantic fields, then reuse the canonical compiler, limits, diagnostics, and
source maps after every change.

## Scope

- Insert a typed visual, motion, or behavior concept relative to a stable authored ID.
- Replace one authored concept or semantic property without rewriting unrelated content.
- Move or reorder concepts where authored order has defined meaning.
- Remove concepts only when references and dependencies remain valid.
- Apply a sequence transactionally, with no partially mutated result after failure.

## Acceptance criteria

- Operations address stable authored IDs and typed semantic paths, never runtime names or generated array indices.
- Every operation validates schema, references, units, cycles, and expansion limits before returning a changed document.
- Failed operations return authored-path diagnostics and leave the input document unchanged.
- Unaffected concepts retain stable IDs, deterministic lowering, and equivalent source-map identity.
- Visual, motion, and behavior operations share one operation envelope and validation pipeline rather than separate ad hoc patch formats.
- Repair and agent workflows can apply the smallest corrective operation and compile immediately instead of regenerating the complete document.
- A multi-step integration suite proves insert, replace, move, remove, rollback, and deterministic reapplication.

## Delivered slices

- #184 / PR #210 established the shared operation envelope with atomic
  `ReplaceVisualNode`: it targets root visual nodes and group descendants by the
  same stable ancestor-scoped identities used by the visual source map, such as
  `frame/panel`; edits are applied to a clone and the complete candidate is lowered
  through the normal AuthoringCompiler before it can be returned. Pattern containers
  and component instances are targetable visual nodes, while repeated pattern-item
  definitions, component definitions, and expanded instance children remain outside
  the root visual target space because they expand to multiple source-map identities
  or live outside the root visual tree.
- #185 / PR #211 completes the milestone with typed insert, move, and remove across
  visual concepts, components, motion concepts, behavior concepts, and raw motion or
  behavior fragments. `AuthoringPlacement` addresses authored containers or
  same-domain before/after anchors without runtime indices. Visual targets retain the
  scoped source-map identity contract; list-backed concepts resolve stable IDs within
  their typed domain.
- `apply_operations` makes multi-step edits transactional at the API boundary and
  runs canonical lowering after every step. A dependency-invalid intermediate or
  final document returns authored diagnostics and exposes no partial changed document.
  Motion and behavior references are never silently retargeted after remove or move.
- Contract tests cover insert, move, remove, rollback, dependency failures,
  cross-domain placement rejection, unaffected source-map/runtime identity, canonical
  builder validation, cross-domain visual/motion/behavior insertion, and deterministic
  replay. Existing #184 replacement contracts continue to cover replace behavior.

## Dependencies

Depends on stable visual, motion, and behavior frontend contracts and their source
maps. #184 delivered the initial replace seam and #185 completes the incremental
operations milestone. Complex AI generation in #186 can now proceed once this PR's
exact-head verification is green and merged.
