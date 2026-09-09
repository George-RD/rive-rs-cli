from pathlib import Path
import json

root = Path('.')

def replace(path, old, new):
    p = root / path
    text = p.read_text()
    assert text.count(old) == 1, (path, old)
    p.write_text(text.replace(old, new))

scalar = lambda n: {'kind': 'literal', 'value': n, 'unit': 'scalar'}
px = lambda n: {'kind': 'literal', 'value': n, 'unit': 'px'}
def pose(id, target, x, y):
    return {'id': id, 'targets': [{'target': target, 'transform': {'x': px(x), 'y': px(y)}}]}
def track(id, pose):
    return {'id': id, 'fps': 60, 'duration_frames': scalar(120), 'keyframes': [{'frame': scalar(0), 'pose': pose}, {'frame': scalar(120), 'pose': pose}]}
def states(prefix):
    return [{'id': 'resting', 'motion': f'{prefix}-rest-track'}, {'id': 'engaged', 'motion': f'{prefix}-active-track'}]
def transition(delay):
    return {'id': 'advance', 'from': 'resting', 'to': 'engaged', 'when': 'always', 'exit_time_ms': scalar(delay), 'duration_ms': scalar(500)}
example = {'authoring_format_version': 0, 'artboard': {'id': 'automatic-sequence', 'width': {'value': 240, 'unit': 'px'}, 'height': {'value': 160, 'unit': 'px'}}, 'visual': {'nodes': []}, 'motion': {'poses': [], 'tracks': []}, 'behavior': {'statecharts': [{'id': 'sequence', 'initial': 'resting', 'states': states('upper'), 'transitions': [transition(500)], 'regions': [{'id': 'lower', 'initial': 'resting', 'states': states('lower'), 'transitions': [transition(1000)]}]}]}}
for prefix, y, color in [('upper', 48, '#246BFD'), ('lower', 112, '#24C890')]:
    example['visual']['nodes'].append({'kind': 'rectangle', 'id': prefix, 'width': px(24), 'height': px(24), 'fill': color, 'transform': {'x': px(40), 'y': px(y)}})
    for name, x in [('rest', 40), ('active', 200)]:
        example['motion']['poses'].append(pose(f'{prefix}-{name}', prefix, x, y))
        example['motion']['tracks'].append(track(f'{prefix}-{name}-track', f'{prefix}-{name}'))
(root / 'examples/authoring/automatic-sequence.v0.json').write_text(json.dumps(example, indent=2) + '\n')

auto = '''## Automatic transitions

Use `"when": "always"` to advance without an input, model binding or trigger. The
field stays required: missing `when`, `null`, unknown strings and `{"always": true}`
are rejected. `always` is a complete guard, not a leaf inside `all`; an empty `all`
remains invalid.

```json
{
  "id": "advance",
  "from": "resting",
  "to": "engaged",
  "when": "always",
  "exit_time_ms": { "kind": "literal", "value": 500, "unit": "scalar" },
  "duration_ms": { "kind": "literal", "value": 500, "unit": "scalar" }
}
```

This transition becomes eligible when the outgoing motion reaches 500ms, then
blends to the destination over 500ms. With no exit gate, `always` is eligible on
state entry; it does not wait for the motion to finish. Eligibility is evaluated
by the runtime on advance, not by a host timer. Keep the outgoing motion long
enough to reach its gate. Existing animation-only exit-source validation still
applies; `always` does not enable exit gates on blend states.

Root charts and parallel regions share this contract. The compiler emits zero
conditions and no extra inputs or bindings. Conditional and automatic transitions
can coexist, retaining authored order. Put an unconditional fallback after the
conditional alternatives it should not pre-empt. Avoid cycles of ungated `always`
transitions: this feature adds neither a scheduler nor a cycle guard.

`examples/authoring/automatic-sequence.v0.json` runs two input-free regions. The
upper panel begins after 500ms and the lower after 1000ms; both blend for 500ms.
`tests/playwright/authoring-automatic-transition-runtime.js` compiles through the
public CLI and retains source, binaries, compile reports, frames and measured
positions under `target/playwright-behavior/automatic-transitions`. It compares
resting, engaged, timed, instantaneous, ungated and explicit-zero-gate variants.

'''
replace('docs/authoring-spec-v0.md', '## Numeric view-model bindings\n', auto + '## Numeric view-model bindings\n')
replace('docs/authoring-spec-v0.md', 'Duration controls the blend after its condition is satisfied;', 'Duration controls the blend after its guard is satisfied;')
replace('docs/authoring-spec-v0.md', 'The `when` condition must also be satisfied.', 'The `when` guard must also be satisfied; `"always"` imposes no input or model condition.')
replace('cairn.blueprint', '                "./tests/authoring_behavior_blend_contract.rs",', '                "./tests/authoring_automatic_transition_contract.rs",\n                "./tests/authoring_behavior_blend_contract.rs",')
replace('.github/workflows/ci.yml', '          node tests/playwright/authoring-transition-guard-runtime.js\n', '          node tests/playwright/authoring-transition-guard-runtime.js\n          node tests/playwright/authoring-automatic-transition-runtime.js\n')
entry = '''[#249](https://github.com/George-RD/rive-rs-cli/issues/249) adds explicit
`when: "always"` transitions through the existing root/region guard lowerer.
Automatic transitions need no dummy inputs or model bindings, preserve named
source identity, and compose with the existing animation exit gate and independent
blend duration. The input-free two-region sequence and retained public-CLI runtime
comparisons cover timed, instantaneous, ungated and explicit-zero-gate behavior.
Missing/null guards remain invalid, and `all` still requires at least one leaf.
This is a bounded continuation of the open behavior todo, not a new scheduler.

'''
replace('ROADMAP.md', 'Select further work from an explicit unblocked issue', entry + 'Select further work from an explicit unblocked issue')
p = root / 'examples/authoring/README.md'
p.write_text(p.read_text() + '''\n## Automatic sequence

`automatic-sequence.v0.json` moves two panels without inputs, events, models or
host mutation. The upper panel starts after its motion reaches 500ms; the lower
parallel region starts at 1000ms. Each transition blends for another 500ms, then
holds the destination. Both use explicit `when: "always"` with named states.

```sh
cargo run -- authoring compile examples/authoring/automatic-sequence.v0.json -o /tmp/automatic-sequence.riv --json
node tests/playwright/authoring-automatic-transition-runtime.js
```

The runtime test retains six comparison variants and their rendered frames under
`target/playwright-behavior/automatic-transitions`. Removing `exit_time_ms` makes
an automatic transition immediately eligible; it does not wait for the motion's
end. Do not create cycles of ungated automatic transitions.
''')
p = root / 'meta/todos/todo.behavior-authoring-compiler.md'
p.write_text(p.read_text() + '''\n## Explicit automatic transitions (#249)

The required transition guard now accepts the exact literal `"always"` beside the
existing leaf and `all` forms. Root charts and parallel regions lower it to an
empty canonical condition array through the same compiler. No inputs, bindings,
new runtime objects or alternate lowering pass are introduced. Existing duration
and animation exit-time validation are unchanged; missing/null guards and empty
`all` groups remain invalid.

Public contracts cover native zero-condition encoding, typed/schema strictness,
source-map identity, root/region bounds and timing, shared bound guards, blend
exit-source rejection and atomic edits. `automatic-sequence.v0.json` proves two
input-free regions through the public CLI/bundled-runtime path, comparing resting,
engaged, timed, instantaneous, ungated and zero-gate output.

TDD: run `34378356290`, source `e2ee148c4310d3fd825f812387754c9b9853bf8e`,
compiled the tracer and failed with `invalid_json` for the unsupported guard. Run
`34378564691` passed that same tracer and the 29 existing guard/duration/exit-time
contracts on formatted source `072d2d91d99a20bf191b79650bb4bb38af3fe5ab`.
Final expanded runtime, exact-head CI and separate Standards/Spec self-review
are recorded on the pull request for #249. The parent behavior todo remains open.
''')
Path(__file__).unlink()
