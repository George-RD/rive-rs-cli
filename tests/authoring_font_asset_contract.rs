use std::path::Path;

use rive_cli::{
    authoring::{authoring_schema, lower_authoring_json},
    builder::{SceneSpec, build_scene},
    objects::core::{PropertyValue, property_keys, type_keys},
};
use serde_json::{Value, json};

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn font_document(font: &str, source: &str) -> String {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "textstage",
            "width": { "value": 480.0, "unit": "px" },
            "height": { "value": 240.0, "unit": "px" }
        },
        "font_assets": {
            "inter": source
        },
        "visual": {
            "nodes": [
                {
                    "kind": "text",
                    "id": "headline",
                    "text": "Embedded type",
                    "font": font,
                    "font_size": literal(32.0, "px"),
                    "fill": "#F8FAFC"
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

#[test]
fn font_assets_lower_deterministically_link_text_and_embed_bytes() {
    let input = font_document("inter", "assets/fonts/Inter-Bold-Subset.ttf");
    let first = lower_authoring_json(&input).expect("first font asset lowering");
    let second = lower_authoring_json(&input).expect("second font asset lowering");

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let children = first.scene["artboard"]["children"]
        .as_array()
        .expect("artboard children");
    assert_eq!(children.len(), 2);

    let asset = &children[0];
    assert_eq!(asset["type"], "font_asset");
    assert_eq!(asset["name"], "auth__textstage__inter__font_asset");
    assert_eq!(asset["source"], "assets/fonts/Inter-Bold-Subset.ttf");

    let anchor = &children[1];
    let style = &anchor["children"][0]["children"][0];
    assert_eq!(style["type"], "text_style");
    assert_eq!(style["font_asset"], "auth__textstage__inter__font_asset");

    let asset_entry = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "inter")
        .expect("font asset source-map entry");
    assert_eq!(asset_entry.authored_path, "$.font_assets.inter");
    assert_eq!(
        asset_entry.runtime_names,
        vec!["auth__textstage__inter__font_asset"]
    );
    assert_eq!(asset_entry.scene_paths, vec!["/artboard/children/0"]);

    let text_entry = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "headline")
        .expect("text source-map entry");
    assert!(
        text_entry
            .scene_paths
            .iter()
            .all(|path| path.starts_with("/artboard/children/1"))
    );

    let scene: SceneSpec =
        serde_json::from_value(first.scene).expect("font SceneSpec must deserialize");
    let objects = build_scene(&scene, Some(Path::new(env!("CARGO_MANIFEST_DIR"))))
        .expect("font source must embed through the canonical builder");
    assert!(
        objects
            .iter()
            .any(|object| object.type_key() == type_keys::FONT_ASSET)
    );
    let embedded_size = objects
        .iter()
        .find(|object| object.type_key() == type_keys::FILE_ASSET_CONTENTS)
        .and_then(|object| {
            object.properties().into_iter().find_map(|property| {
                if property.key != property_keys::FILE_ASSET_CONTENTS_BYTES {
                    return None;
                }
                match property.value {
                    PropertyValue::Bytes(bytes) => Some(bytes.len()),
                    _ => None,
                }
            })
        })
        .expect("embedded font bytes");
    assert!(embedded_size > 1_000);
}

#[test]
fn unknown_font_assets_report_root_and_component_authored_paths() {
    let root_error = lower_authoring_json(&font_document(
        "missing",
        "assets/fonts/Inter-Bold-Subset.ttf",
    ))
    .expect_err("unknown root font must fail");
    assert!(root_error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unknown_font_asset" && diagnostic.path == "$.visual.nodes[0].font"
    }));

    let component = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "textstage",
            "width": { "value": 480.0, "unit": "px" },
            "height": { "value": 240.0, "unit": "px" }
        },
        "components": [
            {
                "id": "label",
                "visual": [
                    {
                        "kind": "text",
                        "id": "copy",
                        "text": "Missing type",
                        "font": "missing",
                        "font_size": literal(24.0, "px"),
                        "fill": "#0F172A"
                    }
                ]
            }
        ],
        "visual": {
            "nodes": [
                { "kind": "instance", "id": "hero", "component": "label" }
            ]
        },
        "motion": {},
        "behavior": {}
    });
    let component_error =
        lower_authoring_json(&component.to_string()).expect_err("unknown component font must fail");
    assert!(component_error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unknown_font_asset"
            && diagnostic.path == "$.components[0].visual[0].font"
    }));
}

#[test]
fn font_asset_definitions_reject_invalid_ids_and_blank_sources() {
    let blank_error = lower_authoring_json(&font_document("inter", "   "))
        .expect_err("blank font source must fail");
    assert!(blank_error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "invalid_asset_source" && diagnostic.path == "$.font_assets.inter"
    }));

    let invalid = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "textstage",
            "width": { "value": 480.0, "unit": "px" },
            "height": { "value": 240.0, "unit": "px" }
        },
        "font_assets": {
            "bad.name": "assets/fonts/Inter-Bold-Subset.ttf"
        },
        "visual": { "nodes": [] },
        "motion": {},
        "behavior": {}
    });
    let invalid_error =
        lower_authoring_json(&invalid.to_string()).expect_err("ambiguous font id must fail");
    assert!(invalid_error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "invalid_asset_id" && diagnostic.path == "$.font_assets"
    }));
}

#[test]
fn font_asset_schema_exposes_semantic_references_without_runtime_indices() {
    let schema = authoring_schema();
    let font_assets = &schema["properties"]["font_assets"];
    assert_eq!(font_assets["type"], "object");
    assert_eq!(font_assets["additionalProperties"]["type"], "string");
    assert_eq!(font_assets["default"], json!({}));

    let text = schema["$defs"]["VisualNode"]["oneOf"]
        .as_array()
        .expect("visual variants")
        .iter()
        .find(|variant| variant["properties"]["kind"]["const"] == "text")
        .expect("text visual variant");
    let properties = text["properties"].as_object().expect("text properties");
    assert!(properties.contains_key("font"));
    assert!(!properties.contains_key("font_asset_id"));
    let required = text["required"].as_array().expect("required text fields");
    assert!(!required.iter().any(|candidate| candidate == "font"));
}
