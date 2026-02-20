#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackingType {
    UInt = 0,
    String = 1,
    Float = 2,
    Color = 3,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    UInt(u64),
    String(String),
    Float(f32),
    Color(u32),
}

impl PropertyValue {
    pub fn backing_type(&self) -> BackingType {
        match self {
            PropertyValue::UInt(_) => BackingType::UInt,
            PropertyValue::String(_) => BackingType::String,
            PropertyValue::Float(_) => BackingType::Float,
            PropertyValue::Color(_) => BackingType::Color,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub key: u16,
    pub value: PropertyValue,
}

pub trait RiveObject {
    fn type_key(&self) -> u16;
    fn properties(&self) -> Vec<Property>;
}

pub mod type_keys {
    pub const ARTBOARD: u16 = 1;
    pub const NODE: u16 = 2;
    pub const SHAPE: u16 = 3;
    pub const ELLIPSE: u16 = 4;
    pub const RECTANGLE: u16 = 7;
    pub const COMPONENT: u16 = 10;
    pub const CONTAINER_COMPONENT: u16 = 11;
    pub const PATH: u16 = 12;
    pub const DRAWABLE: u16 = 13;
    pub const PARAMETRIC_PATH: u16 = 15;
    pub const RADIAL_GRADIENT: u16 = 17;
    pub const SOLID_COLOR: u16 = 18;
    pub const GRADIENT_STOP: u16 = 19;
    pub const FILL: u16 = 20;
    pub const SHAPE_PAINT: u16 = 21;
    pub const LINEAR_GRADIENT: u16 = 22;
    pub const BACKBOARD: u16 = 23;
    pub const STROKE: u16 = 24;
    pub const KEYED_OBJECT: u16 = 25;
    pub const KEYED_PROPERTY: u16 = 26;
    pub const ANIMATION: u16 = 27;
    pub const CUBIC_EASE_INTERPOLATOR: u16 = 28;
    pub const KEY_FRAME: u16 = 29;
    pub const KEY_FRAME_DOUBLE: u16 = 30;
    pub const LINEAR_ANIMATION: u16 = 31;
    pub const KEY_FRAME_COLOR: u16 = 37;
    pub const TRANSFORM_COMPONENT: u16 = 38;
    pub const TRIM_PATH: u16 = 47;
    pub const STATE_MACHINE: u16 = 53;
    pub const STATE_MACHINE_COMPONENT: u16 = 54;
    pub const STATE_MACHINE_INPUT: u16 = 55;
    pub const STATE_MACHINE_NUMBER: u16 = 56;
    pub const STATE_MACHINE_LAYER: u16 = 57;
    pub const STATE_MACHINE_TRIGGER: u16 = 58;
    pub const STATE_MACHINE_BOOL: u16 = 59;
    pub const LAYER_STATE: u16 = 60;
    pub const ANIMATION_STATE: u16 = 61;
    pub const ANY_STATE: u16 = 62;
    pub const ENTRY_STATE: u16 = 63;
    pub const EXIT_STATE: u16 = 64;
    pub const STATE_TRANSITION: u16 = 65;
    pub const TRANSITION_INPUT_CONDITION: u16 = 67;
    pub const TRANSITION_TRIGGER_CONDITION: u16 = 68;
    pub const TRANSITION_VALUE_CONDITION: u16 = 69;
    pub const TRANSITION_NUMBER_CONDITION: u16 = 70;
    pub const TRANSITION_BOOL_CONDITION: u16 = 71;
    pub const WORLD_TRANSFORM_COMPONENT: u16 = 91;
    pub const NESTED_ARTBOARD: u16 = 92;
    pub const CUBIC_VALUE_INTERPOLATOR: u16 = 138;
    pub const CUBIC_INTERPOLATOR: u16 = 139;
    pub const INTERPOLATING_KEY_FRAME: u16 = 170;
    pub const KEYFRAME_INTERPOLATOR: u16 = 175;
    pub const LAYOUT_COMPONENT: u16 = 409;
    pub const TRANSITION_CONDITION: u16 = 476;
}

pub mod property_keys {
    pub const COMPONENT_NAME: u16 = 4;
    pub const COMPONENT_PARENT_ID: u16 = 5;
    pub const LAYOUT_COMPONENT_WIDTH: u16 = 7;
    pub const LAYOUT_COMPONENT_HEIGHT: u16 = 8;
    pub const NODE_X_ARTBOARD: u16 = 9;
    pub const NODE_Y_ARTBOARD: u16 = 10;
    pub const ARTBOARD_ORIGIN_X: u16 = 11;
    pub const ARTBOARD_ORIGIN_Y: u16 = 12;
    pub const NODE_X: u16 = 13;
    pub const NODE_Y: u16 = 14;
    pub const TRANSFORM_ROTATION: u16 = 15;
    pub const TRANSFORM_SCALE_X: u16 = 16;
    pub const TRANSFORM_SCALE_Y: u16 = 17;
    pub const WORLD_TRANSFORM_OPACITY: u16 = 18;
    pub const PARAMETRIC_PATH_WIDTH: u16 = 20;
    pub const PARAMETRIC_PATH_HEIGHT: u16 = 21;
    pub const DRAWABLE_BLEND_MODE: u16 = 23;
    pub const RECTANGLE_CORNER_RADIUS_TL: u16 = 31;
    pub const LINEAR_GRADIENT_START_Y: u16 = 33;
    pub const LINEAR_GRADIENT_END_X: u16 = 34;
    pub const LINEAR_GRADIENT_END_Y: u16 = 35;
    pub const SOLID_COLOR_VALUE: u16 = 37;
    pub const GRADIENT_STOP_COLOR: u16 = 38;
    pub const GRADIENT_STOP_POSITION: u16 = 39;
    pub const FILL_RULE: u16 = 40;
    pub const SHAPE_PAINT_IS_VISIBLE: u16 = 41;
    pub const LINEAR_GRADIENT_START_X: u16 = 42;
    pub const LINEAR_GRADIENT_OPACITY: u16 = 46;
    pub const STROKE_THICKNESS: u16 = 47;
    pub const STROKE_CAP: u16 = 48;
    pub const STROKE_JOIN: u16 = 49;
    pub const STROKE_TRANSFORM_AFFECTS: u16 = 50;
    pub const KEYED_OBJECT_ID: u16 = 51;
    pub const KEYED_PROPERTY_KEY: u16 = 53;
    pub const ANIMATION_NAME: u16 = 55;
    pub const LINEAR_ANIMATION_FPS: u16 = 56;
    pub const LINEAR_ANIMATION_DURATION: u16 = 57;
    pub const LINEAR_ANIMATION_SPEED: u16 = 58;
    pub const LINEAR_ANIMATION_LOOP: u16 = 59;
    pub const LINEAR_ANIMATION_WORK_START: u16 = 60;
    pub const LINEAR_ANIMATION_WORK_END: u16 = 61;
    pub const LINEAR_ANIMATION_ENABLE_WORK_AREA: u16 = 62;
    pub const CUBIC_INTERPOLATOR_X1: u16 = 63;
    pub const CUBIC_INTERPOLATOR_Y1: u16 = 64;
    pub const CUBIC_INTERPOLATOR_X2: u16 = 65;
    pub const CUBIC_INTERPOLATOR_Y2: u16 = 66;
    pub const KEY_FRAME_FRAME: u16 = 67;
    pub const INTERPOLATING_KEY_FRAME_TYPE: u16 = 68;
    pub const INTERPOLATING_KEY_FRAME_INTERPOLATOR_ID: u16 = 69;
    pub const KEY_FRAME_DOUBLE_VALUE: u16 = 70;
    pub const KEY_FRAME_COLOR_VALUE: u16 = 88;
    pub const TRIM_PATH_START: u16 = 114;
    pub const TRIM_PATH_END: u16 = 115;
    pub const TRIM_PATH_OFFSET: u16 = 116;
    pub const TRIM_PATH_MODE_VALUE: u16 = 117;
    pub const PARAMETRIC_PATH_ORIGIN_X: u16 = 123;
    pub const PARAMETRIC_PATH_ORIGIN_Y: u16 = 124;
    pub const PATH_FLAGS: u16 = 128;
    pub const DRAWABLE_FLAGS: u16 = 129;
    pub const STATE_MACHINE_COMPONENT_NAME: u16 = 138;
    pub const STATE_MACHINE_NUMBER_VALUE: u16 = 140;
    pub const STATE_MACHINE_BOOL_VALUE: u16 = 141;
    pub const ANIMATION_STATE_ANIMATION_ID: u16 = 149;
    pub const STATE_TRANSITION_STATE_TO_ID: u16 = 151;
    pub const STATE_TRANSITION_FLAGS: u16 = 152;
    pub const TRANSITION_INPUT_CONDITION_INPUT_ID: u16 = 155;
    pub const TRANSITION_VALUE_CONDITION_OP: u16 = 156;
    pub const TRANSITION_NUMBER_CONDITION_VALUE: u16 = 157;
    pub const STATE_TRANSITION_DURATION: u16 = 158;
    pub const STATE_TRANSITION_EXIT_TIME: u16 = 160;
    pub const RECTANGLE_CORNER_RADIUS_TR: u16 = 161;
    pub const RECTANGLE_CORNER_RADIUS_BL: u16 = 162;
    pub const RECTANGLE_CORNER_RADIUS_BR: u16 = 163;
    pub const RECTANGLE_LINK_CORNER_RADIUS: u16 = 164;
    pub const LAYOUT_COMPONENT_CLIP: u16 = 196;
    pub const NESTED_ARTBOARD_ARTBOARD_ID: u16 = 197;
    pub const ARTBOARD_DEFAULT_STATE_MACHINE_ID: u16 = 236;
    pub const LINEAR_ANIMATION_QUANTIZE: u16 = 376;
    pub const LAYOUT_COMPONENT_STYLE_ID: u16 = 494;
    pub const LAYER_STATE_FLAGS: u16 = 536;
    pub const STATE_TRANSITION_RANDOM_WEIGHT: u16 = 537;
    pub const ARTBOARD_VIEW_MODEL_ID: u16 = 583;
    pub const LAYOUT_COMPONENT_FRACTIONAL_WIDTH: u16 = 706;
    pub const LAYOUT_COMPONENT_FRACTIONAL_HEIGHT: u16 = 707;
    pub const SHAPE_PAINT_BLEND_MODE: u16 = 747;
    pub const PATH_IS_HOLE: u16 = 770;
    pub const SHAPE_LENGTH: u16 = 781;
}

pub fn is_bool_property(key: u16) -> bool {
    matches!(
        key,
        property_keys::SHAPE_PAINT_IS_VISIBLE
            | property_keys::LINEAR_ANIMATION_ENABLE_WORK_AREA
            | property_keys::STATE_MACHINE_BOOL_VALUE
            | property_keys::RECTANGLE_LINK_CORNER_RADIUS
            | property_keys::LINEAR_ANIMATION_QUANTIZE
            | property_keys::LAYOUT_COMPONENT_CLIP
            | property_keys::PATH_IS_HOLE
    )
}

pub fn property_backing_type(key: u16) -> Option<BackingType> {
    match key {
        property_keys::COMPONENT_NAME
        | property_keys::ANIMATION_NAME
        | property_keys::STATE_MACHINE_COMPONENT_NAME => Some(BackingType::String),

        property_keys::LAYOUT_COMPONENT_WIDTH
        | property_keys::LAYOUT_COMPONENT_HEIGHT
        | property_keys::NODE_X_ARTBOARD
        | property_keys::NODE_Y_ARTBOARD
        | property_keys::ARTBOARD_ORIGIN_X
        | property_keys::ARTBOARD_ORIGIN_Y
        | property_keys::NODE_X
        | property_keys::NODE_Y
        | property_keys::TRANSFORM_ROTATION
        | property_keys::TRANSFORM_SCALE_X
        | property_keys::TRANSFORM_SCALE_Y
        | property_keys::WORLD_TRANSFORM_OPACITY
        | property_keys::PARAMETRIC_PATH_WIDTH
        | property_keys::PARAMETRIC_PATH_HEIGHT
        | property_keys::RECTANGLE_CORNER_RADIUS_TL
        | property_keys::LINEAR_GRADIENT_START_Y
        | property_keys::LINEAR_GRADIENT_END_X
        | property_keys::LINEAR_GRADIENT_END_Y
        | property_keys::GRADIENT_STOP_POSITION
        | property_keys::LINEAR_GRADIENT_START_X
        | property_keys::LINEAR_GRADIENT_OPACITY
        | property_keys::STROKE_THICKNESS
        | property_keys::LINEAR_ANIMATION_SPEED
        | property_keys::KEY_FRAME_DOUBLE_VALUE
        | property_keys::PARAMETRIC_PATH_ORIGIN_X
        | property_keys::PARAMETRIC_PATH_ORIGIN_Y
        | property_keys::STATE_MACHINE_NUMBER_VALUE
        | property_keys::TRANSITION_NUMBER_CONDITION_VALUE
        | property_keys::RECTANGLE_CORNER_RADIUS_TR
        | property_keys::RECTANGLE_CORNER_RADIUS_BL
        | property_keys::RECTANGLE_CORNER_RADIUS_BR
        | property_keys::CUBIC_INTERPOLATOR_X1
        | property_keys::CUBIC_INTERPOLATOR_Y1
        | property_keys::CUBIC_INTERPOLATOR_X2
        | property_keys::CUBIC_INTERPOLATOR_Y2
        | property_keys::TRIM_PATH_START
        | property_keys::TRIM_PATH_END
        | property_keys::TRIM_PATH_OFFSET
        | property_keys::LAYOUT_COMPONENT_FRACTIONAL_WIDTH
        | property_keys::LAYOUT_COMPONENT_FRACTIONAL_HEIGHT
        | property_keys::SHAPE_LENGTH => Some(BackingType::Float),

        property_keys::COMPONENT_PARENT_ID
        | property_keys::DRAWABLE_BLEND_MODE
        | property_keys::FILL_RULE
        | property_keys::SHAPE_PAINT_IS_VISIBLE
        | property_keys::STROKE_CAP
        | property_keys::STROKE_JOIN
        | property_keys::STROKE_TRANSFORM_AFFECTS
        | property_keys::KEYED_OBJECT_ID
        | property_keys::KEYED_PROPERTY_KEY
        | property_keys::LINEAR_ANIMATION_FPS
        | property_keys::LINEAR_ANIMATION_DURATION
        | property_keys::LINEAR_ANIMATION_LOOP
        | property_keys::LINEAR_ANIMATION_WORK_START
        | property_keys::LINEAR_ANIMATION_WORK_END
        | property_keys::LINEAR_ANIMATION_ENABLE_WORK_AREA
        | property_keys::KEY_FRAME_FRAME
        | property_keys::INTERPOLATING_KEY_FRAME_TYPE
        | property_keys::INTERPOLATING_KEY_FRAME_INTERPOLATOR_ID
        | property_keys::PATH_FLAGS
        | property_keys::DRAWABLE_FLAGS
        | property_keys::STATE_MACHINE_BOOL_VALUE
        | property_keys::ANIMATION_STATE_ANIMATION_ID
        | property_keys::STATE_TRANSITION_STATE_TO_ID
        | property_keys::STATE_TRANSITION_FLAGS
        | property_keys::TRANSITION_INPUT_CONDITION_INPUT_ID
        | property_keys::TRANSITION_VALUE_CONDITION_OP
        | property_keys::STATE_TRANSITION_DURATION
        | property_keys::STATE_TRANSITION_EXIT_TIME
        | property_keys::RECTANGLE_LINK_CORNER_RADIUS
        | property_keys::LAYOUT_COMPONENT_CLIP
        | property_keys::NESTED_ARTBOARD_ARTBOARD_ID
        | property_keys::ARTBOARD_DEFAULT_STATE_MACHINE_ID
        | property_keys::LINEAR_ANIMATION_QUANTIZE
        | property_keys::LAYOUT_COMPONENT_STYLE_ID
        | property_keys::LAYER_STATE_FLAGS
        | property_keys::STATE_TRANSITION_RANDOM_WEIGHT
        | property_keys::ARTBOARD_VIEW_MODEL_ID
        | property_keys::TRIM_PATH_MODE_VALUE
        | property_keys::SHAPE_PAINT_BLEND_MODE
        | property_keys::PATH_IS_HOLE => Some(BackingType::UInt),

        property_keys::SOLID_COLOR_VALUE
        | property_keys::GRADIENT_STOP_COLOR
        | property_keys::KEY_FRAME_COLOR_VALUE => Some(BackingType::Color),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_backing_type_string() {
        assert_eq!(property_backing_type(4), Some(BackingType::String));
        assert_eq!(property_backing_type(55), Some(BackingType::String));
        assert_eq!(property_backing_type(138), Some(BackingType::String));
    }

    #[test]
    fn test_property_backing_type_float() {
        assert_eq!(
            property_backing_type(property_keys::LAYOUT_COMPONENT_WIDTH),
            Some(BackingType::Float)
        );
        assert_eq!(
            property_backing_type(property_keys::NODE_Y),
            Some(BackingType::Float)
        );
        assert_eq!(
            property_backing_type(property_keys::KEY_FRAME_DOUBLE_VALUE),
            Some(BackingType::Float)
        );
        assert_eq!(
            property_backing_type(property_keys::SHAPE_LENGTH),
            Some(BackingType::Float)
        );
        assert_eq!(
            property_backing_type(property_keys::CUBIC_INTERPOLATOR_X1),
            Some(BackingType::Float)
        );
        assert_eq!(
            property_backing_type(property_keys::CUBIC_INTERPOLATOR_Y1),
            Some(BackingType::Float)
        );
        assert_eq!(
            property_backing_type(property_keys::CUBIC_INTERPOLATOR_X2),
            Some(BackingType::Float)
        );
        assert_eq!(
            property_backing_type(property_keys::CUBIC_INTERPOLATOR_Y2),
            Some(BackingType::Float)
        );
        assert_eq!(
            property_backing_type(property_keys::TRIM_PATH_START),
            Some(BackingType::Float)
        );
        assert_eq!(
            property_backing_type(property_keys::TRIM_PATH_END),
            Some(BackingType::Float)
        );
        assert_eq!(
            property_backing_type(property_keys::TRIM_PATH_OFFSET),
            Some(BackingType::Float)
        );
    }

    #[test]
    fn test_property_backing_type_uint() {
        assert_eq!(
            property_backing_type(property_keys::COMPONENT_PARENT_ID),
            Some(BackingType::UInt)
        );
        assert_eq!(
            property_backing_type(property_keys::LINEAR_ANIMATION_FPS),
            Some(BackingType::UInt)
        );
        assert_eq!(
            property_backing_type(property_keys::TRIM_PATH_MODE_VALUE),
            Some(BackingType::UInt)
        );
        assert_eq!(
            property_backing_type(property_keys::ARTBOARD_VIEW_MODEL_ID),
            Some(BackingType::UInt)
        );
        assert_eq!(
            property_backing_type(property_keys::NESTED_ARTBOARD_ARTBOARD_ID),
            Some(BackingType::UInt)
        );
        assert_eq!(
            property_backing_type(property_keys::PATH_IS_HOLE),
            Some(BackingType::UInt)
        );
    }

    #[test]
    fn test_property_backing_type_color() {
        assert_eq!(property_backing_type(37), Some(BackingType::Color));
        assert_eq!(property_backing_type(38), Some(BackingType::Color));
        assert_eq!(property_backing_type(88), Some(BackingType::Color));
    }

    #[test]
    fn test_property_backing_type_unknown() {
        assert_eq!(property_backing_type(0), None);
        assert_eq!(property_backing_type(999), None);
        assert_eq!(property_backing_type(u16::MAX), None);
    }

    #[test]
    fn test_property_value_backing_type() {
        assert_eq!(PropertyValue::Float(1.0).backing_type(), BackingType::Float);
        assert_eq!(PropertyValue::UInt(42).backing_type(), BackingType::UInt);
        assert_eq!(
            PropertyValue::String("test".to_string()).backing_type(),
            BackingType::String
        );
        assert_eq!(
            PropertyValue::Color(0xFF0000FF).backing_type(),
            BackingType::Color
        );
    }

    #[test]
    fn test_is_bool_property() {
        assert!(is_bool_property(property_keys::SHAPE_PAINT_IS_VISIBLE));
        assert!(is_bool_property(
            property_keys::LINEAR_ANIMATION_ENABLE_WORK_AREA
        ));
        assert!(is_bool_property(property_keys::STATE_MACHINE_BOOL_VALUE));
        assert!(is_bool_property(
            property_keys::RECTANGLE_LINK_CORNER_RADIUS
        ));
        assert!(is_bool_property(property_keys::LINEAR_ANIMATION_QUANTIZE));
        assert!(is_bool_property(property_keys::LAYOUT_COMPONENT_CLIP));
        assert!(is_bool_property(property_keys::PATH_IS_HOLE));
        assert!(!is_bool_property(property_keys::COMPONENT_NAME));
        assert!(!is_bool_property(property_keys::NODE_X));
        assert!(!is_bool_property(property_keys::SOLID_COLOR_VALUE));
        assert!(!is_bool_property(property_keys::COMPONENT_PARENT_ID));
    }

    #[test]
    fn test_type_key_constants() {
        assert_eq!(type_keys::BACKBOARD, 23);
        assert_eq!(type_keys::ARTBOARD, 1);
        assert_eq!(type_keys::NODE, 2);
        assert_eq!(type_keys::SHAPE, 3);
        assert_eq!(type_keys::ELLIPSE, 4);
        assert_eq!(type_keys::RECTANGLE, 7);
        assert_eq!(type_keys::COMPONENT, 10);
        assert_eq!(type_keys::CONTAINER_COMPONENT, 11);
        assert_eq!(type_keys::PATH, 12);
        assert_eq!(type_keys::DRAWABLE, 13);
        assert_eq!(type_keys::PARAMETRIC_PATH, 15);
        assert_eq!(type_keys::RADIAL_GRADIENT, 17);
        assert_eq!(type_keys::SOLID_COLOR, 18);
        assert_eq!(type_keys::GRADIENT_STOP, 19);
        assert_eq!(type_keys::FILL, 20);
        assert_eq!(type_keys::SHAPE_PAINT, 21);
        assert_eq!(type_keys::LINEAR_GRADIENT, 22);
        assert_eq!(type_keys::STROKE, 24);
        assert_eq!(type_keys::KEYED_OBJECT, 25);
        assert_eq!(type_keys::KEYED_PROPERTY, 26);
        assert_eq!(type_keys::ANIMATION, 27);
        assert_eq!(type_keys::KEY_FRAME, 29);
        assert_eq!(type_keys::KEY_FRAME_DOUBLE, 30);
        assert_eq!(type_keys::LINEAR_ANIMATION, 31);
        assert_eq!(type_keys::KEY_FRAME_COLOR, 37);
        assert_eq!(type_keys::TRANSFORM_COMPONENT, 38);
        assert_eq!(type_keys::STATE_MACHINE, 53);
        assert_eq!(type_keys::STATE_MACHINE_COMPONENT, 54);
        assert_eq!(type_keys::STATE_MACHINE_INPUT, 55);
        assert_eq!(type_keys::STATE_MACHINE_NUMBER, 56);
        assert_eq!(type_keys::STATE_MACHINE_LAYER, 57);
        assert_eq!(type_keys::STATE_MACHINE_TRIGGER, 58);
        assert_eq!(type_keys::STATE_MACHINE_BOOL, 59);
        assert_eq!(type_keys::LAYER_STATE, 60);
        assert_eq!(type_keys::ANIMATION_STATE, 61);
        assert_eq!(type_keys::ANY_STATE, 62);
        assert_eq!(type_keys::ENTRY_STATE, 63);
        assert_eq!(type_keys::EXIT_STATE, 64);
        assert_eq!(type_keys::STATE_TRANSITION, 65);
        assert_eq!(type_keys::TRANSITION_INPUT_CONDITION, 67);
        assert_eq!(type_keys::TRANSITION_TRIGGER_CONDITION, 68);
        assert_eq!(type_keys::TRANSITION_VALUE_CONDITION, 69);
        assert_eq!(type_keys::TRANSITION_NUMBER_CONDITION, 70);
        assert_eq!(type_keys::TRANSITION_BOOL_CONDITION, 71);
        assert_eq!(type_keys::TRIM_PATH, 47);
        assert_eq!(type_keys::WORLD_TRANSFORM_COMPONENT, 91);
        assert_eq!(type_keys::NESTED_ARTBOARD, 92);
        assert_eq!(type_keys::CUBIC_VALUE_INTERPOLATOR, 138);
        assert_eq!(type_keys::CUBIC_INTERPOLATOR, 139);
        assert_eq!(type_keys::INTERPOLATING_KEY_FRAME, 170);
        assert_eq!(type_keys::KEYFRAME_INTERPOLATOR, 175);
        assert_eq!(type_keys::CUBIC_EASE_INTERPOLATOR, 28);
        assert_eq!(type_keys::LAYOUT_COMPONENT, 409);
        assert_eq!(type_keys::TRANSITION_CONDITION, 476);
    }
}
