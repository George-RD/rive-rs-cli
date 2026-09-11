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
        "kind": "trigger", "id": "enabled"
    });
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] = json!({
        "binding": "gate-enabled"
    });
    input
}

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("trigger binding must lower")
}

fn compile(scene: &Value) -> Vec<u8> {
    let scene: SceneSpec = serde_json::from_value(scene.clone()).expect("canonical scene");
    compile_scene(&scene, None, 0).expect("canonical scene must compile")
}

fn encoded_type_keys(scene: &Value) -> Vec<u16> {
    parse_riv(&compile(scene), &InspectFilter::default())
        .expect("encoded scene")
        .objects
        .iter()
        .map(|object| object.type_key)
        .collect()
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid binding document");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == code && diagnostic.path == path }),
        "{error:?}"
    );
}

#[test]
fn trigger_view_model_binding_compiles_to_a_model_condition_not_an_input_condition() {
    let lowered = lower(&document());
    let model = &lowered.scene["artboard"]["children"][1];
    assert_eq!(model["children"][0]["type"], "view_model_property_trigger");
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"][0]["type"], "trigger");
    assert!(machine["inputs"][0].get("value").is_none());
    assert_eq!(
        machine["inputs"][0]["view_model_binding"]["property"],
        model["children"][0]["name"]
    );
    let condition = &machine["layers"][0]["transitions"][1]["conditions"][0];
    assert_eq!(condition["input"], machine["inputs"][0]["name"]);
    assert!(condition.get("op").is_none());
    assert!(condition.get("value").is_none());

    let types = encoded_type_keys(&lowered.scene);
    assert!(types.contains(&type_keys::VIEW_MODEL_PROPERTY_TRIGGER));
    assert!(types.contains(&type_keys::TRANSITION_VIEW_MODEL_CONDITION));
    assert!(types.contains(&type_keys::BINDABLE_PROPERTY_TRIGGER));
    assert!(types.contains(&type_keys::DATA_BIND_CONTEXT));
    assert!(types.contains(&type_keys::TRANSITION_PROPERTY_VIEW_MODEL_COMPARATOR));
    assert!(types.contains(&type_keys::TRANSITION_VALUE_TRIGGER_COMPARATOR));
    assert!(!types.contains(&type_keys::TRANSITION_TRIGGER_CONDITION));
}

#[test]
fn a_trigger_model_property_carries_no_authored_value() {
    let mut input = document();
    input["behavior"]["models"][0]["properties"][0] = json!({
        "kind": "trigger", "id": "enabled", "value": true
    });
    let error = lower_authoring_json(&input.to_string()).expect_err("trigger takes no value");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "invalid_json"),
        "{error:?}"
    );
}

#[test]
fn a_trigger_binding_condition_rejects_boolean_and_numeric_properties() {
    for property in [
        json!({ "kind": "bool", "id": "enabled", "value": false }),
        json!({
            "kind": "number", "id": "enabled",
            "value": { "kind": "literal", "value": 0.0, "unit": "scalar" }
        }),
    ] {
        let mut input = document();
        input["behavior"]["models"][0]["properties"][0] = property;
        assert_diagnostic(
            &input,
            "invalid_condition_binding",
            "$.behavior.statecharts[0].transitions[0].when.binding",
        );
    }
}

#[test]
fn boolean_and_numeric_conditions_reject_a_trigger_property() {
    for when in [
        json!({ "binding": "gate-enabled", "equals": true }),
        json!({
            "binding": "gate-enabled", "compare": "greater_or_equal",
            "value": { "kind": "literal", "value": 1.0, "unit": "scalar" }
        }),
    ] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["transitions"][0]["when"] = when;
        assert_diagnostic(
            &input,
            "invalid_condition_binding",
            "$.behavior.statecharts[0].transitions[0].when.binding",
        );
    }
}

#[test]
fn an_unknown_trigger_binding_reports_the_authored_condition_path() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] = json!({ "binding": "missing" });
    assert_diagnostic(
        &input,
        "unknown_behavior_binding",
        "$.behavior.statecharts[0].transitions[0].when.binding",
    );
}

#[test]
fn a_trigger_binding_is_rejected_as_a_blend_weight_source() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][1] = json!({
        "id": "engaged",
        "blend": {
            "binding": "gate-enabled",
            "stops": [
                { "motion": "rest-track", "value": { "kind": "literal", "value": 0.0, "unit": "scalar" } },
                { "motion": "active-track", "value": { "kind": "literal", "value": 100.0, "unit": "scalar" } }
            ]
        }
    });
    assert_diagnostic(
        &input,
        "invalid_blend_binding",
        "$.behavior.statecharts[0].states[1].blend.binding",
    );
}

#[test]
fn a_trigger_binding_used_only_inside_a_region_is_discovered_and_lowered() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["transitions"] = json!([]);
    input["behavior"]["statecharts"][0]["regions"] = json!([{
        "id": "pulse",
        "initial": "resting",
        "states": [
            { "id": "resting", "motion": "rest-track" },
            { "id": "engaged", "motion": "active-track" }
        ],
        "transitions": [
            { "id": "engage", "from": "resting", "to": "engaged", "when": { "binding": "gate-enabled" } }
        ]
    }]);
    let lowered = lower(&input);
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"][0]["type"], "trigger");
    assert!(machine["inputs"][0]["view_model_binding"].is_object());
    let condition = &machine["layers"][1]["transitions"][1]["conditions"][0];
    assert_eq!(condition["input"], machine["inputs"][0]["name"]);
    let types = encoded_type_keys(&lowered.scene);
    assert!(types.contains(&type_keys::BINDABLE_PROPERTY_TRIGGER));
    assert!(!types.contains(&type_keys::TRANSITION_TRIGGER_CONDITION));
}

#[test]
fn one_trigger_binding_shared_by_two_transitions_emits_a_single_input() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["transitions"] = json!([
        { "id": "engage", "from": "resting", "to": "engaged", "when": { "binding": "gate-enabled" } },
        { "id": "release", "from": "engaged", "to": "resting", "when": { "binding": "gate-enabled" } }
    ]);
    let lowered = lower(&input);
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(
        machine["inputs"].as_array().expect("inputs").len(),
        1,
        "{:?}",
        machine["inputs"]
    );
    let transitions = machine["layers"][0]["transitions"]
        .as_array()
        .expect("transitions");
    let bound: Vec<_> = transitions
        .iter()
        .filter(|transition| transition["conditions"][0]["input"] == machine["inputs"][0]["name"])
        .collect();
    assert_eq!(bound.len(), 2, "{transitions:?}");
}

#[test]
fn an_unbound_trigger_condition_keeps_its_previous_canonical_output() {
    let mut input = document();
    input["behavior"]["models"] = json!([]);
    input["behavior"]["bindings"] = json!([]);
    input["behavior"]["statecharts"][0]["inputs"] = json!([{ "kind": "trigger", "id": "fire" }]);
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] = json!({ "trigger": "fire" });
    let types = encoded_type_keys(&lower(&input).scene);
    assert!(types.contains(&type_keys::TRANSITION_TRIGGER_CONDITION));
    assert!(!types.contains(&type_keys::TRANSITION_VIEW_MODEL_CONDITION));
    assert!(!types.contains(&type_keys::BINDABLE_PROPERTY_TRIGGER));
}

#[test]
fn trigger_binding_lowering_is_deterministic_and_keeps_source_map_identity() {
    let input = document();
    let first = lower(&input);
    let second = lower(&input);
    assert_eq!(first.scene, second.scene);
    assert_eq!(compile(&first.scene), compile(&second.scene));
    let property = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "gate-model/enabled")
        .expect("trigger property source entry");
    assert_eq!(property.authored_path, "$.behavior.models[0].properties[0]");
    assert_eq!(
        property.scene_paths,
        vec!["/artboard/children/1/children/0"]
    );
}

#[test]
fn canonical_trigger_binding_rejects_a_non_trigger_model_property() {
    let mut scene = lower(&document()).scene;
    scene["artboard"]["children"][1]["children"][0]["type"] = json!("view_model_property_boolean");
    let scene: SceneSpec = serde_json::from_value(scene).expect("canonical scene");
    let error = compile_scene(&scene, None, 0).expect_err("binding must name a trigger property");
    assert!(error.to_string().contains("trigger"), "{error}");
}

#[test]
fn canonical_trigger_binding_rejects_an_unresolvable_model_property() {
    let mut scene = lower(&document()).scene;
    scene["artboard"]["state_machines"][0]["inputs"][0]["view_model_binding"]["property"] =
        json!("missing");
    let scene: SceneSpec = serde_json::from_value(scene).expect("canonical scene");
    let error = compile_scene(&scene, None, 0).expect_err("binding must resolve");
    assert!(error.to_string().contains("missing"), "{error}");
}

#[test]
fn a_failed_statechart_edit_preserves_trigger_binding_identity() {
    use rive_cli::authoring::{
        AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec,
        AuthoringTarget, apply_operations, lower_authoring,
    };
    let input = document();
    let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed source");
    let snapshot = serde_json::to_value(&spec).expect("snapshot");
    let before = lower_authoring(&spec).expect("initial lowered source");
    let mut replacement = input["behavior"]["statecharts"][0].clone();
    replacement["transitions"][0]["when"] = json!({ "binding": "missing" });
    let operations = [
        AuthoringOperation::Remove {
            target: AuthoringTarget::BehaviorStatechart {
                target_id: "gate".to_string(),
            },
        },
        AuthoringOperation::Insert {
            entity: AuthoringEntity::BehaviorStatechart(
                serde_json::from_value(replacement).expect("replacement"),
            ),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::BehaviorStatecharts,
            },
        },
    ];
    let error =
        apply_operations(&spec, &operations).expect_err("invalid trigger binding must roll back");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|entry| entry.code == "unknown_behavior_binding"),
        "{error:?}"
    );
    assert_eq!(serde_json::to_value(&spec).expect("unchanged"), snapshot);
    assert_eq!(lower_authoring(&spec).expect("still valid"), before);
}

#[test]
fn canonical_bound_trigger_conditions_reject_comparison_operators_and_values() {
    let scene = lower(&document()).scene;
    for extra in [
        json!({ "op": "==" }),
        json!({ "op": ">=" }),
        json!({ "value": true }),
        json!({ "value": false }),
        json!({ "value": 5 }),
    ] {
        let mut invalid = scene.clone();
        let condition = invalid
            .pointer_mut("/artboard/state_machines/0/layers/0/transitions/1/conditions/0")
            .expect("bound trigger condition");
        for (key, value) in extra.as_object().expect("extra fields") {
            condition[key] = value.clone();
        }
        let invalid: SceneSpec = serde_json::from_value(invalid).expect("canonical scene");
        let error = compile_scene(&invalid, None, 0)
            .expect_err("a bound trigger condition must not carry an op or value");
        assert!(error.to_string().contains("trigger"), "{extra}: {error}");
    }
}

#[test]
fn canonical_bound_trigger_conditions_accept_an_explicit_null_value() {
    let mut scene = lower(&document()).scene;
    scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["conditions"][0]["value"] =
        Value::Null;
    let scene: SceneSpec = serde_json::from_value(scene).expect("canonical scene");
    let bytes = compile_scene(&scene, None, 0).expect("an absent value must still compile");
    let types: Vec<_> = parse_riv(&bytes, &InspectFilter::default())
        .expect("encoded scene")
        .objects
        .iter()
        .map(|object| object.type_key)
        .collect();
    assert!(types.contains(&type_keys::TRANSITION_VALUE_TRIGGER_COMPARATOR));
    assert!(!types.contains(&type_keys::TRANSITION_TRIGGER_CONDITION));
}

#[test]
fn bound_trigger_inputs_resolve_chart_local_offsets_across_statecharts() {
    let mut input = document();
    input["behavior"]["models"][0]["properties"]
        .as_array_mut()
        .expect("properties")
        .push(json!({ "kind": "bool", "id": "visible", "value": false }));
    input["behavior"]["bindings"]
        .as_array_mut()
        .expect("bindings")
        .insert(
            0,
            json!({ "id": "gate-visible", "model": "gate-model", "property": "visible" }),
        );
    input["behavior"]["statecharts"][0]["transitions"] = json!([
        { "id": "reveal", "from": "resting", "to": "engaged", "when": { "binding": "gate-visible", "equals": true } },
        { "id": "engage", "from": "engaged", "to": "resting", "when": { "binding": "gate-enabled" } }
    ]);
    let mut other = input["behavior"]["statecharts"][0].clone();
    other["id"] = json!("other");
    other["transitions"] = json!([
        { "id": "engage", "from": "resting", "to": "engaged", "when": { "binding": "gate-enabled" } }
    ]);
    input["behavior"]["statecharts"]
        .as_array_mut()
        .expect("charts")
        .push(other);
    let lowered = lower(&input);
    let machines = lowered.scene["artboard"]["state_machines"]
        .as_array()
        .expect("machines");
    assert_eq!(machines.len(), 2);
    assert_eq!(machines[0]["inputs"].as_array().expect("inputs").len(), 2);
    assert_eq!(machines[1]["inputs"].as_array().expect("inputs").len(), 1);
    let bound = [&machines[0]["inputs"][1], &machines[1]["inputs"][0]];
    for input in bound {
        assert_eq!(input["type"], "trigger");
        assert!(input["view_model_binding"].is_object());
    }
    assert_ne!(bound[0]["name"], bound[1]["name"]);
    assert_eq!(
        bound[0]["view_model_binding"],
        bound[1]["view_model_binding"]
    );
    assert_eq!(
        machines[0]["layers"][0]["transitions"][2]["conditions"][0]["input"],
        bound[0]["name"]
    );
    assert_eq!(
        machines[1]["layers"][0]["transitions"][1]["conditions"][0]["input"],
        bound[1]["name"]
    );
    let binding = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "gate-enabled")
        .expect("trigger binding source entry");
    assert_eq!(
        binding.scene_paths,
        vec![
            "/artboard/state_machines/0/inputs/1",
            "/artboard/state_machines/1/inputs/0"
        ]
    );
    let types = encoded_type_keys(&lowered.scene);
    assert!(types.contains(&type_keys::BINDABLE_PROPERTY_TRIGGER));
    assert!(!types.contains(&type_keys::TRANSITION_TRIGGER_CONDITION));
}

#[test]
fn one_trigger_binding_shared_by_a_root_and_a_region_transition_emits_a_single_input() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["regions"] = json!([{
        "id": "pulse",
        "initial": "resting",
        "states": [
            { "id": "resting", "motion": "rest-track" },
            { "id": "engaged", "motion": "active-track" }
        ],
        "transitions": [
            { "id": "engage", "from": "resting", "to": "engaged", "when": { "binding": "gate-enabled" } }
        ]
    }]);
    let lowered = lower(&input);
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(
        machine["inputs"].as_array().expect("inputs").len(),
        1,
        "{:?}",
        machine["inputs"]
    );
    assert_eq!(machine["inputs"][0]["type"], "trigger");
    assert!(machine["inputs"][0]["view_model_binding"].is_object());
    for layer in [0, 1] {
        assert_eq!(
            machine["layers"][layer]["transitions"][1]["conditions"][0]["input"],
            machine["inputs"][0]["name"],
            "{:?}",
            machine["layers"][layer]["transitions"]
        );
    }
    let binding = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "gate-enabled")
        .expect("trigger binding source entry");
    assert_eq!(
        binding.scene_paths,
        vec!["/artboard/state_machines/0/inputs/0"]
    );
    let types = encoded_type_keys(&lowered.scene);
    assert!(types.contains(&type_keys::BINDABLE_PROPERTY_TRIGGER));
    assert!(!types.contains(&type_keys::TRANSITION_TRIGGER_CONDITION));
}

#[test]
fn an_unbound_trigger_condition_keeps_its_previous_canonical_bytes() {
    use sha2::{Digest, Sha256};
    let mut input = document();
    input["behavior"]["models"] = json!([]);
    input["behavior"]["bindings"] = json!([]);
    input["behavior"]["statecharts"][0]["inputs"] = json!([{ "kind": "trigger", "id": "fire" }]);
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] = json!({ "trigger": "fire" });
    let bytes = compile(&lower(&input).scene);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "42b2cc3e9073135f17453dc0a4f167734428cdeac3352aed620336736c4e984a"
    );
}

#[test]
fn the_shipped_trigger_binding_example_lowers_to_a_bound_and_a_declared_trigger_input() {
    let input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/trigger-binding.v0.json"
    ))
    .expect("trigger binding example");
    let lowered = lower(&input);
    let model = &lowered.scene["artboard"]["children"][1];
    assert_eq!(model["children"][0]["type"], "view_model_property_trigger");
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"].as_array().expect("inputs").len(), 2);
    assert_eq!(machine["inputs"][0]["type"], "trigger");
    assert_eq!(
        machine["inputs"][0]["view_model_binding"]["property"],
        model["children"][0]["name"]
    );
    assert_eq!(machine["inputs"][1]["type"], "trigger");
    assert!(machine["inputs"][1].get("view_model_binding").is_none());
    assert_eq!(
        machine["layers"][0]["transitions"][1]["conditions"][0]["input"],
        machine["inputs"][0]["name"]
    );
    assert_eq!(
        machine["layers"][0]["transitions"][2]["conditions"][0]["input"],
        machine["inputs"][1]["name"]
    );
    let types = encoded_type_keys(&lowered.scene);
    assert!(types.contains(&type_keys::BINDABLE_PROPERTY_TRIGGER));
    assert!(types.contains(&type_keys::TRANSITION_VIEW_MODEL_CONDITION));
    assert!(types.contains(&type_keys::TRANSITION_TRIGGER_CONDITION));
}
