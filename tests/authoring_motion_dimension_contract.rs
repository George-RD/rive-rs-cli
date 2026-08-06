mod support;

use rive_cli::authoring::{authoring_schema, lower_authoring_json};
use serde_json::{Value, json};
use support::assert_builds;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn document() -> Value {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "dimension-stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 200.0, "unit": "px" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "rectangle",
                    "id": "card",
                    "width": literal(80.0, "px"),
                    "height": literal(48.0, "px"),
                    "fill": "#172554"
                }
            ]
        },
        "motion": {
            "poses": [
                {
                    "id": "compact",
                    "targets": [
                        {
                            "target": "card",
                            "transform": { "x": literal(40.0, "px") },
                            "width": literal(80.0, "px"),
                            "height": literal(48.0, "px")
                        }
                    ]
                },
                {
                    "id": "expanded",
                    "targets": [
                        {
                            "target": "card",
                            "transform": { "x": literal(160.0, "px") },
                            "width": literal(160.0, "px"),
                            "height": literal(96.0, "px")
                        }
                    ]
                }
            ],
            "tracks": [
                {
                    "id": "resize",
                    "fps": 60,
                    "duration_frames": literal(30.0, "scalar"),
                    "keyframes": [
                        { "frame": literal(0.0, "scalar"), "pose": "compact" },
                        { "frame": literal(30.0, "scalar"), "pose": "expanded" }
                    ]
                }
            ]
        },
        "behavior": {}
    })
}

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("dimension motion must lower")
}

fn has_diagnostic(error: &rive_cli::authoring::AuthoringError, code: &str, path: &str) -> bool {
    error
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == code && diagnostic.path == path)
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string())
        .expect_err("invalid dimension motion must fail at the authored boundary");
    assert!(
        has_diagnostic(&error, code, path),
        "missing {code} at {path}; diagnostics: {:#?}",
        error.diagnostics
    );
}

fn keyframe_group<'a>(animation: &'a Value, object: &str, property: &str) -> &'a Value {
    animation["keyframes"]
        .as_array()
        .expect("animation keyframes")
        .iter()
        .find(|group| group["object"] == object && group["property"] == property)
        .expect("expected keyed property")
}

#[test]
fn shape_dimension_tracks_route_to_parametric_geometry() {
    let input = document();
    let first = lower(&input);
    let second = lower(&input);

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let animation = &first.scene["artboard"]["animations"][0];
    let shape_name = "auth__dimension_2dstage__card__shape";
    let geometry_name = "auth__dimension_2dstage__card__geometry";

    let x = keyframe_group(animation, shape_name, "x");
    assert_eq!(x["frames"][0]["value"], 40.0);
    assert_eq!(x["frames"][1]["value"], 160.0);

    let width = keyframe_group(animation, geometry_name, "width");
    assert_eq!(width["frames"][0]["value"], 80.0);
    assert_eq!(width["frames"][1]["value"], 160.0);

    let height = keyframe_group(animation, geometry_name, "height");
    assert_eq!(height["frames"][0]["value"], 48.0);
    assert_eq!(height["frames"][1]["value"], 96.0);

    assert_builds(first.scene);
}

#[test]
fn dimension_schema_is_optional_and_expression_typed() {
    let schema = authoring_schema();
    let pose_target = &schema["$defs"]["PoseTargetSpec"];

    for property in ["width", "height"] {
        let property_schema = serde_json::to_string(&pose_target["properties"][property])
            .expect("serialize dimension property schema");
        assert!(property_schema.contains("#/$defs/ScalarExpr"));
    }

    let required = pose_target["required"]
        .as_array()
        .expect("pose-target required properties");
    assert!(required.iter().all(|property| property != "width"));
    assert!(required.iter().all(|property| property != "height"));
}

#[test]
fn dimension_diagnostics_preserve_authored_paths() {
    let mut wrong_unit = document();
    wrong_unit["motion"]["poses"][0]["targets"][0]["width"] = literal(80.0, "scalar");
    assert_diagnostic(
        &wrong_unit,
        "unit_mismatch",
        "$.motion.poses[0].targets[0].width",
    );

    let mut zero_height = document();
    zero_height["motion"]["poses"][0]["targets"][0]["height"] = literal(0.0, "px");
    assert_diagnostic(
        &zero_height,
        "invalid_dimension",
        "$.motion.poses[0].targets[0].height",
    );
}

#[test]
fn dimensions_reject_non_parametric_targets() {
    let mut input = document();
    input["visual"]["nodes"][0] = json!({
        "kind": "group",
        "id": "card",
        "children": []
    });

    assert_diagnostic(
        &input,
        "unsupported_motion_property",
        "$.motion.poses[0].targets[0].width",
    );
}
