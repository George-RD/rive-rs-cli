from pathlib import Path

p = Path('docs/authoring-spec-v0.md')
s = p.read_text()
s = s.replace('its own number input or numeric model binding instead of', 'its own number input, numeric model binding or fixed scalar weight instead of')
s = s.replace('Static expression weights and additive states remain outside the typed subset.', 'Fixed expression weights are described below; additive states remain outside the typed subset.')
s = s.replace('Both source\nfields, neither field, null sources and unknown fields are rejected.', 'A child selects exactly one of `input`, `binding` and `weight`. Multiple source\nfields, missing sources, null sources and unknown fields are rejected.')
if '### Fixed direct weights' not in s:
    s += '''
### Fixed direct weights

Use `weight` for a contribution that is known at compile time. A child selects
exactly one of `input`, `binding` and `weight`; runtime indices remain unavailable.

```json
{
  "id": "blending",
  "direct_blend": {
    "motions": [
      { "motion": "rest-track", "weight": { "kind": "literal", "value": 100, "unit": "scalar" } },
      { "motion": "left-track", "weight": { "kind": "parameter", "name": "contribution" } },
      { "motion": "right-track", "input": "right-weight" }
    ]
  }
}
```

Declare `contribution` in document `parameters`, for example
`{"value": 25.5, "unit": "scalar"}`. The normal scalar-expression operators are
supported. Fixed values must be finite, representable scene scalars between 0 and
100 inclusive; fractions are valid percentages. `invalid_blend_weight` reports a
range violation at the child's `.weight` before narrowing to the runtime float.
Unknown parameters, wrong units, division by zero and numeric representation
errors preserve their existing expression diagnostics and nested authored paths.
Unlike dynamic controls, invalid authored constants are rejected, not clamped.

The compiler emits a native constant without an input or binding object. Input,
model-bound and fixed contributions can be mixed, including in parallel regions.
They retain authored order and are not normalized: a full-weight rest motion
last can overwrite motion that earlier children applied. Changing a fixed value
requires recompilation; use `input` or `binding` for runtime changes.

`examples/authoring/fixed-blend-panel.v0.json` replaces the original panel's
artificial `foundation` input with a fixed 100-percent rest contribution. Its
left/right controls remain independent. Run the shared runtime harness with
`--fixed-weights` for this example plus parameterized zero, fractional, half,
full, reordered and reset/resume cases with no numeric inputs at all. Add
`--model-bound` to prove a fixed rest contribution composes with model-driven
weights without input mirroring. Sources, binaries, compile reports, measurements,
PNGs and hashes are retained under `target/playwright-behavior/fixed-blend` and
`model-fixed-blend`, alongside the unchanged existing runtime modes.
'''
p.write_text(s)

p = Path('meta/contracts/authoring.md')
s = p.read_text().replace('Static\nweight expressions and additive states are not exposed.', 'Fixed weight expressions are exposed by #245 below; additive states are not exposed.')
s = s.replace('Each direct-blend child selects exactly one of `{motion, input}` and\n`{motion, binding}`.', 'Each direct-blend child selects exactly one of `{motion, input}`,\n`{motion, binding}` and, after #245, `{motion, weight}`.')
if '## Fixed direct-blend weights (#245)' not in s:
    s += '''
## Fixed direct-blend weights (#245)

`{motion, weight}` accepts a scalar expression evaluated against document
parameters. The result must be finite and representable in the runtime float,
within 0..=100 percent; fractions are valid. Out-of-range constants report
`invalid_blend_weight` at `.weight` before float narrowing. Existing expression
errors retain their specific codes and authored paths. The typed union and
published schema reject multiple sources, null, missing and runtime-only fields.

Root and parallel-region lowering emit `blend_source: 1` and `mix_value` through
the existing canonical direct-blend child. Fixed weights allocate no inputs or
model contexts. Mixed sources retain authored child order and actual chart-local
input indices. Old input/model-driven JSON, binary output and source-map identities
remain unchanged. Atomic operations reuse these validations and roll back failure.

The public fixed-blend contracts cover encoding, scalar boundaries/arithmetic,
strict schema forms, diagnostic paths, non-finite programmatic parameters, regions,
mixed source offsets across charts, ordering, rollback and legacy model binaries.
The existing input-binary regression remains in the direct/model contracts.
The shared browser harness adds fixed/input and fixed/model modes without replacing
existing cases. Pure-constant cases contain only reset/resume trigger inputs and
must render zero, fractional, half and full weights, authored-order overwrites and
timed exit/re-entry. Evidence includes exact sources, binaries and rendered pixels.
'''
p.write_text(s)

p = Path('meta/todos/todo.behavior-authoring-compiler.md')
s = p.read_text()
if '## Fixed direct-blend weight slice (#245)' not in s:
    s += '''
## Fixed direct-blend weight slice (#245)

Adds exclusive `{motion, weight: <scalar expression>}` children through the shared
root/region compiler and existing canonical fixed-blend source. Finite scalar
percentages in 0..=100 include fractions and document parameter arithmetic.
Out-of-range values fail before float narrowing; existing expression errors retain
authored paths. Fixed children create no synthetic input or model binding object.
Mixed input/model/fixed sources preserve order and chart-local indices.

Eleven public contracts cover native encoding, schema exclusivity, boundaries,
expressions, non-finite typed parameters, regions, mixed source offsets, source-map
identity, atomic rollback and legacy model bytes. The original input-byte guard
remains unchanged. `fixed-blend-panel.v0.json` replaces the artificial foundation
input with a fixed rest weight; the shared browser harness also compiles no-numeric-
input parameter cases and proves fixed weights compose with model-driven controls.

[Fixed blend evidence](../research/fixed-blend-authoring.md) records the observed
red/green runs and runtime contract. Exact delivery-head CI/MSRV and separate
Standards/Spec self-review are recorded on the PR before merge. This parent remains
open for additive states, broader timing, other property kinds and conversions.
'''
p.write_text(s)

p = Path('ROADMAP.md')
s = p.read_text()
if '- #245 adds fixed scalar direct-blend weights' not in s:
    anchor = '## Delivered readiness gate for complex AI generation'
    assert s.count(anchor) == 1
    s = s.replace(anchor, '''- #245 adds fixed scalar direct-blend weights through the existing AuthoringCompiler
  and canonical runtime source. Constant contributions no longer require artificial
  number inputs. Root/region validation, mixed-source ordering, byte compatibility,
  atomic edits and public-CLI/bundled-runtime evidence remain part of the behavior
  frontier; its parent todo stays open for additive states and remaining model gaps.

''' + anchor)
p.write_text(s)

p = Path('meta/research/fixed-blend-authoring.md')
if not p.exists():
    p.write_text('''---
id: res.fixed-blend-authoring
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.verification.rust
  - rive-cli.verification.browser
date: 2026-09-09
method: primary
---

# Fixed direct-blend weights (#245)

## Gap and native semantics

Main `9835234e5f495b6fece43b7658c825abc20b63a6` contained merged PR #244 and no
open PRs. The behavior todo remained open; #240 had explicitly excluded static
weights. The direct panel's `foundation` number input existed only to hold a rest
motion at 100 percent. #245 closes that bounded authoring gap under #175, rather
than adding another roadmap track or a new binary type.

Primary source `rive-app/rive-runtime/include/rive/animation/blend_animation_direct.hpp`
defines `DirectBlendSource::mixValue = 1`, beside input source 0 and data-bind source
2. The canonical builder already preserves explicit fixed sources. The frontend
therefore emits existing `blend_source: 1` and `mix_value`, with no synthetic input,
model context, encoder change or second lowering pass. Each authored child selects
exactly one input, binding or scalar-expression weight. Constants are checked in
f64 before narrowing, preventing values just above 100 from rounding into validity.

## Observed test-first evidence

- Run `34356229933`, job `102481491328`, compiled the initial public contract and
  failed because `BehaviorDirectBlendMotionSpec` did not accept a fixed weight.
  Exported test-only source was `e28f476e04700c969c96fb33da5b71736b70bb77`.
- Run `34356566182` passed native lowering/encoding and no-synthetic-input checks
  after the minimal typed source and compiler implementation.
- Run `34356975522`, job `102484022169`, then compiled and failed the range contract:
  -0.001 incorrectly lowered to a fixed runtime value. The original native-object
  test still passed. The implementation adds one shared checked evaluator.
- Run `34357601961` passed all eleven expanded public contracts, including strict
  forms, parameters/fractions, expression diagnostics, scoped regions, mixed
  fixed/input/model offsets, authored ordering, rollback and legacy model bytes.

Local Cargo and outbound cloning were unavailable. Scoped Actions executed Rust and
retained the tested source and logs. Workflow trigger SHAs may precede the formatted
bot-published tested source; exported artifacts record that source explicitly.
Temporary workflow/patch files are removed from the delivered change.

## Runtime and merge gate

The shared direct-blend harness preserves its input, model and model-1D modes and
adds fixed/input and fixed/model modes. The checked example removes `foundation`;
pure-constant variants use scalar parameters and expose only two trigger inputs.
Pixel assertions cover zero, 25.5/75.5, half, full, reversed contributions, rest-last
ordering and timed reset/resume. Fixed/model checks retain input-only non-effects.
Sources, compile reports, binaries, PNGs, positions and JS/WASM hashes are retained.
Final exact-head CI, Rust 1.88 and the separate Standards/Spec self-review are
recorded on the delivery PR; structural lowering alone is not the runtime gate.
''')
