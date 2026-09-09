use rive_cli::authoring::lower_authoring_json;
use rive_cli::compile::compile_scene;
use rive_cli::objects::core::{property_keys, type_keys};
use rive_cli::validator::{InspectFilter, PropertyValueRead, parse_riv};
use serde_json::{Value, json};

const WEIGHT_PATH: &str = "$.behavior.statecharts[0].states[0].direct_blend.motions[1].weight";

fn document() -> Value {
    let mut input: Value = serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json")).expect("authored fixture");
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["inputs"] = json!([]);
    chart["states"][0] = json!({
        "id": "reading",
        "direct_blend": { "motions": [
            { "motion": "calm-track", "weight": literal(100.0) },
            { "motion": "surge-track", "weight": literal(25.0) }
        ] }
    });
    input
}

fn literal(value: f64) -> Value {
    json!({"kind":"literal", "value":value, "unit":"scalar"})
}

fn set_weight(input: &mut Value, weight: Value) {
    input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"][1]["weight"] = weight;
}

fn compiled(scene: &Value) -> Vec<u8> {
    compile_scene(&serde_json::from_value(scene.clone()).expect("canonical scene"), None, 0)
        .expect("binary compilation")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid fixed weight");
    assert!(error.diagnostics.iter().any(|entry| entry.code == code && entry.path == path), "{error:?}");
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
    assert_eq!(children, &json!([
        { "type": "blend_animation_direct", "animation_id": 0, "blend_source": 1, "mix_value": 100.0 },
        { "type": "blend_animation_direct", "animation_id": 1, "blend_source": 1, "mix_value": 25.0 }
    ]));
    let parsed = parse_riv(&compiled(&first.scene), &InspectFilter::default()).expect("encoded fixed blend");
    assert!(!parsed.objects.iter().any(|object| matches!(object.type_key,
        type_keys::STATE_MACHINE_NUMBER | type_keys::DATA_BIND_CONTEXT
    )));
    let direct = parsed.objects.iter().filter(|object| object.type_key == type_keys::BLEND_ANIMATION_DIRECT).collect::<Vec<_>>();
    assert_eq!(direct.len(), 2);
    for child in &direct {
        assert!(child.properties.iter().any(|field| field.key == property_keys::BLEND_ANIMATION_DIRECT_BLEND_SOURCE && field.value == PropertyValueRead::UInt(1)));
    }
    assert!(direct[1].properties.iter().any(|field| field.key == property_keys::BLEND_ANIMATION_DIRECT_MIX_VALUE && field.value == PropertyValueRead::Float(25.0)));
}

#[test]
fn fixed_weights_reject_out_of_range_percentages_before_float_narrowing() {
    for value in [-0.001, 100.000001, 101.0] {
        let mut input = document();
        set_weight(&mut input, literal(value));
        assert_diagnostic(&input, "invalid_blend_weight", WEIGHT_PATH);
    }
}

#[test]
fn fixed_percentages_accept_boundaries_fractions_and_parameter_arithmetic() {
    for value in [0.0, -0.0, 0.25, 25.5, 99.999999, 100.0] {
        let mut input = document();
        input["parameters"] = json!({"contribution":{"value":value, "unit":"scalar"}});
        set_weight(&mut input, json!({"kind":"divide", "value":{
            "kind":"multiply", "value":{"kind":"parameter", "name":"contribution"}, "factor":2
        }, "divisor":2}));
        let lowered = lower_authoring_json(&input.to_string()).expect("valid fixed percentage");
        assert_eq!(lowered.scene["artboard"]["state_machines"][0]["layers"][0]["states"][1]["children"][1]["mix_value"], json!(value));
        let parsed = parse_riv(&compiled(&lowered.scene), &InspectFilter::default()).expect("valid binary");
        let direct = parsed.objects.iter().filter(|object| object.type_key == type_keys::BLEND_ANIMATION_DIRECT).nth(1).expect("weighted motion");
        let encoded = direct.properties.iter().find(|field| field.key == property_keys::BLEND_ANIMATION_DIRECT_MIX_VALUE)
            .map(|field| field.value.clone()).unwrap_or(PropertyValueRead::Float(100.0));
        assert_eq!(encoded, PropertyValueRead::Float(value as f32));
    }
}

#[test]
fn fixed_sources_are_exclusive_and_the_published_schema_keeps_each_form_strict() {
    use rive_cli::authoring::authoring_schema;
    for child in [
        json!({"motion":"surge-track", "weight":null}),
        json!({"motion":"surge-track", "weight":25}),
        json!({"motion":"surge-track", "weight":literal(25.0), "input":"load"}),
        json!({"motion":"surge-track", "weight":literal(25.0), "binding":"model"}),
        json!({"motion":"surge-track", "weight":literal(25.0), "input_id":0}),
        json!({"motion":"surge-track", "weight":literal(25.0), "blend_source":1}),
        json!({"weight":literal(25.0)}),
    ] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"][1] = child;
        assert_diagnostic(&input, "invalid_json", "$");
    }
    let schema = authoring_schema();
    let forms = schema["$defs"]["BehaviorDirectBlendMotionSpec"]["anyOf"].as_array().expect("exclusive schema forms");
    assert_eq!(forms.len(), 3);
    for (form, source) in forms.iter().zip(["input", "binding", "weight"]) {
        assert_eq!(form["additionalProperties"], false);
        assert_eq!(form["required"], json!(["motion", source]));
    }
}

#[test]
fn invalid_fixed_expressions_retain_precise_authored_diagnostics() {
    for (expression, code, suffix) in [
        (json!({"kind":"literal", "value":25, "unit":"px"}), "unit_mismatch", ""),
        (json!({"kind":"parameter", "name":"missing"}), "unknown_parameter", ".name"),
        (json!({"kind":"divide", "value":literal(25.0), "divisor":0}), "division_by_zero", ".divisor"),
        (literal(1e100), "numeric_out_of_range", ".value"),
        (literal(1e-100), "numeric_out_of_range", ".value"),
    ] {
        let mut input = document();
        set_weight(&mut input, expression);
        assert_diagnostic(&input, code, &format!("{WEIGHT_PATH}{suffix}"));
    }
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"][1]["motion"] = json!("missing");
    assert_diagnostic(&input, "unknown_behavior_motion", "$.behavior.statecharts[0].states[0].direct_blend.motions[1].motion");
}

#[test]
fn programmatic_non_finite_parameters_cannot_reach_fixed_weight_encoding() {
    use rive_cli::authoring::{AuthoringSpec, Quantity, Unit, lower_authoring};
    let mut input = document();
    set_weight(&mut input, json!({"kind":"parameter", "name":"contribution"}));
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed input");
        spec.parameters.insert("contribution".to_string(), Quantity { value, unit: Unit::Scalar });
        let error = lower_authoring(&spec).expect_err("non-finite percentage");
        assert!(error.diagnostics.iter().any(|entry| entry.code == "non_finite"), "{error:?}");
    }
}

#[test]
fn fixed_weights_in_regions_share_parameters_and_preserve_source_identity() {
    let mut input = document();
    input["parameters"] = json!({"contribution":{"value":62.5, "unit":"scalar"}});
    let mut region = input["behavior"]["statecharts"][0].clone();
    region.as_object_mut().expect("region").remove("inputs");
    region["id"] = json!("overlay");
    region["states"][0]["direct_blend"]["motions"][1]["weight"] = json!({"kind":"parameter", "name":"contribution"});
    input["behavior"]["statecharts"][0]["regions"] = json!([region]);
    let first = lower_authoring_json(&input.to_string()).expect("scoped fixed blend");
    assert_eq!(first, lower_authoring_json(&input.to_string()).expect("repeat lowering"));
    assert_eq!(first.scene["artboard"]["state_machines"][0]["layers"][1]["states"][1]["children"][1]["mix_value"], 62.5);
    let source = first.source_map.entries.iter().find(|entry| entry.authored_id == "meter/overlay/reading").expect("region state source");
    assert_eq!(source.scene_paths, ["/artboard/state_machines/0/layers/1/states/1"]);
    for entry in &first.source_map.entries {
        for path in &entry.scene_paths { assert!(first.scene.pointer(path).is_some(), "{path}"); }
    }
    compiled(&first.scene);
    input["behavior"]["statecharts"][0]["regions"][0]["states"][0]["direct_blend"]["motions"][1]["weight"] = literal(100.000001);
    assert_diagnostic(&input, "invalid_blend_weight", "$.behavior.statecharts[0].regions[0].states[0].direct_blend.motions[1].weight");
}

#[test]
fn fixed_input_and_model_sources_keep_real_indices_across_charts() {
    let mut input: Value = serde_json::from_str(include_str!("../examples/authoring/model-blend-panel.v0.json")).expect("model panel");
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["inputs"][0]["id"] = json!("left-control");
    chart["states"][0]["direct_blend"]["motions"][0] = json!({"motion":"rest-track", "weight":literal(100.0)});
    chart["states"][0]["direct_blend"]["motions"][1] = json!({"motion":"left-track", "input":"left-control"});
    let mut earlier = chart.clone();
    earlier["id"] = json!("earlier");
    earlier["states"][0]["direct_blend"]["motions"][2] = json!({"motion":"right-track", "weight":literal(25.0)});
    input["behavior"]["statecharts"].as_array_mut().expect("charts").insert(0, earlier);
    let output = lower_authoring_json(&input.to_string()).expect("mixed sources");
    let machines = &output.scene["artboard"]["state_machines"];
    assert_eq!(machines[0]["inputs"].as_array().expect("earlier inputs").len(), 3);
    assert_eq!(machines[1]["inputs"].as_array().expect("bound inputs").len(), 4);
    for index in 0..2 {
        let children = &machines[index]["layers"][0]["states"][1]["children"];
        assert_eq!(children[0]["blend_source"], 1);
        assert!(children[0].get("input_id").is_none());
        assert_eq!(children[1]["input_id"], index);
    }
    assert_eq!(machines[1]["layers"][0]["states"][1]["children"][2]["input_id"], 0);
    let binding = output.source_map.entries.iter().find(|entry| entry.authored_id == "right-model").expect("used model binding");
    assert_eq!(binding.scene_paths, ["/artboard/state_machines/1/inputs/0"]);
    assert!(!output.source_map.entries.iter().any(|entry| entry.authored_id == "left-model"));
    let parsed = parse_riv(&compiled(&output.scene), &InspectFilter::default()).expect("mixed binary");
    assert_eq!(parsed.objects.iter().filter(|object| object.type_key == type_keys::DATA_BIND_CONTEXT).count(), 1);
}

#[test]
fn fixed_child_order_remains_authored_and_does_not_rewrite_source_identity() {
    let mut input = document();
    let before = lower_authoring_json(&input.to_string()).expect("original order");
    input["behavior"]["statecharts"][0]["states"][0]["direct_blend"]["motions"].as_array_mut().expect("children").reverse();
    let after = lower_authoring_json(&input.to_string()).expect("reversed order");
    assert_eq!(before.source_map, after.source_map);
    let mut expected = before.scene.clone();
    expected["artboard"]["state_machines"][0]["layers"][0]["states"][1]["children"].as_array_mut().expect("children").reverse();
    assert_eq!(after.scene, expected);
    assert_ne!(compiled(&before.scene), compiled(&after.scene));
}

#[test]
fn invalid_fixed_weight_edit_rolls_back_atomically() {
    use rive_cli::authoring::{AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec, AuthoringTarget, apply_operations, lower_authoring};
    let input = document();
    let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed source");
    let before = lower_authoring(&spec).expect("initial lowering");
    let snapshot = serde_json::to_value(&spec).expect("snapshot");
    let mut replacement = input["behavior"]["statecharts"][0].clone();
    replacement["states"][0]["direct_blend"]["motions"][1]["weight"] = literal(-1.0);
    let error = apply_operations(&spec, &[
        AuthoringOperation::Remove { target: AuthoringTarget::BehaviorStatechart { target_id:"meter".to_string() } },
        AuthoringOperation::Insert {
            entity:AuthoringEntity::BehaviorStatechart(serde_json::from_value(replacement).expect("replacement")),
            placement:AuthoringPlacement::Into { container:AuthoringContainer::BehaviorStatecharts },
        },
    ]).expect_err("invalid fixed edit");
    assert!(error.diagnostics.iter().any(|entry| entry.code == "invalid_blend_weight"));
    assert_eq!(snapshot, serde_json::to_value(&spec).expect("unchanged spec"));
    assert_eq!(before, lower_authoring(&spec).expect("unchanged lowering"));
}

#[test]
fn legacy_model_direct_and_one_dimensional_binaries_are_unchanged() {
    use sha2::{Digest, Sha256};
    for (input, expected) in [
        (include_str!("../examples/authoring/model-blend-panel.v0.json"), "bc107cad1542c95190a125b24ce97a6c62e05fd553972bc47f2cad3735c5f8d4"),
        (include_str!("../examples/authoring/model-blend-1d-panel.v0.json"), "bdb14f4f4a5492467d7ff04050f973f652337c9bab59dec6be909d49c3f19d94"),
    ] {
        let lowered = lower_authoring_json(input).expect("legacy example");
        assert_eq!(format!("{:x}", Sha256::digest(compiled(&lowered.scene))), expected);
    }
}
