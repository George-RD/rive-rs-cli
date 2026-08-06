use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn document(duration: f64, final_frame: f64) -> Value {
    json!({
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
                    "duration_frames": literal(duration, "scalar"),
                    "loop_type": "oneshot",
                    "keyframes": [
                        {
                            "frame": literal(0.0, "scalar"),
                            "pose": "rest",
                            "interpolation": "linear"
                        },
                        {
                            "frame": literal(final_frame, "scalar"),
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

fn assert_diagnostic(input: Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string())
        .expect_err("ambiguous motion frame precision must be rejected");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code && diagnostic.path == path),
        "expected {code} at {path}: {:#?}",
        error.diagnostics
    );
}

#[test]
fn exact_half_frame_is_rejected_when_one_ulp_is_half_a_frame() {
    const HALF_FRAME: f64 = 2_251_799_813_685_248.5;
    const NEXT_WHOLE_FRAME: f64 = 2_251_799_813_685_249.0;

    assert_diagnostic(
        document(NEXT_WHOLE_FRAME, HALF_FRAME),
        "invalid_frame",
        "$.motion.tracks[0].keyframes[1].frame",
    );
}

#[test]
fn magnitudes_that_cannot_distinguish_half_frames_are_rejected() {
    const FIRST_WHOLE_FRAME_ULP: f64 = 4_503_599_627_370_496.0;

    assert_diagnostic(
        document(FIRST_WHOLE_FRAME_ULP, 36.0),
        "invalid_duration_frames",
        "$.motion.tracks[0].duration_frames",
    );
}
