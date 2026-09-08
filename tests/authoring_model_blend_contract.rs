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

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("valid bound blend")
}

fn compile(scene: &Value) -> Vec<u8> {
    let scene = serde_json::from_value(scene.clone()).expect("canonical scene");
    compile_scene(&scene, None, 0).expect("binary compilation")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid bound blend");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|entry| entry.code == code && entry.path == path),
        "{error:?}"
    );
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

#[test]
fn model_binding_sources_are_exclusive_and_reject_runtime_indices() {
    for child in [
        json!({"motion":"left-track"}),
        json!({"motion":"left-track", "input":"left-weight", "binding":"left-model"}),
        json!({"motion":"left-track", "binding":null}),
        json!({"motion":"left-track", "binding":"left-model", "input_id":0}),
        json!({"motion":"left-track", "binding":"left-model", "weight":50}),
    ] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"][1] = child;
        assert_diagnostic(&input, "invalid_json", "$");
    }
}

#[test]
fn model_blend_reference_errors_keep_authored_paths() {
    let child = "/behavior/statecharts/0/states/0/direct_blend/motions/1";
    for (pointer, path, code) in [
        (
            format!("{child}/binding"),
            "$.behavior.statecharts[0].states[0].direct_blend.motions[1].binding",
            "unknown_behavior_binding",
        ),
        (
            format!("{child}/motion"),
            "$.behavior.statecharts[0].states[0].direct_blend.motions[1].motion",
            "unknown_behavior_motion",
        ),
        (
            "/behavior/bindings/0/model".to_string(),
            "$.behavior.bindings[0].model",
            "unknown_behavior_model",
        ),
        (
            "/behavior/bindings/0/property".to_string(),
            "$.behavior.bindings[0].property",
            "unknown_behavior_property",
        ),
    ] {
        let mut input = document();
        *input.pointer_mut(&pointer).expect("reference") = json!("missing");
        assert_diagnostic(&input, code, path);
    }
    let mut input = document();
    input["behavior"]["models"][0]["properties"][0] =
        json!({"kind":"bool", "id":"left", "value":false});
    assert_diagnostic(
        &input,
        "invalid_blend_binding",
        "$.behavior.statecharts[0].states[0].direct_blend.motions[1].binding",
    );
}

#[test]
fn a_region_only_model_binding_is_emitted_and_resolves_the_region_path() {
    let mut input = document();
    let state = input["behavior"]["statecharts"][0]["states"][0].clone();
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["states"][0] = json!({"id":"blending", "motion":"rest-track"});
    chart["regions"] = json!([{"id":"secondary", "initial":"blending", "states":[state]}]);
    let output = lower(&input);
    let machine = &output.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"].as_array().expect("inputs").len(), 6);
    assert_eq!(
        machine["layers"][1]["states"][1]["children"][1]["input_id"],
        0
    );
    let source = output
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "panel/secondary/blending")
        .expect("region source");
    assert_eq!(
        source.scene_paths,
        ["/artboard/state_machines/0/layers/1/states/1"]
    );
    compile(&output.scene);
    input["behavior"]["models"][0]["properties"][0] =
        json!({"kind":"bool", "id":"left", "value":false});
    assert_diagnostic(
        &input,
        "invalid_blend_binding",
        "$.behavior.statecharts[0].regions[0].states[0].direct_blend.motions[1].binding",
    );
}

#[test]
fn transition_root_and_region_consumers_share_one_synthesized_input() {
    let mut input = document();
    let state = input["behavior"]["statecharts"][0]["states"][0].clone();
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["regions"] = json!([{"id":"secondary", "initial":"blending", "states":[state]}]);
    chart["transitions"][0]["when"] = json!({"binding":"left-model", "compare":"greater_or_equal", "value":{"kind":"literal", "value":60, "unit":"scalar"}});
    let output = lower(&input);
    let machine = &output.scene["artboard"]["state_machines"][0];
    assert_eq!(machine["inputs"].as_array().expect("inputs").len(), 6);
    for layer in [0, 1] {
        assert_eq!(
            machine["layers"][layer]["states"][1]["children"][1]["input_id"],
            0
        );
    }
    let source = output
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "left-model")
        .expect("binding source");
    assert_eq!(source.scene_paths, ["/artboard/state_machines/0/inputs/0"]);
    let parsed =
        parse_riv(&compile(&output.scene), &InspectFilter::default()).expect("encoded scene");
    assert_eq!(
        parsed
            .objects
            .iter()
            .filter(|object| object.type_key == type_keys::DATA_BIND_CONTEXT)
            .count(),
        3
    );
}

#[test]
fn shared_model_bindings_resolve_chart_local_input_offsets() {
    let mut input = document();
    let mut second = input["behavior"]["statecharts"][0].clone();
    second["id"] = json!("other-panel");
    input["behavior"]["statecharts"]
        .as_array_mut()
        .expect("charts")
        .push(second);
    input["behavior"]["models"][0]["properties"]
        .as_array_mut()
        .expect("properties")
        .push(json!({"kind":"bool", "id":"enabled", "value":false}));
    input["behavior"]["bindings"]
        .as_array_mut()
        .expect("bindings")
        .insert(
            0,
            json!({"id":"enabled-model", "model":"weights", "property":"enabled"}),
        );
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] =
        json!({"binding":"enabled-model", "equals":true});
    let output = lower(&input);
    let machines = &output.scene["artboard"]["state_machines"];
    assert_eq!(
        machines[0]["layers"][0]["states"][1]["children"][1]["input_id"],
        1
    );
    assert_eq!(
        machines[1]["layers"][0]["states"][1]["children"][1]["input_id"],
        0
    );
    let source = output
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "left-model")
        .expect("shared source");
    assert_eq!(
        source.scene_paths,
        [
            "/artboard/state_machines/0/inputs/1",
            "/artboard/state_machines/1/inputs/0"
        ]
    );
    for entry in &output.source_map.entries {
        for pointer in &entry.scene_paths {
            assert!(output.scene.pointer(pointer).is_some(), "{pointer}");
        }
    }
    compile(&output.scene);
    assert_eq!(output, lower(&input));
}

#[test]
fn native_blend_binding_counts_model_properties_and_precedes_its_consumer() {
    use rive_cli::builder::build_scene;
    use rive_cli::objects::core::PropertyValue;
    let mut input = document();
    let left = input["behavior"]["models"][0]["properties"][0].clone();
    let mut properties: Vec<_> = (0..128)
        .map(|index| json!({"kind":"bool", "id":format!("unused-{index}"), "value":false}))
        .collect();
    properties.push(left);
    input["behavior"]["models"][0]["properties"] = json!(properties);
    input["behavior"]["models"]
        .as_array_mut()
        .expect("models")
        .insert(
            0,
            json!({"id":"earlier", "properties":[{"kind":"bool", "id":"unused", "value":false}]}),
        );
    let scene: SceneSpec = serde_json::from_value(lower(&input).scene).expect("canonical scene");
    let objects = build_scene(&scene, None).expect("build scene");
    let context = objects
        .iter()
        .position(|object| object.type_key() == type_keys::DATA_BIND_CONTEXT)
        .expect("binding context");
    assert_eq!(
        objects[context - 1].type_key(),
        type_keys::BINDABLE_PROPERTY_NUMBER
    );
    assert_eq!(
        objects[context + 1].type_key(),
        type_keys::BLEND_ANIMATION_DIRECT
    );
    assert!(objects[context].properties().iter().any(|field| field.key
        == property_keys::DATA_BIND_CONTEXT_SOURCE_PATH_IDS
        && field.value == PropertyValue::Bytes(vec![1, 128, 1])));
    assert!(
        objects[context - 1]
            .properties()
            .iter()
            .any(
                |field| field.key == property_keys::BINDABLE_PROPERTY_NUMBER_VALUE
                    && field.value == PropertyValue::Float(25.0)
            )
    );
    compile_scene(&scene, None, 0).expect("multibyte model binding compiles");
}

#[test]
fn an_explicit_canonical_fixed_weight_is_not_reinterpreted_as_a_model_binding() {
    let mut scene = lower(&document()).scene;
    let child =
        &mut scene["artboard"]["state_machines"][0]["layers"][0]["states"][1]["children"][1];
    child["blend_source"] = json!(1);
    child["mix_value"] = json!(40);
    let parsed =
        parse_riv(&compile(&scene), &InspectFilter::default()).expect("encoded fixed blend");
    assert!(
        !parsed
            .objects
            .iter()
            .any(|object| object.type_key == type_keys::DATA_BIND_CONTEXT)
    );
    let child = parsed
        .objects
        .iter()
        .filter(|object| object.type_key == type_keys::BLEND_ANIMATION_DIRECT)
        .nth(1)
        .expect("fixed child");
    assert!(child.properties.iter().any(|field| field.key
        == property_keys::BLEND_ANIMATION_DIRECT_BLEND_SOURCE
        && field.value == PropertyValueRead::UInt(1)));
    assert!(child.properties.iter().any(|field| field.key
        == property_keys::BLEND_ANIMATION_DIRECT_MIX_VALUE
        && field.value == PropertyValueRead::Float(40.0)));
}

#[test]
fn input_only_direct_blends_retain_their_pre_model_binding_binary() {
    use sha2::{Digest, Sha256};
    let input: Value = serde_json::from_str(include_str!(
        "../examples/authoring/direct-blend-panel.v0.json"
    ))
    .expect("input-driven example");
    let bytes = compile(&lower(&input).scene);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "5f2ae61031eb5c0474e66d87f66c30678825b467fa9b15e82504463074b55f99"
    );
}

#[test]
fn a_failed_statechart_edit_preserves_model_blend_source_identity() {
    use rive_cli::authoring::{
        AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec,
        AuthoringTarget, apply_operations, lower_authoring,
    };
    let input = document();
    let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed source");
    let snapshot = serde_json::to_value(&spec).expect("snapshot");
    let before = lower_authoring(&spec).expect("initial lowered source");
    let mut replacement = input["behavior"]["statecharts"][0].clone();
    replacement["states"][0]["direct_blend"]["motions"][1]["binding"] = json!("missing");
    let operations = [
        AuthoringOperation::Remove {
            target: AuthoringTarget::BehaviorStatechart {
                target_id: "panel".to_string(),
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
        apply_operations(&spec, &operations).expect_err("invalid model source must roll back");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|entry| entry.code == "unknown_behavior_binding")
    );
    assert_eq!(serde_json::to_value(&spec).expect("unchanged"), snapshot);
    assert_eq!(lower_authoring(&spec).expect("still valid"), before);
}
