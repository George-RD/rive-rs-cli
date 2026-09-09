from pathlib import Path


def replace(path, old, new):
    file = Path(path)
    text = file.read_text()
    if old in text:
        assert text.count(old) == 1, (path, text.count(old))
        file.write_text(text.replace(old, new))
    else:
        assert new in text, path


path = 'src/builder/state_machines.rs'
replace(path, '''                        objects.push(Box::new(BlendState1DInput { input_id }));''', '''                        if append_bound_number_input(
                            Some(input_id),
                            state_machine.inputs.as_deref().unwrap_or_default(),
                            &bound_number_input_paths,
                            objects,
                        ) {
                            objects.push(Box::new(BlendState1DViewModel));
                        } else {
                            objects.push(Box::new(BlendState1DInput { input_id }));
                        }''')
replace(path, '''    let binding = input_id
        .and_then(|index| usize::try_from(index).ok())
        .and_then(|index| inputs.get(index))
        .and_then(|input| match input {
            InputSpec::Number { name, value, .. } => {
                bound_number_input_paths.get(name).map(|ids| (*ids, *value))
            }
            _ => None,
        });
    let mut input_id = input_id.unwrap_or(u32::MAX as u64);''', '''    let bound = blend_source.unwrap_or(DIRECT_BLEND_SOURCE_INPUT) == DIRECT_BLEND_SOURCE_INPUT
        && append_bound_number_input(*input_id, inputs, bound_number_input_paths, objects);
    let mut input_id = input_id.unwrap_or(u32::MAX as u64);''')
replace(path, '''    if blend_source == DIRECT_BLEND_SOURCE_INPUT
        && let Some(((view_model_id, property_id), value)) = binding
    {
        objects.push(Box::new(BindablePropertyNumber {
            property_value: value,
        }));
        objects.push(Box::new(DataBindContext::new(
            property_keys::BINDABLE_PROPERTY_NUMBER_VALUE as u64,
            0,
            encode_id_path(&[view_model_id, property_id]),
        )));
        input_id = u32::MAX as u64;''', '''    if bound {
        input_id = u32::MAX as u64;''')
replace(path, '''fn append_transition_child(
''', '''fn append_bound_number_input(
    input_id: Option<u64>,
    inputs: &[InputSpec],
    bound_number_input_paths: &HashMap<String, (u64, u64)>,
    objects: &mut Vec<Box<dyn RiveObject>>,
) -> bool {
    let binding = input_id
        .and_then(|index| usize::try_from(index).ok())
        .and_then(|index| inputs.get(index))
        .and_then(|input| match input {
            InputSpec::Number { name, value, .. } => {
                bound_number_input_paths.get(name).map(|ids| (*ids, *value))
            }
            _ => None,
        });
    let Some(((view_model_id, property_id), value)) = binding else {
        return false;
    };
    objects.push(Box::new(BindablePropertyNumber {
        property_value: value,
    }));
    objects.push(Box::new(DataBindContext::new(
        property_keys::BINDABLE_PROPERTY_NUMBER_VALUE as u64,
        0,
        encode_id_path(&[view_model_id, property_id]),
    )));
    true
}

fn append_transition_child(
''')
