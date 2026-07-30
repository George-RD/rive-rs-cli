use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::{SceneSpec, build_scene};
use serde_json::{Value, json};

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn gradient(kind: &str, stops: Vec<Value>) -> Value {
    json!({
        "kind": kind,
        "start_x": literal(0.0, "px"),
        "start_y": literal(0.0, "px"),
        "end_x": { "kind": "parameter", "name": "gradient_end" },
        "end_y": { "kind": "parameter", "name": "diameter" },
        "stops": stops
    })
}

fn component_scene(kind: &str) -> String {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "gradient-stage",
            "width": { "value": 240.0, "unit": "px" },
            "height": { "value": 180.0, "unit": "px" }
        },
        "components": [
            {
                "id": "badge",
                "parameters": {
                    "diameter": { "value": 72.0, "unit": "px" },
                    "gradient_end": { "value": 72.0, "unit": "px" },
                    "midpoint": { "value": 0.5, "unit": "scalar" }
                },
                "visual": [
                    {
                        "kind": "star",
                        "id": "star",
                        "width": { "kind": "parameter", "name": "diameter" },
                        "height": { "kind": "parameter", "name": "diameter" },
                        "points": 5,
                        "inner_radius": literal(0.45, "scalar"),
                        "fill": gradient(
                            kind,
                            vec![
                                json!({ "color": "#F59E0B", "position": literal(0.0, "scalar") }),
                                json!({ "color": "#EC4899", "position": { "kind": "parameter", "name": "midpoint" } }),
                                json!({ "color": "#7C3AED", "position": literal(1.0, "scalar") })
                            ]
                        )
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

fn simple_scene(fill: Value) -> String {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "gradient-stage",
            "width": { "value": 240.0, "unit": "px" },
            "height": { "value": 180.0, "unit": "px" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "rectangle",
                    "id": "panel",
                    "width": literal(120.0, "px"),
                    "height": literal(80.0, "px"),
                    "fill": fill
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

fn assert_builds(scene: Value) {
    let scene: SceneSpec =
        serde_json::from_value(scene).expect("lowered SceneSpec must deserialize");
    build_scene(&scene, None).expect("lowered SceneSpec must pass the canonical builder");
}

#[test]
fn typed_gradients_lower_through_components_deterministically_and_build() {
    for kind in ["linear_gradient", "radial_gradient"] {
        let input = component_scene(kind);
        let first = lower_authoring_json(&input).expect("first gradient lowering");
        let second = lower_authoring_json(&input).expect("second gradient lowering");

        assert_eq!(first.scene, second.scene);
        assert_eq!(first.source_map, second.source_map);

        let gradient =
            &first.scene["artboard"]["children"][0]["children"][0]["children"][1]["children"][0];
        assert_eq!(gradient["type"], kind);
        assert_eq!(gradient["end_x"], 96.0);
        assert_eq!(gradient["end_y"], 88.0);
        assert_eq!(gradient["children"][1]["position"], 0.65);

        let expanded = first
            .source_map
            .entries
            .iter()
            .find(|entry| entry.authored_id == "left/star")
            .expect("expanded gradient shape source-map entry");
        assert_eq!(expanded.runtime_names.len(), 7);
        assert_eq!(expanded.scene_paths.len(), 7);

        assert_builds(first.scene);
    }
}

#[test]
fn gradient_contract_errors_point_to_the_authored_fill() {
    let cases = [
        (
            gradient(
                "linear_gradient",
                vec![json!({
                    "color": "#F59E0B",
                    "position": literal(0.0, "scalar")
                })],
            ),
            "invalid_gradient_stops",
            "$.visual.nodes[0].fill.stops",
        ),
        (
            gradient(
                "linear_gradient",
                vec![
                    json!({ "color": "#F59E0B", "position": literal(0.0, "scalar") }),
                    json!({ "color": "#7C3AED", "position": literal(1.2, "scalar") }),
                ],
            ),
            "invalid_ratio",
            "$.visual.nodes[0].fill.stops[1].position",
        ),
        (
            gradient(
                "linear_gradient",
                vec![
                    json!({ "color": "#F59E0B", "position": literal(0.75, "scalar") }),
                    json!({ "color": "#7C3AED", "position": literal(0.25, "scalar") }),
                ],
            ),
            "invalid_gradient_stop_order",
            "$.visual.nodes[0].fill.stops[1].position",
        ),
        (
            json!({
                "kind": "radial_gradient",
                "start_x": literal(0.0, "px"),
                "start_y": literal(0.0, "px"),
                "end_x": literal(1.0, "scalar"),
                "end_y": literal(80.0, "px"),
                "stops": [
                    { "color": "#F59E0B", "position": literal(0.0, "scalar") },
                    { "color": "#7C3AED", "position": literal(1.0, "scalar") }
                ]
            }),
            "unit_mismatch",
            "$.visual.nodes[0].fill.end_x",
        ),
    ];

    for (fill, expected_code, expected_path) in cases {
        let error = lower_authoring_json(&simple_scene(fill))
            .expect_err("invalid gradient contract must fail");
        let found = error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == expected_code && diagnostic.path == expected_path
        });
        assert!(
            found,
            "missing {expected_code} at {expected_path}; diagnostics: {:#?}",
            error.diagnostics
        );
    }
}
