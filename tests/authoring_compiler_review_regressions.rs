use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn document() -> Value {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "review-stage",
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
                    "id": "rest",
                    "targets": [
                        {
                            "target": "card",
                            "transform": { "x": literal(16.0, "px") }
                        }
                    ]
                },
                {
                    "id": "moved",
                    "targets": [
                        {
                            "target": "card",
                            "transform": { "x": literal(160.0, "px") }
                        }
                    ]
                }
            ],
            "tracks": [
                {
                    "id": "entrance",
                    "fps": 60,
                    "duration_frames": literal(30.0, "scalar"),
                    "keyframes": [
                        { "frame": literal(0.0, "scalar"), "pose": "rest" },
                        { "frame": literal(30.0, "scalar"), "pose": "moved" }
                    ]
                }
            ],
            "raw_animations": [
                {
                    "id": "raw-tail",
                    "value": {
                        "name": "raw_tail",
                        "fps": 60,
                        "duration": 1,
                        "keyframes": []
                    }
                }
            ]
        },
        "behavior": {}
    })
}

#[test]
fn typed_and_raw_animation_ids_share_one_authored_namespace() {
    let mut input = document();
    input["motion"]["raw_animations"][0]["id"] = json!("entrance");

    let error = lower_authoring_json(&input.to_string())
        .expect_err("typed and raw animation IDs must not collide");
    assert_eq!(error.diagnostics.len(), 1);
    assert_eq!(error.diagnostics[0].code, "duplicate_id");
    assert_eq!(error.diagnostics[0].path, "$.motion.raw_animations[0].id");
    assert_eq!(
        error.diagnostics[0].message,
        "raw fragment id 'entrance' is duplicated"
    );
}

#[test]
fn mixed_animation_id_collision_stays_after_motion_preflight() {
    let mut input = document();
    input["motion"]["tracks"][0]["fps"] = json!(0);
    input["motion"]["raw_animations"][0]["id"] = json!("entrance");

    let error = lower_authoring_json(&input.to_string())
        .expect_err("motion preflight diagnostics must retain their prior precedence");
    assert_eq!(error.diagnostics.len(), 1);
    assert_eq!(error.diagnostics[0].code, "invalid_motion_fps");
    assert_eq!(error.diagnostics[0].path, "$.motion.tracks[0].fps");
}

#[test]
fn no_track_raw_fragment_error_precedes_pose_target_error() {
    let mut input = document();
    input["motion"]["tracks"] = json!([]);
    input["motion"]["poses"][0]["targets"][0]["target"] = json!("missing");
    input["motion"]["raw_animations"][0]["value"] = json!(7);

    let error = lower_authoring_json(&input.to_string())
        .expect_err("raw lowering must retain its no-track diagnostic precedence");
    assert_eq!(error.diagnostics.len(), 1);
    assert_eq!(error.diagnostics[0].code, "invalid_raw_scene_fragment");
    assert_eq!(
        error.diagnostics[0].path,
        "$.motion.raw_animations[0].value"
    );
}
