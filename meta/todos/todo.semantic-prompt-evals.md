---
node: rive-cli.intelligence.ai
status: complete
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

## Coverage

- Static AuthoringSpec semantics: complete in PR #207. Checked-in AuthoringSpec fixtures lower deterministically, retain authored source-map identity, and assert authored object presence/runtime type separately from structural validity.
- Animated AuthoringSpec semantics: complete in PR #207. Official-runtime frames are retained and deterministic frame-pair differences are reported/gated separately from runtime pass/fail.
- Interactive AuthoringSpec semantics: complete in PR #209. Curated cases drive authored inputs and pointer events through the official runtime, retain resolved interaction evidence, assert state-to-motion and transition wiring from authored identity, and gate visible frame response separately from runtime validity.
- Failure attribution: PRs #207 and #209 distinguish AuthoringSpec schema, lowering, structural, runtime, and semantic-mismatch stages while retaining AuthoringSpec diagnostics, lowered scene/source-map evidence, runtime interaction evidence, and rendered outputs.

## Dependencies

Runtime evidence and stable authored source maps are complete. Static/animated coverage completed in #182 / PR #207; interactive coverage completed in #183 / PR #209 on the same evidence model.
