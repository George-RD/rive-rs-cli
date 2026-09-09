use rive_cli::authoring::lower_authoring_json;
use rive_cli::compile::compile_scene;
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
