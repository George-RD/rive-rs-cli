mod support;

use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};
use support::assert_builds;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn pose(id: &str, x: f64) -> Value {
    json!({
        "id": id,
        "targets": [
            {
                "target": "meter",
                "transform": { "x": literal(x, "px"), "y": literal(80.0, "px") }
            }
        ]
    })
}

fn track(id: &str, pose_id: &str) -> Value {
    json!({
        "id": id,
        "fps": 60,
        "duration_frames": literal(1.0, "scalar"),
        "keyframes": [
            { "frame": literal(0.0, "scalar"), "pose": pose_id },
            { "frame": literal(1.0, "scalar"), "pose": pose_id }
        ]
    })
}

fn document() -> Value {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "blend-stage",
            "width": { "value": 240.0, "unit": "px" },
            "height": { "value": 160.0, "unit": "px" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "rectangle",
                    "id": "meter",
                    "width": literal(24.0, "px"),
                    "height": literal(24.0, "px"),
                    "fill": "#22C55E"
                }
            ]
        },
        "motion": {
            "poses": [pose("low", 40.0), pose("high", 200.0), pose("rest", 120.0)],
            "tracks": [
                track("low-track", "low"),
                track("high-track", "high"),
                track("rest-track", "rest")
            ]
        },
        "behavior": {
            "statecharts": [
                {
                    "id": "meter-machine",
                    "inputs": [
                        { "kind": "number", "id": "level", "value": literal(0.0, "scalar") },
                        { "kind": "trigger", "id": "reset" },
                        { "kind": "bool", "id": "armed", "value": false }
                    ],
                    "initial": "resting",
                    "states": [
                        { "id": "resting", "motion": "rest-track" },
                        {
                            "id": "metering",
                            "blend": {
                                "input": "level",
                                "stops": [
                                    { "motion": "low-track", "value": literal(0.0, "scalar") },
                                    { "motion": "high-track", "value": literal(100.0, "scalar") }
                                ]
                            }
                        }
                    ],
                    "transitions": [
                        {
                            "id": "engage",
                            "from": "resting",
                            "to": "metering",
                            "when": { "input": "armed", "equals": true }
                        },
                        {
                            "id": "release",
                            "from": "metering",
                            "to": "resting",
                            "when": { "trigger": "reset" }
                        }
                    ],
                    "regions": [
                        {
                            "id": "pulse",
                            "initial": "quiet",
                            "states": [
                                { "id": "quiet", "motion": "rest-track" },
                                { "id": "loud", "motion": "high-track" }
                            ],
                            "transitions": [
                                {
                                    "id": "raise",
                                    "from": "quiet",
                                    "to": "loud",
                                    "when": {
                                        "input": "level",
                                        "compare": "greater_or_equal",
                                        "value": literal(50.0, "scalar")
                                    }
                                }
                            ]
                        }
                    ]
                }
            ]
        }
    })
}

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("typed behavior must lower")
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string())
        .expect_err("invalid typed behavior must fail at the authored boundary");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code && diagnostic.path == path),
        "missing {code} at {path}; diagnostics: {:#?}",
        error.diagnostics
    );
}

fn source_entry<'a>(
    lowered: &'a rive_cli::authoring::LoweredAuthoring,
    authored_id: &str,
) -> &'a rive_cli::authoring::SourceMapEntry {
    lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == authored_id)
        .expect("source-map entry should exist")
}

#[test]
fn file_assets_compose_with_typed_behavior() {
    let mut input = document();
    input["font_assets"] = json!({ "inter": "../../assets/fonts/Inter-Bold-Subset.ttf" });

    let lowered = lower(&input);
    let assets = lowered.scene["artboard"]["children"]
        .as_array()
        .expect("artboard children")
        .iter()
        .filter(|child| child["type"] == "font_asset")
        .count();
    assert_eq!(assets, 1);
}

#[test]
fn typed_inputs_lower_to_number_trigger_and_bool_state_machine_inputs() {
    let first = lower(&document());
    let second = lower(&document());
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let inputs = first.scene["artboard"]["state_machines"][0]["inputs"]
        .as_array()
        .expect("state machine inputs");
    assert_eq!(inputs[0]["type"], "number");
    assert_eq!(
        inputs[0]["name"],
        "auth__blend_2dstage__meter_2dmachine__level__input"
    );
    assert_eq!(inputs[0]["value"], 0.0);
    assert_eq!(inputs[1]["type"], "trigger");
    assert_eq!(
        inputs[1]["name"],
        "auth__blend_2dstage__meter_2dmachine__reset__input"
    );
    assert_eq!(inputs[2]["type"], "bool");
    assert_eq!(inputs[2]["value"], false);
    assert_builds(first.scene);
}

#[test]
fn blend_states_lower_to_one_dimensional_blends_over_named_motion() {
    let lowered = lower(&document());
    let state = &lowered.scene["artboard"]["state_machines"][0]["layers"][0]["states"][2];

    assert_eq!(state["type"], "blend_state_1d");
    assert_eq!(
        state["input"],
        "auth__blend_2dstage__meter_2dmachine__level__input"
    );
    let children = state["children"].as_array().expect("blend children");
    assert_eq!(children[0]["type"], "blend_animation_1d");
    assert_eq!(
        children[0]["animation"],
        "auth__blend_2dstage__low_2dtrack__animation"
    );
    assert_eq!(children[0]["value"], 0.0);
    assert_eq!(
        children[1]["animation"],
        "auth__blend_2dstage__high_2dtrack__animation"
    );
    assert_eq!(children[1]["value"], 100.0);
    assert_builds(lowered.scene);
}

#[test]
fn trigger_and_number_conditions_lower_without_runtime_indices() {
    let lowered = lower(&document());
    let machine = &lowered.scene["artboard"]["state_machines"][0];

    let release = &machine["layers"][0]["transitions"][2]["conditions"][0];
    assert_eq!(
        release["input"],
        "auth__blend_2dstage__meter_2dmachine__reset__input"
    );
    assert_eq!(release.get("value"), None);
    assert_eq!(release.get("op"), None);

    let raise = &machine["layers"][1]["transitions"][1]["conditions"][0];
    assert_eq!(
        raise["input"],
        "auth__blend_2dstage__meter_2dmachine__level__input"
    );
    assert_eq!(raise["op"], ">=");
    assert_eq!(raise["value"], 50.0);
}

#[test]
fn parallel_regions_lower_to_independent_layers() {
    let lowered = lower(&document());
    let layers = lowered.scene["artboard"]["state_machines"][0]["layers"]
        .as_array()
        .expect("state machine layers");
    assert_eq!(layers.len(), 2);

    let pulse = &layers[1];
    assert_eq!(pulse["states"][0]["type"], "entry");
    assert_eq!(pulse["states"][1]["type"], "animation");
    assert_eq!(pulse["states"][3]["type"], "exit");
    assert_eq!(pulse["transitions"][0]["from"], 0);
    assert_eq!(pulse["transitions"][0]["to"], 1);
    assert_eq!(pulse["transitions"][1]["from"], 1);
    assert_eq!(pulse["transitions"][1]["to"], 2);

    let quiet = source_entry(&lowered, "meter-machine/pulse/quiet");
    assert_eq!(
        quiet.authored_path,
        "$.behavior.statecharts[0].regions[0].states[0]"
    );
    assert_eq!(
        quiet.scene_paths[0],
        "/artboard/state_machines/0/layers/1/states/1"
    );
    let raise = source_entry(&lowered, "meter-machine/pulse/raise");
    assert_eq!(
        raise.scene_paths[0],
        "/artboard/state_machines/0/layers/1/transitions/1"
    );
}

#[test]
fn states_must_declare_exactly_one_motion_source() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][0] = json!({ "id": "resting" });
    assert_diagnostic(
        &input,
        "missing_state_motion",
        "$.behavior.statecharts[0].states[0]",
    );

    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][1]["motion"] = json!("rest-track");
    assert_diagnostic(
        &input,
        "ambiguous_state_motion",
        "$.behavior.statecharts[0].states[1]",
    );
}

#[test]
fn blend_states_require_a_number_input_and_known_motion() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][1]["blend"]["input"] = json!("armed");
    assert_diagnostic(
        &input,
        "invalid_blend_input",
        "$.behavior.statecharts[0].states[1].blend.input",
    );

    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][1]["blend"]["stops"][0]["motion"] =
        json!("missing-track");
    assert_diagnostic(
        &input,
        "unknown_behavior_motion",
        "$.behavior.statecharts[0].states[1].blend.stops[0].motion",
    );
}

#[test]
fn blend_states_require_between_two_and_a_thousand_stops() {
    let mut input = document();
    let stops = input["behavior"]["statecharts"][0]["states"][1]["blend"]["stops"]
        .as_array()
        .expect("blend stops")
        .clone();
    input["behavior"]["statecharts"][0]["states"][1]["blend"]["stops"] = json!([stops[0]]);
    assert_diagnostic(
        &input,
        "invalid_blend_stops",
        "$.behavior.statecharts[0].states[1].blend.stops",
    );

    input["behavior"]["statecharts"][0]["states"][1]["blend"]["stops"] = json!([]);
    assert_diagnostic(
        &input,
        "invalid_blend_stops",
        "$.behavior.statecharts[0].states[1].blend.stops",
    );

    let too_many = (0..1001)
        .map(|index| json!({ "motion": "low-track", "value": literal(f64::from(index), "scalar") }))
        .collect::<Vec<_>>();
    input["behavior"]["statecharts"][0]["states"][1]["blend"]["stops"] = json!(too_many);
    assert_diagnostic(
        &input,
        "invalid_blend_stops",
        "$.behavior.statecharts[0].states[1].blend.stops",
    );
}

#[test]
fn blend_stop_values_must_increase() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][1]["blend"]["stops"][1]["value"] =
        literal(0.0, "scalar");
    assert_diagnostic(
        &input,
        "invalid_blend_stop_order",
        "$.behavior.statecharts[0].states[1].blend.stops[1].value",
    );

    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][1]["blend"]["stops"][0]["value"] =
        literal(100.0, "scalar");
    input["behavior"]["statecharts"][0]["states"][1]["blend"]["stops"][1]["value"] =
        literal(0.0, "scalar");
    assert_diagnostic(
        &input,
        "invalid_blend_stop_order",
        "$.behavior.statecharts[0].states[1].blend.stops[1].value",
    );
}

#[test]
fn region_ids_must_not_alias_any_statechart_scoped_sibling() {
    for alias in ["resting", "engage", "level", "armed"] {
        let mut input = document();
        input["behavior"]["statecharts"][0]["regions"][0]["id"] = json!(alias);
        assert_diagnostic(
            &input,
            "behavior_region_id_collision",
            "$.behavior.statecharts[0].regions[0].id",
        );
    }

    let mut input = document();
    input["behavior"]["statecharts"][0]["events"] = json!([{ "id": "chime" }]);
    input["behavior"]["statecharts"][0]["regions"][0]["id"] = json!("chime");
    assert_diagnostic(
        &input,
        "behavior_region_id_collision",
        "$.behavior.statecharts[0].regions[0].id",
    );
}

#[test]
fn blend_stop_values_must_stay_distinct_once_narrowed_to_the_emitted_f32() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][1]["blend"]["stops"] = json!([
        { "motion": "low-track", "value": literal(1.0, "scalar") },
        { "motion": "high-track", "value": literal(1.000_000_01, "scalar") }
    ]);
    assert_diagnostic(
        &input,
        "invalid_blend_stop_order",
        "$.behavior.statecharts[0].states[1].blend.stops[1].value",
    );
}

#[test]
fn a_blend_stop_expression_failure_is_reported_beside_the_order_check() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["states"][1]["blend"]["stops"] = json!([
        { "motion": "low-track", "value": literal(0.0, "scalar") },
        { "motion": "rest-track", "value": { "kind": "parameter", "name": "undeclared" } },
        { "motion": "high-track", "value": literal(-1.0, "scalar") }
    ]);
    let error = lower_authoring_json(&input.to_string())
        .expect_err("an undefined parameter must fail at the authored boundary");
    let codes = error
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.as_str())
        .collect::<Vec<_>>();
    assert!(
        codes.contains(&"invalid_blend_stop_order"),
        "expected the order diagnostic; got {codes:?}"
    );
    assert!(
        error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "unknown_parameter"
                && diagnostic
                    .path
                    .starts_with("$.behavior.statecharts[0].states[1].blend.stops[1].value")
        }),
        "the failing stop expression was dropped; got {:#?}",
        error.diagnostics
    );
}

#[test]
fn typed_conditions_must_match_the_input_kind() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["transitions"][0]["when"] =
        json!({ "input": "level", "equals": true });
    assert_diagnostic(
        &input,
        "invalid_condition_input",
        "$.behavior.statecharts[0].transitions[0].when.input",
    );

    let mut input = document();
    input["behavior"]["statecharts"][0]["transitions"][1]["when"] = json!({ "trigger": "armed" });
    assert_diagnostic(
        &input,
        "invalid_condition_input",
        "$.behavior.statecharts[0].transitions[1].when.trigger",
    );

    let mut input = document();
    input["behavior"]["statecharts"][0]["regions"][0]["transitions"][0]["when"]["input"] =
        json!("armed");
    assert_diagnostic(
        &input,
        "invalid_condition_input",
        "$.behavior.statecharts[0].regions[0].transitions[0].when.input",
    );
}

#[test]
fn regions_validate_their_own_identities_and_initial_state() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["regions"][0]["initial"] = json!("missing");
    assert_diagnostic(
        &input,
        "unknown_behavior_state",
        "$.behavior.statecharts[0].regions[0].initial",
    );

    let mut input = document();
    let region = input["behavior"]["statecharts"][0]["regions"][0].clone();
    input["behavior"]["statecharts"][0]["regions"]
        .as_array_mut()
        .expect("regions")
        .push(region);
    assert_diagnostic(
        &input,
        "duplicate_behavior_region",
        "$.behavior.statecharts[0].regions[1].id",
    );
}

#[test]
fn listeners_can_change_number_and_trigger_inputs() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["listeners"] = json!([
        {
            "id": "raise-level",
            "target": "meter",
            "listener_type": "down",
            "actions": [
                { "kind": "number_change", "input": "level", "value": literal(100.0, "scalar") },
                { "kind": "trigger_change", "input": "reset" }
            ]
        }
    ]);

    let lowered = lower(&input);
    let actions = lowered.scene["artboard"]["state_machines"][0]["listeners"][0]["actions"]
        .as_array()
        .expect("listener actions");
    assert_eq!(actions[0]["type"], "number_change");
    assert_eq!(
        actions[0]["input"],
        "auth__blend_2dstage__meter_2dmachine__level__input"
    );
    assert_eq!(actions[0]["value"], 100.0);
    assert_eq!(actions[1]["type"], "trigger_change");
    assert_builds(lowered.scene);

    input["behavior"]["statecharts"][0]["listeners"][0]["actions"][0] =
        json!({ "kind": "bool_change", "input": "level", "value": true });
    assert_diagnostic(
        &input,
        "invalid_listener_input",
        "$.behavior.statecharts[0].listeners[0].actions[0].input",
    );
}
