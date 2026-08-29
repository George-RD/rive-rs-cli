---
node: rive-cli.intelligence.authoring
status: open
created: 2026-08-03
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

- #184 / PR #210 establishes the shared operation envelope with atomic
  `ReplaceVisualNode`: it targets root visual nodes and group descendants by the
  same stable ancestor-scoped identities used by the visual source map, such as
  `frame/panel`; edits are applied to a clone and the complete candidate is lowered
  through the normal AuthoringCompiler before it can be returned. Pattern containers
  are targetable visual nodes, while repeated pattern-item definitions remain outside
  this first replace slice because one definition expands to multiple source-map
  identities. Unknown and ambiguous targets have authored diagnostics, failed
  candidates do not mutate the input, and contract tests retain all unaffected
  source-map identities and deterministic reapplication. Insert, move, remove,
  broader entity coverage, and multi-operation transactions remain future slices;
  #185 is the next ready incremental-operations item.

## Dependencies

Depends on stable visual, motion, and behavior frontend contracts and their source
maps. #184 is the first delivered slice; #185 completes the milestone and remains
the blocker for complex AI-generation skills.
