use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};

use crate::objects::artboard::NestedArtboard;
use crate::objects::assets::{self, AudioAsset, FileAssetContents, FontAsset, ImageAsset};
use crate::objects::bones::{Bone, CubicWeight, RootBone, Skin, Tendon, Weight};
use crate::objects::constraints::{
    DistanceConstraint, FollowPathConstraint, IKConstraint, RotationConstraint, ScaleConstraint,
    TransformConstraint, TranslationConstraint,
};
use crate::objects::core::{RiveObject, type_keys};
use crate::objects::data_binding::{
    self, BindablePropertyArtboard, BindablePropertyBoolean, BindablePropertyColor,
    BindablePropertyEnum, BindablePropertyId, BindablePropertyInteger, BindablePropertyList,
    BindablePropertyNumber, BindablePropertyString, BindablePropertyTrigger, DataBind,
    DataBindPath, DataEnum, DataEnumCustom, DataEnumSystem, DataEnumValue, ViewModel,
    ViewModelInstance, ViewModelInstanceArtboard, ViewModelInstanceAssetImage,
    ViewModelInstanceBoolean, ViewModelInstanceColor, ViewModelInstanceEnum, ViewModelInstanceList,
    ViewModelInstanceListItem, ViewModelInstanceNumber, ViewModelInstanceString,
    ViewModelInstanceSymbol, ViewModelInstanceSymbolListIndex, ViewModelInstanceTrigger,
    ViewModelInstanceValue, ViewModelInstanceViewModel, ViewModelProperty,
};
use crate::objects::data_converters;
use crate::objects::layout::{self, LayoutComponent, LayoutComponentStyle};
use crate::objects::paint;
use crate::objects::scripting;
use crate::objects::shapes::{
    self, ClippingShape, CubicAsymmetricVertexObject, CubicDetachedVertexObject,
    CubicMirroredVertexObject, DrawRules, DrawTarget, Ellipse, Fill, GradientStop, Guide, Image,
    Joystick, LinearGradient, Node, PathObject, PointsPathObject, Polygon, RadialGradient,
    Rectangle, Shape, SolidColor, Solo, Star, StraightVertexObject, Stroke, Triangle, TrimPath,
};
use crate::objects::state_machine::{self, Event, NestedSimpleAnimation, NestedStateMachine};
use crate::objects::text::{
    self, Text, TextFollowPathModifier, TextInput, TextInputCursor, TextInputDrawable,
    TextInputSelectedText, TextInputSelection, TextModifierGroup, TextModifierRange, TextStyle,
    TextStyleAxis, TextStyleFeature, TextTargetModifier, TextValueRun, TextVariationModifier,
};

use super::parsers::{
    parse_color, parse_fill_rule, parse_stroke_cap, parse_stroke_join, parse_trim_mode,
    required_u64_field,
};
use super::references::{self, Namespace};
use super::spec::{ObjectSpec, TextModifierGroupChildSpec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FileAssetKind {
    Image,
    Font,
    Audio,
}

impl FileAssetKind {
    fn label(self) -> &'static str {
        match self {
            FileAssetKind::Image => "image_asset",
            FileAssetKind::Font => "font_asset",
            FileAssetKind::Audio => "audio_asset",
        }
    }
}

pub(crate) struct SceneContext<'a> {
    pub asset_ids: &'a HashMap<String, (u64, FileAssetKind)>,
    pub asset_kinds: &'a [FileAssetKind],
}

fn resolve_asset_ordinal(
    owner: &str,
    asset_name: Option<&str>,
    explicit: Option<u64>,
    expected: FileAssetKind,
    ctx: &SceneContext<'_>,
    fields: (&str, &str),
) -> Result<Option<u64>, String> {
    let lookup = |name: &str| ctx.asset_ids.get(name).map(|(ordinal, _)| *ordinal);
    let check = |ordinal: u64| {
        let subject = match asset_name {
            Some(name) => format!("asset '{name}'"),
            None => format!("asset index {ordinal}"),
        };
        match ctx.asset_kinds.get(ordinal as usize) {
            Some(kind) if *kind != expected => Err(format!(
                "references {subject}, which is a {}; it must be a {}",
                kind.label(),
                expected.label()
            )),
            Some(_) => Ok(()),
            None => Err(format!(
                "references {subject}, but the scene declares {} asset(s)",
                ctx.asset_kinds.len()
            )),
        }
    };
    references::resolve(
        owner,
        &Namespace {
            kind: "asset",
            name_field: fields.0,
            index_field: fields.1,
            lookup: &lookup,
            check: Some(&check),
        },
        asset_name,
        explicit,
    )
}

pub(crate) fn file_asset(spec: &ObjectSpec) -> Option<(&str, FileAssetKind)> {
    match spec {
        ObjectSpec::ImageAsset { name, .. } => Some((name, FileAssetKind::Image)),
        ObjectSpec::FontAsset { name, .. } => Some((name, FileAssetKind::Font)),
        ObjectSpec::AudioAsset { name, .. } => Some((name, FileAssetKind::Audio)),
        _ => None,
    }
}

pub(crate) fn is_file_asset(spec: &ObjectSpec) -> bool {
    file_asset(spec).is_some()
}

pub(crate) fn append_file_asset(
    spec: &ObjectSpec,
    objects: &mut Vec<Box<dyn RiveObject>>,
    base_dir: Option<&Path>,
) -> Result<(), String> {
    match spec {
        ObjectSpec::ImageAsset {
            name,
            asset_id,
            cdn_base_url,
            source,
        } => {
            let mut asset = ImageAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
            append_asset_contents(name, source.as_deref(), base_dir, objects)
        }
        ObjectSpec::FontAsset {
            name,
            asset_id,
            cdn_base_url,
            source,
        } => {
            let mut asset = FontAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
            append_asset_contents(name, source.as_deref(), base_dir, objects)
        }
        ObjectSpec::AudioAsset {
            name,
            asset_id,
            cdn_base_url,
        } => {
            let mut asset = AudioAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
            Ok(())
        }
        _ => Ok(()),
    }
}

const PROJECT_MARKERS: [&str; 3] = ["Cargo.toml", ".git", "package.json"];

fn asset_root(base_dir: &Path) -> PathBuf {
    let start = std::path::absolute(base_dir).unwrap_or_else(|_| base_dir.to_path_buf());
    let mut cursor = normalise(&start);
    loop {
        if PROJECT_MARKERS
            .iter()
            .any(|marker| cursor.join(marker).exists())
        {
            return cursor;
        }
        if !cursor.pop() {
            return normalise(&start);
        }
    }
}

fn normalise(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other),
        }
    }
    out
}

fn append_asset_contents(
    asset_name: &str,
    source: Option<&str>,
    base_dir: Option<&Path>,
    objects: &mut Vec<Box<dyn RiveObject>>,
) -> Result<(), String> {
    let Some(source) = source else {
        return Ok(());
    };
    let Some(base_dir) = base_dir else {
        return Err(format!(
            "asset '{asset_name}' sets 'source', but embedding asset files is only supported when generating from a scene file on disk"
        ));
    };
    let relative = Path::new(source);
    if relative.is_absolute() {
        return Err(format!(
            "asset '{asset_name}' source '{source}' must be relative to the scene file's directory so the scene stays portable"
        ));
    }
    let path = base_dir.join(relative);
    let resolved = std::path::absolute(&path).unwrap_or_else(|_| path.clone());
    let root = asset_root(base_dir);
    let canonical_path = path.canonicalize().map_err(|error| {
        format!(
            "asset '{}' source '{}' could not be read from {}: {}",
            asset_name,
            source,
            resolved.display(),
            error
        )
    })?;
    let canonical_root = root.canonicalize().unwrap_or_else(|_| normalise(&root));
    if !canonical_path.starts_with(&canonical_root) {
        return Err(format!(
            "asset '{}' source '{}' resolves to {}, outside the project rooted at {}",
            asset_name,
            source,
            canonical_path.display(),
            canonical_root.display()
        ));
    }
    let bytes = std::fs::read(&canonical_path).map_err(|error| {
        format!(
            "asset '{}' source '{}' could not be read from {}: {}",
            asset_name,
            source,
            resolved.display(),
            error
        )
    })?;
    if bytes.is_empty() {
        return Err(format!(
            "asset '{}' source '{}' is empty at {}",
            asset_name,
            source,
            resolved.display()
        ));
    }
    objects.push(Box::new(FileAssetContents::new(bytes)));
    Ok(())
}

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
    ctx: &SceneContext<'_>,
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
                        ctx,
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
                        ctx,
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
                        ctx,
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::SolidColor { name, color } => {
            let color_value = match color {
                Some(color) => parse_color(color)?,
                None => 0,
            };
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
                        ctx,
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
                        ctx,
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
        ObjectSpec::Node {
            name,
            x,
            y,
            children,
        } => {
            objects.push(Box::new(Node {
                name: name.clone(),
                parent_id,
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::Image {
            name,
            asset_id,
            asset,
            x,
            y,
            children,
        } => {
            let resolved_asset_id = resolve_asset_ordinal(
                name,
                asset.as_deref(),
                *asset_id,
                FileAssetKind::Image,
                ctx,
                ("asset", "asset_id"),
            )?
            .unwrap_or(0);
            let mut image = Image::new(name.clone(), parent_id, resolved_asset_id);
            if let Some(v) = x {
                image.x = *v;
            }
            if let Some(v) = y {
                image.y = *v;
            }
            objects.push(Box::new(image));
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
                        ctx,
                    )?;
                }
            }
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
                        ctx,
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
            children,
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
                        ctx,
                    )?;
                }
            }
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
                        ctx,
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
                        ctx,
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
                        ctx,
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
            if let Some(xx) = xx {
                skin.xx = *xx;
            }
            if let Some(yx) = yx {
                skin.yx = *yx;
            }
            if let Some(xy) = xy {
                skin.xy = *xy;
            }
            if let Some(yy) = yy {
                skin.yy = *yy;
            }
            if let Some(tx) = tx {
                skin.tx = *tx;
            }
            if let Some(ty) = ty {
                skin.ty = *ty;
            }
            objects.push(Box::new(skin));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::Tendon {
            name,
            bone,
            xx,
            yx,
            xy,
            yy,
            tx,
            ty,
        } => {
            let mut tendon = Tendon::new(name.clone(), parent_id);
            if let Some(bone_name) = bone {
                let bone_global = *name_to_index.get(bone_name).ok_or_else(|| {
                    format!("tendon '{}' references unknown bone '{}'", name, bone_name)
                })?;
                tendon.bone_id = bone_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "tendon '{}' bone '{}' precedes current artboard",
                        name, bone_name
                    )
                })? as u64;
            }
            if let Some(xx) = xx {
                tendon.xx = *xx;
            }
            if let Some(yx) = yx {
                tendon.yx = *yx;
            }
            if let Some(xy) = xy {
                tendon.xy = *xy;
            }
            if let Some(yy) = yy {
                tendon.yy = *yy;
            }
            if let Some(tx) = tx {
                tendon.tx = *tx;
            }
            if let Some(ty) = ty {
                tendon.ty = *ty;
            }
            objects.push(Box::new(tendon));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Weight {
            name,
            values,
            indices,
        } => {
            let mut weight = Weight::new(name.clone(), parent_id);
            if let Some(values) = values {
                weight.values = *values;
            }
            if let Some(indices) = indices {
                weight.indices = *indices;
            }
            objects.push(Box::new(weight));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CubicWeight {
            name,
            in_values,
            in_indices,
            out_values,
            out_indices,
        } => {
            let mut cubic_weight = CubicWeight::new(name.clone(), parent_id);
            if let Some(in_values) = in_values {
                cubic_weight.in_values = *in_values;
            }
            if let Some(in_indices) = in_indices {
                cubic_weight.in_indices = *in_indices;
            }
            if let Some(out_values) = out_values {
                cubic_weight.out_values = *out_values;
            }
            if let Some(out_indices) = out_indices {
                cubic_weight.out_indices = *out_indices;
            }
            objects.push(Box::new(cubic_weight));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::IkConstraint {
            name,
            target,
            strength,
            invert_direction,
            parent_bone_count,
        } => {
            let mut ik = IKConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "ik_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                ik.target_id = target_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "ik_constraint '{}' target '{}' precedes current artboard",
                        name, target_name
                    )
                })? as u64;
            }
            if let Some(s) = strength {
                ik.strength = *s;
            }
            if let Some(inv) = invert_direction {
                ik.invert_direction = *inv;
            }
            if let Some(pbc) = parent_bone_count {
                ik.parent_bone_count = *pbc;
            }
            objects.push(Box::new(ik));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DistanceConstraint {
            name,
            target,
            strength,
            distance,
            mode_value,
        } => {
            let mut dc = DistanceConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "distance_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                dc.target_id = target_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "distance_constraint '{}' target '{}' precedes current artboard",
                        name, target_name
                    )
                })? as u64;
            }
            if let Some(s) = strength {
                dc.strength = *s;
            }
            if let Some(d) = distance {
                dc.distance = *d;
            }
            if let Some(mv) = mode_value {
                dc.mode_value = *mv;
            }
            objects.push(Box::new(dc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TransformConstraint {
            name,
            target,
            strength,
            source_space_value,
            dest_space_value,
            origin_x,
            origin_y,
        } => {
            let mut tc = TransformConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "transform_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                tc.target_id = target_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "transform_constraint '{}' target '{}' precedes current artboard",
                        name, target_name
                    )
                })? as u64;
            }
            if let Some(s) = strength {
                tc.strength = *s;
            }
            if let Some(ssv) = source_space_value {
                tc.source_space_value = *ssv;
            }
            if let Some(dsv) = dest_space_value {
                tc.dest_space_value = *dsv;
            }
            if let Some(ox) = origin_x {
                tc.origin_x = *ox;
            }
            if let Some(oy) = origin_y {
                tc.origin_y = *oy;
            }
            objects.push(Box::new(tc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TranslationConstraint {
            name,
            target,
            strength,
            source_space_value,
            dest_space_value,
            copy_factor,
            min_value,
            max_value,
            offset,
            does_copy,
            min,
            max,
            min_max_space_value,
            copy_factor_y,
            min_value_y,
            max_value_y,
            does_copy_y,
            min_y,
            max_y,
        } => {
            let mut tlc = TranslationConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "translation_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                tlc.target_id = target_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "translation_constraint '{}' target '{}' precedes current artboard",
                        name, target_name
                    )
                })? as u64;
            }
            if let Some(s) = strength {
                tlc.strength = *s;
            }
            if let Some(v) = source_space_value {
                tlc.source_space_value = *v;
            }
            if let Some(v) = dest_space_value {
                tlc.dest_space_value = *v;
            }
            if let Some(v) = copy_factor {
                tlc.copy_factor = *v;
            }
            if let Some(v) = min_value {
                tlc.min_value = *v;
            }
            if let Some(v) = max_value {
                tlc.max_value = *v;
            }
            if let Some(v) = offset {
                tlc.offset = *v;
            }
            if let Some(v) = does_copy {
                tlc.does_copy = *v;
            }
            if let Some(v) = min {
                tlc.min = *v;
            }
            if let Some(v) = max {
                tlc.max = *v;
            }
            if let Some(v) = min_max_space_value {
                tlc.min_max_space_value = *v;
            }
            if let Some(v) = copy_factor_y {
                tlc.copy_factor_y = *v;
            }
            if let Some(v) = min_value_y {
                tlc.min_value_y = *v;
            }
            if let Some(v) = max_value_y {
                tlc.max_value_y = *v;
            }
            if let Some(v) = does_copy_y {
                tlc.does_copy_y = *v;
            }
            if let Some(v) = min_y {
                tlc.min_y = *v;
            }
            if let Some(v) = max_y {
                tlc.max_y = *v;
            }
            objects.push(Box::new(tlc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScaleConstraint {
            name,
            target,
            strength,
            source_space_value,
            dest_space_value,
            copy_factor,
            min_value,
            max_value,
            offset,
            does_copy,
            min,
            max,
            min_max_space_value,
            copy_factor_y,
            min_value_y,
            max_value_y,
            does_copy_y,
            min_y,
            max_y,
        } => {
            let mut sc = ScaleConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "scale_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                sc.target_id = target_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "scale_constraint '{}' target '{}' precedes current artboard",
                        name, target_name
                    )
                })? as u64;
            }
            if let Some(s) = strength {
                sc.strength = *s;
            }
            if let Some(v) = source_space_value {
                sc.source_space_value = *v;
            }
            if let Some(v) = dest_space_value {
                sc.dest_space_value = *v;
            }
            if let Some(v) = copy_factor {
                sc.copy_factor = *v;
            }
            if let Some(v) = min_value {
                sc.min_value = *v;
            }
            if let Some(v) = max_value {
                sc.max_value = *v;
            }
            if let Some(v) = offset {
                sc.offset = *v;
            }
            if let Some(v) = does_copy {
                sc.does_copy = *v;
            }
            if let Some(v) = min {
                sc.min = *v;
            }
            if let Some(v) = max {
                sc.max = *v;
            }
            if let Some(v) = min_max_space_value {
                sc.min_max_space_value = *v;
            }
            if let Some(v) = copy_factor_y {
                sc.copy_factor_y = *v;
            }
            if let Some(v) = min_value_y {
                sc.min_value_y = *v;
            }
            if let Some(v) = max_value_y {
                sc.max_value_y = *v;
            }
            if let Some(v) = does_copy_y {
                sc.does_copy_y = *v;
            }
            if let Some(v) = min_y {
                sc.min_y = *v;
            }
            if let Some(v) = max_y {
                sc.max_y = *v;
            }
            objects.push(Box::new(sc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::RotationConstraint {
            name,
            target,
            strength,
            source_space_value,
            dest_space_value,
            copy_factor,
            min_value,
            max_value,
            offset,
            does_copy,
            min,
            max,
            min_max_space_value,
        } => {
            let mut rc = RotationConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "rotation_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                rc.target_id = target_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "rotation_constraint '{}' target '{}' precedes current artboard",
                        name, target_name
                    )
                })? as u64;
            }
            if let Some(s) = strength {
                rc.strength = *s;
            }
            if let Some(v) = source_space_value {
                rc.source_space_value = *v;
            }
            if let Some(v) = dest_space_value {
                rc.dest_space_value = *v;
            }
            if let Some(v) = copy_factor {
                rc.copy_factor = *v;
            }
            if let Some(v) = min_value {
                rc.min_value = *v;
            }
            if let Some(v) = max_value {
                rc.max_value = *v;
            }
            if let Some(v) = offset {
                rc.offset = *v;
            }
            if let Some(v) = does_copy {
                rc.does_copy = *v;
            }
            if let Some(v) = min {
                rc.min = *v;
            }
            if let Some(v) = max {
                rc.max = *v;
            }
            if let Some(v) = min_max_space_value {
                rc.min_max_space_value = *v;
            }
            objects.push(Box::new(rc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::FollowPathConstraint {
            name,
            target,
            strength,
            source_space_value,
            dest_space_value,
            distance,
            orient,
            offset,
        } => {
            let mut fpc = FollowPathConstraint::new(name.clone(), parent_id);
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "follow_path_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                fpc.target_id = target_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "follow_path_constraint '{}' target '{}' precedes current artboard",
                        name, target_name
                    )
                })? as u64;
            }
            if let Some(s) = strength {
                fpc.strength = *s;
            }
            if let Some(v) = source_space_value {
                fpc.source_space_value = *v;
            }
            if let Some(v) = dest_space_value {
                fpc.dest_space_value = *v;
            }
            if let Some(d) = distance {
                fpc.distance = *d;
            }
            if let Some(o) = orient {
                fpc.orient = *o;
            }
            if let Some(o) = offset {
                fpc.offset = *o;
            }
            objects.push(Box::new(fpc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ClippingShape {
            name,
            source,
            fill_rule,
            is_visible,
        } => {
            let mut cs = ClippingShape::new(name.clone(), parent_id);
            if let Some(source_name) = source {
                let source_global = *name_to_index.get(source_name).ok_or_else(|| {
                    format!(
                        "clipping_shape '{}' references unknown source '{}'",
                        name, source_name
                    )
                })?;
                cs.source_id = source_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "clipping_shape '{}' source '{}' precedes current artboard",
                        name, source_name
                    )
                })? as u64;
            }
            if let Some(fr) = fill_rule {
                cs.fill_rule = parse_fill_rule(fr)?;
            }
            if let Some(v) = is_visible {
                cs.is_visible = *v;
            }
            objects.push(Box::new(cs));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DrawRules {
            name,
            draw_target,
            children,
        } => {
            objects.push(Box::new(DrawRules::new(name.clone(), parent_id)));
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
                        ctx,
                    )?;
                }
            }
            if let Some(target_name) = draw_target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "draw_rules '{}' references unknown draw_target '{}'",
                        name, target_name
                    )
                })?;
                let mut dr = DrawRules::new(name.clone(), parent_id);
                dr.draw_target_id = target_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "draw_rules '{}' draw_target '{}' precedes current artboard",
                        name, target_name
                    )
                })? as u64;
                objects[object_index] = Box::new(dr);
            }
        }
        ObjectSpec::DrawTarget {
            name,
            drawable,
            placement_value,
        } => {
            let mut dt = DrawTarget::new(name.clone(), parent_id);
            if let Some(drawable_name) = drawable {
                let drawable_global = *name_to_index.get(drawable_name).ok_or_else(|| {
                    format!(
                        "draw_target '{}' references unknown drawable '{}'",
                        name, drawable_name
                    )
                })?;
                dt.drawable_id = drawable_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "draw_target '{}' drawable '{}' precedes current artboard",
                        name, drawable_name
                    )
                })? as u64;
            }
            if let Some(pv) = placement_value {
                dt.placement_value = *pv;
            }
            objects.push(Box::new(dt));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Joystick {
            name,
            x,
            y,
            x_id,
            y_id,
            pos_x,
            pos_y,
            width,
            height,
            origin_x,
            origin_y,
            flags,
            handle_source_id,
        } => {
            let mut js = Joystick::new(name.clone(), parent_id);
            if let Some(v) = x {
                js.x = *v;
            }
            if let Some(v) = y {
                js.y = *v;
            }
            if let Some(v) = x_id {
                js.x_id = *v;
            }
            if let Some(v) = y_id {
                js.y_id = *v;
            }
            if let Some(v) = pos_x {
                js.pos_x = *v;
            }
            if let Some(v) = pos_y {
                js.pos_y = *v;
            }
            if let Some(v) = width {
                js.width = *v;
            }
            if let Some(v) = height {
                js.height = *v;
            }
            if let Some(v) = origin_x {
                js.origin_x = *v;
            }
            if let Some(v) = origin_y {
                js.origin_y = *v;
            }
            if let Some(v) = flags {
                js.flags = *v;
            }
            if let Some(v) = handle_source_id {
                js.handle_source_id = *v;
            }
            objects.push(Box::new(js));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Text {
            name,
            align_value,
            sizing_value,
            overflow_value,
            width,
            height,
            origin_x,
            origin_y,
            paragraph_spacing,
            origin_value,
            children,
        } => {
            let mut text = Text::new(name.clone(), parent_id);
            if let Some(v) = align_value {
                text.align_value = *v;
            }
            if let Some(v) = sizing_value {
                text.sizing_value = *v;
            }
            if let Some(v) = overflow_value {
                text.overflow_value = *v;
            }
            if let Some(v) = width {
                text.width = *v;
            }
            if let Some(v) = height {
                text.height = *v;
            }
            if let Some(v) = origin_x {
                text.origin_x = *v;
            }
            if let Some(v) = origin_y {
                text.origin_y = *v;
            }
            if let Some(v) = paragraph_spacing {
                text.paragraph_spacing = *v;
            }
            if let Some(v) = origin_value {
                text.origin_value = *v;
            }
            objects.push(Box::new(text));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::TextStyle {
            name,
            font_size,
            line_height,
            letter_spacing,
            font_asset_id,
            font_asset,
            children,
        } => {
            let mut style = TextStyle::new(name.clone(), parent_id);
            if let Some(v) = font_size {
                style.font_size = *v;
            }
            if let Some(v) = line_height {
                style.line_height = *v;
            }
            if let Some(v) = letter_spacing {
                style.letter_spacing = *v;
            }
            if let Some(v) = resolve_asset_ordinal(
                name,
                font_asset.as_deref(),
                *font_asset_id,
                FileAssetKind::Font,
                ctx,
                ("font_asset", "font_asset_id"),
            )? {
                style.font_asset_id = v;
            }
            objects.push(Box::new(style));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::TextValueRun {
            name,
            text,
            style_id,
            style,
        } => {
            let mut run = TextValueRun::new(name.clone(), parent_id, text.clone());
            let lookup = |style_name: &str| {
                name_to_index
                    .get(style_name)
                    .and_then(|index| index.checked_sub(artboard_start))
                    .map(|local| local as u64)
            };
            let check = |local: u64| {
                let subject = match style.as_deref() {
                    Some(style_name) => format!("'{style_name}'"),
                    None => format!("artboard object {local}"),
                };
                match objects
                    .get(artboard_start + local as usize)
                    .map(|object| object.type_key())
                {
                    Some(type_keys::TEXT_STYLE_PAINT) => Ok(()),
                    Some(_) => Err(format!("references {subject}, which is not a text_style")),
                    None => Err(format!(
                        "references {subject}, which is not defined before it"
                    )),
                }
            };
            if let Some(resolved) = references::resolve(
                name,
                &Namespace {
                    kind: "text style",
                    name_field: "style",
                    index_field: "style_id",
                    lookup: &lookup,
                    check: Some(&check),
                },
                style.as_deref(),
                *style_id,
            )? {
                run.style_id = resolved;
            }
            objects.push(Box::new(run));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ImageAsset { name, .. }
        | ObjectSpec::FontAsset { name, .. }
        | ObjectSpec::AudioAsset { name, .. } => {
            return Err(format!(
                "asset '{name}' must be a direct child of an artboard; Rive stores assets at file scope, not inside the object tree"
            ));
        }
        ObjectSpec::LayoutComponent {
            name,
            clip,
            width,
            height,
            style_id,
            fractional_width,
            fractional_height,
            children,
        } => {
            let mut lc = LayoutComponent::new(name.clone(), parent_id);
            if let Some(v) = clip {
                lc.clip = *v;
            }
            if let Some(v) = width {
                lc.width = *v;
            }
            if let Some(v) = height {
                lc.height = *v;
            }
            if let Some(v) = style_id {
                lc.style_id = *v;
            }
            if let Some(v) = fractional_width {
                lc.fractional_width = *v;
            }
            if let Some(v) = fractional_height {
                lc.fractional_height = *v;
            }
            objects.push(Box::new(lc));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::LayoutComponentStyle {
            name,
            gap_horizontal,
            gap_vertical,
            max_width,
            max_height,
            min_width,
            min_height,
            border_left,
            border_right,
            border_top,
            border_bottom,
            margin_left,
            margin_right,
            margin_top,
            margin_bottom,
            padding_left,
            padding_right,
            padding_top,
            padding_bottom,
            position_left,
            position_right,
            position_top,
            position_bottom,
            flex_direction,
            flex_wrap,
            align_items,
            align_content,
            justify_content,
            display,
            position_type,
            overflow,
            intrinsically_sized,
            width_units,
            height_units,
            flex_grow,
            flex_shrink,
            flex_basis,
            aspect_ratio,
        } => {
            let mut style = LayoutComponentStyle::new(name.clone(), parent_id);
            if let Some(v) = gap_horizontal {
                style.gap_horizontal = *v;
            }
            if let Some(v) = gap_vertical {
                style.gap_vertical = *v;
            }
            if let Some(v) = max_width {
                style.max_width = *v;
            }
            if let Some(v) = max_height {
                style.max_height = *v;
            }
            if let Some(v) = min_width {
                style.min_width = *v;
            }
            if let Some(v) = min_height {
                style.min_height = *v;
            }
            if let Some(v) = border_left {
                style.border_left = *v;
            }
            if let Some(v) = border_right {
                style.border_right = *v;
            }
            if let Some(v) = border_top {
                style.border_top = *v;
            }
            if let Some(v) = border_bottom {
                style.border_bottom = *v;
            }
            if let Some(v) = margin_left {
                style.margin_left = *v;
            }
            if let Some(v) = margin_right {
                style.margin_right = *v;
            }
            if let Some(v) = margin_top {
                style.margin_top = *v;
            }
            if let Some(v) = margin_bottom {
                style.margin_bottom = *v;
            }
            if let Some(v) = padding_left {
                style.padding_left = *v;
            }
            if let Some(v) = padding_right {
                style.padding_right = *v;
            }
            if let Some(v) = padding_top {
                style.padding_top = *v;
            }
            if let Some(v) = padding_bottom {
                style.padding_bottom = *v;
            }
            if let Some(v) = position_left {
                style.position_left = *v;
            }
            if let Some(v) = position_right {
                style.position_right = *v;
            }
            if let Some(v) = position_top {
                style.position_top = *v;
            }
            if let Some(v) = position_bottom {
                style.position_bottom = *v;
            }
            if let Some(v) = flex_direction {
                style.flex_direction = *v;
            }
            if let Some(v) = flex_wrap {
                style.flex_wrap = *v;
            }
            if let Some(v) = align_items {
                style.align_items = *v;
            }
            if let Some(v) = align_content {
                style.align_content = *v;
            }
            if let Some(v) = justify_content {
                style.justify_content = *v;
            }
            if let Some(v) = display {
                style.display = *v;
            }
            if let Some(v) = position_type {
                style.position_type = *v;
            }
            if let Some(v) = overflow {
                style.overflow = *v;
            }
            if let Some(v) = intrinsically_sized {
                style.intrinsically_sized = *v;
            }
            if let Some(v) = width_units {
                style.width_units = *v;
            }
            if let Some(v) = height_units {
                style.height_units = *v;
            }
            if let Some(v) = flex_grow {
                style.flex_grow = *v;
            }
            if let Some(v) = flex_shrink {
                style.flex_shrink = *v;
            }
            if let Some(v) = flex_basis {
                style.flex_basis = *v;
            }
            if let Some(v) = aspect_ratio {
                style.aspect_ratio = *v;
            }
            objects.push(Box::new(style));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ViewModel { name, children } => {
            let vm = ViewModel::new(name.clone(), parent_id);
            objects.push(Box::new(vm));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelProperty {
            name,
            property_type_value,
        } => {
            let vmp =
                ViewModelProperty::new(name.clone(), parent_id, property_type_value.unwrap_or(0));
            objects.push(Box::new(vmp));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataBind {
            property_key,
            flags,
            converter_id,
        } => {
            let mut db = DataBind::new(*property_key, *flags);
            if let Some(v) = converter_id {
                db.converter_id = *v;
            }
            objects.push(Box::new(db));
        }
        ObjectSpec::ViewModelInstance { view_model_id } => {
            objects.push(Box::new(ViewModelInstance {
                view_model_id: required_u64_field(
                    *view_model_id,
                    "view_model_instance",
                    "view_model_id",
                )?,
            }));
        }
        ObjectSpec::ViewModelInstanceValue {
            view_model_property_id,
        } => {
            objects.push(Box::new(ViewModelInstanceValue {
                view_model_property_id: required_u64_field(
                    *view_model_property_id,
                    "view_model_instance_value",
                    "view_model_property_id",
                )?,
            }));
        }
        ObjectSpec::ViewModelInstanceColor {
            view_model_property_id,
            value,
        } => {
            let color = parse_color(value)?;
            objects.push(Box::new(ViewModelInstanceColor {
                view_model_property_id: required_u64_field(
                    *view_model_property_id,
                    "view_model_instance_color",
                    "view_model_property_id",
                )?,
                property_value: color,
            }));
        }
        ObjectSpec::ViewModelInstanceString {
            view_model_property_id,
            value,
        } => {
            objects.push(Box::new(ViewModelInstanceString {
                view_model_property_id: required_u64_field(
                    *view_model_property_id,
                    "view_model_instance_string",
                    "view_model_property_id",
                )?,
                property_value: value.clone(),
            }));
        }
        ObjectSpec::ViewModelInstanceNumber {
            view_model_property_id,
            value,
        } => {
            objects.push(Box::new(ViewModelInstanceNumber {
                view_model_property_id: required_u64_field(
                    *view_model_property_id,
                    "view_model_instance_number",
                    "view_model_property_id",
                )?,
                property_value: *value,
            }));
        }
        ObjectSpec::ViewModelInstanceBoolean {
            view_model_property_id,
            value,
        } => {
            objects.push(Box::new(ViewModelInstanceBoolean {
                view_model_property_id: required_u64_field(
                    *view_model_property_id,
                    "view_model_instance_boolean",
                    "view_model_property_id",
                )?,
                property_value: *value,
            }));
        }
        ObjectSpec::ViewModelInstanceEnum {
            view_model_property_id,
            value,
        } => {
            objects.push(Box::new(ViewModelInstanceEnum {
                view_model_property_id: required_u64_field(
                    *view_model_property_id,
                    "view_model_instance_enum",
                    "view_model_property_id",
                )?,
                property_value: required_u64_field(*value, "view_model_instance_enum", "value")?,
            }));
        }
        ObjectSpec::ViewModelInstanceList => {
            objects.push(Box::new(ViewModelInstanceList));
        }
        ObjectSpec::ViewModelInstanceListItem {
            view_model_id,
            view_model_instance_id,
        } => {
            objects.push(Box::new(ViewModelInstanceListItem {
                view_model_id: required_u64_field(
                    *view_model_id,
                    "view_model_instance_list_item",
                    "view_model_id",
                )?,
                view_model_instance_id: required_u64_field(
                    *view_model_instance_id,
                    "view_model_instance_list_item",
                    "view_model_instance_id",
                )?,
            }));
        }
        ObjectSpec::ViewModelInstanceViewModel {
            view_model_property_id,
            value,
        } => {
            objects.push(Box::new(ViewModelInstanceViewModel {
                view_model_property_id: required_u64_field(
                    *view_model_property_id,
                    "view_model_instance_view_model",
                    "view_model_property_id",
                )?,
                property_value: required_u64_field(
                    *value,
                    "view_model_instance_view_model",
                    "value",
                )?,
            }));
        }
        ObjectSpec::TextModifierRange {
            units_value,
            type_value,
            mode_value,
            modify_from,
            modify_to,
            strength,
            clamp,
            falloff_from,
            falloff_to,
            offset,
            run_id,
        } => {
            let mut r = TextModifierRange::new(parent_id);
            if let Some(v) = units_value {
                r.units_value = *v;
            }
            if let Some(v) = type_value {
                r.type_value = *v;
            }
            if let Some(v) = mode_value {
                r.mode_value = *v;
            }
            if let Some(v) = modify_from {
                r.modify_from = *v;
            }
            if let Some(v) = modify_to {
                r.modify_to = *v;
            }
            if let Some(v) = strength {
                r.strength = *v;
            }
            if let Some(v) = clamp {
                r.clamp = *v;
            }
            if let Some(v) = falloff_from {
                r.falloff_from = *v;
            }
            if let Some(v) = falloff_to {
                r.falloff_to = *v;
            }
            if let Some(v) = offset {
                r.offset = *v;
            }
            if let Some(v) = run_id {
                r.run_id = *v;
            }
            objects.push(Box::new(r));
        }
        ObjectSpec::TextModifierGroup {
            name,
            modifier_flags,
            origin_x,
            origin_y,
            opacity,
            x,
            y,
            rotation,
            scale_x,
            scale_y,
            children,
        } => {
            let mut g = TextModifierGroup::new(name.clone(), parent_id);
            if let Some(v) = modifier_flags {
                g.modifier_flags = *v;
            }
            if let Some(v) = origin_x {
                g.origin_x = *v;
            }
            if let Some(v) = origin_y {
                g.origin_y = *v;
            }
            if let Some(v) = opacity {
                g.opacity = *v;
            }
            if let Some(v) = x {
                g.x = *v;
            }
            if let Some(v) = y {
                g.y = *v;
            }
            if let Some(v) = rotation {
                g.rotation = *v;
            }
            if let Some(v) = scale_x {
                g.scale_x = *v;
            }
            if let Some(v) = scale_y {
                g.scale_y = *v;
            }
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
        ObjectSpec::TextVariationModifier {
            axis_tag,
            axis_value,
        } => {
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
        ObjectSpec::Folder {
            name,
            parent_id: pid,
        } => {
            let folder = assets::Folder::new(name.clone(), pid.unwrap_or(parent_id));
            objects.push(Box::new(folder));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::LayeredAsset { name } => {
            objects.push(Box::new(assets::LayeredAsset::new(name.clone())));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::LayerImageAsset {
            name,
            asset_id,
            cdn_base_url,
        } => {
            let mut asset = assets::LayerImageAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::SVGAsset {
            name,
            asset_id,
            cdn_base_url,
        } => {
            let mut asset = assets::SVGAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::LottieAsset {
            name,
            asset_id,
            cdn_base_url,
        } => {
            let mut asset = assets::LottieAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ExportAudio { name, volume } => {
            let mut ea = assets::ExportAudio::new(name.clone(), parent_id);
            if let Some(v) = volume {
                ea.volume = *v;
            }
            objects.push(Box::new(ea));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScriptAsset {
            name,
            asset_id,
            cdn_base_url,
            is_module,
        } => {
            let mut asset = assets::ScriptAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            if let Some(v) = is_module {
                asset.is_module = *v;
            }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::BlobAsset {
            name,
            asset_id,
            cdn_base_url,
        } => {
            let mut asset = assets::BlobAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DashPath {
            name,
            offset,
            offset_is_percentage,
            children,
        } => {
            let mut dp = paint::DashPath::new(name.clone(), parent_id);
            if let Some(v) = offset {
                dp.offset = *v;
            }
            if let Some(v) = offset_is_percentage {
                dp.offset_is_percentage = *v;
            }
            objects.push(Box::new(dp));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::Dash {
            name,
            length,
            length_is_percentage,
        } => {
            let mut d = paint::Dash::new(name.clone(), parent_id);
            if let Some(v) = length {
                d.length = *v;
            }
            if let Some(v) = length_is_percentage {
                d.length_is_percentage = *v;
            }
            objects.push(Box::new(d));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::Feather {
            name,
            strength,
            offset_x,
            offset_y,
            space_value,
            inner,
        } => {
            let mut f = paint::Feather::new(name.clone(), parent_id);
            if let Some(v) = strength {
                f.strength = *v;
            }
            if let Some(v) = offset_x {
                f.offset_x = *v;
            }
            if let Some(v) = offset_y {
                f.offset_y = *v;
            }
            if let Some(v) = space_value {
                f.space_value = *v;
            }
            if let Some(v) = inner {
                f.inner = *v;
            }
            objects.push(Box::new(f));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::OpenUrlEvent {
            name,
            url,
            target_value,
            children,
        } => {
            let mut evt = state_machine::OpenUrlEvent::new(
                name.clone(),
                parent_id,
                url.clone().unwrap_or_default(),
            );
            if let Some(v) = target_value {
                evt.target_value = *v;
            }
            objects.push(Box::new(evt));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::AudioEvent {
            name,
            asset_id,
            children,
        } => {
            let mut evt = state_machine::AudioEvent::new(name.clone(), parent_id);
            if let Some(v) = asset_id {
                evt.asset_id = *v;
            }
            objects.push(Box::new(evt));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::CustomPropertyNumber {
            name,
            property_value,
        } => {
            let mut cp = state_machine::CustomPropertyNumber::new(name.clone(), parent_id);
            if let Some(v) = property_value {
                cp.property_value = *v;
            }
            objects.push(Box::new(cp));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CustomPropertyBoolean {
            name,
            property_value,
        } => {
            let mut cp = state_machine::CustomPropertyBoolean::new(name.clone(), parent_id);
            if let Some(v) = property_value {
                cp.property_value = if v.is_boolean() {
                    if v.as_bool().unwrap_or(false) { 1 } else { 0 }
                } else {
                    v.as_u64().unwrap_or(0)
                };
            }
            objects.push(Box::new(cp));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CustomPropertyString {
            name,
            property_value,
        } => {
            let mut cp = state_machine::CustomPropertyString::new(name.clone(), parent_id);
            if let Some(v) = property_value {
                cp.property_value = v.clone();
            }
            objects.push(Box::new(cp));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CustomPropertyColor {
            name,
            property_value,
        } => {
            let mut cp = state_machine::CustomPropertyColor::new(name.clone(), parent_id);
            if let Some(v) = property_value {
                cp.property_value = parse_color(v)?;
            }
            objects.push(Box::new(cp));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CustomPropertyTrigger { name } => {
            let cp = state_machine::CustomPropertyTrigger::new(name.clone(), parent_id);
            objects.push(Box::new(cp));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CustomPropertyEnum {
            name,
            property_value,
            enum_id,
        } => {
            let mut cp = state_machine::CustomPropertyEnum::new(name.clone(), parent_id);
            if let Some(v) = property_value {
                cp.property_value = *v;
            }
            if let Some(v) = enum_id {
                cp.enum_id = *v;
            }
            objects.push(Box::new(cp));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::CustomPropertyGroup { name, children } => {
            let cp = state_machine::CustomPropertyGroup::new(name.clone(), parent_id);
            objects.push(Box::new(cp));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::TargetEffect { name, target_id } => {
            let mut te = shapes::TargetEffect::new(name.clone(), parent_id);
            if let Some(v) = target_id {
                te.target_id = *v;
            }
            objects.push(Box::new(te));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::GroupEffect { name, children } => {
            let ge = shapes::GroupEffect::new(name.clone(), parent_id);
            objects.push(Box::new(ge));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ListPath {
            name,
            is_closed,
            list_source,
            children,
        } => {
            let mut lp = shapes::ListPath::new(name.clone(), parent_id);
            if let Some(v) = is_closed {
                lp.is_closed = *v;
            }
            if let Some(v) = list_source {
                lp.list_source = *v;
            }
            objects.push(Box::new(lp));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::PointsCommonPath {
            name,
            is_closed,
            children,
        } => {
            let mut pcp = shapes::PointsCommonPath::new(name.clone(), parent_id);
            if let Some(v) = is_closed {
                pcp.is_closed = *v;
            }
            objects.push(Box::new(pcp));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::Guide { name } => {
            objects.push(Box::new(Guide::new(name.clone(), parent_id)));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ArtboardComponentList {
            name,
            list_source,
            children,
        } => {
            let mut acl = shapes::ArtboardComponentList::new(name.clone(), parent_id);
            if let Some(v) = list_source {
                acl.list_source = *v;
            }
            objects.push(Box::new(acl));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ArtboardComponentListOverride {
            name,
            artboard_id,
            instance_width,
            instance_height,
            instance_width_units_value,
            instance_height_units_value,
            instance_width_scale_type,
            instance_height_scale_type,
        } => {
            let mut ov = shapes::ArtboardComponentListOverride::new(name.clone(), parent_id);
            if let Some(v) = artboard_id {
                ov.artboard_id = *v;
            }
            if let Some(v) = instance_width {
                ov.instance_width = *v;
            }
            if let Some(v) = instance_height {
                ov.instance_height = *v;
            }
            if let Some(v) = instance_width_units_value {
                ov.instance_width_units_value = *v;
            }
            if let Some(v) = instance_height_units_value {
                ov.instance_height_units_value = *v;
            }
            if let Some(v) = instance_width_scale_type {
                ov.instance_width_scale_type = *v;
            }
            if let Some(v) = instance_height_scale_type {
                ov.instance_height_scale_type = *v;
            }
            objects.push(Box::new(ov));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ArtboardListMapRule {
            name,
            artboard_id,
            view_model_id,
        } => {
            let mut rule = shapes::ArtboardListMapRule::new(name.clone(), parent_id);
            if let Some(v) = artboard_id {
                rule.artboard_id = *v;
            }
            if let Some(v) = view_model_id {
                rule.view_model_id = *v;
            }
            objects.push(Box::new(rule));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ForegroundLayoutDrawable { name, children } => {
            let fld = layout::ForegroundLayoutDrawable::new(name.clone(), parent_id);
            objects.push(Box::new(fld));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ClampedScrollPhysics {
            friction,
            speed_multiplier,
        } => {
            let mut csp = layout::ClampedScrollPhysics::new();
            if let Some(v) = friction {
                csp.friction = *v;
            }
            if let Some(v) = speed_multiplier {
                csp.speed_multiplier = *v;
            }
            objects.push(Box::new(csp));
        }
        ObjectSpec::ElasticScrollPhysics {
            friction,
            speed_multiplier,
            elastic_factor,
        } => {
            let mut esp = layout::ElasticScrollPhysics::new();
            if let Some(v) = friction {
                esp.friction = *v;
            }
            if let Some(v) = speed_multiplier {
                esp.speed_multiplier = *v;
            }
            if let Some(v) = elastic_factor {
                esp.elastic_factor = *v;
            }
            objects.push(Box::new(esp));
        }
        ObjectSpec::Mesh { name, children } => {
            objects.push(Box::new(crate::objects::mesh::Mesh::new(
                name.clone(),
                parent_id,
            )));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::MeshVertex { name, x, y, u, v } => {
            let mut mv = crate::objects::mesh::MeshVertex::new(name.clone(), parent_id);
            if let Some(val) = x {
                mv.x = *val;
            }
            if let Some(val) = y {
                mv.y = *val;
            }
            if let Some(val) = u {
                mv.u = *val;
            }
            if let Some(val) = v {
                mv.v = *val;
            }
            objects.push(Box::new(mv));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ContourMeshVertex { name, x, y, u, v } => {
            let mut cv = crate::objects::mesh::ContourMeshVertex::new(name.clone(), parent_id);
            if let Some(val) = x {
                cv.x = *val;
            }
            if let Some(val) = y {
                cv.y = *val;
            }
            if let Some(val) = u {
                cv.u = *val;
            }
            if let Some(val) = v {
                cv.v = *val;
            }
            objects.push(Box::new(cv));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ForcedEdge {
            name,
            from_vertex,
            to_vertex,
        } => {
            let mut fe = crate::objects::mesh::ForcedEdge::new(name.clone(), parent_id);
            if let Some(from_name) = from_vertex {
                let from_global = *name_to_index.get(from_name).ok_or_else(|| {
                    format!(
                        "forced_edge '{}' references unknown from_vertex '{}'",
                        name, from_name
                    )
                })?;
                fe.from_id = from_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "forced_edge '{}' from_vertex '{}' precedes current artboard",
                        name, from_name
                    )
                })? as u64;
            }
            if let Some(to_name) = to_vertex {
                let to_global = *name_to_index.get(to_name).ok_or_else(|| {
                    format!(
                        "forced_edge '{}' references unknown to_vertex '{}'",
                        name, to_name
                    )
                })?;
                fe.to_id = to_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "forced_edge '{}' to_vertex '{}' precedes current artboard",
                        name, to_name
                    )
                })? as u64;
            }
            objects.push(Box::new(fe));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NestedLinearAnimation {
            name,
            animation,
            mix,
        } => {
            let animation_id = *animation_name_to_index.get(animation).ok_or_else(|| {
                format!(
                    "nested_linear_animation '{}' references unknown animation '{}'",
                    name, animation
                )
            })? as u64;
            objects.push(Box::new(crate::objects::artboard::NestedLinearAnimation {
                name: name.clone(),
                parent_id,
                animation_id,
                mix: mix.unwrap_or(1.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NestedRemapAnimation {
            name,
            animation,
            time,
        } => {
            let animation_id = *animation_name_to_index.get(animation).ok_or_else(|| {
                format!(
                    "nested_remap_animation '{}' references unknown animation '{}'",
                    name, animation
                )
            })? as u64;
            objects.push(Box::new(crate::objects::artboard::NestedRemapAnimation {
                name: name.clone(),
                parent_id,
                animation_id,
                time: time.unwrap_or(0.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NestedTrigger {
            name,
            nested_input_id,
        } => {
            objects.push(Box::new(crate::objects::artboard::NestedTrigger {
                name: name.clone(),
                parent_id,
                nested_input_id: *nested_input_id,
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NestedBool {
            name,
            nested_input_id,
            value,
        } => {
            objects.push(Box::new(crate::objects::artboard::NestedBool {
                name: name.clone(),
                parent_id,
                nested_input_id: *nested_input_id,
                nested_value: value.unwrap_or(false),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NestedNumber {
            name,
            nested_input_id,
            value,
        } => {
            objects.push(Box::new(crate::objects::artboard::NestedNumber {
                name: name.clone(),
                parent_id,
                nested_input_id: *nested_input_id,
                nested_value: value.unwrap_or(0.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NestedArtboardLeaf {
            name,
            source_artboard,
            x,
            y,
            children,
        } => {
            if source_artboard == current_artboard_name {
                return Err(format!(
                    "nested artboard leaf '{}' cannot reference its own artboard '{}'",
                    name, source_artboard
                ));
            }
            let source_artboard_index =
                *artboard_name_to_index.get(source_artboard).ok_or_else(|| {
                    format!(
                        "nested artboard leaf '{}' references unknown artboard '{}'",
                        name, source_artboard
                    )
                })?;
            objects.push(Box::new(crate::objects::artboard::NestedArtboardLeaf {
                name: name.clone(),
                parent_id,
                artboard_id: source_artboard_index as u64,
                x: x.unwrap_or(0.0),
                y: y.unwrap_or(0.0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::NestedArtboardLayout {
            name,
            source_artboard,
            x,
            y,
            width,
            height,
            style_id,
            children,
        } => {
            if source_artboard == current_artboard_name {
                return Err(format!(
                    "nested artboard layout '{}' cannot reference its own artboard '{}'",
                    name, source_artboard
                ));
            }
            let source_artboard_index =
                *artboard_name_to_index.get(source_artboard).ok_or_else(|| {
                    format!(
                        "nested artboard layout '{}' references unknown artboard '{}'",
                        name, source_artboard
                    )
                })?;
            let mut nal = crate::objects::artboard::NestedArtboardLayout::new(
                name.clone(),
                parent_id,
                source_artboard_index as u64,
            );
            if let Some(v) = x {
                nal.x = *v;
            }
            if let Some(v) = y {
                nal.y = *v;
            }
            if let Some(v) = width {
                nal.width = *v;
            }
            if let Some(v) = height {
                nal.height = *v;
            }
            if let Some(v) = style_id {
                nal.style_id = *v;
            }
            objects.push(Box::new(nal));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::DraggableConstraint {
            name,
            strength,
            direction_value,
        } => {
            let mut dc =
                crate::objects::constraints::DraggableConstraint::new(name.clone(), parent_id);
            if let Some(v) = strength {
                dc.strength = *v;
            }
            if let Some(v) = direction_value {
                dc.direction_value = *v;
            }
            objects.push(Box::new(dc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScrollConstraint {
            name,
            strength,
            direction_value,
            snap,
            physics_id,
            scroll_offset_x,
            scroll_offset_y,
            scroll_percent_x,
            scroll_percent_y,
            scroll_index,
            children,
        } => {
            let mut sc =
                crate::objects::constraints::ScrollConstraint::new(name.clone(), parent_id);
            if let Some(v) = strength {
                sc.strength = *v;
            }
            if let Some(v) = direction_value {
                sc.direction_value = *v;
            }
            if let Some(v) = snap {
                sc.snap = *v;
            }
            if let Some(v) = physics_id {
                sc.physics_id = *v;
            }
            if let Some(v) = scroll_offset_x {
                sc.scroll_offset_x = *v;
            }
            if let Some(v) = scroll_offset_y {
                sc.scroll_offset_y = *v;
            }
            if let Some(v) = scroll_percent_x {
                sc.scroll_percent_x = *v;
            }
            if let Some(v) = scroll_percent_y {
                sc.scroll_percent_y = *v;
            }
            if let Some(v) = scroll_index {
                sc.scroll_index = *v;
            }
            objects.push(Box::new(sc));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ScrollBarConstraint {
            name,
            strength,
            scroll_constraint_id,
            auto_size,
        } => {
            let mut sbc =
                crate::objects::constraints::ScrollBarConstraint::new(name.clone(), parent_id);
            if let Some(v) = strength {
                sbc.strength = *v;
            }
            if let Some(v) = scroll_constraint_id {
                sbc.scroll_constraint_id = *v;
            }
            if let Some(v) = auto_size {
                sbc.auto_size = *v;
            }
            objects.push(Box::new(sbc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ListFollowPathConstraint {
            name,
            target,
            strength,
            orient,
            start,
            end,
            list_source,
            distance_end,
            distance_offset,
            random_mode_value,
        } => {
            let mut lfpc =
                crate::objects::constraints::ListFollowPathConstraint::new(name.clone(), parent_id);
            if let Some(v) = strength {
                lfpc.strength = *v;
            }
            if let Some(target_name) = target {
                let target_global = *name_to_index.get(target_name).ok_or_else(|| {
                    format!(
                        "list_follow_path_constraint '{}' references unknown target '{}'",
                        name, target_name
                    )
                })?;
                lfpc.target_id = target_global.checked_sub(artboard_start).ok_or_else(|| {
                    format!(
                        "list_follow_path_constraint '{}' target '{}' precedes current artboard",
                        name, target_name
                    )
                })? as u64;
            }
            if let Some(v) = orient {
                lfpc.orient = *v;
            }
            if let Some(v) = start {
                lfpc.start = *v;
            }
            if let Some(v) = end {
                lfpc.end = *v;
            }
            if let Some(v) = list_source {
                lfpc.list_source = *v;
            }
            if let Some(v) = distance_end {
                lfpc.distance_end = *v;
            }
            if let Some(v) = distance_offset {
                lfpc.distance_offset = *v;
            }
            if let Some(v) = random_mode_value {
                lfpc.random_mode_value = *v;
            }
            objects.push(Box::new(lfpc));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::NSlicerTileMode {
            name,
            patch_index,
            style,
        } => {
            let mut tm = crate::objects::nslicer::NSlicerTileMode::new(
                name.clone().unwrap_or_default(),
                parent_id,
                *patch_index,
            );
            if let Some(v) = style {
                tm.style = *v;
            }
            objects.push(Box::new(tm));
        }
        ObjectSpec::NSlicer { name, children } => {
            let ns = crate::objects::nslicer::NSlicer::new(name.clone(), parent_id);
            objects.push(Box::new(ns));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::AxisY {
            name,
            offset,
            normalized,
        } => {
            let mut ay = crate::objects::nslicer::AxisY::new(
                name.clone().unwrap_or_default(),
                parent_id,
                *offset,
            );
            if let Some(v) = normalized {
                ay.normalized = *v;
            }
            objects.push(Box::new(ay));
        }
        ObjectSpec::AxisX {
            name,
            offset,
            normalized,
        } => {
            let mut ax = crate::objects::nslicer::AxisX::new(
                name.clone().unwrap_or_default(),
                parent_id,
                *offset,
            );
            if let Some(v) = normalized {
                ax.normalized = *v;
            }
            objects.push(Box::new(ax));
        }
        ObjectSpec::NSlicedNode {
            name,
            x,
            y,
            initial_width,
            initial_height,
            width,
            height,
            children,
        } => {
            let mut node = crate::objects::nslicer::NSlicedNode::new(name.clone(), parent_id);
            if let Some(v) = x {
                node.x = *v;
            }
            if let Some(v) = y {
                node.y = *v;
            }
            if let Some(v) = initial_width {
                node.initial_width = *v;
            }
            if let Some(v) = initial_height {
                node.initial_height = *v;
            }
            if let Some(v) = width {
                node.width = *v;
            }
            if let Some(v) = height {
                node.height = *v;
            }
            objects.push(Box::new(node));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyNumber { name, children } => {
            objects.push(Box::new(data_binding::ViewModelPropertyNumber {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyBoolean { name, children } => {
            objects.push(Box::new(data_binding::ViewModelPropertyBoolean {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyString { name, children } => {
            objects.push(Box::new(data_binding::ViewModelPropertyString {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyColor { name, children } => {
            objects.push(Box::new(data_binding::ViewModelPropertyColor {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyList {
            name,
            view_model_reference_id,
            children,
        } => {
            objects.push(Box::new(data_binding::ViewModelPropertyList {
                name: name.clone(),
                parent_id,
                view_model_reference_id: view_model_reference_id.unwrap_or(0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyViewModel {
            name,
            view_model_reference_id,
            children,
        } => {
            objects.push(Box::new(data_binding::ViewModelPropertyViewModel {
                name: name.clone(),
                parent_id,
                view_model_reference_id: view_model_reference_id.unwrap_or(0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyEnum {
            name,
            enum_id,
            children,
        } => {
            objects.push(Box::new(data_binding::ViewModelPropertyEnum {
                name: name.clone(),
                parent_id,
                enum_id: enum_id.unwrap_or(0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyEnumCustom {
            name,
            enum_id,
            children,
        } => {
            objects.push(Box::new(data_binding::ViewModelPropertyEnumCustom {
                name: name.clone(),
                parent_id,
                enum_id: enum_id.unwrap_or(0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyEnumSystem {
            name,
            enum_id,
            children,
        } => {
            objects.push(Box::new(data_binding::ViewModelPropertyEnumSystem {
                name: name.clone(),
                parent_id,
                enum_id: enum_id.unwrap_or(0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyTrigger { name, children } => {
            objects.push(Box::new(data_binding::ViewModelPropertyTrigger {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyAssetImage { name, children } => {
            objects.push(Box::new(data_binding::ViewModelPropertyAssetImage {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertyArtboard {
            name,
            artboard_id,
            children,
        } => {
            objects.push(Box::new(data_binding::ViewModelPropertyArtboard {
                name: name.clone(),
                parent_id,
                artboard_id: artboard_id.unwrap_or(0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertySymbol {
            name,
            symbol_type_value,
            artboard_id,
            children,
        } => {
            objects.push(Box::new(data_binding::ViewModelPropertySymbol {
                name: name.clone(),
                parent_id,
                symbol_type_value: symbol_type_value.unwrap_or(0),
                artboard_id: artboard_id.unwrap_or(0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelPropertySymbolListIndex {
            name,
            symbol_type_value,
            artboard_id,
            list_source,
            children,
        } => {
            objects.push(Box::new(data_binding::ViewModelPropertySymbolListIndex {
                name: name.clone(),
                parent_id,
                symbol_type_value: symbol_type_value.unwrap_or(0),
                artboard_id: artboard_id.unwrap_or(0),
                list_source: list_source.unwrap_or(0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ViewModelInstanceTrigger {
            view_model_property_id,
            value,
        } => {
            objects.push(Box::new(ViewModelInstanceTrigger {
                view_model_property_id: view_model_property_id.unwrap_or(0),
                property_value: value.unwrap_or(0),
            }));
        }
        ObjectSpec::ViewModelInstanceSymbol {
            view_model_property_id,
            value,
        } => {
            objects.push(Box::new(ViewModelInstanceSymbol {
                view_model_property_id: view_model_property_id.unwrap_or(0),
                property_value: value.unwrap_or(0),
            }));
        }
        ObjectSpec::ViewModelInstanceSymbolListIndex {
            view_model_property_id,
            value,
        } => {
            objects.push(Box::new(ViewModelInstanceSymbolListIndex {
                view_model_property_id: view_model_property_id.unwrap_or(0),
                property_value: value.unwrap_or(0),
            }));
        }
        ObjectSpec::ViewModelInstanceAssetImage {
            view_model_property_id,
            value,
        } => {
            objects.push(Box::new(ViewModelInstanceAssetImage {
                view_model_property_id: view_model_property_id.unwrap_or(0),
                property_value: value.unwrap_or(0),
            }));
        }
        ObjectSpec::ViewModelInstanceArtboard {
            view_model_property_id,
            value,
            artboard_id,
        } => {
            objects.push(Box::new(ViewModelInstanceArtboard {
                view_model_property_id: view_model_property_id.unwrap_or(0),
                property_value: value.unwrap_or(0),
                artboard_id: artboard_id.unwrap_or(0),
            }));
        }
        ObjectSpec::DataEnum { name, children } => {
            objects.push(Box::new(DataEnum {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::DataEnumCustom { name, children } => {
            objects.push(Box::new(DataEnumCustom {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::DataEnumValue { key, value } => {
            objects.push(Box::new(DataEnumValue {
                key: key.clone(),
                value: value.clone(),
            }));
        }
        ObjectSpec::DataEnumSystem { name, enum_type } => {
            objects.push(Box::new(DataEnumSystem {
                name: name.clone(),
                parent_id,
                enum_type: enum_type.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::BindablePropertyString { value } => {
            objects.push(Box::new(BindablePropertyString {
                property_value: value.clone().unwrap_or_default(),
            }));
        }
        ObjectSpec::BindablePropertyBoolean { value } => {
            objects.push(Box::new(BindablePropertyBoolean {
                property_value: value.unwrap_or(0),
            }));
        }
        ObjectSpec::BindablePropertyNumber { value } => {
            objects.push(Box::new(BindablePropertyNumber {
                property_value: value.unwrap_or(0.0),
            }));
        }
        ObjectSpec::BindablePropertyEnum { value } => {
            objects.push(Box::new(BindablePropertyEnum {
                property_value: value.unwrap_or(0),
            }));
        }
        ObjectSpec::BindablePropertyColor { value } => {
            let color = parse_color(value)?;
            objects.push(Box::new(BindablePropertyColor {
                property_value: color,
            }));
        }
        ObjectSpec::BindablePropertyTrigger { value } => {
            objects.push(Box::new(BindablePropertyTrigger {
                property_value: value.unwrap_or(0),
            }));
        }
        ObjectSpec::BindablePropertyInteger { value } => {
            objects.push(Box::new(BindablePropertyInteger {
                property_value: value.unwrap_or(0),
            }));
        }
        ObjectSpec::BindablePropertyList { value } => {
            objects.push(Box::new(BindablePropertyList {
                property_value: value.unwrap_or(0),
            }));
        }
        ObjectSpec::BindablePropertyId { value } => {
            let color = parse_color(value)?;
            objects.push(Box::new(BindablePropertyId {
                property_value: color,
            }));
        }
        ObjectSpec::BindablePropertyArtboard { value } => {
            objects.push(Box::new(BindablePropertyArtboard {
                property_value: value.unwrap_or(0),
            }));
        }
        ObjectSpec::DataBindPath {
            property_key,
            flags,
            converter_id,
        } => {
            let mut dbp = DataBindPath::new(*property_key, *flags);
            if let Some(v) = converter_id {
                dbp.converter_id = *v;
            }
            objects.push(Box::new(dbp));
        }
        ObjectSpec::TextStyleAxis { tag, axis_value } => {
            objects.push(Box::new(TextStyleAxis {
                parent_id,
                tag: tag.unwrap_or(0),
                axis_value: axis_value.unwrap_or(0.0),
            }));
        }
        ObjectSpec::TextTargetModifier { name, target_id } => {
            objects.push(Box::new(TextTargetModifier {
                name: name.clone(),
                parent_id,
                target_id: target_id.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TextFollowPathModifier {
            name,
            target_id,
            orient,
            start,
            end,
            strength,
            offset,
        } => {
            let mut modifier = TextFollowPathModifier::new(name.clone(), parent_id);
            if let Some(v) = target_id {
                modifier.target_id = *v;
            }
            if let Some(v) = orient {
                modifier.orient = *v;
            }
            if let Some(v) = start {
                modifier.start = *v;
            }
            if let Some(v) = end {
                modifier.end = *v;
            }
            if let Some(v) = strength {
                modifier.strength = *v;
            }
            if let Some(v) = offset {
                modifier.offset = *v;
            }
            objects.push(Box::new(modifier));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::TextInput {
            name,
            align_value,
            sizing_value,
            overflow_value,
            width,
            height,
            text,
            selection_radius,
            interactive,
            children,
        } => {
            let mut input = TextInput::new(name.clone(), parent_id);
            if let Some(v) = align_value {
                input.align_value = *v;
            }
            if let Some(v) = sizing_value {
                input.sizing_value = *v;
            }
            if let Some(v) = overflow_value {
                input.overflow_value = *v;
            }
            if let Some(v) = width {
                input.width = *v;
            }
            if let Some(v) = height {
                input.height = *v;
            }
            if let Some(v) = text {
                input.text = v.clone();
            }
            if let Some(v) = selection_radius {
                input.selection_radius = *v;
            }
            if let Some(v) = interactive {
                input.interactive = *v;
            }
            objects.push(Box::new(input));
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::TextInputDrawable { name, children } => {
            objects.push(Box::new(TextInputDrawable {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::TextInputCursor { name, children } => {
            objects.push(Box::new(TextInputCursor {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::TextInputText { name, children } => {
            objects.push(Box::new(text::TextInputText {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::TextInputSelection { name, children } => {
            objects.push(Box::new(TextInputSelection {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::TextInputSelectedText { name, children } => {
            objects.push(Box::new(TextInputSelectedText {
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::DataConverterRounder { name, decimals } => {
            objects.push(Box::new(data_converters::DataConverterRounder {
                name: name.clone(),
                decimals: decimals.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterToString {
            name,
            flags,
            decimals,
            color_format,
        } => {
            objects.push(Box::new(data_converters::DataConverterToString {
                name: name.clone(),
                flags: flags.unwrap_or(0),
                decimals: decimals.unwrap_or(0),
                color_format: color_format.clone().unwrap_or_default(),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterToNumber { name } => {
            objects.push(Box::new(data_converters::DataConverterToNumber {
                name: name.clone(),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterGroup { name, children } => {
            objects.push(Box::new(data_converters::DataConverterGroup {
                name: name.clone(),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::DataConverterGroupItem { converter_id } => {
            objects.push(Box::new(data_converters::DataConverterGroupItem {
                converter_id: converter_id.unwrap_or(u32::MAX as u64),
            }));
        }
        ObjectSpec::DataConverterOperationValue {
            name,
            operation_type,
            operation_value,
        } => {
            objects.push(Box::new(data_converters::DataConverterOperationValue {
                name: name.clone(),
                operation_type: operation_type.unwrap_or(0),
                operation_value: operation_value.unwrap_or(1.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterTrigger { name } => {
            objects.push(Box::new(data_converters::DataConverterTrigger {
                name: name.clone(),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterOperationViewModel {
            name,
            operation_type,
        } => {
            objects.push(Box::new(data_converters::DataConverterOperationViewModel {
                name: name.clone(),
                operation_type: operation_type.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterStringPad {
            name,
            length,
            text,
            pad_type,
        } => {
            objects.push(Box::new(data_converters::DataConverterStringPad {
                name: name.clone(),
                length: length.unwrap_or(1),
                text: text.clone().unwrap_or_default(),
                pad_type: pad_type.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterStringRemoveZeros { name } => {
            objects.push(Box::new(data_converters::DataConverterStringRemoveZeros {
                name: name.clone(),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterStringTrim { name, trim_type } => {
            objects.push(Box::new(data_converters::DataConverterStringTrim {
                name: name.clone(),
                trim_type: trim_type.unwrap_or(1),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterInterpolator {
            name,
            duration,
            interpolation_type,
            interpolator_id,
        } => {
            objects.push(Box::new(data_converters::DataConverterInterpolator {
                name: name.clone(),
                duration: duration.unwrap_or(1.0),
                interpolation_type: interpolation_type.unwrap_or(1),
                interpolator_id: interpolator_id.unwrap_or(u32::MAX as u64),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterBooleanNegate { name } => {
            objects.push(Box::new(data_converters::DataConverterBooleanNegate {
                name: name.clone(),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterRangeMapper {
            name,
            interpolation_type,
            interpolator_id,
            flags,
            min_input,
            max_input,
            min_output,
            max_output,
        } => {
            objects.push(Box::new(data_converters::DataConverterRangeMapper {
                name: name.clone(),
                interpolation_type: interpolation_type.unwrap_or(1),
                interpolator_id: interpolator_id.unwrap_or(u32::MAX as u64),
                flags: flags.unwrap_or(0),
                min_input: min_input.unwrap_or(1.0),
                max_input: max_input.unwrap_or(1.0),
                min_output: min_output.unwrap_or(1.0),
                max_output: max_output.unwrap_or(1.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterFormula {
            name,
            random_mode_value,
            children,
        } => {
            objects.push(Box::new(data_converters::DataConverterFormula {
                name: name.clone(),
                random_mode_value: random_mode_value.unwrap_or(0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::DataConverterSystemDegsToRads {
            name,
            operation_type,
        } => {
            objects.push(Box::new(data_converters::DataConverterSystemDegsToRads {
                name: name.clone(),
                operation_type: operation_type.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterSystemNormalizer {
            name,
            operation_type,
            operation_value,
        } => {
            objects.push(Box::new(data_converters::DataConverterSystemNormalizer {
                name: name.clone(),
                operation_type: operation_type.unwrap_or(0),
                operation_value: operation_value.unwrap_or(1.0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterNumberToList {
            name,
            view_model_id,
        } => {
            objects.push(Box::new(data_converters::DataConverterNumberToList {
                name: name.clone(),
                view_model_id: view_model_id.unwrap_or(u32::MAX as u64),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::DataConverterListToLength { name } => {
            objects.push(Box::new(data_converters::DataConverterListToLength {
                name: name.clone(),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::FormulaTokenArgumentSeparator => {
            objects.push(Box::new(data_converters::FormulaTokenArgumentSeparator {
                parent_id,
            }));
        }
        ObjectSpec::FormulaTokenParenthesisClose => {
            objects.push(Box::new(data_converters::FormulaTokenParenthesisClose {
                parent_id,
            }));
        }
        ObjectSpec::FormulaTokenOperation { operation_type } => {
            objects.push(Box::new(data_converters::FormulaTokenOperation {
                parent_id,
                operation_type: operation_type.unwrap_or(0),
            }));
        }
        ObjectSpec::FormulaTokenFunction { function_type } => {
            objects.push(Box::new(data_converters::FormulaTokenFunction {
                parent_id,
                function_type: function_type.unwrap_or(0),
            }));
        }
        ObjectSpec::FormulaTokenValue { operation_value } => {
            objects.push(Box::new(data_converters::FormulaTokenValue {
                parent_id,
                operation_value: operation_value.unwrap_or(1.0),
            }));
        }
        ObjectSpec::FormulaTokenParenthesisOpen => {
            objects.push(Box::new(data_converters::FormulaTokenParenthesisOpen {
                parent_id,
            }));
        }
        ObjectSpec::FormulaTokenInput => {
            objects.push(Box::new(data_converters::FormulaTokenInput { parent_id }));
        }
        ObjectSpec::ScriptedDrawable {
            name,
            script_asset_id,
            generator_function_ref,
            threshold,
            is_paused,
            speed,
            quantize,
            interactive,
            children,
        } => {
            objects.push(Box::new(scripting::ScriptedDrawable {
                name: name.clone(),
                parent_id,
                script_asset_id: script_asset_id.unwrap_or(0),
                generator_function_ref: generator_function_ref.unwrap_or(0),
                threshold: threshold.unwrap_or(0.0),
                is_paused: is_paused.unwrap_or(false),
                speed: speed.unwrap_or(1.0),
                quantize: quantize.unwrap_or(0.0),
                interactive: interactive.unwrap_or(false),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ScriptedDataConverter {
            name,
            script_asset_id,
        } => {
            objects.push(Box::new(scripting::ScriptedDataConverter {
                name: name.clone(),
                script_asset_id: script_asset_id.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScriptedLayout {
            name,
            script_asset_id,
            children,
        } => {
            objects.push(Box::new(scripting::ScriptedLayout {
                name: name.clone(),
                parent_id,
                script_asset_id: script_asset_id.unwrap_or(0),
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
                        ctx,
                    )?;
                }
            }
        }
        ObjectSpec::ScriptedPathEffect {
            name,
            is_relative,
            target_id,
        } => {
            objects.push(Box::new(scripting::ScriptedPathEffect {
                name: name.clone(),
                parent_id,
                is_relative: is_relative.unwrap_or(false),
                target_id: target_id.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScriptedListenerAction {
            script_asset_id,
            is_stateful,
        } => {
            objects.push(Box::new(scripting::ScriptedListenerAction {
                script_asset_id: script_asset_id.unwrap_or(0),
                is_stateful: is_stateful.unwrap_or(false),
            }));
        }
        ObjectSpec::ScriptedTransitionCondition {
            script_asset_id,
            is_stateful,
        } => {
            objects.push(Box::new(scripting::ScriptedTransitionCondition {
                script_asset_id: script_asset_id.unwrap_or(0),
                is_stateful: is_stateful.unwrap_or(false),
            }));
        }
        ObjectSpec::ScriptInputNumber { name } => {
            objects.push(Box::new(scripting::ScriptInputNumber {
                name: name.clone(),
                parent_id,
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScriptInputViewModelProperty {
            name,
            view_model_id,
        } => {
            objects.push(Box::new(scripting::ScriptInputViewModelProperty {
                name: name.clone(),
                parent_id,
                view_model_id: view_model_id.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScriptInputTrigger { name } => {
            objects.push(Box::new(scripting::ScriptInputTrigger {
                name: name.clone(),
                parent_id,
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScriptInputArtboard { name, artboard_id } => {
            objects.push(Box::new(scripting::ScriptInputArtboard {
                name: name.clone(),
                parent_id,
                artboard_id: artboard_id.unwrap_or(0),
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScriptInputColor { name } => {
            objects.push(Box::new(scripting::ScriptInputColor {
                name: name.clone(),
                parent_id,
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScriptInputString { name } => {
            objects.push(Box::new(scripting::ScriptInputString {
                name: name.clone(),
                parent_id,
            }));
            name_to_index.insert(name.clone(), object_index);
        }
        ObjectSpec::ScriptInputBoolean { name } => {
            objects.push(Box::new(scripting::ScriptInputBoolean {
                name: name.clone(),
                parent_id,
            }));
            name_to_index.insert(name.clone(), object_index);
        }
    }
    Ok(())
}

pub(crate) fn append_text_modifier_group_child(
    spec: &TextModifierGroupChildSpec,
    parent_id: u64,
    objects: &mut Vec<Box<dyn RiveObject>>,
) {
    match spec {
        TextModifierGroupChildSpec::TextModifierRange {
            units_value,
            type_value,
            mode_value,
            modify_from,
            modify_to,
            strength,
            clamp,
            falloff_from,
            falloff_to,
            offset,
            run_id,
        } => {
            let mut range = TextModifierRange::new(parent_id);
            if let Some(v) = units_value {
                range.units_value = *v;
            }
            if let Some(v) = type_value {
                range.type_value = *v;
            }
            if let Some(v) = mode_value {
                range.mode_value = *v;
            }
            if let Some(v) = modify_from {
                range.modify_from = *v;
            }
            if let Some(v) = modify_to {
                range.modify_to = *v;
            }
            if let Some(v) = strength {
                range.strength = *v;
            }
            if let Some(v) = clamp {
                range.clamp = *v;
            }
            if let Some(v) = falloff_from {
                range.falloff_from = *v;
            }
            if let Some(v) = falloff_to {
                range.falloff_to = *v;
            }
            if let Some(v) = offset {
                range.offset = *v;
            }
            if let Some(v) = run_id {
                range.run_id = *v;
            }
            objects.push(Box::new(range));
        }
        TextModifierGroupChildSpec::TextVariationModifier {
            axis_tag,
            axis_value,
        } => {
            objects.push(Box::new(TextVariationModifier {
                parent_id,
                axis_tag: axis_tag.unwrap_or(0),
                axis_value: axis_value.unwrap_or(0.0),
            }));
        }
    }
}
