use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use serde_json::{Value, json};

fn document() -> Value {
    let mut input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/behavior-binding.v0.json"
    ))
    .expect("authoring fixture");
    input["behavior"]["statecharts"][0]["inputs"] = json!([
        { "kind": "bool", "id": "armed", "value": false },
        {
            "kind": "number", "id": "load",
            "value": { "kind": "literal", "value": 0.0, "unit": "scalar" }
        }
    ]);
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] = json!({
        "all": [
            { "input": "armed", "equals": true },
            { "binding": "gate-enabled", "equals": true },
            {
                "input": "load", "compare": "greater_or_equal",
                "value": { "kind": "literal", "value": 60.0, "unit": "scalar" }
            }
        ]
    });
    input
}

#[test]
fn all_guard_lowers_mixed_named_conditions_in_authored_order() {
    let input = document();
    let lowered = lower_authoring_json(&input.to_string()).expect("all guard must lower");
    assert_eq!(
        lowered,
        lower_authoring_json(&input.to_string()).expect("repeat lowering")
    );
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    let conditions = &machine["layers"][0]["transitions"][1]["conditions"];
    assert_eq!(conditions.as_array().expect("conditions").len(), 3);
    assert_eq!(conditions[0]["input"], machine["inputs"][1]["name"]);
    assert_eq!(conditions[1]["input"], machine["inputs"][0]["name"]);
    assert_eq!(conditions[2]["input"], machine["inputs"][2]["name"]);
    assert_eq!(conditions[0]["value"], true);
    assert_eq!(conditions[1]["value"], true);
    assert_eq!(conditions[2]["op"], ">=");
    assert_eq!(conditions[2]["value"], 60.0);
    assert!(machine["inputs"][0]["view_model_binding"].is_object());
    let scene: SceneSpec = serde_json::from_value(lowered.scene).expect("canonical scene");
    assert!(
        !compile_scene(&scene, None, 0)
            .expect("compiled guard")
            .is_empty()
    );
}
