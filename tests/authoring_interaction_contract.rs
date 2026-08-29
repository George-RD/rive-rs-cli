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
            "id": "interaction-stage",
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
            "statecharts": [
                {
                    "id": "gate",
                    "inputs": [
                        { "kind": "bool", "id": "pressed", "value": false }
                    ],
                    "events": [
                        { "id": "completed" }
                    ],
                    "listeners": [
                        {
                            "id": "press-panel",
                            "target": "panel",
                            "listener_type": "down",
                            "actions": [
                                { "kind": "bool_change", "input": "pressed", "value": true }
                            ]
                        },
                        {
                            "id": "completion-event",
                            "target": "completed",
                            "listener_type": "event",
                            "actions": [
                                { "kind": "bool_change", "input": "pressed", "value": true }
                            ]
                        }
                    ],
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
                            "when": { "input": "pressed", "equals": true }
                        }
                    ]
                }
            ]
        }
    })
}

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("typed interaction must lower")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string())
        .expect_err("invalid typed interaction must fail at the authored boundary");
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
fn typed_input_event_and_listeners_lower_without_authored_runtime_indices() {
    let input = document();
    let first = lower(&input);
    let second = lower(&input);

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let machine = &first.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"][0]["type"], "bool");
    assert_eq!(
        machine["inputs"][0]["name"],
        "auth__interaction_2dstage__gate__pressed__input"
    );
    assert_eq!(machine["inputs"][0]["value"], false);

    let panel_runtime = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "panel")
        .and_then(|entry| entry.runtime_names.first())
        .expect("panel runtime name");
    assert_eq!(machine["listeners"][0]["target"], panel_runtime.as_str());
    assert_eq!(machine["listeners"][0]["listener_type"], "down");
    assert_eq!(machine["listeners"][0]["actions"][0]["type"], "bool_change");
    assert_eq!(
        machine["listeners"][0]["actions"][0]["input"],
        "auth__interaction_2dstage__gate__pressed__input"
    );
    assert_eq!(machine["listeners"][0]["actions"][0]["value"], true);

    let event_source = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "gate/completed")
        .expect("event source map");
    assert_eq!(
        event_source.authored_path,
        "$.behavior.statecharts[0].events[0]"
    );
    assert_eq!(event_source.runtime_names.len(), 1);
    assert_eq!(
        machine["listeners"][1]["target"],
        event_source.runtime_names[0]
    );
    assert_eq!(machine["listeners"][1]["listener_type"], "event");

    let transition = &machine["layers"][0]["transitions"][1];
    assert_eq!(
        transition["conditions"][0]["input"],
        "auth__interaction_2dstage__gate__pressed__input"
    );
    assert_eq!(transition["conditions"][0]["value"], true);

    assert_builds(first.scene);
}

#[test]
fn transition_with_unknown_typed_input_reports_authored_path() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["transitions"][0]["when"]["input"] = json!("missing");

    assert_diagnostic(
        &input,
        "unknown_behavior_input",
        "$.behavior.statecharts[0].transitions[0].when.input",
    );
}

#[test]
fn pointer_listener_with_unknown_target_reports_authored_path() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["listeners"][0]["target"] = json!("missing");

    assert_diagnostic(
        &input,
        "unknown_listener_target",
        "$.behavior.statecharts[0].listeners[0].target",
    );
}

#[test]
fn event_listener_with_unknown_event_reports_authored_path() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["listeners"][1]["target"] = json!("missing");

    assert_diagnostic(
        &input,
        "unknown_behavior_event",
        "$.behavior.statecharts[0].listeners[1].target",
    );
}

#[test]
fn listener_action_with_unknown_input_reports_authored_path() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["listeners"][0]["actions"][0]["input"] = json!("missing");

    assert_diagnostic(
        &input,
        "unknown_behavior_input",
        "$.behavior.statecharts[0].listeners[0].actions[0].input",
    );
}
