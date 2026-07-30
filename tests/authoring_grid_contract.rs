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
        "width": literal(16.0, "px"),
        "height": literal(12.0, "px"),
        "fill": "#2563EB"
    })
}

fn grid(id: &str, columns: u64, rows: u64, column_step: Value, row_step: Value, item: Value) -> Value {
    json!({
        "kind": "grid",
        "id": id,
        "columns": columns,
        "rows": rows,
        "column_step": column_step,
        "row_step": row_step,
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
fn grid_patterns_expand_row_major_deterministically_and_build() {
    let mut pattern = grid(
        "tiles",
        3,
        2,
        literal(40.0, "px"),
        literal(30.0, "px"),
        rectangle("tile"),
    );
    pattern["transform"] = json!({
        "x": literal(10.0, "px"),
        "y": literal(20.0, "px")
    });
    let input = document(vec![pattern]);

    let first = lower_authoring_json(&input).expect("first grid lowering");
    let second = lower_authoring_json(&input).expect("second grid lowering");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let wrapper = &first.scene["artboard"]["children"][0];
    assert_eq!(wrapper["type"], "node");
    assert_eq!(wrapper["name"], "auth__stage__tiles__grid");
    assert_eq!(wrapper["x"], 10.0);
    assert_eq!(wrapper["y"], 20.0);

    let cells = wrapper["children"].as_array().expect("grid cells");
    assert_eq!(cells.len(), 6);
    let expected = [
        ("auth__stage__tiles__r0c0__cell", 0.0, 0.0),
        ("auth__stage__tiles__r0c1__cell", 40.0, 0.0),
        ("auth__stage__tiles__r0c2__cell", 80.0, 0.0),
        ("auth__stage__tiles__r1c0__cell", 0.0, 30.0),
        ("auth__stage__tiles__r1c1__cell", 40.0, 30.0),
        ("auth__stage__tiles__r1c2__cell", 80.0, 30.0),
    ];
    for (cell, (name, x, y)) in cells.iter().zip(expected) {
        assert_eq!(cell["type"], "node");
        assert_eq!(cell["name"], name);
        assert_eq!(cell["x"], x);
        assert_eq!(cell["y"], y);
        assert_eq!(cell["children"].as_array().map(Vec::len), Some(1));
    }

    let grid_entry = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "tiles")
        .expect("grid source-map entry");
    assert_eq!(grid_entry.authored_path, "$.visual.nodes[0]");
    assert_eq!(grid_entry.runtime_names.len(), 7);
    assert_eq!(grid_entry.scene_paths.len(), 7);
    assert_eq!(grid_entry.runtime_names[0], "auth__stage__tiles__grid");
    assert_eq!(grid_entry.scene_paths[0], "/artboard/children/0");

    let item_entries = first
        .source_map
        .entries
        .iter()
        .filter(|entry| {
            entry.authored_path == "$.visual.nodes[0].item"
                && entry.authored_id.starts_with("tiles/r")
                && entry.authored_id.ends_with("/tile")
        })
        .count();
    assert_eq!(item_entries, 6);

    assert_builds(first.scene);
}

#[test]
fn grid_steps_flow_through_component_overrides() {
    let input = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 220.0, "unit": "px" }
        },
        "components": [
            {
                "id": "matrix",
                "parameters": {
                    "dx": { "value": 24.0, "unit": "px" },
                    "dy": { "value": 18.0, "unit": "px" }
                },
                "visual": [
                    grid(
                        "cells",
                        2,
                        2,
                        parameter("dx"),
                        parameter("dy"),
                        rectangle("cell")
                    )
                ]
            }
        ],
        "visual": {
            "nodes": [
                {
                    "kind": "instance",
                    "id": "hero",
                    "component": "matrix",
                    "overrides": {
                        "dx": { "value": 50.0, "unit": "px" },
                        "dy": { "value": 25.0, "unit": "px" }
                    }
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string();

    let lowered = lower_authoring_json(&input).expect("component grid lowering");
    let cells = lowered.scene["artboard"]["children"][0]["children"][0]["children"]
        .as_array()
        .expect("expanded grid cells");
    assert_eq!(cells.len(), 4);
    assert_eq!(cells[0]["x"], 0.0);
    assert_eq!(cells[0]["y"], 0.0);
    assert_eq!(cells[1]["x"], 50.0);
    assert_eq!(cells[1]["y"], 0.0);
    assert_eq!(cells[2]["x"], 0.0);
    assert_eq!(cells[2]["y"], 25.0);
    assert_eq!(cells[3]["x"], 50.0);
    assert_eq!(cells[3]["y"], 25.0);

    let expanded = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "hero/cells")
        .expect("expanded grid source-map entry");
    assert_eq!(expanded.definition_path.as_deref(), Some("$.components[0].visual[0]"));

    assert_builds(lowered.scene);
}

#[test]
fn grid_contract_rejects_invalid_counts_units_and_nested_expansion() {
    let cases = [
        (
            grid(
                "bad-rows",
                1,
                0,
                literal(10.0, "px"),
                literal(10.0, "px"),
                rectangle("tile"),
            ),
            "invalid_pattern_count",
            "$.visual.nodes[0].rows",
        ),
        (
            grid(
                "bad-columns",
                101,
                1,
                literal(10.0, "px"),
                literal(10.0, "px"),
                rectangle("tile"),
            ),
            "invalid_pattern_count",
            "$.visual.nodes[0].columns",
        ),
        (
            grid(
                "bad-step",
                2,
                2,
                literal(1.0, "scalar"),
                literal(10.0, "px"),
                rectangle("tile"),
            ),
            "unit_mismatch",
            "$.visual.nodes[0].column_step",
        ),
    ];

    for (node, code, path) in cases {
        let error = lower_authoring_json(&document(vec![node])).expect_err("invalid grid must fail");
        assert!(
            error
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == code && diagnostic.path == path),
            "missing {code} at {path}; diagnostics: {:#?}",
            error.diagnostics
        );
    }

    let nested = grid(
        "outer",
        100,
        100,
        literal(10.0, "px"),
        literal(10.0, "px"),
        grid(
            "inner",
            2,
            1,
            literal(4.0, "px"),
            literal(4.0, "px"),
            rectangle("tile"),
        ),
    );
    let error = lower_authoring_json(&document(vec![nested]))
        .expect_err("nested grid expansion must be bounded");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "pattern_expansion_node_limit"
            && diagnostic.path == "$.visual.nodes[0].item.rows"
    }));
}

#[test]
fn grid_schema_exposes_only_bounded_semantic_fields() {
    let schema = authoring_schema();
    let grid = schema["$defs"]["VisualNode"]["oneOf"]
        .as_array()
        .expect("visual variants")
        .iter()
        .find(|variant| variant["properties"]["kind"]["const"] == "grid")
        .expect("grid schema variant");

    assert_eq!(grid["properties"]["columns"]["minimum"], 1);
    assert_eq!(grid["properties"]["columns"]["maximum"], 100);
    assert_eq!(grid["properties"]["rows"]["minimum"], 1);
    assert_eq!(grid["properties"]["rows"]["maximum"], 100);
    assert!(grid["properties"].get("column_step").is_some());
    assert!(grid["properties"].get("row_step").is_some());
    assert!(grid["properties"].get("item").is_some());

    let serialized = serde_json::to_string(grid).expect("serialize grid schema");
    assert!(!serialized.contains("type_index"));
    assert!(!serialized.contains("property_key"));
    assert!(!serialized.contains("runtime_name"));
}
