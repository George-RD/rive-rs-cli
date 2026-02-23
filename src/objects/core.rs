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
    pub const SKELETAL_COMPONENT: u16 = 39;
    pub const BONE: u16 = 40;
    pub const ROOT_BONE: u16 = 41;
    pub const SKIN: u16 = 43;
    pub const TENDON: u16 = 44;
    pub const WEIGHT: u16 = 45;
    pub const CUBIC_WEIGHT: u16 = 46;
    pub const CONSTRAINT: u16 = 79;
    pub const TARGETED_CONSTRAINT: u16 = 80;
    pub const IK_CONSTRAINT: u16 = 81;
    pub const DISTANCE_CONSTRAINT: u16 = 82;
    pub const TRANSFORM_CONSTRAINT: u16 = 83;
    pub const TRANSFORM_COMPONENT_CONSTRAINT: u16 = 85;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_Y: u16 = 86;
    pub const TRANSLATION_CONSTRAINT: u16 = 87;
    pub const SCALE_CONSTRAINT: u16 = 88;
    pub const ROTATION_CONSTRAINT: u16 = 89;
    pub const TRANSFORM_SPACE_CONSTRAINT: u16 = 90;
    pub const WORLD_TRANSFORM_COMPONENT: u16 = 91;
    pub const NESTED_ARTBOARD: u16 = 92;
    pub const CUBIC_VALUE_INTERPOLATOR: u16 = 138;
    pub const CUBIC_INTERPOLATOR: u16 = 139;
    pub const INTERPOLATING_KEY_FRAME: u16 = 170;
    pub const KEYFRAME_INTERPOLATOR: u16 = 175;
    pub const LAYOUT_COMPONENT: u16 = 409;
    pub const TRANSITION_CONDITION: u16 = 476;
    pub const LAYOUT_COMPONENT_STYLE: u16 = 420;
    pub const VIEW_MODEL: u16 = 435;
    pub const VIEW_MODEL_COMPONENT: u16 = 436;
    pub const VIEW_MODEL_PROPERTY: u16 = 430;
    pub const DATA_BIND: u16 = 446;
    pub const TEXT: u16 = 134;
    pub const TEXT_VALUE_RUN: u16 = 135;
    pub const TEXT_STYLE: u16 = 573;
    pub const IMAGE_ASSET: u16 = 105;
    pub const FONT_ASSET: u16 = 141;
    pub const AUDIO_ASSET: u16 = 406;
    pub const FILE_ASSET_CONTENTS: u16 = 106;
    pub const ASSET: u16 = 99;
    pub const FILE_ASSET: u16 = 103;
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
    pub const BONE_LENGTH: u16 = 89;
    pub const ROOT_BONE_X: u16 = 90;
    pub const ROOT_BONE_Y: u16 = 91;
    pub const TENDON_BONE_ID: u16 = 95;
    pub const TENDON_XX: u16 = 96;
    pub const TENDON_YX: u16 = 97;
    pub const TENDON_XY: u16 = 98;
    pub const TENDON_YY: u16 = 99;
    pub const TENDON_TX: u16 = 100;
    pub const TENDON_TY: u16 = 101;
    pub const WEIGHT_VALUES: u16 = 102;
    pub const WEIGHT_INDICES: u16 = 103;
    pub const SKIN_XX: u16 = 104;
    pub const SKIN_YX: u16 = 105;
    pub const SKIN_XY: u16 = 106;
    pub const SKIN_YY: u16 = 107;
    pub const SKIN_TX: u16 = 108;
    pub const SKIN_TY: u16 = 109;
    pub const CUBIC_WEIGHT_IN_VALUES: u16 = 110;
    pub const CUBIC_WEIGHT_IN_INDICES: u16 = 111;
    pub const CUBIC_WEIGHT_OUT_VALUES: u16 = 112;
    pub const CUBIC_WEIGHT_OUT_INDICES: u16 = 113;
    pub const CONSTRAINT_STRENGTH: u16 = 172;
    pub const TARGETED_CONSTRAINT_TARGET_ID: u16 = 173;
    pub const IK_CONSTRAINT_INVERT_DIRECTION: u16 = 174;
    pub const IK_CONSTRAINT_PARENT_BONE_COUNT: u16 = 175;
    pub const DISTANCE_CONSTRAINT_DISTANCE: u16 = 177;
    pub const DISTANCE_CONSTRAINT_MODE_VALUE: u16 = 178;
    pub const TRANSFORM_SPACE_SOURCE_SPACE_VALUE: u16 = 179;
    pub const TRANSFORM_SPACE_DEST_SPACE_VALUE: u16 = 180;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_COPY_FACTOR: u16 = 182;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_MIN_VALUE: u16 = 183;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_MAX_VALUE: u16 = 184;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_Y_COPY_FACTOR_Y: u16 = 185;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_Y_MIN_VALUE_Y: u16 = 186;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_Y_MAX_VALUE_Y: u16 = 187;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_OFFSET: u16 = 188;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_DOES_COPY: u16 = 189;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_MIN: u16 = 190;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_MAX: u16 = 191;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_Y_DOES_COPY_Y: u16 = 192;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_Y_MIN_Y: u16 = 193;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_Y_MAX_Y: u16 = 194;
    pub const TRANSFORM_COMPONENT_CONSTRAINT_MIN_MAX_SPACE_VALUE: u16 = 195;
    pub const TRANSFORM_CONSTRAINT_ORIGIN_X: u16 = 372;
    pub const TRANSFORM_CONSTRAINT_ORIGIN_Y: u16 = 373;
    pub const ASSET_NAME: u16 = 203;
    pub const FILE_ASSET_ASSET_ID: u16 = 204;
    pub const FILE_ASSET_CDN_BASE_URL: u16 = 362;
    pub const FILE_ASSET_CONTENTS_BYTES: u16 = 212;
    pub const TEXT_ALIGN_VALUE: u16 = 281;
    pub const TEXT_SIZING_VALUE: u16 = 284;
    pub const TEXT_WIDTH: u16 = 285;
    pub const TEXT_HEIGHT: u16 = 286;
    pub const TEXT_OVERFLOW_VALUE: u16 = 287;
    pub const TEXT_ORIGIN_X: u16 = 366;
    pub const TEXT_ORIGIN_Y: u16 = 367;
    pub const TEXT_PARAGRAPH_SPACING: u16 = 371;
    pub const TEXT_ORIGIN_VALUE: u16 = 377;
    pub const TEXT_STYLE_FONT_SIZE: u16 = 274;
    pub const TEXT_STYLE_LINE_HEIGHT: u16 = 370;
    pub const TEXT_STYLE_LETTER_SPACING: u16 = 390;
    pub const TEXT_STYLE_FONT_ASSET_ID: u16 = 279;
    pub const TEXT_VALUE_RUN_STYLE_ID: u16 = 272;
    pub const TEXT_VALUE_RUN_TEXT: u16 = 268;
    pub const LAYOUT_STYLE_GAP_HORIZONTAL: u16 = 498;
    pub const LAYOUT_STYLE_GAP_VERTICAL: u16 = 499;
    pub const LAYOUT_STYLE_MAX_WIDTH: u16 = 500;
    pub const LAYOUT_STYLE_MAX_HEIGHT: u16 = 501;
    pub const LAYOUT_STYLE_MIN_WIDTH: u16 = 502;
    pub const LAYOUT_STYLE_MIN_HEIGHT: u16 = 503;
    pub const LAYOUT_STYLE_BORDER_LEFT: u16 = 504;
    pub const LAYOUT_STYLE_BORDER_RIGHT: u16 = 505;
    pub const LAYOUT_STYLE_BORDER_TOP: u16 = 506;
    pub const LAYOUT_STYLE_BORDER_BOTTOM: u16 = 507;
    pub const LAYOUT_STYLE_MARGIN_LEFT: u16 = 508;
    pub const LAYOUT_STYLE_MARGIN_RIGHT: u16 = 509;
    pub const LAYOUT_STYLE_MARGIN_TOP: u16 = 510;
    pub const LAYOUT_STYLE_MARGIN_BOTTOM: u16 = 511;
    pub const LAYOUT_STYLE_PADDING_LEFT: u16 = 512;
    pub const LAYOUT_STYLE_PADDING_RIGHT: u16 = 513;
    pub const LAYOUT_STYLE_PADDING_TOP: u16 = 514;
    pub const LAYOUT_STYLE_PADDING_BOTTOM: u16 = 515;
    pub const LAYOUT_STYLE_POSITION_LEFT: u16 = 516;
    pub const LAYOUT_STYLE_POSITION_RIGHT: u16 = 517;
    pub const LAYOUT_STYLE_POSITION_TOP: u16 = 518;
    pub const LAYOUT_STYLE_POSITION_BOTTOM: u16 = 519;
    pub const LAYOUT_STYLE_FLEX_DIRECTION: u16 = 520;
    pub const LAYOUT_STYLE_FLEX_WRAP: u16 = 524;
    pub const LAYOUT_STYLE_ALIGN_ITEMS: u16 = 521;
    pub const LAYOUT_STYLE_ALIGN_CONTENT: u16 = 522;
    pub const LAYOUT_STYLE_JUSTIFY_CONTENT: u16 = 523;
    pub const LAYOUT_STYLE_DISPLAY: u16 = 525;
    pub const LAYOUT_STYLE_POSITION_TYPE: u16 = 526;
    pub const LAYOUT_STYLE_OVERFLOW: u16 = 527;
    pub const LAYOUT_STYLE_INTRINSICALLY_SIZED_VALUE: u16 = 606;
    pub const LAYOUT_STYLE_WIDTH_UNITS: u16 = 607;
    pub const LAYOUT_STYLE_HEIGHT_UNITS: u16 = 608;
    pub const LAYOUT_STYLE_FLEX_GROW: u16 = 609;
    pub const LAYOUT_STYLE_FLEX_SHRINK: u16 = 610;
    pub const LAYOUT_STYLE_FLEX_BASIS: u16 = 611;
    pub const LAYOUT_STYLE_ASPECT_RATIO: u16 = 612;
    pub const VIEW_MODEL_PROPERTY_TYPE_VALUE: u16 = 875;
    pub const DATA_BIND_PROPERTY_KEY: u16 = 586;
    pub const DATA_BIND_FLAGS: u16 = 587;
    pub const DATA_BIND_CONVERTER_ID: u16 = 660;
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
            | property_keys::IK_CONSTRAINT_INVERT_DIRECTION
            | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_OFFSET
            | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_DOES_COPY
            | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN
            | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MAX
            | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_DOES_COPY_Y
            | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MIN_Y
            | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MAX_Y
            | property_keys::LAYOUT_STYLE_INTRINSICALLY_SIZED_VALUE
    )
}

pub fn property_backing_type(key: u16) -> Option<BackingType> {
    match key {
        property_keys::COMPONENT_NAME
        | property_keys::ANIMATION_NAME
        | property_keys::STATE_MACHINE_COMPONENT_NAME
        | property_keys::ASSET_NAME
        | property_keys::FILE_ASSET_CDN_BASE_URL
        | property_keys::TEXT_VALUE_RUN_TEXT => Some(BackingType::String),
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
        | property_keys::SHAPE_LENGTH
        | property_keys::BONE_LENGTH
        | property_keys::ROOT_BONE_X
        | property_keys::ROOT_BONE_Y
        | property_keys::TENDON_XX
        | property_keys::TENDON_YX
        | property_keys::TENDON_XY
        | property_keys::TENDON_YY
        | property_keys::TENDON_TX
        | property_keys::TENDON_TY
        | property_keys::SKIN_XX
        | property_keys::SKIN_YX
        | property_keys::SKIN_XY
        | property_keys::SKIN_YY
        | property_keys::SKIN_TX
        | property_keys::SKIN_TY
        | property_keys::CONSTRAINT_STRENGTH
        | property_keys::DISTANCE_CONSTRAINT_DISTANCE
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_COPY_FACTOR
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN_VALUE
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MAX_VALUE
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_COPY_FACTOR_Y
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MIN_VALUE_Y
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MAX_VALUE_Y
        | property_keys::TRANSFORM_CONSTRAINT_ORIGIN_X
        | property_keys::TRANSFORM_CONSTRAINT_ORIGIN_Y
        | property_keys::TEXT_WIDTH
        | property_keys::TEXT_HEIGHT
        | property_keys::TEXT_ORIGIN_X
        | property_keys::TEXT_ORIGIN_Y
        | property_keys::TEXT_PARAGRAPH_SPACING
        | property_keys::TEXT_STYLE_FONT_SIZE
        | property_keys::TEXT_STYLE_LINE_HEIGHT
        | property_keys::TEXT_STYLE_LETTER_SPACING
        | property_keys::LAYOUT_STYLE_GAP_HORIZONTAL
        | property_keys::LAYOUT_STYLE_GAP_VERTICAL
        | property_keys::LAYOUT_STYLE_MAX_WIDTH
        | property_keys::LAYOUT_STYLE_MAX_HEIGHT
        | property_keys::LAYOUT_STYLE_MIN_WIDTH
        | property_keys::LAYOUT_STYLE_MIN_HEIGHT
        | property_keys::LAYOUT_STYLE_BORDER_LEFT
        | property_keys::LAYOUT_STYLE_BORDER_RIGHT
        | property_keys::LAYOUT_STYLE_BORDER_TOP
        | property_keys::LAYOUT_STYLE_BORDER_BOTTOM
        | property_keys::LAYOUT_STYLE_MARGIN_LEFT
        | property_keys::LAYOUT_STYLE_MARGIN_RIGHT
        | property_keys::LAYOUT_STYLE_MARGIN_TOP
        | property_keys::LAYOUT_STYLE_MARGIN_BOTTOM
        | property_keys::LAYOUT_STYLE_PADDING_LEFT
        | property_keys::LAYOUT_STYLE_PADDING_RIGHT
        | property_keys::LAYOUT_STYLE_PADDING_TOP
        | property_keys::LAYOUT_STYLE_PADDING_BOTTOM
        | property_keys::LAYOUT_STYLE_POSITION_LEFT
        | property_keys::LAYOUT_STYLE_POSITION_RIGHT
        | property_keys::LAYOUT_STYLE_POSITION_TOP
        | property_keys::LAYOUT_STYLE_POSITION_BOTTOM
        | property_keys::LAYOUT_STYLE_FLEX_GROW
        | property_keys::LAYOUT_STYLE_FLEX_SHRINK
        | property_keys::LAYOUT_STYLE_FLEX_BASIS
        | property_keys::LAYOUT_STYLE_ASPECT_RATIO => Some(BackingType::Float),

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
        | property_keys::PATH_IS_HOLE
        | property_keys::TENDON_BONE_ID
        | property_keys::WEIGHT_VALUES
        | property_keys::WEIGHT_INDICES
        | property_keys::CUBIC_WEIGHT_IN_VALUES
        | property_keys::CUBIC_WEIGHT_IN_INDICES
        | property_keys::CUBIC_WEIGHT_OUT_VALUES
        | property_keys::CUBIC_WEIGHT_OUT_INDICES
        | property_keys::TARGETED_CONSTRAINT_TARGET_ID
        | property_keys::IK_CONSTRAINT_INVERT_DIRECTION
        | property_keys::IK_CONSTRAINT_PARENT_BONE_COUNT
        | property_keys::DISTANCE_CONSTRAINT_MODE_VALUE
        | property_keys::TRANSFORM_SPACE_SOURCE_SPACE_VALUE
        | property_keys::TRANSFORM_SPACE_DEST_SPACE_VALUE
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_OFFSET
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_DOES_COPY
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MIN
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_MAX
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_DOES_COPY_Y
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MIN_Y
        | property_keys::TRANSFORM_COMPONENT_CONSTRAINT_Y_MAX_Y
        | property_keys::TEXT_ALIGN_VALUE
        | property_keys::TEXT_SIZING_VALUE
        | property_keys::TEXT_OVERFLOW_VALUE
        | property_keys::TEXT_ORIGIN_VALUE
        | property_keys::TEXT_STYLE_FONT_ASSET_ID
        | property_keys::TEXT_VALUE_RUN_STYLE_ID
        | property_keys::FILE_ASSET_ASSET_ID
        | property_keys::FILE_ASSET_CONTENTS_BYTES
        | property_keys::LAYOUT_STYLE_FLEX_DIRECTION
        | property_keys::LAYOUT_STYLE_FLEX_WRAP
        | property_keys::LAYOUT_STYLE_ALIGN_ITEMS
        | property_keys::LAYOUT_STYLE_ALIGN_CONTENT
        | property_keys::LAYOUT_STYLE_JUSTIFY_CONTENT
        | property_keys::LAYOUT_STYLE_DISPLAY
        | property_keys::LAYOUT_STYLE_POSITION_TYPE
        | property_keys::LAYOUT_STYLE_OVERFLOW
        | property_keys::LAYOUT_STYLE_INTRINSICALLY_SIZED_VALUE
        | property_keys::LAYOUT_STYLE_WIDTH_UNITS
        | property_keys::LAYOUT_STYLE_HEIGHT_UNITS
        | property_keys::VIEW_MODEL_PROPERTY_TYPE_VALUE
        | property_keys::DATA_BIND_PROPERTY_KEY
        | property_keys::DATA_BIND_FLAGS
        | property_keys::DATA_BIND_CONVERTER_ID => Some(BackingType::UInt),

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
