---
node: rive-cli.intelligence.ai
status: in_progress
created: 2026-07-29
---

# P0 — Add official-runtime evidence to AI evaluations

Finish and merge PR #134 after the foundation PR lands. Record runtime loading,
required rendered frames, non-blank checks, distinct-colour evidence, and a
runtime pass-rate independently from structural validity.

## Acceptance criteria

- Offline template suites run without an external model API.
- Each runtime-enabled case retains its `.riv`, manifest, rendered frames, and failure reason.
- Runtime failures do not reduce or overwrite the structural-validity metric.
- CI executes the offline runtime contract and retains evidence on failure.
