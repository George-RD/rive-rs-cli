use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::objects::core::{property_keys, type_keys};
use rive_cli::validator::{InspectFilter, PropertyValueRead, parse_riv};
use serde_json::{Value, json};

#[test]
fn authored_exit_time_reaches_the_encoded_transition_with_its_enable_flag() {
    let mut input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/pointer-statechart.v0.json"
    ))
    .expect("authoring fixture");
    input["parameters"] = json!({ "gate": { "value": 750, "unit": "scalar" } });
    input["behavior"]["statecharts"][0]["transitions"][0]["exit_time_ms"] =
        json!({ "kind": "parameter", "name": "gate" });
    let lowered = lower_authoring_json(&input.to_string()).expect("exit-time gate must lower");
    let transition = &lowered.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1];
    assert_eq!(transition["exit_time"], 750);
    assert_eq!(transition["from"], 1);
    assert_eq!(transition["to"], 2);
    let scene: SceneSpec = serde_json::from_value(lowered.scene).expect("canonical scene");
    let bytes = compile_scene(&scene, None, 0).expect("gate must compile");
    let parsed = parse_riv(&bytes, &InspectFilter::default()).expect("encoded scene");
    let encoded = parsed
        .objects
        .iter()
        .filter(|object| object.type_key == type_keys::STATE_TRANSITION)
        .nth(1)
        .expect("authored transition after entry transition");
    let property = |key| {
        encoded
            .properties
            .iter()
            .find(|property| property.key == key)
            .map(|property| &property.value)
    };
    assert_eq!(
        property(property_keys::STATE_TRANSITION_EXIT_TIME),
        Some(&PropertyValueRead::UInt(750))
    );
    const ENABLE_EXIT_TIME: u64 = 1 << 2;
    assert_eq!(
        property(property_keys::STATE_TRANSITION_FLAGS),
        Some(&PropertyValueRead::UInt(ENABLE_EXIT_TIME))
    );
}
