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

## Dependencies

Depends on stable visual, motion, and behavior frontend contracts and their source
maps. This work blocks the complex AI-generation skills milestone.
