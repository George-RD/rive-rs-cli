use serde::Deserialize;

pub(crate) const SCENE_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Deserialize)]
pub struct SceneSpec {
    pub scene_format_version: u32,
    #[serde(default)]
    pub artboard: Option<ArtboardSpec>,
    #[serde(default)]
    pub artboards: Option<Vec<ArtboardSpec>>,
}

#[derive(Debug, Deserialize)]
pub struct ArtboardSpec {
    pub name: String,
    #[serde(default)]
    pub preset: Option<String>,
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    pub children: Vec<ObjectSpec>,
    pub animations: Option<Vec<AnimationSpec>>,
    pub state_machines: Option<Vec<StateMachineSpec>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum ObjectSpec {
    Shape {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    Solo {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
        active_component: Option<String>,
    },
    Ellipse {
        name: String,
        width: f32,
        height: f32,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
    },
    Rectangle {
        name: String,
        width: f32,
        height: f32,
        corner_radius: Option<f32>,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
    },
    Triangle {
        name: String,
        width: f32,
        height: f32,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
    },
    Polygon {
        name: String,
        width: f32,
        height: f32,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
        points: Option<u64>,
        corner_radius: Option<f32>,
    },
    Star {
        name: String,
        width: f32,
        height: f32,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
        points: Option<u64>,
        corner_radius: Option<f32>,
        inner_radius: Option<f32>,
    },
    Fill {
        name: String,
        fill_rule: Option<serde_json::Value>,
        is_visible: Option<bool>,
        children: Option<Vec<ObjectSpec>>,
    },
    Stroke {
        name: String,
        thickness: Option<f32>,
        cap: Option<serde_json::Value>,
        join: Option<serde_json::Value>,
        is_visible: Option<bool>,
        children: Option<Vec<ObjectSpec>>,
    },
    SolidColor {
        name: String,
        color: String,
    },
    LinearGradient {
        name: String,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        children: Option<Vec<ObjectSpec>>,
    },
    RadialGradient {
        name: String,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        children: Option<Vec<ObjectSpec>>,
    },
    GradientStop {
        name: Option<String>,
        color: String,
        position: f32,
    },
    Node {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
    },
    Image {
        name: String,
        asset_id: u64,
        x: Option<f32>,
        y: Option<f32>,
    },
    Path {
        name: String,
        path_flags: Option<u64>,
    },
    #[serde(rename = "points_path")]
    PointsPath {
        name: String,
        #[serde(default)]
        x: Option<f32>,
        #[serde(default)]
        y: Option<f32>,
        #[serde(default)]
        is_closed: Option<bool>,
        #[serde(default)]
        path_flags: Option<u64>,
        #[serde(default)]
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "straight_vertex")]
    StraightVertex {
        name: String,
        #[serde(default)]
        x: Option<f32>,
        #[serde(default)]
        y: Option<f32>,
        #[serde(default)]
        radius: Option<f32>,
    },
    #[serde(rename = "cubic_mirrored_vertex")]
    CubicMirroredVertex {
        name: String,
        #[serde(default)]
        x: Option<f32>,
        #[serde(default)]
        y: Option<f32>,
        #[serde(default)]
        rotation: Option<f32>,
        #[serde(default)]
        distance: Option<f32>,
    },
    #[serde(rename = "cubic_detached_vertex")]
    CubicDetachedVertex {
        name: String,
        #[serde(default)]
        x: Option<f32>,
        #[serde(default)]
        y: Option<f32>,
        #[serde(default)]
        in_rotation: Option<f32>,
        #[serde(default)]
        in_distance: Option<f32>,
        #[serde(default)]
        out_rotation: Option<f32>,
        #[serde(default)]
        out_distance: Option<f32>,
    },
    #[serde(rename = "cubic_asymmetric_vertex")]
    CubicAsymmetricVertex {
        name: String,
        #[serde(default)]
        x: Option<f32>,
        #[serde(default)]
        y: Option<f32>,
        #[serde(default)]
        rotation: Option<f32>,
        #[serde(default)]
        in_distance: Option<f32>,
        #[serde(default)]
        out_distance: Option<f32>,
    },
    TrimPath {
        name: String,
        start: Option<f32>,
        end: Option<f32>,
        offset: Option<f32>,
        mode: Option<serde_json::Value>,
    },
    NestedArtboard {
        name: String,
        source_artboard: String,
        x: Option<f32>,
        y: Option<f32>,
    },
    NestedStateMachine {
        name: String,
        animation: String,
    },
    Event {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    NestedSimpleAnimation {
        name: String,
        animation: String,
        speed: Option<f32>,
        is_playing: Option<bool>,
        mix: Option<f32>,
    },
    Bone {
        name: String,
        length: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    RootBone {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
        length: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    Skin {
        name: String,
        xx: Option<f32>,
        yx: Option<f32>,
        xy: Option<f32>,
        yy: Option<f32>,
        tx: Option<f32>,
        ty: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    Tendon {
        name: String,
        bone: Option<String>,
        xx: Option<f32>,
        yx: Option<f32>,
        xy: Option<f32>,
        yy: Option<f32>,
        tx: Option<f32>,
        ty: Option<f32>,
    },
    Weight {
        name: String,
        values: Option<u64>,
        indices: Option<u64>,
    },
    CubicWeight {
        name: String,
        in_values: Option<u64>,
        in_indices: Option<u64>,
        out_values: Option<u64>,
        out_indices: Option<u64>,
    },
    IkConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        invert_direction: Option<bool>,
        parent_bone_count: Option<u64>,
    },
    DistanceConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        distance: Option<f32>,
        mode_value: Option<u64>,
    },
    TransformConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        source_space_value: Option<u64>,
        dest_space_value: Option<u64>,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
    },
    TranslationConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        source_space_value: Option<u64>,
        dest_space_value: Option<u64>,
        copy_factor: Option<f32>,
        min_value: Option<f32>,
        max_value: Option<f32>,
        offset: Option<bool>,
        does_copy: Option<bool>,
        min: Option<bool>,
        max: Option<bool>,
        min_max_space_value: Option<u64>,
        copy_factor_y: Option<f32>,
        min_value_y: Option<f32>,
        max_value_y: Option<f32>,
        does_copy_y: Option<bool>,
        min_y: Option<bool>,
        max_y: Option<bool>,
    },
    ScaleConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        source_space_value: Option<u64>,
        dest_space_value: Option<u64>,
        copy_factor: Option<f32>,
        min_value: Option<f32>,
        max_value: Option<f32>,
        offset: Option<bool>,
        does_copy: Option<bool>,
        min: Option<bool>,
        max: Option<bool>,
        min_max_space_value: Option<u64>,
        copy_factor_y: Option<f32>,
        min_value_y: Option<f32>,
        max_value_y: Option<f32>,
        does_copy_y: Option<bool>,
        min_y: Option<bool>,
        max_y: Option<bool>,
    },
    RotationConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        source_space_value: Option<u64>,
        dest_space_value: Option<u64>,
        copy_factor: Option<f32>,
        min_value: Option<f32>,
        max_value: Option<f32>,
        offset: Option<bool>,
        does_copy: Option<bool>,
        min: Option<bool>,
        max: Option<bool>,
        min_max_space_value: Option<u64>,
    },
    #[serde(rename = "follow_path_constraint")]
    FollowPathConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        source_space_value: Option<u64>,
        dest_space_value: Option<u64>,
        distance: Option<f32>,
        orient: Option<bool>,
        offset: Option<bool>,
    },
    #[serde(rename = "clipping_shape")]
    ClippingShape {
        name: String,
        source: Option<String>,
        fill_rule: Option<serde_json::Value>,
        is_visible: Option<bool>,
    },
    #[serde(rename = "draw_rules")]
    DrawRules {
        name: String,
        draw_target: Option<String>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "draw_target")]
    DrawTarget {
        name: String,
        drawable: Option<String>,
        placement_value: Option<u64>,
    },
    Joystick {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
        x_id: Option<u64>,
        y_id: Option<u64>,
        pos_x: Option<f32>,
        pos_y: Option<f32>,
        width: Option<f32>,
        height: Option<f32>,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
        flags: Option<u64>,
        handle_source_id: Option<u64>,
    },
    Text {
        name: String,
        align_value: Option<u64>,
        sizing_value: Option<u64>,
        overflow_value: Option<u64>,
        width: Option<f32>,
        height: Option<f32>,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
        paragraph_spacing: Option<f32>,
        origin_value: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    TextStyle {
        name: String,
        font_size: Option<f32>,
        line_height: Option<f32>,
        letter_spacing: Option<f32>,
        font_asset_id: Option<u64>,
        children: Option<Vec<TextStyleChildSpec>>,
    },
    TextValueRun {
        name: String,
        text: String,
        style_id: Option<u64>,
    },
    ImageAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
    },
    FontAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
    },
    AudioAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
    },
    LayoutComponent {
        name: String,
        clip: Option<bool>,
        width: Option<f32>,
        height: Option<f32>,
        style_id: Option<u64>,
        fractional_width: Option<f32>,
        fractional_height: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    LayoutComponentStyle {
        name: String,
        gap_horizontal: Option<f32>,
        gap_vertical: Option<f32>,
        max_width: Option<f32>,
        max_height: Option<f32>,
        min_width: Option<f32>,
        min_height: Option<f32>,
        border_left: Option<f32>,
        border_right: Option<f32>,
        border_top: Option<f32>,
        border_bottom: Option<f32>,
        margin_left: Option<f32>,
        margin_right: Option<f32>,
        margin_top: Option<f32>,
        margin_bottom: Option<f32>,
        padding_left: Option<f32>,
        padding_right: Option<f32>,
        padding_top: Option<f32>,
        padding_bottom: Option<f32>,
        position_left: Option<f32>,
        position_right: Option<f32>,
        position_top: Option<f32>,
        position_bottom: Option<f32>,
        flex_direction: Option<u64>,
        flex_wrap: Option<u64>,
        align_items: Option<u64>,
        align_content: Option<u64>,
        justify_content: Option<u64>,
        display: Option<u64>,
        position_type: Option<u64>,
        overflow: Option<u64>,
        intrinsically_sized: Option<bool>,
        width_units: Option<u64>,
        height_units: Option<u64>,
        flex_grow: Option<f32>,
        flex_shrink: Option<f32>,
        flex_basis: Option<f32>,
        aspect_ratio: Option<f32>,
    },
    ViewModel {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    ViewModelProperty {
        name: String,
        property_type_value: Option<u64>,
    },
    DataBind {
        property_key: u64,
        flags: u64,
        converter_id: Option<u64>,
    },
    ViewModelInstance {
        view_model_id: Option<u64>,
    },
    ViewModelInstanceValue {
        view_model_property_id: Option<u64>,
    },
    ViewModelInstanceColor {
        view_model_property_id: Option<u64>,
        value: String,
    },
    ViewModelInstanceString {
        view_model_property_id: Option<u64>,
        value: String,
    },
    ViewModelInstanceNumber {
        view_model_property_id: Option<u64>,
        value: f32,
    },
    ViewModelInstanceBoolean {
        view_model_property_id: Option<u64>,
        value: bool,
    },
    ViewModelInstanceEnum {
        view_model_property_id: Option<u64>,
        value: Option<u64>,
    },
    ViewModelInstanceList,
    ViewModelInstanceListItem {
        view_model_id: Option<u64>,
        view_model_instance_id: Option<u64>,
    },
    ViewModelInstanceViewModel {
        view_model_property_id: Option<u64>,
        value: Option<u64>,
    },
    TextModifierRange {
        units_value: Option<u64>,
        type_value: Option<u64>,
        mode_value: Option<u64>,
        modify_from: Option<f32>,
        modify_to: Option<f32>,
        strength: Option<f32>,
        clamp: Option<bool>,
        falloff_from: Option<f32>,
        falloff_to: Option<f32>,
        offset: Option<f32>,
        run_id: Option<u64>,
    },
    TextModifierGroup {
        name: String,
        modifier_flags: Option<u64>,
        origin_x: Option<f32>,
        origin_y: Option<f32>,
        opacity: Option<f32>,
        x: Option<f32>,
        y: Option<f32>,
        rotation: Option<f32>,
        scale_x: Option<f32>,
        scale_y: Option<f32>,
        children: Option<Vec<TextModifierGroupChildSpec>>,
    },
    TextVariationModifier {
        axis_tag: Option<u64>,
        axis_value: Option<f32>,
    },
    TextStyleFeature {
        tag: Option<u64>,
        feature_value: Option<u64>,
    },
}

#[derive(Debug, Deserialize)]
pub struct InterpolatorSpec {
    pub name: String,
    #[serde(default, rename = "type", alias = "interpolation_type")]
    pub interpolation_type: Option<String>,
    pub x1: Option<f32>,
    pub y1: Option<f32>,
    pub x2: Option<f32>,
    pub y2: Option<f32>,
    pub easing_value: Option<u64>,
    pub amplitude: Option<f32>,
    pub period: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct AnimationSpec {
    pub name: String,
    pub fps: u64,
    pub duration: u64,
    pub speed: Option<f32>,
    pub loop_type: Option<serde_json::Value>,
    pub interpolators: Option<Vec<InterpolatorSpec>>,
    pub keyframes: Vec<KeyframeGroupSpec>,
}

#[derive(Debug, Deserialize)]
pub struct KeyframeGroupSpec {
    pub object: String,
    pub property: String,
    pub frames: Vec<KeyframeSpec>,
}

#[derive(Debug, Deserialize)]
pub struct KeyframeSpec {
    pub frame: u64,
    pub value: serde_json::Value,
    pub interpolation: Option<String>,
    pub interpolator: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StateMachineSpec {
    pub name: String,
    pub inputs: Option<Vec<InputSpec>>,
    pub listeners: Option<Vec<StateMachineListenerSpec>>,
    pub layers: Vec<LayerSpec>,
}

#[derive(Debug, Deserialize)]
pub struct StateMachineListenerSpec {
    pub target: String,
    pub listener_type_value: Option<u64>,
    pub actions: Option<Vec<ListenerActionSpec>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum ListenerActionSpec {
    BoolChange {
        input: String,
        value: Option<serde_json::Value>,
    },
    TriggerChange {
        input: String,
    },
    NumberChange {
        input: String,
        value: Option<serde_json::Value>,
    },
}

#[derive(Clone, Copy)]
pub(crate) enum InterpolatorDef {
    Cubic {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    },
    Elastic {
        easing_value: u64,
        amplitude: f32,
        period: f32,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputSpec {
    Number { name: String, value: f32 },
    Bool { name: String, value: bool },
    Trigger { name: String },
}

#[derive(Debug, Deserialize)]
pub struct LayerSpec {
    pub states: Vec<StateSpec>,
    pub transitions: Option<Vec<TransitionSpec>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StateSpec {
    Entry,
    Exit,
    Any,
    Animation {
        animation: String,
    },
    BlendState {
        children: Option<Vec<BlendStateChildSpec>>,
    },
    BlendStateDirect {
        children: Option<Vec<BlendStateDirectChildSpec>>,
    },
    #[serde(alias = "blend_state_1d")]
    BlendState1d {
        input_id: Option<u64>,
        children: Option<Vec<BlendState1DChildSpec>>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlendStateChildSpec {
    BlendAnimation { animation_id: u64 },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlendStateDirectChildSpec {
    BlendAnimationDirect {
        animation_id: u64,
        input_id: Option<u64>,
        mix_value: Option<f32>,
        blend_source: Option<u64>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlendState1DChildSpec {
    #[serde(alias = "blend_animation_1d")]
    BlendAnimation1D {
        animation_id: u64,
        value: Option<f32>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextStyleChildSpec {
    TextStyleFeature {
        tag: Option<u64>,
        feature_value: Option<u64>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextModifierGroupChildSpec {
    TextModifierRange {
        units_value: Option<u64>,
        type_value: Option<u64>,
        mode_value: Option<u64>,
        modify_from: Option<f32>,
        modify_to: Option<f32>,
        strength: Option<f32>,
        clamp: Option<bool>,
        falloff_from: Option<f32>,
        falloff_to: Option<f32>,
        offset: Option<f32>,
        run_id: Option<u64>,
    },
    TextVariationModifier {
        axis_tag: Option<u64>,
        axis_value: Option<f32>,
    },
}

#[derive(Debug, Deserialize)]
pub struct TransitionSpec {
    pub from: usize,
    pub to: usize,
    pub duration: Option<u64>,
    pub conditions: Option<Vec<ConditionSpec>>,
    pub children: Option<Vec<TransitionChildSpec>>,
}

#[derive(Debug, Deserialize)]
pub struct ConditionSpec {
    pub input: String,
    pub op: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum TransitionChildSpec {
    TransitionPropertyComparator,
    TransitionViewModelCondition { op_value: Option<u64> },
    TransitionValueBooleanComparator { value: bool },
    TransitionValueColorComparator { value: String },
    TransitionValueNumberComparator { value: f32 },
    TransitionValueEnumComparator,
    TransitionValueStringComparator { value: String },
    TransitionValueTriggerComparator { value: Option<u64> },
}

pub(crate) enum ParentKind {
    Artboard,
    Shape,
    PointsPath,
    Fill,
    Stroke,
    Gradient,
    Bone,
    Text,
    LayoutComponent,
    ViewModel,
}
