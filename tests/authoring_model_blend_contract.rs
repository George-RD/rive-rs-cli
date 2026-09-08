use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::objects::core::{property_keys, type_keys};
use rive_cli::validator::{InspectFilter, PropertyValueRead, parse_riv};
use serde_json::{Value, json};

fn document() -> Value {
    let mut input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/direct-blend-panel.v0.json"
    ))
    .expect("authored fixture");
    input["behavior"]["models"] = json!([{
        "id": "weights",
        "properties": [{
            "kind": "number", "id": "left",
            "value": { "kind": "literal", "value": 25.0, "unit": "scalar" }
        }]
    }]);
    input["behavior"]["bindings"] = json!([
        { "id": "left-model", "model": "weights", "property": "left" }
    ]);
    input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"][1] =
        json!({ "motion": "left-track", "binding": "left-model" });
    input
}

#[test]
fn model_weights_compile_to_native_bound_direct_blends_without_transition_use() {
    let input = document().to_string();
    let first = lower_authoring_json(&input).expect("model-bound direct blend must lower");
    let second = lower_authoring_json(&input).expect("repeat lowering");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);
    let machine = &first.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"][0]["value"], 25.0);
    assert!(machine["inputs"][0]["view_model_binding"].is_object());
    let children = &machine["layers"][0]["states"][1]["children"];
    assert_eq!(children[0]["input_id"], 1);
    assert_eq!(children[1]["input_id"], 0);
    assert_eq!(children[2]["input_id"], 3);
    let binding = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "left-model")
        .expect("binding source map");
    assert_eq!(binding.scene_paths, ["/artboard/state_machines/0/inputs/0"]);
    let scene: SceneSpec = serde_json::from_value(first.scene).expect("canonical scene");
    let bytes = compile_scene(&scene, None, 0).expect("binary compilation");
    let parsed = parse_riv(&bytes, &InspectFilter::default()).expect("encoded scene");
    let direct: Vec<_> = parsed
        .objects
        .iter()
        .filter(|object| object.type_key == type_keys::BLEND_ANIMATION_DIRECT)
        .collect();
    assert_eq!(direct.len(), 3);
    assert!(direct[1].properties.iter().any(|field| field.key
        == property_keys::BLEND_ANIMATION_DIRECT_BLEND_SOURCE
        && field.value == PropertyValueRead::UInt(2)));
    assert!(
        !direct[1]
            .properties
            .iter()
            .any(|field| field.key == property_keys::BLEND_ANIMATION_DIRECT_INPUT_ID)
    );
    assert_eq!(
        parsed
            .objects
            .iter()
            .filter(|object| object.type_key == type_keys::BINDABLE_PROPERTY_NUMBER)
            .count(),
        1
    );
    assert_eq!(
        parsed
            .objects
            .iter()
            .filter(|object| object.type_key == type_keys::DATA_BIND_CONTEXT)
            .count(),
        1
    );
}
