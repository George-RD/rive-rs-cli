---
node: rive-cli.intelligence.authoring
status: in_progress
created: 2026-08-25
---

# P1 — Deepen the Authoring delivery path

Make the high-level AuthoringSpec path easy to consume without creating a second
compilation pipeline. This todo is the durable Cairn companion to GitHub spec #175
and covers the architecture/productization gate through the complex animated
showcase. Existing motion, behavior, semantic-evaluation, incremental-operation,
and AI-generation todos retain their own capability acceptance criteria.

## Acceptance criteria

- Typed motion and raw animation escapes share one compiler-owned canonical scene
  draft without cloning and lowering the full AuthoringSpec a second time.
- Raw SceneSpec generation and future AuthoringSpec application adapters share one
  canonical SceneSpec build + encode owner.
- AuthoringSpec can be compiled to `.riv` with one public CLI command while raw
  SceneSpec remains the explicit expert path.
- Authoring diagnostics remain source-mapped and machine-actionable in JSON mode.
- A generic complex animated AuthoringSpec showcase compiles, validates, and
  renders through the official runtime evidence loop.
- Execution order and blockers remain visible in ROADMAP.md and GitHub issues so a
  fresh agent can identify the implementation frontier without reconstructing
  repository history.

## Execution map

Parent spec: GitHub #175.

1. #176 — one-pass typed motion lowering in AuthoringCompiler, complete in PR #192.
2. #177 — canonical SceneSpec compilation seam used by raw `generate`, complete in
   PR #195.
3. #174 — first-class AuthoringSpec CLI input, complete in PR #196.
4. #178 — prove the complex animated AuthoringSpec exit gate. This is the current
   implementation frontier.

After #178, typed behavior (#179-#181) and static/animated semantic evaluation
(#182) may advance independently. Their downstream convergence, incremental edit
work, and AI-generation gate are recorded in ROADMAP.md and their existing Cairn
todos.

## Architecture constraints

- Preserve `AuthoringSpec -> SceneSpec -> canonical builder -> encoder`.
- Do not add a direct AuthoringSpec-to-binary path.
- Do not implement #174 by copying the existing generate/build/encode
  orchestration into another command handler; use the shared compilation seam from
  #177.
- Behavior must reuse the compiler state established by the one-pass motion gate.
- Broad lower-level object coverage remains independent unless evidence shows it
  blocks a supported Authoring output.

## Issue reconciliation

- #122 is closed as superseded by the current Cairn/TDD/exact-head/runtime evidence
  workflow; focused lower-level correctness issues remain open independently.
- #129 is closed as superseded because complex AI generation should target
  AuthoringSpec; #186 is the replacement future execution gate.
- #123-#128 remain evidence-driven lower-level work and are not implicit blockers
  for this todo.
