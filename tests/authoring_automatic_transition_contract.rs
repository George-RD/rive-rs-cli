use rive_cli::authoring::{
    AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec,
    AuthoringTarget, LoweredAuthoring, apply_operations, authoring_schema, lower_authoring,
    lower_authoring_json,
};
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::objects::core::type_keys;
use rive_cli::validator::{InspectFilter, parse_riv};
use serde_json::{Value, json};

fn scalar(value: f64) -> Value {
    json!({ "kind": "literal", "value": value, "unit": "scalar" })
}

fn document() -> Value {
    let mut input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/pointer-statechart.v0.json"
    ))
    .expect("authoring fixture");
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["inputs"] = json!([]);
    chart["events"] = json!([]);
    chart["listeners"] = json!([]);
    chart["transitions"][0]["when"] = json!("always");
    input
}

fn lower(input: &Value) -> LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("automatic transition must lower")
}

fn compile(scene: &Value) -> Vec<u8> {
    let scene: SceneSpec = serde_json::from_value(scene.clone()).expect("canonical scene");
    compile_scene(&scene, None, 0).expect("canonical compile")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid transition");
    assert!(
        error.diagnostics.iter().any(|diagnostic| diagnostic.code == code && diagnostic.path == path),
        "{error:?}"
    );
}

#[test]
fn explicit_always_guard_compiles_a_timed_transition_without_inputs() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["transitions"][0]["exit_time_ms"] = scalar(750.0);
    let lowered = lower(&input);
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"], json!([]));
    assert_eq!(
        machine["layers"][0]["transitions"][1],
        json!({ "from": 1, "to": 2, "conditions": [], "exit_time": 750 })
    );
    let binary = compile(&lowered.scene);
    let parsed = parse_riv(&binary, &InspectFilter::default()).expect("encoded automatic transition");
    for key in [type_keys::TRANSITION_BOOL_CONDITION, type_keys::TRANSITION_NUMBER_CONDITION,
        type_keys::TRANSITION_TRIGGER_CONDITION, type_keys::TRANSITION_VIEW_MODEL_CONDITION] {
        assert!(!parsed.objects.iter().any(|object| object.type_key == key));
    }
    let mut canonical = lowered.scene.clone();
    canonical["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]
        .as_object_mut().expect("transition").remove("conditions");
    assert_eq!(binary, compile(&canonical));
}

#[test]
fn automatic_guard_round_trips_through_the_typed_api_without_source_map_drift() {
    let input = document();
    let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed document");
    let lowered = lower(&input);
    assert_eq!(lowered, lower_authoring(&spec).expect("typed lowering"));
    let serialized = serde_json::to_value(&spec).expect("serialized document");
    assert_eq!(serialized["behavior"]["statecharts"][0]["transitions"][0]["when"], "always");
    assert_eq!(lowered, lower(&serialized));
    let source = lowered.source_map.entries.iter()
        .find(|entry| entry.authored_id == "gate/engage").expect("transition source");
    assert_eq!(source.scene_paths, ["/artboard/state_machines/0/layers/0/transitions/1"]);
}

#[test]
fn automatic_guards_are_explicit_and_cannot_hide_in_leaf_groups() {
    for guard in [Value::Null, json!(true), json!("Always"), json!("never"), json!(""),
        json!({}), json!({"always":true}), json!({"all":["always"]}), json!({"all":[],"always":true})] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["transitions"][0]["when"] = guard;
        assert_diagnostic(&input, "invalid_json", "$");
    }
    let mut input = document();
    input["behavior"]["statecharts"][0]["transitions"][0].as_object_mut().expect("transition").remove("when");
    assert_diagnostic(&input, "invalid_json", "$");
    let schema = authoring_schema();
    assert_eq!(schema["$defs"]["BehaviorAlwaysGuardSpec"]["enum"], json!(["always"]));
    assert!(schema["$defs"]["BehaviorTransitionSpec"]["required"].as_array().expect("required fields").contains(&json!("when")));
}

#[test]
fn always_does_not_relax_all_guard_limits_in_either_scope() {
    for in_region in [false, true] {
        for count in [0, 1001] {
            let mut input = document();
            let chart = &mut input["behavior"]["statecharts"][0];
            let invalid = json!({"id":"bad", "from":"resting", "to":"engaged", "when":{"all":vec![json!({"input":"missing", "equals":true});count]}});
            let path = if in_region {
                chart["regions"] = json!([{"id":"secondary", "initial":"resting", "states":chart["states"].clone(), "transitions":[invalid]}]);
                "$.behavior.statecharts[0].regions[0].transitions[0].when.all"
            } else {
                chart["transitions"].as_array_mut().expect("transitions").push(invalid);
                "$.behavior.statecharts[0].transitions[1].when.all"
            };
            assert_diagnostic(&input, "invalid_behavior_collection_count", path);
            let spec: AuthoringSpec = serde_json::from_value(input).expect("typed invalid count");
            let error = lower_authoring(&spec).expect_err("typed count must fail");
            assert_eq!(error.diagnostics[0].path, path);
            assert_eq!(error.diagnostics[0].code, "invalid_behavior_collection_count");
        }
    }
}

#[test]
fn root_and_region_automatic_sequences_keep_independent_named_identity() {
    let input: Value = serde_json::from_str(include_str!("../examples/authoring/automatic-sequence.v0.json")).expect("automatic example");
    let lowered = lower(&input);
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"], json!([]));
    for (layer, gate, id) in [(0, 500, "sequence/advance"), (1, 1000, "sequence/lower/advance")] {
        assert_eq!(machine["layers"][layer]["transitions"][1],
            json!({"from":1, "to":2, "conditions":[], "exit_time":gate, "duration":500}));
        let source = lowered.source_map.entries.iter().find(|entry| entry.authored_id == id).expect("scoped transition");
        assert_eq!(source.scene_paths, [format!("/artboard/state_machines/0/layers/{layer}/transitions/1")]);
    }
    assert_eq!(lowered, lower(&input));
    assert_eq!(compile(&lowered.scene), compile(&lower(&input).scene));
}

#[test]
fn automatic_transitions_compose_with_bound_and_all_guards_without_extra_inputs() {
    let mut input: Value = serde_json::from_str(include_str!("../examples/authoring/behavior-binding.v0.json")).expect("binding example");
    let baseline = lower_authoring_json(&input.to_string()).expect("original bound scene");
    let chart = &mut input["behavior"]["statecharts"][0];
    let guarded = chart["transitions"][0].clone();
    chart["transitions"] = json!([
        {"id":"auto", "from":"resting", "to":"engaged", "when":"always"},
        guarded,
        {"id":"return", "from":"engaged", "to":"resting", "when":{"all":[{"binding":"gate-enabled", "equals":false}]}}
    ]);
    let lowered = lower(&input);
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"], baseline.scene["artboard"]["state_machines"][0]["inputs"]);
    assert_eq!(machine["layers"][0]["transitions"][1]["conditions"], json!([]));
    assert_eq!(machine["layers"][0]["transitions"][2]["conditions"][0]["input"], machine["inputs"][0]["name"]);
    assert_eq!(machine["layers"][0]["transitions"][3]["conditions"][0]["input"], machine["inputs"][0]["name"]);
    assert!(!compile(&lowered.scene).is_empty());
}

#[test]
fn automatic_timing_keeps_omission_zero_expressions_and_authored_errors() {
    let mut input = document();
    let omitted = lower(&input);
    assert!(omitted.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1].get("exit_time").is_none());
    input["behavior"]["statecharts"][0]["transitions"][0]["exit_time_ms"] = scalar(0.0);
    let zero = lower(&input);
    assert_eq!(omitted.source_map, zero.source_map);
    assert_ne!(compile(&omitted.scene), compile(&zero.scene));
    input["parameters"] = json!({"gate":{"value":750, "unit":"scalar"}});
    input["behavior"]["statecharts"][0]["transitions"][0]["exit_time_ms"] = json!({"kind":"parameter", "name":"gate"});
    input["behavior"]["statecharts"][0]["transitions"][0]["duration_ms"] = scalar(250.0);
    let timed = lower(&input);
    assert_eq!(timed.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1],
        json!({"from":1,"to":2,"conditions":[],"exit_time":750,"duration":250}));
    input["behavior"]["statecharts"][0]["transitions"][0]["exit_time_ms"] = json!({"kind":"parameter", "name":"missing"});
    assert_diagnostic(&input, "unknown_parameter", "$.behavior.statecharts[0].transitions[0].exit_time_ms.name");
}

#[test]
fn automatic_blend_sources_still_reject_unsupported_exit_gates() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][0] = json!({"id":"resting", "direct_blend":{"motions":[
        {"motion":"rest-track", "weight":scalar(100.0)}
    ]}});
    assert!(!compile(&lower(&input).scene).is_empty());
    input["behavior"]["statecharts"][0]["transitions"][0]["exit_time_ms"] = scalar(750.0);
    assert_diagnostic(&input, "unsupported_transition_exit_source", "$.behavior.statecharts[0].transitions[0].exit_time_ms");
}

#[test]
fn automatic_statechart_edits_commit_atomically_and_bad_references_roll_back() {
    let input = document();
    let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed document");
    let before = lower_authoring(&spec).expect("original lowering");
    for valid in [true, false] {
        let mut replacement = input["behavior"]["statecharts"][0].clone();
        replacement["transitions"][0]["duration_ms"] = scalar(250.0);
        if !valid { replacement["transitions"][0]["to"] = json!("missing"); }
        let operations = [
            AuthoringOperation::Remove {target:AuthoringTarget::BehaviorStatechart {target_id:"gate".to_string()}},
            AuthoringOperation::Insert {entity:AuthoringEntity::BehaviorStatechart(serde_json::from_value(replacement).expect("replacement chart")),
                placement:AuthoringPlacement::Into {container:AuthoringContainer::BehaviorStatecharts}},
        ];
        let result = apply_operations(&spec, &operations);
        if valid {
            let applied = result.expect("valid automatic chart replacement");
            assert_eq!(applied.lowered.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["duration"], json!(250));
            assert_eq!(applied.lowered.source_map, before.source_map);
            assert_eq!(lower_authoring(&applied.spec).expect("edited document"), applied.lowered);
        }
        else { assert!(result.is_err(), "bad target must roll back"); }
        assert_eq!(lower_authoring(&spec).expect("unchanged source"), before);
    }
}
