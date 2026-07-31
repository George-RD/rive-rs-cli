mod support;

use rive_cli::authoring::{authoring_schema, lower_authoring_json};
use serde_json::{Value, json};
use support::assert_builds;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn rectangle(id: &str) -> Value {
    json!({
        "kind": "rectangle",
        "id": id,
        "width": literal(24.0, "px"),
        "height": literal(16.0, "px"),
        "fill": "#2563EB"
    })
}

fn mirror(id: &str, axis: &str, item: Value) -> Value {
    json!({
        "kind": "mirror",
        "id": id,
        "axis": axis,
        "item": item
    })
}

fn document(nodes: Vec<Value>) -> String {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 220.0, "unit": "px" }
        },
        "visual": { "nodes": nodes },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

#[test]
fn vertical_mirror_expands_deterministically_and_builds() {
    let mut item = rectangle("tile");
    item["transform"] = json!({ "x": literal(36.0, "px") });
    let mut pattern = mirror("duet", "vertical", item);
    pattern["transform"] = json!({
        "x": literal(160.0, "px"),
        "y": literal(110.0, "px")
    });
    let input = document(vec![pattern]);

    let first = lower_authoring_json(&input).expect("first mirror lowering");
    let second = lower_authoring_json(&input).expect("second mirror lowering");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let wrapper = &first.scene["artboard"]["children"][0];
    assert_eq!(wrapper["type"], "node");
    assert_eq!(wrapper["name"], "auth__stage__duet__mirror");
    assert_eq!(wrapper["x"], 160.0);
    assert_eq!(wrapper["y"], 110.0);

    let cells = wrapper["children"].as_array().expect("mirror cells");
    assert_eq!(cells.len(), 2);
    assert_eq!(cells[0]["name"], "auth__stage__duet__original__cell");
    assert_eq!(cells[0]["scale_x"], 1.0);
    assert_eq!(cells[0]["scale_y"], 1.0);
    assert_eq!(cells[1]["name"], "auth__stage__duet__mirrored__cell");
    assert_eq!(cells[1]["scale_x"], -1.0);
    assert_eq!(cells[1]["scale_y"], 1.0);
    assert_eq!(cells[0]["children"][0]["x"], 36.0);
    assert_eq!(cells[1]["children"][0]["x"], 36.0);

    let mirror_entry = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "duet")
        .expect("mirror source-map entry");
    assert_eq!(mirror_entry.authored_path, "$.visual.nodes[0]");
    assert_eq!(
        mirror_entry.runtime_names,
        [
            "auth__stage__duet__mirror",
            "auth__stage__duet__original__cell",
            "auth__stage__duet__mirrored__cell"
        ]
    );
    assert_eq!(mirror_entry.scene_paths.len(), 3);

    let expanded_ids = first
        .source_map
        .entries
        .iter()
        .filter(|entry| entry.authored_path == "$.visual.nodes[0].item")
        .map(|entry| entry.authored_id.as_str())
        .collect::<Vec<_>>();
    assert!(expanded_ids.contains(&"duet/original/tile"));
    assert!(expanded_ids.contains(&"duet/mirrored/tile"));

    assert_builds(first.scene);
}

#[test]
fn horizontal_mirror_preserves_component_paths_and_reflects_y() {
    let input = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 220.0, "unit": "px" }
        },
        "components": [
            {
                "id": "badge",
                "visual": [mirror("pair", "horizontal", rectangle("tile"))]
            }
        ],
        "visual": {
            "nodes": [
                {
                    "kind": "instance",
                    "id": "hero",
                    "component": "badge"
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string();

    let lowered = lower_authoring_json(&input).expect("component mirror lowering");
    let cells = lowered.scene["artboard"]["children"][0]["children"][0]["children"]
        .as_array()
        .expect("expanded mirror cells");
    assert_eq!(cells.len(), 2);
    assert_eq!(cells[0]["scale_x"], 1.0);
    assert_eq!(cells[0]["scale_y"], 1.0);
    assert_eq!(cells[1]["scale_x"], 1.0);
    assert_eq!(cells[1]["scale_y"], -1.0);

    let expanded = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "hero/pair")
        .expect("expanded mirror source-map entry");
    assert_eq!(
        expanded.definition_path.as_deref(),
        Some("$.components[0].visual[0]")
    );

    assert_builds(lowered.scene);
}

#[test]
fn mirror_repetition_is_bounded_and_rejects_raw_scene_items() {
    let raw_item = json!({
        "kind": "raw_scene_object",
        "id": "raw-tile",
        "object": {
            "type": "node",
            "name": "embedded-node",
            "x": 0.0,
            "y": 0.0,
            "rotation": 0.0,
            "scale_x": 1.0,
            "scale_y": 1.0,
            "children": []
        }
    });
    let error = lower_authoring_json(&document(vec![mirror("raw-pair", "vertical", raw_item)]))
        .expect_err("mirrored raw objects must fail before runtime-name registration");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unsupported_repeated_raw_scene_object"
            && diagnostic.path == "$.visual.nodes[0].item"
    }));

    let mut nested = rectangle("tile");
    for depth in 0..14 {
        nested = mirror(&format!("mirror-{depth}"), "vertical", nested);
    }
    let error = lower_authoring_json(&document(vec![nested]))
        .expect_err("nested mirrors must share the generated-node budget");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "pattern_expansion_node_limit" && diagnostic.path.ends_with(".item")
    }));
}

#[test]
fn mirror_schema_exposes_only_semantic_fields() {
    let schema = authoring_schema();
    let mirror = schema["$defs"]["VisualNode"]["oneOf"]
        .as_array()
        .expect("visual variants")
        .iter()
        .find(|variant| variant["properties"]["kind"]["const"] == "mirror")
        .expect("mirror schema variant");

    let properties = mirror["properties"].as_object().expect("mirror properties");
    assert!(properties.contains_key("axis"));
    assert!(properties.contains_key("item"));
    assert!(properties.contains_key("transform"));
    assert!(!properties.contains_key("copies"));
    assert!(!properties.contains_key("scale_x"));
    assert!(!properties.contains_key("scale_y"));

    let required = mirror["required"]
        .as_array()
        .expect("required mirror fields");
    for field in ["kind", "id", "axis", "item"] {
        assert!(required.iter().any(|candidate| candidate == field));
    }
}
