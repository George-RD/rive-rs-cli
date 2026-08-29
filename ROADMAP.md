# Roadmap

This roadmap sequences development. Cairn todos under `meta/todos/` hold the
status and acceptance criteria; accepted decisions under `meta/decisions/` govern
architecture. GitHub issues are the execution surface; blocker relations determine
the implementation frontier.

## Product direction

`SceneSpec` remains the canonical explicit IR and expert escape hatch. It is not
the long-term AI authoring interface for complex files. The next product layer is
a parametric, component-based, view-model-first `AuthoringSpec` that compiles to
SceneSpec and inherits the existing validation, encoding, rendering, and parity
loop.

Specialized skills for generating complex Rive files are deliberately blocked
until that frontend and its evidence gates exist.

## Current implementation frontier

Parent delivery spec: [#175](https://github.com/George-RD/rive-rs-cli/issues/175).

**Current next item: [#180 — drive typed behavior from inputs and pointer events](https://github.com/George-RD/rive-rs-cli/issues/180).**
[#179](https://github.com/George-RD/rive-rs-cli/issues/179) completed the first typed
behavior tracer bullet in PR #202, including boolean view-model binding, named state
transitions, authored-path source identity, canonical-builder validation, and retained
official-runtime transition evidence. [#182](https://github.com/George-RD/rive-rs-cli/issues/182)
is independently unblocked and may proceed in parallel with the behavior branch.

The ordered execution graph is:

```text
#176 one-pass motion compiler (complete in PR #192)
  -> #177 shared SceneSpec compilation seam (complete in PR #195)
  -> #174 first-class AuthoringSpec CLI (complete in PR #196)
  -> #178 complex animated AuthoringSpec exit gate (complete in PR #197)
       |-> #179 typed statechart tracer bullet (complete in PR #202) -> #180 inputs/events -> #181 interactive showcase
       |-> #182 static + animated semantic evaluations

#181 + #182 -> #183 interactive semantic evaluation
#181        -> #184 atomic replace -> #185 insert/move/remove
#183 + #185 -> #186 complex AI generation through AuthoringSpec
```

After #178, the behavior and static/animated semantic-evaluation branches are
independent and may proceed in parallel. Do not invent dependencies merely to make
the graph linear.

Issues #123-#128 remain independent lower-level correctness/coverage work. They
pre-empt the Authoring frontier only when current parity/runtime evidence shows a
supported output depends on them, or when a bounded correctness audit is
deliberately selected. Broad 104-type coverage is not an implicit prerequisite for
AuthoringSpec progress.

## Priority order

| Priority | Work | Status | Exit gate |
|---|---|---|---|
| P0 | Foundation refactor, Cairn map, roadmap, and deterministic CI | complete in PR #133 | all Rust, browser, visual, and Cairn gates green; merged to `main` |
| P0 | Official-runtime evidence in `ai lab` | complete in PR #134 | per-case frames and runtime pass rate retained separately from structural validity |
| P0 | [AuthoringSpec v0 and lowering boundary](meta/todos/todo.authoring-spec-v0.md) | complete in PR #135 | strict schema, deterministic lowering, source map, two validated examples |
| P0 | [Repair duplicate-reference correctness defect](meta/todos/todo.repair-engine-modularization.md#immediate-correctness-gate) | complete in PR #166 | duplicate-name repair cannot silently retarget an existing reference |
| P1 | [Visual/component compiler slice](meta/todos/todo.visual-authoring-compiler.md) | complete in PR #156 | components, parameters, patterns, simple constraints, complex static showcase |
| P1 | [Authoring delivery path](meta/todos/todo.authoring-delivery-path.md) | complete in PR #197 | one-pass compiler, shared compile seam, first-class Authoring CLI, complex animated runtime proof |
| P2 | [Pose and motion compiler slice](meta/todos/todo.motion-authoring-compiler.md) | complete in PR #197 | compact tracks and poses reproduce complex motion with retained official-runtime proof through one compiler-owned scene draft |
| P2 | [Behavior and statechart compiler slice](meta/todos/todo.behavior-authoring-compiler.md) | open; #179 complete in PR #202; #180 is next, then #181 | view-model bindings and named statecharts reproduce interaction with runtime proof |
| P2 | [Semantic prompt evaluations](meta/todos/todo.semantic-prompt-evals.md) | open; #182 independently unblocked, #183 after #181 + #182 | semantic evidence reported separately from structural and runtime results |
| P3 | [Incremental typed authoring operations](meta/todos/todo.incremental-authoring-operations.md) | open; #184 -> #185 after #181 | stable-ID edits validate atomically and preserve unaffected source-map identity |
| P3 | [AI generation skills](meta/todos/todo.ai-generation-skills.md) | blocked; #186 requires #183 + #185 | frontend slices, complex showcase coverage, runtime eval, semantic eval, incremental operations |
| P4 | [Repair-engine modularization](meta/todos/todo.repair-engine-modularization.md) | open; defer until Authoring compiler state is stable | characterization-preserving split aligned to authored source maps |

## Delivery dependencies

- #176 completed the one-pass motion compiler architecture gate in PR #192.
- #177 completed one canonical `SceneSpec -> .riv` application seam in PR #195 so
  #174 could add AuthoringSpec input without copying the raw generate/build/encode
  flow.
- #174 completed the explicit AuthoringSpec CLI in PR #196 while preserving raw
  SceneSpec `generate` as the expert path.
- #178 completed the generic scattered -> overloaded -> connected -> next-action
  animated showcase and retained official-runtime gate in PR #197.
- #179 completed the first typed statechart tracer bullet in PR #202 on the same
  compiler-owned scene, source-map, runtime-name, and checked-binding state.
- #180 extends that slice with typed inputs, events, and listeners plus retained
  interaction evidence before #181 closes the complex interactive showcase gate.
- Static/animated semantic evaluation (#182) may run in parallel with behavior
  after #178; interactive semantic evaluation (#183) requires both branches.
- Incremental operations begin only after the typed behavior contract is stable
  enough to preserve unaffected authored identity.
- Complex AI-generation skills remain blocked until #183 and #185 close.

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

Issue #122's old process-hardening programme and #129's raw-SceneSpec AI prompt
expansion are closed as superseded. Their valid concrete correctness concerns
remain represented by focused issues and the current Cairn/TDD/evidence workflow.
