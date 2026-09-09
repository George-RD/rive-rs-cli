use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use serde_json::{Value, json};

#[test]
fn explicit_always_guard_compiles_a_timed_transition_without_inputs() {
    let mut input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/pointer-statechart.v0.json"
    ))
    .expect("authoring fixture");
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["inputs"] = json!([]);
    chart["events"] = json!([]);
    chart["listeners"] = json!([]);
    chart["transitions"][0]["when"] = json!("always");
    chart["transitions"][0]["exit_time_ms"] =
        json!({ "kind": "literal", "value": 750, "unit": "scalar" });
    let lowered = lower_authoring_json(&input.to_string())
        .expect("an explicit always guard must lower without dummy inputs");
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"], json!([]));
    assert_eq!(
        machine["layers"][0]["transitions"][1],
        json!({ "from": 1, "to": 2, "conditions": [], "exit_time": 750 })
    );
    let scene: SceneSpec = serde_json::from_value(lowered.scene).expect("canonical scene");
    assert!(
        !compile_scene(&scene, None, 0)
            .expect("canonical compile")
            .is_empty()
    );
}
