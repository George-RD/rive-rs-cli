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
                key: property_keys::LAYOUT_COMPONENT_WIDTH,
                value: PropertyValue::Float(self.width),
            },
            Property {
                key: property_keys::LAYOUT_COMPONENT_HEIGHT,
                value: PropertyValue::Float(self.height),
            },
            Property {
                key: property_keys::COMPONENT_NAME,
                value: PropertyValue::String(self.name.clone()),
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

        if let Some(sm_id) = self.default_state_machine_id {
            props.push(Property {
                key: property_keys::ARTBOARD_DEFAULT_STATE_MACHINE_ID,
                value: PropertyValue::UInt(sm_id),
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

        assert_eq!(props[0].key, property_keys::LAYOUT_COMPONENT_WIDTH);
        assert_eq!(props[0].value, PropertyValue::Float(500.0));

        assert_eq!(props[1].key, property_keys::LAYOUT_COMPONENT_HEIGHT);
        assert_eq!(props[1].value, PropertyValue::Float(500.0));

        assert_eq!(props[2].key, property_keys::COMPONENT_NAME);
        assert_eq!(
            props[2].value,
            PropertyValue::String("MyArtboard".to_string())
        );
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
        assert!(keys.contains(&property_keys::NODE_X));
        assert!(keys.contains(&property_keys::NODE_Y));
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
}
