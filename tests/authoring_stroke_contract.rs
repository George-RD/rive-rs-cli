mod support;

use rive_cli::authoring::{authoring_schema, lower_authoring_json};
use serde_json::{Map, Value, json};
use support::assert_builds;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn parameter(name: &str) -> Value {
    json!({ "kind": "parameter", "name": name })
}

fn gradient(kind: &str, stops: Vec<Value>) -> Value {
    json!({
        "kind": kind,
        "start_x": literal(0.0, "px"),
        "start_y": literal(0.0, "px"),
        "end_x": parameter("gradient_end"),
        "end_y": parameter("diameter"),
        "stops": stops
    })
}

fn stroke(field: &str, paint: Value, width: Value) -> Value {
    let mut stroke = Map::new();
    stroke.insert(field.to_string(), paint);
    stroke.insert("width".to_string(), width);
    Value::Object(stroke)
}

fn component_scene(stroke: Value) -> String {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stroke-stage",
            "width": { "value": 240.0, "unit": "px" },
            "height": { "value": 180.0, "unit": "px" }
        },
        "components": [
            {
                "id": "badge",
                "parameters": {
                    "diameter": { "value": 72.0, "unit": "px" },
                    "outline": { "value": 4.0, "unit": "px" },
                    "gradient_end": { "value": 72.0, "unit": "px" },
                    "midpoint": { "value": 0.5, "unit": "scalar" }
                },
                "visual": [
                    {
                        "kind": "star",
                        "id": "star",
                        "width": parameter("diameter"),
                        "height": parameter("diameter"),
                        "points": 5,
                        "inner_radius": literal(0.45, "scalar"),
                        "fill": "#F59E0B",
                        "stroke": stroke
                    }
                ]
            }
        ],
        "visual": {
            "nodes": [
                {
                    "kind": "instance",
                    "id": "left",
                    "component": "badge",
                    "overrides": {
                        "diameter": { "value": 88.0, "unit": "px" },
                        "outline": { "value": 6.0, "unit": "px" },
                        "gradient_end": { "value": 96.0, "unit": "px" },
                        "midpoint": { "value": 0.65, "unit": "scalar" }
                    },
                    "transform": {
                        "x": literal(120.0, "px"),
                        "y": literal(90.0, "px")
                    }
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

fn standard_gradient(kind: &str) -> Value {
    gradient(
        kind,
        vec![
            json!({ "color": "#0F172A", "position": literal(0.0, "scalar") }),
            json!({ "color": "#2563EB", "position": parameter("midpoint") }),
            json!({ "color": "#F8FAFC", "position": literal(1.0, "scalar") }),
        ],
    )
}

#[test]
fn legacy_solid_stroke_alias_lowers_through_components_and_builds() {
    let input = component_scene(stroke(
        "color",
        json!("#0F172A"),
        parameter("outline"),
    ));
    let first = lower_authoring_json(&input).expect("first stroke lowering");
    let second = lower_authoring_json(&input).expect("second stroke lowering");

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let children = first.scene["artboard"]["children"][0]["children"][0]["children"]
        .as_array()
        .expect("shape children");
    assert_eq!(children[2]["type"], "stroke");
    assert_eq!(children[2]["thickness"], 6.0);
    assert_eq!(children[2]["children"][0]["type"], "solid_color");
    assert_eq!(children[2]["children"][0]["color"], "#0F172A");

    let expanded = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "left/star")
        .expect("expanded stroked shape source-map entry");
    assert_eq!(expanded.runtime_names.len(), 6);
    assert_eq!(expanded.scene_paths.len(), 6);

    assert_builds(first.scene);
}

#[test]
fn typed_gradient_strokes_lower_through_components_deterministically_and_build() {
    for kind in ["linear_gradient", "radial_gradient"] {
        let input = component_scene(stroke(
            "paint",
            standard_gradient(kind),
            parameter("outline"),
        ));
        let first = lower_authoring_json(&input).expect("first gradient stroke lowering");
        let second = lower_authoring_json(&input).expect("second gradient stroke lowering");

        assert_eq!(first.scene, second.scene);
        assert_eq!(first.source_map, second.source_map);

        let stroke = &first.scene["artboard"]["children"][0]["children"][0]["children"][2];
        assert_eq!(stroke["type"], "stroke");
        assert_eq!(stroke["thickness"], 6.0);
        assert_eq!(stroke["children"][0]["type"], kind);
        assert_eq!(stroke["children"][0]["end_x"], 96.0);
        assert_eq!(stroke["children"][0]["end_y"], 88.0);
        assert_eq!(stroke["children"][0]["children"][1]["position"], 0.65);

        let expanded = first
            .source_map
            .entries
            .iter()
            .find(|entry| entry.authored_id == "left/star")
            .expect("expanded gradient-stroked shape source-map entry");
        assert_eq!(expanded.runtime_names.len(), 9);
        assert_eq!(expanded.scene_paths.len(), 9);

        assert_builds(first.scene);
    }
}

#[test]
fn gradient_stroke_contract_errors_point_to_the_authored_paint() {
    let cases = [
        (
            gradient(
                "linear_gradient",
                vec![json!({
                    "color": "#0F172A",
                    "position": literal(0.0, "scalar")
                })],
            ),
            "invalid_gradient_stops",
            "$.components[0].visual[0].stroke.paint.stops",
        ),
        (
            gradient(
                "linear_gradient",
                vec![
                    json!({ "color": "#0F172A", "position": literal(0.0, "scalar") }),
                    json!({ "color": "#F8FAFC", "position": literal(1.2, "scalar") }),
                ],
            ),
            "invalid_ratio",
            "$.components[0].visual[0].stroke.paint.stops[1].position",
        ),
        (
            gradient(
                "linear_gradient",
                vec![
                    json!({ "color": "#0F172A", "position": literal(0.75, "scalar") }),
                    json!({ "color": "#F8FAFC", "position": literal(0.25, "scalar") }),
                ],
            ),
            "invalid_gradient_stop_order",
            "$.components[0].visual[0].stroke.paint.stops[1].position",
        ),
        (
            json!({
                "kind": "radial_gradient",
                "start_x": literal(0.0, "px"),
                "start_y": literal(0.0, "px"),
                "end_x": literal(1.0, "scalar"),
                "end_y": literal(80.0, "px"),
                "stops": [
                    { "color": "#0F172A", "position": literal(0.0, "scalar") },
                    { "color": "#F8FAFC", "position": literal(1.0, "scalar") }
                ]
            }),
            "unit_mismatch",
            "$.components[0].visual[0].stroke.paint.end_x",
        ),
    ];

    for (paint, expected_code, expected_path) in cases {
        let input = component_scene(stroke("paint", paint, parameter("outline")));
        let error = lower_authoring_json(&input).expect_err("invalid gradient stroke must fail");
        assert!(
            error.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == expected_code && diagnostic.path == expected_path
            }),
            "missing {expected_code} at {expected_path}; diagnostics: {:#?}",
            error.diagnostics
        );
    }
}

#[test]
fn stroke_width_requires_positive_pixels_at_the_authored_path() {
    for (width, expected_code) in [
        (literal(0.0, "px"), "invalid_dimension"),
        (literal(2.0, "scalar"), "unit_mismatch"),
    ] {
        let input = component_scene(stroke("color", json!("#0F172A"), width));
        let error = lower_authoring_json(&input).expect_err("invalid stroke width must fail");

        assert!(error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == expected_code
                && diagnostic.path == "$.components[0].visual[0].stroke.width"
        }));
    }
}

#[test]
fn stroke_schema_exposes_the_shared_paint_contract() {
    let schema = authoring_schema();
    let stroke = &schema["$defs"]["StrokeSpec"];
    assert_eq!(stroke["properties"]["paint"]["$ref"], "#/$defs/PaintSpec");
    assert!(
        stroke["required"]
            .as_array()
            .is_some_and(|required| required.iter().any(|field| field.as_str() == Some("paint")))
    );
}
