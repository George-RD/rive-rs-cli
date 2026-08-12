# Roadmap

This roadmap sequences development. Cairn todos under `meta/todos/` hold the
status and acceptance criteria; accepted decisions under `meta/decisions/` govern
architecture.

## Product direction

`SceneSpec` remains the canonical explicit IR and expert escape hatch. It is not
the long-term AI authoring interface for complex files. The next product layer is
a parametric, component-based, view-model-first `AuthoringSpec` that compiles to
SceneSpec and inherits the existing validation, encoding, rendering, and parity
loop.

Specialized skills for generating complex Rive files are deliberately blocked
until that frontend and its evidence gates exist.

## Priority order

| Priority | Work | Status | Exit gate |
|---|---|---|---|
| P0 | Foundation refactor, Cairn map, roadmap, and deterministic CI | complete in PR #133 | all Rust, browser, visual, and Cairn gates green; merged to `main` |
| P0 | Official-runtime evidence in `ai lab` | complete in PR #134 | per-case frames and runtime pass rate retained separately from structural validity |
| P0 | [AuthoringSpec v0 and lowering boundary](meta/todos/todo.authoring-spec-v0.md) | complete in PR #135 | strict schema, deterministic lowering, source map, two validated examples |
| P0 | [Repair duplicate-reference correctness defect](meta/todos/todo.repair-engine-modularization.md#immediate-correctness-gate) | complete in PR #166 | duplicate-name repair cannot silently retarget an existing reference |
| P1 | [Visual/component compiler slice](meta/todos/todo.visual-authoring-compiler.md) | complete in PR #156 | components, parameters, patterns, simple constraints, complex static showcase |
| P2 | [Pose and motion compiler slice](meta/todos/todo.motion-authoring-compiler.md) | in progress; typed poses, tracks, easings, opacity, and dimensions complete through PR #168; compiler characterization in PR #169, boundary in PR #170, and compiler-owned scene/source-map/runtime-name state in PR #171; checked compiler indexes and one-pass scene mutation remain | compact tracks and poses reproduce complex motion with runtime proof through one compiler-owned scene draft |
| P2 | [Behavior and statechart compiler slice](meta/todos/todo.behavior-authoring-compiler.md) | open; gated on the one-pass compiler state | view-model bindings and named statecharts reproduce interaction with runtime proof |
| P2 | [Semantic prompt evaluations](meta/todos/todo.semantic-prompt-evals.md) | open | semantic evidence reported separately from structural and runtime results |
| P3 | [Incremental typed authoring operations](meta/todos/todo.incremental-authoring-operations.md) | open | stable-ID edits validate atomically and preserve unaffected source-map identity |
| P3 | [AI generation skills](meta/todos/todo.ai-generation-skills.md) | blocked | frontend slices, complex showcase coverage, runtime eval, semantic eval, incremental operations |
| P4 | [Repair-engine modularization](meta/todos/todo.repair-engine-modularization.md) | open; defer until Authoring compiler state is stable | characterization-preserving split aligned to authored source maps |

## Delivery dependencies

- The next development session moves resolved symbols, checked runtime bindings, the motion-target index, and mutation-oriented source-map construction into `AuthoringCompiler`, then lowers typed motion directly into its existing scene draft and begins deleting the cloned second lower.
- Complete the one-pass motion compiler architecture gate before behavior/statechart lowering or broader Authoring feature expansion.
- Semantic evaluation can advance in parallel, but its animated and interactive exit cases require the motion and behavior showcases.
- Add incremental operations only after the typed visual, motion, and behavior contracts are stable enough to preserve unchanged authored identity.
- AI-generation skills remain blocked until the frontend, retained runtime and semantic evidence, and incremental operations are complete.

## Readiness gate for complex AI generation

Complex generation is ready for skill investment only when all of the following
are true:

1. An agent authors stable IDs, components, parameters, poses, motion, bindings,
   and named statecharts without handling runtime indices or containment objects.
2. The compiler lowers deterministically to SceneSpec and returns source-mapped
   errors against authored paths.
3. At least one complex static, one complex animated, and one interactive showcase
   compile through the frontend without raw escapes for the supported subset.
4. Structural, official-runtime, semantic, and drift eval dimensions all pass and
   retain inspectable evidence.
5. Incremental authoring operations can validate after each change, so repair is
   local rather than whole-document regeneration.

## Continuing lower-level work

Binary correctness, Rive type coverage, parity, fuzzing, and runtime compatibility
continue when they unblock the roadmap or close evidenced defects. Broad object
coverage and specialized skills are not substitutes for the authoring abstraction.
