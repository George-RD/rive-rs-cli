from pathlib import Path


def replace(path, old, new):
    file = Path(path)
    text = file.read_text()
    if old in text:
        assert text.count(old) == 1, (path, text.count(old))
        file.write_text(text.replace(old, new))
    else:
        assert new in text, path


replace('src/authoring/spec.rs', '''#[serde(deny_unknown_fields)]
pub struct BehaviorBlendSpec {
    pub input: String,
    #[schemars(length(min = 2, max = 1000))]
    pub stops: Vec<BehaviorBlendStopSpec>,
}''', '''#[serde(untagged, deny_unknown_fields)]
pub enum BehaviorBlendSpec {
    Input {
        input: String,
        #[schemars(length(min = 2, max = 1000))]
        stops: Vec<BehaviorBlendStopSpec>,
    },
    Binding {
        binding: String,
        #[schemars(length(min = 2, max = 1000))]
        stops: Vec<BehaviorBlendStopSpec>,
    },
}

impl BehaviorBlendSpec {
    pub(crate) fn source_id(&self) -> &str {
        match self {
            Self::Input { input, .. } => input,
            Self::Binding { binding, .. } => binding,
        }
    }

    pub(crate) fn binding(&self) -> Option<&str> {
        match self {
            Self::Input { .. } => None,
            Self::Binding { binding, .. } => Some(binding),
        }
    }

    pub(crate) fn stops(&self) -> &[BehaviorBlendStopSpec] {
        match self {
            Self::Input { stops, .. } | Self::Binding { stops, .. } => stops,
        }
    }
}''')

path = 'src/authoring/frontend/compiler/behavior.rs'
replace(path, '''            .collect::<HashSet<_>>();
        let mut input_name_by_binding''', '''            .chain(
                statechart
                    .states
                    .iter()
                    .chain(statechart.regions.iter().flat_map(|region| &region.states))
                    .filter_map(|state| state.blend.as_ref())
                    .filter_map(|blend| blend.binding()),
            )
            .collect::<HashSet<_>>();
        let mut input_name_by_binding''')
replace(path, '''                let input = input_name_by_id
                    .get(blend.input.as_str())
                    .expect("validated blend input");''', '''                let input_names = if blend.binding().is_some() {
                    input_name_by_binding
                } else {
                    input_name_by_id
                };
                let input = input_names
                    .get(blend.source_id())
                    .expect("validated blend source");''')
replace(path, '''            match inputs.get(blend.input.as_str()) {
                None => diagnostics.push(AuthoringDiagnostic::new(
                    format!("{state_path}.blend.input"),
                    "unknown_behavior_input",
                    format!(
                        "behavior input '{}' is not defined in statechart '{statechart_id}'",
                        blend.input
                    ),
                )),
                Some(input) if input.kind() != BehaviorInputKind::Number => {
                    diagnostics.push(AuthoringDiagnostic::new(
                        format!("{state_path}.blend.input"),
                        "invalid_blend_input",
                        format!(
                            "blend input '{}' must be a number input but is declared as {}",
                            blend.input,
                            input.kind().as_str()
                        ),
                    ));
                }
                Some(_) => {}
            }''', '''            if let Some(binding) = blend.binding() {
                match bindings.get(binding) {
                    None => diagnostics.push(AuthoringDiagnostic::new(
                        format!("{state_path}.blend.binding"),
                        "unknown_behavior_binding",
                        format!("behavior binding '{binding}' is not defined"),
                    )),
                    Some(Some(actual)) if *actual != BehaviorInputKind::Number => {
                        diagnostics.push(AuthoringDiagnostic::new(
                            format!("{state_path}.blend.binding"),
                            "invalid_blend_binding",
                            format!(
                                "blend binding '{binding}' must reference a number property but references {}",
                                actual.as_str()
                            ),
                        ));
                    }
                    Some(_) => {}
                }
            } else {
                match inputs.get(blend.source_id()) {
                    None => diagnostics.push(AuthoringDiagnostic::new(
                        format!("{state_path}.blend.input"),
                        "unknown_behavior_input",
                        format!(
                            "behavior input '{}' is not defined in statechart '{statechart_id}'",
                            blend.source_id()
                        ),
                    )),
                    Some(input) if input.kind() != BehaviorInputKind::Number => {
                        diagnostics.push(AuthoringDiagnostic::new(
                            format!("{state_path}.blend.input"),
                            "invalid_blend_input",
                            format!(
                                "blend input '{}' must be a number input but is declared as {}",
                                blend.source_id(),
                                input.kind().as_str()
                            ),
                        ));
                    }
                    Some(_) => {}
                }
            }''')
file = Path(path)
text = file.read_text().replace('blend.stops.len()', 'blend.stops().len()').replace('blend.stops.iter()', 'blend.stops().iter()')
file.write_text(text)
replace('tests/authoring_model_blend_1d_contract.rs', '''    assert_eq!(machine["layers"][0]["states"][1]["input_id"], 0);''', '''    assert_eq!(
        machine["layers"][0]["states"][1]["input"],
        machine["inputs"][0]["name"]
    );''')
