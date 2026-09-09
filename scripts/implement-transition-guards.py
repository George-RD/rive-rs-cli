from pathlib import Path


def replace_once(text, old, new):
    assert text.count(old) == 1, (old, text.count(old))
    return text.replace(old, new, 1)


path = Path('src/authoring/spec.rs')
text = path.read_text()
marker = '#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\n#[serde(deny_unknown_fields)]\npub struct BehaviorTransitionSpec {'
addition = '''#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged, deny_unknown_fields)]
pub enum BehaviorTransitionGuardSpec {
    Condition(BehaviorTransitionConditionSpec),
    All {
        #[schemars(length(min = 1, max = 1000))]
        all: Vec<BehaviorTransitionConditionSpec>,
    },
}

impl BehaviorTransitionGuardSpec {
    pub(crate) fn conditions(&self) -> &[BehaviorTransitionConditionSpec] {
        match self {
            Self::Condition(condition) => std::slice::from_ref(condition),
            Self::All { all } => all,
        }
    }

    pub(crate) fn condition_path(&self, transition_path: &str, index: usize) -> String {
        match self {
            Self::Condition(_) => format!("{transition_path}.when"),
            Self::All { .. } => format!("{transition_path}.when.all[{index}]"),
        }
    }
}

'''
text = replace_once(text, marker, addition + marker)
text = replace_once(text, 'pub when: BehaviorTransitionConditionSpec,', 'pub when: BehaviorTransitionGuardSpec,')
path.write_text(text)

path = Path('src/authoring/frontend/compiler/behavior.rs')
text = path.read_text()
text = replace_once(text,
    '.filter_map(|transition| transition.when.binding().map(|(id, _)| id))',
    '.flat_map(|transition| transition.when.conditions())\n            .filter_map(|condition| condition.binding().map(|(id, _)| id))')
start = text.index('        let condition = match &transition.when {')
end = text.index('        let mut lowered_transition', start)
old = text[start:end]
match_body = old[len('        let condition = match &transition.when '):].strip()
assert match_body.endswith(';')
match_body = match_body[:-1].replace('{transition_path}.when.value', '{condition_path}.value')
replacement = '''        let conditions = transition.when.conditions().iter().enumerate().map(|(index, condition)| {
            let condition_path = transition.when.condition_path(&transition_path, index);
            Ok(match condition ''' + match_body + ''')
        }).collect::<Result<Vec<_>, AuthoringDiagnostic>>()?;
'''
text = text[:start] + replacement + text[end:]
text = replace_once(text, '"conditions": [condition]', '"conditions": conditions')
text = replace_once(text, '''        validate_condition(
            &transition.when,
            &transition_path,
            statechart_id,
            inputs,
            bindings,
            diagnostics,
        );''', '''        for (index, condition) in transition.when.conditions().iter().enumerate() {
            validate_condition(
                condition,
                &transition.when.condition_path(&transition_path, index),
                statechart_id,
                inputs,
                bindings,
                diagnostics,
            );
        }''')
start = text.index('fn validate_condition(')
end = text.index('fn resolve_listener_target(', start)
body = text[start:end].replace('transition_path: &str', 'condition_path: &str').replace('{transition_path}.when', '{condition_path}')
text = text[:start] + body + text[end:]
path.write_text(text)

path = Path('src/authoring/behavior_limits.rs')
text = path.read_text()
text = replace_once(text,
    'use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec};',
    'use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec, BehaviorTransitionSpec};')
text = replace_once(text,
    '        for (listener_index, listener) in chart.listeners.iter().enumerate() {',
    '        validate_transition_guards(&chart.transitions, &path)?;\n        for (listener_index, listener) in chart.listeners.iter().enumerate() {')
text = replace_once(text,
    '            validate_count(region.states.len(), 1, &format!("{region_path}.states"))?;',
    '            validate_transition_guards(&region.transitions, &region_path)?;\n            validate_count(region.states.len(), 1, &format!("{region_path}.states"))?;')
addition = '''fn validate_transition_guards(
    transitions: &[BehaviorTransitionSpec],
    path: &str,
) -> Result<(), AuthoringError> {
    for (index, transition) in transitions.iter().enumerate() {
        validate_count(
            transition.when.conditions().len(),
            1,
            &format!("{path}.transitions[{index}].when.all"),
        )?;
    }
    Ok(())
}

'''
text = replace_once(text, 'fn validate_count(', addition + 'fn validate_count(')
path.write_text(text)

path = Path('cairn.blueprint')
text = path.read_text()
text = replace_once(text,
    '                "./tests/authoring_transition_exit_time_contract.rs",',
    '                "./tests/authoring_transition_exit_time_contract.rs",\n                "./tests/authoring_transition_guard_contract.rs",')
path.write_text(text)
