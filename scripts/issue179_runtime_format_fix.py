from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    target = Path(path)
    text = target.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one replacement, found {count}")
    target.write_text(text.replace(old, new, 1))


replace_once(
    "src/objects/core.rs",
    "    pub const COMPONENT_PARENT_ID: u16 = 5;\n",
    "    pub const COMPONENT_PARENT_ID: u16 = 5;\n    pub const VIEW_MODEL_COMPONENT_NAME: u16 = 557;\n",
)
replace_once(
    "src/objects/core.rs",
    "        property_keys::COMPONENT_NAME\n        | property_keys::ANIMATION_NAME\n",
    "        property_keys::COMPONENT_NAME\n        | property_keys::VIEW_MODEL_COMPONENT_NAME\n        | property_keys::ANIMATION_NAME\n",
)

replace_once(
    "src/objects/data_binding.rs",
    """    fn properties(&self) -> Vec<Property> {
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
}

pub struct ViewModelProperty {""",
    """    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::VIEW_MODEL_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}

pub struct ViewModelProperty {""",
)
replace_once(
    "src/objects/data_binding.rs",
    """impl RiveObject for ViewModelPropertyBoolean {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_BOOLEAN
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
}
""",
    """impl RiveObject for ViewModelPropertyBoolean {
    fn type_key(&self) -> u16 {
        type_keys::VIEW_MODEL_PROPERTY_BOOLEAN
    }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::VIEW_MODEL_COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }]
    }
}
""",
)

state_machines = Path("src/builder/state_machines.rs")
text = state_machines.read_text()
old_import = """    BlendState1DChildSpec, BlendStateChildSpec, BlendStateDirectChildSpec, InputSpec,
    ListenerActionSpec, StateMachineComponentSpec, StateMachineSpec, StateSpec,
"""
new_import = """    BlendState1DChildSpec, BlendStateChildSpec, BlendStateDirectChildSpec, InputSpec,
    ListenerActionSpec, ObjectSpec, StateMachineComponentSpec, StateMachineSpec, StateSpec,
"""
if text.count(old_import) != 1:
    raise SystemExit("state-machine spec import shape changed")
text = text.replace(old_import, new_import, 1)

marker = "/// Builds all state machine objects for an artboard.\npub(crate) fn build_state_machines(\n"
helper = """fn view_model_property_name(object: &ObjectSpec) -> Option<&str> {
    match object {
        ObjectSpec::ViewModelProperty { name, .. }
        | ObjectSpec::ViewModelPropertyNumber { name, .. }
        | ObjectSpec::ViewModelPropertyBoolean { name, .. }
        | ObjectSpec::ViewModelPropertyString { name, .. }
        | ObjectSpec::ViewModelPropertyColor { name, .. }
        | ObjectSpec::ViewModelPropertyList { name, .. }
        | ObjectSpec::ViewModelPropertyViewModel { name, .. }
        | ObjectSpec::ViewModelPropertyEnum { name, .. }
        | ObjectSpec::ViewModelPropertyEnumCustom { name, .. }
        | ObjectSpec::ViewModelPropertyEnumSystem { name, .. }
        | ObjectSpec::ViewModelPropertyTrigger { name, .. }
        | ObjectSpec::ViewModelPropertyAssetImage { name, .. }
        | ObjectSpec::ViewModelPropertyArtboard { name, .. }
        | ObjectSpec::ViewModelPropertySymbol { name, .. }
        | ObjectSpec::ViewModelPropertySymbolListIndex { name, .. } => Some(name),
        _ => None,
    }
}

fn resolve_view_model_binding_ids(
    artboard_children: &[ObjectSpec],
    view_model_id_base: u64,
    view_model_name: &str,
    property_name: &str,
) -> Option<(u64, u64)> {
    let mut view_model_id = view_model_id_base;
    for child in artboard_children {
        let ObjectSpec::ViewModel { name, children } = child else {
            continue;
        };
        if name == view_model_name {
            let mut property_id = 0u64;
            for property in children.as_deref().unwrap_or_default() {
                if let Some(name) = view_model_property_name(property) {
                    if name == property_name {
                        return Some((view_model_id, property_id));
                    }
                    property_id += 1;
                }
            }
            return None;
        }
        view_model_id += 1;
    }
    None
}

/// Builds all state machine objects for an artboard.
pub(crate) fn build_state_machines(
"""
if text.count(marker) != 1:
    raise SystemExit("state-machine helper insertion point changed")
text = text.replace(marker, helper, 1)

old_signature = """    object_name_to_index: &HashMap<String, usize>,
    animation_name_to_index: &HashMap<String, usize>,
) -> Result<(), String> {"""
new_signature = """    object_name_to_index: &HashMap<String, usize>,
    animation_name_to_index: &HashMap<String, usize>,
    artboard_children: &[ObjectSpec],
    view_model_id_base: u64,
) -> Result<(), String> {"""
if text.count(old_signature) != 1:
    raise SystemExit("state-machine builder signature changed")
text = text.replace(old_signature, new_signature, 1)

old_resolution = """                            let view_model_global = *object_name_to_index
                                .get(&binding.view_model)
                                .ok_or_else(|| {
                                    format!(
                                        "unknown view model referenced by bool input '{}': '{}'",
                                        name, binding.view_model
                                    )
                                })?;
                            let property_global = *object_name_to_index
                                .get(&binding.property)
                                .ok_or_else(|| {
                                    format!(
                                        "unknown view-model property referenced by bool input '{}': '{}'",
                                        name, binding.property
                                    )
                                })?;
                            let view_model_id = view_model_global
                                .checked_sub(artboard_start)
                                .ok_or_else(|| {
                                    format!(
                                        "view model '{}' precedes current artboard",
                                        binding.view_model
                                    )
                                })? as u64;
                            let property_id =
                                property_global.checked_sub(artboard_start).ok_or_else(|| {
                                    format!(
                                        "view-model property '{}' precedes current artboard",
                                        binding.property
                                    )
                                })? as u64;
"""
new_resolution = """                            let (view_model_id, property_id) =
                                resolve_view_model_binding_ids(
                                    artboard_children,
                                    view_model_id_base,
                                    &binding.view_model,
                                    &binding.property,
                                )
                                .ok_or_else(|| {
                                    format!(
                                        "unknown view-model binding referenced by bool input '{}': '{}.{}'",
                                        name, binding.view_model, binding.property
                                    )
                                })?;
"""
if text.count(old_resolution) != 1:
    raise SystemExit("bool binding resolution shape changed")
state_machines.write_text(text.replace(old_resolution, new_resolution, 1))

scene = Path("src/builder/scene.rs")
text = scene.read_text()
old_loop = "    for artboard_spec in &artboard_specs {\n"
new_loop = "    let mut view_model_id_base = 0u64;\n    for artboard_spec in &artboard_specs {\n"
if text.count(old_loop) != 1:
    raise SystemExit("artboard build loop shape changed")
text = text.replace(old_loop, new_loop, 1)

old_call = """                &object_name_to_index,
                &animation_name_to_index,
            )?;
        }
    }

    Ok(objects)
}"""
new_call = """                &object_name_to_index,
                &animation_name_to_index,
                &artboard_spec.children,
                view_model_id_base,
            )?;
        }

        view_model_id_base += artboard_spec
            .children
            .iter()
            .filter(|child| matches!(child, ObjectSpec::ViewModel { .. }))
            .count() as u64;
    }

    Ok(objects)
}"""
if text.count(old_call) != 1:
    raise SystemExit("state-machine call site shape changed")
scene.write_text(text.replace(old_call, new_call, 1))
