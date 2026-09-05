use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};

fn document() -> Value {
    serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json"))
        .expect("committed behavior example must parse")
}

#[test]
fn events_and_inputs_must_not_share_a_behavior_source_identity() {
    let mut input = document();
    lower_authoring_json(&input.to_string()).expect("uncorrupted example must lower");
    input["behavior"]["statecharts"][0]["events"] = json!([{ "id": "load" }]);

    let error = lower_authoring_json(&input.to_string())
        .expect_err("an event and input must not publish the same authored identity");
    assert_eq!(error.diagnostics.len(), 1);
    let diagnostic = &error.diagnostics[0];
    assert_eq!(diagnostic.code, "behavior_source_id_collision");
    assert_eq!(diagnostic.path, "$.behavior.statecharts[0].inputs[0].id");
    assert!(diagnostic.message.contains("meter/load"));
    assert!(diagnostic.message.contains("$.behavior.statecharts[0].events[0]"));
}
