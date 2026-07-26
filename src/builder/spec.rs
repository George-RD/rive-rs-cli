use schemars::JsonSchema;
use serde::Deserialize;

pub(crate) const SCENE_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SceneSpec {
    pub scene_format_version: u32,
    #[serde(default)]
    pub artboard: Option<ArtboardSpec>,
    #[serde(default)]
    pub artboards: Option<Vec<ArtboardSpec>>,
}

#[derive(Debug, Deserialize, Default, JsonSchema)]
pub struct ArtboardSpec {
    pub name: String,
    #[serde(default)]
    pub preset: Option<String>,
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    #[serde(default)]
    pub origin_x: Option<f32>,
    #[serde(default)]
    pub origin_y: Option<f32>,
    #[serde(default)]
    pub x: Option<f32>,
    #[serde(default)]
    pub y: Option<f32>,
    pub children: Vec<ObjectSpec>,
    pub animations: Option<Vec<AnimationSpec>>,
    pub state_machines: Option<Vec<StateMachineSpec>>,
}
#[derive(Debug, Deserialize, JsonSchema)]
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
        color: Option<String>,
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
        children: Option<Vec<ObjectSpec>>,
    },
    Image {
        name: String,
        asset_id: u64,
        x: Option<f32>,
        y: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
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
        children: Option<Vec<ObjectSpec>>,
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
    Folder {
        name: String,
        #[serde(default)]
        parent_id: Option<u64>,
    },
    LayeredAsset {
        name: String,
    },
    #[serde(rename = "layer_image_asset")]
    LayerImageAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
    },
    #[serde(rename = "svg_asset")]
    SVGAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
    },
    LottieAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
    },
    ExportAudio {
        name: String,
        volume: Option<f32>,
    },
    ScriptAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
        is_module: Option<bool>,
    },
    BlobAsset {
        name: String,
        asset_id: Option<u64>,
        cdn_base_url: Option<String>,
    },
    #[serde(rename = "dash_path")]
    DashPath {
        name: String,
        offset: Option<f32>,
        offset_is_percentage: Option<bool>,
        children: Option<Vec<ObjectSpec>>,
    },
    Dash {
        name: String,
        length: Option<f32>,
        length_is_percentage: Option<bool>,
    },
    Feather {
        name: String,
        strength: Option<f32>,
        offset_x: Option<f32>,
        offset_y: Option<f32>,
        space_value: Option<u64>,
        inner: Option<bool>,
    },
    #[serde(rename = "open_url_event")]
    OpenUrlEvent {
        name: String,
        url: Option<String>,
        target_value: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "audio_event")]
    AudioEvent {
        name: String,
        asset_id: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "custom_property_number")]
    CustomPropertyNumber {
        name: String,
        property_value: Option<f32>,
    },
    #[serde(rename = "custom_property_boolean")]
    CustomPropertyBoolean {
        name: String,
        property_value: Option<serde_json::Value>,
    },
    #[serde(rename = "custom_property_string")]
    CustomPropertyString {
        name: String,
        property_value: Option<String>,
    },
    #[serde(rename = "custom_property_color")]
    CustomPropertyColor {
        name: String,
        property_value: Option<String>,
    },
    #[serde(rename = "custom_property_trigger")]
    CustomPropertyTrigger {
        name: String,
    },
    #[serde(rename = "custom_property_enum")]
    CustomPropertyEnum {
        name: String,
        property_value: Option<u64>,
        enum_id: Option<u64>,
    },
    #[serde(rename = "custom_property_group")]
    CustomPropertyGroup {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "target_effect")]
    TargetEffect {
        name: String,
        target_id: Option<u64>,
    },
    #[serde(rename = "group_effect")]
    GroupEffect {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "list_path")]
    ListPath {
        name: String,
        is_closed: Option<bool>,
        list_source: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "points_common_path")]
    PointsCommonPath {
        name: String,
        is_closed: Option<bool>,
        children: Option<Vec<ObjectSpec>>,
    },
    Guide {
        name: String,
    },
    #[serde(rename = "artboard_component_list")]
    ArtboardComponentList {
        name: String,
        list_source: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "artboard_component_list_override")]
    ArtboardComponentListOverride {
        name: String,
        artboard_id: Option<u64>,
        instance_width: Option<f32>,
        instance_height: Option<f32>,
        instance_width_units_value: Option<u64>,
        instance_height_units_value: Option<u64>,
        instance_width_scale_type: Option<u64>,
        instance_height_scale_type: Option<u64>,
    },
    #[serde(rename = "artboard_list_map_rule")]
    ArtboardListMapRule {
        name: String,
        artboard_id: Option<u64>,
        view_model_id: Option<u64>,
    },
    #[serde(rename = "foreground_layout_drawable")]
    ForegroundLayoutDrawable {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "clamped_scroll_physics")]
    ClampedScrollPhysics {
        friction: Option<f32>,
        speed_multiplier: Option<f32>,
    },
    #[serde(rename = "elastic_scroll_physics")]
    ElasticScrollPhysics {
        friction: Option<f32>,
        speed_multiplier: Option<f32>,
        elastic_factor: Option<f32>,
    },
    Mesh {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "mesh_vertex")]
    MeshVertex {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
        u: Option<f32>,
        v: Option<f32>,
    },
    #[serde(rename = "contour_mesh_vertex")]
    ContourMeshVertex {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
        u: Option<f32>,
        v: Option<f32>,
    },
    #[serde(rename = "forced_edge")]
    ForcedEdge {
        name: String,
        from_vertex: Option<String>,
        to_vertex: Option<String>,
    },
    #[serde(rename = "nested_linear_animation")]
    NestedLinearAnimation {
        name: String,
        animation: String,
        mix: Option<f32>,
    },
    #[serde(rename = "nested_remap_animation")]
    NestedRemapAnimation {
        name: String,
        animation: String,
        time: Option<f32>,
    },
    #[serde(rename = "nested_trigger")]
    NestedTrigger {
        name: String,
        nested_input_id: u64,
    },
    #[serde(rename = "nested_bool")]
    NestedBool {
        name: String,
        nested_input_id: u64,
        value: Option<bool>,
    },
    #[serde(rename = "nested_number")]
    NestedNumber {
        name: String,
        nested_input_id: u64,
        value: Option<f32>,
    },
    #[serde(rename = "nested_artboard_leaf")]
    NestedArtboardLeaf {
        name: String,
        source_artboard: String,
        x: Option<f32>,
        y: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "nested_artboard_layout")]
    NestedArtboardLayout {
        name: String,
        source_artboard: String,
        x: Option<f32>,
        y: Option<f32>,
        width: Option<f32>,
        height: Option<f32>,
        style_id: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "draggable_constraint")]
    DraggableConstraint {
        name: String,
        strength: Option<f32>,
        direction_value: Option<u64>,
    },
    #[serde(rename = "scroll_constraint")]
    ScrollConstraint {
        name: String,
        strength: Option<f32>,
        direction_value: Option<u64>,
        snap: Option<bool>,
        physics_id: Option<u64>,
        scroll_offset_x: Option<f32>,
        scroll_offset_y: Option<f32>,
        scroll_percent_x: Option<f32>,
        scroll_percent_y: Option<f32>,
        scroll_index: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "scroll_bar_constraint")]
    ScrollBarConstraint {
        name: String,
        strength: Option<f32>,
        scroll_constraint_id: Option<u64>,
        auto_size: Option<bool>,
    },
    #[serde(rename = "list_follow_path_constraint")]
    ListFollowPathConstraint {
        name: String,
        target: Option<String>,
        strength: Option<f32>,
        orient: Option<bool>,
        start: Option<f32>,
        end: Option<f32>,
        list_source: Option<u64>,
        distance_end: Option<f32>,
        distance_offset: Option<f32>,
        random_mode_value: Option<u64>,
    },
    #[serde(rename = "nslicer_tile_mode")]
    NSlicerTileMode {
        name: Option<String>,
        patch_index: u64,
        style: Option<u64>,
    },
    #[serde(rename = "nslicer")]
    NSlicer {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "axis_y")]
    AxisY {
        name: Option<String>,
        offset: f32,
        normalized: Option<bool>,
    },
    #[serde(rename = "axis_x")]
    AxisX {
        name: Option<String>,
        offset: f32,
        normalized: Option<bool>,
    },
    #[serde(rename = "n_sliced_node")]
    NSlicedNode {
        name: String,
        x: Option<f32>,
        y: Option<f32>,
        initial_width: Option<f32>,
        initial_height: Option<f32>,
        width: Option<f32>,
        height: Option<f32>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_number")]
    ViewModelPropertyNumber {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_boolean")]
    ViewModelPropertyBoolean {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_string")]
    ViewModelPropertyString {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_color")]
    ViewModelPropertyColor {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_list")]
    ViewModelPropertyList {
        name: String,
        view_model_reference_id: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_view_model")]
    ViewModelPropertyViewModel {
        name: String,
        view_model_reference_id: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_enum")]
    ViewModelPropertyEnum {
        name: String,
        enum_id: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_enum_custom")]
    ViewModelPropertyEnumCustom {
        name: String,
        enum_id: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_enum_system")]
    ViewModelPropertyEnumSystem {
        name: String,
        enum_id: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_trigger")]
    ViewModelPropertyTrigger {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_asset_image")]
    ViewModelPropertyAssetImage {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_artboard")]
    ViewModelPropertyArtboard {
        name: String,
        artboard_id: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_symbol")]
    ViewModelPropertySymbol {
        name: String,
        symbol_type_value: Option<u64>,
        artboard_id: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_property_symbol_list_index")]
    ViewModelPropertySymbolListIndex {
        name: String,
        symbol_type_value: Option<u64>,
        artboard_id: Option<u64>,
        list_source: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "view_model_instance_trigger")]
    ViewModelInstanceTrigger {
        view_model_property_id: Option<u64>,
        value: Option<u64>,
    },
    #[serde(rename = "view_model_instance_symbol")]
    ViewModelInstanceSymbol {
        view_model_property_id: Option<u64>,
        value: Option<u64>,
    },
    #[serde(rename = "view_model_instance_symbol_list_index")]
    ViewModelInstanceSymbolListIndex {
        view_model_property_id: Option<u64>,
        value: Option<u64>,
    },
    #[serde(rename = "view_model_instance_asset_image")]
    ViewModelInstanceAssetImage {
        view_model_property_id: Option<u64>,
        value: Option<u64>,
    },
    #[serde(rename = "view_model_instance_artboard")]
    ViewModelInstanceArtboard {
        view_model_property_id: Option<u64>,
        value: Option<u64>,
        artboard_id: Option<u64>,
    },
    #[serde(rename = "data_enum")]
    DataEnum {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "data_enum_custom")]
    DataEnumCustom {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    DataEnumValue {
        key: String,
        value: String,
    },
    #[serde(rename = "data_enum_system")]
    DataEnumSystem {
        name: String,
        enum_type: Option<u64>,
    },
    #[serde(rename = "bindable_property_string")]
    BindablePropertyString {
        value: Option<String>,
    },
    #[serde(rename = "bindable_property_boolean")]
    BindablePropertyBoolean {
        value: Option<u64>,
    },
    #[serde(rename = "bindable_property_number")]
    BindablePropertyNumber {
        value: Option<f32>,
    },
    #[serde(rename = "bindable_property_enum")]
    BindablePropertyEnum {
        value: Option<u64>,
    },
    #[serde(rename = "bindable_property_color")]
    BindablePropertyColor {
        value: String,
    },
    #[serde(rename = "bindable_property_trigger")]
    BindablePropertyTrigger {
        value: Option<u64>,
    },
    #[serde(rename = "bindable_property_integer")]
    BindablePropertyInteger {
        value: Option<u64>,
    },
    #[serde(rename = "bindable_property_list")]
    BindablePropertyList {
        value: Option<u64>,
    },
    #[serde(rename = "bindable_property_id")]
    BindablePropertyId {
        value: String,
    },
    #[serde(rename = "bindable_property_artboard")]
    BindablePropertyArtboard {
        value: Option<u64>,
    },
    DataBindPath {
        property_key: u64,
        flags: u64,
        converter_id: Option<u64>,
    },
    TextStylePaint {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    TextStyleAxis {
        tag: Option<u64>,
        axis_value: Option<f32>,
    },
    TextTargetModifier {
        name: String,
        target_id: Option<u64>,
    },
    TextFollowPathModifier {
        name: String,
        target_id: Option<u64>,
        orient: Option<bool>,
        start: Option<f32>,
        end: Option<f32>,
        strength: Option<f32>,
        offset: Option<f32>,
    },
    TextInput {
        name: String,
        align_value: Option<u64>,
        sizing_value: Option<u64>,
        overflow_value: Option<u64>,
        width: Option<f32>,
        height: Option<f32>,
        text: Option<String>,
        selection_radius: Option<f32>,
        interactive: Option<bool>,
        children: Option<Vec<ObjectSpec>>,
    },
    TextInputDrawable {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    TextInputCursor {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    TextInputText {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    TextInputSelection {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    TextInputSelectedText {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "data_converter_rounder")]
    DataConverterRounder {
        name: String,
        decimals: Option<u64>,
    },
    #[serde(rename = "data_converter_to_string")]
    DataConverterToString {
        name: String,
        flags: Option<u64>,
        decimals: Option<u64>,
        color_format: Option<String>,
    },
    #[serde(rename = "data_converter_to_number")]
    DataConverterToNumber {
        name: String,
    },
    #[serde(rename = "data_converter_group")]
    DataConverterGroup {
        name: String,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "data_converter_group_item")]
    DataConverterGroupItem {
        converter_id: Option<u64>,
    },
    #[serde(rename = "data_converter_operation_value")]
    DataConverterOperationValue {
        name: String,
        operation_type: Option<u64>,
        operation_value: Option<f32>,
    },
    #[serde(rename = "data_converter_trigger")]
    DataConverterTrigger {
        name: String,
    },
    #[serde(rename = "data_converter_operation_view_model")]
    DataConverterOperationViewModel {
        name: String,
        operation_type: Option<u64>,
    },
    #[serde(rename = "data_converter_string_pad")]
    DataConverterStringPad {
        name: String,
        length: Option<u64>,
        text: Option<String>,
        pad_type: Option<u64>,
    },
    #[serde(rename = "data_converter_string_remove_zeros")]
    DataConverterStringRemoveZeros {
        name: String,
    },
    #[serde(rename = "data_converter_string_trim")]
    DataConverterStringTrim {
        name: String,
        trim_type: Option<u64>,
    },
    #[serde(rename = "data_converter_interpolator")]
    DataConverterInterpolator {
        name: String,
        duration: Option<f32>,
        interpolation_type: Option<u64>,
        interpolator_id: Option<u64>,
    },
    #[serde(rename = "data_converter_boolean_negate")]
    DataConverterBooleanNegate {
        name: String,
    },
    #[serde(rename = "data_converter_range_mapper")]
    DataConverterRangeMapper {
        name: String,
        interpolation_type: Option<u64>,
        interpolator_id: Option<u64>,
        flags: Option<u64>,
        min_input: Option<f32>,
        max_input: Option<f32>,
        min_output: Option<f32>,
        max_output: Option<f32>,
    },
    #[serde(rename = "data_converter_formula")]
    DataConverterFormula {
        name: String,
        random_mode_value: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "data_converter_system_degs_to_rads")]
    DataConverterSystemDegsToRads {
        name: String,
        operation_type: Option<u64>,
    },
    #[serde(rename = "data_converter_system_normalizer")]
    DataConverterSystemNormalizer {
        name: String,
        operation_type: Option<u64>,
        operation_value: Option<f32>,
    },
    #[serde(rename = "data_converter_number_to_list")]
    DataConverterNumberToList {
        name: String,
        view_model_id: Option<u64>,
    },
    #[serde(rename = "data_converter_list_to_length")]
    DataConverterListToLength {
        name: String,
    },
    #[serde(rename = "formula_token_argument_separator")]
    FormulaTokenArgumentSeparator,
    #[serde(rename = "formula_token_parenthesis_close")]
    FormulaTokenParenthesisClose,
    #[serde(rename = "formula_token_operation")]
    FormulaTokenOperation {
        operation_type: Option<u64>,
    },
    #[serde(rename = "formula_token_function")]
    FormulaTokenFunction {
        function_type: Option<u64>,
    },
    #[serde(rename = "formula_token_value")]
    FormulaTokenValue {
        operation_value: Option<f32>,
    },
    #[serde(rename = "formula_token_parenthesis_open")]
    FormulaTokenParenthesisOpen,
    #[serde(rename = "formula_token_input")]
    FormulaTokenInput,
    #[serde(rename = "scripted_drawable")]
    ScriptedDrawable {
        name: String,
        script_asset_id: Option<u64>,
        generator_function_ref: Option<u64>,
        threshold: Option<f32>,
        is_paused: Option<bool>,
        speed: Option<f32>,
        quantize: Option<f32>,
        interactive: Option<bool>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "scripted_data_converter")]
    ScriptedDataConverter {
        name: String,
        script_asset_id: Option<u64>,
    },
    #[serde(rename = "scripted_layout")]
    ScriptedLayout {
        name: String,
        script_asset_id: Option<u64>,
        children: Option<Vec<ObjectSpec>>,
    },
    #[serde(rename = "scripted_path_effect")]
    ScriptedPathEffect {
        name: String,
        is_relative: Option<bool>,
        target_id: Option<u64>,
    },
    #[serde(rename = "scripted_listener_action")]
    ScriptedListenerAction {
        script_asset_id: Option<u64>,
        is_stateful: Option<bool>,
    },
    #[serde(rename = "scripted_transition_condition")]
    ScriptedTransitionCondition {
        script_asset_id: Option<u64>,
        is_stateful: Option<bool>,
    },
    #[serde(rename = "script_input_number")]
    ScriptInputNumber {
        name: String,
    },
    #[serde(rename = "script_input_view_model_property")]
    ScriptInputViewModelProperty {
        name: String,
        view_model_id: Option<u64>,
    },
    #[serde(rename = "script_input_trigger")]
    ScriptInputTrigger {
        name: String,
    },
    #[serde(rename = "script_input_artboard")]
    ScriptInputArtboard {
        name: String,
        artboard_id: Option<u64>,
    },
    #[serde(rename = "script_input_color")]
    ScriptInputColor {
        name: String,
    },
    #[serde(rename = "script_input_string")]
    ScriptInputString {
        name: String,
    },
    #[serde(rename = "script_input_boolean")]
    ScriptInputBoolean {
        name: String,
    },
}

#[derive(Debug, Deserialize, JsonSchema)]
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

#[derive(Debug, Deserialize, Default, JsonSchema)]
pub struct AnimationSpec {
    pub name: String,
    pub fps: u64,
    pub duration: u64,
    pub speed: Option<f32>,
    pub loop_type: Option<serde_json::Value>,
    pub quantize: Option<u64>,
    pub work_start: Option<u64>,
    pub work_end: Option<u64>,
    pub enable_work_area: Option<bool>,
    pub interpolators: Option<Vec<InterpolatorSpec>>,
    pub keyframes: Vec<KeyframeGroupSpec>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct KeyframeGroupSpec {
    pub object: String,
    pub property: String,
    pub frames: Vec<KeyframeSpec>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct KeyframeSpec {
    pub frame: u64,
    pub value: serde_json::Value,
    pub interpolation: Option<String>,
    pub interpolator: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StateMachineSpec {
    pub name: String,
    pub inputs: Option<Vec<InputSpec>>,
    pub listeners: Option<Vec<StateMachineListenerSpec>>,
    pub components: Option<Vec<StateMachineComponentSpec>>,
    pub layers: Vec<LayerSpec>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StateMachineComponentSpec {
    FireEvent {
        name: String,
        event_id: Option<u64>,
        occurs_value: Option<u64>,
    },
    FireTrigger {
        name: String,
    },
    FireAction {
        name: String,
    },
    NestedArtboard {
        name: String,
        artboard_id: Option<u64>,
    },
    NestedInput {
        name: String,
        nested_input_id: Option<u64>,
    },
    #[serde(alias = "blend_state_1d_view_model")]
    BlendState1DViewModel,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StateMachineListenerSpec {
    pub target: String,
    pub listener_type_value: Option<u64>,
    pub actions: Option<Vec<ListenerActionSpec>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
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
    AlignTarget {
        target_id: Option<u64>,
    },
    FireEvent {
        event_id: Option<u64>,
    },
    ViewModelChange {
        view_model_property_id: Option<u64>,
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

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputSpec {
    Number { name: String, value: f32 },
    Bool { name: String, value: bool },
    Trigger { name: String },
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LayerSpec {
    pub states: Vec<StateSpec>,
    pub transitions: Option<Vec<TransitionSpec>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
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

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlendStateChildSpec {
    BlendAnimation { animation_id: u64 },
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlendStateDirectChildSpec {
    BlendAnimationDirect {
        animation_id: u64,
        input_id: Option<u64>,
        mix_value: Option<f32>,
        blend_source: Option<u64>,
    },
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlendState1DChildSpec {
    #[serde(alias = "blend_animation_1d")]
    BlendAnimation1D {
        animation_id: u64,
        value: Option<f32>,
    },
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextStyleChildSpec {
    TextStyleFeature {
        tag: Option<u64>,
        feature_value: Option<u64>,
    },
}

#[derive(Debug, Deserialize, JsonSchema)]
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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TransitionSpec {
    pub from: usize,
    pub to: usize,
    pub duration: Option<u64>,
    pub conditions: Option<Vec<ConditionSpec>>,
    pub children: Option<Vec<TransitionChildSpec>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConditionSpec {
    pub input: String,
    pub op: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, JsonSchema)]
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
    TransitionPropertyViewModelComparator,
    TransitionPropertyArtboardComparator,
    TransitionArtboardCondition { op_value: Option<u64> },
    TransitionSelfComparator,
    TransitionValueIdComparator { value: Option<u64> },
    TransitionValueAssetComparator { value: Option<u64> },
    TransitionValueArtboardComparator { value: Option<u64> },
}

#[allow(dead_code)]
#[derive(Debug)]
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
    Mesh,
    NSlicer,
    NSlicedNode,
    Image,
    NestedArtboard,
    DashPath,
}

#[cfg(test)]
mod tests {
    use super::ObjectSpec;

    fn collect_type_consts(value: &serde_json::Value, tags: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                if let Some(serde_json::Value::String(tag)) = map
                    .get("type")
                    .and_then(|t| t.as_object())
                    .and_then(|t| t.get("const"))
                {
                    tags.push(tag.clone());
                }
                for child in map.values() {
                    collect_type_consts(child, tags);
                }
            }
            serde_json::Value::Array(items) => {
                for child in items {
                    collect_type_consts(child, tags);
                }
            }
            _ => {}
        }
    }

    fn object_variant_tags(schema: &serde_json::Value) -> Vec<String> {
        let defs = schema
            .get("$defs")
            .and_then(|d| d.as_object())
            .expect("prompt schema has $defs");
        let variants = defs
            .get("object")
            .and_then(|o| o.get("oneOf"))
            .and_then(|o| o.as_array())
            .expect("prompt schema has $defs.object.oneOf");
        let mut tags = Vec::new();
        for variant in variants {
            let resolved = match variant.get("$ref").and_then(|r| r.as_str()) {
                Some(reference) => {
                    let name = reference
                        .strip_prefix("#/$defs/")
                        .expect("object variant $ref points into $defs");
                    defs.get(name)
                        .unwrap_or_else(|| panic!("prompt schema has $defs.{}", name))
                }
                None => variant,
            };
            collect_type_consts(resolved, &mut tags);
        }
        tags
    }

    #[test]
    fn ai_prompt_schema_types_all_exist() {
        let raw = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/ai/scene-prompt-schema.json"
        ));
        let schema: serde_json::Value = serde_json::from_str(raw).expect("parse prompt schema");
        let tags = object_variant_tags(&schema);
        assert!(!tags.is_empty(), "prompt schema declares no object types");
        for tag in tags {
            let probe = format!("{{\"type\":\"{}\"}}", tag);
            let err = serde_json::from_str::<ObjectSpec>(&probe)
                .err()
                .map(|e| e.to_string())
                .unwrap_or_default();
            assert!(
                !err.contains("unknown variant"),
                "prompt schema declares object type '{}' that ObjectSpec does not have",
                tag
            );
        }
    }
}
