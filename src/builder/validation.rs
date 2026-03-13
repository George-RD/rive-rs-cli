use std::collections::{HashMap, HashSet};

use crate::objects::core::{BackingType, is_bool_property, property_backing_type, type_keys};

use super::parsers::{
    condition_op_is_valid, interpolation_type_from_name, json_value_to_color, json_value_to_f32,
    json_value_to_string, json_value_to_u64, parse_color, parse_fill_rule, parse_loop_type,
    parse_stroke_cap, parse_stroke_join, parse_trim_mode, property_key_for_object,
    required_u64_field, validate_discrete_keyframe_interpolation,
};
use super::scene::resolve_artboard_dimensions;
use super::spec::{
    ArtboardSpec, BlendState1DChildSpec, BlendStateChildSpec, BlendStateDirectChildSpec, InputSpec,
    ListenerActionSpec, ObjectSpec, ParentKind, SCENE_FORMAT_VERSION, SceneSpec, StateSpec,
    TextModifierGroupChildSpec, TextStyleChildSpec, TransitionChildSpec,
};

pub(crate) fn validate_scene_spec(spec: &SceneSpec) -> Result<(), String> {
    if spec.scene_format_version != SCENE_FORMAT_VERSION {
        return Err(format!(
            "unsupported scene_format_version {} (expected {})",
            spec.scene_format_version, SCENE_FORMAT_VERSION
        ));
    }

    let artboard_specs = super::scene::resolve_artboards(spec)?;

    let mut artboard_names: HashSet<String> = HashSet::new();
    for artboard_spec in &artboard_specs {
        if artboard_names.contains(&artboard_spec.name) {
            return Err(format!("duplicate artboard name '{}'", artboard_spec.name));
        }
        artboard_names.insert(artboard_spec.name.clone());
        validate_artboard_spec(artboard_spec)?;
    }

    let mut artboard_deps: HashMap<String, Vec<String>> = HashMap::new();
    for artboard_spec in &artboard_specs {
        let refs = collect_nested_artboard_refs(&artboard_spec.children);
        if !refs.is_empty() {
            artboard_deps.insert(artboard_spec.name.clone(), refs);
        }
    }
    detect_artboard_cycles(&artboard_deps)?;

    Ok(())
}

pub(crate) fn validate_artboard_spec(artboard_spec: &ArtboardSpec) -> Result<(), String> {
    if artboard_spec.width < 0.0 {
        return Err(format!(
            "artboard '{}' width must be non-negative",
            artboard_spec.name
        ));
    }
    if artboard_spec.height < 0.0 {
        return Err(format!(
            "artboard '{}' height must be non-negative",
            artboard_spec.name
        ));
    }

    resolve_artboard_dimensions(artboard_spec)?;

    let mut object_names: HashSet<String> = HashSet::new();
    let mut object_type_keys: HashMap<String, u16> = HashMap::new();
    for child in &artboard_spec.children {
        validate_object_spec(child, &mut object_names, &ParentKind::Artboard)?;
        collect_object_type_key(child, &mut object_type_keys);
    }
    validate_image_asset_references(&artboard_spec.children)?;

    let mut animation_names: HashSet<String> = HashSet::new();
    if let Some(animations) = &artboard_spec.animations {
        for animation in animations {
            if animation.duration == 0 {
                return Err(format!(
                    "animation '{}' duration must be greater than 0",
                    animation.name
                ));
            }
            if animation_names.contains(&animation.name) {
                return Err(format!("duplicate animation name '{}'", animation.name));
            }
            animation_names.insert(animation.name.clone());

            if let Some(loop_type) = &animation.loop_type {
                parse_loop_type(loop_type)
                    .map_err(|e| format!("animation '{}': {}", animation.name, e))?;
            }
            let mut interp_names: HashSet<String> = HashSet::new();
            if let Some(interpolators) = &animation.interpolators {
                for interp in interpolators {
                    if interp_names.contains(&interp.name) {
                        return Err(format!(
                            "duplicate interpolator name '{}' in animation '{}'",
                            interp.name, animation.name
                        ));
                    }
                    interp_names.insert(interp.name.clone());
                }
            }

            for group in &animation.keyframes {
                let object_type_key = *object_type_keys.get(&group.object).ok_or_else(|| {
                    format!("unknown object referenced in keyframes: '{}'", group.object)
                })?;
                let property_key = property_key_for_object(&group.property, object_type_key)
                    .ok_or_else(|| {
                        format!(
                            "unknown property referenced in keyframes: '{}'",
                            group.property
                        )
                    })?;

                for frame in &group.frames {
                    if let Some(interp_name) = &frame.interpolator
                        && !interp_names.contains(interp_name)
                    {
                        return Err(format!(
                            "unknown interpolator '{}' referenced in keyframe",
                            interp_name
                        ));
                    }
                    if let Some(interp_type) = &frame.interpolation {
                        interpolation_type_from_name(interp_type)?;
                    }
                    let interp_type = match &frame.interpolation {
                        Some(name) => interpolation_type_from_name(name)?,
                        None => 1,
                    };
                    let interp_id = if frame.interpolator.is_some() {
                        0
                    } else {
                        u32::MAX as u64
                    };

                    match property_backing_type(property_key) {
                        Some(BackingType::Color) => {
                            if json_value_to_color(&frame.value).is_none() {
                                return Err(format!(
                                    "invalid color keyframe value for object '{}' property '{}' at frame {}",
                                    group.object, group.property, frame.frame
                                ));
                            }
                        }
                        Some(BackingType::String) => {
                            validate_discrete_keyframe_interpolation(
                                &group.object,
                                &group.property,
                                frame.frame,
                                frame.interpolation.as_deref(),
                                frame.interpolator.as_deref(),
                                interp_type,
                                interp_id,
                            )?;
                            if json_value_to_string(&frame.value).is_none() {
                                return Err(format!(
                                    "invalid string keyframe value for object '{}' property '{}' at frame {}",
                                    group.object, group.property, frame.frame
                                ));
                            }
                        }
                        Some(BackingType::UInt) => {
                            if json_value_to_u64(&frame.value).is_none() {
                                return Err(format!(
                                    "invalid integer keyframe value for object '{}' property '{}' at frame {}",
                                    group.object, group.property, frame.frame
                                ));
                            }
                            if is_bool_property(property_key) {
                                validate_discrete_keyframe_interpolation(
                                    &group.object,
                                    &group.property,
                                    frame.frame,
                                    frame.interpolation.as_deref(),
                                    frame.interpolator.as_deref(),
                                    interp_type,
                                    interp_id,
                                )?;
                            }
                        }
                        _ => {
                            if json_value_to_f32(&frame.value).is_none() {
                                return Err(format!(
                                    "invalid numeric keyframe value for object '{}' property '{}' at frame {}",
                                    group.object, group.property, frame.frame
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(state_machines) = &artboard_spec.state_machines {
        for state_machine in state_machines {
            let mut input_names: std::collections::HashMap<String, &str> =
                std::collections::HashMap::new();
            if let Some(inputs) = &state_machine.inputs {
                for input in inputs {
                    let name = match input {
                        InputSpec::Number { name, .. } => name,
                        InputSpec::Bool { name, .. } => name,
                        InputSpec::Trigger { name } => name,
                    };
                    if input_names.contains_key(name) {
                        return Err(format!(
                            "duplicate state machine input '{}' in '{}'",
                            name, state_machine.name
                        ));
                    }
                    let kind = match input {
                        InputSpec::Number { .. } => "number",
                        InputSpec::Bool { .. } => "bool",
                        InputSpec::Trigger { .. } => "trigger",
                    };
                    input_names.insert(name.clone(), kind);
                }
            }

            if let Some(listeners) = &state_machine.listeners {
                for listener in listeners {
                    if !object_names.contains(&listener.target) {
                        return Err(format!(
                            "unknown target referenced in state machine listener: '{}'",
                            listener.target
                        ));
                    }
                    if let Some(actions) = &listener.actions {
                        for action in actions {
                            match action {
                                ListenerActionSpec::BoolChange { input, value } => {
                                    match input_names.get(input.as_str()) {
                                        Some(&"bool") => {}
                                        Some(kind) => {
                                            return Err(format!(
                                                "listener bool_change targets {} input '{}', expected bool",
                                                kind, input
                                            ));
                                        }
                                        None => {
                                            return Err(format!(
                                                "unknown input referenced in listener action: '{}'",
                                                input
                                            ));
                                        }
                                    }
                                    if let Some(value) = value
                                        && json_value_to_u64(value).is_none()
                                    {
                                        return Err(format!(
                                            "listener bool_change value for input '{}' must be bool or unsigned integer",
                                            input
                                        ));
                                    }
                                }
                                ListenerActionSpec::TriggerChange { input } => {
                                    match input_names.get(input.as_str()) {
                                        Some(&"trigger") => {}
                                        Some(kind) => {
                                            return Err(format!(
                                                "listener trigger_change targets {} input '{}', expected trigger",
                                                kind, input
                                            ));
                                        }
                                        None => {
                                            return Err(format!(
                                                "unknown input referenced in listener action: '{}'",
                                                input
                                            ));
                                        }
                                    }
                                }
                                ListenerActionSpec::NumberChange { input, value } => {
                                    match input_names.get(input.as_str()) {
                                        Some(&"number") => {}
                                        Some(kind) => {
                                            return Err(format!(
                                                "listener number_change targets {} input '{}', expected number",
                                                kind, input
                                            ));
                                        }
                                        None => {
                                            return Err(format!(
                                                "unknown input referenced in listener action: '{}'",
                                                input
                                            ));
                                        }
                                    }
                                    if let Some(value) = value
                                        && json_value_to_f32(value).is_none()
                                    {
                                        return Err(format!(
                                            "listener number_change value for input '{}' must be numeric",
                                            input
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for layer in &state_machine.layers {
                for state in &layer.states {
                    match state {
                        StateSpec::Animation { animation }
                            if !animation_names.contains(animation) =>
                        {
                            return Err(format!(
                                "unknown animation referenced in state machine '{}': '{}'",
                                state_machine.name, animation
                            ));
                        }
                        StateSpec::BlendState {
                            children: Some(children),
                        } => {
                            validate_blend_state_children(
                                children,
                                animation_names.len(),
                                state_machine.name.as_str(),
                            )?;
                        }
                        StateSpec::BlendStateDirect {
                            children: Some(children),
                        } => {
                            validate_blend_state_direct_children(
                                children,
                                animation_names.len(),
                                state_machine.inputs.as_deref(),
                                state_machine.name.as_str(),
                            )?;
                        }
                        StateSpec::BlendState1d { input_id, children } => {
                            if let Some(input_id) = input_id {
                                validate_number_input(
                                    *input_id,
                                    state_machine.inputs.as_deref(),
                                    "blend_state_1d input_id",
                                    state_machine.name.as_str(),
                                )?;
                            }
                            if let Some(children) = children {
                                validate_blend_state_1d_children(
                                    children,
                                    animation_names.len(),
                                    state_machine.name.as_str(),
                                )?;
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(transitions) = &layer.transitions {
                    for transition in transitions {
                        if transition.from >= layer.states.len() {
                            return Err(format!(
                                "transition source index {} out of bounds (layer has {} states)",
                                transition.from,
                                layer.states.len()
                            ));
                        }
                        if transition.to >= layer.states.len() {
                            return Err(format!(
                                "transition target index {} out of bounds (layer has {} states)",
                                transition.to,
                                layer.states.len()
                            ));
                        }

                        if let Some(conditions) = &transition.conditions {
                            for condition in conditions {
                                if !input_names.contains_key(&condition.input) {
                                    return Err(format!(
                                        "unknown input referenced in condition: '{}'",
                                        condition.input
                                    ));
                                }
                                if let Some(op) = condition.op.as_deref()
                                    && !condition_op_is_valid(op)
                                {
                                    return Err(format!(
                                        "unknown condition operator '{}' in state machine '{}'",
                                        op, state_machine.name
                                    ));
                                }
                                if let Some(serde_json::Value::String(color)) =
                                    condition.value.as_ref()
                                {
                                    parse_color(color)?;
                                }
                            }
                        }

                        if let Some(children) = &transition.children {
                            validate_transition_children(children, state_machine.name.as_str())?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub(crate) fn validate_object_spec(
    spec: &ObjectSpec,
    object_names: &mut HashSet<String>,
    parent_kind: &ParentKind,
) -> Result<(), String> {
    match spec {
        ObjectSpec::Shape { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Shape)?;
                }
            }
        }
        ObjectSpec::Solo {
            name,
            children,
            active_component,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                let mut child_names: HashSet<String> = HashSet::new();
                for child in children {
                    match child {
                        ObjectSpec::Shape { name, .. }
                        | ObjectSpec::Solo { name, .. }
                        | ObjectSpec::Ellipse { name, .. }
                        | ObjectSpec::Rectangle { name, .. }
                        | ObjectSpec::Triangle { name, .. }
                        | ObjectSpec::Polygon { name, .. }
                        | ObjectSpec::Star { name, .. }
                        | ObjectSpec::Fill { name, .. }
                        | ObjectSpec::Stroke { name, .. }
                        | ObjectSpec::SolidColor { name, .. }
                        | ObjectSpec::LinearGradient { name, .. }
                        | ObjectSpec::RadialGradient { name, .. }
                        | ObjectSpec::Node { name, .. }
                        | ObjectSpec::Image { name, .. }
                        | ObjectSpec::Path { name, .. }
                        | ObjectSpec::PointsPath { name, .. }
                        | ObjectSpec::StraightVertex { name, .. }
                        | ObjectSpec::CubicMirroredVertex { name, .. }
                        | ObjectSpec::CubicDetachedVertex { name, .. }
                        | ObjectSpec::CubicAsymmetricVertex { name, .. }
                        | ObjectSpec::TrimPath { name, .. }
                        | ObjectSpec::NestedArtboard { name, .. }
                        | ObjectSpec::NestedStateMachine { name, .. }
                        | ObjectSpec::NestedSimpleAnimation { name, .. }
                        | ObjectSpec::Event { name, .. }
                        | ObjectSpec::Bone { name, .. }
                        | ObjectSpec::RootBone { name, .. }
                        | ObjectSpec::Skin { name, .. }
                        | ObjectSpec::Tendon { name, .. }
                        | ObjectSpec::Weight { name, .. }
                        | ObjectSpec::CubicWeight { name, .. }
                        | ObjectSpec::IkConstraint { name, .. }
                        | ObjectSpec::DistanceConstraint { name, .. }
                        | ObjectSpec::TransformConstraint { name, .. }
                        | ObjectSpec::TranslationConstraint { name, .. }
                        | ObjectSpec::ScaleConstraint { name, .. }
                        | ObjectSpec::RotationConstraint { name, .. }
                        | ObjectSpec::FollowPathConstraint { name, .. }
                        | ObjectSpec::ClippingShape { name, .. }
                        | ObjectSpec::DrawRules { name, .. }
                        | ObjectSpec::DrawTarget { name, .. }
                        | ObjectSpec::Joystick { name, .. }
                        | ObjectSpec::Text { name, .. }
                        | ObjectSpec::TextStyle { name, .. }
                        | ObjectSpec::TextValueRun { name, .. }
                        | ObjectSpec::ImageAsset { name, .. }
                        | ObjectSpec::FontAsset { name, .. }
                        | ObjectSpec::AudioAsset { name, .. }
                        | ObjectSpec::LayoutComponent { name, .. }
                        | ObjectSpec::LayoutComponentStyle { name, .. }
                        | ObjectSpec::ViewModel { name, .. }
                        | ObjectSpec::ViewModelProperty { name, .. }
                        | ObjectSpec::TextModifierGroup { name, .. } => {
                            child_names.insert(name.clone());
                        }
                        ObjectSpec::GradientStop { name, .. } => {
                            if let Some(name) = name {
                                child_names.insert(name.clone());
                            }
                        }
                        ObjectSpec::DataBind { .. }
                        | ObjectSpec::ViewModelInstance { .. }
                        | ObjectSpec::ViewModelInstanceValue { .. }
                        | ObjectSpec::ViewModelInstanceColor { .. }
                        | ObjectSpec::ViewModelInstanceString { .. }
                        | ObjectSpec::ViewModelInstanceNumber { .. }
                        | ObjectSpec::ViewModelInstanceBoolean { .. }
                        | ObjectSpec::ViewModelInstanceEnum { .. }
                        | ObjectSpec::ViewModelInstanceList
                        | ObjectSpec::ViewModelInstanceListItem { .. }
                        | ObjectSpec::ViewModelInstanceViewModel { .. }
                        | ObjectSpec::TextModifierRange { .. }
                        | ObjectSpec::TextVariationModifier { .. }
                        | ObjectSpec::TextStyleFeature { .. } => {}
                    }
                    validate_object_spec(child, object_names, &ParentKind::Artboard)?;
                }
                if let Some(active_component_name) = active_component
                    && !child_names.contains(active_component_name)
                {
                    return Err(format!(
                        "solo '{}' active_component '{}' must reference a direct child",
                        name, active_component_name
                    ));
                }
            }
        }
        ObjectSpec::Ellipse {
            name,
            width,
            height,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if *width < 0.0 {
                return Err(format!("ellipse '{}' width must be non-negative", name));
            }
            if *height < 0.0 {
                return Err(format!("ellipse '{}' height must be non-negative", name));
            }
        }
        ObjectSpec::Rectangle {
            name,
            width,
            height,
            corner_radius,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if *width < 0.0 {
                return Err(format!("rectangle '{}' width must be non-negative", name));
            }
            if *height < 0.0 {
                return Err(format!("rectangle '{}' height must be non-negative", name));
            }
            if let Some(corner_radius) = corner_radius
                && *corner_radius < 0.0
            {
                return Err(format!(
                    "rectangle '{}' corner_radius must be non-negative",
                    name
                ));
            }
        }
        ObjectSpec::Triangle {
            name,
            width,
            height,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if *width < 0.0 {
                return Err(format!("'{}' width must be non-negative", name));
            }
            if *height < 0.0 {
                return Err(format!("'{}' height must be non-negative", name));
            }
        }
        ObjectSpec::Polygon {
            name,
            width,
            height,
            points,
            corner_radius,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if *width < 0.0 {
                return Err(format!("'{}' width must be non-negative", name));
            }
            if *height < 0.0 {
                return Err(format!("'{}' height must be non-negative", name));
            }
            if let Some(p) = points
                && *p == 0
            {
                return Err(format!("polygon '{}' points must be greater than 0", name));
            }
            if let Some(cr) = corner_radius
                && *cr < 0.0
            {
                return Err(format!("'{}' corner_radius must be non-negative", name));
            }
        }
        ObjectSpec::Star {
            name,
            width,
            height,
            points,
            corner_radius,
            inner_radius,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if *width < 0.0 {
                return Err(format!("'{}' width must be non-negative", name));
            }
            if *height < 0.0 {
                return Err(format!("'{}' height must be non-negative", name));
            }
            if let Some(p) = points
                && *p == 0
            {
                return Err(format!("star '{}' points must be greater than 0", name));
            }
            if let Some(cr) = corner_radius
                && *cr < 0.0
            {
                return Err(format!("'{}' corner_radius must be non-negative", name));
            }
            if let Some(ir) = inner_radius
                && *ir < 0.0
            {
                return Err(format!("'{}' inner_radius must be non-negative", name));
            }
        }
        ObjectSpec::Fill {
            name,
            fill_rule,
            children,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if let Some(fill_rule) = fill_rule {
                parse_fill_rule(fill_rule).map_err(|e| format!("fill '{}': {}", name, e))?;
            }
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Fill)?;
                }
            }
        }
        ObjectSpec::Stroke {
            name,
            thickness,
            cap,
            join,
            children,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if let Some(thickness) = thickness
                && *thickness < 0.0
            {
                return Err(format!("stroke '{}' thickness must be non-negative", name));
            }
            if let Some(cap) = cap {
                parse_stroke_cap(cap).map_err(|e| format!("stroke '{}': {}", name, e))?;
            }
            if let Some(join) = join {
                parse_stroke_join(join).map_err(|e| format!("stroke '{}': {}", name, e))?;
            }
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Stroke)?;
                }
            }
        }
        ObjectSpec::SolidColor { name, color } => {
            ensure_unique_name(name, object_names)?;
            parse_color(color)?;
        }
        ObjectSpec::LinearGradient { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Gradient)?;
                }
            }
        }
        ObjectSpec::RadialGradient { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Gradient)?;
                }
            }
        }
        ObjectSpec::GradientStop {
            name,
            color,
            position,
        } => {
            let effective_name = name
                .clone()
                .unwrap_or_else(|| format!("gradient_stop_{}", object_names.len()));
            ensure_unique_name(&effective_name, object_names)?;
            parse_color(color)?;
            if !(0.0..=1.0).contains(position) {
                return Err(format!(
                    "gradient stop '{}' position must be between 0 and 1",
                    effective_name
                ));
            }
        }
        ObjectSpec::Node { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::Image { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::Path { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::PointsPath { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if !matches!(parent_kind, ParentKind::Shape) {
                return Err(format!("points_path '{}' must be a child of a shape", name));
            }
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::PointsPath)?;
                }
            }
        }
        ObjectSpec::StraightVertex { name, radius, .. } => {
            ensure_unique_name(name, object_names)?;
            if !matches!(parent_kind, ParentKind::PointsPath) {
                return Err(format!(
                    "straight_vertex '{}' must be a child of points_path",
                    name
                ));
            }
            if let Some(radius) = radius
                && *radius < 0.0
            {
                return Err(format!(
                    "straight_vertex '{}' radius must be non-negative",
                    name
                ));
            }
        }
        ObjectSpec::CubicMirroredVertex { name, distance, .. } => {
            ensure_unique_name(name, object_names)?;
            if !matches!(parent_kind, ParentKind::PointsPath) {
                return Err(format!(
                    "cubic_mirrored_vertex '{}' must be a child of points_path",
                    name
                ));
            }
            if let Some(distance) = distance
                && *distance < 0.0
            {
                return Err(format!(
                    "cubic_mirrored_vertex '{}' distance must be non-negative",
                    name
                ));
            }
        }
        ObjectSpec::CubicDetachedVertex {
            name,
            in_distance,
            out_distance,
            ..
        }
        | ObjectSpec::CubicAsymmetricVertex {
            name,
            in_distance,
            out_distance,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if !matches!(parent_kind, ParentKind::PointsPath) {
                return Err(format!(
                    "cubic vertex '{}' must be a child of points_path",
                    name
                ));
            }
            if let Some(distance) = in_distance
                && *distance < 0.0
            {
                return Err(format!(
                    "cubic vertex '{}' in_distance must be non-negative",
                    name
                ));
            }
            if let Some(distance) = out_distance
                && *distance < 0.0
            {
                return Err(format!(
                    "cubic vertex '{}' out_distance must be non-negative",
                    name
                ));
            }
        }
        ObjectSpec::TrimPath {
            name,
            start,
            end,
            mode,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if !matches!(parent_kind, ParentKind::Fill | ParentKind::Stroke) {
                return Err(format!(
                    "trim_path '{}' must be a child of a fill or stroke, not a shape or artboard",
                    name
                ));
            }
            if let Some(start) = start
                && *start < 0.0
            {
                return Err(format!("trim_path '{}' start must be non-negative", name));
            }
            if let Some(end) = end
                && *end < 0.0
            {
                return Err(format!("trim_path '{}' end must be non-negative", name));
            }
            if let Some(mode) = mode {
                parse_trim_mode(mode).map_err(|e| format!("trim_path '{}': {}", name, e))?;
            }
        }
        ObjectSpec::NestedArtboard { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::NestedStateMachine { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::NestedSimpleAnimation { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::Event { name, children } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Artboard)?;
                }
            }
        }
        ObjectSpec::Bone { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Bone)?;
                }
            }
        }
        ObjectSpec::RootBone { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Bone)?;
                }
            }
        }
        ObjectSpec::Skin { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Bone)?;
                }
            }
        }
        ObjectSpec::Tendon { name, bone, .. } => {
            ensure_unique_name(name, object_names)?;
            if bone.is_none() {
                return Err(format!("tendon '{}' must reference a bone", name));
            }
        }
        ObjectSpec::Weight { name, .. } | ObjectSpec::CubicWeight { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::IkConstraint { name, target, .. }
        | ObjectSpec::DistanceConstraint { name, target, .. }
        | ObjectSpec::TransformConstraint { name, target, .. }
        | ObjectSpec::TranslationConstraint { name, target, .. }
        | ObjectSpec::ScaleConstraint { name, target, .. }
        | ObjectSpec::RotationConstraint { name, target, .. }
        | ObjectSpec::FollowPathConstraint { name, target, .. } => {
            ensure_unique_name(name, object_names)?;
            if target.is_none() {
                return Err(format!("constraint '{}' must specify a target", name));
            }
        }
        ObjectSpec::ClippingShape {
            name, fill_rule, ..
        } => {
            ensure_unique_name(name, object_names)?;
            if let Some(fr) = fill_rule {
                parse_fill_rule(fr).map_err(|e| format!("clipping_shape '{}': {}", name, e))?;
            }
        }
        ObjectSpec::DrawTarget { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::Joystick {
            name,
            width,
            height,
            ..
        } => {
            ensure_unique_name(name, object_names)?;
            if let Some(w) = width
                && *w < 0.0
            {
                return Err(format!("joystick '{}' width must be non-negative", name));
            }
            if let Some(h) = height
                && *h < 0.0
            {
                return Err(format!("joystick '{}' height must be non-negative", name));
            }
        }
        ObjectSpec::DrawRules { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Artboard)?;
                }
            }
        }
        ObjectSpec::Text { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::Text)?;
                }
            }
        }
        ObjectSpec::TextStyle { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_text_style_child_spec(child);
                }
            }
        }
        ObjectSpec::TextValueRun { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::ImageAsset { name, .. }
        | ObjectSpec::FontAsset { name, .. }
        | ObjectSpec::AudioAsset { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::LayoutComponent { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::LayoutComponent)?;
                }
            }
        }
        ObjectSpec::LayoutComponentStyle { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::ViewModel { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_object_spec(child, object_names, &ParentKind::ViewModel)?;
                }
            }
        }
        ObjectSpec::ViewModelProperty { name, .. } => {
            ensure_unique_name(name, object_names)?;
        }
        ObjectSpec::DataBind { .. } => {}
        ObjectSpec::ViewModelInstance { view_model_id } => {
            required_u64_field(*view_model_id, "view_model_instance", "view_model_id")?;
        }
        ObjectSpec::ViewModelInstanceValue {
            view_model_property_id,
        }
        | ObjectSpec::ViewModelInstanceColor {
            view_model_property_id,
            ..
        }
        | ObjectSpec::ViewModelInstanceString {
            view_model_property_id,
            ..
        }
        | ObjectSpec::ViewModelInstanceNumber {
            view_model_property_id,
            ..
        }
        | ObjectSpec::ViewModelInstanceBoolean {
            view_model_property_id,
            ..
        } => {
            required_u64_field(
                *view_model_property_id,
                "view_model_instance_value",
                "view_model_property_id",
            )?;
        }
        ObjectSpec::ViewModelInstanceEnum {
            view_model_property_id,
            value,
        } => {
            required_u64_field(
                *view_model_property_id,
                "view_model_instance_enum",
                "view_model_property_id",
            )?;
            required_u64_field(*value, "view_model_instance_enum", "value")?;
        }
        ObjectSpec::ViewModelInstanceList => {}
        ObjectSpec::ViewModelInstanceListItem {
            view_model_id,
            view_model_instance_id,
        } => {
            required_u64_field(
                *view_model_id,
                "view_model_instance_list_item",
                "view_model_id",
            )?;
            required_u64_field(
                *view_model_instance_id,
                "view_model_instance_list_item",
                "view_model_instance_id",
            )?;
        }
        ObjectSpec::ViewModelInstanceViewModel {
            view_model_property_id,
            value,
        } => {
            required_u64_field(
                *view_model_property_id,
                "view_model_instance_view_model",
                "view_model_property_id",
            )?;
            required_u64_field(*value, "view_model_instance_view_model", "value")?;
        }
        ObjectSpec::TextModifierRange { .. } => {
            return Err(
                "text_modifier_range must be nested under text_modifier_group.children".to_string(),
            );
        }
        ObjectSpec::TextVariationModifier { .. } => {
            return Err(
                "text_variation_modifier must be nested under text_modifier_group.children"
                    .to_string(),
            );
        }
        ObjectSpec::TextStyleFeature { .. } => {
            return Err("text_style_feature must be nested under text_style.children".to_string());
        }
        ObjectSpec::TextModifierGroup { name, children, .. } => {
            ensure_unique_name(name, object_names)?;
            if let Some(children) = children {
                for child in children {
                    validate_text_modifier_group_child_spec(child);
                }
            }
        }
    }

    Ok(())
}

pub(crate) fn collect_object_type_key(
    spec: &ObjectSpec,
    object_type_keys: &mut HashMap<String, u16>,
) {
    match spec {
        ObjectSpec::Shape { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::SHAPE);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::Solo { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::SOLO);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::Ellipse { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::ELLIPSE);
        }
        ObjectSpec::Rectangle { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::RECTANGLE);
        }
        ObjectSpec::Triangle { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::TRIANGLE);
        }
        ObjectSpec::Polygon { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::POLYGON);
        }
        ObjectSpec::Star { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::STAR);
        }
        ObjectSpec::Fill { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::FILL);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::Stroke { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::STROKE);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::SolidColor { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::SOLID_COLOR);
        }
        ObjectSpec::LinearGradient { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::LINEAR_GRADIENT);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::RadialGradient { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::RADIAL_GRADIENT);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::GradientStop { name, .. } => {
            if let Some(name) = name {
                object_type_keys.insert(name.clone(), type_keys::GRADIENT_STOP);
            }
        }
        ObjectSpec::Node { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::NODE);
        }
        ObjectSpec::Image { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::IMAGE);
        }
        ObjectSpec::Path { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::PATH);
        }
        ObjectSpec::PointsPath { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::POINTS_PATH);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::StraightVertex { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::STRAIGHT_VERTEX);
        }
        ObjectSpec::CubicMirroredVertex { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::CUBIC_MIRRORED_VERTEX);
        }
        ObjectSpec::CubicDetachedVertex { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::CUBIC_DETACHED_VERTEX);
        }
        ObjectSpec::CubicAsymmetricVertex { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::CUBIC_ASYMMETRIC_VERTEX);
        }
        ObjectSpec::TrimPath { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::TRIM_PATH);
        }
        ObjectSpec::NestedArtboard { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::NESTED_ARTBOARD);
        }
        ObjectSpec::NestedStateMachine { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::NESTED_STATE_MACHINE);
        }
        ObjectSpec::NestedSimpleAnimation { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::NESTED_SIMPLE_ANIMATION);
        }
        ObjectSpec::Event { name, children } => {
            object_type_keys.insert(name.clone(), type_keys::EVENT);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::Bone { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::BONE);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::RootBone { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::ROOT_BONE);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::Skin { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::SKIN);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::Tendon { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::TENDON);
        }
        ObjectSpec::Weight { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::WEIGHT);
        }
        ObjectSpec::CubicWeight { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::CUBIC_WEIGHT);
        }
        ObjectSpec::IkConstraint { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::IK_CONSTRAINT);
        }
        ObjectSpec::DistanceConstraint { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::DISTANCE_CONSTRAINT);
        }
        ObjectSpec::TransformConstraint { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::TRANSFORM_CONSTRAINT);
        }
        ObjectSpec::TranslationConstraint { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::TRANSLATION_CONSTRAINT);
        }
        ObjectSpec::ScaleConstraint { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::SCALE_CONSTRAINT);
        }
        ObjectSpec::RotationConstraint { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::ROTATION_CONSTRAINT);
        }
        ObjectSpec::FollowPathConstraint { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::FOLLOW_PATH_CONSTRAINT);
        }
        ObjectSpec::ClippingShape { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::CLIPPING_SHAPE);
        }
        ObjectSpec::DrawRules { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::DRAW_RULES);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::DrawTarget { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::DRAW_TARGET);
        }
        ObjectSpec::Joystick { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::JOYSTICK);
        }
        ObjectSpec::Text { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::TEXT);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::TextStyle { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::TEXT_STYLE);
        }
        ObjectSpec::TextValueRun { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::TEXT_VALUE_RUN);
        }
        ObjectSpec::ImageAsset { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::IMAGE_ASSET);
        }
        ObjectSpec::FontAsset { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::FONT_ASSET);
        }
        ObjectSpec::AudioAsset { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::AUDIO_ASSET);
        }
        ObjectSpec::LayoutComponent { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::LAYOUT_COMPONENT);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::LayoutComponentStyle { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::LAYOUT_COMPONENT_STYLE);
        }
        ObjectSpec::ViewModel { name, children, .. } => {
            object_type_keys.insert(name.clone(), type_keys::VIEW_MODEL);
            if let Some(children) = children {
                for child in children {
                    collect_object_type_key(child, object_type_keys);
                }
            }
        }
        ObjectSpec::ViewModelProperty { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::VIEW_MODEL_PROPERTY);
        }
        ObjectSpec::DataBind { .. } => {}
        ObjectSpec::ViewModelInstance { .. }
        | ObjectSpec::ViewModelInstanceValue { .. }
        | ObjectSpec::ViewModelInstanceColor { .. }
        | ObjectSpec::ViewModelInstanceString { .. }
        | ObjectSpec::ViewModelInstanceNumber { .. }
        | ObjectSpec::ViewModelInstanceBoolean { .. }
        | ObjectSpec::ViewModelInstanceEnum { .. }
        | ObjectSpec::ViewModelInstanceList
        | ObjectSpec::ViewModelInstanceListItem { .. }
        | ObjectSpec::ViewModelInstanceViewModel { .. }
        | ObjectSpec::TextModifierRange { .. }
        | ObjectSpec::TextVariationModifier { .. }
        | ObjectSpec::TextStyleFeature { .. } => {}
        ObjectSpec::TextModifierGroup { name, .. } => {
            object_type_keys.insert(name.clone(), type_keys::TEXT_MODIFIER_GROUP);
        }
    }
}

pub(crate) fn collect_nested_artboard_refs(children: &[ObjectSpec]) -> Vec<String> {
    let mut refs = Vec::new();
    for child in children {
        match child {
            ObjectSpec::NestedArtboard {
                source_artboard, ..
            } => {
                refs.push(source_artboard.clone());
            }
            ObjectSpec::Shape { children, .. }
            | ObjectSpec::Solo { children, .. }
            | ObjectSpec::Fill { children, .. }
            | ObjectSpec::Stroke { children, .. }
            | ObjectSpec::Event { children, .. }
            | ObjectSpec::PointsPath { children, .. }
            | ObjectSpec::Bone { children, .. }
            | ObjectSpec::RootBone { children, .. }
            | ObjectSpec::Skin { children, .. }
            | ObjectSpec::Text { children, .. }
            | ObjectSpec::LayoutComponent { children, .. }
            | ObjectSpec::ViewModel { children, .. }
            | ObjectSpec::DrawRules { children, .. } => {
                if let Some(kids) = children {
                    refs.extend(collect_nested_artboard_refs(kids));
                }
            }
            _ => {}
        }
    }
    refs
}

pub(crate) fn detect_artboard_cycles(
    artboard_deps: &HashMap<String, Vec<String>>,
) -> Result<(), String> {
    for start in artboard_deps.keys() {
        let mut visited: HashSet<&str> = HashSet::new();
        let mut stack = vec![start.as_str()];
        visited.insert(start.as_str());
        while let Some(current) = stack.pop() {
            if let Some(deps) = artboard_deps.get(current) {
                for dep in deps {
                    if dep == start {
                        return Err(format!(
                            "circular nested artboard reference detected: '{}' eventually references itself",
                            start
                        ));
                    }
                    if !visited.contains(dep.as_str()) {
                        visited.insert(dep.as_str());
                        stack.push(dep.as_str());
                    }
                }
            }
        }
    }
    Ok(())
}

fn validate_index(
    index: u64,
    len: usize,
    label: &str,
    state_machine_name: &str,
) -> Result<(), String> {
    if index >= len as u64 {
        return Err(format!(
            "{} {} out of bounds in state machine '{}' (len {})",
            label, index, state_machine_name, len
        ));
    }
    Ok(())
}

fn validate_number_input(
    index: u64,
    inputs: Option<&[InputSpec]>,
    label: &str,
    state_machine_name: &str,
) -> Result<(), String> {
    let inputs = inputs.unwrap_or(&[]);
    validate_index(index, inputs.len(), label, state_machine_name)?;
    match inputs.get(index as usize) {
        Some(InputSpec::Number { .. }) => Ok(()),
        Some(InputSpec::Bool { .. }) => Err(format!(
            "{} {} in state machine '{}' must reference a number input, found bool",
            label, index, state_machine_name
        )),
        Some(InputSpec::Trigger { .. }) => Err(format!(
            "{} {} in state machine '{}' must reference a number input, found trigger",
            label, index, state_machine_name
        )),
        None => Err(format!(
            "{} {} out of bounds in state machine '{}' (len {})",
            label,
            index,
            state_machine_name,
            inputs.len()
        )),
    }
}

fn validate_blend_state_children(
    children: &[BlendStateChildSpec],
    animation_count: usize,
    state_machine_name: &str,
) -> Result<(), String> {
    for child in children {
        let BlendStateChildSpec::BlendAnimation { animation_id } = child;
        validate_index(
            *animation_id,
            animation_count,
            "blend_animation animation_id",
            state_machine_name,
        )?;
    }
    Ok(())
}

fn validate_blend_state_direct_children(
    children: &[BlendStateDirectChildSpec],
    animation_count: usize,
    inputs: Option<&[InputSpec]>,
    state_machine_name: &str,
) -> Result<(), String> {
    for child in children {
        let BlendStateDirectChildSpec::BlendAnimationDirect {
            animation_id,
            input_id,
            ..
        } = child;
        validate_index(
            *animation_id,
            animation_count,
            "blend_animation_direct animation_id",
            state_machine_name,
        )?;
        if let Some(input_id) = input_id {
            validate_number_input(
                *input_id,
                inputs,
                "blend_animation_direct input_id",
                state_machine_name,
            )?;
        }
    }
    Ok(())
}

fn validate_blend_state_1d_children(
    children: &[BlendState1DChildSpec],
    animation_count: usize,
    state_machine_name: &str,
) -> Result<(), String> {
    for child in children {
        let BlendState1DChildSpec::BlendAnimation1D { animation_id, .. } = child;
        validate_index(
            *animation_id,
            animation_count,
            "blend_animation_1d animation_id",
            state_machine_name,
        )?;
    }
    Ok(())
}

fn validate_transition_children(
    children: &[TransitionChildSpec],
    state_machine_name: &str,
) -> Result<(), String> {
    for child in children {
        if let TransitionChildSpec::TransitionViewModelCondition { op_value } = child
            && let Some(op_value) = op_value
            && *op_value > 5
        {
            return Err(format!(
                "transition_view_model_condition op_value {} out of range in state machine '{}'",
                op_value, state_machine_name
            ));
        }
        if let TransitionChildSpec::TransitionValueColorComparator { value } = child {
            parse_color(value).map_err(|e| {
                format!(
                    "invalid transition_value_color_comparator in state machine '{}': {}",
                    state_machine_name, e
                )
            })?;
        }
    }
    Ok(())
}

fn ensure_unique_name(name: &str, object_names: &mut HashSet<String>) -> Result<(), String> {
    if object_names.contains(name) {
        return Err(format!("duplicate object name '{}'", name));
    }
    object_names.insert(name.to_string());
    Ok(())
}

fn validate_image_asset_references(children: &[ObjectSpec]) -> Result<(), String> {
    fn walk(spec: &ObjectSpec, image_assets_seen: &mut u64) -> Result<(), String> {
        match spec {
            ObjectSpec::ImageAsset { .. } => {
                *image_assets_seen += 1;
            }
            ObjectSpec::Image { name, asset_id, .. } => {
                if *asset_id >= *image_assets_seen {
                    return Err(format!(
                        "image '{}' references image asset index {} but only {} image asset(s) were defined before it",
                        name, asset_id, image_assets_seen
                    ));
                }
            }
            ObjectSpec::Shape { children, .. }
            | ObjectSpec::Solo { children, .. }
            | ObjectSpec::Fill { children, .. }
            | ObjectSpec::Stroke { children, .. }
            | ObjectSpec::Event { children, .. }
            | ObjectSpec::PointsPath { children, .. }
            | ObjectSpec::LinearGradient { children, .. }
            | ObjectSpec::RadialGradient { children, .. }
            | ObjectSpec::Bone { children, .. }
            | ObjectSpec::RootBone { children, .. }
            | ObjectSpec::Text { children, .. }
            | ObjectSpec::LayoutComponent { children, .. }
            | ObjectSpec::ViewModel { children, .. }
            | ObjectSpec::DrawRules { children, .. } => {
                if let Some(children) = children {
                    for child in children {
                        walk(child, image_assets_seen)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    let mut image_assets_seen = 0;
    for child in children {
        walk(child, &mut image_assets_seen)?;
    }
    Ok(())
}

fn validate_text_style_child_spec(spec: &TextStyleChildSpec) {
    match spec {
        TextStyleChildSpec::TextStyleFeature { .. } => {}
    }
}

fn validate_text_modifier_group_child_spec(spec: &TextModifierGroupChildSpec) {
    match spec {
        TextModifierGroupChildSpec::TextModifierRange { .. }
        | TextModifierGroupChildSpec::TextVariationModifier { .. } => {}
    }
}
