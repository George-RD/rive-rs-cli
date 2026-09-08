from pathlib import Path
import re


def replace_once(text, old, new):
    assert text.count(old) == 1, (old[:120], text.count(old))
    return text.replace(old, new, 1)


spec_path = Path('src/authoring/spec.rs')
if 'pub struct BehaviorDirectBlendSpec' not in spec_path.read_text():
    spec = spec_path.read_text()
    spec = replace_once(spec, '    pub blend: Option<BehaviorBlendSpec>,\n', '    pub blend: Option<BehaviorBlendSpec>,\n    #[serde(default)]\n    pub direct_blend: Option<BehaviorDirectBlendSpec>,\n')
    marker = '#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]\n#[serde(deny_unknown_fields)]\npub struct BehaviorStateSpec'
    spec = replace_once(spec, marker, '''#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDirectBlendMotionSpec {
    pub motion: String,
    pub input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDirectBlendSpec {
    #[schemars(length(min = 1, max = 1000))]
    pub motions: Vec<BehaviorDirectBlendMotionSpec>,
}

''' + marker)

    compiler_path = Path('src/authoring/frontend/compiler.rs')
    compiler = compiler_path.read_text()
    compiler = replace_once(compiler, 'behavior::lower_behavior(spec, child_index_base, 0, listener_targets)', '''behavior::lower_behavior(
            spec, child_index_base, 0, listener_targets,
            lowered.scene["artboard"]["animations"].as_array().map(Vec::as_slice).unwrap_or(&[]),
        )''')

    behavior_path = Path('src/authoring/frontend/compiler/behavior.rs')
    behavior = behavior_path.read_text()
    behavior = replace_once(behavior, '    listener_targets: MotionTargetIndex,\n', '    listener_targets: MotionTargetIndex,\n    animations: &[Value],\n')
    behavior = replace_once(behavior, '    let mut source_entries = Vec::new();\n', '''    let animation_index_by_name = animations.iter().enumerate()
        .filter_map(|(index, animation)| animation["name"].as_str().map(|name| (name, index)))
        .collect::<HashMap<_, _>>();
    let mut source_entries = Vec::new();
''')
    behavior = replace_once(behavior, '        let mut input_name_by_id = HashMap::new();\n', '        let mut input_name_by_id = HashMap::new();\n        let mut input_index_by_id = HashMap::new();\n')
    behavior = replace_once(behavior, '            input_name_by_id.insert(input.id(), input_name.clone());\n', '            input_name_by_id.insert(input.id(), input_name.clone());\n            input_index_by_id.insert(input.id(), scene_input_index);\n')
    behavior, count = re.subn(r'(&input_name_by_binding,\n)(\s+)(&mut source_entries,)', r'\1\2&input_index_by_id,\n\2&animation_index_by_name,\n\2\3', behavior)
    assert count == 2, count
    behavior = replace_once(behavior, '    input_name_by_binding: &HashMap<&str, String>,\n', '    input_name_by_binding: &HashMap<&str, String>,\n    input_index_by_id: &HashMap<&str, usize>,\n    animation_index_by_name: &HashMap<&str, usize>,\n')
    behavior = replace_once(behavior, '        let lowered = match (&state.motion, &state.blend) {', '        let lowered = match (&state.motion, &state.blend, &state.direct_blend) {')
    behavior = replace_once(behavior, '            (Some(motion), None) => json!({', '            (Some(motion), None, None) => json!({')
    behavior = replace_once(behavior, '            (None, Some(blend)) => {', '            (None, Some(blend), None) => {')
    marker = '''            _ => {
                return Err(AuthoringDiagnostic::new(
                    state_path,
                    "missing_state_motion",'''
    behavior = replace_once(behavior, marker, '''            (None, None, Some(blend)) => {
                let children = blend.motions.iter().map(|motion| {
                    let animation_name = animation_runtime_name(spec, &motion.motion);
                    json!({
                        "type": "blend_animation_direct",
                        "animation_id": animation_index_by_name.get(animation_name.as_str())
                            .expect("validated motion has a lowered animation"),
                        "input_id": input_index_by_id.get(motion.input.as_str())
                            .expect("validated direct blend input")
                    })
                }).collect::<Vec<_>>();
                json!({ "type": "blend_state_direct", "children": children })
            }
''' + marker)
    behavior = replace_once(behavior, "a behavior state must declare exactly one of 'motion' or 'blend'", "a behavior state must declare exactly one of 'motion', 'blend', or 'direct_blend'")
    behavior = replace_once(behavior, '            if authored_states[from - 1].blend.is_some() {', '            if authored_states[from - 1].blend.is_some()\n                || authored_states[from - 1].direct_blend.is_some()\n            {')
    marker = '''    match (&state.motion, &state.blend) {
        (None, None)'''
    behavior = replace_once(behavior, marker, '''    if let Some(blend) = &state.direct_blend {
        if state.motion.is_some() || state.blend.is_some() {
            diagnostics.push(AuthoringDiagnostic::new(
                state_path,
                "ambiguous_state_motion",
                "a behavior state must declare exactly one of 'motion', 'blend', or 'direct_blend'",
            ));
            return;
        }
        if !(1..=BLEND_STOP_LIMIT).contains(&blend.motions.len()) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{state_path}.direct_blend.motions"),
                "invalid_direct_blend_motions",
                format!("a direct blend needs between 1 and {BLEND_STOP_LIMIT} motions"),
            ));
        }
        for (index, motion) in blend.motions.iter().enumerate() {
            let path = format!("{state_path}.direct_blend.motions[{index}]");
            if !motion_tracks.contains(motion.motion.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{path}.motion"),
                    "unknown_behavior_motion",
                    format!("motion track '{}' is not defined", motion.motion),
                ));
            }
            match inputs.get(motion.input.as_str()) {
                None => diagnostics.push(AuthoringDiagnostic::new(
                    format!("{path}.input"),
                    "unknown_behavior_input",
                    format!("behavior input '{}' is not defined in statechart '{statechart_id}'", motion.input),
                )),
                Some(input) if input.kind() != BehaviorInputKind::Number => {
                    diagnostics.push(AuthoringDiagnostic::new(
                        format!("{path}.input"),
                        "invalid_blend_input",
                        format!("direct blend input '{}' must be a number input but is declared as {}", motion.input, input.kind().as_str()),
                    ));
                }
                Some(_) => {}
            }
        }
        return;
    }
    match (&state.motion, &state.blend) {
        (None, None)''')

    spec_path.write_text(spec)
    compiler_path.write_text(compiler)
    behavior_path.write_text(behavior)
