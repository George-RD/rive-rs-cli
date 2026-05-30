use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct ScriptedDrawable {
    pub name: String,
    pub parent_id: u64,
    pub script_asset_id: u64,
    pub generator_function_ref: u64,
    pub threshold: f32,
    pub is_paused: bool,
    pub speed: f32,
    pub quantize: f32,
    pub interactive: bool,
}

impl RiveObject for ScriptedDrawable {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPTED_DRAWABLE
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.script_asset_id != 0 {
            props.push(Property {
                key: property_keys::SCRIPTED_DRAWABLE_SCRIPT_ASSET_ID,
                value: PropertyValue::UInt(self.script_asset_id),
            });
        }
        if self.generator_function_ref != 0 {
            props.push(Property {
                key: property_keys::SCRIPTED_DRAWABLE_GENERATOR_FUNCTION_REF,
                value: PropertyValue::UInt(self.generator_function_ref),
            });
        }
        if self.threshold != 0.0 {
            props.push(Property {
                key: property_keys::SCRIPTED_DRAWABLE_THRESHOLD,
                value: PropertyValue::Float(self.threshold),
            });
        }
        if self.is_paused {
            props.push(Property {
                key: property_keys::SCRIPTED_DRAWABLE_IS_PAUSED,
                value: PropertyValue::Bool(true),
            });
        }
        if self.speed != 1.0 {
            props.push(Property {
                key: property_keys::SCRIPTED_DRAWABLE_SPEED,
                value: PropertyValue::Float(self.speed),
            });
        }
        if self.quantize != 0.0 {
            props.push(Property {
                key: property_keys::SCRIPTED_DRAWABLE_QUANTIZE,
                value: PropertyValue::Float(self.quantize),
            });
        }
        if self.interactive {
            props.push(Property {
                key: property_keys::SCRIPTED_DRAWABLE_INTERACTIVE,
                value: PropertyValue::Bool(true),
            });
        }
        props
    }
}

pub struct ScriptedDataConverter {
    pub name: String,
    pub script_asset_id: u64,
}

impl RiveObject for ScriptedDataConverter {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPTED_DATA_CONVERTER
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if !self.name.is_empty() {
            props.push(Property {
                key: property_keys::DATA_CONVERTER_NAME,
                value: PropertyValue::String(self.name.clone()),
            });
        }
        if self.script_asset_id != 0 {
            props.push(Property {
                key: property_keys::SCRIPTED_DATA_CONVERTER_SCRIPT_ASSET_ID,
                value: PropertyValue::UInt(self.script_asset_id),
            });
        }
        props
    }
}

pub struct ScriptedLayout {
    pub name: String,
    pub parent_id: u64,
    pub script_asset_id: u64,
}

impl RiveObject for ScriptedLayout {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPTED_LAYOUT
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.script_asset_id != 0 {
            props.push(Property {
                key: property_keys::SCRIPTED_LAYOUT_SCRIPT_ASSET_ID,
                value: PropertyValue::UInt(self.script_asset_id),
            });
        }
        props
    }
}

pub struct ScriptedPathEffect {
    pub name: String,
    pub parent_id: u64,
    pub is_relative: bool,
    pub target_id: u64,
}

impl RiveObject for ScriptedPathEffect {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPTED_PATH_EFFECT
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.is_relative {
            props.push(Property {
                key: property_keys::SCRIPTED_PATH_EFFECT_IS_RELATIVE,
                value: PropertyValue::Bool(true),
            });
        }
        if self.target_id != 0 {
            props.push(Property {
                key: property_keys::SCRIPTED_PATH_EFFECT_TARGET_ID,
                value: PropertyValue::UInt(self.target_id),
            });
        }
        props
    }
}

pub struct ScriptedListenerAction {
    pub script_asset_id: u64,
    pub is_stateful: bool,
}

impl RiveObject for ScriptedListenerAction {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPTED_LISTENER_ACTION
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.script_asset_id != 0 {
            props.push(Property {
                key: property_keys::SCRIPTED_LISTENER_ACTION_SCRIPT_ASSET_ID,
                value: PropertyValue::UInt(self.script_asset_id),
            });
        }
        if self.is_stateful {
            props.push(Property {
                key: property_keys::SCRIPTED_IS_STATEFUL,
                value: PropertyValue::Bool(true),
            });
        }
        props
    }
}

pub struct ScriptedTransitionCondition {
    pub script_asset_id: u64,
    pub is_stateful: bool,
}

impl RiveObject for ScriptedTransitionCondition {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPTED_TRANSITION_CONDITION
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.script_asset_id != 0 {
            props.push(Property {
                key: property_keys::SCRIPTED_TRANSITION_CONDITION_SCRIPT_ASSET_ID,
                value: PropertyValue::UInt(self.script_asset_id),
            });
        }
        if self.is_stateful {
            props.push(Property {
                key: property_keys::SCRIPTED_IS_STATEFUL,
                value: PropertyValue::Bool(true),
            });
        }
        props
    }
}

pub struct ScriptInputNumber {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ScriptInputNumber {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPT_INPUT_NUMBER
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

pub struct ScriptInputViewModelProperty {
    pub name: String,
    pub parent_id: u64,
    pub view_model_id: u64,
}

impl RiveObject for ScriptInputViewModelProperty {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPT_INPUT_VIEW_MODEL_PROPERTY
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.view_model_id != 0 {
            props.push(Property {
                key: property_keys::SCRIPT_INPUT_VIEW_MODEL_PROPERTY_VIEW_MODEL_ID,
                value: PropertyValue::UInt(self.view_model_id),
            });
        }
        props
    }
}

pub struct ScriptInputTrigger {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ScriptInputTrigger {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPT_INPUT_TRIGGER
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

pub struct ScriptInputArtboard {
    pub name: String,
    pub parent_id: u64,
    pub artboard_id: u64,
}

impl RiveObject for ScriptInputArtboard {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPT_INPUT_ARTBOARD
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
            },
        ];
        if self.artboard_id != 0 {
            props.push(Property {
                key: property_keys::SCRIPT_INPUT_ARTBOARD_ARTBOARD_ID,
                value: PropertyValue::UInt(self.artboard_id),
            });
        }
        props
    }
}

pub struct ScriptInputColor {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ScriptInputColor {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPT_INPUT_COLOR
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

pub struct ScriptInputString {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ScriptInputString {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPT_INPUT_STRING
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

pub struct ScriptInputBoolean {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ScriptInputBoolean {
    fn type_key(&self) -> u16 {
        type_keys::SCRIPT_INPUT_BOOLEAN
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scripted_drawable_type_key() {
        let obj = ScriptedDrawable {
            name: "sd".to_string(),
            parent_id: 1,
            script_asset_id: 0,
            generator_function_ref: 0,
            threshold: 0.0,
            is_paused: false,
            speed: 1.0,
            quantize: 0.0,
            interactive: false,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPTED_DRAWABLE);
    }

    #[test]
    fn test_scripted_drawable_defaults() {
        let obj = ScriptedDrawable {
            name: "sd".to_string(),
            parent_id: 1,
            script_asset_id: 0,
            generator_function_ref: 0,
            threshold: 0.0,
            is_paused: false,
            speed: 1.0,
            quantize: 0.0,
            interactive: false,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
    }

    #[test]
    fn test_scripted_drawable_non_default() {
        let obj = ScriptedDrawable {
            name: "sd".to_string(),
            parent_id: 1,
            script_asset_id: 5,
            generator_function_ref: 0,
            threshold: 0.0,
            is_paused: true,
            speed: 1.0,
            quantize: 0.0,
            interactive: true,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 5);
        assert_eq!(
            props[2].key,
            property_keys::SCRIPTED_DRAWABLE_SCRIPT_ASSET_ID
        );
        assert_eq!(props[3].key, property_keys::SCRIPTED_DRAWABLE_IS_PAUSED);
        assert_eq!(props[3].value, PropertyValue::Bool(true));
        assert_eq!(props[4].key, property_keys::SCRIPTED_DRAWABLE_INTERACTIVE);
        assert_eq!(props[4].value, PropertyValue::Bool(true));
    }

    #[test]
    fn test_scripted_data_converter_type_key() {
        let obj = ScriptedDataConverter {
            name: "sdc".to_string(),
            script_asset_id: 0,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPTED_DATA_CONVERTER);
    }

    #[test]
    fn test_scripted_data_converter_properties() {
        let obj = ScriptedDataConverter {
            name: "sdc".to_string(),
            script_asset_id: 3,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::DATA_CONVERTER_NAME);
        assert_eq!(
            props[1].key,
            property_keys::SCRIPTED_DATA_CONVERTER_SCRIPT_ASSET_ID
        );
    }

    #[test]
    fn test_scripted_layout_type_key() {
        let obj = ScriptedLayout {
            name: "sl".to_string(),
            parent_id: 1,
            script_asset_id: 0,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPTED_LAYOUT);
    }

    #[test]
    fn test_scripted_layout_properties() {
        let obj = ScriptedLayout {
            name: "sl".to_string(),
            parent_id: 1,
            script_asset_id: 7,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[2].key, property_keys::SCRIPTED_LAYOUT_SCRIPT_ASSET_ID);
    }

    #[test]
    fn test_scripted_path_effect_type_key() {
        let obj = ScriptedPathEffect {
            name: "spe".to_string(),
            parent_id: 1,
            is_relative: false,
            target_id: 0,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPTED_PATH_EFFECT);
    }

    #[test]
    fn test_scripted_path_effect_non_default() {
        let obj = ScriptedPathEffect {
            name: "spe".to_string(),
            parent_id: 1,
            is_relative: true,
            target_id: 5,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(
            props[2].key,
            property_keys::SCRIPTED_PATH_EFFECT_IS_RELATIVE
        );
        assert_eq!(props[2].value, PropertyValue::Bool(true));
        assert_eq!(props[3].key, property_keys::SCRIPTED_PATH_EFFECT_TARGET_ID);
    }

    #[test]
    fn test_scripted_listener_action_type_key() {
        let obj = ScriptedListenerAction {
            script_asset_id: 0,
            is_stateful: false,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPTED_LISTENER_ACTION);
    }

    #[test]
    fn test_scripted_listener_action_no_name_parent() {
        let obj = ScriptedListenerAction {
            script_asset_id: 0,
            is_stateful: false,
        };
        let props = obj.properties();
        assert!(props.is_empty());
    }

    #[test]
    fn test_scripted_listener_action_non_default() {
        let obj = ScriptedListenerAction {
            script_asset_id: 10,
            is_stateful: true,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(
            props[0].key,
            property_keys::SCRIPTED_LISTENER_ACTION_SCRIPT_ASSET_ID
        );
        assert_eq!(props[1].key, property_keys::SCRIPTED_IS_STATEFUL);
        assert_eq!(props[1].value, PropertyValue::Bool(true));
    }

    #[test]
    fn test_scripted_transition_condition_type_key() {
        let obj = ScriptedTransitionCondition {
            script_asset_id: 0,
            is_stateful: false,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPTED_TRANSITION_CONDITION);
    }

    #[test]
    fn test_scripted_transition_condition_no_name_parent() {
        let obj = ScriptedTransitionCondition {
            script_asset_id: 0,
            is_stateful: false,
        };
        let props = obj.properties();
        assert!(props.is_empty());
    }

    #[test]
    fn test_script_input_number_type_key() {
        let obj = ScriptInputNumber {
            name: "num".to_string(),
            parent_id: 1,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPT_INPUT_NUMBER);
    }

    #[test]
    fn test_script_input_number_always_emits_name_parent() {
        let obj = ScriptInputNumber {
            name: "num".to_string(),
            parent_id: 1,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
    }

    #[test]
    fn test_script_input_view_model_property_type_key() {
        let obj = ScriptInputViewModelProperty {
            name: "vmp".to_string(),
            parent_id: 1,
            view_model_id: 0,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPT_INPUT_VIEW_MODEL_PROPERTY);
    }

    #[test]
    fn test_script_input_view_model_property_with_vm_id() {
        let obj = ScriptInputViewModelProperty {
            name: "vmp".to_string(),
            parent_id: 1,
            view_model_id: 5,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(
            props[2].key,
            property_keys::SCRIPT_INPUT_VIEW_MODEL_PROPERTY_VIEW_MODEL_ID
        );
    }

    #[test]
    fn test_script_input_trigger_type_key() {
        let obj = ScriptInputTrigger {
            name: "trig".to_string(),
            parent_id: 1,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPT_INPUT_TRIGGER);
    }

    #[test]
    fn test_script_input_artboard_type_key() {
        let obj = ScriptInputArtboard {
            name: "ab".to_string(),
            parent_id: 1,
            artboard_id: 0,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPT_INPUT_ARTBOARD);
    }

    #[test]
    fn test_script_input_artboard_with_id() {
        let obj = ScriptInputArtboard {
            name: "ab".to_string(),
            parent_id: 1,
            artboard_id: 3,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(
            props[2].key,
            property_keys::SCRIPT_INPUT_ARTBOARD_ARTBOARD_ID
        );
    }

    #[test]
    fn test_script_input_color_type_key() {
        let obj = ScriptInputColor {
            name: "col".to_string(),
            parent_id: 1,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPT_INPUT_COLOR);
    }

    #[test]
    fn test_script_input_string_type_key() {
        let obj = ScriptInputString {
            name: "str".to_string(),
            parent_id: 1,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPT_INPUT_STRING);
    }

    #[test]
    fn test_script_input_boolean_type_key() {
        let obj = ScriptInputBoolean {
            name: "bool".to_string(),
            parent_id: 1,
        };
        assert_eq!(obj.type_key(), type_keys::SCRIPT_INPUT_BOOLEAN);
    }
}
