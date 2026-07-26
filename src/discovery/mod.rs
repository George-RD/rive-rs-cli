use crate::builder;
use serde::Serialize;

use serde_json::{Map, Value};
use std::collections::BTreeMap;

const CATEGORIES: &[&str] = &[
    "shape",
    "path",
    "paint",
    "animation",
    "state-machine",
    "constraint",
    "layout",
    "text",
    "bone",
    "asset",
    "data-bind",
    "event",
    "scripting",
    "other",
];
const EXAMPLE_ARTBOARD_WIDTH: f32 = 100.0;
const EXAMPLE_ARTBOARD_HEIGHT: f32 = 100.0;
const EXAMPLE_COLOR: &str = "#336699";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TypeInfo {
    pub name: String,
    pub category: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct FieldInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub json_type: String,
    pub required: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enum_values: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TypeDescription {
    pub name: String,
    pub category: String,
    pub summary: String,
    pub fields: Vec<FieldInfo>,
    pub valid_parents: Vec<String>,
    pub animatable: Vec<String>,
    pub example: String,
}

pub fn categories() -> Vec<String> {
    CATEGORIES
        .iter()
        .map(|category| (*category).to_owned())
        .collect()
}

pub fn list_types(category: Option<&str>) -> Vec<TypeInfo> {
    let requested = category.map(str::to_ascii_lowercase);
    if let Some(requested) = requested.as_deref()
        && !CATEGORIES.contains(&requested)
    {
        return Vec::new();
    }

    let mut types: Vec<_> = object_variants()
        .into_iter()
        .map(|variant| {
            let name = variant_name(&variant).unwrap_or_default().to_owned();
            TypeInfo {
                category: category_for(&name).to_owned(),
                summary: summary_for(&name),
                name,
            }
        })
        .filter(|info| {
            requested
                .as_deref()
                .is_none_or(|category| info.category == category)
        })
        .collect();
    types.sort_by(|left, right| left.name.cmp(&right.name));
    types
}

pub fn describe(type_name: &str) -> Option<TypeDescription> {
    let variant = object_variants().into_iter().find(|variant| {
        variant_name(variant).is_some_and(|name| name.eq_ignore_ascii_case(type_name))
    })?;
    let name = variant_name(&variant)?.to_owned();
    let properties = variant.get("properties")?.as_object()?;
    let required = variant
        .get("required")
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .unwrap_or_default();

    let mut fields: Vec<_> = properties
        .iter()
        .filter(|(field, _)| field.as_str() != "type")
        .map(|(field, schema)| {
            let enum_values = enum_values_for(&name, field, schema);
            FieldInfo {
                name: field.clone(),
                json_type: field_type(schema, &enum_values),
                required: required.contains(&field.as_str()),
                enum_values,
                description: schema
                    .get("description")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
            }
        })
        .collect();
    fields.sort_by(|left, right| left.name.cmp(&right.name));

    Some(TypeDescription {
        category: category_for(&name).to_owned(),
        summary: summary_for(&name),
        valid_parents: valid_parents_for(&name),
        animatable: builder::animatable_properties_for(&name)
            .into_iter()
            .map(str::to_owned)
            .collect(),
        example: example_for(&name, properties, &required),
        name,
        fields,
    })
}

pub fn closest_type(type_name: &str) -> Option<String> {
    let needle = type_name.to_ascii_lowercase();
    let variants = object_variants();
    let (name, distance) = variants
        .iter()
        .filter_map(variant_name)
        .map(|name| (name, levenshtein(&needle, name)))
        .min_by_key(|(_, distance)| *distance)?;
    let limit = if needle.len() <= 4 {
        1
    } else {
        (needle.len() / 3).max(2)
    };
    (distance <= limit).then(|| name.to_owned())
}

pub fn render_types_text(types: &[TypeInfo]) -> String {
    let mut grouped: BTreeMap<&str, Vec<&TypeInfo>> = BTreeMap::new();
    for info in types {
        grouped.entry(&info.category).or_default().push(info);
    }
    let mut output = String::new();
    for category in CATEGORIES {
        let Some(entries) = grouped.get(category) else {
            continue;
        };
        output.push_str(&format!("{}\n{}\n", category, "-".repeat(category.len())));
        let width = entries
            .iter()
            .map(|entry| entry.name.len())
            .max()
            .unwrap_or(0);
        for entry in entries {
            output.push_str(&format!(
                "  {:width$}  {}\n",
                entry.name,
                entry.summary,
                width = width
            ));
        }
        output.push('\n');
    }
    output.trim_end().to_owned()
}

pub fn render_description_text(description: &TypeDescription) -> String {
    let mut output = format!(
        "{} ({})\n{}\n\n{}\n\nFIELDS\n",
        description.name,
        description.category,
        "=".repeat(description.name.len() + description.category.len() + 3),
        description.summary
    );
    let width = description
        .fields
        .iter()
        .map(|field| field.name.len())
        .max()
        .unwrap_or(0);
    for field in &description.fields {
        let required = if field.required {
            "required"
        } else {
            "optional"
        };
        let values = if field.enum_values.is_empty() {
            String::new()
        } else {
            format!("  one of: {}", field.enum_values.join(", "))
        };
        let detail = field.description.as_deref().unwrap_or("");
        output.push_str(&format!(
            "  {:width$}  {:<12} {:<8}{} {}\n",
            field.name,
            field.json_type,
            required,
            values,
            detail,
            width = width
        ));
    }
    let animatable = if description.animatable.is_empty() {
        "none".to_owned()
    } else {
        description.animatable.join(", ")
    };
    let example = if description.example.trim().is_empty() {
        "  (no standalone example: this type needs additional scene context)".to_owned()
    } else {
        description.example.clone()
    };
    output.push_str(&format!(
        "\nVALID PARENTS\n  {}\n\nANIMATABLE\n  {}\n\nEXAMPLE\n{}",
        description.valid_parents.join(", "),
        animatable,
        example
    ));
    output
}

fn object_variants() -> Vec<Value> {
    crate::builder::scene_schema()
        .pointer("/$defs/ObjectSpec/oneOf")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

fn variant_name(variant: &Value) -> Option<&str> {
    variant
        .pointer("/properties/type/const")
        .and_then(Value::as_str)
}

fn category_for(name: &str) -> &'static str {
    match name {
        "shape"
        | "solo"
        | "ellipse"
        | "rectangle"
        | "triangle"
        | "polygon"
        | "star"
        | "node"
        | "image"
        | "clipping_shape"
        | "draw_rules"
        | "draw_target"
        | "joystick"
        | "guide"
        | "mesh"
        | "mesh_vertex"
        | "contour_mesh_vertex"
        | "forced_edge"
        | "n_sliced_node" => "shape",
        "path"
        | "points_path"
        | "straight_vertex"
        | "cubic_mirrored_vertex"
        | "cubic_detached_vertex"
        | "cubic_asymmetric_vertex"
        | "list_path"
        | "points_common_path" => "path",
        "fill"
        | "stroke"
        | "solid_color"
        | "linear_gradient"
        | "radial_gradient"
        | "gradient_stop"
        | "trim_path"
        | "dash_path"
        | "dash"
        | "feather"
        | "text_style_paint"
        | "scripted_path_effect" => "paint",
        name if name.contains("animation") || name.starts_with("nested_") => "animation",
        "input"
        | "layer"
        | "state"
        | "transition"
        | "listener"
        | "listener_action"
        | "scripted_listener_action"
        | "scripted_transition_condition" => "state-machine",
        name if name.contains("constraint") || name.contains("scroll_physics") => "constraint",
        name if name.contains("layout")
            || name.contains("slicer")
            || name == "axis_x"
            || name == "axis_y" =>
        {
            "layout"
        }
        name if name.starts_with("text_") || matches!(name, "text" | "text_style") => "text",
        "bone" | "root_bone" | "skin" | "tendon" | "weight" | "cubic_weight" => "bone",
        name if name.ends_with("asset")
            || matches!(
                name,
                "folder"
                    | "layered_asset"
                    | "layer_image_asset"
                    | "svg_asset"
                    | "lottie_asset"
                    | "export_audio"
                    | "blob_asset"
            ) =>
        {
            "asset"
        }
        name if name.starts_with("view_model")
            || name.starts_with("data_")
            || name.starts_with("bindable_")
            || name.starts_with("custom_property") =>
        {
            "data-bind"
        }
        name if name.contains("event") || name.ends_with("trigger") => "event",
        name if name.starts_with("script") || name.starts_with("formula_") => "scripting",
        _ => "other",
    }
}

fn summary_for(name: &str) -> String {
    let summary = match name {
        "shape" => "Groups geometry and paint components into a drawable vector shape.",
        "node" => "Provides a transformable scene-graph component.",
        "ellipse" => "Draws an oval geometry inside a shape.",
        "rectangle" => "Draws a rectangular geometry inside a shape.",
        "triangle" => "Draws a triangular geometry inside a shape.",
        "polygon" => "Draws a regular polygon geometry inside a shape.",
        "star" => "Draws a regular star geometry inside a shape.",
        "points_path" => "Defines a path from ordered vertex children.",
        "straight_vertex" => "Adds a straight segment vertex to a points path.",
        "cubic_mirrored_vertex" => "Adds a cubic vertex with mirrored handles to a points path.",
        "cubic_detached_vertex" => {
            "Adds a cubic vertex with independently positioned handles to a points path."
        }
        "cubic_asymmetric_vertex" => {
            "Adds a cubic vertex with independent handle lengths to a points path."
        }
        "fill" => "Paints the enclosed area of shape geometry.",
        "stroke" => "Paints the outline of shape geometry.",
        "solid_color" => "Supplies a single color to a fill or stroke.",
        "linear_gradient" => "Supplies a linear gradient paint to a fill or stroke.",
        "radial_gradient" => "Supplies a radial gradient paint to a fill or stroke.",
        "gradient_stop" => "Sets one color and position within a gradient.",
        "trim_path" => "Limits how much of a fill or stroke path is drawn.",
        "dash_path" => "Applies a repeating dash pattern to a fill or stroke.",
        "dash" => "Defines one dash or gap length in a dash pattern.",
        "feather" => "Softens the edge of a fill or stroke.",
        "clipping_shape" => "Uses a path to clip later drawables.",
        "solo" => "Selects one active child component at a time.",
        "nested_artboard" => "Embeds another artboard as a drawable component.",
        "nested_state_machine" => "Embeds a state machine from another artboard.",
        "nested_simple_animation" => "Embeds a linear animation from another artboard.",
        "ik_constraint" => "Rotates a bone chain toward a target.",
        "distance_constraint" => "Maintains a distance between constrained components.",
        "transform_constraint" => "Copies a target transform with configurable strengths.",
        "translation_constraint" => "Copies target translation with configurable strengths.",
        "scale_constraint" => "Copies target scale with configurable strengths.",
        "rotation_constraint" => "Copies target rotation with configurable strengths.",
        "follow_path_constraint" => "Constrains a component to follow a path.",
        "draggable_constraint" => "Makes a component respond to drag interaction.",
        "scroll_constraint" => "Converts drag interaction into scrolling.",
        "scroll_bar_constraint" => "Connects a scrollbar component to scrolling.",
        "list_follow_path_constraint" => "Places list items along a path.",
        "text" => "Draws text using text runs and styles.",
        "text_style" => "Defines typography and layout for text.",
        "text_value_run" => "Supplies editable text content for a text object.",
        "text_modifier_group" => "Groups modifiers that alter text layout or glyphs.",
        "text_style_paint" => "Applies paint styling to text.",
        "bone" => "Defines a transformable bone in a skeletal hierarchy.",
        "root_bone" => "Defines the root of a skeletal hierarchy.",
        "skin" => "Binds mesh geometry to a bone hierarchy.",
        "tendon" => "Connects bones for coordinated deformation.",
        "weight" => "Assigns a bone influence to a mesh vertex.",
        "cubic_weight" => "Assigns a cubic bone influence to a mesh vertex.",
        _ => return generic_summary(name),
    };
    summary.to_owned()
}

fn generic_summary(name: &str) -> String {
    let words = name.replace('_', " ");
    let article = match words.chars().next() {
        Some('a' | 'e' | 'i' | 'o' | 'u') => "An",
        _ => "A",
    };
    format!("{article} {words} object.")
}

fn valid_parents_for(name: &str) -> Vec<String> {
    let parents: &[&str] = match name {
        "ellipse" | "rectangle" | "triangle" | "polygon" | "star" | "points_path" | "fill"
        | "stroke" | "clipping_shape" => &["shape"],
        "straight_vertex"
        | "cubic_mirrored_vertex"
        | "cubic_detached_vertex"
        | "cubic_asymmetric_vertex" => &["points_path"],
        "solid_color" => &["fill", "stroke"],
        "linear_gradient" | "radial_gradient" => &["fill", "stroke"],
        "gradient_stop" => &["linear_gradient", "radial_gradient"],
        "trim_path" | "feather" | "dash_path" => &["fill", "stroke"],
        "dash" => &["dash_path"],
        "text_style" | "text_value_run" | "text_modifier_group" => &["text"],
        "text_modifier_range" | "text_variation_modifier" => &["text_modifier_group.children"],
        "text_style_feature" => &["text_style.children"],
        _ => &["any"],
    };
    parents.iter().map(|parent| (*parent).to_owned()).collect()
}

fn schema_type(schema: &Value) -> String {
    if let Some(types) = schema.get("type") {
        return match types {
            Value::String(value) => value.clone(),
            Value::Array(values) => values
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(" | "),
            _ => "unknown".to_owned(),
        };
    }
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        return reference.rsplit('/').next().unwrap_or(reference).to_owned();
    }
    if schema.get("enum").is_some() || schema.get("const").is_some() {
        return "string".to_owned();
    }
    "any".to_owned()
}

fn field_type(schema: &Value, enum_values: &[String]) -> String {
    let schema_type = schema_type(schema);
    if schema_type == "any" && !enum_values.is_empty() {
        "string".to_owned()
    } else {
        schema_type
    }
}

fn enum_values_for(type_name: &str, field_name: &str, schema: &Value) -> Vec<String> {
    let schema_values = enum_values(schema);
    if !schema_values.is_empty() {
        return schema_values;
    }
    let values = match (type_name, field_name) {
        ("stroke", "cap") => &["butt", "round", "square"][..],
        ("stroke", "join") => &["miter", "round", "bevel"][..],
        ("fill" | "clipping_shape", "fill_rule") => &["nonzero", "evenodd"][..],
        ("trim_path", "mode") => &["sequential", "synchronized"][..],
        _ => &[],
    };
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn enum_values(schema: &Value) -> Vec<String> {
    let mut values = schema
        .get("enum")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(json_scalar)
        .collect::<Vec<_>>();
    if let Some(value) = schema.get("const") {
        values.push(json_scalar(value));
    }
    values
}

fn json_scalar(value: &Value) -> String {
    match value {
        Value::String(value) => value.clone(),
        _ => value.to_string(),
    }
}

fn nested_artboard_example(name: &str) -> Option<String> {
    let component = serde_json::json!({
        "name": "Component",
        "width": EXAMPLE_ARTBOARD_WIDTH,
        "height": EXAMPLE_ARTBOARD_HEIGHT,
        "children": [
            {"type": "shape", "name": "shape", "children": [
                {"type": "ellipse", "name": "circle", "width": 60.0, "height": 60.0},
                {"type": "fill", "name": "fill", "children": [
                    {"type": "solid_color", "name": "color", "color": "#3B82F6"}
                ]}
            ]}
        ],
        "animations": [
            {"name": "spin", "fps": 60, "duration": 60, "keyframes": [
                {"object": "shape", "property": "rotation", "frames": [
                    {"frame": 0, "value": 0.0},
                    {"frame": 59, "value": std::f64::consts::TAU}
                ]}
            ]}
        ],
        "state_machines": [
            {"name": "Machine", "inputs": [
                {"type": "bool", "name": "toggle", "value": false},
                {"type": "number", "name": "amount", "value": 0},
                {"type": "trigger", "name": "ping"}
            ], "layers": [
                {"states": [{"type": "entry"}, {"type": "animation", "animation": "spin"}, {"type": "exit"}],
                 "transitions": [{"from": 0, "to": 1}]}
            ]}
        ]
    });
    let nested_child = match name {
        "nested_artboard" | "nested_artboard_layout" | "nested_artboard_leaf" => None,
        "nested_linear_animation" | "nested_simple_animation" | "nested_remap_animation" => {
            Some(serde_json::json!({"type": name, "name": "clip", "animation": "spin"}))
        }
        "nested_state_machine" => {
            Some(serde_json::json!({"type": name, "name": "machine", "animation": "spin"}))
        }
        "nested_bool" => Some(
            serde_json::json!({"type": name, "name": "flag", "nested_input_id": 0, "value": true}),
        ),
        "nested_number" => Some(
            serde_json::json!({"type": name, "name": "level", "nested_input_id": 1, "value": 1.0}),
        ),
        "nested_trigger" => {
            Some(serde_json::json!({"type": name, "name": "fire", "nested_input_id": 2}))
        }
        _ => return None,
    };
    let mut host = serde_json::json!({
        "type": "nested_artboard",
        "name": "embedded",
        "source_artboard": "Component",
        "x": 50,
        "y": 50
    });
    match nested_child {
        Some(child) => {
            host["children"] = serde_json::json!([child]);
        }
        None => {
            host["type"] = Value::String(name.to_owned());
        }
    }
    let mut host_artboard = component.clone();
    host_artboard["name"] = Value::String("Example".to_owned());
    let mut host_children = vec![host];
    if let Some(existing) = component["children"].as_array() {
        host_children.extend(existing.iter().cloned());
    }
    host_artboard["children"] = Value::Array(host_children);
    validated_example(&serde_json::json!({
        "scene_format_version": 1,
        "artboards": [host_artboard, component]
    }))
}

fn bone_example(name: &str) -> Option<Vec<Value>> {
    let root = serde_json::json!({
        "type": "root_bone", "name": "Spine", "x": 100.0, "y": 160.0, "length": 60.0,
        "children": [{"type": "bone", "name": "Torso", "length": 40.0}]
    });
    match name {
        "bone" => Some(vec![serde_json::json!({
            "type": "root_bone", "name": "Spine", "x": 100.0, "y": 160.0, "length": 60.0,
            "children": [{"type": "bone", "name": "example", "length": 40.0}]
        })]),
        "root_bone" => Some(vec![serde_json::json!({
            "type": "root_bone", "name": "example", "x": 100.0, "y": 160.0, "length": 60.0
        })]),
        "tendon" => Some(vec![
            root,
            serde_json::json!({
                "type": "skin", "name": "skin", "xx": 1.0, "yy": 1.0,
                "children": [{"type": "tendon", "name": "example", "bone": "Spine", "xx": 1.0, "yy": 1.0}]
            }),
        ]),
        "skin" => Some(vec![
            root,
            serde_json::json!({
                "type": "skin", "name": "example", "xx": 1.0, "yy": 1.0,
                "children": [{"type": "tendon", "name": "tendon", "bone": "Spine", "xx": 1.0, "yy": 1.0}]
            }),
        ]),
        _ => None,
    }
}

fn validated_example(scene: &Value) -> Option<String> {
    let spec: crate::builder::SceneSpec = serde_json::from_value(scene.clone()).ok()?;
    crate::builder::build_scene(&spec).ok()?;
    serde_json::to_string_pretty(scene).ok()
}

fn example_for(name: &str, properties: &Map<String, Value>, required: &[&str]) -> String {
    if let Some(scene) = nested_artboard_example(name) {
        return scene;
    }
    let object = minimal_object(name, properties, required);
    let context = match name {
        "fill" | "stroke" => {
            serde_json::json!({"type":"shape", "name":"shape", "children":[object]})
        }
        "solid_color" => {
            serde_json::json!({"type":"shape", "name":"shape", "children":[{"type":"fill", "name":"fill", "children":[object]}]})
        }
        "linear_gradient" | "radial_gradient" => {
            serde_json::json!({"type":"shape", "name":"shape", "children":[{"type":"fill", "name":"fill", "children":[object]}]})
        }
        "gradient_stop" => {
            serde_json::json!({"type":"shape", "name":"shape", "children":[{"type":"fill", "name":"fill", "children":[{"type":"linear_gradient", "name":"gradient", "start_x":0, "start_y":0, "end_x":100, "end_y":100, "children":[object]}]}]})
        }
        "ik_constraint"
        | "distance_constraint"
        | "transform_constraint"
        | "translation_constraint"
        | "scale_constraint"
        | "rotation_constraint"
        | "follow_path_constraint"
        | "list_follow_path_constraint" => {
            serde_json::json!({"type":"shape", "name":"shape", "children":[object]})
        }
        "image" => {
            serde_json::json!({"type":"shape", "name":"shape", "children":[{"type":"image_asset", "name":"asset"}, object]})
        }
        "points_path" => serde_json::json!({"type":"shape", "name":"shape", "children":[object]}),
        "straight_vertex"
        | "cubic_mirrored_vertex"
        | "cubic_detached_vertex"
        | "cubic_asymmetric_vertex" => {
            serde_json::json!({"type":"shape", "name":"shape", "children":[{"type":"points_path", "name":"path", "children":[object]}]})
        }
        "trim_path" | "feather" | "dash_path" => {
            serde_json::json!({"type":"shape", "name":"shape", "children":[{"type":"stroke", "name":"stroke", "children":[object]}]})
        }
        "dash" => {
            serde_json::json!({"type":"shape", "name":"shape", "children":[{"type":"stroke", "name":"stroke", "children":[{"type":"dash_path", "name":"dashes", "children":[object]}]}]})
        }
        "text_modifier_range" | "text_variation_modifier" => {
            serde_json::json!({"type":"text", "name":"text", "children":[{"type":"text_modifier_group", "name":"modifiers", "children":[object]}]})
        }
        "text_style_feature" => {
            serde_json::json!({"type":"text", "name":"text", "children":[{"type":"text_style", "name":"style", "children":[object]}]})
        }
        _ => object,
    };
    let children = match bone_example(name) {
        Some(objects) => Value::Array(objects),
        None => serde_json::json!([context]),
    };
    let scene = serde_json::json!({
        "scene_format_version": 1,
        "artboard": {"name": "Example", "width": EXAMPLE_ARTBOARD_WIDTH, "height": EXAMPLE_ARTBOARD_HEIGHT, "children": children}
    });
    validated_example(&scene).unwrap_or_default()
}

fn minimal_object(name: &str, properties: &Map<String, Value>, required: &[&str]) -> Value {
    let mut object = Map::new();
    object.insert("type".to_owned(), Value::String(name.to_owned()));
    for field in required {
        if *field != "type"
            && let Some(schema) = properties.get(*field)
        {
            object.insert((*field).to_owned(), sample_value(name, field, schema));
        }
    }
    if properties.contains_key("target") {
        object.insert("target".to_owned(), Value::String("shape".to_owned()));
    }
    Value::Object(object)
}

fn sample_value(type_name: &str, field: &str, schema: &Value) -> Value {
    if let Some(value) = schema.get("const") {
        return value.clone();
    }
    if let Some(value) = schema
        .get("enum")
        .and_then(Value::as_array)
        .and_then(|values| values.first())
    {
        return value.clone();
    }
    if field == "asset_id" {
        return Value::from(0);
    }
    if field == "target" || field == "source" {
        return Value::String("shape".to_owned());
    }
    if type_name.contains("color")
        || type_name.contains("bindable_property_id")
        || field.contains("color")
    {
        return Value::String(EXAMPLE_COLOR.to_owned());
    }
    let types = schema.get("type");
    match types {
        Some(Value::String(kind)) => sample_for_type(kind),
        Some(Value::Array(kinds)) => kinds
            .iter()
            .filter_map(Value::as_str)
            .find(|kind| *kind != "null")
            .map(sample_for_type)
            .unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

fn sample_for_type(kind: &str) -> Value {
    match kind {
        "string" => Value::String("example".to_owned()),
        "integer" => Value::from(1),
        "number" => Value::from(1),
        "boolean" => Value::Bool(false),
        "array" => Value::Array(Vec::new()),
        "object" => Value::Object(Map::new()),
        _ => Value::Null,
    }
}

fn levenshtein(left: &str, right: &str) -> usize {
    let mut previous: Vec<usize> = (0..=right.chars().count()).collect();
    for (left_index, left_char) in left.chars().enumerate() {
        let mut current = vec![left_index + 1];
        for (right_index, right_char) in right.chars().enumerate() {
            current.push(
                (previous[right_index + 1] + 1)
                    .min(current[right_index] + 1)
                    .min(previous[right_index] + usize::from(left_char != right_char)),
            );
        }
        previous = current;
    }
    previous[right.chars().count()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_schema_object_type_has_a_category() {
        let schema_count = object_variants().len();
        let discovered = list_types(None);
        assert_eq!(discovered.len(), schema_count);
        let known = categories();
        for info in &discovered {
            assert!(
                known.contains(&info.category),
                "type {} has category '{}' which is not in the published category list {:?}",
                info.name,
                info.category,
                known
            );
            assert!(
                !list_types(Some(&info.category)).is_empty(),
                "category '{}' must be selectable via types --category",
                info.category
            );
        }
    }

    #[test]
    fn every_type_example_builds() {
        let mut with_examples = 0;
        for info in list_types(None) {
            let description = describe(&info.name).expect("listed type is describable");
            if description.example.trim().is_empty() {
                continue;
            }
            with_examples += 1;
            let spec: builder::SceneSpec = serde_json::from_str(&description.example)
                .unwrap_or_else(|error| panic!("{} example JSON failed: {}", info.name, error));
            builder::build_scene(&spec)
                .unwrap_or_else(|error| panic!("{} example failed: {}", info.name, error));
        }
        assert!(
            with_examples > 100,
            "expected most types to carry a validated example, got {with_examples}"
        );
        for core in [
            "shape",
            "ellipse",
            "rectangle",
            "fill",
            "stroke",
            "solid_color",
            "linear_gradient",
            "gradient_stop",
            "trim_path",
            "nested_artboard",
        ] {
            let description = describe(core).expect("core type is describable");
            assert!(
                !description.example.trim().is_empty(),
                "core type {core} must carry a validated example"
            );
        }
    }

    #[test]
    fn trim_path_uses_animation_property_names_and_paint_parents() {
        let description = describe("trim_path").unwrap();
        assert!(description.animatable.contains(&"trim_start".to_owned()));
        assert!(!description.animatable.contains(&"start".to_owned()));
        assert!(description.valid_parents.contains(&"stroke".to_owned()));
        assert!(description.valid_parents.contains(&"fill".to_owned()));
        assert!(!description.valid_parents.contains(&"shape".to_owned()));
    }

    #[test]
    fn stroke_thickness_is_not_animatable() {
        assert!(
            !describe("stroke")
                .unwrap()
                .animatable
                .contains(&"thickness".to_owned())
        );
    }

    #[test]
    fn text_style_has_only_text_style_properties() {
        assert_eq!(
            describe("text_style").unwrap().animatable,
            ["font_size", "line_height", "letter_spacing"]
        );
    }

    #[test]
    fn animatable_lists_match_the_builder_resolver() {
        for info in list_types(None) {
            let description = describe(&info.name).expect("listed type is describable");
            for property in &description.animatable {
                assert!(
                    builder::property_key_for_object(&info.name, property).is_some(),
                    "{} reports unresolved animatable property {}",
                    info.name,
                    property
                );
            }
        }

        let reachable: std::collections::BTreeSet<_> = list_types(None)
            .into_iter()
            .flat_map(|info| describe(&info.name).unwrap().animatable)
            .collect();
        for property in builder::generic_animatable_properties() {
            assert!(
                reachable.contains(property),
                "generic property {} is not reported by any object type",
                property
            );
        }
    }

    #[test]
    fn closest_type_corrects_common_ellipse_typo() {
        assert_eq!(closest_type("elipse").as_deref(), Some("ellipse"));
    }
    #[test]
    fn summaries_use_correct_articles() {
        for info in list_types(None) {
            let summary = info.summary.to_ascii_lowercase();
            assert!(!summary.starts_with("a a"));
            assert!(!summary.starts_with("a e"));
            assert!(!summary.starts_with("a i"));
            assert!(!summary.starts_with("a o"));
        }
    }

    #[test]
    fn stroke_cap_reports_legal_values() {
        let field = describe("stroke")
            .unwrap()
            .fields
            .into_iter()
            .find(|field| field.name == "cap")
            .unwrap();
        assert_eq!(field.enum_values, ["butt", "round", "square"]);
    }
}
