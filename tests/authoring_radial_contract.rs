mod support;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI};

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

fn radial(
    id: &str,
    copies: u64,
    radius: Value,
    start_angle: Value,
    angle_step: Value,
    rotate_items: bool,
    item: Value,
) -> Value {
    json!({
        "kind": "radial",
        "id": id,
        "copies": copies,
        "radius": radius,
        "start_angle": start_angle,
        "angle_step": angle_step,
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

fn assert_close(value: &Value, expected: f64) {
    let actual = value.as_f64().expect("scene value must be numeric");
    assert!(
        (actual - expected).abs() < 1.0e-6,
        "expected {expected}, found {actual}"
    );
}

#[test]
fn radial_patterns_expand_deterministically_and_build() {
    let mut pattern = radial(
        "orbit",
        4,
        literal(50.0, "px"),
        literal(0.0, "degrees"),
        literal(90.0, "degrees"),
        false,
        rectangle("tile"),
    );
    pattern["transform"] = json!({
        "x": literal(160.0, "px"),
        "y": literal(110.0, "px")
    });
    let input = document(vec![pattern]);

    let first = lower_authoring_json(&input).expect("first radial lowering");
    let second = lower_authoring_json(&input).expect("second radial lowering");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let wrapper = &first.scene["artboard"]["children"][0];
    assert_eq!(wrapper["type"], "node");
    assert_eq!(wrapper["name"], "auth__stage__orbit__radial");
    assert_eq!(wrapper["x"], 160.0);
    assert_eq!(wrapper["y"], 110.0);

    let cells = wrapper["children"].as_array().expect("radial cells");
    assert_eq!(cells.len(), 4);
    let expected = [
        ("auth__stage__orbit__p0__cell", 50.0, 0.0),
        ("auth__stage__orbit__p1__cell", 0.0, 50.0),
        ("auth__stage__orbit__p2__cell", -50.0, 0.0),
        ("auth__stage__orbit__p3__cell", 0.0, -50.0),
    ];
    for (cell, (name, x, y)) in cells.iter().zip(expected) {
        assert_eq!(cell["type"], "node");
        assert_eq!(cell["name"], name);
        assert_close(&cell["x"], x);
        assert_close(&cell["y"], y);
        assert_close(&cell["rotation"], 0.0);
        assert_eq!(cell["children"].as_array().map(Vec::len), Some(1));
    }

    let radial_entry = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "orbit")
        .expect("radial source-map entry");
    assert_eq!(radial_entry.authored_path, "$.visual.nodes[0]");
    assert_eq!(radial_entry.runtime_names.len(), 5);
    assert_eq!(radial_entry.scene_paths.len(), 5);
    assert_eq!(radial_entry.runtime_names[0], "auth__stage__orbit__radial");

    let item_entries = first
        .source_map
        .entries
        .iter()
        .filter(|entry| {
            entry.authored_path == "$.visual.nodes[0].item"
                && entry.authored_id.starts_with("orbit/p")
                && entry.authored_id.ends_with("/tile")
        })
        .count();
    assert_eq!(item_entries, 4);

    assert_builds(first.scene);
}

#[test]
fn radial_parameters_and_item_rotation_flow_through_component_overrides() {
    let input = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 220.0, "unit": "px" }
        },
        "components": [
            {
                "id": "orbit-component",
                "parameters": {
                    "radius": { "value": 30.0, "unit": "px" },
                    "start": { "value": 0.0, "unit": "degrees" },
                    "step": { "value": 120.0, "unit": "degrees" }
                },
                "visual": [
                    radial(
                        "orbit",
                        3,
                        parameter("radius"),
                        parameter("start"),
                        parameter("step"),
                        true,
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
                    "component": "orbit-component",
                    "overrides": {
                        "radius": { "value": 40.0, "unit": "px" },
                        "start": { "value": 30.0, "unit": "degrees" },
                        "step": { "value": 120.0, "unit": "degrees" }
                    }
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string();

    let lowered = lower_authoring_json(&input).expect("component radial lowering");
    let cells = lowered.scene["artboard"]["children"][0]["children"][0]["children"]
        .as_array()
        .expect("expanded radial cells");
    assert_eq!(cells.len(), 3);

    let angles = [FRAC_PI_6, 5.0 * FRAC_PI_6, 3.0 * FRAC_PI_2];
    for (cell, angle) in cells.iter().zip(angles) {
        assert_close(&cell["x"], 40.0 * angle.cos());
        assert_close(&cell["y"], 40.0 * angle.sin());
        assert_close(&cell["rotation"], angle);
    }

    let expanded = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "hero/orbit")
        .expect("expanded radial source-map entry");
    assert_eq!(
        expanded.definition_path.as_deref(),
        Some("$.components[0].visual[0]")
    );

    assert_builds(lowered.scene);
}

#[test]
fn radial_contract_rejects_invalid_counts_units_radius_and_nested_expansion() {
    let cases = [
        (
            radial(
                "bad-count",
                0,
                literal(20.0, "px"),
                literal(0.0, "degrees"),
                literal(90.0, "degrees"),
                false,
                rectangle("tile"),
            ),
            "invalid_pattern_count",
            "$.visual.nodes[0].copies",
        ),
        (
            radial(
                "bad-radius-unit",
                2,
                literal(1.0, "scalar"),
                literal(0.0, "degrees"),
                literal(90.0, "degrees"),
                false,
                rectangle("tile"),
            ),
            "unit_mismatch",
            "$.visual.nodes[0].radius",
        ),
        (
            radial(
                "negative-radius",
                2,
                literal(-1.0, "px"),
                literal(0.0, "degrees"),
                literal(90.0, "degrees"),
                false,
                rectangle("tile"),
            ),
            "invalid_pattern_radius",
            "$.visual.nodes[0].radius",
        ),
        (
            radial(
                "bad-start-unit",
                2,
                literal(20.0, "px"),
                literal(1.0, "px"),
                literal(90.0, "degrees"),
                false,
                rectangle("tile"),
            ),
            "unit_mismatch",
            "$.visual.nodes[0].start_angle",
        ),
        (
            radial(
                "bad-step-unit",
                2,
                literal(20.0, "px"),
                literal(0.0, "degrees"),
                literal(1.0, "px"),
                false,
                rectangle("tile"),
            ),
            "unit_mismatch",
            "$.visual.nodes[0].angle_step",
        ),
    ];

    for (node, code, path) in cases {
        let error =
            lower_authoring_json(&document(vec![node])).expect_err("invalid radial must fail");
        assert!(
            error
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == code && diagnostic.path == path),
            "missing {code} at {path}; diagnostics: {:#?}",
            error.diagnostics
        );
    }

    let nested = radial(
        "outer",
        100,
        literal(20.0, "px"),
        literal(0.0, "degrees"),
        literal(3.6, "degrees"),
        false,
        radial(
            "inner",
            100,
            literal(5.0, "px"),
            literal(0.0, "degrees"),
            literal(3.6, "degrees"),
            false,
            rectangle("tile"),
        ),
    );
    let error = lower_authoring_json(&document(vec![nested]))
        .expect_err("nested radial expansion must be bounded");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "pattern_expansion_node_limit"
            && diagnostic.path == "$.visual.nodes[0].item.copies"
    }));
}

#[test]
fn radial_contract_counts_generated_descendants_against_global_budget() {
    let children = (0..101)
        .map(|index| rectangle(&format!("tile-{index}")))
        .collect::<Vec<_>>();
    let input = document(vec![radial(
        "too-many-descendants",
        100,
        literal(20.0, "px"),
        literal(0.0, "degrees"),
        literal(3.6, "degrees"),
        false,
        json!({
            "kind": "group",
            "id": "bundle",
            "children": children
        }),
    )]);

    let error = lower_authoring_json(&input)
        .expect_err("pattern descendants must share the generated-item budget");
    assert!(
        error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "pattern_expansion_node_limit"
                && diagnostic.path == "$.visual.nodes[0].copies"
        }),
        "missing descendant expansion diagnostic: {:#?}",
        error.diagnostics
    );
}

#[test]
fn radial_contract_rejects_repeated_raw_scene_items_at_authored_path() {
    let input = document(vec![radial(
        "raw-orbit",
        2,
        literal(20.0, "px"),
        literal(0.0, "degrees"),
        literal(180.0, "degrees"),
        false,
        json!({
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
          }),
    )]);

    let error = lower_authoring_json(&input)
        .expect_err("repeated raw scene items must fail before runtime-name registration");
    assert!(
        error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unsupported_repeated_raw_scene_object"
                && diagnostic.path == "$.visual.nodes[0].item"
        }),
        "missing repeated raw item diagnostic: {:#?}",
        error.diagnostics
    );
}

#[test]
fn radial_rejects_derived_angles_outside_scene_number_range() {
    let input = document(vec![radial(
        "too-far",
        4,
        literal(20.0, "px"),
        literal(0.0, "radians"),
        literal(f64::from(f32::MAX / 2.0), "radians"),
        true,
        rectangle("tile"),
    )]);

    let error = lower_authoring_json(&input)
        .expect_err("derived radial angles must fit the canonical f32 scene representation");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "numeric_out_of_range"
            && diagnostic.path == "$.visual.nodes[0].angle_step"
    }));
}

#[test]
fn radial_schema_exposes_only_bounded_semantic_fields() {
    let schema = authoring_schema();
    let radial = schema["$defs"]["VisualNode"]["oneOf"]
        .as_array()
        .expect("visual variants")
        .iter()
        .find(|variant| variant["properties"]["kind"]["const"] == "radial")
        .expect("radial schema variant");

    assert_eq!(radial["properties"]["copies"]["minimum"], 1);
    assert_eq!(radial["properties"]["copies"]["maximum"], 100);
    assert!(radial["properties"].get("radius").is_some());
    assert!(radial["properties"].get("start_angle").is_some());
    assert!(radial["properties"].get("angle_step").is_some());
    assert!(radial["properties"].get("rotate_items").is_some());
    assert!(radial["properties"].get("item").is_some());

    let serialized = serde_json::to_string(radial).expect("serialize radial schema");
    assert!(!serialized.contains("type_index"));
    assert!(!serialized.contains("property_key"));
    assert!(!serialized.contains("runtime_name"));
}

#[test]
fn radial_item_component_errors_resolve_to_definition_paths() {
    let input = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 220.0, "unit": "px" }
        },
        "components": [
            {
                "id": "radial-host",
                "visual": [
                    radial(
                        "orbit",
                        1,
                        literal(20.0, "px"),
                        literal(0.0, "degrees"),
                        literal(90.0, "degrees"),
                        false,
                        json!({
                            "kind": "instance",
                            "id": "nested",
                            "component": "invalid-shape"
                        })
                    )
                ]
            },
            {
                "id": "invalid-shape",
                "visual": [
                    {
                        "kind": "rectangle",
                        "id": "bad-shape",
                        "width": literal(1.0, "scalar"),
                        "height": literal(10.0, "px"),
                        "fill": "#111827"
                    }
                ]
            }
        ],
        "visual": { "nodes": [] },
        "motion": {},
        "behavior": {}
    })
    .to_string();

    let error =
        lower_authoring_json(&input).expect_err("invalid nested component must fail validation");
    assert!(
        error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unit_mismatch"
                && diagnostic.path == "$.components[1].visual[0].width"
        }),
        "nested radial diagnostics were not rewritten to the component definition: {:#?}",
        error.diagnostics
    );
    assert!(
        error
            .diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.path.contains(".expanded[")),
        "expanded implementation paths leaked into diagnostics: {:#?}",
        error.diagnostics
    );
}

#[test]
fn radial_zero_radius_is_valid_for_rotation_only_patterns() {
    let input = document(vec![radial(
        "spinner",
        4,
        literal(0.0, "px"),
        literal(0.0, "degrees"),
        literal(90.0, "degrees"),
        true,
        rectangle("tile"),
    )]);

    let lowered = lower_authoring_json(&input).expect("zero-radius radial lowering");
    let cells = lowered.scene["artboard"]["children"][0]["children"]
        .as_array()
        .expect("radial cells");
    for (index, cell) in cells.iter().enumerate() {
        assert_close(&cell["x"], 0.0);
        assert_close(&cell["y"], 0.0);
        assert_close(&cell["rotation"], index as f64 * FRAC_PI_2);
    }
    assert_close(&cells[2]["rotation"], PI);
    assert_builds(lowered.scene);
}

#[test]
fn radial_coordinates_use_pinned_deterministic_trigonometry() {
    let angle = f64::from_bits(0xbfe9_0003_ce1c_711f);
    let input = document(vec![radial(
        "deterministic-orbit",
        1,
        literal(1.0, "px"),
        literal(angle, "radians"),
        literal(0.0, "radians"),
        false,
        rectangle("tile"),
    )]);

    let lowered = lower_authoring_json(&input).expect("deterministic radial lowering");
    let cell = &lowered.scene["artboard"]["children"][0]["children"][0];
    assert_eq!(
        cell["x"].as_f64().expect("radial x").to_bits(),
        0x3fe6_b896_4cae_d975,
        "radial x must use the pinned pure-Rust cosine result"
    );
    assert_eq!(
        cell["y"].as_f64().expect("radial y").to_bits(),
        0xbfe6_888d_01ba_048a,
        "radial y must use the pinned pure-Rust sine result"
    );
}
