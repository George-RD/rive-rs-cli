use rive_cli::authoring::lower_authoring_json;
use rive_cli::compile::compile_scene;
use rive_cli::objects::core::{property_keys, type_keys};
use rive_cli::validator::{InspectFilter, PropertyValueRead, parse_riv};
use serde_json::{Value, json};

fn document() -> Value {
    let mut input: Value =
        serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json"))
            .expect("authored fixture");
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["inputs"] = json!([]);
    chart["states"][0] = json!({
        "id": "reading",
        "direct_blend": { "motions": [
            { "motion": "calm-track", "weight": { "kind": "literal", "value": 100, "unit": "scalar" } },
            { "motion": "surge-track", "weight": { "kind": "literal", "value": 25, "unit": "scalar" } }
        ] }
    });
    input
}

#[test]
fn fixed_weights_lower_to_native_constants_without_synthetic_inputs() {
    let input = document().to_string();
    let first = lower_authoring_json(&input).expect("fixed weights must lower");
    let second = lower_authoring_json(&input).expect("repeat lowering");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);
    let machine = &first.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"], json!([]));
    let children = &machine["layers"][0]["states"][1]["children"];
    assert_eq!(
        children,
        &json!([
            { "type": "blend_animation_direct", "animation_id": 0, "blend_source": 1, "mix_value": 100.0 },
            { "type": "blend_animation_direct", "animation_id": 1, "blend_source": 1, "mix_value": 25.0 }
        ])
    );
    let scene = serde_json::from_value(first.scene).expect("canonical scene");
    let bytes = compile_scene(&scene, None, 0).expect("binary compilation");
    let parsed = parse_riv(&bytes, &InspectFilter::default()).expect("encoded fixed blend");
    assert!(!parsed.objects.iter().any(|object| matches!(
        object.type_key,
        type_keys::STATE_MACHINE_NUMBER | type_keys::DATA_BIND_CONTEXT
    )));
    let direct = parsed
        .objects
        .iter()
        .filter(|object| object.type_key == type_keys::BLEND_ANIMATION_DIRECT)
        .collect::<Vec<_>>();
    assert_eq!(direct.len(), 2);
    for child in &direct {
        assert!(child.properties.iter().any(|field| field.key
            == property_keys::BLEND_ANIMATION_DIRECT_BLEND_SOURCE
            && field.value == PropertyValueRead::UInt(1)));
    }
    assert!(direct[1].properties.iter().any(|field| field.key
        == property_keys::BLEND_ANIMATION_DIRECT_MIX_VALUE
        && field.value == PropertyValueRead::Float(25.0)));
}
