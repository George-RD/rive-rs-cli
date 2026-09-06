from pathlib import Path


def replace(path, before, after, marker):
    file = Path(path)
    text = file.read_text()
    if marker in text:
        return
    assert text.count(before) == 1, (path, text.count(before))
    file.write_text(text.replace(before, after))


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
