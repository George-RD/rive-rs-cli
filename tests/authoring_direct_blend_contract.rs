mod support;

use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};
use support::assert_builds;

fn document() -> Value {
    let mut document: Value =
        serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json"))
            .expect("valid authored fixture");
    let chart = &mut document["behavior"]["statecharts"][0];
    chart["inputs"].as_array_mut().expect("inputs").push(json!({
        "kind": "number", "id": "counterweight",
        "value": { "kind": "literal", "value": 100.0, "unit": "scalar" }
    }));
    chart["states"][0] = json!({
        "id": "reading",
        "direct_blend": {
            "motions": [
                { "motion": "surge-track", "input": "load" },
                { "motion": "calm-track", "input": "counterweight" }
            ]
        }
    });
    document
}

#[test]
fn named_direct_blends_preserve_authored_order_and_resolve_canonical_indices() {
    let input = document().to_string();
    let first = lower_authoring_json(&input).expect("direct blends must lower");
    let second = lower_authoring_json(&input).expect("direct blends must lower again");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);
    let state = &first.scene["artboard"]["state_machines"][0]["layers"][0]["states"][1];
    assert_eq!(state["type"], "blend_state_direct");
    let children = state["children"].as_array().expect("direct children");
    assert_eq!(children.len(), 2);
    assert_eq!(children[0]["type"], "blend_animation_direct");
    assert_eq!(children[0]["animation_id"], 1);
    assert_eq!(children[0]["input_id"], 0);
    assert_eq!(children[1]["animation_id"], 0);
    assert_eq!(children[1]["input_id"], 1);
    let state_source = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "meter/reading")
        .expect("named state source");
    assert_eq!(
        state_source.authored_path,
        "$.behavior.statecharts[0].states[0]"
    );
    assert_eq!(
        state_source.scene_paths,
        ["/artboard/state_machines/0/layers/0/states/1"]
    );
    assert_builds(&first.scene);
}
