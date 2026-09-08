from pathlib import Path


def replace(path, before, after):
    p = Path(path)
    text = p.read_text()
    if before not in text:
        if after in text:
            return
        raise RuntimeError(f'missing patch anchor: {path}: {before[:80]}')
    if text.count(before) != 1:
        raise RuntimeError(f'ambiguous patch anchor: {path}: {before[:80]}')
    p.write_text(text.replace(before, after))


replace('src/authoring/spec.rs', '''pub enum BehaviorPropertySpec {
    Bool { id: String, value: bool },
}''', '''pub enum BehaviorPropertySpec {
    Bool { id: String, value: bool },
    Number { id: String, value: ScalarExpr },
}''')
replace('src/authoring/spec.rs', '''impl BehaviorPropertySpec {
    pub(crate) fn id(&self) -> &str {
        match self {
            Self::Bool { id, .. } => id,
        }
    }
}''', '''impl BehaviorPropertySpec {
    pub(crate) fn id(&self) -> &str {
        match self {
            Self::Bool { id, .. } | Self::Number { id, .. } => id,
        }
    }

    pub(crate) fn kind(&self) -> BehaviorInputKind {
        match self {
            Self::Bool { .. } => BehaviorInputKind::Bool,
            Self::Number { .. } => BehaviorInputKind::Number,
        }
    }
}''')
replace('src/authoring/spec.rs', '''pub struct BehaviorNumberConditionSpec {
    pub input: String,
    pub compare: BehaviorCompare,
    pub value: ScalarExpr,
}
''', '''pub struct BehaviorNumberConditionSpec {
    pub input: String,
    pub compare: BehaviorCompare,
    pub value: ScalarExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BehaviorNumberBindingConditionSpec {
    pub binding: String,
    pub compare: BehaviorCompare,
    pub value: ScalarExpr,
}
''')
replace('src/authoring/spec.rs', '''    Number(BehaviorNumberConditionSpec),
    Trigger(BehaviorTriggerConditionSpec),
}''', '''    Number(BehaviorNumberConditionSpec),
    NumberBinding(BehaviorNumberBindingConditionSpec),
    Trigger(BehaviorTriggerConditionSpec),
}

impl BehaviorTransitionConditionSpec {
    pub(crate) fn binding(&self) -> Option<&str> {
        match self {
            Self::Binding(condition) => Some(&condition.binding),
            Self::NumberBinding(condition) => Some(&condition.binding),
            _ => None,
        }
    }
}''')
p = 'src/authoring/frontend/compiler/behavior.rs'
replace(p, 'pub(super) fn lower_behavior(', '''struct LoweredBehaviorProperty {
    runtime_name: String,
    kind: BehaviorInputKind,
    value: Value,
}

pub(super) fn lower_behavior(''')
replace(p, '''            match property {
                BehaviorPropertySpec::Bool { .. } => properties.push(json!({
                    "type": "view_model_property_boolean",
                    "name": property_name
                })),
            }
            property_runtime_by_id
                .insert((model.id.as_str(), property.id()), property_name.clone());''', '''            let (property_type, value) = match property {
                BehaviorPropertySpec::Bool { value, .. } => {
                    ("view_model_property_boolean", json!(value))
                }
                BehaviorPropertySpec::Number { value, .. } => (
                    "view_model_property_number",
                    json!(evaluate_expression(
                        value,
                        &format!("$.behavior.models[{model_index}].properties[{property_index}].value"),
                        &spec.parameters,
                        Unit::Scalar,
                    ).map_err(AuthoringError::one)?),
                ),
            };
            properties.push(json!({ "type": property_type, "name": property_name }));
            property_runtime_by_id.insert(
                (model.id.as_str(), property.id()),
                LoweredBehaviorProperty {
                    runtime_name: property_name.clone(),
                    kind: property.kind(),
                    value,
                },
            );''')
replace(p, '''            .filter_map(|transition| match &transition.when {
                BehaviorTransitionConditionSpec::Binding(condition) => {
                    Some(condition.binding.as_str())
                }
                _ => None,
            })''', '''            .filter_map(|transition| transition.when.binding())''')
replace(p, '''            let property_name = property_runtime_by_id
                .get(&(binding.model.as_str(), binding.property.as_str()))
                .expect("validated behavior property");
            inputs.push(json!({
                "type": "bool",
                "name": input_name,
                "value": binding_bool_value(spec, binding),
                "view_model_binding": {
                    "view_model": model_name,
                    "property": property_name
                }
            }));''', '''            let property = property_runtime_by_id
                .get(&(binding.model.as_str(), binding.property.as_str()))
                .expect("validated behavior property");
            inputs.push(json!({
                "type": property.kind.as_str(),
                "name": input_name,
                "value": property.value,
                "view_model_binding": {
                    "view_model": model_name,
                    "property": property.runtime_name
                }
            }));''')
replace(p, '''            BehaviorTransitionConditionSpec::Input(condition) => json!({''', '''            BehaviorTransitionConditionSpec::NumberBinding(condition) => json!({
                "input": input_name_by_binding
                    .get(condition.binding.as_str())
                    .expect("validated transition binding"),
                "op": condition.compare.as_str(),
                "value": evaluate_expression(
                    &condition.value,
                    &format!("{transition_path}.when.value"),
                    &spec.parameters,
                    Unit::Scalar,
                )?
            }),
            BehaviorTransitionConditionSpec::Input(condition) => json!({''')
replace(p, '''    let (field, id, expected) = match condition {
        BehaviorTransitionConditionSpec::Binding(condition) => {
            if !bindings.contains_key(condition.binding.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{transition_path}.when.binding"),
                    "unknown_behavior_binding",
                    format!("behavior binding '{}' is not defined", condition.binding),
                ));
            }
            return;
        }''', '''    if let Some(binding) = condition.binding() {
        if !bindings.contains_key(binding) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{transition_path}.when.binding"),
                "unknown_behavior_binding",
                format!("behavior binding '{binding}' is not defined"),
            ));
        }
        return;
    }
    let (field, id, expected) = match condition {
        BehaviorTransitionConditionSpec::Binding(_)
        | BehaviorTransitionConditionSpec::NumberBinding(_) => return,''')
replace(p, '''
fn binding_bool_value(spec: &AuthoringSpec, binding: &BehaviorBindingSpec) -> bool {
    let model = spec
        .behavior
        .models
        .iter()
        .find(|model| model.id == binding.model)
        .expect("validated behavior model");
    match find_property(model, &binding.property).expect("validated behavior property") {
        BehaviorPropertySpec::Bool { value, .. } => *value,
    }
}
''', '\n')
replace('src/builder/spec.rs', '''    Number {
        name: String,
        value: f32,
    },
    Bool {''', '''    Number {
        name: String,
        value: f32,
        #[serde(default)]
        view_model_binding: Option<ViewModelInputBindingSpec>,
    },
    Bool {''')
p = 'src/builder/state_machines.rs'
replace(p, 'use crate::objects::data_binding::{BindablePropertyBoolean, DataBindContext};', 'use crate::objects::data_binding::{BindablePropertyBoolean, BindablePropertyNumber, DataBindContext};')
replace(p, '''        let mut bound_bool_input_paths: HashMap<String, (u64, u64)> = HashMap::new();''', '''        let mut bound_bool_input_paths: HashMap<String, (u64, u64)> = HashMap::new();
        let mut bound_number_input_paths: HashMap<String, (u64, u64)> = HashMap::new();''')
replace(p, '''                    InputSpec::Number { name, value } => {
                        objects.push(Box::new(StateMachineNumber {
                            name: name.clone(),
                            value: *value,
                        }));
                        input_name_to_index.insert(name.clone(), input_index);
                    }''', '''                    InputSpec::Number { name, value, view_model_binding } => {
                        objects.push(Box::new(StateMachineNumber {
                            name: name.clone(),
                            value: *value,
                        }));
                        if let Some(binding) = view_model_binding {
                            let path = resolve_view_model_binding_ids(
                                artboard_children,
                                view_model_id_base,
                                &binding.view_model,
                                &binding.property,
                            ).ok_or_else(|| format!(
                                "unknown view-model binding referenced by number input '{}': '{}.{}'",
                                name, binding.view_model, binding.property
                            ))?;
                            bound_number_input_paths.insert(name.clone(), path);
                        }
                        input_name_to_index.insert(name.clone(), input_index);
                    }''')
replace(p, '''                                            objects.push(Box::new(TransitionNumberCondition::new(
                                                input_id, op, value,
                                            )));''', '''                                            if let Some(&(view_model_id, property_id)) =
                                                bound_number_input_paths.get(&condition.input)
                                            {
                                                objects.push(Box::new(TransitionViewModelCondition {
                                                    op_value: op,
                                                }));
                                                objects.push(Box::new(BindablePropertyNumber {
                                                    property_value: 0.0,
                                                }));
                                                objects.push(Box::new(DataBindContext::new(
                                                    property_keys::BINDABLE_PROPERTY_NUMBER_VALUE as u64,
                                                    0,
                                                    encode_id_path(&[view_model_id, property_id]),
                                                )));
                                                objects.push(Box::new(TransitionPropertyViewModelComparator));
                                                objects.push(Box::new(TransitionValueNumberComparator { value }));
                                            } else {
                                                objects.push(Box::new(TransitionNumberCondition::new(
                                                    input_id, op, value,
                                                )));
                                            }''')
