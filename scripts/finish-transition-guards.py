from pathlib import Path


def replace_once(path, old, new):
    file = Path(path)
    text = file.read_text()
    assert text.count(old) == 1, (path, old, text.count(old))
    file.write_text(text.replace(old, new, 1))


section = '''## Combined transition conditions

Keep a single condition in `when`, or require several conditions together with
`when.all`. For example, this transition requires both an armed input and a load
of at least 60:

```json
{
  "id": "engage",
  "from": "resting",
  "to": "engaged",
  "when": {
    "all": [
      { "input": "armed", "equals": true },
      {
        "input": "load",
        "compare": "greater_or_equal",
        "value": { "kind": "literal", "value": 60, "unit": "scalar" }
      }
    ]
  }
}
```

An `all` group contains 1–1000 of the existing five condition forms. Conditions
are emitted in authored order and must all hold in the same runtime update.
Boolean and numeric model bindings, explicit inputs, and triggers can be mixed.
A trigger is momentary: a trigger fired while another condition is false is not
queued until that condition becomes true. Fire it again when the other conditions
are satisfied.

The group is flat. Nested groups, `any`, negation, and an `all` object containing
additional condition fields are rejected. An empty or oversized group reports
`invalid_behavior_collection_count` at `.when.all` through both the JSON and typed
Rust entry points. Leaf errors retain their existing codes at indexed paths such
as `$.behavior.statecharts[0].transitions[0].when.all[1].input`; numeric expression
errors can point further into `.value`. Regions use the corresponding
`.regions[i].transitions[j].when.all[k]` paths.

Existing single-condition documents keep their SceneSpec, source-map and binary
output. Wrapping one condition in `all` gives the same output. Groups do not add
runtime inputs beyond the bindings already required by their leaves; bindings are
shared with other transitions and blend consumers in the same chart. Model leaves
still read the bound model instance, not a mirrored machine input.

`duration_ms` and `exit_time_ms` work independently of the guard. The exit gate
cannot bypass a false member of `all`. The public CLI and official-runtime
contracts retain mixed model/input/trigger truth cases for root and region
transitions, blocked-trigger/re-fire evidence, and timed frames in the
`typed-behavior-runtime-evidence` artifact under `transition-guards`.

'''
replace_once('docs/authoring-spec-v0.md', '## Numeric view-model bindings\n', section + '## Numeric view-model bindings\n')

summary = '''[#247](https://github.com/George-RD/rive-rs-cli/issues/247), implemented in PR #248,
adds flat `when.all` transition guards through the existing condition lowerer.
Groups combine named input, model-binding and trigger conditions without raw
SceneSpec or extra states. Single-condition output stays unchanged; root charts
and parallel regions share bounds, authored diagnostics, timing, binding reuse and
atomic operations. Public contracts and retained runtime truth cases cover the
slice. This is part of the open behavior todo, not a new architecture track.

'''
replace_once('ROADMAP.md', 'Select further work from an explicit unblocked issue', summary + 'Select further work from an explicit unblocked issue')

path = Path('meta/todos/todo.behavior-authoring-compiler.md')
text = path.read_text()
assert '## Conjunctive transition guards (#247)' not in text
path.write_text(text.rstrip() + '''

## Conjunctive transition guards (#247)

PR #248 adds `when: {"all": [...]}` as a flat group of 1–1000 existing leaf
conditions. All members must hold together. A distinct guard wrapper preserves the
five leaf types and prevents recursive groups in the schema and deserializer.
Both JSON and typed Rust lowering enforce the collection bounds before compiler
work. Existing single-condition SceneSpec, source maps and binaries stay unchanged.

The existing root/region lowerer iterates the guard's conditions in authored order.
Reference and expression diagnostics retain indexed authored paths; used bindings
are collected across every leaf and shared with blend consumers. No canonical
builder/encoder change, second lowering pass, new runtime type or host-side input
mirroring is added. Duration and exit-time gates remain independent of conditions.

`tests/authoring_transition_guard_contract.rs` covers mixed order, five native
encoded condition kinds, single-leaf byte equivalence, region binding collection
and reuse, JSON/typed bounds, schema bounds, strict group forms, indexed reference
and parameter errors, timing and operation-batch rollback.
`tests/playwright/authoring-transition-guard-runtime.js` compiles retained sources
through the public CLI. Root and region truth cases require each of the five
condition kinds, prove blocked triggers are not replayed, and separate model
mutation from synthesized input mutation. CLI render cases prove each false input
still blocks after exit time and the complete guard respects both exit time and
blend duration. Sources, compile reports, binaries, PNGs and hashes are retained
under `target/playwright-behavior/transition-guards` by the existing artifact.

TDD red/green evidence is retained in development run `34370679285`, job
`102530793770`: the public tracer failed with `invalid_json` before the wrapper was
introduced, then passed after implementation and schema regeneration. Compiler
implementation commit: `cb08ccd703c42d46492b195aa37b884505993c85`. The temporary
branch-scoped development runner removed itself and its patch script from the
resulting tree. Final exact-head checks and separate Standards/Spec self-review
are recorded on PR #248. Local Rust execution was unavailable in this chat.

The parent todo remains open for the other behavior capability gaps.
''')

replace_once('.github/workflows/ci.yml',
    '          node tests/playwright/authoring-number-binding-runtime.js\n',
    '          node tests/playwright/authoring-number-binding-runtime.js\n          node tests/playwright/authoring-transition-guard-runtime.js\n')
