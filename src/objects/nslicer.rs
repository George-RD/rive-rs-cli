use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct NSlicerTileMode {
    pub name: String,
    pub parent_id: u64,
    pub patch_index: u64,
    pub style: u64,
}

impl NSlicerTileMode {
    pub fn new(name: String, parent_id: u64, patch_index: u64) -> Self {
        Self {
            name,
            parent_id,
            patch_index,
            style: 0,
        }
    }
}

impl RiveObject for NSlicerTileMode {
    fn type_key(&self) -> u16 {
        type_keys::NSLICER_TILE_MODE
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
                key: property_keys::NSLICER_TILE_MODE_PATCH_INDEX,
                value: PropertyValue::UInt(self.patch_index),
            },
        ];
        if self.style != 0 {
            props.push(Property {
                key: property_keys::NSLICER_TILE_MODE_STYLE,
                value: PropertyValue::UInt(self.style),
            });
        }
        props
    }
}

pub struct NSlicer {
    pub name: String,
    pub parent_id: u64,
    pub initial_width: f32,
    pub initial_height: f32,
    pub width: f32,
    pub height: f32,
}

impl NSlicer {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            initial_width: 0.0,
            initial_height: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl RiveObject for NSlicer {
    fn type_key(&self) -> u16 {
        type_keys::NSLICER
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
        if self.initial_width != 0.0 {
            props.push(Property {
                key: property_keys::NSLICER_INITIAL_WIDTH,
                value: PropertyValue::Float(self.initial_width),
            });
        }
        if self.initial_height != 0.0 {
            props.push(Property {
                key: property_keys::NSLICER_INITIAL_HEIGHT,
                value: PropertyValue::Float(self.initial_height),
            });
        }
        if self.width != 0.0 {
            props.push(Property {
                key: property_keys::NSLICER_WIDTH,
                value: PropertyValue::Float(self.width),
            });
        }
        if self.height != 0.0 {
            props.push(Property {
                key: property_keys::NSLICER_HEIGHT,
                value: PropertyValue::Float(self.height),
            });
        }
        props
    }
}

pub struct AxisY {
    pub name: String,
    pub parent_id: u64,
    pub offset: f32,
    pub normalized: bool,
}

impl AxisY {
    pub fn new(name: String, parent_id: u64, offset: f32) -> Self {
        Self {
            name,
            parent_id,
            offset,
            normalized: false,
        }
    }
}

impl RiveObject for AxisY {
    fn type_key(&self) -> u16 {
        type_keys::AXIS_Y
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
        if self.offset != 0.0 {
            props.push(Property {
                key: property_keys::AXIS_OFFSET,
                value: PropertyValue::Float(self.offset),
            });
        }
        if self.normalized {
            props.push(Property {
                key: property_keys::AXIS_NORMALIZED,
                value: PropertyValue::Bool(self.normalized),
            });
        }
        props
    }
}

pub struct AxisX {
    pub name: String,
    pub parent_id: u64,
    pub offset: f32,
    pub normalized: bool,
}

impl AxisX {
    pub fn new(name: String, parent_id: u64, offset: f32) -> Self {
        Self {
            name,
            parent_id,
            offset,
            normalized: false,
        }
    }
}

impl RiveObject for AxisX {
    fn type_key(&self) -> u16 {
        type_keys::AXIS_X
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
        if self.offset != 0.0 {
            props.push(Property {
                key: property_keys::AXIS_OFFSET,
                value: PropertyValue::Float(self.offset),
            });
        }
        if self.normalized {
            props.push(Property {
                key: property_keys::AXIS_NORMALIZED,
                value: PropertyValue::Bool(self.normalized),
            });
        }
        props
    }
}

pub struct NSlicedNode {
    pub name: String,
    pub parent_id: u64,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub opacity: f32,
}

impl NSlicedNode {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            opacity: 1.0,
        }
    }
}

impl RiveObject for NSlicedNode {
    fn type_key(&self) -> u16 {
        type_keys::NSLICED_NODE
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
        if self.opacity != 1.0 {
            props.push(Property {
                key: property_keys::WORLD_TRANSFORM_OPACITY,
                value: PropertyValue::Float(self.opacity),
            });
        }
        props
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nslicer_tile_mode_type_key() {
        let obj = NSlicerTileMode::new("tm".to_string(), 1, 4);
        assert_eq!(obj.type_key(), 491);
    }

    #[test]
    fn test_nslicer_tile_mode_properties() {
        let mut obj = NSlicerTileMode::new("tm".to_string(), 1, 4);
        obj.style = 1;
        let props = obj.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[2].key, property_keys::NSLICER_TILE_MODE_PATCH_INDEX);
        assert_eq!(props[2].value, PropertyValue::UInt(4));
        assert_eq!(props[3].key, property_keys::NSLICER_TILE_MODE_STYLE);
        assert_eq!(props[3].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_nslicer_type_key() {
        let obj = NSlicer::new("slicer".to_string(), 1);
        assert_eq!(obj.type_key(), 493);
    }

    #[test]
    fn test_nslicer_defaults() {
        let obj = NSlicer::new("slicer".to_string(), 1);
        assert_eq!(obj.properties().len(), 2);
    }

    #[test]
    fn test_nslicer_with_values() {
        let mut obj = NSlicer::new("slicer".to_string(), 1);
        obj.initial_width = 200.0;
        obj.initial_height = 80.0;
        obj.width = 400.0;
        obj.height = 80.0;
        let props = obj.properties();
        assert_eq!(props.len(), 6);
    }

    #[test]
    fn test_axis_y_type_key() {
        let obj = AxisY::new("ay".to_string(), 1, 30.0);
        assert_eq!(obj.type_key(), 494);
    }

    #[test]
    fn test_axis_y_properties() {
        let obj = AxisY::new("ay".to_string(), 1, 30.0);
        let props = obj.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[2].key, property_keys::AXIS_OFFSET);
        assert_eq!(props[2].value, PropertyValue::Float(30.0));
    }

    #[test]
    fn test_axis_y_normalized() {
        let mut obj = AxisY::new("ay".to_string(), 1, 0.5);
        obj.normalized = true;
        let props = obj.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[3].key, property_keys::AXIS_NORMALIZED);
    }

    #[test]
    fn test_axis_x_type_key() {
        let obj = AxisX::new("ax".to_string(), 1, 30.0);
        assert_eq!(obj.type_key(), 495);
    }

    #[test]
    fn test_nsliced_node_type_key() {
        let obj = NSlicedNode::new("node".to_string(), 0);
        assert_eq!(obj.type_key(), 508);
    }

    #[test]
    fn test_nsliced_node_defaults() {
        let obj = NSlicedNode::new("node".to_string(), 0);
        assert_eq!(obj.properties().len(), 2);
    }

    #[test]
    fn test_nsliced_node_with_values() {
        let mut obj = NSlicedNode::new("node".to_string(), 0);
        obj.x = 100.0;
        obj.y = 50.0;
        obj.opacity = 0.8;
        let props = obj.properties();
        assert_eq!(props.len(), 5);
    }
}
