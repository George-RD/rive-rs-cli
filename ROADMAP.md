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

**Current next item: [#186 — complex AI generation through AuthoringSpec](https://github.com/George-RD/rive-rs-cli/issues/186).**
[#179](https://github.com/George-RD/rive-rs-cli/issues/179) completed the first typed
behavior tracer bullet in PR #203. [#180](https://github.com/George-RD/rive-rs-cli/issues/180)
completed authored inputs, events, typed listeners, and retained `render --input` /
`render --pointer` evidence in PR #204. [#181](https://github.com/George-RD/rive-rs-cli/issues/181)
completed the complex interactive AuthoringSpec showcase gate in PR #206 with an
exact canonical SceneSpec equivalence contract. [#182](https://github.com/George-RD/rive-rs-cli/issues/182)
completed deterministic static and animated AuthoringSpec semantic evidence in PR #207.
[#183](https://github.com/George-RD/rive-rs-cli/issues/183) completed interactive semantic
evaluation in PR #209. [#184](https://github.com/George-RD/rive-rs-cli/issues/184)
completed atomic stable-ID visual replacement in PR #210. [#185](https://github.com/George-RD/rive-rs-cli/issues/185)
completed typed insert, move, remove, and multi-operation transactions in PR #212,
clearing the incremental-authoring blocker for #186.

The ordered execution graph is:

```text
#176 one-pass motion compiler (complete in PR #192)
  -> #177 shared SceneSpec compilation seam (complete in PR #195)
  -> #174 first-class AuthoringSpec CLI (complete in PR #196)
  -> #178 complex animated AuthoringSpec exit gate (complete in PR #197)
       |-> #179 typed statechart tracer bullet (complete in PR #203) -> #180 inputs/events (complete in PR #204) -> #181 interactive showcase (complete in PR #206)
       |-> #182 static + animated semantic evaluations (complete in PR #207)

#181 + #182 -> #183 interactive semantic evaluation (complete in PR #209)
#181        -> #184 atomic replace (complete in PR #210) -> #185 insert/move/remove (complete in PR #212)
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
| P2 | [Behavior and statechart compiler slice](meta/todos/todo.behavior-authoring-compiler.md) | open; #179 complete in PR #203; #180 complete in PR #204; #181 complete in PR #206 | supported typed behavior reproduces a complex interactive showcase exactly; broader power-user constructs remain open |
| P2 | [Semantic prompt evaluations](meta/todos/todo.semantic-prompt-evals.md) | complete in PR #209 | static, animated, and interactive semantic evidence are independently gated from structural/runtime results |
| P3 | [Incremental typed authoring operations](meta/todos/todo.incremental-authoring-operations.md) | complete in PR #212 | stable-ID edits validate atomically and preserve unaffected source-map identity |
| P3 | [AI generation skills](meta/todos/todo.ai-generation-skills.md) | ready; #186 requires complete #183 + #185 | frontend slices, complex showcase coverage, runtime eval, semantic eval, incremental operations |
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
- #179 completed the first typed statechart tracer bullet in PR #203 on the same
  compiler-owned scene, source-map, runtime-name, and checked-binding state.
- #180 completed typed boolean state-machine inputs, named events, typed listeners,
  input-driven transitions, authored-path diagnostics, and retained CLI interaction
  evidence in PR #204.
- #181 completed the complex interactive showcase gate in PR #206 with three typed
  states, four bidirectional transitions, two authored inputs, pointer/event listeners,
  a named reset event, exact whole-scene equivalence against a directly specified
  canonical state machine, and canonical-builder validation. It does not claim the
  broader behavior todo's blend-state or parallel-layer capability.
- #182 completed deterministic AuthoringSpec-first static and animated semantic
  evidence in PR #207. The report retains source-map/static assertions and official-
  runtime frame-difference assertions as separate dimensions from structural validity,
  runtime pass/fail, reproducibility, and drift.
- #183 completed interactive AuthoringSpec semantic evaluation in PR #209. The same
  evidence model now drives authored inputs and pointer events, retains resolved
  interaction evidence, checks authored state-motion/transition intent, and gates
  visible runtime response separately from structural and runtime validity.
- #184 completed the first stable-ID transactional operation in PR #210: root visual
  replacement is applied to a clone, validated through the normal AuthoringCompiler,
  and preserves unaffected source-map identity.
- #185 completed incremental authoring in PR #212 with typed insert, move, remove,
  and multi-operation transactions across visual, motion, and behavior concepts. Each
  step reuses canonical lowering, dependency validation, authored diagnostics, and
  source-map identity rules. With #183 and #185 complete, #186 is the next Authoring
  frontier.

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
