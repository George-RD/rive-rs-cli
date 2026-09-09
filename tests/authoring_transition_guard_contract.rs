use rive_cli::authoring::{
    AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec,
    AuthoringTarget, apply_operations, authoring_schema, lower_authoring, lower_authoring_json,
};
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::objects::core::type_keys;
use rive_cli::validator::{InspectFilter, parse_riv};
use serde_json::{Value, json};

const WHEN: &str = "/behavior/statecharts/0/transitions/0/when";

fn scalar(value: f64) -> Value {
    json!({ "kind": "literal", "value": value, "unit": "scalar" })
}

fn document() -> Value {
    let mut input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/behavior-binding.v0.json"
    ))
    .expect("authoring fixture");
    input["behavior"]["statecharts"][0]["inputs"] = json!([
        { "kind": "bool", "id": "armed", "value": false },
        { "kind": "number", "id": "load", "value": scalar(0.0) }
    ]);
    *input.pointer_mut(WHEN).expect("guard") = json!({
        "all": [
            { "input": "armed", "equals": true },
            { "binding": "gate-enabled", "equals": true },
            { "input": "load", "compare": "greater_or_equal", "value": scalar(60.0) }
        ]
    });
    input
}

fn every_kind() -> Value {
    let mut input = document();
    input["behavior"]["models"][0]["properties"]
        .as_array_mut()
        .expect("properties")
        .push(json!({ "kind": "number", "id": "level", "value": scalar(25.0) }));
    input["behavior"]["bindings"]
        .as_array_mut()
        .expect("bindings")
        .push(json!({ "id": "gate-level", "model": "gate-model", "property": "level" }));
    input["behavior"]["statecharts"][0]["inputs"]
        .as_array_mut()
        .expect("inputs")
        .push(json!({ "kind": "trigger", "id": "release" }));
    let all = input.pointer_mut(WHEN).expect("guard")["all"]
        .as_array_mut()
        .expect("conditions");
    all.push(json!({ "binding": "gate-level", "compare": "less", "value": scalar(80.0) }));
    all.push(json!({ "trigger": "release" }));
    input
}

fn compile(scene: &Value) -> Vec<u8> {
    let scene: SceneSpec = serde_json::from_value(scene.clone()).expect("canonical scene");
    compile_scene(&scene, None, 0).expect("compiled guard")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid guard");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == code && diagnostic.path == path }),
        "{error:?}"
    );
}

#[test]
fn all_guard_lowers_mixed_named_conditions_in_authored_order() {
    let input = document();
    let lowered = lower_authoring_json(&input.to_string()).expect("all guard must lower");
    assert_eq!(
        lowered,
        lower_authoring_json(&input.to_string()).expect("repeat lowering")
    );
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    let conditions = &machine["layers"][0]["transitions"][1]["conditions"];
    assert_eq!(conditions.as_array().expect("conditions").len(), 3);
    assert_eq!(conditions[0]["input"], machine["inputs"][1]["name"]);
    assert_eq!(conditions[1]["input"], machine["inputs"][0]["name"]);
    assert_eq!(conditions[2]["input"], machine["inputs"][2]["name"]);
    assert_eq!(conditions[0]["value"], true);
    assert_eq!(conditions[1]["value"], true);
    assert_eq!(conditions[2]["op"], ">=");
    assert_eq!(conditions[2]["value"], 60.0);
    assert!(machine["inputs"][0]["view_model_binding"].is_object());
    assert!(!compile(&lowered.scene).is_empty());
}

#[test]
fn each_leaf_keeps_its_single_condition_bytes_and_source_map() {
    let template = every_kind();
    for leaf in template.pointer(WHEN).expect("guard")["all"]
        .as_array()
        .expect("leaves")
    {
        let mut single = template.clone();
        *single.pointer_mut(WHEN).expect("guard") = leaf.clone();
        let before = lower_authoring_json(&single.to_string()).expect("legacy single condition");
        *single.pointer_mut(WHEN).expect("guard") = json!({ "all": [leaf] });
        let after = lower_authoring_json(&single.to_string()).expect("one-member group");
        assert_eq!(before, after);
        assert_eq!(compile(&before.scene), compile(&after.scene));
    }
}

#[test]
fn every_condition_kind_retains_its_native_encoded_type() {
    let input = every_kind();
    let lowered = lower_authoring_json(&input.to_string()).expect("five condition kinds");
    let binary = compile(&lowered.scene);
    let parsed = parse_riv(&binary, &InspectFilter::default()).expect("encoded scene");
    let count = |key| {
        parsed
            .objects
            .iter()
            .filter(|object| object.type_key == key)
            .count()
    };
    assert_eq!(count(type_keys::TRANSITION_VIEW_MODEL_CONDITION), 2);
    assert_eq!(count(type_keys::TRANSITION_BOOL_CONDITION), 1);
    assert_eq!(count(type_keys::TRANSITION_NUMBER_CONDITION), 1);
    assert_eq!(count(type_keys::TRANSITION_TRIGGER_CONDITION), 1);
}

#[test]
fn region_only_bindings_are_collected_and_shared_with_blend_consumers() {
    for with_blend in [false, true] {
        let mut input = every_kind();
        let chart = &mut input["behavior"]["statecharts"][0];
        chart["regions"] = json!([{
            "id": "secondary", "initial": "resting",
            "states": chart["states"].clone(), "transitions": chart["transitions"].clone()
        }]);
        chart["transitions"][0]["when"] = json!({ "input": "armed", "equals": true });
        if with_blend {
            chart["states"][0] = json!({
                "id": "resting", "blend": { "binding": "gate-level", "stops": [
                    { "motion": "rest-track", "value": scalar(0.0) },
                    { "motion": "active-track", "value": scalar(100.0) }
                ] }
            });
        }
        let lowered = lower_authoring_json(&input.to_string()).expect("region guard");
        let machine = &lowered.scene["artboard"]["state_machines"][0];
        assert_eq!(machine["inputs"].as_array().expect("inputs").len(), 5);
        let conditions = &machine["layers"][1]["transitions"][1]["conditions"];
        assert_eq!(conditions[1]["input"], machine["inputs"][0]["name"]);
        assert_eq!(conditions[3]["input"], machine["inputs"][1]["name"]);
        if with_blend {
            assert_eq!(
                machine["layers"][0]["states"][1]["input"],
                conditions[3]["input"]
            );
        }
        let source = lowered
            .source_map
            .entries
            .iter()
            .find(|entry| entry.authored_id == "gate/secondary/engage")
            .expect("region transition source");
        assert_eq!(
            source.scene_paths,
            ["/artboard/state_machines/0/layers/1/transitions/1"]
        );
        assert!(!compile(&lowered.scene).is_empty());
    }
}

#[test]
fn empty_and_oversized_groups_fail_on_json_and_typed_paths_in_each_scope() {
    for in_region in [false, true] {
        for count in [0, 1001] {
            let mut input = document();
            let guard = json!({ "all": vec![json!({ "input": "armed", "equals": true }); count] });
            let chart = &mut input["behavior"]["statecharts"][0];
            let path = if in_region {
                chart["regions"] = json!([{
                    "id": "secondary", "initial": "resting", "states": chart["states"].clone(),
                    "transitions": [{ "id": "go", "from": "resting", "to": "engaged", "when": guard }]
                }]);
                "$.behavior.statecharts[0].regions[0].transitions[0].when.all"
            } else {
                chart["transitions"][0]["when"] = guard;
                "$.behavior.statecharts[0].transitions[0].when.all"
            };
            let spec: AuthoringSpec =
                serde_json::from_value(input.clone()).expect("typed document");
            let typed = lower_authoring(&spec).expect_err("invalid group count");
            let from_json =
                lower_authoring_json(&input.to_string()).expect_err("invalid JSON count");
            assert_eq!(typed, from_json);
            assert_diagnostic(&input, "invalid_behavior_collection_count", path);
        }
    }
}

#[test]
fn a_thousand_conditions_are_supported_without_changing_the_group_limit() {
    let mut input = document();
    *input.pointer_mut(WHEN).expect("guard") = json!({
        "all": vec![json!({ "input": "armed", "equals": true }); 1000]
    });
    let lowered = lower_authoring_json(&input.to_string()).expect("upper boundary");
    assert_eq!(
        lowered.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["conditions"]
            .as_array()
            .expect("conditions")
            .len(),
        1000
    );
    assert!(!compile(&lowered.scene).is_empty());
    let schema = authoring_schema();
    let group = schema["$defs"]["BehaviorTransitionGuardSpec"]["anyOf"]
        .as_array()
        .expect("guard variants")
        .iter()
        .find(|variant| variant["properties"]["all"].is_object())
        .expect("all variant");
    assert_eq!(group["properties"]["all"]["minItems"], 1);
    assert_eq!(group["properties"]["all"]["maxItems"], 1000);
    assert_eq!(group["additionalProperties"], false);
}

#[test]
fn ambiguous_nested_and_unknown_guard_forms_are_rejected() {
    let leaf = json!({ "input": "armed", "equals": true });
    for guard in [
        json!({ "all": [leaf.clone()], "input": "armed", "equals": true }),
        json!({ "all": [{ "all": [leaf.clone()] }] }),
        json!({ "any": [leaf.clone()] }),
        json!({ "all": null }),
        json!({ "all": [{ "input": "armed", "equals": true, "extra": 1 }] }),
        json!({ "all": [leaf], "extra": 1 }),
    ] {
        let mut input = document();
        *input.pointer_mut(WHEN).expect("guard") = guard;
        assert_diagnostic(&input, "invalid_json", "$");
    }
}

#[test]
fn leaf_reference_errors_report_the_indexed_authored_path() {
    for (leaf, code, field) in [
        (
            json!({ "input": "missing", "equals": true }),
            "unknown_behavior_input",
            "input",
        ),
        (
            json!({ "input": "load", "equals": true }),
            "invalid_condition_input",
            "input",
        ),
        (
            json!({ "binding": "missing", "equals": true }),
            "unknown_behavior_binding",
            "binding",
        ),
        (
            json!({ "binding": "gate-enabled", "compare": "equal", "value": scalar(0.0) }),
            "invalid_condition_binding",
            "binding",
        ),
        (
            json!({ "trigger": "armed" }),
            "invalid_condition_input",
            "trigger",
        ),
    ] {
        let mut input = document();
        input.pointer_mut(WHEN).expect("guard")["all"][1] = leaf;
        assert_diagnostic(
            &input,
            code,
            &format!("$.behavior.statecharts[0].transitions[0].when.all[1].{field}"),
        );
    }
}

#[test]
fn numeric_guard_expressions_keep_parameters_timing_and_error_paths() {
    let mut input = document();
    input["parameters"] = json!({ "threshold": { "value": 75.0, "unit": "scalar" } });
    input.pointer_mut(WHEN).expect("guard")["all"][2]["value"] =
        json!({ "kind": "parameter", "name": "threshold" });
    let transition = &mut input["behavior"]["statecharts"][0]["transitions"][0];
    transition["duration_ms"] = scalar(250.0);
    transition["exit_time_ms"] = scalar(0.0);
    let lowered = lower_authoring_json(&input.to_string()).expect("parameter and timing");
    let emitted = &lowered.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1];
    assert_eq!(emitted["conditions"][2]["value"], 75.0);
    assert_eq!(emitted["duration"], 250);
    assert_eq!(emitted["exit_time"], 0);
    assert!(!compile(&lowered.scene).is_empty());
    input.pointer_mut(WHEN).expect("guard")["all"][2]["value"] =
        json!({ "kind": "parameter", "name": "missing" });
    assert_diagnostic(
        &input,
        "unknown_parameter",
        "$.behavior.statecharts[0].transitions[0].when.all[2].value.name",
    );
}

#[test]
fn a_bad_guard_rolls_back_a_statechart_operation_batch() {
    let input = document();
    let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed document");
    let before = lower_authoring(&spec).expect("original lowering");
    let snapshot = serde_json::to_value(&spec).expect("original document");
    let mut replacement = input["behavior"]["statecharts"][0].clone();
    replacement["transitions"][0]["when"]["all"][1] =
        json!({ "binding": "missing", "equals": true });
    let operations = [
        AuthoringOperation::Remove {
            target: AuthoringTarget::BehaviorStatechart {
                target_id: "gate".to_string(),
            },
        },
        AuthoringOperation::Insert {
            entity: AuthoringEntity::BehaviorStatechart(
                serde_json::from_value(replacement).expect("replacement chart"),
            ),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::BehaviorStatecharts,
            },
        },
    ];
    let error = apply_operations(&spec, &operations).expect_err("invalid batch");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "unknown_behavior_binding"
                && diagnostic.path
                    == "$.behavior.statecharts[0].transitions[0].when.all[1].binding")
    );
    assert_eq!(
        serde_json::to_value(&spec).expect("unchanged document"),
        snapshot
    );
    assert_eq!(lower_authoring(&spec).expect("unchanged lowering"), before);
}

#[test]
fn oversized_transition_collections_fail_before_nested_guards() {
    for in_region in [false, true] {
        let mut input = document();
        let many = json!(vec![
            json!({
                "id": "go", "from": "resting", "to": "engaged", "when": { "all": [] }
            });
            1001
        ]);
        let chart = &mut input["behavior"]["statecharts"][0];
        let path = if in_region {
            chart["regions"] = json!([{
                "id": "secondary", "initial": "resting",
                "states": chart["states"].clone(), "transitions": many
            }]);
            "$.behavior.statecharts[0].regions[0].transitions"
        } else {
            chart["transitions"] = many;
            "$.behavior.statecharts[0].transitions"
        };
        let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed document");
        let typed = lower_authoring(&spec).expect_err("oversized outer collection");
        let from_json = lower_authoring_json(&input.to_string()).expect_err("same JSON boundary");
        assert_eq!(typed, from_json);
        assert_eq!(
            typed.diagnostics[0].code,
            "invalid_behavior_collection_count"
        );
        assert_eq!(typed.diagnostics[0].path, path);
    }
}
