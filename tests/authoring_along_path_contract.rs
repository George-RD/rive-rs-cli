mod support;

use std::f64::consts::FRAC_PI_2;

use rive_cli::authoring::{authoring_schema, lower_authoring_json};
use serde_json::{Value, json};
use support::assert_builds;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn parameter(name: &str) -> Value {
    json!({ "kind": "parameter", "name": name })
}

fn point(x: Value, y: Value) -> Value {
    json!({ "x": x, "y": y })
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

fn along_path(
    id: &str,
    copies: u64,
    points: Vec<Value>,
    rotate_items: bool,
    item: Value,
) -> Value {
    json!({
        "kind": "along_path",
        "id": id,
        "copies": copies,
        "points": points,
        "rotate_items": rotate_items,
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
fn along_path_expands_equal_arc_length_placements_and_builds() {
    let mut item = rectangle("tile");
    item["transform"] = json!({
        "x": literal(4.0, "px"),
        "y": literal(2.0, "px")
    });
    let mut pattern = along_path(
        "route",
        5,
        vec![
            point(literal(0.0, "px"), literal(0.0, "px")),
            point(literal(60.0, "px"), literal(0.0, "px")),
            point(literal(60.0, "px"), literal(60.0, "px")),
        ],
        true,
        item,
    );
    pattern["transform"] = json!({
        "x": literal(20.0, "px"),
        "y": literal(30.0, "px")
    });
    let input = document(vec![pattern]);

    let first = lower_authoring_json(&input).expect("first along-path lowering");
    let second = lower_authoring_json(&input).expect("second along-path lowering");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let wrapper = &first.scene["artboard"]["children"][0];
    assert_eq!(wrapper["type"], "node");
    assert_eq!(wrapper["name"], "auth__stage__route__along_path");
    assert_eq!(wrapper["x"], 20.0);
    assert_eq!(wrapper["y"], 30.0);

    let cells = wrapper["children"].as_array().expect("along-path cells");
    assert_eq!(cells.len(), 5);
    let expected = [
        (0.0, 0.0, 0.0),
        (30.0, 0.0, 0.0),
        (60.0, 0.0, FRAC_PI_2),
        (60.0, 30.0, FRAC_PI_2),
        (60.0, 60.0, FRAC_PI_2),
    ];
    for (index, (x, y, rotation)) in expected.into_iter().enumerate() {
        assert_eq!(
            cells[index]["name"],
            format!("auth__stage__route__p{index}__cell")
        );
        assert_eq!(cells[index]["x"], x);
        assert_eq!(cells[index]["y"], y);
        assert_eq!(cells[index]["rotation"], rotation);
        assert_eq!(cells[index]["scale_x"], 1.0);
        assert_eq!(cells[index]["scale_y"], 1.0);
        assert_eq!(cells[index]["children"][0]["x"], 4.0);
        assert_eq!(cells[index]["children"][0]["y"], 2.0);
    }

    let pattern_entry = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "route")
        .expect("along-path source-map entry");
    assert_eq!(pattern_entry.authored_path, "$.visual.nodes[0]");
    assert_eq!(pattern_entry.runtime_names.len(), 6);
    assert_eq!(pattern_entry.scene_paths.len(), 6);

    let expanded_ids = first
        .source_map
        .entries
        .iter()
        .filter(|entry| entry.authored_path == "$.visual.nodes[0].item")
        .map(|entry| entry.authored_id.as_str())
        .collect::<Vec<_>>();
    for index in 0..5 {
        assert!(expanded_ids.contains(&format!("route/p{index}/tile").as_str()));
    }

    assert_builds(first.scene);
}

#[test]
fn along_path_preserves_component_definition_paths_and_overrides() {
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
                    "span": { "value": 120.0, "unit": "px" },
                    "rise": { "value": 80.0, "unit": "px" }
                },
                "visual": [
                    along_path(
                        "route",
                        3,
                        vec![
                            point(literal(0.0, "px"), literal(0.0, "px")),
                            point(parameter("span"), literal(0.0, "px")),
                            point(parameter("span"), parameter("rise"))
                        ],
                        false,
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
                        "span": { "value": 40.0, "unit": "px" },
                        "rise": { "value": 40.0, "unit": "px" }
                    }
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string();

    let lowered = lower_authoring_json(&input).expect("component along-path lowering");
    let cells = lowered.scene["artboard"]["children"][0]["children"][0]["children"]
        .as_array()
        .expect("expanded along-path cells");
    assert_eq!(cells.len(), 3);
    assert_eq!(cells[0]["x"], 0.0);
    assert_eq!(cells[0]["y"], 0.0);
    assert_eq!(cells[1]["x"], 40.0);
    assert_eq!(cells[1]["y"], 0.0);
    assert_eq!(cells[2]["x"], 40.0);
    assert_eq!(cells[2]["y"], 40.0);
    assert!(cells.iter().all(|cell| cell["rotation"] == 0.0));

    let expanded = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "hero/route")
        .expect("expanded along-path source-map entry");
    assert_eq!(
        expanded.definition_path.as_deref(),
        Some("$.components[0].visual[0]")
    );

    assert_builds(lowered.scene);
}

#[test]
fn along_path_rejects_invalid_units_and_zero_length_segments() {
    for (point_index, field) in [(0, "x"), (0, "y"), (1, "x"), (1, "y")] {
        let mut pattern = along_path(
            "invalid-unit",
            2,
            vec![
                point(literal(0.0, "px"), literal(0.0, "px")),
                point(literal(100.0, "px"), literal(50.0, "px")),
            ],
            false,
            rectangle("tile"),
        );
        pattern["points"][point_index][field] = literal(1.0, "degrees");

        let error = lower_authoring_json(&document(vec![pattern]))
            .expect_err("non-pixel along-path point must fail");
        assert!(error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unit_mismatch"
                && diagnostic.path
                    == format!("$.visual.nodes[0].points[{point_index}].{field}")
        }));
    }

    let repeated_point = along_path(
        "repeated-point",
        3,
        vec![
            point(literal(0.0, "px"), literal(0.0, "px")),
            point(literal(0.0, "px"), literal(0.0, "px")),
            point(literal(20.0, "px"), literal(0.0, "px")),
        ],
        true,
        rectangle("tile"),
    );
    let error = lower_authoring_json(&document(vec![repeated_point]))
        .expect_err("zero-length along-path segment must fail");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "invalid_path_segment"
            && diagnostic.path == "$.visual.nodes[0].points[1]"
    }));
}

#[test]
fn along_path_counts_and_repetition_share_pattern_limits() {
    let path_points = vec![
        point(literal(0.0, "px"), literal(0.0, "px")),
        point(literal(100.0, "px"), literal(0.0, "px")),
    ];
    for copies in [1, 101] {
        let pattern = along_path(
            "invalid-copies",
            copies,
            path_points.clone(),
            false,
            rectangle("tile"),
        );
        let error = lower_authoring_json(&document(vec![pattern]))
            .expect_err("invalid along-path copy count must fail");
        assert!(error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "invalid_pattern_count"
                && diagnostic.path == "$.visual.nodes[0].copies"
        }));
    }

    let one_point = along_path(
        "too-short",
        2,
        vec![point(literal(0.0, "px"), literal(0.0, "px"))],
        false,
        rectangle("tile"),
    );
    let error = lower_authoring_json(&document(vec![one_point]))
        .expect_err("an along-path pattern requires at least two points");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "invalid_path_point_count"
            && diagnostic.path == "$.visual.nodes[0].points"
    }));

    let too_many_points = (0..=100)
        .map(|index| point(literal(index as f64, "px"), literal(0.0, "px")))
        .collect::<Vec<_>>();
    let pattern = along_path(
        "too-many-points",
        2,
        too_many_points,
        false,
        rectangle("tile"),
    );
    let error = lower_authoring_json(&document(vec![pattern]))
        .expect_err("an along-path pattern must bound its point count");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "invalid_path_point_count"
            && diagnostic.path == "$.visual.nodes[0].points"
    }));

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
    let raw_pattern = along_path("raw-path", 2, path_points.clone(), false, raw_item);
    let error = lower_authoring_json(&document(vec![raw_pattern]))
        .expect_err("along-path raw objects must fail before runtime-name registration");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unsupported_repeated_raw_scene_object"
            && diagnostic.path == "$.visual.nodes[0].item"
    }));

    let mut nested = rectangle("tile");
    for depth in 0..9 {
        nested = along_path(
            &format!("path-{depth}"),
            3,
            path_points.clone(),
            false,
            nested,
        );
    }
    let error = lower_authoring_json(&document(vec![nested]))
        .expect_err("nested along-path patterns must share the generated-node budget");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "pattern_expansion_node_limit" && diagnostic.path.ends_with(".copies")
    }));
}

#[test]
fn along_path_schema_exposes_only_semantic_fields() {
    let schema = authoring_schema();
    let along_path = schema["$defs"]["VisualNode"]["oneOf"]
        .as_array()
        .expect("visual variants")
        .iter()
        .find(|variant| variant["properties"]["kind"]["const"] == "along_path")
        .expect("along-path schema variant");

    let properties = along_path["properties"]
        .as_object()
        .expect("along-path properties");
    for field in [
        "copies",
        "points",
        "rotate_items",
        "item",
        "transform",
    ] {
        assert!(properties.contains_key(field));
    }
    for field in ["step", "path_length", "tangent", "rotation"] {
        assert!(!properties.contains_key(field));
    }

    assert_eq!(properties["copies"]["minimum"], 2);
    assert_eq!(properties["copies"]["maximum"], 100);
    assert_eq!(properties["points"]["minItems"], 2);
    assert_eq!(properties["points"]["maxItems"], 100);

    let required = along_path["required"]
        .as_array()
        .expect("required along-path fields");
    for field in ["kind", "id", "copies", "points", "item"] {
        assert!(required.iter().any(|candidate| candidate == field));
    }
    assert!(!required.iter().any(|candidate| candidate == "rotate_items"));

    let point = &schema["$defs"]["PathPointSpec"];
    let point_properties = point["properties"]
        .as_object()
        .expect("path-point properties");
    assert!(point_properties.contains_key("x"));
    assert!(point_properties.contains_key("y"));
    let point_required = point["required"]
        .as_array()
        .expect("required path-point fields");
    assert!(point_required.iter().any(|candidate| candidate == "x"));
    assert!(point_required.iter().any(|candidate| candidate == "y"));
}
