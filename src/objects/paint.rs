use super::core::{Property, PropertyValue, RiveObject, property_keys, type_keys};

pub struct Fill {
    pub name: String,
    pub parent_id: u64,
    pub fill_rule: u64,
    pub is_visible: u64,
}

impl Fill {
    pub fn new(name: String, parent_id: u64) -> Self {
        Fill {
            name,
            parent_id,
            fill_rule: 0,
            is_visible: 1,
        }
    }
}

impl RiveObject for Fill {
    fn type_key(&self) -> u16 {
        type_keys::FILL
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
        if self.fill_rule != 0 {
            props.push(Property {
                key: property_keys::FILL_RULE,
                value: PropertyValue::UInt(self.fill_rule),
            });
        }
        if self.is_visible != 1 {
            props.push(Property {
                key: property_keys::SHAPE_PAINT_IS_VISIBLE,
                value: PropertyValue::UInt(self.is_visible),
            });
        }
        props
    }
}

pub struct Stroke {
    pub name: String,
    pub parent_id: u64,
    pub thickness: f32,
    pub cap: u64,
    pub join: u64,
    pub is_visible: u64,
    pub transform_affects: u64,
}

impl Stroke {
    pub fn new(name: String, parent_id: u64, thickness: f32) -> Self {
        Stroke {
            name,
            parent_id,
            thickness,
            cap: 0,
            join: 0,
            is_visible: 1,
            transform_affects: 0,
        }
    }
}

impl RiveObject for Stroke {
    fn type_key(&self) -> u16 {
        type_keys::STROKE
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
        if self.thickness != 0.0 {
            props.push(Property {
                key: property_keys::STROKE_THICKNESS,
                value: PropertyValue::Float(self.thickness),
            });
        }
        if self.cap != 0 {
            props.push(Property {
                key: property_keys::STROKE_CAP,
                value: PropertyValue::UInt(self.cap),
            });
        }
        if self.join != 0 {
            props.push(Property {
                key: property_keys::STROKE_JOIN,
                value: PropertyValue::UInt(self.join),
            });
        }
        if self.is_visible != 1 {
            props.push(Property {
                key: property_keys::SHAPE_PAINT_IS_VISIBLE,
                value: PropertyValue::UInt(self.is_visible),
            });
        }
        if self.transform_affects != 0 {
            props.push(Property {
                key: property_keys::STROKE_TRANSFORM_AFFECTS,
                value: PropertyValue::UInt(self.transform_affects),
            });
        }
        props
    }
}

pub struct SolidColor {
    pub name: String,
    pub parent_id: u64,
    pub color_value: u32,
}

impl SolidColor {
    pub fn new(name: String, parent_id: u64, color_value: u32) -> Self {
        SolidColor {
            name,
            parent_id,
            color_value,
        }
    }
}

impl RiveObject for SolidColor {
    fn type_key(&self) -> u16 {
        type_keys::SOLID_COLOR
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
        if self.color_value != 0 {
            props.push(Property {
                key: property_keys::SOLID_COLOR_VALUE,
                value: PropertyValue::Color(self.color_value),
            });
        }
        props
    }
}

pub struct LinearGradient {
    pub name: String,
    pub parent_id: u64,
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub opacity: f32,
}

impl RiveObject for LinearGradient {
    fn type_key(&self) -> u16 {
        type_keys::LINEAR_GRADIENT
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
        if self.start_x != 0.0 {
            props.push(Property {
                key: property_keys::LINEAR_GRADIENT_START_X,
                value: PropertyValue::Float(self.start_x),
            });
        }
        if self.start_y != 0.0 {
            props.push(Property {
                key: property_keys::LINEAR_GRADIENT_START_Y,
                value: PropertyValue::Float(self.start_y),
            });
        }
        if self.end_x != 0.0 {
            props.push(Property {
                key: property_keys::LINEAR_GRADIENT_END_X,
                value: PropertyValue::Float(self.end_x),
            });
        }
        if self.end_y != 0.0 {
            props.push(Property {
                key: property_keys::LINEAR_GRADIENT_END_Y,
                value: PropertyValue::Float(self.end_y),
            });
        }
        if self.opacity != 0.0 {
            props.push(Property {
                key: property_keys::LINEAR_GRADIENT_OPACITY,
                value: PropertyValue::Float(self.opacity),
            });
        }
        props
    }
}

pub struct RadialGradient {
    pub name: String,
    pub parent_id: u64,
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub opacity: f32,
}

impl RiveObject for RadialGradient {
    fn type_key(&self) -> u16 {
        type_keys::RADIAL_GRADIENT
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
        if self.start_x != 0.0 {
            props.push(Property {
                key: property_keys::LINEAR_GRADIENT_START_X,
                value: PropertyValue::Float(self.start_x),
            });
        }
        if self.start_y != 0.0 {
            props.push(Property {
                key: property_keys::LINEAR_GRADIENT_START_Y,
                value: PropertyValue::Float(self.start_y),
            });
        }
        if self.end_x != 0.0 {
            props.push(Property {
                key: property_keys::LINEAR_GRADIENT_END_X,
                value: PropertyValue::Float(self.end_x),
            });
        }
        if self.end_y != 0.0 {
            props.push(Property {
                key: property_keys::LINEAR_GRADIENT_END_Y,
                value: PropertyValue::Float(self.end_y),
            });
        }
        if self.opacity != 0.0 {
            props.push(Property {
                key: property_keys::LINEAR_GRADIENT_OPACITY,
                value: PropertyValue::Float(self.opacity),
            });
        }
        props
    }
}

pub struct GradientStop {
    pub name: String,
    pub parent_id: u64,
    pub color: u32,
    pub position: f32,
}

impl RiveObject for GradientStop {
    fn type_key(&self) -> u16 {
        type_keys::GRADIENT_STOP
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
        if self.color != 0 {
            props.push(Property {
                key: property_keys::GRADIENT_STOP_COLOR,
                value: PropertyValue::Color(self.color),
            });
        }
        if self.position != 0.0 {
            props.push(Property {
                key: property_keys::GRADIENT_STOP_POSITION,
                value: PropertyValue::Float(self.position),
            });
        }
        props
    }
}

pub struct DashPath {
    pub name: String,
    pub parent_id: u64,
    pub offset: f32,
    pub offset_is_percentage: bool,
}

impl DashPath {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            offset: 0.0,
            offset_is_percentage: false,
        }
    }
}

impl RiveObject for DashPath {
    fn type_key(&self) -> u16 {
        type_keys::DASH_PATH
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
                key: property_keys::DASH_PATH_OFFSET,
                value: PropertyValue::Float(self.offset),
            });
        }
        if self.offset_is_percentage {
            props.push(Property {
                key: property_keys::DASH_PATH_OFFSET_IS_PERCENTAGE,
                value: PropertyValue::Bool(true),
            });
        }
        props
    }
}

pub struct Dash {
    pub name: String,
    pub parent_id: u64,
    pub length: f32,
    pub length_is_percentage: bool,
}

impl Dash {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            length: 0.0,
            length_is_percentage: false,
        }
    }
}

impl RiveObject for Dash {
    fn type_key(&self) -> u16 {
        type_keys::DASH
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
        if self.length != 0.0 {
            props.push(Property {
                key: property_keys::DASH_LENGTH,
                value: PropertyValue::Float(self.length),
            });
        }
        if self.length_is_percentage {
            props.push(Property {
                key: property_keys::DASH_LENGTH_IS_PERCENTAGE,
                value: PropertyValue::Bool(true),
            });
        }
        props
    }
}

pub struct Feather {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub space_value: u64,
    pub inner: bool,
}

impl Feather {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            strength: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            space_value: 0,
            inner: false,
        }
    }
}

impl RiveObject for Feather {
    fn type_key(&self) -> u16 {
        type_keys::FEATHER
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
        if self.strength != 0.0 {
            props.push(Property {
                key: property_keys::FEATHER_STRENGTH,
                value: PropertyValue::Float(self.strength),
            });
        }
        if self.offset_x != 0.0 {
            props.push(Property {
                key: property_keys::FEATHER_OFFSET_X,
                value: PropertyValue::Float(self.offset_x),
            });
        }
        if self.offset_y != 0.0 {
            props.push(Property {
                key: property_keys::FEATHER_OFFSET_Y,
                value: PropertyValue::Float(self.offset_y),
            });
        }
        if self.space_value != 0 {
            props.push(Property {
                key: property_keys::FEATHER_SPACE_VALUE,
                value: PropertyValue::UInt(self.space_value),
            });
        }
        if self.inner {
            props.push(Property {
                key: property_keys::FEATHER_INNER,
                value: PropertyValue::Bool(true),
            });
        }
        props
    }
}

#[allow(dead_code)] // abstract base type from rive-runtime hierarchy
pub struct ShapePaint {
    pub name: String,
    pub parent_id: u64,
    pub is_visible: u64,
}

impl RiveObject for ShapePaint {
    fn type_key(&self) -> u16 {
        type_keys::SHAPE_PAINT
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
        if self.is_visible != 1 {
            props.push(Property {
                key: property_keys::SHAPE_PAINT_IS_VISIBLE,
                value: PropertyValue::UInt(self.is_visible),
            });
        }
        props
    }
}
