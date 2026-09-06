mod support;

use rive_cli::authoring::{
    AuthoringError, AuthoringSpec, LoweredAuthoring, ScalarExpr, Unit, lower_authoring,
    lower_authoring_json,
};
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use serde_json::{Value, json};
use support::assert_builds;

const DURATION_PATH: &str = "$.behavior.statecharts[0].transitions[0].duration_ms";

fn document() -> Value {
    let mut input: Value =
        serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json"))
            .expect("authoring fixture");
    input["behavior"]["statecharts"][0] = json!({
        "id": "meter",
        "inputs": [{ "kind": "bool", "id": "enabled", "value": false }],
        "initial": "calm",
        "states": [
            { "id": "calm", "motion": "calm-track" },
            { "id": "surge", "motion": "surge-track" }
        ],
        "transitions": [{
            "id": "activate",
            "from": "calm",
            "to": "surge",
            "when": { "input": "enabled", "equals": true }
        }]
    });
    input
}

fn literal(value: f64) -> Value {
    json!({ "kind": "literal", "value": value, "unit": "scalar" })
}

fn set_duration(input: &mut Value, expression: Value) {
    input["behavior"]["statecharts"][0]["transitions"][0]["duration_ms"] = expression;
}

fn lower(input: &Value) -> LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("typed transition must lower")
}

fn compiled(scene: &Value) -> Vec<u8> {
    let scene: SceneSpec = serde_json::from_value(scene.clone()).expect("canonical scene");
    compile_scene(&scene, None, 0).expect("canonical scene must compile")
}

fn assert_diagnostic(error: &AuthoringError, code: &str, path: &str) {
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code && diagnostic.path == path),
        "missing {code} at {path}; diagnostics: {:#?}",
        error.diagnostics
    );
}

#[test]
fn transition_duration_lowers_in_milliseconds_from_a_parameter() {
    let mut input = document();
    let baseline = lower(&input);
    input["parameters"] = json!({ "crossfade": { "value": 250.0, "unit": "scalar" } });
    set_duration(&mut input, json!({ "kind": "parameter", "name": "crossfade" }));

    let first = lower(&input);
    let second = lower(&input);
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);
    assert_eq!(first.source_map, baseline.source_map);
    let transitions = &first.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"];
    assert_eq!(transitions[1]["duration"], 250);
    assert_eq!(transitions[1]["from"], 1);
    assert_eq!(transitions[1]["to"], 2);
    assert!(transitions[0].get("duration").is_none());
    assert_builds(first.scene);
}

#[test]
fn omitted_duration_preserves_the_scene_and_zero_preserves_binary_output() {
    let mut input = document();
    let baseline = lower(&input);
    assert_eq!(
        baseline.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"],
        json!([
            { "from": 0, "to": 1 },
            {
                "from": 1,
                "to": 2,
                "conditions": [{
                    "input": "auth__blend_2dmeter__meter__enabled__input",
                    "value": true
                }]
            }
        ])
    );
    set_duration(&mut input, literal(0.0));
    let mut zero = lower(&input);
    assert_eq!(zero.source_map, baseline.source_map);
    assert_eq!(compiled(&zero.scene), compiled(&baseline.scene));
    let transition = &mut zero.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1];
    assert_eq!(transition["duration"], 0);
    transition.as_object_mut().expect("transition object").remove("duration");
    assert_eq!(zero.scene, baseline.scene);
}

#[test]
fn whole_millisecond_boundaries_are_preserved_as_integers() {
    for duration in [0_u32, 1, u32::MAX] {
        let mut input = document();
        set_duration(&mut input, literal(f64::from(duration)));
        let lowered = lower(&input);
        assert_eq!(
            lowered.scene["artboard"]["state_machines"][0]["layers"][0]["transitions"][1]["duration"].as_u64(),
            Some(u64::from(duration))
        );
        assert_builds(lowered.scene);
    }
}

#[test]
fn negative_fractional_and_out_of_range_durations_are_rejected() {
    for duration in [-1.0, 0.25, f64::from(u32::MAX) + 1.0] {
        let mut input = document();
        set_duration(&mut input, literal(duration));
        let error = lower_authoring_json(&input.to_string()).expect_err("invalid duration");
        assert_diagnostic(&error, "invalid_transition_duration", DURATION_PATH);
    }
}

#[test]
fn duration_expressions_preserve_their_authored_diagnostics() {
    for (expression, code, suffix) in [
        (json!({ "kind": "literal", "value": 250, "unit": "px" }), "unit_mismatch", ""),
        (json!({ "kind": "parameter", "name": "missing" }), "unknown_parameter", ".name"),
        (json!({ "kind": "divide", "value": literal(250.0), "divisor": 0 }), "division_by_zero", ".divisor"),
        (json!({ "kind": "divide", "value": literal(1.0), "divisor": 2 }), "invalid_transition_duration", ""),
    ] {
        let mut input = document();
        set_duration(&mut input, expression);
        let error = lower_authoring_json(&input.to_string()).expect_err("invalid expression");
        assert_diagnostic(&error, code, &format!("{DURATION_PATH}{suffix}"));
    }
}

#[test]
fn programmatic_non_finite_duration_is_rejected_before_encoding() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut spec: AuthoringSpec = serde_json::from_value(document()).expect("typed document");
        spec.behavior.statecharts[0].transitions[0].duration_ms = Some(ScalarExpr::Literal {
            value,
            unit: Unit::Scalar,
        });
        let error = lower_authoring(&spec).expect_err("non-finite duration");
        assert_diagnostic(&error, "non_finite", &format!("{DURATION_PATH}.value"));
    }
}

#[test]
fn region_duration_uses_shared_parameters_and_keeps_scoped_source_identity() {
    let mut input = document();
    input["parameters"] = json!({ "crossfade": { "value": 250.0, "unit": "scalar" } });
    let mut region = input["behavior"]["statecharts"][0].clone();
    region.as_object_mut().expect("region").remove("inputs");
    region["id"] = json!("alert");
    region["transitions"][0]["duration_ms"] = json!({
        "kind": "divide",
        "value": { "kind": "parameter", "name": "crossfade" },
        "divisor": 2
    });
    input["behavior"]["statecharts"][0]["regions"] = json!([region]);
    let lowered = lower(&input);
    let layers = &lowered.scene["artboard"]["state_machines"][0]["layers"];
    assert!(layers[0]["transitions"][1].get("duration").is_none());
    assert_eq!(layers[1]["transitions"][1]["duration"], 125);
    let source = lowered.source_map.entries.iter()
        .find(|entry| entry.authored_id == "meter/alert/activate")
        .expect("region transition source map");
    assert_eq!(source.authored_path, "$.behavior.statecharts[0].regions[0].transitions[0]");
    assert_eq!(source.scene_paths, ["/artboard/state_machines/0/layers/1/transitions/1"]);
    assert_builds(lowered.scene);

    input["behavior"]["statecharts"][0]["regions"][0]["transitions"][0]["duration_ms"] = literal(-1.0);
    let error = lower_authoring_json(&input.to_string()).expect_err("invalid region duration");
    assert_diagnostic(
        &error,
        "invalid_transition_duration",
        "$.behavior.statecharts[0].regions[0].transitions[0].duration_ms",
    );
}
