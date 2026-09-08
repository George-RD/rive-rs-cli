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
    assert_builds(first.scene);
}

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("valid direct blend")
}

fn compile(scene: &Value) -> Vec<u8> {
    let scene = serde_json::from_value(scene.clone()).expect("canonical scene");
    rive_cli::compile::compile_scene(&scene, None, 0).expect("canonical binary")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid direct blend");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code && diagnostic.path == path),
        "{error:?}"
    );
}

#[test]
fn direct_blend_compiles_through_the_existing_binary_objects() {
    use rive_cli::objects::core::type_keys;
    use rive_cli::validator::{InspectFilter, parse_riv, validate_riv};
    let lowered = lower(&document());
    let bytes = compile(&lowered.scene);
    assert_eq!(bytes, compile(&lower(&document()).scene));
    assert!(validate_riv(&bytes).expect("valid binary").valid);
    let parsed = parse_riv(&bytes, &InspectFilter::default()).expect("parsed binary");
    assert_eq!(
        parsed
            .objects
            .iter()
            .filter(|object| object.type_key == type_keys::BLEND_STATE_DIRECT)
            .count(),
        1
    );
    assert_eq!(
        parsed
            .objects
            .iter()
            .filter(|object| object.type_key == type_keys::BLEND_ANIMATION_DIRECT)
            .count(),
        2
    );
}

#[test]
fn direct_blends_resolve_inputs_after_binding_injection_and_compose_with_raw_motion() {
    let mut input = document();
    input["motion"]["raw_animations"] = json!([
        { "id": "expert-track", "value": { "name": "expert_animation", "fps": 60, "duration": 1, "keyframes": [] } }
    ]);
    input["behavior"]["models"] = json!([
        { "id": "model", "properties": [{ "kind": "bool", "id": "enabled", "value": false }] }
    ]);
    input["behavior"]["bindings"] = json!([
        { "id": "enabled-binding", "model": "model", "property": "enabled" }
    ]);
    input["behavior"]["statecharts"][0]["transitions"] = json!([
        { "id": "refresh", "from": "reading", "to": "reading", "when": { "binding": "enabled-binding", "equals": true } }
    ]);
    let lowered = lower(&input);
    assert_eq!(
        lowered.scene["artboard"]["animations"]
            .as_array()
            .expect("animations")
            .len(),
        3
    );
    assert_eq!(
        lowered.scene["artboard"]["animations"][2]["name"],
        "expert_animation"
    );
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"][0]["type"], "bool");
    let children = &machine["layers"][0]["states"][1]["children"];
    assert_eq!(children[0]["animation_id"], 1);
    assert_eq!(children[0]["input_id"], 1);
    assert_eq!(children[1]["animation_id"], 0);
    assert_eq!(children[1]["input_id"], 2);
    compile(&lowered.scene);
}

#[test]
fn direct_blends_share_chart_inputs_in_parallel_regions() {
    let mut input = document();
    let state = input["behavior"]["statecharts"][0]["states"][0].clone();
    input["behavior"]["statecharts"][0]["regions"] = json!([
        { "id": "secondary", "initial": "reading", "states": [state] }
    ]);
    let lowered = lower(&input);
    let layers = &lowered.scene["artboard"]["state_machines"][0]["layers"];
    assert_eq!(layers[0]["states"][1], layers[1]["states"][1]);
    let source = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "meter/secondary/reading")
        .expect("region source map");
    assert_eq!(
        source.authored_path,
        "$.behavior.statecharts[0].regions[0].states[0]"
    );
    assert_eq!(
        source.scene_paths,
        ["/artboard/state_machines/0/layers/1/states/1"]
    );
    compile(&lowered.scene);
}

#[test]
fn direct_blend_references_and_number_input_kinds_have_authored_diagnostics() {
    let root = "$.behavior.statecharts[0].states[0].direct_blend.motions[0]";
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"][0]["motion"] =
        json!("missing");
    assert_diagnostic(&input, "unknown_behavior_motion", &format!("{root}.motion"));
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"][0]["input"] =
        json!("missing");
    assert_diagnostic(&input, "unknown_behavior_input", &format!("{root}.input"));
    for replacement in [
        json!({"kind":"bool", "id":"load", "value":false}),
        json!({"kind":"trigger", "id":"load"}),
    ] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["inputs"][0] = replacement;
        assert_diagnostic(&input, "invalid_blend_input", &format!("{root}.input"));
    }
}

#[test]
fn a_state_cannot_combine_direct_blend_with_another_motion_source() {
    for (field, value) in [
        ("motion", json!("calm-track")),
        (
            "blend",
            json!({"input":"load", "stops":[
                {"motion":"calm-track", "value":{"kind":"literal","value":0,"unit":"scalar"}},
                {"motion":"surge-track", "value":{"kind":"literal","value":100,"unit":"scalar"}}
            ]}),
        ),
    ] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["states"][0][field] = value;
        assert_diagnostic(
            &input,
            "ambiguous_state_motion",
            "$.behavior.statecharts[0].states[0]",
        );
    }
}

#[test]
fn direct_blend_child_count_accepts_one_and_a_thousand_but_rejects_empty_and_overflow() {
    let motion = json!({ "motion":"calm-track", "input":"load" });
    for count in [1, 1000] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"] =
            json!(vec![motion.clone(); count]);
        compile(&lower(&input).scene);
    }
    for count in [0, 1001] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"] =
            json!(vec![motion.clone(); count]);
        assert_diagnostic(
            &input,
            "invalid_direct_blend_motions",
            "$.behavior.statecharts[0].states[0].direct_blend.motions",
        );
    }
}

#[test]
fn direct_blend_exit_time_is_rejected_but_duration_and_destination_gates_remain_valid() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"]
        .as_array_mut()
        .expect("states")
        .push(json!({"id":"rest", "motion":"calm-track"}));
    input["behavior"]["statecharts"][0]["transitions"] = json!([
        {"id":"leave", "from":"reading", "to":"rest", "when":{"input":"load", "compare":"greater", "value":{"kind":"literal","value":50,"unit":"scalar"}}, "duration_ms":{"kind":"literal","value":125,"unit":"scalar"}},
        {"id":"return", "from":"rest", "to":"reading", "when":{"input":"load", "compare":"less", "value":{"kind":"literal","value":50,"unit":"scalar"}}, "exit_time_ms":{"kind":"literal","value":10,"unit":"scalar"}}
    ]);
    let lowered = lower(&input);
    assert_eq!(
        lowered.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["duration"],
        125
    );
    compile(&lowered.scene);
    input["behavior"]["statecharts"][0]["transitions"][0]["exit_time_ms"] =
        json!({"kind":"literal","value":0,"unit":"scalar"});
    assert_diagnostic(
        &input,
        "unsupported_transition_exit_source",
        "$.behavior.statecharts[0].transitions[0].exit_time_ms",
    );
}

#[test]
fn removing_a_direct_blend_motion_is_atomic() {
    use rive_cli::authoring::{
        AuthoringOperation, AuthoringSpec, AuthoringTarget, apply_operation, lower_authoring,
    };
    let document: AuthoringSpec = serde_json::from_value(document()).expect("typed document");
    let before = serde_json::to_value(&document).expect("original");
    let lowered = lower_authoring(&document).expect("original lowers");
    let operation = AuthoringOperation::Remove {
        target: AuthoringTarget::MotionTrack {
            target_id: "surge-track".to_string(),
        },
    };
    let error =
        apply_operation(&document, &operation).expect_err("referenced motion cannot be removed");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "unknown_behavior_motion"
                && diagnostic.path
                    == "$.behavior.statecharts[0].states[0].direct_blend.motions[0].motion"),
        "{error:?}"
    );
    assert_eq!(serde_json::to_value(&document).expect("unchanged"), before);
    assert_eq!(lower_authoring(&document).expect("still lowers"), lowered);
}

#[test]
fn direct_blend_schema_is_strict_and_matches_the_published_contract() {
    use rive_cli::authoring::authoring_schema;
    let schema = authoring_schema();
    let direct = &schema["$defs"]["BehaviorDirectBlendSpec"];
    assert_eq!(direct["additionalProperties"], false);
    assert_eq!(direct["properties"]["motions"]["minItems"], 1);
    assert_eq!(direct["properties"]["motions"]["maxItems"], 1000);
    let motion = &schema["$defs"]["BehaviorDirectBlendMotionSpec"];
    assert_eq!(motion["additionalProperties"], false);
    assert_eq!(motion["required"], json!(["motion", "input"]));
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"][0]["input_id"] =
        json!(0);
    let error =
        lower_authoring_json(&input.to_string()).expect_err("runtime index escape is not accepted");
    assert_eq!(error.diagnostics[0].code, "invalid_json");
}

#[test]
fn absent_direct_blends_preserve_existing_canonical_scene_map_and_bytes() {
    let original: Value =
        serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json"))
            .expect("existing example");
    let mut explicit_null = original.clone();
    explicit_null["behavior"]["statecharts"][0]["states"][0]["direct_blend"] = Value::Null;
    let before = lower(&original);
    let after = lower(&explicit_null);
    assert_eq!(before, after);
    assert_eq!(compile(&before.scene), compile(&after.scene));
}
