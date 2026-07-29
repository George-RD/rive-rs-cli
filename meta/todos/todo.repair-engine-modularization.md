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

## Acceptance criteria

- Existing repair outputs and diagnostics remain compatible.
- Passes have explicit ownership and bounded inputs/outputs.
- Repair results map back to AuthoringSpec source paths when the frontend exists.
