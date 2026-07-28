use std::collections::HashSet;

use serde_json::Value;

pub const SUPPORTED_TRAITS: &[&str] = &[
    "has_animation",
    "has_assets",
    "has_bones",
    "has_constraints",
    "has_data_binding",
    "has_gradients",
    "has_layout",
    "has_state_machine",
    "has_text",
    "has_trim_path",
    "multi_artboard",
];

fn collect_object_types(value: &Value, out: &mut HashSet<String>) {
    if let Some(object) = value.as_object() {
        if let Some(object_type) = object.get("type").and_then(Value::as_str) {
            out.insert(object_type.to_string());
        }
        for child in object.values() {
            collect_object_types(child, out);
        }
    }
    if let Some(array) = value.as_array() {
        for child in array {
            collect_object_types(child, out);
        }
    }
}

fn has_collection(scene: &Value, field: &str) -> bool {
    if scene
        .get("artboard")
        .and_then(|artboard| artboard.get(field))
        .and_then(Value::as_array)
        .is_some_and(|items| !items.is_empty())
    {
        return true;
    }
    scene
        .get("artboards")
        .and_then(Value::as_array)
        .is_some_and(|artboards| {
            artboards.iter().any(|artboard| {
                artboard
                    .get(field)
                    .and_then(Value::as_array)
                    .is_some_and(|items| !items.is_empty())
            })
        })
}

fn scene_traits(scene: &Value) -> HashSet<String> {
    let mut object_types = HashSet::new();
    collect_object_types(scene, &mut object_types);

    let mut traits = HashSet::new();
    if has_collection(scene, "animations") {
        traits.insert("has_animation".to_string());
    }
    if has_collection(scene, "state_machines") {
        traits.insert("has_state_machine".to_string());
    }
    if scene
        .get("artboards")
        .and_then(Value::as_array)
        .is_some_and(|artboards| artboards.len() > 1)
    {
        traits.insert("multi_artboard".to_string());
    }

    for (object_type, trait_name) in [
        ("text", "has_text"),
        ("text_style", "has_text"),
        ("text_value_run", "has_text"),
        ("layout_component", "has_layout"),
        ("layout_component_style", "has_layout"),
        ("view_model", "has_data_binding"),
        ("view_model_property", "has_data_binding"),
        ("data_bind", "has_data_binding"),
        ("image_asset", "has_assets"),
        ("font_asset", "has_assets"),
        ("audio_asset", "has_assets"),
        ("file_asset_contents", "has_assets"),
        ("bone", "has_bones"),
        ("root_bone", "has_bones"),
        ("skin", "has_bones"),
        ("tendon", "has_bones"),
        ("weight", "has_bones"),
        ("cubic_weight", "has_bones"),
        ("ik_constraint", "has_constraints"),
        ("distance_constraint", "has_constraints"),
        ("transform_constraint", "has_constraints"),
        ("translation_constraint", "has_constraints"),
        ("scale_constraint", "has_constraints"),
        ("rotation_constraint", "has_constraints"),
        ("linear_gradient", "has_gradients"),
        ("radial_gradient", "has_gradients"),
        ("trim_path", "has_trim_path"),
    ] {
        if object_types.contains(object_type) {
            traits.insert(trait_name.to_string());
        }
    }
    traits
}

pub fn trait_score(scene: &Value, expected_traits: &[String]) -> (f64, Vec<String>) {
    if expected_traits.is_empty() {
        return (1.0, Vec::new());
    }
    let traits = scene_traits(scene);
    let matched = expected_traits
        .iter()
        .filter(|trait_name| traits.contains(trait_name.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    (matched.len() as f64 / expected_traits.len() as f64, matched)
}
