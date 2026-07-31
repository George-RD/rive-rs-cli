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

fn image_document(asset: &str, source: &str) -> String {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "imagestage",
            "width": { "value": 480.0, "unit": "px" },
            "height": { "value": 320.0, "unit": "px" }
        },
        "font_assets": {
            "inter": "assets/fonts/Inter-Bold-Subset.ttf"
        },
        "image_assets": {
            "aurora": source
        },
        "visual": {
            "nodes": [
                {
                    "kind": "image",
                    "id": "hero",
                    "asset": asset,
                    "transform": {
                        "x": literal(200.0, "px"),
                        "y": literal(96.0, "px"),
                        "rotation": literal(15.0, "degrees"),
                        "scale_x": literal(0.75, "scalar"),
                        "scale_y": literal(0.5, "scalar")
                    }
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

#[test]
fn image_assets_lower_deterministically_after_fonts_and_embed_bytes() {
    let input = image_document("aurora", "assets/textures/aurora.png");
    let first = lower_authoring_json(&input).expect("first image asset lowering");
    let second = lower_authoring_json(&input).expect("second image asset lowering");

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let children = first.scene["artboard"]["children"]
        .as_array()
        .expect("artboard children");
    assert_eq!(children.len(), 3);
    assert_eq!(children[0]["type"], "font_asset");

    let asset = &children[1];
    assert_eq!(asset["type"], "image_asset");
    assert_eq!(asset["name"], "auth__imagestage__aurora__image_asset");
    assert_eq!(asset["source"], "assets/textures/aurora.png");

    let anchor = &children[2];
    assert_eq!(anchor["type"], "node");
    assert_eq!(anchor["name"], "auth__imagestage__hero__image_anchor");
    assert_eq!(anchor["x"], 200.0);
    assert_eq!(anchor["y"], 96.0);
    assert_eq!(anchor["scale_x"], 0.75);
    assert_eq!(anchor["scale_y"], 0.5);

    let image = &anchor["children"][0];
    assert_eq!(image["type"], "image");
    assert_eq!(image["name"], "auth__imagestage__hero__image");
    assert_eq!(image["asset"], "auth__imagestage__aurora__image_asset");
    assert!(image.get("asset_id").is_none());

    let asset_entry = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_path == "$.image_assets.aurora")
        .expect("image asset source-map entry");
    assert_eq!(asset_entry.authored_id, "aurora");
    assert_eq!(
        asset_entry.runtime_names,
        vec!["auth__imagestage__aurora__image_asset"]
    );
    assert_eq!(asset_entry.scene_paths, vec!["/artboard/children/1"]);

    let image_entry = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "hero")
        .expect("image source-map entry");
    assert_eq!(
        image_entry.runtime_names,
        vec![
            "auth__imagestage__hero__image_anchor",
            "auth__imagestage__hero__image"
        ]
    );
    assert_eq!(
        image_entry.scene_paths,
        vec!["/artboard/children/2", "/artboard/children/2/children/0"]
    );

    let scene: SceneSpec =
        serde_json::from_value(first.scene).expect("image SceneSpec must deserialize");
    let objects = build_scene(&scene, Some(Path::new(env!("CARGO_MANIFEST_DIR"))))
        .expect("image source must embed through the canonical builder");
    assert!(
        objects
            .iter()
            .any(|object| object.type_key() == type_keys::IMAGE_ASSET)
    );
    let image_asset_ordinal = objects
        .iter()
        .find(|object| object.type_key() == type_keys::IMAGE)
        .and_then(|object| {
            object.properties().into_iter().find_map(|property| {
                if property.key != property_keys::IMAGE_ASSET_ID {
                    return None;
                }
                match property.value {
                    PropertyValue::UInt(value) => Some(value),
                    _ => None,
                }
            })
        })
        .expect("image asset ordinal");
    assert_eq!(image_asset_ordinal, 1);

    let embedded_sizes = objects
        .iter()
        .filter(|object| object.type_key() == type_keys::FILE_ASSET_CONTENTS)
        .filter_map(|object| {
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
        .collect::<Vec<_>>();
    assert_eq!(embedded_sizes.len(), 2);
    assert!(embedded_sizes.iter().all(|size| *size > 100));
}

#[test]
fn unknown_image_assets_report_root_and_component_authored_paths() {
    let root_error = lower_authoring_json(&image_document("missing", "assets/textures/aurora.png"))
        .expect_err("unknown root image asset must fail");
    assert!(root_error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unknown_image_asset" && diagnostic.path == "$.visual.nodes[0].asset"
    }));

    let component = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "imagestage",
            "width": { "value": 480.0, "unit": "px" },
            "height": { "value": 320.0, "unit": "px" }
        },
        "components": [
            {
                "id": "picture",
                "visual": [
                    {
                        "kind": "image",
                        "id": "photo",
                        "asset": "missing"
                    }
                ]
            }
        ],
        "visual": {
            "nodes": [
                { "kind": "instance", "id": "hero", "component": "picture" }
            ]
        },
        "motion": {},
        "behavior": {}
    });
    let component_error = lower_authoring_json(&component.to_string())
        .expect_err("unknown component image asset must fail");
    assert!(component_error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unknown_image_asset"
            && diagnostic.path == "$.components[0].visual[0].asset"
    }));
}

#[test]
fn image_asset_definitions_reject_invalid_ids_and_blank_sources() {
    let blank_error = lower_authoring_json(&image_document("aurora", "   "))
        .expect_err("blank image source must fail");
    assert!(blank_error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "invalid_asset_source" && diagnostic.path == "$.image_assets.aurora"
    }));

    let invalid = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "imagestage",
            "width": { "value": 480.0, "unit": "px" },
            "height": { "value": 320.0, "unit": "px" }
        },
        "image_assets": {
            "bad.name": "assets/textures/aurora.png"
        },
        "visual": { "nodes": [] },
        "motion": {},
        "behavior": {}
    });
    let invalid_error =
        lower_authoring_json(&invalid.to_string()).expect_err("ambiguous image id must fail");
    assert!(invalid_error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "invalid_asset_id" && diagnostic.path == "$.image_assets"
    }));
}

#[test]
fn image_asset_schema_exposes_semantic_references_without_runtime_indices() {
    let schema = authoring_schema();
    let image_assets = &schema["properties"]["image_assets"];
    assert_eq!(image_assets["type"], "object");
    assert_eq!(image_assets["additionalProperties"]["type"], "string");
    assert_eq!(image_assets["default"], json!({}));

    let image = schema["$defs"]["VisualNode"]["oneOf"]
        .as_array()
        .expect("visual variants")
        .iter()
        .find(|variant| variant["properties"]["kind"]["const"] == "image")
        .expect("image visual variant");
    let properties = image["properties"].as_object().expect("image properties");
    assert!(properties.contains_key("asset"));
    assert!(properties.contains_key("transform"));
    assert!(!properties.contains_key("asset_id"));
    let required = image["required"].as_array().expect("required image fields");
    assert!(required.iter().any(|candidate| candidate == "asset"));
}
