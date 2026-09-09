from pathlib import Path

path = Path('src/authoring/spec.rs')
text = path.read_text()
old = '#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\n#[serde(untagged, deny_unknown_fields)]\npub enum BehaviorTransitionGuardSpec {'
new = '#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]\n#[serde(rename_all = "snake_case")]\npub enum BehaviorAlwaysGuardSpec {\n    Always,\n}\n\n' + old
assert text.count(old) == 1
text = text.replace(old, new)
text = text.replace('pub enum BehaviorTransitionGuardSpec {\n', 'pub enum BehaviorTransitionGuardSpec {\n    Always(BehaviorAlwaysGuardSpec),\n', 1)
text = text.replace('            Self::Condition(condition) => std::slice::from_ref(condition),', '            Self::Always(_) => &[],\n            Self::Condition(condition) => std::slice::from_ref(condition),', 1)
text = text.replace('            Self::Condition(_) => format!("{transition_path}.when"),', '            Self::Always(_) | Self::Condition(_) => format!("{transition_path}.when"),', 1)
path.write_text(text)
path = Path('src/authoring/behavior_limits.rs')
text = path.read_text()
text = text.replace('AuthoringSpec, BehaviorTransitionSpec', 'AuthoringSpec, BehaviorTransitionGuardSpec, BehaviorTransitionSpec', 1)
old = '''        validate_count(
            transition.when.conditions().len(),
            1,
            &format!("{path}.transitions[{index}].when.all"),
        )?;'''
new = '''        if let BehaviorTransitionGuardSpec::All { all } = &transition.when {
            validate_count(
                all.len(),
                1,
                &format!("{path}.transitions[{index}].when.all"),
            )?;
        }'''
assert text.count(old) == 1
path.write_text(text.replace(old, new))
Path(__file__).unlink()
