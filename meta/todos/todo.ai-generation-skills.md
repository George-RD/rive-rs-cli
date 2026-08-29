---
node: rive-cli.intelligence.ai
status: done
created: 2026-07-29
unblocked: 2026-08-29
completed: 2026-08-29
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

All four readiness conditions were satisfied by the Authoring delivery through
#185 / PR #212. #183 already retained interactive semantic/runtime evidence; #185
added transactional stable-ID insert, move, remove, and multi-operation validation.

## Acceptance criteria after unblocking

- Skills teach intent-level AuthoringSpec, not runtime-object bookkeeping.
- The model receives task-specific schema slices and relevant source-map context.
- Repairs target the smallest failed authored concept rather than regenerating the document.

## Delivered in #186 / PR #214

- Prompt-based `ai generate` targets AuthoringSpec and lowers through the canonical
  Authoring frontend; built-in templates and direct SceneSpec generation remain the
  explicit lower-level escape path.
- OpenAI prompt context is generated from the current AuthoringSpec schema, narrowed
  to the task's static/animated/interactive top-level slice, paired with the closest
  checked-in showcase and its authored-to-runtime source-map evidence.
- Typed lowering failures enter a bounded repair loop that requests exactly one
  stable-ID operation at a time and applies it through the #185 atomic operation
  seam rather than regenerating the whole authored document.
- Prompt eval cases retain generated/repaired AuthoringSpec, repair attempts,
  lowered SceneSpec, source maps, runtime evidence, and the existing static,
  animated, and interactive semantic gates as separate evidence dimensions.
- MCP exposes the live `schema://authoring/v0` resource and a first-class
  `generate_authoring` compilation tool while keeping SceneSpec resources/tools as
  the expert IR surface.
- Agent guidance is Authoring-first and explicit about typed coverage boundaries;
  stale duplicate OpenCode generation/schema/anti-pattern skills that contradicted
  the authoritative resolver were removed, while the validation entry now routes
  back to the live CLI and canonical skill guidance.
