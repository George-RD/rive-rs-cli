mod support;

use rive_cli::authoring::{
    AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec,
    AuthoringTarget, apply_operations, authoring_schema, lower_authoring, lower_authoring_json,
};
use serde_json::{Value, json};
use support::assert_builds;

const COLLECTIONS: &[(&str, &str)] = &[
    ("/behavior/models", "$.behavior.models"),
    ("/behavior/bindings", "$.behavior.bindings"),
    ("/behavior/statecharts", "$.behavior.statecharts"),
    (
        "/behavior/models/0/properties",
        "$.behavior.models[0].properties",
    ),
    (
        "/behavior/statecharts/0/inputs",
        "$.behavior.statecharts[0].inputs",
    ),
    (
        "/behavior/statecharts/0/events",
        "$.behavior.statecharts[0].events",
    ),
    (
        "/behavior/statecharts/0/listeners",
        "$.behavior.statecharts[0].listeners",
    ),
    (
        "/behavior/statecharts/0/listeners/0/actions",
        "$.behavior.statecharts[0].listeners[0].actions",
    ),
    (
        "/behavior/statecharts/0/states",
        "$.behavior.statecharts[0].states",
    ),
    (
        "/behavior/statecharts/0/transitions",
        "$.behavior.statecharts[0].transitions",
    ),
    (
        "/behavior/statecharts/0/regions",
        "$.behavior.statecharts[0].regions",
    ),
    (
        "/behavior/statecharts/0/regions/0/states",
        "$.behavior.statecharts[0].regions[0].states",
    ),
    (
        "/behavior/statecharts/0/regions/0/transitions",
        "$.behavior.statecharts[0].regions[0].transitions",
    ),
];

fn document() -> Value {
    serde_json::from_str(include_str!("../examples/authoring/blend-meter.v0.json"))
        .expect("committed behavior example must parse")
}

fn boundary_document() -> Value {
    let mut input = document();
    input["behavior"]["models"] = json!([{
        "id": "settings",
        "properties": [{ "kind": "bool", "id": "ready", "value": false }]
    }]);
    input["behavior"]["bindings"] = json!([{
        "id": "ready-binding", "model": "settings", "property": "ready"
    }]);
    let chart = &mut input["behavior"]["statecharts"][0];
    chart["events"] = json!([{ "id": "activated" }]);
    chart["listeners"] = json!([{
        "id": "reset", "target": "needle", "listener_type": "down",
        "actions": [{
            "kind": "number_change", "input": "load",
            "value": { "kind": "literal", "value": 0.0, "unit": "scalar" }
        }]
    }]);
    chart["transitions"] = json!([{
        "id": "refresh", "from": "reading", "to": "reading",
        "when": {
            "input": "load", "compare": "greater",
            "value": { "kind": "literal", "value": 0.0, "unit": "scalar" }
        }
    }]);
    chart["regions"] = json!([{
        "id": "pulse", "initial": "reading",
        "states": [{ "id": "reading", "motion": "calm-track" }],
        "transitions": chart["transitions"].clone()
    }]);
    input
}

fn resize_collection(input: &mut Value, pointer: &str, count: usize) {
    let collection = input
        .pointer_mut(pointer)
        .expect("known collection")
        .as_array_mut()
        .expect("collection array");
    let template = collection[0].clone();
    *collection = (0..count)
        .map(|index| {
            let mut item = template.clone();
            if index > 0
                && let Some(id) = item.get_mut("id")
            {
                *id = json!(format!("copy-{index}"));
            }
            item
        })
        .collect();
}

fn assert_collection_error(input: Value, path: &str) {
    let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed document");
    let error = lower_authoring(&spec).expect_err("out-of-range collection must fail");
    let from_json = lower_authoring_json(&input.to_string()).expect_err("same JSON boundary");
    assert_eq!(error, from_json);
    assert_eq!(error.diagnostics.len(), 1);
    let diagnostic = &error.diagnostics[0];
    assert_eq!(diagnostic.code, "invalid_behavior_collection_count");
    assert_eq!(diagnostic.path, path);
    assert!(diagnostic.message.contains("1000"));
}

#[test]
fn typed_behavior_models_must_respect_the_published_collection_limit() {
    let mut input = document();
    input["behavior"]["models"] = json!(
        (0..1001)
            .map(|index| json!({ "id": format!("model-{index}"), "properties": [] }))
            .collect::<Vec<_>>()
    );
    assert_collection_error(input, "$.behavior.models");
}

#[test]
fn every_behavior_collection_rejects_one_item_above_its_limit() {
    for &(pointer, path) in COLLECTIONS {
        let mut input = boundary_document();
        resize_collection(&mut input, pointer, 1001);
        assert_collection_error(input, path);
    }
}

#[test]
fn every_upper_boundary_lowers_deterministically_and_builds() {
    for &(pointer, _) in COLLECTIONS {
        let mut input = boundary_document();
        resize_collection(&mut input, pointer, 1000);
        let spec: AuthoringSpec = serde_json::from_value(input.clone()).expect("typed document");
        let first = lower_authoring(&spec)
            .unwrap_or_else(|error| panic!("valid upper boundary {pointer}: {error:?}"));
        let second = lower_authoring_json(&input.to_string()).expect("same JSON boundary");
        assert_eq!(first.scene, second.scene);
        assert_eq!(first.source_map, second.source_map);
        assert_builds(first.scene);
    }
}

#[test]
fn root_and_region_state_collections_require_at_least_one_state() {
    for (pointer, path) in [
        (
            "/behavior/statecharts/0/states",
            "$.behavior.statecharts[0].states",
        ),
        (
            "/behavior/statecharts/0/regions/0/states",
            "$.behavior.statecharts[0].regions[0].states",
        ),
    ] {
        let mut input = boundary_document();
        resize_collection(&mut input, pointer, 0);
        assert_collection_error(input, path);
    }
}

#[test]
fn optional_behavior_collections_may_be_empty() {
    let mut input = document();
    input["behavior"] = json!({});
    let lowered = lower_authoring_json(&input.to_string()).expect("empty behavior");
    assert_builds(lowered.scene);

    input["behavior"] = json!({
        "models": [{ "id": "empty-model", "properties": [] }],
        "bindings": [],
        "statecharts": [{
            "id": "minimal", "initial": "reading",
            "inputs": [], "events": [], "transitions": [],
            "states": [{ "id": "reading", "motion": "calm-track" }],
            "listeners": [{
                "id": "noop", "target": "needle", "listener_type": "down", "actions": []
            }],
            "regions": [{
                "id": "pulse", "initial": "reading", "transitions": [],
                "states": [{ "id": "reading", "motion": "calm-track" }]
            }]
        }]
    });
    let lowered = lower_authoring_json(&input.to_string()).expect("empty optional collections");
    assert_builds(lowered.scene);
}

#[test]
fn the_limit_is_per_collection_not_a_statechart_total() {
    let mut input = boundary_document();
    resize_collection(&mut input, "/behavior/statecharts/0/events", 1000);
    resize_collection(&mut input, "/behavior/statecharts", 2);
    let lowered = lower_authoring_json(&input.to_string()).expect("two bounded event collections");
    assert_builds(lowered.scene);
}

#[test]
fn raw_state_machines_do_not_inherit_the_typed_collection_limit() {
    let mut input = document();
    input["behavior"]["raw_state_machines"] = json!(
        (0..1001)
            .map(|index| json!({
                "id": format!("legacy-{index}"),
                "value": {
                    "name": format!("legacy_machine_{index}"),
                    "layers": [{
                        "states": [{ "type": "entry" }, { "type": "exit" }],
                        "transitions": [{ "from": 0, "to": 1 }]
                    }]
                }
            }))
            .collect::<Vec<_>>()
    );
    let lowered = lower_authoring_json(&input.to_string()).expect("unbounded raw escape list");
    assert_builds(lowered.scene);
}

#[test]
fn an_oversized_statechart_replacement_rolls_back_the_operation_batch() {
    let spec: AuthoringSpec = serde_json::from_value(document()).expect("typed document");
    let snapshot = serde_json::to_value(&spec).expect("original document");
    let before = lower_authoring(&spec).expect("original lowering");
    let mut replacement = boundary_document();
    resize_collection(&mut replacement, "/behavior/statecharts/0/events", 1001);
    let operations = [
        AuthoringOperation::Remove {
            target: AuthoringTarget::BehaviorStatechart {
                target_id: "meter".to_string(),
            },
        },
        AuthoringOperation::Insert {
            entity: AuthoringEntity::BehaviorStatechart(
                serde_json::from_value(replacement["behavior"]["statecharts"][0].clone())
                    .expect("replacement statechart"),
            ),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::BehaviorStatecharts,
            },
        },
    ];
    let error = apply_operations(&spec, &operations).expect_err("oversized replacement");
    assert_eq!(error.diagnostics.len(), 1);
    let diagnostic = &error.diagnostics[0];
    assert_eq!(diagnostic.code, "invalid_behavior_collection_count");
    assert_eq!(diagnostic.path, "$.behavior.statecharts[0].events");
    assert_eq!(
        serde_json::to_value(&spec).expect("unchanged spec"),
        snapshot
    );
    let after = lower_authoring(&spec).expect("original remains valid");
    assert_eq!(before.scene, after.scene);
    assert_eq!(before.source_map, after.source_map);
}

#[test]
fn behavior_collection_bounds_match_the_published_schema() {
    let schema = authoring_schema();
    for (owner, fields) in [
        ("BehaviorSection", &["models", "bindings", "statecharts"][..]),
        ("BehaviorModelSpec", &["properties"][..]),
        ("BehaviorListenerSpec", &["actions"][..]),
        ("BehaviorRegionSpec", &["states", "transitions"][..]),
        (
            "BehaviorStatechartSpec",
            &[
                "inputs",
                "events",
                "listeners",
                "states",
                "transitions",
                "regions",
            ][..],
        ),
    ] {
        let properties = &schema["$defs"][owner]["properties"];
        for &field in fields {
            assert_eq!(properties[field]["maxItems"], 1000);
        }
    }
    for owner in ["BehaviorStatechartSpec", "BehaviorRegionSpec"] {
        let states = &schema["$defs"][owner]["properties"]["states"];
        assert_eq!(states["minItems"], 1);
    }
    let raw = &schema["$defs"]["BehaviorSection"]["properties"]["raw_state_machines"];
    assert!(raw.get("maxItems").is_none());
}
