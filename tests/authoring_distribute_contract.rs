mod support;

use rive_cli::authoring::{authoring_schema, lower_authoring_json};
use serde_json::{Value, json};
use support::assert_builds;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn parameter(name: &str) -> Value {
    json!({ "kind": "parameter", "name": name })
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

fn distribute(
    id: &str,
    copies: u64,
    start_x: Value,
    start_y: Value,
    end_x: Value,
    end_y: Value,
    item: Value,
) -> Value {
    json!({
        "kind": "distribute",
        "id": id,
        "copies": copies,
        "start_x": start_x,
        "start_y": start_y,
        "end_x": end_x,
        "end_y": end_y,
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
fn distribute_expands_endpoint_inclusive_placements_deterministically_and_builds() {
    let mut item = rectangle("tile");
    item["transform"] = json!({
        "x": literal(12.0, "px"),
        "y": literal(8.0, "px")
    });
    let mut pattern = distribute(
        "run",
        4,
        literal(0.0, "px"),
        literal(0.0, "px"),
        literal(90.0, "px"),
        literal(45.0, "px"),
        item,
    );
    pattern["transform"] = json!({
        "x": literal(20.0, "px"),
        "y": literal(30.0, "px")
    });
    let input = document(vec![pattern]);

    let first = lower_authoring_json(&input).expect("first distribute lowering");
    let second = lower_authoring_json(&input).expect("second distribute lowering");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let wrapper = &first.scene["artboard"]["children"][0];
    assert_eq!(wrapper["type"], "node");
    assert_eq!(wrapper["name"], "auth__stage__run__distribute");
    assert_eq!(wrapper["x"], 20.0);
    assert_eq!(wrapper["y"], 30.0);

    let cells = wrapper["children"].as_array().expect("distribute cells");
    assert_eq!(cells.len(), 4);
    for (index, (x, y)) in [(0.0, 0.0), (30.0, 15.0), (60.0, 30.0), (90.0, 45.0)]
        .into_iter()
        .enumerate()
    {
        assert_eq!(
            cells[index]["name"],
            format!("auth__stage__run__d{index}__cell")
        );
        assert_eq!(cells[index]["x"], x);
        assert_eq!(cells[index]["y"], y);
        assert_eq!(cells[index]["rotation"], 0.0);
        assert_eq!(cells[index]["scale_x"], 1.0);
        assert_eq!(cells[index]["scale_y"], 1.0);
        assert_eq!(cells[index]["children"][0]["x"], 12.0);
        assert_eq!(cells[index]["children"][0]["y"], 8.0);
    }

    let distribute_entry = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "run")
        .expect("distribute source-map entry");
    assert_eq!(distribute_entry.authored_path, "$.visual.nodes[0]");
    assert_eq!(distribute_entry.runtime_names.len(), 5);
    assert_eq!(distribute_entry.scene_paths.len(), 5);

    let expanded_ids = first
        .source_map
        .entries
        .iter()
        .filter(|entry| entry.authored_path == "$.visual.nodes[0].item")
        .map(|entry| entry.authored_id.as_str())
        .collect::<Vec<_>>();
    for index in 0..4 {
        assert!(expanded_ids.contains(&format!("run/d{index}/tile").as_str()));
    }

    assert_builds(first.scene);
}

#[test]
fn distribute_preserves_component_definition_paths_and_parameter_overrides() {
    let input = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 220.0, "unit": "px" }
        },
        "components": [
            {
                "id": "rail",
                "parameters": {
                    "span": { "value": 120.0, "unit": "px" }
                },
                "visual": [
                    distribute(
                        "line",
                        3,
                        literal(0.0, "px"),
                        literal(10.0, "px"),
                        parameter("span"),
                        literal(50.0, "px"),
                        rectangle("tile")
                    )
                ]
            }
        ],
        "visual": {
            "nodes": [
                {
                    "kind": "instance",
                    "id": "hero",
                    "component": "rail",
                    "overrides": {
                        "span": { "value": 80.0, "unit": "px" }
                    }
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string();

    let lowered = lower_authoring_json(&input).expect("component distribute lowering");
    let cells = lowered.scene["artboard"]["children"][0]["children"][0]["children"]
        .as_array()
        .expect("expanded distribute cells");
    assert_eq!(cells.len(), 3);
    assert_eq!(cells[0]["x"], 0.0);
    assert_eq!(cells[0]["y"], 10.0);
    assert_eq!(cells[1]["x"], 40.0);
    assert_eq!(cells[1]["y"], 30.0);
    assert_eq!(cells[2]["x"], 80.0);
    assert_eq!(cells[2]["y"], 50.0);

    let expanded = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "hero/line")
        .expect("expanded distribute source-map entry");
    assert_eq!(
        expanded.definition_path.as_deref(),
        Some("$.components[0].visual[0]")
    );

    assert_builds(lowered.scene);
}

#[test]
fn distribute_counts_and_repetition_share_pattern_safety_limits() {
    for copies in [1, 101] {
        let pattern = distribute(
            "invalid",
            copies,
            literal(0.0, "px"),
            literal(0.0, "px"),
            literal(100.0, "px"),
            literal(0.0, "px"),
            rectangle("tile"),
        );
        let error = lower_authoring_json(&document(vec![pattern]))
            .expect_err("invalid distribute count must fail");
        assert!(error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "invalid_pattern_count"
                && diagnostic.path == "$.visual.nodes[0].copies"
        }));
    }

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
    let raw_pattern = distribute(
        "raw-line",
        2,
        literal(0.0, "px"),
        literal(0.0, "px"),
        literal(100.0, "px"),
        literal(0.0, "px"),
        raw_item,
    );
    let error = lower_authoring_json(&document(vec![raw_pattern]))
        .expect_err("distributed raw objects must fail before runtime-name registration");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unsupported_repeated_raw_scene_object"
            && diagnostic.path == "$.visual.nodes[0].item"
    }));

    let mut nested = rectangle("tile");
    for depth in 0..9 {
        nested = distribute(
            &format!("line-{depth}"),
            3,
            literal(0.0, "px"),
            literal(0.0, "px"),
            literal(30.0, "px"),
            literal(0.0, "px"),
            nested,
        );
    }
    let error = lower_authoring_json(&document(vec![nested]))
        .expect_err("nested distributions must share the generated-node budget");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "pattern_expansion_node_limit" && diagnostic.path.ends_with(".copies")
    }));
}

#[test]
fn distribute_schema_exposes_only_semantic_fields() {
    let schema = authoring_schema();
    let distribute = schema["$defs"]["VisualNode"]["oneOf"]
        .as_array()
        .expect("visual variants")
        .iter()
        .find(|variant| variant["properties"]["kind"]["const"] == "distribute")
        .expect("distribute schema variant");

    let properties = distribute["properties"]
        .as_object()
        .expect("distribute properties");
    for field in [
        "copies",
        "start_x",
        "start_y",
        "end_x",
        "end_y",
        "item",
        "transform",
    ] {
        assert!(properties.contains_key(field));
    }
    for field in ["step_x", "step_y", "rotation", "scale_x", "scale_y"] {
        assert!(!properties.contains_key(field));
    }

    let required = distribute["required"]
        .as_array()
        .expect("required distribute fields");
    for field in [
        "kind", "id", "copies", "start_x", "start_y", "end_x", "end_y", "item",
    ] {
        assert!(required.iter().any(|candidate| candidate == field));
    }
}
