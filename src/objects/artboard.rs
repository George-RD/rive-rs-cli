use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct Backboard;

impl RiveObject for Backboard {
    fn type_key(&self) -> u16 {
        type_keys::BACKBOARD
    }

    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}

pub struct Artboard {
    pub name: String,
    #[allow(dead_code)] // artboards are implicitly children of the backboard
    pub parent_id: u64,
    pub width: f32,
    pub height: f32,
    pub origin_x: f32,
    pub origin_y: f32,
    pub x: f32,
    pub y: f32,
    pub default_state_machine_id: Option<u64>,
}

pub struct NestedArtboard {
    pub name: String,
    pub parent_id: u64,
    pub artboard_id: u64,
    pub x: f32,
    pub y: f32,
}

impl RiveObject for NestedArtboard {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_ARTBOARD
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
            Property {
                key: property_keys::NESTED_ARTBOARD_ARTBOARD_ID,
                value: PropertyValue::UInt(self.artboard_id),
            },
        ];

        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::NODE_X,
                value: PropertyValue::Float(self.x),
            });
        }

        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::NODE_Y,
                value: PropertyValue::Float(self.y),
            });
        }

        props
    }
}

impl Artboard {
    pub fn new(name: String, width: f32, height: f32) -> Self {
        Artboard {
            name,
            parent_id: 0,
            width,
            height,
            origin_x: 0.0,
            origin_y: 0.0,
            x: 0.0,
            y: 0.0,
            default_state_machine_id: None,
        }
    }
}

impl RiveObject for Artboard {
    fn type_key(&self) -> u16 {
        type_keys::ARTBOARD
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
            },
            Property {
                key: property_keys::LAYOUT_COMPONENT_WIDTH,
                value: PropertyValue::Float(self.width),
            },
            Property {
                key: property_keys::LAYOUT_COMPONENT_HEIGHT,
                value: PropertyValue::Float(self.height),
            },
        ];

        if self.origin_x != 0.0 {
            props.push(Property {
                key: property_keys::ARTBOARD_ORIGIN_X,
                value: PropertyValue::Float(self.origin_x),
            });
        }

        if self.origin_y != 0.0 {
            props.push(Property {
                key: property_keys::ARTBOARD_ORIGIN_Y,
                value: PropertyValue::Float(self.origin_y),
            });
        }

        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::NODE_X_ARTBOARD,
                value: PropertyValue::Float(self.x),
            });
        }

        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::NODE_Y_ARTBOARD,
                value: PropertyValue::Float(self.y),
            });
        }
        if let Some(sm_id) = self.default_state_machine_id {
            props.push(Property {
                key: property_keys::ARTBOARD_DEFAULT_STATE_MACHINE_ID,
                value: PropertyValue::UInt(sm_id),
            });
        }

        props
    }
}

pub struct NestedLinearAnimation {
    pub name: String,
    pub parent_id: u64,
    pub animation_id: u64,
    pub mix: f32,
}

impl RiveObject for NestedLinearAnimation {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_LINEAR_ANIMATION
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
            Property {
                key: property_keys::NESTED_ANIMATION_ID,
                value: PropertyValue::UInt(self.animation_id),
            },
        ];
        if self.mix != 1.0 {
            props.push(Property {
                key: property_keys::NESTED_MIX,
                value: PropertyValue::Float(self.mix),
            });
        }
        props
    }
}

pub struct NestedRemapAnimation {
    pub name: String,
    pub parent_id: u64,
    pub animation_id: u64,
    pub time: f32,
}

impl RiveObject for NestedRemapAnimation {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_REMAP_ANIMATION
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
            Property {
                key: property_keys::NESTED_ANIMATION_ID,
                value: PropertyValue::UInt(self.animation_id),
            },
        ];
        if self.time != 0.0 {
            props.push(Property {
                key: property_keys::NESTED_REMAP_TIME,
                value: PropertyValue::Float(self.time),
            });
        }
        props
    }
}

#[allow(dead_code)]
pub struct NestedInput {
    pub name: String,
    pub parent_id: u64,
    pub nested_input_id: u64,
}

impl RiveObject for NestedInput {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_INPUT
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
        if self.nested_input_id != 0 {
            props.push(Property {
                key: property_keys::NESTED_INPUT_ID,
                value: PropertyValue::UInt(self.nested_input_id),
            });
        }
        props
    }
}

pub struct NestedTrigger {
    pub name: String,
    pub parent_id: u64,
    pub nested_input_id: u64,
}

impl RiveObject for NestedTrigger {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_TRIGGER
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
        if self.nested_input_id != 0 {
            props.push(Property {
                key: property_keys::NESTED_INPUT_ID,
                value: PropertyValue::UInt(self.nested_input_id),
            });
        }
        props
    }
}

pub struct NestedBool {
    pub name: String,
    pub parent_id: u64,
    pub nested_input_id: u64,
    pub nested_value: bool,
}

impl RiveObject for NestedBool {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_BOOL
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
        if self.nested_input_id != 0 {
            props.push(Property {
                key: property_keys::NESTED_INPUT_ID,
                value: PropertyValue::UInt(self.nested_input_id),
            });
        }
        if self.nested_value {
            props.push(Property {
                key: property_keys::NESTED_BOOL_VALUE,
                value: PropertyValue::Bool(self.nested_value),
            });
        }
        props
    }
}

pub struct NestedNumber {
    pub name: String,
    pub parent_id: u64,
    pub nested_input_id: u64,
    pub nested_value: f32,
}

impl RiveObject for NestedNumber {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_NUMBER
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
        if self.nested_input_id != 0 {
            props.push(Property {
                key: property_keys::NESTED_INPUT_ID,
                value: PropertyValue::UInt(self.nested_input_id),
            });
        }
        if self.nested_value != 0.0 {
            props.push(Property {
                key: property_keys::NESTED_NUMBER_VALUE,
                value: PropertyValue::Float(self.nested_value),
            });
        }
        props
    }
}

pub struct NestedArtboardLeaf {
    pub name: String,
    pub parent_id: u64,
    pub artboard_id: u64,
    pub x: f32,
    pub y: f32,
}

impl RiveObject for NestedArtboardLeaf {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_ARTBOARD_LEAF
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
            Property {
                key: property_keys::NESTED_ARTBOARD_ARTBOARD_ID,
                value: PropertyValue::UInt(self.artboard_id),
            },
        ];
        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::NODE_X,
                value: PropertyValue::Float(self.x),
            });
        }
        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::NODE_Y,
                value: PropertyValue::Float(self.y),
            });
        }
        props
    }
}

pub struct NestedArtboardLayout {
    pub name: String,
    pub parent_id: u64,
    pub artboard_id: u64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub style_id: u64,
    pub instance_width: f32,
    pub instance_height: f32,
    pub instance_width_units_value: u64,
    pub instance_height_units_value: u64,
    pub instance_width_scale_type: u64,
    pub instance_height_scale_type: u64,
    pub fractional_width: f32,
    pub fractional_height: f32,
}

impl NestedArtboardLayout {
    pub fn new(name: String, parent_id: u64, artboard_id: u64) -> Self {
        Self {
            name,
            parent_id,
            artboard_id,
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            style_id: 0,
            instance_width: 0.0,
            instance_height: 0.0,
            instance_width_units_value: 0,
            instance_height_units_value: 0,
            instance_width_scale_type: 0,
            instance_height_scale_type: 0,
            fractional_width: 0.0,
            fractional_height: 0.0,
        }
    }
}

impl RiveObject for NestedArtboardLayout {
    fn type_key(&self) -> u16 {
        type_keys::NESTED_ARTBOARD_LAYOUT
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
            Property {
                key: property_keys::NESTED_ARTBOARD_ARTBOARD_ID,
                value: PropertyValue::UInt(self.artboard_id),
            },
        ];
        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::NODE_X,
                value: PropertyValue::Float(self.x),
            });
        }
        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::NODE_Y,
                value: PropertyValue::Float(self.y),
            });
        }
        if self.width != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_WIDTH,
                value: PropertyValue::Float(self.width),
            });
        }
        if self.height != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_HEIGHT,
                value: PropertyValue::Float(self.height),
            });
        }
        if self.style_id != 0 {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_STYLE_ID,
                value: PropertyValue::UInt(self.style_id),
            });
        }
        if self.instance_width != 0.0 {
            props.push(Property {
                key: property_keys::NESTED_ARTBOARD_LAYOUT_INSTANCE_WIDTH,
                value: PropertyValue::Float(self.instance_width),
            });
        }
        if self.instance_height != 0.0 {
            props.push(Property {
                key: property_keys::NESTED_ARTBOARD_LAYOUT_INSTANCE_HEIGHT,
                value: PropertyValue::Float(self.instance_height),
            });
        }
        if self.instance_width_units_value != 0 {
            props.push(Property {
                key: property_keys::NESTED_ARTBOARD_LAYOUT_INSTANCE_WIDTH_UNITS_VALUE,
                value: PropertyValue::UInt(self.instance_width_units_value),
            });
        }
        if self.instance_height_units_value != 0 {
            props.push(Property {
                key: property_keys::NESTED_ARTBOARD_LAYOUT_INSTANCE_HEIGHT_UNITS_VALUE,
                value: PropertyValue::UInt(self.instance_height_units_value),
            });
        }
        if self.instance_width_scale_type != 0 {
            props.push(Property {
                key: property_keys::NESTED_ARTBOARD_LAYOUT_INSTANCE_WIDTH_SCALE_TYPE,
                value: PropertyValue::UInt(self.instance_width_scale_type),
            });
        }
        if self.instance_height_scale_type != 0 {
            props.push(Property {
                key: property_keys::NESTED_ARTBOARD_LAYOUT_INSTANCE_HEIGHT_SCALE_TYPE,
                value: PropertyValue::UInt(self.instance_height_scale_type),
            });
        }
        if self.fractional_width != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_FRACTIONAL_WIDTH,
                value: PropertyValue::Float(self.fractional_width),
            });
        }
        if self.fractional_height != 0.0 {
            props.push(Property {
                key: property_keys::LAYOUT_COMPONENT_FRACTIONAL_HEIGHT,
                value: PropertyValue::Float(self.fractional_height),
            });
        }
        props
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backboard_type_key() {
        let backboard = Backboard;
        assert_eq!(backboard.type_key(), 23);
    }

    #[test]
    fn test_backboard_properties() {
        let backboard = Backboard;
        assert_eq!(backboard.properties(), vec![]);
    }

    #[test]
    fn test_artboard_type_key() {
        let artboard = Artboard::new("MyArtboard".to_string(), 500.0, 500.0);
        assert_eq!(artboard.type_key(), 1);
    }

    #[test]
    fn test_artboard_properties() {
        let artboard = Artboard::new("MyArtboard".to_string(), 500.0, 500.0);
        let props = artboard.properties();

        assert_eq!(props.len(), 3);

        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(
            props[0].value,
            PropertyValue::String("MyArtboard".to_string())
        );

        assert_eq!(props[1].key, property_keys::LAYOUT_COMPONENT_WIDTH);
        assert_eq!(props[1].value, PropertyValue::Float(500.0));

        assert_eq!(props[2].key, property_keys::LAYOUT_COMPONENT_HEIGHT);
        assert_eq!(props[2].value, PropertyValue::Float(500.0));
    }

    #[test]
    fn test_artboard_with_non_default_values() {
        let mut artboard = Artboard::new("Test".to_string(), 800.0, 600.0);
        artboard.origin_x = 10.0;
        artboard.origin_y = 20.0;
        artboard.x = 5.0;
        artboard.y = 15.0;

        let props = artboard.properties();

        assert_eq!(props.len(), 7);

        let keys: Vec<u16> = props.iter().map(|p| p.key).collect();
        assert!(keys.contains(&property_keys::ARTBOARD_ORIGIN_X));
        assert!(keys.contains(&property_keys::ARTBOARD_ORIGIN_Y));
        assert!(keys.contains(&property_keys::NODE_X_ARTBOARD));
        assert!(keys.contains(&property_keys::NODE_Y_ARTBOARD));
    }

    #[test]
    fn test_nested_artboard_type_key() {
        let nested = NestedArtboard {
            name: "nested".to_string(),
            parent_id: 2,
            artboard_id: 1,
            x: 0.0,
            y: 0.0,
        };
        assert_eq!(nested.type_key(), type_keys::NESTED_ARTBOARD);
    }

    #[test]
    fn test_nested_artboard_properties() {
        let nested = NestedArtboard {
            name: "embedded_component".to_string(),
            parent_id: 3,
            artboard_id: 1,
            x: 100.0,
            y: 200.0,
        };

        let props = nested.properties();
        assert_eq!(props.len(), 5);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(
            props[0].value,
            PropertyValue::String("embedded_component".to_string())
        );
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[1].value, PropertyValue::UInt(3));
        assert_eq!(props[2].key, property_keys::NESTED_ARTBOARD_ARTBOARD_ID);
        assert_eq!(props[2].value, PropertyValue::UInt(1));
        assert_eq!(props[3].key, property_keys::NODE_X);
        assert_eq!(props[3].value, PropertyValue::Float(100.0));
        assert_eq!(props[4].key, property_keys::NODE_Y);
        assert_eq!(props[4].value, PropertyValue::Float(200.0));
    }

    #[test]
    fn test_nested_artboard_zero_position_omitted() {
        let nested = NestedArtboard {
            name: "embedded_component".to_string(),
            parent_id: 3,
            artboard_id: 0,
            x: 0.0,
            y: 0.0,
        };

        let props = nested.properties();
        assert_eq!(props.len(), 3);
        assert!(!props.iter().any(|p| p.key == property_keys::NODE_X));
        assert!(!props.iter().any(|p| p.key == property_keys::NODE_Y));
    }

    #[test]
    fn test_nested_linear_animation_type_key() {
        let obj = NestedLinearAnimation {
            name: "anim".to_string(),
            parent_id: 1,
            animation_id: 0,
            mix: 1.0,
        };
        assert_eq!(obj.type_key(), 97);
        assert_eq!(obj.properties().len(), 3);
    }

    #[test]
    fn test_nested_linear_animation_mix() {
        let obj = NestedLinearAnimation {
            name: "anim".to_string(),
            parent_id: 1,
            animation_id: 0,
            mix: 0.5,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[3].key, property_keys::NESTED_MIX);
    }

    #[test]
    fn test_nested_remap_animation_type_key() {
        let obj = NestedRemapAnimation {
            name: "remap".to_string(),
            parent_id: 1,
            animation_id: 0,
            time: 0.0,
        };
        assert_eq!(obj.type_key(), 98);
        assert_eq!(obj.properties().len(), 3);
    }

    #[test]
    fn test_nested_remap_animation_time() {
        let obj = NestedRemapAnimation {
            name: "remap".to_string(),
            parent_id: 1,
            animation_id: 0,
            time: 2.5,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[3].key, property_keys::NESTED_REMAP_TIME);
    }

    #[test]
    fn test_nested_input_type_key() {
        let obj = NestedInput {
            name: "in".to_string(),
            parent_id: 1,
            nested_input_id: 0,
        };
        assert_eq!(obj.type_key(), 121);
    }

    #[test]
    fn test_nested_trigger_type_key() {
        let obj = NestedTrigger {
            name: "trig".to_string(),
            parent_id: 1,
            nested_input_id: 3,
        };
        assert_eq!(obj.type_key(), 122);
        let props = obj.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[2].key, property_keys::NESTED_INPUT_ID);
    }

    #[test]
    fn test_nested_bool_type_key() {
        let obj = NestedBool {
            name: "b".to_string(),
            parent_id: 1,
            nested_input_id: 2,
            nested_value: true,
        };
        assert_eq!(obj.type_key(), 123);
        let props = obj.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[3].key, property_keys::NESTED_BOOL_VALUE);
        assert_eq!(props[3].value, PropertyValue::Bool(true));
    }

    #[test]
    fn test_nested_bool_false_omitted() {
        let obj = NestedBool {
            name: "b".to_string(),
            parent_id: 1,
            nested_input_id: 0,
            nested_value: false,
        };
        let props = obj.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_nested_number_type_key() {
        let obj = NestedNumber {
            name: "n".to_string(),
            parent_id: 1,
            nested_input_id: 1,
            nested_value: 42.0,
        };
        assert_eq!(obj.type_key(), 124);
        let props = obj.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[3].key, property_keys::NESTED_NUMBER_VALUE);
    }

    #[test]
    fn test_nested_artboard_leaf_type_key() {
        let obj = NestedArtboardLeaf {
            name: "leaf".to_string(),
            parent_id: 0,
            artboard_id: 1,
            x: 0.0,
            y: 0.0,
        };
        assert_eq!(obj.type_key(), 451);
        assert_eq!(obj.properties().len(), 3);
    }

    #[test]
    fn test_nested_artboard_layout_type_key() {
        let obj = NestedArtboardLayout::new("layout".to_string(), 0, 1);
        assert_eq!(obj.type_key(), 452);
        assert_eq!(obj.properties().len(), 3);
    }

    #[test]
    fn test_nested_artboard_layout_with_values() {
        let mut obj = NestedArtboardLayout::new("layout".to_string(), 0, 1);
        obj.width = 300.0;
        obj.height = 200.0;
        obj.style_id = 5;
        let props = obj.properties();
        assert_eq!(props.len(), 6);
    }
}
