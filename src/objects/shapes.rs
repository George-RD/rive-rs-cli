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

pub struct Shape {
    pub name: String,
    pub parent_id: u64,
    pub x: f32,
    pub y: f32,
}

impl Shape {
    pub fn new(name: String, parent_id: u64) -> Self {
        Shape {
            name,
            parent_id,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl RiveObject for Shape {
    fn type_key(&self) -> u16 {
        type_keys::SHAPE
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

pub struct Ellipse {
    pub name: String,
    pub parent_id: u64,
    pub width: f32,
    pub height: f32,
    pub origin_x: f32,
    pub origin_y: f32,
}

impl Ellipse {
    pub fn new(name: String, parent_id: u64, width: f32, height: f32) -> Self {
        Ellipse {
            name,
            parent_id,
            width,
            height,
            origin_x: 0.0,
            origin_y: 0.0,
        }
    }
}

impl RiveObject for Ellipse {
    fn type_key(&self) -> u16 {
        type_keys::ELLIPSE
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
                key: property_keys::PARAMETRIC_PATH_WIDTH,
                value: PropertyValue::Float(self.width),
            },
            Property {
                key: property_keys::PARAMETRIC_PATH_HEIGHT,
                value: PropertyValue::Float(self.height),
            },
        ];
        if self.origin_x != 0.0 {
            props.push(Property {
                key: property_keys::PARAMETRIC_PATH_ORIGIN_X,
                value: PropertyValue::Float(self.origin_x),
            });
        }
        if self.origin_y != 0.0 {
            props.push(Property {
                key: property_keys::PARAMETRIC_PATH_ORIGIN_Y,
                value: PropertyValue::Float(self.origin_y),
            });
        }
        props
    }
}

pub struct Rectangle {
    pub name: String,
    pub parent_id: u64,
    pub width: f32,
    pub height: f32,
    pub origin_x: f32,
    pub origin_y: f32,
    pub corner_radius_tl: f32,
    pub corner_radius_tr: f32,
    pub corner_radius_bl: f32,
    pub corner_radius_br: f32,
    pub link_corner_radius: u64,
}

pub struct Triangle {
    pub name: String,
    pub parent_id: u64,
    pub width: f32,
    pub height: f32,
    pub origin_x: f32,
    pub origin_y: f32,
}

impl Rectangle {
    pub fn new(name: String, parent_id: u64, width: f32, height: f32) -> Self {
        Rectangle {
            name,
            parent_id,
            width,
            height,
            origin_x: 0.0,
            origin_y: 0.0,
            corner_radius_tl: 0.0,
            corner_radius_tr: 0.0,
            corner_radius_bl: 0.0,
            corner_radius_br: 0.0,
            link_corner_radius: 0,
        }
    }
}

impl RiveObject for Rectangle {
    fn type_key(&self) -> u16 {
        type_keys::RECTANGLE
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
                key: property_keys::PARAMETRIC_PATH_WIDTH,
                value: PropertyValue::Float(self.width),
            },
            Property {
                key: property_keys::PARAMETRIC_PATH_HEIGHT,
                value: PropertyValue::Float(self.height),
            },
        ];
        if self.origin_x != 0.0 {
            props.push(Property {
                key: property_keys::PARAMETRIC_PATH_ORIGIN_X,
                value: PropertyValue::Float(self.origin_x),
            });
        }
        if self.origin_y != 0.0 {
            props.push(Property {
                key: property_keys::PARAMETRIC_PATH_ORIGIN_Y,
                value: PropertyValue::Float(self.origin_y),
            });
        }
        if self.corner_radius_tl != 0.0 {
            props.push(Property {
                key: property_keys::RECTANGLE_CORNER_RADIUS_TL,
                value: PropertyValue::Float(self.corner_radius_tl),
            });
        }
        if self.corner_radius_tr != 0.0 {
            props.push(Property {
                key: property_keys::RECTANGLE_CORNER_RADIUS_TR,
                value: PropertyValue::Float(self.corner_radius_tr),
            });
        }
        if self.corner_radius_bl != 0.0 {
            props.push(Property {
                key: property_keys::RECTANGLE_CORNER_RADIUS_BL,
                value: PropertyValue::Float(self.corner_radius_bl),
            });
        }
        if self.corner_radius_br != 0.0 {
            props.push(Property {
                key: property_keys::RECTANGLE_CORNER_RADIUS_BR,
                value: PropertyValue::Float(self.corner_radius_br),
            });
        }
        if self.link_corner_radius != 0 {
            props.push(Property {
                key: property_keys::RECTANGLE_LINK_CORNER_RADIUS,
                value: PropertyValue::UInt(self.link_corner_radius),
            });
        }
        props
    }
}

impl Triangle {
    pub fn new(name: String, parent_id: u64, width: f32, height: f32) -> Self {
        Triangle {
            name,
            parent_id,
            width,
            height,
            origin_x: 0.5,
            origin_y: 0.5,
        }
    }
}

impl RiveObject for Triangle {
    fn type_key(&self) -> u16 {
        type_keys::TRIANGLE
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
                key: property_keys::PARAMETRIC_PATH_WIDTH,
                value: PropertyValue::Float(self.width),
            },
            Property {
                key: property_keys::PARAMETRIC_PATH_HEIGHT,
                value: PropertyValue::Float(self.height),
            },
        ];
        if self.origin_x != 0.5 {
            props.push(Property {
                key: property_keys::PARAMETRIC_PATH_ORIGIN_X,
                value: PropertyValue::Float(self.origin_x),
            });
        }
        if self.origin_y != 0.5 {
            props.push(Property {
                key: property_keys::PARAMETRIC_PATH_ORIGIN_Y,
                value: PropertyValue::Float(self.origin_y),
            });
        }
        props
    }
}

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

pub struct PathObject {
    pub name: String,
    pub parent_id: u64,
    pub path_flags: u64,
}

pub struct PointsPathObject {
    pub name: String,
    pub parent_id: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub is_closed: bool,
    pub path_flags: u32,
}

impl RiveObject for PointsPathObject {
    fn type_key(&self) -> u16 {
        type_keys::POINTS_PATH
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }];
        if let Some(parent_id) = self.parent_id {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(parent_id as u64),
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
        if self.is_closed {
            props.push(Property {
                key: property_keys::POINTS_PATH_IS_CLOSED,
                value: PropertyValue::UInt(1),
            });
        }
        if self.path_flags != 0 {
            props.push(Property {
                key: property_keys::PATH_FLAGS,
                value: PropertyValue::UInt(self.path_flags as u64),
            });
        }
        props
    }
}

pub struct StraightVertexObject {
    pub name: String,
    pub parent_id: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl RiveObject for StraightVertexObject {
    fn type_key(&self) -> u16 {
        type_keys::STRAIGHT_VERTEX
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }];
        if let Some(parent_id) = self.parent_id {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(parent_id as u64),
            });
        }
        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::VERTEX_X,
                value: PropertyValue::Float(self.x),
            });
        }
        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::VERTEX_Y,
                value: PropertyValue::Float(self.y),
            });
        }
        if self.radius != 0.0 {
            props.push(Property {
                key: property_keys::STRAIGHT_VERTEX_RADIUS,
                value: PropertyValue::Float(self.radius),
            });
        }
        props
    }
}

pub struct CubicMirroredVertexObject {
    pub name: String,
    pub parent_id: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub distance: f32,
}

impl RiveObject for CubicMirroredVertexObject {
    fn type_key(&self) -> u16 {
        type_keys::CUBIC_MIRRORED_VERTEX
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }];
        if let Some(parent_id) = self.parent_id {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(parent_id as u64),
            });
        }
        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::VERTEX_X,
                value: PropertyValue::Float(self.x),
            });
        }
        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::VERTEX_Y,
                value: PropertyValue::Float(self.y),
            });
        }
        if self.rotation != 0.0 {
            props.push(Property {
                key: property_keys::CUBIC_MIRRORED_VERTEX_ROTATION,
                value: PropertyValue::Float(self.rotation),
            });
        }
        if self.distance != 0.0 {
            props.push(Property {
                key: property_keys::CUBIC_MIRRORED_VERTEX_DISTANCE,
                value: PropertyValue::Float(self.distance),
            });
        }
        props
    }
}

pub struct CubicDetachedVertexObject {
    pub name: String,
    pub parent_id: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub in_rotation: f32,
    pub in_distance: f32,
    pub out_rotation: f32,
    pub out_distance: f32,
}

impl RiveObject for CubicDetachedVertexObject {
    fn type_key(&self) -> u16 {
        type_keys::CUBIC_DETACHED_VERTEX
    }

    fn properties(&self) -> Vec<Property> {
        let mut props = vec![Property {
            key: property_keys::COMPONENT_NAME,
            value: PropertyValue::String(self.name.clone()),
        }];
        if let Some(parent_id) = self.parent_id {
            props.push(Property {
                key: property_keys::COMPONENT_PARENT_ID,
                value: PropertyValue::UInt(parent_id as u64),
            });
        }
        if self.x != 0.0 {
            props.push(Property {
                key: property_keys::VERTEX_X,
                value: PropertyValue::Float(self.x),
            });
        }
        if self.y != 0.0 {
            props.push(Property {
                key: property_keys::VERTEX_Y,
                value: PropertyValue::Float(self.y),
            });
        }
        if self.in_rotation != 0.0 {
            props.push(Property {
                key: property_keys::CUBIC_DETACHED_VERTEX_IN_ROTATION,
                value: PropertyValue::Float(self.in_rotation),
            });
        }
        if self.in_distance != 0.0 {
            props.push(Property {
                key: property_keys::CUBIC_DETACHED_VERTEX_IN_DISTANCE,
                value: PropertyValue::Float(self.in_distance),
            });
        }
        if self.out_rotation != 0.0 {
            props.push(Property {
                key: property_keys::CUBIC_DETACHED_VERTEX_OUT_ROTATION,
                value: PropertyValue::Float(self.out_rotation),
            });
        }
        if self.out_distance != 0.0 {
            props.push(Property {
                key: property_keys::CUBIC_DETACHED_VERTEX_OUT_DISTANCE,
                value: PropertyValue::Float(self.out_distance),
            });
        }
        props
    }
}

impl RiveObject for PathObject {
    fn type_key(&self) -> u16 {
        type_keys::PATH
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
        if self.path_flags != 0 {
            props.push(Property {
                key: property_keys::PATH_FLAGS,
                value: PropertyValue::UInt(self.path_flags),
            });
        }
        props
    }
}

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

pub struct Image {
    pub name: String,
    pub parent_id: u64,
    pub asset_id: u64,
    pub x: f32,
    pub y: f32,
}

impl Image {
    pub fn new(name: String, parent_id: u64, asset_id: u64) -> Self {
        Image {
            name,
            parent_id,
            asset_id,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl RiveObject for Image {
    fn type_key(&self) -> u16 {
        type_keys::IMAGE
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
                key: property_keys::IMAGE_ASSET_ID,
                value: PropertyValue::UInt(self.asset_id),
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

pub struct TrimPath {
    pub name: String,
    pub parent_id: u64,
    pub start: f32,
    pub end: f32,
    pub offset: f32,
    pub(crate) mode_value: u64,
}

impl TrimPath {
    pub fn new(name: String, parent_id: u64) -> Self {
        Self {
            name,
            parent_id,
            start: 0.0,
            end: 0.0,
            offset: 0.0,
            mode_value: 1,
        }
    }

    pub fn set_mode(&mut self, mode: u64) -> Result<(), String> {
        if mode != 1 && mode != 2 {
            return Err(format!(
                "TrimPath mode must be 1 (sequential) or 2 (synchronized), got {}",
                mode
            ));
        }
        self.mode_value = mode;
        Ok(())
    }
}

impl RiveObject for TrimPath {
    fn type_key(&self) -> u16 {
        type_keys::TRIM_PATH
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
        if self.start != 0.0 {
            props.push(Property {
                key: property_keys::TRIM_PATH_START,
                value: PropertyValue::Float(self.start),
            });
        }
        if self.end != 0.0 {
            props.push(Property {
                key: property_keys::TRIM_PATH_END,
                value: PropertyValue::Float(self.end),
            });
        }
        if self.offset != 0.0 {
            props.push(Property {
                key: property_keys::TRIM_PATH_OFFSET,
                value: PropertyValue::Float(self.offset),
            });
        }
        props.push(Property {
            key: property_keys::TRIM_PATH_MODE_VALUE,
            value: PropertyValue::UInt(self.mode_value),
        });
        props
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_type_key() {
        let shape = Shape::new("TestShape".to_string(), 1);
        assert_eq!(shape.type_key(), 3);
    }

    #[test]
    fn test_shape_properties() {
        let shape = Shape::new("MyShape".to_string(), 2);
        let props = shape.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[0].value, PropertyValue::String("MyShape".to_string()));
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[1].value, PropertyValue::UInt(2));
    }

    #[test]
    fn test_shape_properties_with_position() {
        let mut shape = Shape::new("Centered".to_string(), 0);
        shape.x = 250.0;
        shape.y = 300.0;
        let props = shape.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[2].key, property_keys::NODE_X);
        assert_eq!(props[2].value, PropertyValue::Float(250.0));
        assert_eq!(props[3].key, property_keys::NODE_Y);
        assert_eq!(props[3].value, PropertyValue::Float(300.0));
    }

    #[test]
    fn test_shape_zero_position_omitted() {
        let shape = Shape::new("Default".to_string(), 0);
        let props = shape.properties();
        assert_eq!(props.len(), 2);
        assert!(!props.iter().any(|p| p.key == property_keys::NODE_X));
        assert!(!props.iter().any(|p| p.key == property_keys::NODE_Y));
    }

    #[test]
    fn test_ellipse_type_key() {
        let ellipse = Ellipse::new("E".to_string(), 1, 100.0, 50.0);
        assert_eq!(ellipse.type_key(), 4);
    }

    #[test]
    fn test_ellipse_properties_defaults() {
        let ellipse = Ellipse::new("Circle".to_string(), 3, 200.0, 200.0);
        let props = ellipse.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[0].value, PropertyValue::String("Circle".to_string()));
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[1].value, PropertyValue::UInt(3));
        assert_eq!(props[2].key, property_keys::PARAMETRIC_PATH_WIDTH);
        assert_eq!(props[2].value, PropertyValue::Float(200.0));
        assert_eq!(props[3].key, property_keys::PARAMETRIC_PATH_HEIGHT);
        assert_eq!(props[3].value, PropertyValue::Float(200.0));
    }

    #[test]
    fn test_ellipse_properties_with_origin() {
        let mut ellipse = Ellipse::new("E".to_string(), 1, 100.0, 50.0);
        ellipse.origin_x = 0.5;
        ellipse.origin_y = 0.5;
        let props = ellipse.properties();
        assert_eq!(props.len(), 6);
        assert_eq!(props[4].key, property_keys::PARAMETRIC_PATH_ORIGIN_X);
        assert_eq!(props[4].value, PropertyValue::Float(0.5));
        assert_eq!(props[5].key, property_keys::PARAMETRIC_PATH_ORIGIN_Y);
        assert_eq!(props[5].value, PropertyValue::Float(0.5));
    }

    #[test]
    fn test_rectangle_type_key() {
        let rect = Rectangle::new("R".to_string(), 1, 100.0, 50.0);
        assert_eq!(rect.type_key(), 7);
    }

    #[test]
    fn test_rectangle_properties_defaults() {
        let rect = Rectangle::new("Rect".to_string(), 2, 300.0, 150.0);
        let props = rect.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[2].key, property_keys::PARAMETRIC_PATH_WIDTH);
        assert_eq!(props[2].value, PropertyValue::Float(300.0));
        assert_eq!(props[3].key, property_keys::PARAMETRIC_PATH_HEIGHT);
        assert_eq!(props[3].value, PropertyValue::Float(150.0));
    }

    #[test]
    fn test_rectangle_properties_with_corners() {
        let mut rect = Rectangle::new("R".to_string(), 1, 100.0, 100.0);
        rect.corner_radius_tl = 10.0;
        rect.corner_radius_tr = 20.0;
        rect.corner_radius_bl = 30.0;
        rect.corner_radius_br = 40.0;
        rect.link_corner_radius = 1;
        let props = rect.properties();
        assert_eq!(props.len(), 9);
        let keys: Vec<u16> = props.iter().map(|p| p.key).collect();
        assert!(keys.contains(&property_keys::RECTANGLE_CORNER_RADIUS_TL));
        assert!(keys.contains(&property_keys::RECTANGLE_CORNER_RADIUS_TR));
        assert!(keys.contains(&property_keys::RECTANGLE_CORNER_RADIUS_BL));
        assert!(keys.contains(&property_keys::RECTANGLE_CORNER_RADIUS_BR));
        assert!(keys.contains(&property_keys::RECTANGLE_LINK_CORNER_RADIUS));
    }

    #[test]
    fn test_triangle_type_key() {
        let tri = Triangle::new("T".to_string(), 1, 100.0, 50.0);
        assert_eq!(tri.type_key(), type_keys::TRIANGLE);
    }

    #[test]
    fn test_triangle_properties_defaults() {
        let tri = Triangle::new("Tri".to_string(), 2, 300.0, 150.0);
        let props = tri.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[2].key, property_keys::PARAMETRIC_PATH_WIDTH);
        assert_eq!(props[2].value, PropertyValue::Float(300.0));
        assert_eq!(props[3].key, property_keys::PARAMETRIC_PATH_HEIGHT);
        assert_eq!(props[3].value, PropertyValue::Float(150.0));
    }

    #[test]
    fn test_triangle_properties_with_origin() {
        let mut tri = Triangle::new("Tri".to_string(), 2, 300.0, 150.0);
        tri.origin_x = 0.0;
        tri.origin_y = 1.0;
        let props = tri.properties();
        assert_eq!(props.len(), 6);
        assert_eq!(props[4].key, property_keys::PARAMETRIC_PATH_ORIGIN_X);
        assert_eq!(props[4].value, PropertyValue::Float(0.0));
        assert_eq!(props[5].key, property_keys::PARAMETRIC_PATH_ORIGIN_Y);
        assert_eq!(props[5].value, PropertyValue::Float(1.0));
    }

    #[test]
    fn test_fill_type_key() {
        let fill = Fill::new("F".to_string(), 1);
        assert_eq!(fill.type_key(), 20);
    }

    #[test]
    fn test_fill_properties_defaults() {
        let fill = Fill::new("Fill".to_string(), 5);
        let props = fill.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[1].value, PropertyValue::UInt(5));
    }

    #[test]
    fn test_fill_invisible() {
        let mut fill = Fill::new("F".to_string(), 1);
        fill.is_visible = 0;
        let props = fill.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[2].key, property_keys::SHAPE_PAINT_IS_VISIBLE);
        assert_eq!(props[2].value, PropertyValue::UInt(0));
    }

    #[test]
    fn test_fill_properties_with_fill_rule() {
        let mut fill = Fill::new("F".to_string(), 1);
        fill.fill_rule = 1;
        let props = fill.properties();
        assert_eq!(props.len(), 3);
        let keys: Vec<u16> = props.iter().map(|p| p.key).collect();
        assert!(keys.contains(&property_keys::FILL_RULE));
    }

    #[test]
    fn test_stroke_type_key() {
        let stroke = Stroke::new("S".to_string(), 1, 2.0);
        assert_eq!(stroke.type_key(), 24);
    }

    #[test]
    fn test_stroke_properties_defaults() {
        let stroke = Stroke::new("Stroke".to_string(), 3, 5.0);
        let props = stroke.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[1].value, PropertyValue::UInt(3));
        assert_eq!(props[2].key, property_keys::STROKE_THICKNESS);
        assert_eq!(props[2].value, PropertyValue::Float(5.0));
    }

    #[test]
    fn test_stroke_invisible() {
        let mut stroke = Stroke::new("S".to_string(), 1, 2.0);
        stroke.is_visible = 0;
        let props = stroke.properties();
        assert_eq!(props.len(), 4);
        assert!(
            props
                .iter()
                .any(|p| p.key == property_keys::SHAPE_PAINT_IS_VISIBLE
                    && p.value == PropertyValue::UInt(0))
        );
    }

    #[test]
    fn test_stroke_properties_all_set() {
        let mut stroke = Stroke::new("S".to_string(), 1, 3.0);
        stroke.cap = 1;
        stroke.join = 2;
        stroke.transform_affects = 1;
        let props = stroke.properties();
        assert_eq!(props.len(), 6);
        let keys: Vec<u16> = props.iter().map(|p| p.key).collect();
        assert!(keys.contains(&property_keys::STROKE_CAP));
        assert!(keys.contains(&property_keys::STROKE_JOIN));
        assert!(keys.contains(&property_keys::STROKE_TRANSFORM_AFFECTS));
    }

    #[test]
    fn test_solid_color_type_key() {
        let sc = SolidColor::new("SC".to_string(), 1, 0xFF0000FF);
        assert_eq!(sc.type_key(), 18);
    }

    #[test]
    fn test_solid_color_properties() {
        let sc = SolidColor::new("Red".to_string(), 4, 0xFFFF0000);
        let props = sc.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[1].value, PropertyValue::UInt(4));
        assert_eq!(props[2].key, property_keys::SOLID_COLOR_VALUE);
        assert_eq!(props[2].value, PropertyValue::Color(0xFFFF0000));
    }

    #[test]
    fn test_solid_color_zero_color_skipped() {
        let sc = SolidColor::new("Transparent".to_string(), 1, 0);
        let props = sc.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_linear_gradient_type_key() {
        let lg = LinearGradient {
            name: "LG".to_string(),
            parent_id: 1,
            start_x: 0.0,
            start_y: 0.0,
            end_x: 100.0,
            end_y: 100.0,
            opacity: 1.0,
        };
        assert_eq!(lg.type_key(), 22);
    }

    #[test]
    fn test_linear_gradient_properties() {
        let lg = LinearGradient {
            name: "Grad".to_string(),
            parent_id: 2,
            start_x: 10.0,
            start_y: 20.0,
            end_x: 30.0,
            end_y: 40.0,
            opacity: 0.8,
        };
        let props = lg.properties();
        assert_eq!(props.len(), 7);
        assert_eq!(props[2].key, property_keys::LINEAR_GRADIENT_START_X);
        assert_eq!(props[2].value, PropertyValue::Float(10.0));
        assert_eq!(props[3].key, property_keys::LINEAR_GRADIENT_START_Y);
        assert_eq!(props[3].value, PropertyValue::Float(20.0));
        assert_eq!(props[4].key, property_keys::LINEAR_GRADIENT_END_X);
        assert_eq!(props[4].value, PropertyValue::Float(30.0));
        assert_eq!(props[5].key, property_keys::LINEAR_GRADIENT_END_Y);
        assert_eq!(props[5].value, PropertyValue::Float(40.0));
        assert_eq!(props[6].key, property_keys::LINEAR_GRADIENT_OPACITY);
        assert_eq!(props[6].value, PropertyValue::Float(0.8));
    }

    #[test]
    fn test_linear_gradient_defaults_skipped() {
        let lg = LinearGradient {
            name: "LG".to_string(),
            parent_id: 1,
            start_x: 0.0,
            start_y: 0.0,
            end_x: 0.0,
            end_y: 0.0,
            opacity: 0.0,
        };
        let props = lg.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_gradient_stop_type_key() {
        let gs = GradientStop {
            name: "GS".to_string(),
            parent_id: 1,
            color: 0xFF0000FF,
            position: 0.5,
        };
        assert_eq!(gs.type_key(), 19);
    }

    #[test]
    fn test_gradient_stop_properties() {
        let gs = GradientStop {
            name: "Stop".to_string(),
            parent_id: 3,
            color: 0xFF00FF00,
            position: 0.75,
        };
        let props = gs.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[2].key, property_keys::GRADIENT_STOP_COLOR);
        assert_eq!(props[2].value, PropertyValue::Color(0xFF00FF00));
        assert_eq!(props[3].key, property_keys::GRADIENT_STOP_POSITION);
        assert_eq!(props[3].value, PropertyValue::Float(0.75));
    }

    #[test]
    fn test_gradient_stop_defaults_skipped() {
        let gs = GradientStop {
            name: "GS".to_string(),
            parent_id: 1,
            color: 0,
            position: 0.0,
        };
        let props = gs.properties();
        assert_eq!(props.len(), 2);
    }

    #[test]
    fn test_node_type_key() {
        let node = Node {
            name: "N".to_string(),
            parent_id: 1,
            x: 0.0,
            y: 0.0,
        };
        assert_eq!(node.type_key(), 2);
    }

    #[test]
    fn test_solo_type_key() {
        let solo = Solo {
            name: "SoloNode".to_string(),
            parent_id: 1,
            x: 0.0,
            y: 0.0,
            active_component_id: 0,
        };
        assert_eq!(solo.type_key(), type_keys::SOLO);
    }

    #[test]
    fn test_solo_properties_default_omission() {
        let solo = Solo {
            name: "SoloNode".to_string(),
            parent_id: 1,
            x: 0.0,
            y: 0.0,
            active_component_id: 0,
        };
        let props = solo.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert!(!props.iter().any(|p| p.key == property_keys::NODE_X));
        assert!(!props.iter().any(|p| p.key == property_keys::NODE_Y));
        assert!(
            !props
                .iter()
                .any(|p| p.key == property_keys::SOLO_ACTIVE_COMPONENT_ID)
        );
    }

    #[test]
    fn test_solo_properties_with_active_component() {
        let solo = Solo {
            name: "SoloNode".to_string(),
            parent_id: 2,
            x: 12.0,
            y: 24.0,
            active_component_id: 5,
        };
        let props = solo.properties();
        assert_eq!(props.len(), 5);
        assert_eq!(props[2].key, property_keys::NODE_X);
        assert_eq!(props[2].value, PropertyValue::Float(12.0));
        assert_eq!(props[3].key, property_keys::NODE_Y);
        assert_eq!(props[3].value, PropertyValue::Float(24.0));
        assert_eq!(props[4].key, property_keys::SOLO_ACTIVE_COMPONENT_ID);
        assert_eq!(props[4].value, PropertyValue::UInt(5));
    }

    #[test]
    fn test_transform_component_type_key() {
        let tc = TransformComponent {
            name: "TC".to_string(),
            parent_id: 1,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        };
        assert_eq!(tc.type_key(), 38);
        let props = tc.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
    }

    #[test]
    fn test_radial_gradient_type_key() {
        let rg = RadialGradient {
            name: "RG".to_string(),
            parent_id: 1,
            start_x: 0.0,
            start_y: 0.0,
            end_x: 50.0,
            end_y: 50.0,
            opacity: 1.0,
        };
        assert_eq!(rg.type_key(), 17);
    }

    #[test]
    fn test_path_object_type_key() {
        let po = PathObject {
            name: "P".to_string(),
            parent_id: 1,
            path_flags: 0,
        };
        assert_eq!(po.type_key(), 12);
    }

    #[test]
    fn test_drawable_type_key() {
        let d = Drawable {
            name: "D".to_string(),
            parent_id: 1,
            blend_mode: 0,
            drawable_flags: 0,
        };
        assert_eq!(d.type_key(), 13);
    }

    #[test]
    fn test_image_type_key() {
        let image = Image::new("Img".to_string(), 1, 0);
        assert_eq!(image.type_key(), 100);
    }

    #[test]
    fn test_image_default_properties() {
        let image = Image::new("Img".to_string(), 2, 0);
        let props = image.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
        assert_eq!(props[2].key, property_keys::IMAGE_ASSET_ID);
        assert_eq!(props[2].value, PropertyValue::UInt(0));
    }

    #[test]
    fn test_image_with_position() {
        let mut image = Image::new("Img".to_string(), 2, 1);
        image.x = 12.0;
        image.y = 24.0;
        let props = image.properties();
        assert_eq!(props.len(), 5);
        assert_eq!(props[3].key, property_keys::NODE_X);
        assert_eq!(props[3].value, PropertyValue::Float(12.0));
        assert_eq!(props[4].key, property_keys::NODE_Y);
        assert_eq!(props[4].value, PropertyValue::Float(24.0));
    }

    #[test]
    fn test_shape_paint_type_key() {
        let sp = ShapePaint {
            name: "SP".to_string(),
            parent_id: 1,
            is_visible: 1,
        };
        assert_eq!(sp.type_key(), 21);
    }

    #[test]
    fn test_node_properties_with_position() {
        let node = Node {
            name: "Positioned".to_string(),
            parent_id: 2,
            x: 50.0,
            y: 75.0,
        };
        let props = node.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[2].key, property_keys::NODE_X);
        assert_eq!(props[2].value, PropertyValue::Float(50.0));
        assert_eq!(props[3].key, property_keys::NODE_Y);
        assert_eq!(props[3].value, PropertyValue::Float(75.0));
    }

    #[test]
    fn test_transform_component_properties() {
        let tc = TransformComponent {
            name: "TC".to_string(),
            parent_id: 1,
            rotation: 1.5,
            scale_x: 2.0,
            scale_y: 3.0,
        };
        let props = tc.properties();
        assert_eq!(props.len(), 5);
        assert_eq!(props[2].key, property_keys::TRANSFORM_ROTATION);
        assert_eq!(props[3].key, property_keys::TRANSFORM_SCALE_X);
        assert_eq!(props[4].key, property_keys::TRANSFORM_SCALE_Y);
    }

    #[test]
    fn test_drawable_properties_with_blend_mode() {
        let d = Drawable {
            name: "D".to_string(),
            parent_id: 1,
            blend_mode: 3,
            drawable_flags: 1,
        };
        let props = d.properties();
        assert_eq!(props.len(), 4);
        assert_eq!(props[2].key, property_keys::DRAWABLE_BLEND_MODE);
        assert_eq!(props[2].value, PropertyValue::UInt(3));
        assert_eq!(props[3].key, property_keys::DRAWABLE_FLAGS);
        assert_eq!(props[3].value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_path_object_properties_with_flags() {
        let po = PathObject {
            name: "P".to_string(),
            parent_id: 1,
            path_flags: 2,
        };
        let props = po.properties();
        assert_eq!(props.len(), 3);
        assert_eq!(props[2].key, property_keys::PATH_FLAGS);
        assert_eq!(props[2].value, PropertyValue::UInt(2));
    }

    #[test]
    fn test_points_path_object_properties() {
        let ppo = PointsPathObject {
            name: "pp".to_string(),
            parent_id: Some(1),
            x: 10.0,
            y: 20.0,
            is_closed: true,
            path_flags: 3,
        };
        assert_eq!(ppo.type_key(), type_keys::POINTS_PATH);
        let props = ppo.properties();
        assert_eq!(props.len(), 6);
        assert_eq!(props[2].key, property_keys::NODE_X);
        assert_eq!(props[3].key, property_keys::NODE_Y);
        assert_eq!(props[4].key, property_keys::POINTS_PATH_IS_CLOSED);
        assert_eq!(props[5].key, property_keys::PATH_FLAGS);
    }

    #[test]
    fn test_straight_vertex_uses_vertex_xy_keys() {
        let vertex = StraightVertexObject {
            name: "v".to_string(),
            parent_id: Some(2),
            x: 1.0,
            y: 2.0,
            radius: 3.0,
        };
        assert_eq!(vertex.type_key(), type_keys::STRAIGHT_VERTEX);
        let props = vertex.properties();
        assert_eq!(props[2].key, property_keys::VERTEX_X);
        assert_eq!(props[3].key, property_keys::VERTEX_Y);
        assert_eq!(props[4].key, property_keys::STRAIGHT_VERTEX_RADIUS);
    }

    #[test]
    fn test_cubic_mirrored_vertex_properties() {
        let vertex = CubicMirroredVertexObject {
            name: "cmv".to_string(),
            parent_id: Some(2),
            x: 1.0,
            y: 2.0,
            rotation: 0.5,
            distance: 9.0,
        };
        assert_eq!(vertex.type_key(), type_keys::CUBIC_MIRRORED_VERTEX);
        let props = vertex.properties();
        assert_eq!(props[4].key, property_keys::CUBIC_MIRRORED_VERTEX_ROTATION);
        assert_eq!(props[5].key, property_keys::CUBIC_MIRRORED_VERTEX_DISTANCE);
    }

    #[test]
    fn test_cubic_detached_vertex_properties() {
        let vertex = CubicDetachedVertexObject {
            name: "cdv".to_string(),
            parent_id: Some(2),
            x: 1.0,
            y: 2.0,
            in_rotation: 0.1,
            in_distance: 10.0,
            out_rotation: 0.2,
            out_distance: 20.0,
        };
        assert_eq!(vertex.type_key(), type_keys::CUBIC_DETACHED_VERTEX);
        let props = vertex.properties();
        assert_eq!(
            props[4].key,
            property_keys::CUBIC_DETACHED_VERTEX_IN_ROTATION
        );
        assert_eq!(
            props[5].key,
            property_keys::CUBIC_DETACHED_VERTEX_IN_DISTANCE
        );
        assert_eq!(
            props[6].key,
            property_keys::CUBIC_DETACHED_VERTEX_OUT_ROTATION
        );
        assert_eq!(
            props[7].key,
            property_keys::CUBIC_DETACHED_VERTEX_OUT_DISTANCE
        );
    }

    #[test]
    fn test_shape_paint_properties() {
        let sp = ShapePaint {
            name: "SP".to_string(),
            parent_id: 1,
            is_visible: 1,
        };
        let props = sp.properties();
        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, property_keys::COMPONENT_NAME);
        assert_eq!(props[1].key, property_keys::COMPONENT_PARENT_ID);
    }

    #[test]
    fn test_trim_path_type_key() {
        let tp = TrimPath::new("trim1".to_string(), 2);
        assert_eq!(tp.type_key(), type_keys::TRIM_PATH);
    }

    #[test]
    fn test_trim_path_default_properties() {
        let tp = TrimPath::new("trim1".to_string(), 2);
        let props = tp.properties();
        assert_eq!(props.len(), 3);
        assert!(props.iter().any(|p| p.key == property_keys::COMPONENT_NAME));
        assert!(
            props
                .iter()
                .any(|p| p.key == property_keys::COMPONENT_PARENT_ID)
        );
        let mode = props
            .iter()
            .find(|p| p.key == property_keys::TRIM_PATH_MODE_VALUE)
            .unwrap();
        assert_eq!(mode.value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_trim_path_all_properties() {
        let mut tp = TrimPath::new("trim1".to_string(), 2);
        tp.start = 0.1;
        tp.end = 0.75;
        tp.offset = 0.5;
        tp.mode_value = 1;
        let props = tp.properties();
        assert_eq!(props.len(), 6);
        let start = props
            .iter()
            .find(|p| p.key == property_keys::TRIM_PATH_START)
            .unwrap();
        assert_eq!(start.value, PropertyValue::Float(0.1));
        let end = props
            .iter()
            .find(|p| p.key == property_keys::TRIM_PATH_END)
            .unwrap();
        assert_eq!(end.value, PropertyValue::Float(0.75));
        let offset = props
            .iter()
            .find(|p| p.key == property_keys::TRIM_PATH_OFFSET)
            .unwrap();
        assert_eq!(offset.value, PropertyValue::Float(0.5));
        let mode = props
            .iter()
            .find(|p| p.key == property_keys::TRIM_PATH_MODE_VALUE)
            .unwrap();
        assert_eq!(mode.value, PropertyValue::UInt(1));
    }

    #[test]
    fn test_trim_path_zero_values_omitted() {
        let tp = TrimPath::new("trim1".to_string(), 2);
        let props = tp.properties();
        assert!(
            props
                .iter()
                .all(|p| p.key != property_keys::TRIM_PATH_START)
        );
        assert!(props.iter().all(|p| p.key != property_keys::TRIM_PATH_END));
        assert!(
            props
                .iter()
                .all(|p| p.key != property_keys::TRIM_PATH_OFFSET)
        );
        assert!(
            props
                .iter()
                .any(|p| p.key == property_keys::TRIM_PATH_MODE_VALUE)
        );
    }

    #[test]
    fn test_trim_path_set_mode_valid() {
        let mut trim = TrimPath::new("T".to_string(), 1);
        assert!(trim.set_mode(1).is_ok());
        assert_eq!(trim.mode_value, 1);
        assert!(trim.set_mode(2).is_ok());
        assert_eq!(trim.mode_value, 2);
    }

    #[test]
    fn test_trim_path_set_mode_invalid() {
        let mut trim = TrimPath::new("T".to_string(), 1);
        assert!(trim.set_mode(0).is_err());
        assert!(trim.set_mode(3).is_err());
        assert!(trim.set_mode(999).is_err());
    }
}
