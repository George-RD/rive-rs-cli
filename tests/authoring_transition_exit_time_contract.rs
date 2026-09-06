use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::objects::core::{property_keys, type_keys};
use rive_cli::validator::{InspectFilter, PropertyValueRead, parse_riv};
use serde_json::{Value, json};

fn document() -> Value {
    serde_json::from_str(include_str!(
        "../examples/authoring/pointer-statechart.v0.json"
    ))
    .expect("authoring fixture")
}

#[test]
fn authored_exit_time_reaches_the_encoded_transition_with_its_enable_flag() {
    let mut input = document();
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

#[test]
fn authored_blend_sources_reject_exit_time_at_the_authored_field() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["inputs"]
        .as_array_mut()
        .expect("inputs")
        .push(json!({
            "kind": "number", "id": "load",
            "value": { "kind": "literal", "value": 0, "unit": "scalar" }
        }));
    input["behavior"]["statecharts"][0]["states"][0] = json!({
        "id": "resting",
        "blend": { "input": "load", "stops": [
            { "motion": "rest-track", "value": { "kind": "literal", "value": 0, "unit": "scalar" } },
            { "motion": "active-track", "value": { "kind": "literal", "value": 1, "unit": "scalar" } }
        ] }
    });
    lower_authoring_json(&input.to_string()).expect("ungated blend source remains supported");
    input["behavior"]["statecharts"][0]["transitions"][0]["exit_time_ms"] =
        json!({ "kind": "literal", "value": 750, "unit": "scalar" });
    let error = lower_authoring_json(&input.to_string()).expect_err("blend gate must not be ignored");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unsupported_transition_exit_source"
            && diagnostic.path == "$.behavior.statecharts[0].transitions[0].exit_time_ms"
    }), "{error:?}");
}

#[test]
fn canonical_exit_time_rejects_non_animation_sources() {
    let baseline = lower_authoring_json(&document().to_string()).expect("baseline").scene;
    for source in ["entry", "exit", "any", "blend_state", "blend_state_direct", "blend_state_1d"] {
        let mut scene = baseline.clone();
        let layer = &mut scene["artboard"]["state_machines"][0]["layers"][0];
        layer["states"][1] = json!({ "type": source });
        let ungated: SceneSpec = serde_json::from_value(scene.clone()).expect("canonical scene");
        compile_scene(&ungated, None, 0).expect("ungated canonical source remains supported");
        scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["exit_time"] = json!(0);
        let gated: SceneSpec = serde_json::from_value(scene).expect("canonical gate");
        let error = compile_scene(&gated, None, 0).expect_err("non-animation gate must be rejected");
        assert!(error.to_string().contains("exit_time requires an animation source state"), "{source}: {error}");
    }
}
