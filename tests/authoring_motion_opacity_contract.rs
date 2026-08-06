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
            "id": "opacity-stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 200.0, "unit": "px" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "rectangle",
                    "id": "card",
                    "width": literal(120.0, "px"),
                    "height": literal(72.0, "px"),
                    "fill": "#172554"
                }
            ]
        },
        "motion": {
            "poses": [
                {
                    "id": "hidden",
                    "targets": [
                        {
                            "target": "card",
                            "opacity": literal(0.0, "scalar")
                        }
                    ]
                },
                {
                    "id": "shown",
                    "targets": [
                        {
                            "target": "card",
                            "opacity": literal(1.0, "scalar")
                        }
                    ]
                }
            ],
            "tracks": [
                {
                    "id": "fade-in",
                    "fps": 60,
                    "duration_frames": literal(24.0, "scalar"),
                    "keyframes": [
                        {
                            "frame": literal(0.0, "scalar"),
                            "pose": "hidden"
                        },
                        {
                            "frame": literal(24.0, "scalar"),
                            "pose": "shown"
                        }
                    ]
                }
            ]
        },
        "behavior": {}
    })
}

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("opacity motion must lower")
}

fn has_diagnostic(error: &rive_cli::authoring::AuthoringError, code: &str, path: &str) -> bool {
    error
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == code && diagnostic.path == path)
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string())
        .expect_err("invalid opacity motion must fail at the authored boundary");
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
fn opacity_pose_tracks_lower_deterministically_and_build() {
    let input = document();
    let first = lower(&input);
    let second = lower(&input);

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let animation = &first.scene["artboard"]["animations"][0];
    let opacity = keyframe_group(animation, "auth__opacity_2dstage__card__shape", "opacity");
    assert_eq!(opacity["frames"][0]["value"], 0.0);
    assert_eq!(opacity["frames"][1]["value"], 1.0);
    assert_eq!(opacity["frames"][0]["interpolation"], "linear");

    assert_builds(first.scene);
}

#[test]
fn opacity_schema_is_optional_and_expression_typed() {
    let schema = authoring_schema();
    let pose_target = &schema["$defs"]["PoseTargetSpec"];
    let opacity = serde_json::to_string(&pose_target["properties"]["opacity"])
        .expect("serialize opacity property schema");
    assert!(opacity.contains("#/$defs/ScalarExpr"));

    let required = pose_target["required"]
        .as_array()
        .expect("pose-target required properties");
    assert!(required.iter().all(|property| property != "transform"));
}

#[test]
fn opacity_diagnostics_preserve_authored_paths() {
    let path = "$.motion.poses[0].targets[0].opacity";

    let mut wrong_unit = document();
    wrong_unit["motion"]["poses"][0]["targets"][0]["opacity"] = literal(0.5, "px");
    assert_diagnostic(&wrong_unit, "unit_mismatch", path);

    let mut out_of_range = document();
    out_of_range["motion"]["poses"][0]["targets"][0]["opacity"] = literal(1.01, "scalar");
    assert_diagnostic(&out_of_range, "invalid_ratio", path);

    let mut mismatched_pose = document();
    let target = mismatched_pose["motion"]["poses"][1]["targets"][0]
        .as_object_mut()
        .expect("shown target");
    target.remove("opacity");
    target.insert("transform".to_string(), json!({ "x": literal(0.0, "px") }));
    assert_diagnostic(
        &mismatched_pose,
        "pose_shape_mismatch",
        "$.motion.tracks[0].keyframes[1].pose",
    );
}

#[test]
fn empty_pose_target_diagnostic_uses_existing_authored_path() {
    let mut input = document();
    input["motion"]["poses"][0]["targets"][0]
        .as_object_mut()
        .expect("hidden target")
        .remove("opacity");

    assert_diagnostic(&input, "empty_pose_target", "$.motion.poses[0].targets[0]");
}

#[test]
fn opacity_counts_toward_motion_expansion_budget() {
    const TARGET_COUNT: usize = 501;
    const KEYFRAME_COUNT: usize = 20;

    let mut input = document();
    input["visual"]["nodes"] = Value::Array(
        (0..TARGET_COUNT)
            .map(|index| {
                json!({
                    "kind": "rectangle",
                    "id": format!("card-{index}"),
                    "width": literal(8.0, "px"),
                    "height": literal(8.0, "px"),
                    "fill": "#172554"
                })
            })
            .collect(),
    );

    let targets = |opacity: f64| {
        Value::Array(
            (0..TARGET_COUNT)
                .map(|index| {
                    json!({
                        "target": format!("card-{index}"),
                        "opacity": literal(opacity, "scalar")
                    })
                })
                .collect(),
        )
    };
    input["motion"]["poses"] = json!([
        { "id": "hidden", "targets": targets(0.0) },
        { "id": "shown", "targets": targets(1.0) }
    ]);
    input["motion"]["tracks"][0]["duration_frames"] =
        literal((KEYFRAME_COUNT - 1) as f64, "scalar");
    input["motion"]["tracks"][0]["keyframes"] = Value::Array(
        (0..KEYFRAME_COUNT)
            .map(|index| {
                json!({
                    "frame": literal(index as f64, "scalar"),
                    "pose": if index % 2 == 0 { "hidden" } else { "shown" }
                })
            })
            .collect(),
    );

    assert_diagnostic(
        &input,
        "motion_keyframe_expansion_limit",
        "$.motion.tracks[0].keyframes",
    );
}
