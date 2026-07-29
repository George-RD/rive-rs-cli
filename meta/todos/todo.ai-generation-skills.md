---
node: rive-cli.intelligence.ai
status: blocked
created: 2026-07-29
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

## Acceptance criteria after unblocking

- Skills teach intent-level AuthoringSpec, not runtime-object bookkeeping.
- The model receives task-specific schema slices and relevant source-map context.
- Repairs target the smallest failed authored concept rather than regenerating the document.
