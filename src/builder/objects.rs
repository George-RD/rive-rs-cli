use std::collections::HashMap;

use crate::objects::artboard::NestedArtboard;
use crate::objects::assets::{AudioAsset, FontAsset, ImageAsset};
use crate::objects::bones::{Bone, CubicWeight, RootBone, Skin, Tendon, Weight};
use crate::objects::constraints::{
    DistanceConstraint, FollowPathConstraint, IKConstraint, RotationConstraint, ScaleConstraint,
    TransformConstraint, TranslationConstraint,
};
use crate::objects::core::RiveObject;
use crate::objects::data_binding::{
    DataBind, ViewModel, ViewModelInstance, ViewModelInstanceBoolean, ViewModelInstanceColor,
    ViewModelInstanceEnum, ViewModelInstanceList, ViewModelInstanceListItem,
    ViewModelInstanceNumber, ViewModelInstanceString, ViewModelInstanceValue,
    ViewModelInstanceViewModel, ViewModelProperty,
};
use crate::objects::layout::{LayoutComponent, LayoutComponentStyle};
use crate::objects::shapes::{
    ClippingShape, CubicAsymmetricVertexObject, CubicDetachedVertexObject,
    CubicMirroredVertexObject, DrawRules, DrawTarget, Ellipse, Fill, GradientStop, Image, Joystick,
    LinearGradient, Node, PathObject, PointsPathObject, Polygon, RadialGradient, Rectangle, Shape,
    SolidColor, Solo, Star, StraightVertexObject, Stroke, Triangle, TrimPath,
};
use crate::objects::state_machine::{Event, NestedSimpleAnimation, NestedStateMachine};
use crate::objects::text::{
    Text, TextModifierGroup, TextModifierRange, TextStyle, TextStyleFeature, TextValueRun,
    TextVariationModifier,
};

use super::parsers::{
    parse_color, parse_fill_rule, parse_stroke_cap, parse_stroke_join, parse_trim_mode,
    required_u64_field,
};
use super::spec::{ObjectSpec, TextModifierGroupChildSpec, TextStyleChildSpec};

#[allow(clippy::too_many_arguments)]
pub(crate) fn append_object(
    spec: &ObjectSpec,
    parent_index: usize,
    artboard_start: usize,
    objects: &mut Vec<Box<dyn RiveObject>>,
    name_to_index: &mut HashMap<String, usize>,
    artboard_name_to_index: &HashMap<String, usize>,
    current_artboard_name: &str,
    animation_name_to_index: &HashMap<String, usize>,
) -> Result<(), String> {
    let object_index = objects.len();
    let parent_id = parent_index
        .checked_sub(artboard_start)
        .ok_or("internal error: parent index precedes artboard start".to_string())?
        as u64;

    match spec {
        ObjectSpec::Shape {
            name,
            x,
            y,
            children,
        } => {
            let mut shape = Shape::new(name.clone(), parent_id);
            if let Some(x) = x {
                shape.x = *x;
            }
            if let Some(y) = y {
                shape.y = *y;
            }
            objects.push(Box::new(shape));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                        animation_name_to_index,
                    )?;
                }
            }
        }
        ObjectSpec::Solo {
            name,
            x,
            y,
            children,
            active_component,
        } => {
            let mut solo = Solo {
                name: name.clone(),
                parent_id,
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
                active_component_id: 0,
            };
            objects.push(Box::new(Solo {
                name: solo.name.clone(),
                parent_id: solo.parent_id,
                x: solo.x,
                y: solo.y,
                active_component_id: 0,
            }));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                        animation_name_to_index,
                    )?;
                }
            }
            if let Some(active_component_name) = active_component {
                let active_global = *name_to_index.get(active_component_name).ok_or_else(|| {
                    format!(
                        "solo '{}' references unknown active_component '{}'",
                        name, active_component_name
                    )
                })?;
                solo.active_component_id =
                    active_global.checked_sub(artboard_start).ok_or_else(|| {
                        format!(
                            "solo '{}' active_component '{}' precedes current artboard",
                            name, active_component_name
                        )
                    })? as u64;
                objects[object_index] = Box::new(solo);
            }
        }
        ObjectSpec::Ellipse {
            name,
            width,
            height,
            origin_x,
            origin_y,
        } => {
            let mut ellipse = Ellipse::new(name.clone(), parent_id, *width, *height);
            if let Some(origin_x) = origin_x {
                ellipse.origin_x = *origin_x;
            }
            if let Some(origin_y) = origin_y {
                ellipse.origin_y = *origin_y;
            }
            objects.push(Box::new(ellipse));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Rectangle {
            name,
            width,
            height,
            corner_radius,
            origin_x,
            origin_y,
        } => {
            let mut rectangle = Rectangle::new(name.clone(), parent_id, *width, *height);
            if let Some(origin_x) = origin_x {
                rectangle.origin_x = *origin_x;
            }
            if let Some(origin_y) = origin_y {
                rectangle.origin_y = *origin_y;
            }
            if let Some(corner_radius) = corner_radius {
                rectangle.corner_radius_tl = *corner_radius;
                rectangle.corner_radius_tr = *corner_radius;
                rectangle.corner_radius_bl = *corner_radius;
                rectangle.corner_radius_br = *corner_radius;
                rectangle.link_corner_radius = 1;
            }
            objects.push(Box::new(rectangle));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Triangle {
            name,
            width,
            height,
            origin_x,
            origin_y,
        } => {
            let mut triangle = Triangle::new(name.clone(), parent_id, *width, *height);
            if let Some(origin_x) = origin_x {
                triangle.origin_x = *origin_x;
            }
            if let Some(origin_y) = origin_y {
                triangle.origin_y = *origin_y;
            }
            objects.push(Box::new(triangle));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Polygon {
            name,
            width,
            height,
            origin_x,
            origin_y,
            points,
            corner_radius,
        } => {
            let mut polygon = Polygon::new(name.clone(), parent_id, *width, *height);
            if let Some(origin_x) = origin_x {
                polygon.origin_x = *origin_x;
            }
            if let Some(origin_y) = origin_y {
                polygon.origin_y = *origin_y;
            }
            if let Some(points) = points {
                polygon.points = *points;
            }
            if let Some(corner_radius) = corner_radius {
                polygon.corner_radius = *corner_radius;
            }
            objects.push(Box::new(polygon));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Star {
            name,
            width,
            height,
            origin_x,
            origin_y,
            points,
            corner_radius,
            inner_radius,
        } => {
            let mut star = Star::new(name.clone(), parent_id, *width, *height);
            if let Some(origin_x) = origin_x {
                star.polygon.origin_x = *origin_x;
            }
            if let Some(origin_y) = origin_y {
                star.polygon.origin_y = *origin_y;
            }
            if let Some(points) = points {
                star.polygon.points = *points;
            }
            if let Some(corner_radius) = corner_radius {
                star.polygon.corner_radius = *corner_radius;
            }
            if let Some(inner_radius) = inner_radius {
                star.inner_radius = *inner_radius;
            }
            objects.push(Box::new(star));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Fill {
            name,
            fill_rule,
            is_visible,
            children,
        } => {
            let mut fill = Fill::new(name.clone(), parent_id);
            if let Some(fill_rule) = fill_rule {
                fill.fill_rule = parse_fill_rule(fill_rule)?;
            }
            if let Some(false) = is_visible {
                fill.is_visible = 0;
            }
            objects.push(Box::new(fill));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                        animation_name_to_index,
                    )?;
                }
            }
        }
        ObjectSpec::Stroke {
            name,
            thickness,
            cap,
            join,
            is_visible,
            children,
        } => {
            let mut stroke = Stroke::new(name.clone(), parent_id, thickness.unwrap_or(1.0));
            if let Some(cap) = cap {
                stroke.cap = parse_stroke_cap(cap)?;
            }
            if let Some(join) = join {
                stroke.join = parse_stroke_join(join)?;
            }
            if let Some(false) = is_visible {
                stroke.is_visible = 0;
            }
            objects.push(Box::new(stroke));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                        animation_name_to_index,
                    )?;
                }
            }
        }
        ObjectSpec::SolidColor { name, color } => {
            let color_value = parse_color(color)?;
            objects.push(Box::new(SolidColor::new(
                name.clone(),
                parent_id,
                color_value,
            )));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::LinearGradient {
            name,
            start_x,
            start_y,
            end_x,
            end_y,
            children,
        } => {
            objects.push(Box::new(LinearGradient {
                name: name.clone(),
                parent_id,
                start_x: *start_x,
                start_y: *start_y,
                end_x: *end_x,
                end_y: *end_y,
                opacity: 1.0,
            }));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                        animation_name_to_index,
                    )?;
                }
            }
        }
        ObjectSpec::RadialGradient {
            name,
            start_x,
            start_y,
            end_x,
            end_y,
            children,
        } => {
            objects.push(Box::new(RadialGradient {
                name: name.clone(),
                parent_id,
                start_x: *start_x,
                start_y: *start_y,
                end_x: *end_x,
                end_y: *end_y,
                opacity: 1.0,
            }));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                        animation_name_to_index,
                    )?;
                }
            }
        }
        ObjectSpec::GradientStop {
            name,
            color,
            position,
        } => {
            let generated_name = name
                .clone()
                .unwrap_or_else(|| format!("gradient_stop_{}", name_to_index.len()));
            objects.push(Box::new(GradientStop {
                name: generated_name.clone(),
                parent_id,
                color: parse_color(color)?,
                position: *position,
            }));
            name_to_index.insert(generated_name, object_index);
        }
        ObjectSpec::Node { name, x, y } => {
            objects.push(Box::new(Node {
                name: name.clone(),
                parent_id,
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Image {
            name,
            asset_id,
            x,
            y,
        } => {
            let mut image = Image::new(name.clone(), parent_id, *asset_id);
            if let Some(v) = x {
                image.x = *v;
            }
            if let Some(v) = y {
                image.y = *v;
            }
            objects.push(Box::new(image));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Path { name, path_flags } => {
            objects.push(Box::new(PathObject {
                name: name.clone(),
                parent_id,
                path_flags: path_flags.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::PointsPath {
            name,
            x,
            y,
            is_closed,
            path_flags,
            children,
        } => {
            objects.push(Box::new(PointsPathObject {
                name: name.clone(),
                parent_id: Some(parent_id as u32),
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
                is_closed: is_closed.unwrap_or(false),
                path_flags: path_flags.unwrap_or(0) as u32,
            }));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                        animation_name_to_index,
                    )?;
                }
            }
        }
        ObjectSpec::StraightVertex { name, x, y, radius } => {
            objects.push(Box::new(StraightVertexObject {
                name: name.clone(),
                parent_id: Some(parent_id as u32),
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
                radius: radius.unwrap_or(0.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CubicMirroredVertex {
            name,
            x,
            y,
            rotation,
            distance,
        } => {
            objects.push(Box::new(CubicMirroredVertexObject {
                name: name.clone(),
                parent_id: Some(parent_id as u32),
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
                rotation: rotation.unwrap_or(0.0),
                distance: distance.unwrap_or(0.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CubicDetachedVertex {
            name,
            x,
            y,
            in_rotation,
            in_distance,
            out_rotation,
            out_distance,
        } => {
            objects.push(Box::new(CubicDetachedVertexObject {
                name: name.clone(),
                parent_id: Some(parent_id as u32),
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
                in_rotation: in_rotation.unwrap_or(0.0),
                in_distance: in_distance.unwrap_or(0.0),
                out_rotation: out_rotation.unwrap_or(0.0),
                out_distance: out_distance.unwrap_or(0.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CubicAsymmetricVertex {
            name,
            x,
            y,
            rotation,
            in_distance,
            out_distance,
        } => {
            objects.push(Box::new(CubicAsymmetricVertexObject {
                name: name.clone(),
                parent_id: Some(parent_id as u32),
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
                rotation: rotation.unwrap_or(0.0),
                in_distance: in_distance.unwrap_or(0.0),
                out_distance: out_distance.unwrap_or(0.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TrimPath {
            name,
            start,
            end,
            offset,
            mode,
        } => {
            let mut trim_path = TrimPath::new(name.clone(), parent_id);
            if let Some(start) = start {
                trim_path.start = *start;
            }
            if let Some(end) = end {
                trim_path.end = *end;
            }
            if let Some(offset) = offset {
                trim_path.offset = *offset;
            }
            if let Some(mode) = mode {
                let mode_val = parse_trim_mode(mode)?;
                trim_path
                    .set_mode(mode_val)
                    .map_err(|e| format!("trim_path '{}': {}", name, e))?;
            }
            objects.push(Box::new(trim_path));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NestedArtboard {
            name,
            source_artboard,
            x,
            y,
        } => {
            if source_artboard == current_artboard_name {
                return Err(format!(
                    "nested artboard '{}' cannot reference its own artboard '{}'",
                    name, source_artboard
                ));
            }
            let source_artboard_index =
                *artboard_name_to_index.get(source_artboard).ok_or_else(|| {
                    format!(
                        "nested artboard '{}' references unknown artboard '{}'",
                        name, source_artboard
                    )
                })?;
            objects.push(Box::new(NestedArtboard {
                name: name.clone(),
                parent_id,
                artboard_id: source_artboard_index as u64,
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NestedStateMachine { name, animation } => {
            let animation_id = *animation_name_to_index.get(animation).ok_or_else(|| {
                format!(
                    "nested_state_machine '{}' references unknown animation '{}'",
                    name, animation
                )
            })? as u64;
            objects.push(Box::new(NestedStateMachine {
                name: name.clone(),
                parent_id,
                animation_id,
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NestedSimpleAnimation {
            name,
            animation,
            speed,
            is_playing,
            mix,
        } => {
            let animation_id = *animation_name_to_index.get(animation).ok_or_else(|| {
                format!(
                    "nested_simple_animation '{}' references unknown animation '{}'",
                    name, animation
                )
            })? as u64;
            objects.push(Box::new(NestedSimpleAnimation {
                name: name.clone(),
                parent_id,
                animation_id,
                speed: speed.unwrap_or(1.0),
                is_playing: is_playing.unwrap_or(false),
                mix: mix.unwrap_or(1.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Event { name, children } => {
            objects.push(Box::new(Event {
                name: name.clone(),
                parent_id,
            }));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                        animation_name_to_index,
                    )?;
                }
            }
        }
        ObjectSpec::Bone {
            name,
            length,
            children,
        } => {
            let mut bone = Bone::new(name.clone(), parent_id);
            if let Some(length) = length {
                bone.length = *length;
            }
            objects.push(Box::new(bone));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                        animation_name_to_index,
                    )?;
                }
            }
        }
        ObjectSpec::RootBone {
            name,
            x,
            y,
            length,
            children,
        } => {
            let mut root_bone = RootBone::new(name.clone(), parent_id);
            if let Some(x) = x {
                root_bone.x = *x;
            }
            if let Some(y) = y {
                root_bone.y = *y;
            }
            if let Some(length) = length {
                root_bone.length = *length;
            }
            objects.push(Box::new(root_bone));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(
                        child,
                        object_index,
                        artboard_start,
                        objects,
                        name_to_index,
                        artboard_name_to_index,
                        current_artboard_name,
                        animation_name_to_index,
                    )?;
                }
            }
        }
        ObjectSpec::Skin {
            name,
            xx,
            yx,
            xy,
            yy,
            tx,
            ty,
            children,
        } => {
            let mut skin = Skin::new(name.clone(), parent_id);
            if let Some(xx) = xx { skin.xx = *xx; }
            if let Some(yx) = yx { skin.yx = *yx; }
            if let Some(xy) = xy { skin.xy = *xy; }
            if let Some(yy) = yy { skin.yy = *yy; }
            if let Some(tx) = tx { skin.tx = *tx; }
            if let Some(ty) = ty { skin.ty = *ty; }
            objects.push(Box::new(skin));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, artboard_start, objects, name_to_index, artboard_name_to_index, current_artboard_name, animation_name_to_index)?;
                }
            }
        }
        ObjectSpec::Tendon { name, bone, xx, yx, xy, yy, tx, ty } => {
            let mut tendon = Tendon::new(name.clone(), parent_id);
            if let Some(bone_name) = bone {
                let bone_global = *name_to_index.get(bone_name).ok_or_else(|| {
                    format!("tendon '{}' references unknown bone '{}'", name, bone_name)
                })?;
                tendon.bone_id = (bone_global - artboard_start) as u64;
            }
            if let Some(xx) = xx { tendon.xx = *xx; }
            if let Some(yx) = yx { tendon.yx = *yx; }
            if let Some(xy) = xy { tendon.xy = *xy; }
            if let Some(yy) = yy { tendon.yy = *yy; }
            if let Some(tx) = tx { tendon.tx = *tx; }
            if let Some(ty) = ty { tendon.ty = *ty; }
            objects.push(Box::new(tendon));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Weight { name, values, indices } => {
            let mut weight = Weight::new(name.clone(), parent_id);
            if let Some(values) = values { weight.values = *values; }
            if let Some(indices) = indices { weight.indices = *indices; }
            objects.push(Box::new(weight));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CubicWeight { name, in_values, in_indices, out_values, out_indices } => {
            let mut cubic_weight = CubicWeight::new(name.clone(), parent_id);
            if let Some(in_values) = in_values { cubic_weight.in_values = *in_values; }
            if let Some(in_indices) = in_indices { cubic_weight.in_indices = *in_indices; }
            if let Some(out_values) = out_values { cubic_weight.out_values = *out_values; }
            if let Some(out_indices) = out_indices { cubic_weight.out_indices = *out_indices; }
            objects.push(Box::new(cubic_weight));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::IkConstraint { name, target, strength, invert_direction, parent_bone_count } => {
            let mut ik = IKConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| format!("ik_constraint '{}' references unknown target '{}'", name, target_name))?;
                ik.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength { ik.strength = *s; }
            if let Some(inv) = invert_direction { ik.invert_direction = *inv; }
            if let Some(pbc) = parent_bone_count { ik.parent_bone_count = *pbc; }
            objects.push(Box::new(ik));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DistanceConstraint { name, target, strength, distance, mode_value } => {
            let mut dc = DistanceConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| format!("distance_constraint '{}' references unknown target '{}'", name, target_name))?;
                dc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength { dc.strength = *s; }
            if let Some(d) = distance { dc.distance = *d; }
            if let Some(mv) = mode_value { dc.mode_value = *mv; }
            objects.push(Box::new(dc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TransformConstraint { name, target, strength, source_space_value, dest_space_value, origin_x, origin_y } => {
            let mut tc = TransformConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| format!("transform_constraint '{}' references unknown target '{}'", name, target_name))?;
                tc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength { tc.strength = *s; }
            if let Some(ssv) = source_space_value { tc.source_space_value = *ssv; }
            if let Some(dsv) = dest_space_value { tc.dest_space_value = *dsv; }
            if let Some(ox) = origin_x { tc.origin_x = *ox; }
            if let Some(oy) = origin_y { tc.origin_y = *oy; }
            objects.push(Box::new(tc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TranslationConstraint { name, target, strength, source_space_value, dest_space_value, copy_factor, min_value, max_value, offset, does_copy, min, max, min_max_space_value, copy_factor_y, min_value_y, max_value_y, does_copy_y, min_y, max_y } => {
            let mut tlc = TranslationConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| format!("translation_constraint '{}' references unknown target '{}'", name, target_name))?;
                tlc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength { tlc.strength = *s; }
            if let Some(v) = source_space_value { tlc.source_space_value = *v; }
            if let Some(v) = dest_space_value { tlc.dest_space_value = *v; }
            if let Some(v) = copy_factor { tlc.copy_factor = *v; }
            if let Some(v) = min_value { tlc.min_value = *v; }
            if let Some(v) = max_value { tlc.max_value = *v; }
            if let Some(v) = offset { tlc.offset = *v; }
            if let Some(v) = does_copy { tlc.does_copy = *v; }
            if let Some(v) = min { tlc.min = *v; }
            if let Some(v) = max { tlc.max = *v; }
            if let Some(v) = min_max_space_value { tlc.min_max_space_value = *v; }
            if let Some(v) = copy_factor_y { tlc.copy_factor_y = *v; }
            if let Some(v) = min_value_y { tlc.min_value_y = *v; }
            if let Some(v) = max_value_y { tlc.max_value_y = *v; }
            if let Some(v) = does_copy_y { tlc.does_copy_y = *v; }
            if let Some(v) = min_y { tlc.min_y = *v; }
            if let Some(v) = max_y { tlc.max_y = *v; }
            objects.push(Box::new(tlc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScaleConstraint { name, target, strength, source_space_value, dest_space_value, copy_factor, min_value, max_value, offset, does_copy, min, max, min_max_space_value, copy_factor_y, min_value_y, max_value_y, does_copy_y, min_y, max_y } => {
            let mut sc = ScaleConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| format!("scale_constraint '{}' references unknown target '{}'", name, target_name))?;
                sc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength { sc.strength = *s; }
            if let Some(v) = source_space_value { sc.source_space_value = *v; }
            if let Some(v) = dest_space_value { sc.dest_space_value = *v; }
            if let Some(v) = copy_factor { sc.copy_factor = *v; }
            if let Some(v) = min_value { sc.min_value = *v; }
            if let Some(v) = max_value { sc.max_value = *v; }
            if let Some(v) = offset { sc.offset = *v; }
            if let Some(v) = does_copy { sc.does_copy = *v; }
            if let Some(v) = min { sc.min = *v; }
            if let Some(v) = max { sc.max = *v; }
            if let Some(v) = min_max_space_value { sc.min_max_space_value = *v; }
            if let Some(v) = copy_factor_y { sc.copy_factor_y = *v; }
            if let Some(v) = min_value_y { sc.min_value_y = *v; }
            if let Some(v) = max_value_y { sc.max_value_y = *v; }
            if let Some(v) = does_copy_y { sc.does_copy_y = *v; }
            if let Some(v) = min_y { sc.min_y = *v; }
            if let Some(v) = max_y { sc.max_y = *v; }
            objects.push(Box::new(sc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::RotationConstraint { name, target, strength, source_space_value, dest_space_value, copy_factor, min_value, max_value, offset, does_copy, min, max, min_max_space_value } => {
            let mut rc = RotationConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| format!("rotation_constraint '{}' references unknown target '{}'", name, target_name))?;
                rc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength { rc.strength = *s; }
            if let Some(v) = source_space_value { rc.source_space_value = *v; }
            if let Some(v) = dest_space_value { rc.dest_space_value = *v; }
            if let Some(v) = copy_factor { rc.copy_factor = *v; }
            if let Some(v) = min_value { rc.min_value = *v; }
            if let Some(v) = max_value { rc.max_value = *v; }
            if let Some(v) = offset { rc.offset = *v; }
            if let Some(v) = does_copy { rc.does_copy = *v; }
            if let Some(v) = min { rc.min = *v; }
            if let Some(v) = max { rc.max = *v; }
            if let Some(v) = min_max_space_value { rc.min_max_space_value = *v; }
            objects.push(Box::new(rc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::FollowPathConstraint { name, target, strength, source_space_value, dest_space_value, distance, orient, offset } => {
            let mut fpc = FollowPathConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| format!("follow_path_constraint '{}' references unknown target '{}'", name, target_name))?;
                fpc.target_id = (target_global - artboard_start) as u64;
            }
            if let Some(s) = strength { fpc.strength = *s; }
            if let Some(v) = source_space_value { fpc.source_space_value = *v; }
            if let Some(v) = dest_space_value { fpc.dest_space_value = *v; }
            if let Some(d) = distance { fpc.distance = *d; }
            if let Some(o) = orient { fpc.orient = *o; }
            if let Some(o) = offset { fpc.offset = *o; }
            objects.push(Box::new(fpc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ClippingShape { name, source, fill_rule, is_visible } => {
            let mut cs = ClippingShape::new(name.clone(), parent_id);
            if let Some(source_name) = source {
                let source_global = *name_to_index.get(source_name).ok_or_else(|| format!("clipping_shape '{}' references unknown source '{}'", name, source_name))?;
                cs.source_id = (source_global - artboard_start) as u64;
            }
            if let Some(fr) = fill_rule { cs.fill_rule = parse_fill_rule(fr)?; }
            if let Some(v) = is_visible { cs.is_visible = *v; }
            objects.push(Box::new(cs));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DrawRules { name, draw_target, children } => {
            objects.push(Box::new(DrawRules::new(name.clone(), parent_id)));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, artboard_start, objects, name_to_index, artboard_name_to_index, current_artboard_name, animation_name_to_index)?;
                }
            }
            if let Some(target_name) = draw_target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| format!("draw_rules '{}' references unknown draw_target '{}'", name, target_name))?;
                let mut dr = DrawRules::new(name.clone(), parent_id);
                dr.draw_target_id = (target_global - artboard_start) as u64;
                objects[object_index] = Box::new(dr);
            }
        }
        ObjectSpec::DrawTarget { name, drawable, placement_value } => {
            let mut dt = DrawTarget::new(name.clone(), parent_id);
            if let Some(drawable_name) = drawable {
                let drawable_global = *name_to_index.get(drawable_name).ok_or_else(|| format!("draw_target '{}' references unknown drawable '{}'", name, drawable_name))?;
                dt.drawable_id = (drawable_global - artboard_start) as u64;
            }
            if let Some(pv) = placement_value { dt.placement_value = *pv; }
            objects.push(Box::new(dt));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Joystick { name, x, y, x_id, y_id, pos_x, pos_y, width, height, origin_x, origin_y, flags, handle_source_id } => {
            let mut js = Joystick::new(name.clone(), parent_id);
            if let Some(v) = x { js.x = *v; }
            if let Some(v) = y { js.y = *v; }
            if let Some(v) = x_id { js.x_id = *v; }
            if let Some(v) = y_id { js.y_id = *v; }
            if let Some(v) = pos_x { js.pos_x = *v; }
            if let Some(v) = pos_y { js.pos_y = *v; }
            if let Some(v) = width { js.width = *v; }
            if let Some(v) = height { js.height = *v; }
            if let Some(v) = origin_x { js.origin_x = *v; }
            if let Some(v) = origin_y { js.origin_y = *v; }
            if let Some(v) = flags { js.flags = *v; }
            if let Some(v) = handle_source_id { js.handle_source_id = *v; }
            objects.push(Box::new(js));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Text { name, align_value, sizing_value, overflow_value, width, height, origin_x, origin_y, paragraph_spacing, origin_value, children } => {
            let mut text = Text::new(name.clone(), parent_id);
            if let Some(v) = align_value { text.align_value = *v; }
            if let Some(v) = sizing_value { text.sizing_value = *v; }
            if let Some(v) = overflow_value { text.overflow_value = *v; }
            if let Some(v) = width { text.width = *v; }
            if let Some(v) = height { text.height = *v; }
            if let Some(v) = origin_x { text.origin_x = *v; }
            if let Some(v) = origin_y { text.origin_y = *v; }
            if let Some(v) = paragraph_spacing { text.paragraph_spacing = *v; }
            if let Some(v) = origin_value { text.origin_value = *v; }
            objects.push(Box::new(text));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, artboard_start, objects, name_to_index, artboard_name_to_index, current_artboard_name, animation_name_to_index)?;
                }
            }
        }
        ObjectSpec::TextStyle { name, font_size, line_height, letter_spacing, font_asset_id, children } => {
            let mut style = TextStyle::new(name.clone(), parent_id);
            if let Some(v) = font_size { style.font_size = *v; }
            if let Some(v) = line_height { style.line_height = *v; }
            if let Some(v) = letter_spacing { style.letter_spacing = *v; }
            if let Some(v) = font_asset_id { style.font_asset_id = *v; }
            objects.push(Box::new(style));
            name_to_index.insert(name.clone(), object_index);
            let child_parent_id = object_index
                .checked_sub(artboard_start)
                .ok_or("internal error: parent index precedes artboard start".to_string())?
                as u64;
            if let Some(children) = children {
                for child in children {
                    append_text_style_child(child, child_parent_id, objects);
                }
            }
        }
        ObjectSpec::TextValueRun { name, text, style_id } => {
            let mut run = TextValueRun::new(name.clone(), parent_id, text.clone());
            if let Some(v) = style_id { run.style_id = *v; }
            objects.push(Box::new(run));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ImageAsset { name, asset_id, cdn_base_url } => {
            let mut asset = ImageAsset::new(name.clone());
            if let Some(v) = asset_id { asset.asset_id = *v; }
            if let Some(v) = cdn_base_url { asset.cdn_base_url = v.clone(); }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::FontAsset { name, asset_id, cdn_base_url } => {
            let mut asset = FontAsset::new(name.clone());
            if let Some(v) = asset_id { asset.asset_id = *v; }
            if let Some(v) = cdn_base_url { asset.cdn_base_url = v.clone(); }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::AudioAsset { name, asset_id, cdn_base_url } => {
            let mut asset = AudioAsset::new(name.clone());
            if let Some(v) = asset_id { asset.asset_id = *v; }
            if let Some(v) = cdn_base_url { asset.cdn_base_url = v.clone(); }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::LayoutComponent { name, clip, width, height, style_id, fractional_width, fractional_height, children } => {
            let mut lc = LayoutComponent::new(name.clone(), parent_id);
            if let Some(v) = clip { lc.clip = *v; }
            if let Some(v) = width { lc.width = *v; }
            if let Some(v) = height { lc.height = *v; }
            if let Some(v) = style_id { lc.style_id = *v; }
            if let Some(v) = fractional_width { lc.fractional_width = *v; }
            if let Some(v) = fractional_height { lc.fractional_height = *v; }
            objects.push(Box::new(lc));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, artboard_start, objects, name_to_index, artboard_name_to_index, current_artboard_name, animation_name_to_index)?;
                }
            }
        }
        ObjectSpec::LayoutComponentStyle { name, gap_horizontal, gap_vertical, max_width, max_height, min_width, min_height, border_left, border_right, border_top, border_bottom, margin_left, margin_right, margin_top, margin_bottom, padding_left, padding_right, padding_top, padding_bottom, position_left, position_right, position_top, position_bottom, flex_direction, flex_wrap, align_items, align_content, justify_content, display, position_type, overflow, intrinsically_sized, width_units, height_units, flex_grow, flex_shrink, flex_basis, aspect_ratio } => {
            let mut style = LayoutComponentStyle::new(name.clone(), parent_id);
            if let Some(v) = gap_horizontal { style.gap_horizontal = *v; }
            if let Some(v) = gap_vertical { style.gap_vertical = *v; }
            if let Some(v) = max_width { style.max_width = *v; }
            if let Some(v) = max_height { style.max_height = *v; }
            if let Some(v) = min_width { style.min_width = *v; }
            if let Some(v) = min_height { style.min_height = *v; }
            if let Some(v) = border_left { style.border_left = *v; }
            if let Some(v) = border_right { style.border_right = *v; }
            if let Some(v) = border_top { style.border_top = *v; }
            if let Some(v) = border_bottom { style.border_bottom = *v; }
            if let Some(v) = margin_left { style.margin_left = *v; }
            if let Some(v) = margin_right { style.margin_right = *v; }
            if let Some(v) = margin_top { style.margin_top = *v; }
            if let Some(v) = margin_bottom { style.margin_bottom = *v; }
            if let Some(v) = padding_left { style.padding_left = *v; }
            if let Some(v) = padding_right { style.padding_right = *v; }
            if let Some(v) = padding_top { style.padding_top = *v; }
            if let Some(v) = padding_bottom { style.padding_bottom = *v; }
            if let Some(v) = position_left { style.position_left = *v; }
            if let Some(v) = position_right { style.position_right = *v; }
            if let Some(v) = position_top { style.position_top = *v; }
            if let Some(v) = position_bottom { style.position_bottom = *v; }
            if let Some(v) = flex_direction { style.flex_direction = *v; }
            if let Some(v) = flex_wrap { style.flex_wrap = *v; }
            if let Some(v) = align_items { style.align_items = *v; }
            if let Some(v) = align_content { style.align_content = *v; }
            if let Some(v) = justify_content { style.justify_content = *v; }
            if let Some(v) = display { style.display = *v; }
            if let Some(v) = position_type { style.position_type = *v; }
            if let Some(v) = overflow { style.overflow = *v; }
            if let Some(v) = intrinsically_sized { style.intrinsically_sized = *v; }
            if let Some(v) = width_units { style.width_units = *v; }
            if let Some(v) = height_units { style.height_units = *v; }
            if let Some(v) = flex_grow { style.flex_grow = *v; }
            if let Some(v) = flex_shrink { style.flex_shrink = *v; }
            if let Some(v) = flex_basis { style.flex_basis = *v; }
            if let Some(v) = aspect_ratio { style.aspect_ratio = *v; }
            objects.push(Box::new(style));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ViewModel { name, children } => {
            let vm = ViewModel::new(name.clone(), parent_id);
            objects.push(Box::new(vm));
            name_to_index.insert(name.clone(), object_index);
            if let Some(children) = children {
                for child in children {
                    append_object(child, object_index, artboard_start, objects, name_to_index, artboard_name_to_index, current_artboard_name, animation_name_to_index)?;
                }
            }
        }
        ObjectSpec::ViewModelProperty { name, property_type_value } => {
            let vmp = ViewModelProperty::new(name.clone(), parent_id, property_type_value.unwrap_or(0));
            objects.push(Box::new(vmp));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataBind { property_key, flags, converter_id } => {
            let mut db = DataBind::new(*property_key, *flags);
            if let Some(v) = converter_id { db.converter_id = *v; }
            objects.push(Box::new(db));
        }
        ObjectSpec::ViewModelInstance { view_model_id } => {
            objects.push(Box::new(ViewModelInstance {
                view_model_id: required_u64_field(*view_model_id, "view_model_instance", "view_model_id")?,
            }));
        }
        ObjectSpec::ViewModelInstanceValue { view_model_property_id } => {
            objects.push(Box::new(ViewModelInstanceValue {
                view_model_property_id: required_u64_field(*view_model_property_id, "view_model_instance_value", "view_model_property_id")?,
            }));
        }
        ObjectSpec::ViewModelInstanceColor { view_model_property_id, value } => {
            let color = parse_color(value)?;
            objects.push(Box::new(ViewModelInstanceColor {
                view_model_property_id: required_u64_field(*view_model_property_id, "view_model_instance_color", "view_model_property_id")?,
                property_value: color,
            }));
        }
        ObjectSpec::ViewModelInstanceString { view_model_property_id, value } => {
            objects.push(Box::new(ViewModelInstanceString {
                view_model_property_id: required_u64_field(*view_model_property_id, "view_model_instance_string", "view_model_property_id")?,
                property_value: value.clone(),
            }));
        }
        ObjectSpec::ViewModelInstanceNumber { view_model_property_id, value } => {
            objects.push(Box::new(ViewModelInstanceNumber {
                view_model_property_id: required_u64_field(*view_model_property_id, "view_model_instance_number", "view_model_property_id")?,
                property_value: *value,
            }));
        }
        ObjectSpec::ViewModelInstanceBoolean { view_model_property_id, value } => {
            objects.push(Box::new(ViewModelInstanceBoolean {
                view_model_property_id: required_u64_field(*view_model_property_id, "view_model_instance_boolean", "view_model_property_id")?,
                property_value: *value,
            }));
        }
        ObjectSpec::ViewModelInstanceEnum { view_model_property_id, value } => {
            objects.push(Box::new(ViewModelInstanceEnum {
                view_model_property_id: required_u64_field(*view_model_property_id, "view_model_instance_enum", "view_model_property_id")?,
                property_value: required_u64_field(*value, "view_model_instance_enum", "value")?,
            }));
        }
        ObjectSpec::ViewModelInstanceList => {
            objects.push(Box::new(ViewModelInstanceList));
        }
        ObjectSpec::ViewModelInstanceListItem { view_model_id, view_model_instance_id } => {
            objects.push(Box::new(ViewModelInstanceListItem {
                view_model_id: required_u64_field(*view_model_id, "view_model_instance_list_item", "view_model_id")?,
                view_model_instance_id: required_u64_field(*view_model_instance_id, "view_model_instance_list_item", "view_model_instance_id")?,
            }));
        }
        ObjectSpec::ViewModelInstanceViewModel { view_model_property_id, value } => {
            objects.push(Box::new(ViewModelInstanceViewModel {
                view_model_property_id: required_u64_field(*view_model_property_id, "view_model_instance_view_model", "view_model_property_id")?,
                property_value: required_u64_field(*value, "view_model_instance_view_model", "value")?,
            }));
        }
        ObjectSpec::TextModifierRange { units_value, type_value, mode_value, modify_from, modify_to, strength, clamp, falloff_from, falloff_to, offset, run_id } => {
            let mut r = TextModifierRange::new(parent_id);
            if let Some(v) = units_value { r.units_value = *v; }
            if let Some(v) = type_value { r.type_value = *v; }
            if let Some(v) = mode_value { r.mode_value = *v; }
            if let Some(v) = modify_from { r.modify_from = *v; }
            if let Some(v) = modify_to { r.modify_to = *v; }
            if let Some(v) = strength { r.strength = *v; }
            if let Some(v) = clamp { r.clamp = *v; }
            if let Some(v) = falloff_from { r.falloff_from = *v; }
            if let Some(v) = falloff_to { r.falloff_to = *v; }
            if let Some(v) = offset { r.offset = *v; }
            if let Some(v) = run_id { r.run_id = *v; }
            objects.push(Box::new(r));
        }
        ObjectSpec::TextModifierGroup { name, modifier_flags, origin_x, origin_y, opacity, x, y, rotation, scale_x, scale_y, children } => {
            let mut g = TextModifierGroup::new(name.clone(), parent_id);
            if let Some(v) = modifier_flags { g.modifier_flags = *v; }
            if let Some(v) = origin_x { g.origin_x = *v; }
            if let Some(v) = origin_y { g.origin_y = *v; }
            if let Some(v) = opacity { g.opacity = *v; }
            if let Some(v) = x { g.x = *v; }
            if let Some(v) = y { g.y = *v; }
            if let Some(v) = rotation { g.rotation = *v; }
            if let Some(v) = scale_x { g.scale_x = *v; }
            if let Some(v) = scale_y { g.scale_y = *v; }
            objects.push(Box::new(g));
            name_to_index.insert(name.clone(), object_index);
            let child_parent_id = object_index
                .checked_sub(artboard_start)
                .ok_or("internal error: parent index precedes artboard start".to_string())?
                as u64;
            if let Some(children) = children {
                for child in children {
                    append_text_modifier_group_child(child, child_parent_id, objects);
                }
            }
        }
        ObjectSpec::TextVariationModifier { axis_tag, axis_value } => {
            objects.push(Box::new(TextVariationModifier {
                parent_id,
                axis_tag: axis_tag.unwrap_or(0),
                axis_value: axis_value.unwrap_or(0.0),
            }));
        }
        ObjectSpec::TextStyleFeature { tag, feature_value } => {
            objects.push(Box::new(TextStyleFeature {
                parent_id,
                tag: tag.unwrap_or(0),
                feature_value: feature_value.unwrap_or(0),
            }));
        }
    }
    Ok(())
}

pub(crate) fn append_text_style_child(
    spec: &TextStyleChildSpec,
    parent_id: u64,
    objects: &mut Vec<Box<dyn RiveObject>>,
) {
    match spec {
        TextStyleChildSpec::TextStyleFeature { tag, feature_value } => {
            objects.push(Box::new(TextStyleFeature {
                parent_id,
                tag: tag.unwrap_or(0),
                feature_value: feature_value.unwrap_or(0),
            }));
        }
    }
}

pub(crate) fn append_text_modifier_group_child(
    spec: &TextModifierGroupChildSpec,
    parent_id: u64,
    objects: &mut Vec<Box<dyn RiveObject>>,
) {
    match spec {
        TextModifierGroupChildSpec::TextModifierRange {
            units_value, type_value, mode_value, modify_from, modify_to, strength, clamp, falloff_from, falloff_to, offset, run_id,
        } => {
            let mut range = TextModifierRange::new(parent_id);
            if let Some(v) = units_value { range.units_value = *v; }
            if let Some(v) = type_value { range.type_value = *v; }
            if let Some(v) = mode_value { range.mode_value = *v; }
            if let Some(v) = modify_from { range.modify_from = *v; }
            if let Some(v) = modify_to { range.modify_to = *v; }
            if let Some(v) = strength { range.strength = *v; }
            if let Some(v) = clamp { range.clamp = *v; }
            if let Some(v) = falloff_from { range.falloff_from = *v; }
            if let Some(v) = falloff_to { range.falloff_to = *v; }
            if let Some(v) = offset { range.offset = *v; }
            if let Some(v) = run_id { range.run_id = *v; }
            objects.push(Box::new(range));
        }
        TextModifierGroupChildSpec::TextVariationModifier { axis_tag, axis_value } => {
            objects.push(Box::new(TextVariationModifier {
                parent_id,
                axis_tag: axis_tag.unwrap_or(0),
                axis_value: axis_value.unwrap_or(0.0),
            }));
        }
    }
}
