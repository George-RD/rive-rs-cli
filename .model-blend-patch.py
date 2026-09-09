from pathlib import Path

p = Path('tests/authoring_model_blend_1d_contract.rs')
if 'fn input_only_blends_retain_the_pre_binding_binary' not in p.read_text():
    p.write_text(p.read_text() + '''
#[test]
fn input_only_blends_retain_the_pre_binding_binary() {
    use sha2::{Digest, Sha256};
    let input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/blend-meter.v0.json"
    )).expect("input-driven example");
    let bytes = compile(&lower(&input).scene);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "9d03090b542d1e9e11364524b7e49d9f8517edcc89c845036dd01fe6f2bdf15d"
    );
}
''')

p = Path('src/builder/state_machines.rs')
p.write_text(p.read_text().replace('append_bound_number_input', 'append_bound_number_source'))

p = Path('docs/authoring-spec-v0.md')
p.write_text(p.read_text().replace(
    'Both checks run on the typed Rust path as well as through the published JSON schema.',
    'Both checks run on the typed Rust path. The published JSON schema also records the stop-count bounds; ordering is checked by the compiler, not JSON Schema.'
))

p = Path('tests/playwright/authoring-direct-blend-runtime.js')
text = p.read_text()
old = '''    console.log(`${MODE}: independent weights clamped, reversed, exited and resumed${MODEL_BOUND ? "; model changes, not synthesized inputs, controlled the result" : ""}`);'''
new = '''    const behavior = ONE_DIMENSIONAL ? "numeric stops interpolated, clamped, reversed, exited and resumed" : "independent weights clamped, reversed, exited and resumed";
    console.log(`${MODE}: ${behavior}${MODEL_BOUND ? "; model changes, not synthesized inputs, controlled the result" : ""}`);'''
if new not in text:
    assert text.count(old) == 1
    p.write_text(text.replace(old, new))

p = Path('meta/research/model-blend-1d-authoring.md')
if '## Observed full-workbench result' not in p.read_text():
    p.write_text(p.read_text() + '''
## Observed full-workbench result

Run `34308249467`, job `102329341322`, on exported source
`07f0d2b42162ddd438309155936a1ad980c74526` passed all 1,113 then-present Rust tests
(including the first 12 new contracts; one pre-existing schema helper ignored),
Clippy, Cairn scan/lint, all three blend runtime modes and the pinned input-byte
comparison. Scan/lint reported zero errors and the existing 97 warnings/7 info.
Artifact `10087518990` has SHA-256
`32eb622a81c9bc8635ac8156913329e8ead3e9208992ee33ef663f89dc9337b7`.
This precedes the source-validation refactor, descriptive example rename and final
compatibility regression; it is not the final exact-head merge gate.

The baseline input example compiled from main
`74a88084b456318a1192e762751e094766c3e67a` and the changed source produced identical
bytes, SHA-256 `9d03090b542d1e9e11364524b7e49d9f8517edcc89c845036dd01fe6f2bdf15d`.
That independently measured baseline is pinned by the thirteenth public contract.

The bundled runtime JS SHA-256 was
`9bfa2546433e72e7fb6e1cb63d863febe24563ba8644ddbb095518f2ef29b4a7`, WASM
`0e018bfd0826a276c4fbefae4d3dd0fe1be127eed10bedce4268f3890e54d47b`, with browser
`151.0.7922.34`. At model values 25/75, the measured centers were x=93/173; reversing
values reversed the centers. Exact 0/50/100 stops measured x=39.5/119.5/199.5.
Out-of-range -20/120 clamped to the endpoints. Reset held both at x=39.5 while models
changed to 25/75; resuming restored x=93/173. Synthetic inputs stayed at 90/10 after
the input-only probe throughout subsequent model changes. The retained intermediate
PNG was visually inspected and showed both independent panels in the expected rows.
These are observed renders, not a claim of arithmetic midpoint interpolation.
''')
