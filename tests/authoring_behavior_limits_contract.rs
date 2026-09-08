use rive_cli::authoring::{AuthoringSpec, lower_authoring};
use serde_json::{Value, json};

fn document() -> Value {
    serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json"))
        .expect("committed behavior example must parse")
}

#[test]
fn typed_behavior_models_must_respect_the_published_collection_limit() {
    let mut input = document();
    input["behavior"]["models"] = json!((0..1001)
        .map(|index| json!({ "id": format!("model-{index}"), "properties": [] }))
        .collect::<Vec<_>>());
    let spec: AuthoringSpec = serde_json::from_value(input).expect("typed document must parse");

    let result = lower_authoring(&spec);
    assert!(result.is_err(), "1001 behavior models must not lower");
    let error = result.expect_err("collection exceeds the published limit");
    assert_eq!(error.diagnostics.len(), 1);
    assert_eq!(error.diagnostics[0].code, "invalid_behavior_collection_count");
    assert_eq!(error.diagnostics[0].path, "$.behavior.models");
}
