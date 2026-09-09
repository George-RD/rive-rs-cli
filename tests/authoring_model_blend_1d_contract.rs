use rive_cli::authoring::lower_authoring_json;
use rive_cli::compile::compile_scene;
use rive_cli::objects::core::{property_keys, type_keys};
use rive_cli::validator::{InspectFilter, PropertyValueRead, parse_riv};
use serde_json::{Value, json};

fn document() -> Value {
    let mut input: Value =
        serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json"))
            .expect("authored fixture");
    input["behavior"]["models"] = json!([{
        "id": "model",
        "properties": [{
            "kind": "number", "id": "load",
            "value": {"kind": "literal", "value": 25.0, "unit": "scalar"}
        }]
    }]);
    input["behavior"]["bindings"] =
        json!([{"id": "model-load", "model": "model", "property": "load"}]);
    let blend = input["behavior"]["statecharts"][0]["states"][0]["blend"]
        .as_object_mut()
        .expect("blend");
    blend.remove("input");
    blend.insert("binding".to_string(), json!("model-load"));
    input
}

#[test]
fn model_bound_one_dimensional_blends_lower_without_transition_consumers() {
    let input = document().to_string();
    let first = lower_authoring_json(&input).expect("model-bound 1d blend must lower");
    let second = lower_authoring_json(&input).expect("repeat lowering");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);
    let machine = &first.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"][0]["value"], 25.0);
    assert!(machine["inputs"][0]["view_model_binding"].is_object());
    assert_eq!(machine["layers"][0]["states"][1]["type"], "blend_state_1d");
    assert_eq!(
        machine["layers"][0]["states"][1]["input"],
        machine["inputs"][0]["name"]
    );
    let binding = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "model-load")
        .expect("binding source map");
    assert_eq!(binding.scene_paths, ["/artboard/state_machines/0/inputs/0"]);
    let scene = serde_json::from_value(first.scene).expect("canonical scene");
    compile_scene(&scene, None, 0).expect("binary compilation");
}

#[test]
fn model_blends_encode_a_native_view_model_consumer_and_binding_context() {
    let lowered = lower_authoring_json(&document().to_string()).expect("bound blend");
    let scene = serde_json::from_value(lowered.scene).expect("canonical scene");
    let bytes = compile_scene(&scene, None, 0).expect("binary compilation");
    let parsed = parse_riv(&bytes, &InspectFilter::default()).expect("encoded scene");
    let state_index = parsed
        .objects
        .iter()
        .position(|object| object.type_key == type_keys::BLEND_STATE_1D_VIEW_MODEL)
        .expect("model-bound blend must use the native model state");
    assert_eq!(
        parsed.objects[state_index - 2].type_key,
        type_keys::BINDABLE_PROPERTY_NUMBER
    );
    assert_eq!(
        parsed.objects[state_index - 1].type_key,
        type_keys::DATA_BIND_CONTEXT
    );
    assert_eq!(
        parsed.objects[state_index + 1].type_key,
        type_keys::BLEND_ANIMATION_1D
    );
    assert!(
        parsed.objects[state_index - 2]
            .properties
            .iter()
            .any(
                |field| field.key == property_keys::BINDABLE_PROPERTY_NUMBER_VALUE
                    && field.value == PropertyValueRead::Float(25.0)
            )
    );
    assert!(
        parsed.objects[state_index - 1]
            .properties
            .iter()
            .any(
                |field| field.key == property_keys::DATA_BIND_CONTEXT_SOURCE_PATH_IDS
                    && field.value == PropertyValueRead::Bytes(vec![0, 0])
            )
    );
    assert!(
        !parsed
            .objects
            .iter()
            .any(|object| object.type_key == type_keys::BLEND_STATE_1D_INPUT)
    );
}
