---
node: rive-cli.intelligence.ai
status: open
created: 2026-07-29
---

# P2 — Add semantic prompt-satisfaction evaluation

Measure whether generated output satisfies requested composition, motion, and
behavior independently from structural and runtime validity.

## Acceptance criteria

- Cases define explicit semantic expectations rather than a vague quality score.
- Deterministic checks cover inspectable facts such as object presence, motion between frames, and interaction response.
- Model-assisted visual judgement, when used, is recorded as a separate non-deterministic evidence source.
- Reports keep structural, runtime, semantic, and baseline-drift dimensions separate.

## Dependencies

Requires runtime evidence and stable authored source maps.
