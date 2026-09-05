mod support;

use rive_cli::authoring::{
    AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec,
    AuthoringTarget, apply_operations, lower_authoring, lower_authoring_json,
};
use serde_json::{Value, json};
use support::assert_builds;

fn document() -> Value {
    serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json"))
        .expect("committed behavior example must parse")
}

fn transition(id: &str) -> Value {
    json!({
        "id": id,
        "from": "reading",
        "to": "reading",
        "when": {
            "input": "load",
            "compare": "greater",
            "value": { "kind": "literal", "value": 0.0, "unit": "scalar" }
        }
    })
}

fn region(id: &str) -> Value {
    json!({
        "id": id,
        "initial": "reading",
        "states": [{ "id": "reading", "motion": "calm-track" }],
        "transitions": [transition("refresh")]
    })
}

#[test]
fn events_and_inputs_must_not_share_a_behavior_source_identity() {
    let mut input = document();
    lower_authoring_json(&input.to_string()).expect("uncorrupted example must lower");
    input["behavior"]["statecharts"][0]["events"] = json!([{ "id": "load" }]);

    let error = lower_authoring_json(&input.to_string())
        .expect_err("an event and input must not publish the same authored identity");
    assert_eq!(error.diagnostics.len(), 1);
    let diagnostic = &error.diagnostics[0];
    assert_eq!(diagnostic.code, "behavior_source_id_collision");
    assert_eq!(diagnostic.path, "$.behavior.statecharts[0].inputs[0].id");
    assert!(diagnostic.message.contains("meter/load"));
    assert!(
        diagnostic
            .message
            .contains("$.behavior.statecharts[0].events[0]")
    );
}

#[test]
fn every_later_collision_reports_the_original_claim_in_deterministic_order() {
    let mut input = document();
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["events"] = json!([{ "id": "reading" }]);
    chart["inputs"]
        .as_array_mut()
        .expect("inputs")
        .push(json!({ "kind": "bool", "id": "reading", "value": false }));
    chart["listeners"] = json!([{
        "id": "reading",
        "target": "needle",
        "listener_type": "down",
        "actions": [{
            "kind": "number_change",
            "input": "load",
            "value": { "kind": "literal", "value": 0.0, "unit": "scalar" }
        }]
    }]);
    chart["transitions"] = json!([transition("reading")]);

    let first = lower_authoring_json(&input.to_string()).expect_err("colliding identities");
    let second = lower_authoring_json(&input.to_string()).expect_err("colliding identities");
    assert_eq!(first.diagnostics, second.diagnostics);
    let paths = first
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        paths,
        [
            "$.behavior.statecharts[0].inputs[1].id",
            "$.behavior.statecharts[0].listeners[0].id",
            "$.behavior.statecharts[0].states[0].id",
            "$.behavior.statecharts[0].transitions[0].id",
        ]
    );
    for diagnostic in &first.diagnostics {
        assert_eq!(diagnostic.code, "behavior_source_id_collision");
        assert!(diagnostic.message.contains("meter/reading"));
        assert!(
            diagnostic
                .message
                .contains("$.behavior.statecharts[0].events[0]")
        );
    }
}

#[test]
fn region_states_and_transitions_must_not_share_a_source_identity() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["regions"] = json!([region("pulse")]);
    lower_authoring_json(&input.to_string()).expect("uncorrupted region must lower");
    input["behavior"]["statecharts"][0]["regions"][0]["transitions"][0]["id"] = json!("reading");

    let error = lower_authoring_json(&input.to_string()).expect_err("colliding region identities");
    assert_eq!(error.diagnostics.len(), 1);
    let diagnostic = &error.diagnostics[0];
    assert_eq!(diagnostic.code, "behavior_source_id_collision");
    assert_eq!(
        diagnostic.path,
        "$.behavior.statecharts[0].regions[0].transitions[0].id"
    );
    assert!(diagnostic.message.contains("meter/pulse/reading"));
    assert!(
        diagnostic
            .message
            .contains("$.behavior.statecharts[0].regions[0].states[0]")
    );
}

#[test]
fn local_ids_remain_reusable_in_distinct_statechart_and_region_scopes() {
    let mut input = document();
    input["behavior"]["statecharts"][0]["regions"] = json!([region("left"), region("right")]);
    let mut other = input["behavior"]["statecharts"][0].clone();
    other["id"] = json!("other");
    input["behavior"]["statecharts"]
        .as_array_mut()
        .expect("statecharts")
        .push(other);

    let first = lower_authoring_json(&input.to_string()).expect("scoped IDs must lower");
    let second = lower_authoring_json(&input.to_string()).expect("scoped IDs must lower");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);
    for id in [
        "meter/reading",
        "meter/load",
        "meter/left/reading",
        "meter/left/refresh",
        "meter/right/reading",
        "meter/right/refresh",
        "other/reading",
        "other/load",
        "other/left/reading",
        "other/right/reading",
    ] {
        assert_eq!(
            first
                .source_map
                .entries
                .iter()
                .filter(|entry| entry.authored_id == id)
                .count(),
            1,
            "expected one source entry for {id}"
        );
    }
    assert_builds(first.scene);
}

#[test]
fn existing_duplicate_input_and_region_diagnostics_keep_precedence() {
    let mut input = document();
    let duplicate = input["behavior"]["statecharts"][0]["inputs"][0].clone();
    input["behavior"]["statecharts"][0]["inputs"]
        .as_array_mut()
        .expect("inputs")
        .push(duplicate);
    let error = lower_authoring_json(&input.to_string()).expect_err("duplicate input");
    assert_eq!(error.diagnostics.len(), 1);
    assert_eq!(error.diagnostics[0].code, "duplicate_behavior_input");
    assert_eq!(
        error.diagnostics[0].path,
        "$.behavior.statecharts[0].inputs[1].id"
    );

    let mut input = document();
    input["behavior"]["statecharts"][0]["regions"] = json!([region("load")]);
    let error = lower_authoring_json(&input.to_string()).expect_err("region aliases input");
    assert_eq!(error.diagnostics.len(), 1);
    assert_eq!(error.diagnostics[0].code, "behavior_region_id_collision");
    assert_eq!(
        error.diagnostics[0].path,
        "$.behavior.statecharts[0].regions[0].id"
    );
}

#[test]
fn identity_collision_rolls_back_a_statechart_replacement_batch() {
    let spec: AuthoringSpec = serde_json::from_value(document()).expect("typed authoring document");
    let snapshot = serde_json::to_value(&spec).expect("original document");
    let before = lower_authoring(&spec).expect("original lowering");
    let mut replacement = document()["behavior"]["statecharts"][0].clone();
    replacement["events"] = json!([{ "id": "load" }]);
    let operations = [
        AuthoringOperation::Remove {
            target: AuthoringTarget::BehaviorStatechart {
                target_id: "meter".to_string(),
            },
        },
        AuthoringOperation::Insert {
            entity: AuthoringEntity::BehaviorStatechart(
                serde_json::from_value(replacement).expect("replacement statechart"),
            ),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::BehaviorStatecharts,
            },
        },
    ];

    let error = apply_operations(&spec, &operations).expect_err("ambiguous replacement must fail");
    assert_eq!(error.diagnostics.len(), 1);
    assert_eq!(error.diagnostics[0].code, "behavior_source_id_collision");
    assert_eq!(
        error.diagnostics[0].path,
        "$.behavior.statecharts[0].inputs[0].id"
    );
    assert_eq!(
        serde_json::to_value(&spec).expect("unchanged spec"),
        snapshot
    );
    let after = lower_authoring(&spec).expect("original document remains valid");
    assert_eq!(before.scene, after.scene);
    assert_eq!(before.source_map, after.source_map);
}
