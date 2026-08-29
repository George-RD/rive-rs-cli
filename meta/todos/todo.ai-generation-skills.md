---
node: rive-cli.intelligence.ai
status: open
created: 2026-07-29
unblocked: 2026-08-29
---

# P3 — Build AI generation skills on AuthoringSpec

Do not create a family of complex Rive-generation skills around raw SceneSpec.
The existing skill remains a bounded expert workflow for simple scenes and raw
escape-hatch work.

## Unblock conditions

- AuthoringSpec visual, motion, and behavior slices are merged.
- Representative complex showcases compile through the frontend.
- Runtime and semantic eval suites pass with retained evidence.
- Incremental typed authoring operations can compile and validate after each step.

All four readiness conditions are satisfied by the Authoring delivery through
#185 / PR #212. #183 already retained interactive semantic/runtime evidence; #185
adds transactional stable-ID insert, move, remove, and multi-operation validation.
This todo becomes the next Authoring frontier as #186 after PR #212 merges.

## Acceptance criteria after unblocking

- Skills teach intent-level AuthoringSpec, not runtime-object bookkeeping.
- The model receives task-specific schema slices and relevant source-map context.
- Repairs target the smallest failed authored concept rather than regenerating the document.
