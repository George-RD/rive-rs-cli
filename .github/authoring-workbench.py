from pathlib import Path


def replace(path, before, after, count=1):
    file = Path(path)
    text = file.read_text()
    if after in text and before not in text:
        return
    assert text.count(before) == count, (path, text.count(before), count)
    file.write_text(text.replace(before, after))


replace('src/authoring/spec.rs',
        '    pub duration_ms: Option<ScalarExpr>,\n}',
        '    pub duration_ms: Option<ScalarExpr>,\n    #[serde(default)]\n    pub exit_time_ms: Option<ScalarExpr>,\n}')
replace('src/builder/spec.rs',
        '    pub duration: Option<u64>,\n    pub conditions: Option<Vec<ConditionSpec>>,',
        '    pub duration: Option<u64>,\n    pub exit_time: Option<u32>,\n    pub conditions: Option<Vec<ConditionSpec>>,')
replace('src/builder/scene.rs',
        '                            duration: None,\n',
        '                            duration: None,\n                            exit_time: None,\n', 3)
replace('src/objects/state_machine.rs',
        'impl StateTransition {\n    pub fn new',
        'impl StateTransition {\n    pub const ENABLE_EXIT_TIME: u64 = 1 << 2;\n\n    pub fn new')
replace('src/builder/state_machines.rs',
        '''                        if let Some(duration) = transition.duration {
                            state_transition.duration = duration;
                        }
                        objects.push(Box::new(state_transition));''',
        '''                        if let Some(duration) = transition.duration {
                            state_transition.duration = duration;
                        }
                        if let Some(exit_time) = transition.exit_time {
                            state_transition.exit_time = u64::from(exit_time);
                            state_transition.flags |= StateTransition::ENABLE_EXIT_TIME;
                        }
                        objects.push(Box::new(state_transition));''')
replace('src/authoring/frontend/compiler/behavior.rs',
        '''            lowered_transition["duration"] = json!(duration as u64);
        }
        transitions.push(lowered_transition);''',
        '''            lowered_transition["duration"] = json!(duration as u64);
        }
        if let Some(expression) = &transition.exit_time_ms {
            let path = format!("{transition_path}.exit_time_ms");
            let exit_time = evaluate_expression(expression, &path, &spec.parameters, Unit::Scalar)?;
            if !(0.0..=f64::from(u32::MAX)).contains(&exit_time) || exit_time.fract() != 0.0 {
                return Err(AuthoringDiagnostic::new(
                    path,
                    "invalid_transition_exit_time",
                    format!(
                        "transition exit time must be whole milliseconds between 0 and {}",
                        u32::MAX
                    ),
                ));
            }
            lowered_transition["exit_time"] = json!(exit_time as u32);
        }
        transitions.push(lowered_transition);''')
