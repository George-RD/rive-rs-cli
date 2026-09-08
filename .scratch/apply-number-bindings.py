from pathlib import Path


def replace(path, before, after):
    target = Path(path)
    text = target.read_text()
    if before not in text:
        if after in text:
            return
        raise RuntimeError(f'missing patch anchor: {path}')
    if text.count(before) != 1:
        raise RuntimeError(f'ambiguous patch anchor: {path}')
    target.write_text(text.replace(before, after))


replace('src/objects/data_binding.rs', '''impl RiveObject for ViewModelPropertyNumber {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_NUMBER
    }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ]
    }
}''', '''impl RiveObject for ViewModelPropertyNumber {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_NUMBER
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::VIEW_MODEL_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}''')
replace('src/builder/state_machines.rs', '''                    InputSpec::Number {
                        name,
                        value,
                        view_model_binding,
                    } => {
                        objects.push''', '''                    InputSpec::Number {
                        name,
                        value,
                        view_model_binding,
                    } => {
                        if view_model_binding.is_some() && !value.is_finite() {
                            return Err(format!(
                                "bound number input '{name}' requires a finite initial value"
                            ));
                        }
                        objects.push''')
replace('src/builder/state_machines.rs', '''                                    if bound_number_input_paths.contains_key(&condition.input)
                                        && condition
                                            .value
                                            .as_ref()
                                            .and_then(json_value_to_f32)
                                            .is_none()''', '''                                    if bound_number_input_paths.contains_key(&condition.input)
                                        && condition
                                            .value
                                            .as_ref()
                                            .and_then(json_value_to_f32)
                                            .filter(|value| value.is_finite())
                                            .is_none()''')
