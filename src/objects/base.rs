use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct Node {
    pub name: String,
    pub parent_id: u64,
    pub x: f32,
    pub y: f32,
}

pub struct Solo {
    pub name: String,
    pub parent_id: u64,
    pub x: f32,
    pub y: f32,
    pub active_component_id: u64,
}

impl RiveObject for Node {
    fn type_key(&self) -> u16 {
        type_keys::NODE
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

impl RiveObject for Solo {
    fn type_key(&self) -> u16 {
        type_keys::SOLO
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
        if self.active_component_id != 0 {
            props.push(Property {
                key: property_keys::SOLO_ACTIVE_COMPONENT_ID,
                value: PropertyValue::UInt(self.active_component_id),
            });
        }
        props
    }
}

#[allow(dead_code)] // abstract base type from rive-runtime hierarchy
pub struct TransformComponent {
    pub name: String,
    pub parent_id: u64,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl RiveObject for TransformComponent {
    fn type_key(&self) -> u16 {
        type_keys::TRANSFORM_COMPONENT
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
        if self.rotation != 0.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_ROTATION,
                value: PropertyValue::Float(self.rotation),
            });
        }
        if self.scale_x != 1.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_SCALE_X,
                value: PropertyValue::Float(self.scale_x),
            });
        }
        if self.scale_y != 1.0 {
            props.push(Property {
                key: property_keys::TRANSFORM_SCALE_Y,
                value: PropertyValue::Float(self.scale_y),
            });
        }
        props
    }
}

#[allow(dead_code)] // abstract base type from rive-runtime hierarchy
pub struct Drawable {
    pub name: String,
    pub parent_id: u64,
    pub blend_mode: u64,
    pub drawable_flags: u64,
}

impl RiveObject for Drawable {
    fn type_key(&self) -> u16 {
        type_keys::DRAWABLE
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
        if self.blend_mode != 0 {
            props.push(Property {
                key: property_keys::DRAWABLE_BLEND_MODE,
                value: PropertyValue::UInt(self.blend_mode),
            });
        }
        if self.drawable_flags != 0 {
            props.push(Property {
                key: property_keys::DRAWABLE_FLAGS,
                value: PropertyValue::UInt(self.drawable_flags),
            });
        }
        props
    }
}
