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

fn document() -> Value {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "motion-stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 200.0, "unit": "px" }
        },
        "parameters": {
            "finish_frame": { "value": 36.0, "unit": "scalar" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "group",
                    "id": "panel",
                    "children": [
                        {
                            "kind": "rectangle",
                            "id": "surface",
                            "width": literal(96.0, "px"),
                            "height": literal(56.0, "px"),
                            "fill": "#172554"
                        }
                    ]
                },
                {
                    "kind": "ellipse",
                    "id": "orb",
                    "width": literal(32.0, "px"),
                    "height": literal(32.0, "px"),
                    "fill": "#22D3EE"
                }
            ]
        },
        "motion": {
            "poses": [
                {
                    "id": "rest",
                    "targets": [
                        {
                            "target": "panel",
                            "transform": {
                                "x": literal(20.0, "px"),
                                "y": literal(40.0, "px"),
                                "rotation": literal(-8.0, "degrees")
                            }
                        },
                        {
                            "target": "orb",
                            "transform": {
                                "x": literal(40.0, "px"),
                                "y": literal(120.0, "px"),
                                "scale_x": literal(0.5, "scalar"),
                                "scale_y": literal(0.5, "scalar")
                            }
                        }
                    ]
                },
                {
                    "id": "settled",
                    "targets": [
                        {
                            "target": "panel",
                            "transform": {
                                "x": literal(112.0, "px"),
                                "y": literal(40.0, "px"),
                                "rotation": literal(0.0, "degrees")
                            }
                        },
                        {
                            "target": "orb",
                            "transform": {
                                "x": literal(240.0, "px"),
                                "y": literal(120.0, "px"),
                                "scale_x": literal(1.0, "scalar"),
                                "scale_y": literal(1.0, "scalar")
                            }
                        }
                    ]
                }
            ],
            "tracks": [
                {
                    "id": "entrance",
                    "fps": 60,
                    "duration_frames": parameter("finish_frame"),
                    "loop_type": "oneshot",
                    "keyframes": [
                        {
                            "frame": literal(0.0, "scalar"),
                            "pose": "rest",
                            "interpolation": "linear"
                        },
                        {
                            "frame": parameter("finish_frame"),
                            "pose": "settled",
                            "interpolation": "linear"
                        }
                    ]
                }
            ]
        },
        "behavior": {}
    })
}

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("motion authoring must lower")
}

fn has_diagnostic(error: &rive_cli::authoring::AuthoringError, code: &str, path: &str) -> bool {
    error
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == code && diagnostic.path == path)
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string())
        .expect_err("invalid typed motion must fail at the authored boundary");
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
fn named_poses_lower_to_deterministic_canonical_tracks() {
    let input = document();
    let first = lower(&input);
    let second = lower(&input);

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let animations = first.scene["artboard"]["animations"]
        .as_array()
        .expect("typed animation list");
    assert_eq!(animations.len(), 1);
    let animation = &animations[0];
    assert_eq!(
        animation["name"],
        "auth__motion_2dstage__entrance__animation"
    );
    assert_eq!(animation["fps"], 60);
    assert_eq!(animation["duration"], 36);
    assert_eq!(animation["loop_type"], "oneshot");

    let panel_x = keyframe_group(animation, "auth__motion_2dstage__panel__group", "x");
    assert_eq!(panel_x["frames"][0]["frame"], 0);
    assert_eq!(panel_x["frames"][0]["value"], 20.0);
    assert_eq!(panel_x["frames"][1]["frame"], 36);
    assert_eq!(panel_x["frames"][1]["value"], 112.0);
    assert_eq!(panel_x["frames"][0]["interpolation"], "linear");

    let orb_scale = keyframe_group(animation, "auth__motion_2dstage__orb__shape", "scale_x");
    assert_eq!(orb_scale["frames"][0]["value"], 0.5);
    assert_eq!(orb_scale["frames"][1]["value"], 1.0);

    let source = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "entrance")
        .expect("typed animation source map");
    assert_eq!(source.authored_path, "$.motion.tracks[0]");
    assert_eq!(
        source.runtime_names,
        vec!["auth__motion_2dstage__entrance__animation"]
    );
    assert_eq!(source.scene_paths, vec!["/artboard/animations/0"]);

    assert_builds(first.scene);
}

#[test]
fn typed_tracks_keep_raw_animation_source_paths_at_their_real_offset() {
    let mut input = document();
    input["motion"]["raw_animations"] = json!([
        {
            "id": "raw-tail",
            "value": {
                "name": "raw_tail",
                "fps": 60,
                "duration": 1,
                "keyframes": []
            }
        }
    ]);

    let lowered = lower(&input);
    assert_eq!(
        lowered.scene["artboard"]["animations"][1]["name"],
        "raw_tail"
    );
    let raw = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "raw-tail")
        .expect("raw animation source map");
    assert_eq!(raw.scene_paths, vec!["/artboard/animations/1"]);
    assert_builds(lowered.scene);
}

#[test]
fn raw_state_machine_can_reference_typed_track_runtime_name() {
    let mut input = document();
    input["behavior"]["raw_state_machines"] = json!([
        {
            "id": "entrance-machine",
            "value": {
                "name": "entrance_machine",
                "layers": [
                    {
                        "states": [
                            { "type": "entry" },
                            { "type": "exit" },
                            {
                                "type": "animation",
                                "animation": "auth__motion_2dstage__entrance__animation"
                            }
                        ]
                    }
                ]
            }
        }
    ]);

    let lowered = lower(&input);
    assert_eq!(
        lowered.scene["artboard"]["state_machines"][0]["name"],
        "entrance_machine"
    );
    let source = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "entrance-machine")
        .expect("raw state-machine source map");
    assert_eq!(source.authored_path, "$.behavior.raw_state_machines[0]");
    assert_eq!(source.scene_paths, vec!["/artboard/state_machines/0"]);
    assert_builds(lowered.scene);
}

#[test]
fn motion_diagnostics_point_to_authored_pose_and_track_paths() {
    let mut cases = Vec::new();

    let mut duplicate_pose = document();
    duplicate_pose["motion"]["poses"][1]["id"] = json!("rest");
    cases.push((duplicate_pose, "duplicate_pose", "$.motion.poses[1].id"));

    let mut unknown_pose = document();
    unknown_pose["motion"]["tracks"][0]["keyframes"][1]["pose"] = json!("missing");
    cases.push((
        unknown_pose,
        "unknown_pose",
        "$.motion.tracks[0].keyframes[1].pose",
    ));

    let mut unknown_target = document();
    unknown_target["motion"]["tracks"] = json!([]);
    unknown_target["motion"]["poses"][0]["targets"][0]["target"] = json!("missing");
    cases.push((
        unknown_target,
        "unknown_motion_target",
        "$.motion.poses[0].targets[0].target",
    ));

    let mut mismatched_pose = document();
    mismatched_pose["motion"]["poses"][1]["targets"][1]["transform"]
        .as_object_mut()
        .expect("pose transform")
        .remove("scale_y");
    cases.push((
        mismatched_pose,
        "pose_shape_mismatch",
        "$.motion.tracks[0].keyframes[1].pose",
    ));

    let mut fractional_frame = document();
    fractional_frame["motion"]["tracks"][0]["keyframes"][1]["frame"] = literal(12.5, "scalar");
    cases.push((
        fractional_frame,
        "invalid_frame",
        "$.motion.tracks[0].keyframes[1].frame",
    ));

    for (input, expected_code, expected_path) in cases {
        assert_diagnostic(&input, expected_code, expected_path);
    }
}

#[test]
fn near_integer_frame_expressions_round_deterministically() {
    let mut input = document();
    input["motion"]["tracks"][0]["keyframes"][1]["frame"] = json!({
        "kind": "multiply",
        "value": {
            "kind": "add",
            "left": literal(0.1, "scalar"),
            "right": literal(0.2, "scalar")
        },
        "factor": 10.0
    });

    let lowered = lower(&input);
    let animation = &lowered.scene["artboard"]["animations"][0];
    let panel_x = keyframe_group(animation, "auth__motion_2dstage__panel__group", "x");
    assert_eq!(panel_x["frames"][1]["frame"], 3);
    assert_builds(lowered.scene);
}

#[test]
fn large_near_integer_frame_expressions_accept_one_ulp_roundoff() {
    let mut input = document();
    input["motion"]["tracks"][0]["duration_frames"] = literal(300_000_000.0, "scalar");
    input["motion"]["tracks"][0]["keyframes"][1]["frame"] = json!({
        "kind": "multiply",
        "value": {
            "kind": "add",
            "left": literal(0.1, "scalar"),
            "right": literal(0.2, "scalar")
        },
        "factor": 1_000_000_000.0
    });

    let lowered = lower(&input);
    let animation = &lowered.scene["artboard"]["animations"][0];
    let panel_x = keyframe_group(animation, "auth__motion_2dstage__panel__group", "x");
    assert_eq!(animation["duration"], 300_000_000);
    assert_eq!(panel_x["frames"][1]["frame"], 300_000_000);
    assert_builds(lowered.scene);
}

#[test]
fn exact_half_frame_is_rejected_when_one_ulp_is_half_a_frame() {
    const HALF_FRAME: f64 = 2_251_799_813_685_248.5;
    const NEXT_WHOLE_FRAME: f64 = 2_251_799_813_685_249.0;

    let mut input = document();
    input["motion"]["tracks"][0]["duration_frames"] = literal(NEXT_WHOLE_FRAME, "scalar");
    input["motion"]["tracks"][0]["keyframes"][1]["frame"] = literal(HALF_FRAME, "scalar");

    assert_diagnostic(
        &input,
        "invalid_frame",
        "$.motion.tracks[0].keyframes[1].frame",
    );
}

#[test]
fn magnitudes_that_cannot_distinguish_half_frames_are_rejected() {
    const FIRST_WHOLE_FRAME_ULP: f64 = 4_503_599_627_370_496.0;

    let mut input = document();
    input["motion"]["tracks"][0]["duration_frames"] = literal(FIRST_WHOLE_FRAME_ULP, "scalar");

    assert_diagnostic(
        &input,
        "invalid_duration_frames",
        "$.motion.tracks[0].duration_frames",
    );
}

#[test]
fn aggregate_motion_keyframe_expansion_is_bounded() {
    const TARGET_COUNT: usize = 50;
    const LAST_FRAME: u64 = 20;

    let mut input = document();
    input["visual"]["nodes"] = Value::Array(
        (0..TARGET_COUNT)
            .map(|index| {
                json!({
                    "kind": "rectangle",
                    "id": format!("target-{index}"),
                    "width": literal(8.0, "px"),
                    "height": literal(8.0, "px"),
                    "fill": "#172554"
                })
            })
            .collect(),
    );

    let pose = |id: &str, offset: f64| {
        let targets = (0..TARGET_COUNT)
            .map(|index| {
                json!({
                    "target": format!("target-{index}"),
                    "transform": {
                        "x": literal(index as f64 + offset, "px"),
                        "y": literal(index as f64, "px"),
                        "rotation": literal(offset, "degrees"),
                        "scale_x": literal(1.0 + offset / 100.0, "scalar"),
                        "scale_y": literal(1.0 + offset / 100.0, "scalar")
                    }
                })
            })
            .collect::<Vec<_>>();
        json!({ "id": id, "targets": targets })
    };
    input["motion"]["poses"] =
        Value::Array(vec![pose("first-pose", 0.0), pose("second-pose", 1.0)]);

    let track = |id: &str| {
        let keyframes = (0..=LAST_FRAME)
            .map(|frame| {
                json!({
                    "frame": literal(frame as f64, "scalar"),
                    "pose": if frame % 2 == 0 { "first-pose" } else { "second-pose" },
                    "interpolation": "linear"
                })
            })
            .collect::<Vec<_>>();
        json!({
            "id": id,
            "fps": 60,
            "duration_frames": literal(LAST_FRAME as f64, "scalar"),
            "loop_type": "oneshot",
            "keyframes": keyframes
        })
    };
    input["motion"]["tracks"] = Value::Array(vec![track("first-track"), track("second-track")]);

    let error = lower_authoring_json(&input.to_string())
        .expect_err("aggregate motion expansion must be rejected before lowering");
    assert!(
        has_diagnostic(
            &error,
            "motion_keyframe_expansion_limit",
            "$.motion.tracks[1].keyframes"
        ),
        "missing aggregate motion expansion diagnostic: {:#?}",
        error.diagnostics
    );
}

#[test]
fn schema_exposes_typed_pose_tracks_and_bounded_motion_enums() {
    let schema = authoring_schema();
    let motion = &schema["$defs"]["MotionSection"];
    assert_eq!(
        motion["properties"]["poses"]["items"]["$ref"],
        "#/$defs/PoseSpec"
    );
    assert_eq!(
        motion["properties"]["tracks"]["items"]["$ref"],
        "#/$defs/MotionTrackSpec"
    );
    assert_eq!(
        schema["$defs"]["PoseTargetSpec"]["properties"]["transform"]["$ref"],
        "#/$defs/TransformSpec"
    );
    assert_eq!(
        schema["$defs"]["MotionTrackSpec"]["properties"]["duration_frames"]["$ref"],
        "#/$defs/ScalarExpr"
    );
    assert_eq!(
        schema["$defs"]["MotionInterpolation"]["enum"],
        json!(["hold", "linear"])
    );
    assert_eq!(
        schema["$defs"]["MotionLoop"]["enum"],
        json!(["oneshot", "loop", "pingpong"])
    );
}
