mod support;

use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};
use support::assert_builds;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn document() -> Value {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "behavior-stage",
            "width": { "value": 240.0, "unit": "px" },
            "height": { "value": 160.0, "unit": "px" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "rectangle",
                    "id": "panel",
                    "width": literal(80.0, "px"),
                    "height": literal(48.0, "px"),
                    "fill": "#246BFD"
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
                                "x": literal(40.0, "px"),
                                "y": literal(80.0, "px")
                            }
                        }
                    ]
                },
                {
                    "id": "active",
                    "targets": [
                        {
                            "target": "panel",
                            "transform": {
                                "x": literal(160.0, "px"),
                                "y": literal(80.0, "px")
                            }
                        }
                    ]
                }
            ],
            "tracks": [
                {
                    "id": "rest-track",
                    "fps": 60,
                    "duration_frames": literal(1.0, "scalar"),
                    "keyframes": [
                        { "frame": literal(0.0, "scalar"), "pose": "rest" },
                        { "frame": literal(1.0, "scalar"), "pose": "rest" }
                    ]
                },
                {
                    "id": "active-track",
                    "fps": 60,
                    "duration_frames": literal(1.0, "scalar"),
                    "keyframes": [
                        { "frame": literal(0.0, "scalar"), "pose": "active" },
                        { "frame": literal(1.0, "scalar"), "pose": "active" }
                    ]
                }
            ]
        },
        "behavior": {
            "models": [
                {
                    "id": "gate-model",
                    "properties": [
                        { "kind": "bool", "id": "enabled", "value": true }
                    ]
                }
            ],
            "bindings": [
                {
                    "id": "gate-enabled",
                    "model": "gate-model",
                    "property": "enabled"
                }
            ],
            "statecharts": [
                {
                    "id": "gate",
                    "initial": "resting",
                    "states": [
                        { "id": "resting", "motion": "rest-track" },
                        { "id": "engaged", "motion": "active-track" }
                    ],
                    "transitions": [
                        {
                            "id": "engage",
                            "from": "resting",
                            "to": "engaged",
                            "when": { "binding": "gate-enabled", "equals": true }
                        }
                    ]
                }
            ]
        }
    })
}

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("typed behavior must lower")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string())
        .expect_err("invalid typed behavior must fail at the authored boundary");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code && diagnostic.path == path),
        "missing {code} at {path}; diagnostics: {:#?}",
        error.diagnostics
    );
}

#[test]
fn typed_bool_binding_lowers_named_states_and_transition_without_runtime_indices() {
    let input = document();
    let first = lower(&input);
    let second = lower(&input);

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let machine = &first.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["name"], "auth__behavior_2dstage__gate__state_machine");
    assert_eq!(machine["inputs"][0]["type"], "bool");
    assert_eq!(
        machine["inputs"][0]["name"],
        "auth__behavior_2dstage__gate_2denabled__input"
    );
    assert_eq!(machine["inputs"][0]["value"], true);

    let layer = &machine["layers"][0];
    assert_eq!(layer["states"][0]["type"], "entry");
    assert_eq!(layer["states"][1]["type"], "animation");
    assert_eq!(
        layer["states"][1]["animation"],
        "auth__behavior_2dstage__rest_2dtrack__animation"
    );
    assert_eq!(layer["states"][2]["type"], "animation");
    assert_eq!(
        layer["states"][2]["animation"],
        "auth__behavior_2dstage__active_2dtrack__animation"
    );

    assert_eq!(layer["transitions"][0]["from"], 0);
    assert_eq!(layer["transitions"][0]["to"], 1);
    assert_eq!(layer["transitions"][1]["from"], 1);
    assert_eq!(layer["transitions"][1]["to"], 2);
    assert_eq!(
        layer["transitions"][1]["conditions"][0]["input"],
        "auth__behavior_2dstage__gate_2denabled__input"
    );
    assert_eq!(layer["transitions"][1]["conditions"][0]["value"], true);

    let statechart_source = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "gate")
        .expect("statechart source map");
    assert_eq!(statechart_source.authored_path, "$.behavior.statecharts[0]");
    assert_eq!(
        statechart_source.runtime_names,
        vec!["auth__behavior_2dstage__gate__state_machine"]
    );
    assert_eq!(statechart_source.scene_paths, vec!["/artboard/state_machines/0"]);

    let transition_source = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "gate/engage")
        .expect("transition source map");
    assert_eq!(
        transition_source.authored_path,
        "$.behavior.statecharts[0].transitions[0]"
    );
    assert_eq!(
        transition_source.scene_paths,
        vec!["/artboard/state_machines/0/layers/0/transitions/1"]
    );

    assert_builds(first.scene);
}

#[test]
fn binding_to_unknown_model_property_reports_authored_path() {
    let mut input = document();
    input["behavior"]["bindings"][0]["property"] = json!("missing");

    assert_diagnostic(
        &input,
        "unknown_behavior_property",
        "$.behavior.bindings[0].property",
    );
}

#[test]
fn transition_to_unknown_named_state_reports_authored_path() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["transitions"][0]["to"] = json!("missing");

    assert_diagnostic(
        &input,
        "unknown_behavior_state",
        "$.behavior.statecharts[0].transitions[0].to",
    );
}

#[test]
fn state_with_unknown_motion_track_reports_authored_path() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][0]["motion"] = json!("missing");

    assert_diagnostic(
        &input,
        "unknown_behavior_motion",
        "$.behavior.statecharts[0].states[0].motion",
    );
}
