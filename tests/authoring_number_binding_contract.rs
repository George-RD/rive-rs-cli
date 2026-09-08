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
        "kind": "number", "id": "enabled",
        "value": { "kind": "literal", "value": 25.0, "unit": "scalar" }
    });
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] = json!({
        "binding": "gate-enabled", "compare": "greater_or_equal",
        "value": { "kind": "literal", "value": 60.0, "unit": "scalar" }
    });
    input
}

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("numeric binding must lower")
}

fn compile(scene: &Value) -> Vec<u8> {
    let scene: SceneSpec = serde_json::from_value(scene.clone()).expect("canonical scene");
    compile_scene(&scene, None, 0).expect("canonical scene must compile")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid binding document");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == code && diagnostic.path == path
    }), "{error:?}");
}

fn assert_binding_type_error(input: &Value) {
    assert_diagnostic(input, "invalid_condition_binding", "$.behavior.statecharts[0].transitions[0].when.binding");
}

#[test]
fn numeric_view_model_binding_compiles_to_a_model_condition_not_an_input_condition() {
    let lowered = lower(&document());
    let model = &lowered.scene["artboard"]["children"][1];
    assert_eq!(model["children"][0]["type"], "view_model_property_number");
    let machine = &lowered.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"][0]["type"], "number");
    assert_eq!(machine["inputs"][0]["value"], 25.0);
    assert_eq!(machine["inputs"][0]["view_model_binding"]["property"], model["children"][0]["name"]);
    let condition = &machine["layers"][0]["transitions"][1]["conditions"][0];
    assert_eq!(condition["op"], ">=");
    assert_eq!(condition["value"], 60.0);
    let parsed = parse_riv(&compile(&lowered.scene), &InspectFilter::default()).expect("encoded scene");
    let types: Vec<_> = parsed.objects.iter().map(|object| object.type_key).collect();
    assert!(types.contains(&type_keys::TRANSITION_VIEW_MODEL_CONDITION));
    assert!(types.contains(&type_keys::BINDABLE_PROPERTY_NUMBER));
    assert!(types.contains(&type_keys::TRANSITION_VALUE_NUMBER_COMPARATOR));
    assert!(!types.contains(&type_keys::TRANSITION_NUMBER_CONDITION));
}

#[test]
fn numeric_binding_rejects_a_boolean_property_at_the_authored_condition() {
    let mut input = document();
    input["behavior"]["models"][0]["properties"][0] = json!({
        "kind": "bool", "id": "enabled", "value": false
    });
    assert_binding_type_error(&input);
}

#[test]
fn boolean_binding_rejects_a_numeric_property_at_the_authored_condition() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] = json!({
        "binding": "gate-enabled", "equals": true
    });
    assert_binding_type_error(&input);
}

#[test]
fn canonical_number_binding_rejects_a_non_number_model_property() {
    let mut scene = lower(&document()).scene;
    scene["artboard"]["children"][1]["children"][0]["type"] = json!("view_model_property_boolean");
    let scene: SceneSpec = serde_json::from_value(scene).expect("canonical scene");
    let error = compile_scene(&scene, None, 0).expect_err("binding must name a number property");
    assert!(error.to_string().contains("number"), "{error}");
}

#[test]
fn canonical_number_binding_rejects_non_numeric_conditions() {
    let scene = lower(&document()).scene;
    for value in [json!(true), Value::Null] {
        let mut invalid = scene.clone();
        invalid["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["conditions"][0]["value"] = value;
        let invalid: SceneSpec = serde_json::from_value(invalid).expect("canonical scene");
        let error = compile_scene(&invalid, None, 0).expect_err("bound number condition must be numeric");
        assert!(error.to_string().contains("number"), "{error}");
    }
}

#[test]
fn canonical_number_binding_rejects_values_that_overflow_runtime_floats() {
    for pointer in [
        "/artboard/state_machines/0/inputs/0/value",
        "/artboard/state_machines/0/layers/0/transitions/1/conditions/0/value",
    ] {
        let mut scene = lower(&document()).scene;
        *scene.pointer_mut(pointer).expect("numeric field") = json!(f64::MAX);
        let scene: SceneSpec = serde_json::from_value(scene).expect("canonical scene");
        let error = compile_scene(&scene, None, 0).expect_err("runtime float must remain finite");
        assert!(error.to_string().contains("finite"), "{pointer}: {error}");
    }
}

#[test]
fn numeric_model_property_uses_the_runtime_view_model_name_field() {
    use rive_cli::objects::core::property_keys;
    use rive_cli::validator::PropertyValueRead;
    let scene = lower(&document()).scene;
    let expected_name = scene["artboard"]["children"][1]["children"][0]["name"].as_str().expect("name");
    let parsed = parse_riv(&compile(&scene), &InspectFilter::default()).expect("encoded scene");
    let property = parsed.objects.iter().find(|object| object.type_key == type_keys::VIEW_MODEL_PROPERTY_NUMBER).expect("model property");
    assert!(property.properties.iter().any(|field| field.key == property_keys::VIEW_MODEL_COMPONENT_NAME
        && field.value == PropertyValueRead::String(expected_name.to_string())));
    assert!(!property.properties.iter().any(|field| matches!(field.key, property_keys::COMPONENT_NAME | property_keys::COMPONENT_PARENT_ID)));
}

#[test]
fn every_numeric_binding_comparison_keeps_its_encoded_operator_and_value() {
    use rive_cli::objects::core::property_keys;
    use rive_cli::validator::PropertyValueRead;
    for (comparison, operator, encoded) in [
        ("equal", "==", 0), ("not_equal", "!=", 1), ("greater", ">", 2),
        ("greater_or_equal", ">=", 3), ("less", "<", 4), ("less_or_equal", "<=", 5),
    ] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["transitions"][0]["when"]["compare"] = json!(comparison);
        let first = lower(&input);
        let second = lower(&input);
        assert_eq!(first.scene, second.scene);
        assert_eq!(first.source_map, second.source_map);
        assert_eq!(compile(&first.scene), compile(&second.scene));
        assert_eq!(first.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["conditions"][0]["op"], operator);
        let parsed = parse_riv(&compile(&first.scene), &InspectFilter::default()).expect("encoded scene");
        let condition = parsed.objects.iter().find(|object| object.type_key == type_keys::TRANSITION_VIEW_MODEL_CONDITION).expect("condition");
        let actual = condition.properties.iter().find(|field| field.key == property_keys::TRANSITION_VIEW_MODEL_CONDITION_OP_VALUE)
            .map(|field| field.value.clone()).unwrap_or(PropertyValueRead::UInt(0));
        assert_eq!(actual, PropertyValueRead::UInt(encoded), "{comparison}");
        let comparator = parsed.objects.iter().find(|object| object.type_key == type_keys::TRANSITION_VALUE_NUMBER_COMPARATOR).expect("comparator");
        assert!(comparator.properties.iter().any(|field| field.key == property_keys::TRANSITION_VALUE_NUMBER_COMPARATOR_VALUE && field.value == PropertyValueRead::Float(60.0)));
    }
}

#[test]
fn model_defaults_and_thresholds_use_document_parameters() {
    let input: Value = serde_json::from_str(include_str!("../examples/authoring/number-binding.v0.json")).expect("parameterized example");
    let output = lower(&input);
    let machine = &output.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"][0]["value"], 25.0);
    assert_eq!(machine["layers"][0]["transitions"][1]["conditions"][0]["value"], 60.0);
    assert_eq!(machine["layers"][0]["transitions"][2]["conditions"][0]["value"], 60.0);
    compile(&output.scene);
}

#[test]
fn numeric_expression_errors_keep_model_and_condition_authored_paths() {
    for (pointer, path) in [
        ("/behavior/models/0/properties/0/value", "$.behavior.models[0].properties[0].value"),
        ("/behavior/statecharts/0/transitions/0/when/value", "$.behavior.statecharts[0].transitions[0].when.value"),
    ] {
        for (expression, code, suffix) in [
            (json!({"kind": "parameter", "name": "missing"}), "unknown_parameter", ".name"),
            (json!({"kind": "literal", "value": 5.0, "unit": "px"}), "unit_mismatch", ""),
            (json!({"kind": "literal", "value": f64::MAX, "unit": "scalar"}), "numeric_out_of_range", ".value"),
            (json!({"kind": "literal", "value": 1e-100, "unit": "scalar"}), "numeric_out_of_range", ".value"),
        ] {
            let mut input = document();
            *input.pointer_mut(pointer).expect("expression") = expression;
            assert_diagnostic(&input, code, &format!("{path}{suffix}"));
        }
    }
    let mut input = document();
    input["behavior"]["models"][0]["properties"][0]["value"] = json!({"kind": "parameter", "name": "missing"});
    input["behavior"]["bindings"] = json!([]);
    input["behavior"]["statecharts"] = json!([]);
    assert_diagnostic(&input, "unknown_parameter", "$.behavior.models[0].properties[0].value.name");
}

#[test]
fn numeric_binding_reference_errors_are_authored_and_specific() {
    for (pointer, path, code) in [
        ("/behavior/bindings/0/model", "$.behavior.bindings[0].model", "unknown_behavior_model"),
        ("/behavior/bindings/0/property", "$.behavior.bindings[0].property", "unknown_behavior_property"),
        ("/behavior/statecharts/0/transitions/0/when/binding", "$.behavior.statecharts[0].transitions[0].when.binding", "unknown_behavior_binding"),
    ] {
        let mut input = document();
        *input.pointer_mut(pointer).expect("reference") = json!("missing");
        assert_diagnostic(&input, code, path);
    }
    for field in ["view_model", "property"] {
        let mut scene = lower(&document()).scene;
        scene["artboard"]["state_machines"][0]["inputs"][0]["view_model_binding"][field] = json!("missing");
        let scene: SceneSpec = serde_json::from_value(scene).expect("canonical scene");
        assert!(compile_scene(&scene, None, 0).is_err(), "{field}");
    }
}

#[test]
fn numeric_bindings_are_shared_within_a_chart_and_scoped_across_charts() {
    let mut input = document();
    let mut region = input["behavior"]["statecharts"][0].clone();
    region["id"] = json!("parallel");
    input["behavior"]["statecharts"][0]["regions"] = json!([region]);
    let mut other = input["behavior"]["statecharts"][0].clone();
    other["id"] = json!("other");
    input["behavior"]["statecharts"].as_array_mut().expect("charts").push(other);
    let output = lower(&input);
    let again = lower(&input);
    assert_eq!(output.scene, again.scene);
    assert_eq!(output.source_map, again.source_map);
    let machines = output.scene["artboard"]["state_machines"].as_array().expect("machines");
    assert_eq!(machines.len(), 2);
    assert_ne!(machines[0]["inputs"][0]["name"], machines[1]["inputs"][0]["name"]);
    for machine in machines {
        assert_eq!(machine["inputs"].as_array().expect("inputs").len(), 1);
        for layer in machine["layers"].as_array().expect("layers") {
            assert_eq!(layer["transitions"][1]["conditions"][0]["input"], machine["inputs"][0]["name"]);
        }
    }
    let binding = output.source_map.entries.iter().find(|entry| entry.authored_id == "gate-enabled").expect("binding");
    assert_eq!(binding.scene_paths.len(), 2);
    for entry in &output.source_map.entries {
        for pointer in &entry.scene_paths {
            assert!(output.scene.pointer(pointer).is_some(), "{pointer}");
        }
    }
    compile(&output.scene);
    input["behavior"]["statecharts"][0]["regions"][0]["transitions"][0]["when"] = json!({"binding": "gate-enabled", "equals": true});
    assert_diagnostic(&input, "invalid_condition_binding", "$.behavior.statecharts[0].regions[0].transitions[0].when.binding");
}

#[test]
fn numeric_binding_ids_count_models_and_properties_not_visual_objects() {
    use rive_cli::builder::build_scene;
    use rive_cli::objects::core::{PropertyValue, property_keys};
    let mut input = document();
    let model = &mut input["behavior"]["models"][0];
    let enabled = model["properties"][0].clone();
    let mut properties: Vec<_> = (0..128).map(|index| json!({"kind": "bool", "id": format!("unused-{index}"), "value": false})).collect();
    properties.push(enabled);
    model["properties"] = json!(properties);
    input["behavior"]["models"].as_array_mut().expect("models").insert(0, json!({"id": "earlier", "properties": [{"kind": "bool", "id": "unused", "value": false}]}));
    let scene: SceneSpec = serde_json::from_value(lower(&input).scene).expect("canonical scene");
    let objects = build_scene(&scene, None).expect("builder");
    let paths: Vec<_> = objects.iter().filter(|object| object.type_key() == type_keys::DATA_BIND_CONTEXT)
        .flat_map(|object| object.properties()).filter(|property| property.key == property_keys::DATA_BIND_CONTEXT_SOURCE_PATH_IDS)
        .map(|property| property.value).collect();
    assert_eq!(paths, vec![PropertyValue::Bytes(vec![1, 128, 1])]);
    compile_scene(&scene, None, 0).expect("encoded binding with multibyte property index");
}

#[test]
fn unbound_number_and_boolean_conditions_keep_their_existing_object_families() {
    let mut unbound = lower(&document()).scene;
    unbound["artboard"]["state_machines"][0]["inputs"][0].as_object_mut().expect("input").remove("view_model_binding");
    let parsed = parse_riv(&compile(&unbound), &InspectFilter::default()).expect("number input");
    assert!(parsed.objects.iter().any(|object| object.type_key == type_keys::TRANSITION_NUMBER_CONDITION));
    assert!(!parsed.objects.iter().any(|object| object.type_key == type_keys::TRANSITION_VIEW_MODEL_CONDITION));
    let boolean: Value = serde_json::from_str(include_str!("../examples/authoring/behavior-binding.v0.json")).expect("boolean example");
    let parsed = parse_riv(&compile(&lower(&boolean).scene), &InspectFilter::default()).expect("boolean binding");
    assert!(parsed.objects.iter().any(|object| object.type_key == type_keys::BINDABLE_PROPERTY_BOOLEAN));
    assert!(parsed.objects.iter().any(|object| object.type_key == type_keys::TRANSITION_VALUE_BOOLEAN_COMPARATOR));
    assert!(!parsed.objects.iter().any(|object| object.type_key == type_keys::BINDABLE_PROPERTY_NUMBER));
}

#[test]
fn invalid_numeric_binding_replacement_is_atomic() {
    use rive_cli::authoring::{AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec, AuthoringTarget, apply_operations, lower_authoring};
    let spec: AuthoringSpec = serde_json::from_value(document()).expect("authoring");
    let snapshot = serde_json::to_value(&spec).expect("snapshot");
    let before = lower_authoring(&spec).expect("original");
    let mut replacement = document()["behavior"]["statecharts"][0].clone();
    replacement["transitions"][0]["when"] = json!({"binding": "gate-enabled", "equals": true});
    let operations = [
        AuthoringOperation::Remove { target: AuthoringTarget::BehaviorStatechart { target_id: "gate".to_string() } },
        AuthoringOperation::Insert {
            entity: AuthoringEntity::BehaviorStatechart(serde_json::from_value(replacement).expect("replacement")),
            placement: AuthoringPlacement::Into { container: AuthoringContainer::BehaviorStatecharts },
        },
    ];
    let error = apply_operations(&spec, &operations).expect_err("mismatched binding must roll back");
    assert!(error.diagnostics.iter().any(|diagnostic| diagnostic.code == "invalid_condition_binding"));
    assert_eq!(serde_json::to_value(&spec).expect("unchanged"), snapshot);
    let after = lower_authoring(&spec).expect("still valid");
    assert_eq!(before.scene, after.scene);
    assert_eq!(before.source_map, after.source_map);
}
