use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

#[test]
fn aggregate_motion_expansion_reports_only_the_first_limit_crossing() {
    const TARGET_COUNT: usize = 50;
    const LAST_FRAME: u64 = 20;

    let targets = (0..TARGET_COUNT)
        .map(|index| {
            json!({
                "target": format!("target-{index}"),
                "transform": {
                    "x": literal(index as f64, "px"),
                    "y": literal(index as f64, "px"),
                    "rotation": literal(0.0, "degrees"),
                    "scale_x": literal(1.0, "scalar"),
                    "scale_y": literal(1.0, "scalar")
                }
            })
        })
        .collect::<Vec<_>>();
    let track = |id: &str| {
        let keyframes = (0..=LAST_FRAME)
            .map(|frame| {
                json!({
                    "frame": literal(frame as f64, "scalar"),
                    "pose": "expanded-pose",
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
    let input = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "motion-stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 200.0, "unit": "px" }
        },
        "visual": { "nodes": [] },
        "motion": {
            "poses": [
                {
                    "id": "expanded-pose",
                    "targets": targets
                }
            ],
            "tracks": [
                track("first-track"),
                track("second-track"),
                track("third-track")
            ]
        },
        "behavior": {}
    });

    let error = lower_authoring_json(&input.to_string())
        .expect_err("aggregate motion expansion must fail at its first limit crossing");
    let expansion_diagnostics = error
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "motion_keyframe_expansion_limit")
        .collect::<Vec<_>>();

    assert_eq!(
        expansion_diagnostics.len(),
        1,
        "the aggregate limit should be reported once: {:#?}",
        error.diagnostics
    );
    assert_eq!(
        expansion_diagnostics[0].path,
        "$.motion.tracks[1].keyframes"
    );
}
