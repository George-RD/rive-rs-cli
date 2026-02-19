use super::core::{property_keys, type_keys, Property, PropertyValue, RiveObject};

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
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(self.parent_id),
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

        assert_eq!(props.len(), 4);

        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(
            props[0].value,
            PropertyValue::String("MyArtboard".to_string())
        );

        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[1].value, PropertyValue::UInt(0));

        assert_eq!(props[2].key, property_keys::LAYOUT_COMPONENT_WIDTH);
        assert_eq!(props[2].value, PropertyValue::Float(500.0));

        assert_eq!(props[3].key, property_keys::LAYOUT_COMPONENT_HEIGHT);
        assert_eq!(props[3].value, PropertyValue::Float(500.0));
    }

    #[test]
    fn test_artboard_with_non_default_values() {
        let mut artboard = Artboard::new("Test".to_string(), 800.0, 600.0);
        artboard.origin_x = 10.0;
        artboard.origin_y = 20.0;
        artboard.x = 5.0;
        artboard.y = 15.0;

        let props = artboard.properties();

        assert_eq!(props.len(), 8);

        let keys: Vec<u16> = props.iter().map(|p| p.key).collect();
        assert!(keys.contains(&property_keys::ARTBOARD_ORIGIN_X));
        assert!(keys.contains(&property_keys::ARTBOARD_ORIGIN_Y));
        assert!(keys.contains(&property_keys::NODE_X));
        assert!(keys.contains(&property_keys::NODE_Y));
    }
}
