from pathlib import Path


def replace(path, before, after, marker=None):
    file = Path(path)
    text = file.read_text()
    if (marker or after) in text:
        return
    assert text.count(before) == 1, (path, text.count(before))
    file.write_text(text.replace(before, after))


def append(path, text):
    file = Path(path)
    if text.strip() not in file.read_text():
        file.write_text(file.read_text().rstrip() + '\n\n' + text.strip() + '\n')


replace('src/authoring/frontend/compiler/behavior.rs',
        '''        if let Some(expression) = &transition.exit_time_ms {
            let path = format!("{transition_path}.exit_time_ms");''',
        '''        if let Some(expression) = &transition.exit_time_ms {
            let path = format!("{transition_path}.exit_time_ms");
            if authored_states[from - 1].blend.is_some() {
                return Err(AuthoringDiagnostic::new(
                    path,
                    "unsupported_transition_exit_source",
                    "transition exit time requires a named motion source state, not a blend",
                ));
            }''',
        '"unsupported_transition_exit_source"')
replace('src/builder/validation.rs',
        '''                        if let Some(conditions) = &transition.conditions {''',
        '''                        if transition.exit_time.is_some()
                            && !matches!(&layer.states[transition.from], StateSpec::Animation { .. })
                        {
                            return Err(format!(
                                "exit_time requires an animation source state, but transition source {} in state machine '{}' is not an animation",
                                transition.from, state_machine.name
                            ));
                        }

                        if let Some(conditions) = &transition.conditions {''',
        '"exit_time requires an animation source state, but transition source')
file = Path('tests/authoring_transition_exit_time_contract.rs')
text = file.read_text().replace('PropertyValueReadRead::', 'PropertyValueRead::')
if 'for (expression, code, suffix)' not in text:
    text = text.replace('for (expression, code) in [', 'for (expression, code, suffix) in [')
    for code, suffix in [('unit_mismatch', ''), ('unknown_parameter', '.name'), ('division_by_zero', '.divisor'), ('invalid_transition_exit_time', '')]:
        text = text.replace(f'"{code}"),', f'"{code}", "{suffix}"),')
    text = text.replace('assert_diagnostic(&error, code, EXIT_PATH);', 'assert_diagnostic(&error, code, &format!("{EXIT_PATH}{suffix}"));')
text = text.replace('assert_diagnostic(&error, "non_finite", EXIT_PATH);', 'assert_diagnostic(&error, "non_finite", &format!("{EXIT_PATH}.value"));')
file.write_text(text)

replace('cairn.blueprint',
        '                "./tests/authoring_trim_path_contract.rs",',
        '                "./tests/authoring_transition_duration_contract.rs",\n'
        '                "./tests/authoring_transition_exit_time_contract.rs",\n'
        '                "./tests/authoring_trim_path_contract.rs",')
replace('CHANGELOG.md', '## [Unreleased]\n\n### Added\n', '''## [Unreleased]

### Added

- **Exit-time gates for authored transitions.** Optional `exit_time_ms` scalar expressions wait for the outgoing animation timeline while still requiring the authored condition. Gates compose with transition duration and work in parallel regions. Finite whole milliseconds from 0 through `u32::MAX` lower to canonical `exit_time` and the existing runtime enable flag; omission preserves prior output, while explicit zero enables a zero-time gate. Typed and canonical builders reject non-animation sources rather than silently ignoring the gate. Public contracts and retained official-runtime comparisons cover early and late inputs, unmet conditions, looping, and gate-plus-duration behavior.
''')
replace('docs/authoring-spec-v0.md', '## Blend states\n', '''## Transition exit time

`exit_time_ms` prevents a transition from leaving its named motion state before the outgoing animation reaches the specified point. The `when` condition must also be satisfied. It is animation time, not a delay started by an input:

```json
{
  "id": "engage",
  "from": "resting",
  "to": "engaged",
  "when": { "input": "pressed", "equals": true },
  "exit_time_ms": { "kind": "literal", "value": 1000, "unit": "scalar" },
  "duration_ms": { "kind": "literal", "value": 250, "unit": "scalar" }
}
```

With a one-shot outgoing track, an input that becomes true early waits for 1000ms of animation time; an input that becomes true after that point can transition immediately. Once allowed, the optional `duration_ms` controls the blend independently. For looping animations, the official runtime repeats gates within the animation's duration on subsequent cycles; gates beyond one cycle use accumulated animation time. This field does not change that runtime behavior.

The scalar expression uses document parameters and must resolve to a finite integer from 0 through 4294967295 milliseconds. Negative, fractional, and oversized results report `invalid_transition_exit_time` at the authored `.exit_time_ms` path; expression errors retain their existing codes and paths. Parallel-region transitions use the same validation and lowering. The source must name a motion track: a blend source reports `unsupported_transition_exit_source` at `.exit_time_ms`, because an ordinary runtime transition cannot enforce that gate on a blend. A blend destination is allowed.

Omitted or null gates preserve the previous canonical scene, source map, and compiled bytes. Explicit zero writes canonical `exit_time: 0` and enables the runtime gate flag, so its binary representation differs from omission. Entry transitions remain ungated. Canonical `TransitionSpec.exit_time` accepts an unsigned 32-bit millisecond value and rejects entry, exit, any, and blend source states. Percentage timing, pause-on-exit, early exit, and blend-source animation selection remain outside this field. Both format versions are unchanged.

`tests/authoring_transition_exit_time_contract.rs` verifies the public compiler and encoded properties. `tests/playwright/authoring-behavior-runtime.js` retains authored source, compiled binaries, representative PNGs, and hashes under `typed-interaction/exit-time` in the typed-behavior runtime CI artifact. Comparisons cover early input, late input, unmet conditions, looping, and exit time combined with duration.

## Blend states
''')
replace('docs/authoring-spec-v0.md',
        'Additive blend states, direct blend states, exit time, and view-model number and trigger properties remain outside',
        'Additive blend states, direct blend states, advanced exit timing, and view-model number and trigger properties remain outside')
replace('docs/authoring-spec-v0.md',
        '`duration_ms` is an optional transition field.',
        '`exit_time_ms` is an optional outgoing-animation gate; its contract is described under Transition exit time. `duration_ms` is an optional transition field.')
replace('meta/contracts/authoring.md',
        'A behavior state declares exactly one of `motion` and `blend`.',
        '''A transition may also declare `exit_time_ms`, evaluated in document scalar scope
as whole milliseconds from 0 through `u32::MAX`. Invalid numeric values report
`invalid_transition_exit_time`; expression errors retain their own codes and paths.
Only named motion sources are supported: a blend source reports
`unsupported_transition_exit_source` at the authored `.exit_time_ms` path. Blend
destinations remain valid. Root and region transitions use the same compiler path.
The gate requires outgoing animation time and the existing condition, not elapsed
time since the input. Duration controls blending independently; runtime loop timing
is unchanged. Omission/null preserves output and source-map identity; explicit zero
enables a zero-time gate and therefore changes encoded flags. Canonical `exit_time`
is an unsigned 32-bit value wired through the existing transition object; no second
lowering pass or encoder is introduced.

A behavior state declares exactly one of `motion` and `blend`.''')
replace('meta/contracts/authoring.md', 'direct blend states, exit time, and', 'direct blend states, advanced exit timing, and')
replace('meta/contracts/authoring.md',
        '`stacking`, `continuity`, `waypoint`, `blend`, `regions`, and `duration_ms` are optional and',
        '`stacking`, `continuity`, `waypoint`, `blend`, `regions`, `duration_ms`, and `exit_time_ms` are optional and')
append('meta/contracts/builder.md', '''## Transition exit-time gates

`TransitionSpec.exit_time` is optional whole milliseconds represented by `u32`.
When present, it sets the existing StateTransition exit-time property and named
EnableExitTime flag, including when zero. Other timing flags remain unset.
Omission/null preserves prior binary behavior. `duration` is independent.
Validation checks transition bounds before inspecting its source, then rejects any
exit-time gate whose source is not an animation state. Entry, exit, any, and blend
sources cannot silently bypass the gate. Source references and condition checks
retain the existing validate-first path and SceneSpec v1 remains unchanged.''')
replace('ROADMAP.md', 'Additive and direct blend states, exit time,', 'Additive and direct blend states, advanced exit timing,')
replace('ROADMAP.md',
        'Exit-time gates remain part of the open behavior slice.',
        '''[#227](https://github.com/George-RD/rive-rs-cli/issues/227) adds `exit_time_ms`
gates for outgoing named motion states, preserving conditions and supporting
parallel regions and independent blend duration. Canonical `exit_time` wires the
existing runtime flag; unsupported source types are rejected. Public contracts and
retained runtime comparisons cover boundary values, source maps, early/late inputs,
unmet conditions, looping, and gate-plus-duration behavior. Blend-source and other
advanced exit timing remain outside this slice.''')
replace('ROADMAP.md', 'transition duration added in #225 |', 'transition duration added in #225; animation exit-time gates added in #227 |')
replace('ROADMAP.md', 'additive and direct blend states, exit time, and', 'additive and direct blend states, advanced exit timing, and')
replace('meta/todos/todo.behavior-authoring-compiler.md', 'direct blend states, exit time, and', 'direct blend states, advanced exit timing, and')
append('meta/todos/todo.behavior-authoring-compiler.md', '''## Exit-time gate slice (#227)

`exit_time_ms` adds outgoing-animation gates through the existing root/region
compiler path. It accepts document-scoped scalar expressions resolving to finite
whole milliseconds in `0..=u32::MAX`; invalid values preserve authored paths.
Conditions remain required and `duration_ms` remains independent. Omitted/null
gates preserve prior output and explicit zero enables the runtime flag. Typed
blend sources and canonical non-animation sources are rejected rather than
silently ignoring their gate. No encoder redesign or second compiler pass is added.

The public exit-time contract covers parameter evaluation, integer boundaries,
expression errors, programmatic non-finite values, zero/omission, conditions,
deterministic source maps, region scope, supported destinations, and canonical
source validation. The existing typed-behavior runtime artifact retains authored
JSON, compiled binaries, PNGs, and hashes for early/late inputs, false conditions,
loop timing, and exit time composed with duration.

TDD: test-only head `103b9d33a7bc052192181aade451527fa50eaf00`, run
`34024382002`, failed because `exit_time_ms` was unknown. Run `34024457586`
passed the same public encoded-property contract and the duration regressions.
Test-only head `f75b09a25a2cb1fc8bf0c754e63e305af67c23e8`, run
`34024595568`, then failed both source-rejection contracts because the unsupported
gates were accepted. The subsequent implementation rejects those sources before
encoding. Final exact-head CI and separate Standards/Spec self-review are recorded
on the pull request for #227. Local Cargo execution was unavailable; Rust execution
and schema generation used GitHub Actions. This parent todo remains open for
additive/direct blends, advanced exit timing, and non-boolean view-model properties.''')

runtime = r'''function verifyCliTransitionExitTime() {
  const exitTimeMs = 1000;
  const durationMs = 1000;
  const frames = [0, 1, 30, 54, 66, 81, 96, 126];
  const input = `${INTERACTION_PLAN.input}=true@1`;
  const lateInput = `${INTERACTION_PLAN.input}=true@90`;
  const directory = path.join(INTERACTION_DIR, "exit-time");
  fs.mkdirSync(directory, { recursive: true });
  const document = JSON.parse(fs.readFileSync(INTERACTION_INPUT, "utf8"));
  for (const track of document.motion.tracks) {
    track.duration_frames.value = 120;
    track.keyframes[1].frame.value = 120;
  }
  const compile = (label) => {
    const source = path.join(directory, `${label}.v0.json`);
    const riv = path.join(directory, `${label}.riv`);
    fs.writeFileSync(source, `${JSON.stringify(document, null, 2)}\n`);
    run("cargo", ["run", "--quiet", "--", "authoring", "compile", source, "-o", riv]);
    return riv;
  };
  const ungatedRiv = compile("ungated");
  const transition = document.behavior.statecharts[0].transitions[0];
  transition.exit_time_ms = { kind: "literal", value: exitTimeMs, unit: "scalar" };
  const gatedRiv = compile("gated");
  transition.duration_ms = { kind: "literal", value: durationMs, unit: "scalar" };
  const blendedRiv = compile("blended");
  const render = (riv, label, scheduledInput) => renderInteraction(
    riv, `exit-time/${label}`, scheduledInput ? ["--input", scheduledInput] : [], frames.join(","),
  );
  const results = {
    control: render(gatedRiv, "control"),
    ungated: render(ungatedRiv, "ungated", input),
    gated: render(gatedRiv, "gated", input),
    late: render(gatedRiv, "late", lateInput),
    blended: render(blendedRiv, "blended", input),
  };
  for (const [label, result] of Object.entries(results)) {
    if (result.pngs.length !== frames.length) {
      throw new Error(`${label} exit-time evidence has an unexpected frame count`);
    }
  }
  const rest = results.control.first;
  const destination = results.ungated.last;
  if (rest.equals(destination) || fs.readFileSync(results.ungated.pngs[2]).equals(rest)) {
    throw new Error("ungated comparison did not reach a distinct destination before the gate");
  }
  for (const png of results.control.pngs) {
    if (!fs.readFileSync(png).equals(rest)) {
      throw new Error("exit time bypassed its unmet input condition");
    }
  }
  for (const label of ["gated", "blended", "late"]) {
    for (const index of [0, 1, 2, 3]) {
      if (!fs.readFileSync(results[label].pngs[index]).equals(rest)) {
        throw new Error(`${label} left the outgoing state before the exit-time gate`);
      }
    }
    if (!results[label].last.equals(destination)) {
      throw new Error(`${label} did not finish at the expected destination`);
    }
  }
  if (!fs.readFileSync(results.gated.pngs[4]).equals(destination)) {
    throw new Error("satisfied exit-time gate did not release the transition");
  }
  if (!fs.readFileSync(results.late.pngs[5]).equals(rest)
      || !fs.readFileSync(results.late.pngs[6]).equals(destination)) {
    throw new Error("exit time was treated as a delay from the input instead of animation time");
  }
  const intermediateHashes = new Set();
  for (const index of [4, 5, 6]) {
    const frame = fs.readFileSync(results.blended.pngs[index]);
    if (frame.equals(rest) || frame.equals(destination)) {
      throw new Error(`exit-time plus duration has no intermediate pose at frame ${frames[index]}`);
    }
    intermediateHashes.add(sha256(frame));
  }
  if (intermediateHashes.size !== 3) {
    throw new Error("exit-time plus duration did not advance through distinct blended poses");
  }
  delete transition.duration_ms;
  document.motion.tracks[0].loop_type = "loop";
  const loopedRiv = compile("looped");
  const loopFrames = [0, 126, 150, 174, 186, 216];
  const loopInput = `${INTERACTION_PLAN.input}=true@150`;
  const looped = renderInteraction(loopedRiv, "exit-time/looped", ["--input", loopInput], loopFrames.join(","));
  if (looped.pngs.length !== loopFrames.length) {
    throw new Error("looped exit-time evidence has an unexpected frame count");
  }
  for (const index of [0, 1, 2, 3]) {
    if (!fs.readFileSync(looped.pngs[index]).equals(rest)) {
      throw new Error("looped transition did not wait for its next-cycle gate");
    }
  }
  for (const index of [4, 5]) {
    if (!fs.readFileSync(looped.pngs[index]).equals(destination)) {
      throw new Error("looped exit-time gate did not release the transition");
    }
  }
  const evidence = {
    exitTimeMs, durationMs, fps: 60, input, lateInput, frames, loopFrames, loopInput,
    looped: looped.pngs.map((png) => sha256(fs.readFileSync(png))),
  };
  for (const [label, result] of Object.entries(results)) {
    evidence[label] = result.pngs.map((png) => sha256(fs.readFileSync(png)));
  }
  fs.writeFileSync(path.join(directory, "evidence.json"), `${JSON.stringify(evidence, null, 2)}\n`);
  console.log("typed exit-time gate waits for animation time, preserves conditions, and composes with duration");
}

'''
replace('tests/playwright/authoring-behavior-runtime.js',
        '  verifyCliTransitionDuration(riv);',
        '  verifyCliTransitionDuration(riv);\n  verifyCliTransitionExitTime();')
replace('tests/playwright/authoring-behavior-runtime.js',
        'function assertSamePng(actualPath, baselinePath, label) {',
        runtime + 'function assertSamePng(actualPath, baselinePath, label) {',
        'function verifyCliTransitionExitTime() {')
