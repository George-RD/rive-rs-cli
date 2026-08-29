# Roadmap

This roadmap sequences development. Cairn todos under `meta/todos/` hold the
status and acceptance criteria; accepted decisions under `meta/decisions/` govern
architecture. GitHub issues are the execution surface; blocker relations determine
the implementation frontier.

## Product direction

`SceneSpec` remains the canonical explicit IR and expert escape hatch. It is not
the long-term AI authoring interface for complex files. Complex AI generation now
targets the parametric, component-based, view-model-first `AuthoringSpec`, which
compiles to SceneSpec and inherits the existing validation, encoding, rendering,
parity, runtime, and semantic-evidence loop.

Specialized low-level SceneSpec guidance remains bounded to expert escape-hatch
work. Complex generation guidance must stay aligned to the live AuthoringSpec
schema, stable authored identity, source maps, and incremental operation seam.

## Current implementation frontier

Authoring delivery spec: [#175](https://github.com/George-RD/rive-rs-cli/issues/175).
Public proof delivery spec: [#198](https://github.com/George-RD/rive-rs-cli/issues/198).

[#199](https://github.com/George-RD/rive-rs-cli/issues/199) completed frame-locked
Verification Lab playback in PR #216: each parity pair waits for both Rive runtimes,
then shares deterministic play/pause and representative-frame seeking through one
reusable site playback seam. Backward state-machine seeks rebuild and replay from a
fixed logical clock instead of carrying later state backward.

[#200](https://github.com/George-RD/rive-rs-cli/issues/200) completes the dedicated,
manifest-driven original-work showcase in PR #218. It reuses the #199 playback seam,
keeps AuthoringSpec and SceneSpec provenance inspectable, stages local evidence, and
promotes the complex animated AuthoringSpec behind a public-CLI drift guard.

**Current next item: [#201 — Lead the public site with original and production proof](https://github.com/George-RD/rive-rs-cli/issues/201).**
It follows #200 on the independent public-proof track: replace the landing proof with
original work and add retained Horaxon production provenance without changing the
completed Authoring dependency graph.

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
completed typed insert, move, remove, and multi-operation transactions in PR #212.
[#186](https://github.com/George-RD/rive-rs-cli/issues/186) completes complex AI
generation through AuthoringSpec in PR #215, including Authoring-first prompts,
incremental repair, source-mapped eval evidence, MCP exposure, and skill governance.

The ordered Authoring execution graph is:

```text
#176 one-pass motion compiler (complete in PR #192)
  -> #177 shared SceneSpec compilation seam (complete in PR #195)
  -> #174 first-class AuthoringSpec CLI (complete in PR #196)
  -> #178 complex animated AuthoringSpec exit gate (complete in PR #197)
       |-> #179 typed statechart tracer bullet (complete in PR #203) -> #180 inputs/events (complete in PR #204) -> #181 interactive showcase (complete in PR #206)
       |-> #182 static + animated semantic evaluations (complete in PR #207)

#181 + #182 -> #183 interactive semantic evaluation (complete in PR #209)
#181        -> #184 atomic replace (complete in PR #210) -> #185 insert/move/remove (complete in PR #212)
#183 + #185 -> #186 complex AI generation through AuthoringSpec (complete in PR #215)
```

The behavior and semantic-evaluation branches were intentionally independent where
the dependency graph allowed it. Do not invent dependencies merely to make future
work linear. The Verification Lab/public-proof track remains independent of the
completed Authoring chain unless a measured runtime or parity defect creates a real
cross-track blocker.

Issues #123-#128 remain independent lower-level correctness/coverage work. They
pre-empt selected roadmap work only when current parity/runtime evidence shows a
supported output depends on them, or when a bounded correctness audit is
deliberately selected. Broad 104-type coverage is not an implicit prerequisite for
AuthoringSpec or Verification Lab progress.

## Priority order

| Priority | Work | Status | Exit gate |
|---|---|---|---|
| P0 | Foundation refactor, Cairn map, roadmap, and deterministic CI | complete in PR #133 | all Rust, browser, visual, and Cairn gates green; merged to `main` |
| P0 | Official-runtime evidence in `ai lab` | complete in PR #134 | per-case frames and runtime pass rate retained separately from structural validity |
| P0 | [AuthoringSpec v0 and lowering boundary](meta/todos/todo.authoring-spec-v0.md) | complete in PR #135 | strict schema, deterministic lowering, source map, two validated examples |
| P0 | [Repair duplicate-reference correctness defect](meta/todos/todo.repair-engine-modularization.md#immediate-correctness-gate) | complete in PR #166 | duplicate-name repair cannot silently retarget an existing reference |
| P1 | [Visual/component compiler slice](meta/todos/todo.visual-authoring-compiler.md) | complete in PR #156 | components, parameters, patterns, simple constraints, complex static showcase |
| P1 | [Authoring delivery path](meta/todos/todo.authoring-delivery-path.md) | complete in PR #197 | one-pass compiler, shared compile seam, first-class Authoring CLI, complex animated runtime proof |
| P1 | [Public verification and original-work proof](meta/todos/todo.verification-lab-public-proof.md) | open; #199 complete in PR #216; #200 completes in PR #218; #201 next | frame-locked parity proof, manifest-driven original showcase, then landing/production proof with separate provenance |
| P2 | [Pose and motion compiler slice](meta/todos/todo.motion-authoring-compiler.md) | complete in PR #197 | compact tracks and poses reproduce complex motion with retained official-runtime proof through one compiler-owned scene draft |
| P2 | [Behavior and statechart compiler slice](meta/todos/todo.behavior-authoring-compiler.md) | open; #179 complete in PR #203; #180 complete in PR #204; #181 complete in PR #206 | supported typed behavior reproduces a complex interactive showcase exactly; broader power-user constructs remain open |
| P2 | [Semantic prompt evaluations](meta/todos/todo.semantic-prompt-evals.md) | complete in PR #209 | static, animated, and interactive semantic evidence are independently gated from structural/runtime results |
| P3 | [Incremental typed authoring operations](meta/todos/todo.incremental-authoring-operations.md) | complete in PR #212 | stable-ID edits validate atomically and preserve unaffected source-map identity |
| P3 | [AI generation skills](meta/todos/todo.ai-generation-skills.md) | complete in PR #215 | complex prompts target AuthoringSpec; task-focused schema/source-map context and stable-ID repair are integrated with eval evidence |
| P4 | [Repair-engine modularization](meta/todos/todo.repair-engine-modularization.md) | open; defer until a measured lower-level repair need justifies it | characterization-preserving split aligned to authored source maps |

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
  source-map identity rules.
- #186 completes the Authoring AI-generation frontier in PR #215: prompts target the
  current AuthoringSpec schema, choose a task-focused schema slice/showcase/source-map
  context, apply one stable-ID repair operation at a time through #185, retain the
  existing semantic/runtime evidence in prompt evals, and expose AuthoringSpec through
  skills and MCP without displacing SceneSpec as the expert lower IR.
- #198 is the independent public-proof delivery spec. #199 completed its first slice
  in PR #216 by replacing per-canvas autoplay with one deterministic paired playback
  seam while keeping `parity/results.json` authoritative for measured evidence.
- #200 completes the separate provenance-aware original-work showcase in PR #218,
  including manifest-driven staging and a public-CLI drift-guarded AuthoringSpec
  artifact. #201 is the next slice and moves the landing page to original/production
  proof without adding a dependency into the completed Authoring execution graph.

## Delivered readiness gate for complex AI generation

The gate required all of the following and is satisfied by #174-#186:

1. An agent authors stable IDs, components, parameters, poses, motion, bindings,
   and named statecharts without handling runtime indices or containment objects.
2. The compiler lowers deterministically to SceneSpec and returns source-mapped
   errors against authored paths.
3. At least one complex static, one complex animated, and one interactive showcase
   compile through the frontend without raw escapes for the supported subset.
4. Structural, official-runtime, semantic, and drift eval dimensions all pass and
   retain inspectable evidence.
5. Incremental authoring operations validate after each change, so generated
   AuthoringSpec repair can be local rather than whole-document regeneration.

## Continuing lower-level work

Binary correctness, Rive type coverage, parity, fuzzing, and runtime compatibility
continue when they unblock the roadmap or close evidenced defects. Broad object
coverage and specialized low-level skills are not substitutes for the authoring
abstraction.

Issue #122's old process-hardening programme and #129's raw-SceneSpec AI prompt
expansion are closed as superseded. Their valid concrete correctness concerns
remain represented by focused issues and the current Cairn/TDD/evidence workflow.
