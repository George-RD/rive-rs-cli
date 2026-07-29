---
id: dec.ai-authoring-layer
nodes:
  - rive-cli.intelligence.ai
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
status: accepted
date: 2026-07-29
revisit_triggers:
  - "Direct SceneSpec generation matches the compiled frontend on complex-scene runtime and semantic evals"
  - "Rive exposes a stable higher-level authoring API that removes the need for this frontend"
  - "The frontend requires raw escapes for most supported showcase concepts"
informed_by:
  - res.ai-authoring-format
---

# Keep SceneSpec as the lowered IR and add an AI-facing authoring layer

## Decision

- Keep `SceneSpec` v1 as the canonical explicit representation consumed by the
  builder and encoder.
- Add a strict JSON `AuthoringSpec` frontend that captures design intent and
  lowers deterministically to SceneSpec.
- Separate visual, motion, and behavior concepts in the authored model while
  allowing the compiler to choose valid Rive mechanisms.
- Generate stable runtime names and a source map from authored IDs to expanded
  SceneSpec objects.
- Make behavior view-model-first and use named states and transitions at the
  authoring level.
- Preserve direct SceneSpec and raw-object escape hatches for experts and
  unsupported advanced types.
- Block investment in specialized complex-generation skills until the frontend,
  official-runtime evals, and semantic prompt evals satisfy the roadmap gates.

## Why

The current representation is optimized for deterministic compilation and Rive
coverage, not for model reliability. Asking a model to author it directly combines
layout, motion, behavior, graph identity, runtime containment, and index
bookkeeping in one output. A compiler can perform the mechanical work with higher
reliability and much lower token cost.

## Alternatives considered

- **Generate SceneSpec directly with larger skills.** Lowest implementation cost,
  but complexity and repair cost scale with every new Rive capability.
- **Adopt raw Lottie conventions.** Loses readable names and does not naturally
  express Rive's integrated data and behavior model.
- **Build a full CAD DSL and solver.** Potentially powerful, but premature and
  likely to introduce opaque constraint and identity failures.
- **Allow arbitrary scripts.** Compact but unsafe, difficult to validate, and not
  deterministic enough for a canonical AI interface.

## Trade-offs

The repository takes on a second schema and a lowering compiler. In return, the
runtime-facing schema stays stable, complex outputs become compositional and
repairable, and evaluation can identify whether a failure belongs to authoring,
lowering, encoding, runtime behavior, or semantic intent.
