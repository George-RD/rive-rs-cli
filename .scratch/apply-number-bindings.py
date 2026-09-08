from pathlib import Path

files = {}


def replace(path, before, after, count=1):
    text = files.setdefault(path, Path(path).read_text())
    if text.count(before) != count:
        raise RuntimeError(f'patch anchor count for {path}: {before[:80]}')
    files[path] = text.replace(before, after)


compiler = 'src/authoring/frontend/compiler/behavior.rs'
if 'invalid_condition_binding' not in Path(compiler).read_text():
    replace('src/authoring/spec.rs', '''    pub(crate) fn binding(&self) -> Option<&str> {
        match self {
            Self::Binding(condition) => Some(&condition.binding),
            Self::NumberBinding(condition) => Some(&condition.binding),
            _ => None,
        }
    }''', '''    pub(crate) fn binding(&self) -> Option<(&str, BehaviorInputKind)> {
        match self {
            Self::Binding(condition) => Some((&condition.binding, BehaviorInputKind::Bool)),
            Self::NumberBinding(condition) => Some((&condition.binding, BehaviorInputKind::Number)),
            _ => None,
        }
    }''')
    replace(compiler, 'AuthoringSpec, BehaviorBindingSpec, BehaviorInputKind,', 'AuthoringSpec, BehaviorInputKind,')
    replace(compiler, '.filter_map(|transition| transition.when.binding())', '.filter_map(|transition| transition.when.binding().map(|(id, _)| id))')
    replace(compiler, '''        if bindings.insert(binding.id.as_str(), binding).is_some() {''', '''        let property_kind = models
            .get(binding.model.as_str())
            .and_then(|model| find_property(model, &binding.property))
            .map(BehaviorPropertySpec::kind);
        if bindings.insert(binding.id.as_str(), property_kind).is_some() {''')
    text = files[compiler]
    count = text.count('HashMap<&str, &BehaviorBindingSpec>')
    if count != 2:
        raise RuntimeError(f'expected two typed binding validation maps, found {count}')
    replace(compiler, 'HashMap<&str, &BehaviorBindingSpec>', 'HashMap<&str, Option<BehaviorInputKind>>', count=2)
    replace(compiler, '''    if let Some(binding) = condition.binding() {
        if !bindings.contains_key(binding) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{transition_path}.when.binding"),
                "unknown_behavior_binding",
                format!("behavior binding '{binding}' is not defined"),
            ));
        }
        return;
    }''', '''    if let Some((binding, expected)) = condition.binding() {
        match bindings.get(binding) {
            None => diagnostics.push(AuthoringDiagnostic::new(
                format!("{transition_path}.when.binding"),
                "unknown_behavior_binding",
                format!("behavior binding '{binding}' is not defined"),
            )),
            Some(Some(actual)) if *actual != expected => {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{transition_path}.when.binding"),
                    "invalid_condition_binding",
                    format!(
                        "condition expects a {} binding but '{binding}' references a {} property",
                        expected.as_str(), actual.as_str()
                    ),
                ));
            }
            Some(_) => {}
        }
        return;
    }''')
    builder = 'src/builder/state_machines.rs'
    replace(builder, '''fn resolve_view_model_binding_ids(
    artboard_children: &[ObjectSpec],''', '''fn resolve_view_model_binding_ids<'a>(
    artboard_children: &'a [ObjectSpec],''')
    replace(builder, ') -> Option<(u64, u64)> {', ") -> Option<(u64, u64, &'a ObjectSpec)> {")
    replace(builder, 'return Some((view_model_id, property_id));', 'return Some((view_model_id, property_id, property));')
    replace(builder, 'let path = resolve_view_model_binding_ids(', 'let (view_model_id, property_id, property) = resolve_view_model_binding_ids(')
    replace(builder, '''                            bound_number_input_paths.insert(name.clone(), path);''', '''                            if !matches!(property, ObjectSpec::ViewModelPropertyNumber { .. }) {
                                return Err(format!(
                                    "number input '{}' requires a number view-model property: '{}.{}'",
                                    name, binding.view_model, binding.property
                                ));
                            }
                            bound_number_input_paths.insert(name.clone(), (view_model_id, property_id));''')
    replace(builder, '''                            let (view_model_id, property_id) =
                                resolve_view_model_binding_ids(''', '''                            let (view_model_id, property_id, _) =
                                resolve_view_model_binding_ids(''')
    replace(builder, '''                                    match condition.value.as_ref() {''', '''                                    if bound_number_input_paths.contains_key(&condition.input)
                                        && condition.value.as_ref().and_then(json_value_to_f32).is_none()
                                    {
                                        return Err(format!(
                                            "bound number input '{}' requires a finite numeric condition value",
                                            condition.input
                                        ));
                                    }
                                    match condition.value.as_ref() {''')
    for path, text in files.items():
        Path(path).write_text(text)
