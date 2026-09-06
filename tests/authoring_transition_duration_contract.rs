mod support;

use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};
use support::assert_builds;

fn document() -> Value {
    let mut input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/blend-meter.v0.json"
    ))
    .expect("authoring fixture");
    input["behavior"]["statecharts"][0] = json!({
        "id": "meter",
        "inputs": [{ "kind": "bool", "id": "enabled", "value": false }],
        "initial": "calm",
        "states": [
            { "id": "calm", "motion": "calm-track" },
            { "id": "surge", "motion": "surge-track" }
        ],
        "transitions": [{
            "id": "activate",
            "from": "calm",
            "to": "surge",
            "when": { "input": "enabled", "equals": true }
        }]
    });
    input
}

#[test]
fn transition_duration_lowers_in_milliseconds_from_a_parameter() {
    let mut input = document();
    input["parameters"] = json!({ "crossfade": { "value": 250.0, "unit": "scalar" } });
    input["behavior"]["statecharts"][0]["transitions"][0]["duration_ms"] =
        json!({ "kind": "parameter", "name": "crossfade" });

    let lowered = lower_authoring_json(&input.to_string()).expect("timed transition must lower");
    let transitions = &lowered.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"];
    assert_eq!(transitions[1]["duration"], 250);
    assert_eq!(transitions[1]["from"], 1);
    assert_eq!(transitions[1]["to"], 2);
    assert!(transitions[0].get("duration").is_none());
    assert_builds(lowered.scene);
}
