use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::objects::core::type_keys;
use rive_cli::validator::{InspectFilter, parse_riv};
use serde_json::{Value, json};

fn document() -> Value {
    let mut input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/behavior-binding.v0.json"
    ))
    .expect("authoring fixture");
    input["behavior"]["models"][0]["properties"][0] = json!({
        "kind": "number", "id": "enabled",
        "value": { "kind": "literal", "value": 25.0, "unit": "scalar" }
    });
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] = json!({
        "binding": "gate-enabled", "compare": "greater_or_equal",
        "value": { "kind": "literal", "value": 60.0, "unit": "scalar" }
    });
    input
}

#[test]
fn numeric_view_model_binding_compiles_to_a_model_condition_not_an_input_condition() {
    let lowered = lower_authoring_json(&document().to_string())
        .expect("numeric view-model binding must lower");
    let model = &lowered.scene["artboard"]["children"][1];
    assert_eq!(model["children"][0]["type"], "view_model_property_number");
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"][0]["type"], "number");
    assert_eq!(machine["inputs"][0]["value"], 25.0);
    assert_eq!(
        machine["inputs"][0]["view_model_binding"]["property"],
        model["children"][0]["name"]
    );
    let condition = &machine["layers"][0]["transitions"][1]["conditions"][0];
    assert_eq!(condition["op"], ">=");
    assert_eq!(condition["value"], 60.0);
    let scene: SceneSpec = serde_json::from_value(lowered.scene).expect("canonical scene");
    let bytes = compile_scene(&scene, None, 0).expect("bound scene must compile");
    let parsed = parse_riv(&bytes, &InspectFilter::default()).expect("encoded scene");
    let types: Vec<_> = parsed.objects.iter().map(|object| object.type_key).collect();
    assert!(types.contains(&type_keys::TRANSITION_VIEW_MODEL_CONDITION));
    assert!(types.contains(&type_keys::BINDABLE_PROPERTY_NUMBER));
    assert!(types.contains(&type_keys::TRANSITION_VALUE_NUMBER_COMPARATOR));
    assert!(!types.contains(&type_keys::TRANSITION_NUMBER_CONDITION));
}
