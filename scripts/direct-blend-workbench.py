from pathlib import Path
import json

marker = Path('meta/research/direct-blend-authoring.md')
if not marker.exists():
    lit = lambda v, u='scalar': {'kind': 'literal', 'value': v, 'unit': u}
    def target(id, x, y):
        return {'target': id, 'transform': {'x': lit(x, 'px'), 'y': lit(y, 'px')}}
    def track(id, pose):
        return {'id': id, 'fps': 60, 'duration_frames': lit(1), 'keyframes': [{'frame': lit(0), 'pose': pose}, {'frame': lit(1), 'pose': pose}]}
    doc = {'authoring_format_version': 0, 'artboard': {'id': 'direct-blend-panel', 'width': {'value': 240, 'unit': 'px'}, 'height': {'value': 160, 'unit': 'px'}}, 'visual': {'nodes': [
        {'kind': 'rectangle', 'id': 'left-panel', 'width': lit(24, 'px'), 'height': lit(24, 'px'), 'fill': '#246BFD', 'transform': {'x': lit(40, 'px'), 'y': lit(40, 'px')}},
        {'kind': 'rectangle', 'id': 'right-panel', 'width': lit(24, 'px'), 'height': lit(24, 'px'), 'fill': '#22C55E', 'transform': {'x': lit(40, 'px'), 'y': lit(120, 'px')}}
    ]}, 'motion': {'poses': [{'id': 'rest', 'targets': [target('left-panel', 40, 40), target('right-panel', 40, 120)]}, {'id': 'left-moved', 'targets': [target('left-panel', 200, 40)]}, {'id': 'right-moved', 'targets': [target('right-panel', 200, 120)]}], 'tracks': [track('rest-track', 'rest'), track('left-track', 'left-moved'), track('right-track', 'right-moved')]}, 'behavior': {'statecharts': [{'id': 'panel', 'inputs': [{'kind': 'number', 'id': 'foundation', 'value': lit(100)}, {'kind': 'number', 'id': 'left-weight', 'value': lit(0)}, {'kind': 'number', 'id': 'right-weight', 'value': lit(0)}, {'kind': 'trigger', 'id': 'reset'}, {'kind': 'trigger', 'id': 'resume'}], 'initial': 'blending', 'states': [{'id': 'blending', 'direct_blend': {'motions': [{'motion': 'rest-track', 'input': 'foundation'}, {'motion': 'left-track', 'input': 'left-weight'}, {'motion': 'right-track', 'input': 'right-weight'}]}}, {'id': 'resting', 'motion': 'rest-track'}], 'transitions': [{'id': 'reset-state', 'from': 'blending', 'to': 'resting', 'when': {'trigger': 'reset'}, 'duration_ms': lit(100)}, {'id': 'resume-state', 'from': 'resting', 'to': 'blending', 'when': {'trigger': 'resume'}, 'duration_ms': lit(100)}]}]}}
    Path('examples/authoring/direct-blend-panel.v0.json').write_text(json.dumps(doc, indent=2) + '\n')

    p = Path('cairn.blueprint')
    s = p.read_text()
    old = '                "./tests/authoring_distribute_contract.rs",'
    assert s.count(old) == 1
    p.write_text(s.replace(old, '                "./tests/authoring_direct_blend_contract.rs",\n' + old))
    p = Path('.github/workflows/ci.yml')
    s = p.read_text()
    old = '          node tests/playwright/authoring-number-binding-runtime.js'
    assert s.count(old) == 1
    p.write_text(s.replace(old, old + '\n          node tests/playwright/authoring-direct-blend-runtime.js'))

    p = Path('docs/authoring-spec-v0.md')
    s = p.read_text().replace('state `blend`, and statechart `regions`', 'state `blend` and `direct_blend`, and statechart `regions`').replace('absent `blend` and `regions`', 'absent `blend`, `direct_blend`, and `regions`').replace('named states that play one motion track or blend at least two', 'named states that play one motion track, use a one-dimensional blend, or mix independent motion weights').replace('A behavior state declares exactly one of `motion` and `blend`.', 'A behavior state declares exactly one of `motion`, `blend`, and `direct_blend`.').replace('A state with neither field fails with `missing_state_motion` and a state with both fails with `ambiguous_state_motion`', 'A state with no motion source fails with `missing_state_motion` and a state with multiple sources fails with `ambiguous_state_motion`').replace('Additive blend states, direct blend states, advanced exit timing,', 'Additive blend states, model-bound direct blends, advanced exit timing,')
    section = '''## Direct blend states

`direct_blend` gives each named motion its own local number input instead of selecting neighbouring stops from a shared input:

```json
{
  "id": "blending",
  "direct_blend": {
    "motions": [
      { "motion": "rest-track", "input": "foundation" },
      { "motion": "left-track", "input": "left-weight" },
      { "motion": "right-track", "input": "right-weight" }
    ]
  }
}
```

Each input is a percentage weight. The official runtime clamps values below 0 to no contribution and above 100 to full contribution. These are not relative weights normalized to a total. Children are applied in authored order, so later motions can modify properties already changed by earlier motions. The compiler preserves that order.

`examples/authoring/direct-blend-panel.v0.json` uses a first rest motion with its `foundation` number input initialized to 100. That reestablishes both panels' baseline before two independent inputs contribute motion. With that baseline, a weight of 50 places its panel halfway from x 40 to x 200; reversing either weight returns that panel towards rest without changing the other panel. This example does not claim that every arbitrary combination of overlapping tracks has the same midpoint semantics.

The collection contains 1 through 1000 children. `invalid_direct_blend_motions` reports an empty or oversized collection at `.direct_blend.motions`. Each `motion` must name a typed motion track; each `input` must name a number input declared in the same chart. Unknown references report `unknown_behavior_motion` or `unknown_behavior_input` at the child's `.motion` or `.input`; non-number inputs report `invalid_blend_input` at `.input`. Binding IDs and raw-animation IDs are not substitutes for these references. Unknown fields, including runtime indices, are rejected.

The same form works in parallel regions. The compiler resolves animation indices from its existing lowered scene and input indices from the emitted chart inputs, including offsets from binding-generated inputs. It lowers to existing `blend_state_direct` and `blend_animation_direct` objects without a new encoder path. Source-map identities remain attached to named states and regions. Transitions may enter or leave direct states and use `duration_ms`; `exit_time_ms` on a direct-blend source is rejected with `unsupported_transition_exit_source`, just as for a one-dimensional blend.

The runtime contract `tests/playwright/authoring-direct-blend-runtime.js` compiles the source through the public CLI and retains the source, binary, compile report, measured positions, PNGs and hashes. Cases cover independent 0/50/100 inputs, runtime clamping, reversal, leaving the direct state, resuming it and returning both inputs to zero. Static expression weights, additive states and model-bound direct blends remain outside this slice. Omission or null leaves existing canonical output and bytes unchanged; the authoring format stays at version 0.

'''
    assert s.count('## Parallel regions') == 1
    p.write_text(s.replace('## Parallel regions', section + '## Parallel regions'))

    p = Path('ROADMAP.md')
    s = p.read_text().replace('Additive and direct blend states, advanced exit timing,', 'Additive blend states, model-bound direct blends, advanced exit timing,').replace('additive and direct blend states, advanced exit timing,', 'additive blend states, model-bound direct blends, advanced exit timing,').replace('numeric model bindings added in #237 / PR #238 |', 'numeric model bindings added in #237 / PR #238; input-driven direct blends added in #239 |')
    note = '''[#239](https://github.com/George-RD/rive-rs-cli/issues/239) adds input-driven
`direct_blend` states over named motions. Root charts and parallel regions resolve
actual emitted animation/input indices, retain authored order, and reuse the existing
canonical builder. The direct-blend panel example and retained runtime contract cover
independent weights, partial/full contribution, clamping, reversal and state transitions.
The broader behavior todo stays open for its remaining capabilities.

'''
    index = s.index('## ', s.index('[#227]') + 1)
    p.write_text(s[:index] + note + s[index:])

    p = Path('meta/contracts/authoring.md')
    s = p.read_text().replace('Additive blend states, direct blend states, advanced exit timing,', 'Additive blend states, model-bound direct blends, advanced exit timing,').replace('`waypoint`, `blend`, `regions`', '`waypoint`, `blend`, `direct_blend`, `regions`')
    p.write_text(s + '''
## Input-driven direct blends (#239)

A state chooses exactly one of `motion`, `blend`, and `direct_blend`. Direct blends
contain 1..=1000 ordered `{motion, input}` children naming typed motion tracks and
chart-local number inputs. Canonical animation indices come from the existing
lowered scene; input indices include any preceding binding-generated inputs.
Root charts and regions share this lowering. The canonical builder and encoder
remain the only binary path. Source-map state identities and omitted-field output
remain stable. Invalid references, kinds, counts and competing sources are rejected
at authored paths. Removing a referenced motion is atomic.

Runtime weights are clamped percentages and applied sequentially, not normalized
relative shares. The panel example deliberately puts a full-weight rest motion
first, followed by separate motion contributions. The browser contract checks
partial/full/zero weights, independence, reversal, clamping and state transitions.
Duration is supported; exit-time gates on direct sources are rejected. Model-bound
weights, static weight expressions and additive states are not exposed.
''')
    p = Path('examples/authoring/README.md')
    p.write_text(p.read_text() + '''
## Direct blend panel

`direct-blend-panel.v0.json` applies two independently weighted panel motions in one
state. Keep `foundation` at its initial 100 to apply the rest motion first, then set
`left-weight` and `right-weight` independently from 0 to 100. Rive applies ordered
percentage contributions; the compiler does not normalize the inputs. `reset` exits
to the rest state and `resume` returns to the blend, with 100ms transition duration.

```sh
cargo run -- authoring compile examples/authoring/direct-blend-panel.v0.json -o /tmp/direct-blend-panel.riv --json
node tests/playwright/authoring-direct-blend-runtime.js
```

The browser test retains its source, binary, compile report, frames, positions and
hashes under `target/playwright-behavior/direct-blend`. Public Rust contracts cover
canonical indices, binding-generated offsets, regions, authored diagnostics, strict
schema, deterministic bytes and rejected incremental edits.
''')
    p = Path('meta/todos/todo.behavior-authoring-compiler.md')
    p.write_text(p.read_text() + '''
## Input-driven direct blend slice (#239)

Named `direct_blend` states now lower ordered `{motion, input}` children through
the existing canonical direct-blend objects. Inputs are chart-local numbers, not
bindings. Animation indices come from the compiler's actual lowered scene; input
indices include binding-generated inputs. Regions, state source maps and transition
durations share the same compiler path. Direct-source exit gates are rejected.

The new public contract covers deterministic compilation, schema/cardinality,
raw-motion composition, binding input offsets, region reuse, invalid references and
types, competing motion sources, old-output compatibility and atomic motion removal.
The public CLI/runtime contract retains independent-weight and state-transition
evidence for `direct-blend-panel.v0.json`.

[Direct blend evidence](../research/direct-blend-authoring.md) records observed
red/green revisions and runtime proof. The parent remains open for additive states,
model-bound direct blends, advanced timing, other property kinds and conversions.
''')
    marker.write_text('''---
id: res.direct-blend-authoring
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
  - rive-cli.verification.rust
  - rive-cli.verification.browser
date: 2026-09-08
method: primary
---

# Input-driven direct blends (#239)

## Reconciliation and scope

Main `63032288ff67181d3c23697b52d8bd3bd07df9bf` had no open pull requests. The
ordered AuthoringSpec/public-proof issues were complete. The parent behavior todo
explicitly retained direct blends, so #239 is a bounded continuation of #175 rather
than an independent roadmap programme.

A state accepts `direct_blend: {motions: [{motion, input}]}` as an alternative to
`motion` or the existing one-dimensional `blend`. Only chart-local number inputs
and typed motion tracks are exposed. Existing canonical direct-blend objects own
runtime semantics; actual emitted animation/input arrays own numeric addressing.
Authored order is retained. No new binary encoder or host-side simulation is added.

## Observed red/green

- Test-only head `5e3641a8d22c2c7162b85392f9ac7c1fb6a20469`, Actions run
  `34258204739`, compiled then failed because `direct_blend` was unknown.
- Run `34258438290` generated source head
  `8283b4a6b0dbd2aaec1b44963ff2ff20c2a6c85a`; the unchanged lowering/index/source-map
  test passed, and all-target/all-feature Clippy passed.

Local network cloning and Cargo execution were unavailable. Source editing was
performed through the connected GitHub API, with branch-scoped temporary Actions
for Rust execution, formatting and generated schema. Downloaded source snapshots
were used for local inspection and review. Temporary workbench files are removed
before the final merge head. Final exact-head checks and separate Standards/Spec
self-review are recorded on the delivery pull request.

## Runtime contract

`examples/authoring/direct-blend-panel.v0.json` supplies a full-weight baseline
motion first, then separate left/right motion contributions. The public CLI compiles
the source; the bundled official runtime receives the named number and trigger
inputs. Pixel positions, not host-side animation calculations, determine success.

`tests/playwright/authoring-direct-blend-runtime.js` retains source, binary, compile
report, representative PNGs, measured positions and hashes in
`target/playwright-behavior/direct-blend`. It checks zero/partial/full weights,
independence, clamping, reversal, leaving/resuming the state and return to zero.
Runtime observation is pending until the recorded delivery check completes.

## Remaining scope

The parent behavior todo stays open. Static weight expressions, additive states,
model-bound direct blends, advanced exit timing, other property kinds and binding
conversions are not implemented by this slice.
''')
