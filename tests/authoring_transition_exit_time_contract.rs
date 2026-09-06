use rive_cli::authoring::{
    AuthoringError, AuthoringSpec, LoweredAuthoring, ScalarExpr, Unit, lower_authoring,
    lower_authoring_json,
};
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::objects::core::{property_keys, type_keys};
use rive_cli::validator::{InspectFilter, PropertyValueRead, RivObject, parse_riv};
use serde_json::{Value, json};

const EXIT_PATH: &str = "$.behavior.statecharts[0].transitions[0].exit_time_ms";
const ENABLE_EXIT_TIME: u64 = 1 << 2;

fn document() -> Value {
    serde_json::from_str(include_str!(
        "../examples/authoring/pointer-statechart.v0.json"
    ))
    .expect("authoring fixture")
}

fn literal(value: f64) -> Value {
    json!({ "kind": "literal", "value": value, "unit": "scalar" })
}

fn lower(input: &Value) -> LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("typed transition must lower")
}

fn compile(scene: &Value) -> Vec<u8> {
    let scene: SceneSpec = serde_json::from_value(scene.clone()).expect("canonical scene");
    compile_scene(&scene, None, 0).expect("canonical scene must compile")
}

fn encoded_transitions(scene: &Value) -> Vec<RivObject> {
    parse_riv(&compile(scene), &InspectFilter::default())
        .expect("encoded scene")
        .objects
        .into_iter()
        .filter(|object| object.type_key == type_keys::STATE_TRANSITION)
        .collect()
}

fn property(object: &RivObject, key: u16) -> Option<PropertyValueRead> {
    object
        .properties
        .iter()
        .find(|property| property.key == key)
        .map(|property| property.value.clone())
}

fn assert_diagnostic(error: &AuthoringError, code: &str, path: &str) {
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == code && diagnostic.path == path }),
        "missing {code} at {path}: {error:?}"
    );
}

fn set_exit_time(input: &mut Value, expression: Value) {
    input["behavior"]["statecharts"][0]["transitions"][0]["exit_time_ms"] = expression;
}

fn blend_state() -> Value {
    json!({
        "id": "resting",
        "blend": { "input": "load", "stops": [
            { "motion": "rest-track", "value": literal(0.0) },
            { "motion": "active-track", "value": literal(1.0) }
        ] }
    })
}

fn add_blend_input(input: &mut Value) {
    input["behavior"]["statecharts"][0]["inputs"]
        .as_array_mut()
        .expect("inputs")
        .push(json!({ "kind": "number", "id": "load", "value": literal(0.0) }));
}

#[test]
fn authored_exit_time_reaches_the_encoded_transition_with_its_enable_flag() {
    let mut input = document();
    input["parameters"] = json!({ "gate": { "value": 750, "unit": "scalar" } });
    set_exit_time(&mut input, json!({ "kind": "parameter", "name": "gate" }));
    let lowered = lower(&input);
    let transition = &lowered.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1];
    assert_eq!(transition["exit_time"], 750);
    assert_eq!(transition["from"], 1);
    assert_eq!(transition["to"], 2);
    let encoded = encoded_transitions(&lowered.scene);
    assert_eq!(
        property(&encoded[1], property_keys::STATE_TRANSITION_EXIT_TIME),
        Some(PropertyValueRead::UInt(750))
    );
    assert_eq!(
        property(&encoded[1], property_keys::STATE_TRANSITION_FLAGS),
        Some(PropertyValueRead::UInt(ENABLE_EXIT_TIME))
    );
    assert_eq!(
        property(&encoded[0], property_keys::STATE_TRANSITION_FLAGS),
        None
    );
}

#[test]
fn authored_blend_sources_reject_exit_time_at_the_authored_field() {
    let mut input = document();
    add_blend_input(&mut input);
    input["behavior"]["statecharts"][0]["states"][0] = blend_state();
    lower(&input);
    set_exit_time(&mut input, literal(750.0));
    let error =
        lower_authoring_json(&input.to_string()).expect_err("blend gate must not be ignored");
    assert_diagnostic(&error, "unsupported_transition_exit_source", EXIT_PATH);

    input["behavior"]["statecharts"][0]["transitions"][0]["from"] = json!("engaged");
    input["behavior"]["statecharts"][0]["transitions"][0]["to"] = json!("resting");
    compile(&lower(&input).scene);
}

#[test]
fn canonical_exit_time_rejects_non_animation_sources() {
    let baseline = lower(&document()).scene;
    for source in [
        "entry",
        "exit",
        "any",
        "blend_state",
        "blend_state_direct",
        "blend_state_1d",
    ] {
        let mut scene = baseline.clone();
        scene["artboard"]["state_machines"][0]["inputs"]
            .as_array_mut()
            .expect("inputs")
            .push(json!({ "type": "number", "name": "load", "value": 0 }));
        scene["artboard"]["state_machines"][0]["layers"][0]["states"][1] =
            if source == "blend_state_1d" {
                json!({ "type": source, "input": "load" })
            } else {
                json!({ "type": source })
            };
        compile(&scene);
        scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["exit_time"] =
            json!(0);
        let gated: SceneSpec = serde_json::from_value(scene).expect("canonical gate");
        let error =
            compile_scene(&gated, None, 0).expect_err("non-animation gate must be rejected");
        assert!(
            error
                .to_string()
                .contains("exit_time requires an animation source state"),
            "{source}: {error}"
        );
    }
}

#[test]
fn omitted_and_null_gates_preserve_output_but_explicit_zero_enables_the_gate() {
    let mut input = document();
    let baseline = lower(&input);
    let transitions = &baseline.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"];
    assert_eq!(
        transitions,
        &json!([
            { "from": 0, "to": 1 },
            { "from": 1, "to": 2, "conditions": [{
                "input": "auth__interaction_2dstage__gate__pressed__input", "value": true
            }] }
        ])
    );
    set_exit_time(&mut input, Value::Null);
    let omitted = lower(&input);
    assert_eq!(omitted.scene, baseline.scene);
    assert_eq!(compile(&omitted.scene), compile(&baseline.scene));
    set_exit_time(&mut input, literal(0.0));
    let zero = lower(&input);
    assert_eq!(zero.source_map, baseline.source_map);
    assert_eq!(
        zero.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["exit_time"],
        0
    );
    let encoded = encoded_transitions(&zero.scene);
    assert_eq!(
        property(&encoded[1], property_keys::STATE_TRANSITION_FLAGS),
        Some(PropertyValueRead::UInt(ENABLE_EXIT_TIME))
    );
    assert_eq!(
        property(&encoded[1], property_keys::STATE_TRANSITION_EXIT_TIME),
        None
    );
    assert_ne!(compile(&zero.scene), compile(&baseline.scene));
}

#[test]
fn exit_time_boundaries_round_trip_without_integer_narrowing() {
    for value in [0, 1, u32::MAX] {
        let mut input = document();
        set_exit_time(&mut input, literal(f64::from(value)));
        let lowered = lower(&input);
        assert_eq!(
            lowered.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["exit_time"],
            value
        );
        let encoded = encoded_transitions(&lowered.scene);
        assert_eq!(
            property(&encoded[1], property_keys::STATE_TRANSITION_EXIT_TIME),
            if value == 0 {
                None
            } else {
                Some(PropertyValueRead::UInt(u64::from(value)))
            }
        );
    }
}

#[test]
fn exit_time_rejects_negative_fractional_and_oversized_values() {
    for value in [-1.0, 0.5, f64::from(u32::MAX) + 1.0] {
        let mut input = document();
        set_exit_time(&mut input, literal(value));
        let error = lower_authoring_json(&input.to_string()).expect_err("invalid milliseconds");
        assert_diagnostic(&error, "invalid_transition_exit_time", EXIT_PATH);
    }
}

#[test]
fn exit_time_preserves_expression_error_codes_and_paths() {
    for (expression, code, suffix) in [
        (
            json!({ "kind": "literal", "value": 10, "unit": "px" }),
            "unit_mismatch",
            "",
        ),
        (
            json!({ "kind": "parameter", "name": "missing" }),
            "unknown_parameter",
            ".name",
        ),
        (
            json!({ "kind": "divide", "value": literal(10.0), "divisor": 0 }),
            "division_by_zero",
            ".divisor",
        ),
        (
            json!({ "kind": "divide", "value": literal(1.0), "divisor": 2 }),
            "invalid_transition_exit_time",
            "",
        ),
    ] {
        let mut input = document();
        set_exit_time(&mut input, expression);
        let error = lower_authoring_json(&input.to_string()).expect_err("invalid expression");
        assert_diagnostic(&error, code, &format!("{EXIT_PATH}{suffix}"));
    }
}

#[test]
fn programmatic_exit_time_rejects_non_finite_values() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut input: AuthoringSpec = serde_json::from_value(document()).expect("typed document");
        input.behavior.statecharts[0].transitions[0].exit_time_ms = Some(ScalarExpr::Literal {
            value,
            unit: Unit::Scalar,
        });
        let error = lower_authoring(&input).expect_err("non-finite milliseconds");
        assert_diagnostic(&error, "non_finite", &format!("{EXIT_PATH}.value"));
    }
}

#[test]
fn exit_time_composes_with_duration_without_changing_identity_or_conditions() {
    let mut input = document();
    let baseline = lower(&input);
    set_exit_time(&mut input, literal(750.0));
    input["behavior"]["statecharts"][0]["transitions"][0]["duration_ms"] = literal(250.0);
    let first = lower(&input);
    let second = lower(&input);
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);
    assert_eq!(first.source_map, baseline.source_map);
    assert_eq!(compile(&first.scene), compile(&second.scene));
    assert_eq!(
        first.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["conditions"],
        baseline.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["conditions"]
    );
    let encoded = encoded_transitions(&first.scene);
    assert_eq!(
        property(&encoded[1], property_keys::STATE_TRANSITION_DURATION),
        Some(PropertyValueRead::UInt(250))
    );
    assert_eq!(
        property(&encoded[1], property_keys::STATE_TRANSITION_EXIT_TIME),
        Some(PropertyValueRead::UInt(750))
    );
    assert_eq!(
        property(&encoded[1], property_keys::STATE_TRANSITION_FLAGS),
        Some(PropertyValueRead::UInt(ENABLE_EXIT_TIME))
    );
}

#[test]
fn parallel_region_gates_use_document_parameters_and_region_source_paths() {
    let mut input = document();
    input["parameters"] = json!({ "gate": { "value": 1500, "unit": "scalar" } });
    add_blend_input(&mut input);
    let chart = &mut input["behavior"]["statecharts"][0];
    let mut region = json!({
        "id": "parallel", "initial": "resting",
        "states": chart["states"], "transitions": chart["transitions"]
    });
    region["transitions"][0]["exit_time_ms"] = json!({
        "kind": "divide", "value": { "kind": "parameter", "name": "gate" }, "divisor": 2
    });
    chart["regions"] = json!([region]);
    let lowered = lower(&input);
    let layers = &lowered.scene["artboard"]["state_machines"][0]["layers"];
    assert!(layers[0]["transitions"][1].get("exit_time").is_none());
    assert_eq!(layers[1]["transitions"][1]["exit_time"], 750);
    let source = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "gate/parallel/engage")
        .expect("region transition source map");
    assert_eq!(
        source.authored_path,
        "$.behavior.statecharts[0].regions[0].transitions[0]"
    );
    assert_eq!(
        source.scene_paths,
        ["/artboard/state_machines/0/layers/1/transitions/1"]
    );
    let encoded = encoded_transitions(&lowered.scene);
    assert_eq!(
        property(&encoded[1], property_keys::STATE_TRANSITION_FLAGS),
        None
    );
    assert_eq!(
        property(&encoded[3], property_keys::STATE_TRANSITION_EXIT_TIME),
        Some(PropertyValueRead::UInt(750))
    );
    let path = "$.behavior.statecharts[0].regions[0].transitions[0].exit_time_ms";
    input["behavior"]["statecharts"][0]["regions"][0]["transitions"][0]["exit_time_ms"] =
        literal(-1.0);
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid region gate");
    assert_diagnostic(&error, "invalid_transition_exit_time", path);
    input["behavior"]["statecharts"][0]["regions"][0]["transitions"][0]["exit_time_ms"] =
        literal(750.0);
    input["behavior"]["statecharts"][0]["regions"][0]["states"][0] = blend_state();
    let error = lower_authoring_json(&input.to_string()).expect_err("unsupported region source");
    assert_diagnostic(&error, "unsupported_transition_exit_source", path);
}

#[test]
fn canonical_exit_time_enforces_u32_and_checks_source_bounds_before_indexing() {
    let baseline = lower(&document()).scene;
    for value in [json!(-1), json!(1.5), json!(u64::from(u32::MAX) + 1)] {
        let mut scene = baseline.clone();
        scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["exit_time"] = value;
        assert!(serde_json::from_value::<SceneSpec>(scene).is_err());
    }
    let mut scene = baseline;
    scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["exit_time"] = json!(0);
    scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["from"] = json!(999);
    let scene: SceneSpec = serde_json::from_value(scene).expect("canonical gate");
    let error = compile_scene(&scene, None, 0).expect_err("invalid source must not panic");
    assert!(error.to_string().contains("source index 999 out of bounds"));
}
