use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

#[test]
fn exact_half_frame_is_rejected_when_one_ulp_is_half_a_frame() {
    const HALF_FRAME: f64 = 2_251_799_813_685_248.5;
    const NEXT_WHOLE_FRAME: f64 = 2_251_799_813_685_249.0;

    let input = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "motion-stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 200.0, "unit": "px" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "rectangle",
                    "id": "panel",
                    "width": literal(96.0, "px"),
                    "height": literal(56.0, "px"),
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
                            "target": "panel",
                            "transform": { "x": literal(0.0, "px") }
                        }
                    ]
                },
                {
                    "id": "settled",
                    "targets": [
                        {
                            "target": "panel",
                            "transform": { "x": literal(100.0, "px") }
                        }
                    ]
                }
            ],
            "tracks": [
                {
                    "id": "entrance",
                    "fps": 60,
                    "duration_frames": literal(NEXT_WHOLE_FRAME, "scalar"),
                    "loop_type": "oneshot",
                    "keyframes": [
                        {
                            "frame": literal(0.0, "scalar"),
                            "pose": "rest",
                            "interpolation": "linear"
                        },
                        {
                            "frame": literal(HALF_FRAME, "scalar"),
                            "pose": "settled",
                            "interpolation": "linear"
                        }
                    ]
                }
            ]
        },
        "behavior": {}
    });

    let error = lower_authoring_json(&input.to_string())
        .expect_err("an exactly authored half frame must remain fractional");
    assert!(
        error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "invalid_frame"
                && diagnostic.path == "$.motion.tracks[0].keyframes[1].frame"
        }),
        "expected invalid_frame at the authored keyframe path: {:#?}",
        error.diagnostics
    );
}
