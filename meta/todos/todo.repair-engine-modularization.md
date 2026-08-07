---
node: rive-cli.intelligence.ai
status: open
created: 2026-07-29
---

# P4 — Split the repair engine behind characterization tests

Decompose the existing repair engine only after the authoring and evaluation
boundaries are stable. Preserve behavior with characterization tests and avoid
optimizing a direct-SceneSpec generation path that is no longer the product
priority.

## Immediate correctness gate

A bounded P0 defect interrupts the deferred P4 refactor. `deduplicate_names` keeps
the first object named `foo`, renames later duplicates, then records multiple
`foo -> foo_N` rewrites in a map keyed by the original name. The final duplicate
wins, so recognized references to `foo` can be redirected to the last renamed
object even though the first object still owns `foo`.

The intended duplicate cannot be inferred from an ambiguous pre-repair reference.
The accepted safe policy is therefore:

- the first object retains the original name;
- later duplicates receive deterministic suffixes;
- an existing reference to the duplicated original name remains on the first object;
- repair must not silently infer that the reference meant a later duplicate.

Implement this as a focused defect PR before the next Authoring feature or
architecture slice:

1. Add a RED regression with three objects named `foo` and at least one currently supported keyframe or nested-artboard reference to `foo`.
2. Require repaired names `foo`, `foo_2`, and `foo_3` while the reference remains `foo`.
3. Remove or narrow the ambiguous global duplicate-name reference rewrite; do not combine this fix with module extraction.
4. Run all repair, builder, CLI, browser, Cairn, and exact-head CI gates and record the evidence here.

## Acceptance criteria

- The immediate duplicate-reference regression is fixed without changing unrelated repair behavior.
- Existing repair outputs and diagnostics remain compatible.
- Passes have explicit ownership and bounded inputs/outputs.
- Repair results map back to AuthoringSpec source paths when the frontend exists.

The broader P4 modularization remains deferred until the Authoring compiler
architecture is stable. Line count alone is not sufficient reason to split a
cohesive module.
