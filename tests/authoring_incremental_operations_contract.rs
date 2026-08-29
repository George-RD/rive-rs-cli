mod support;

use rive_cli::authoring::{
    AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec,
    AuthoringTarget, BehaviorModelSpec, LoweredAuthoring, PoseSpec, SourceMapEntry, VisualNode,
    apply_operation, apply_operations, lower_authoring,
};
use serde_json::{Value, json};
use support::assert_builds;

const BASE_DOCUMENT: &str = r##"
{
  "authoring_format_version": 0,
  "artboard": {
    "id": "stage",
    "width": { "value": 320.0, "unit": "px" },
    "height": { "value": 240.0, "unit": "px" }
  },
  "visual": {
    "nodes": [
      {
        "kind": "group",
        "id": "frame",
        "children": [
          {
            "kind": "rectangle",
            "id": "panel",
            "width": { "kind": "literal", "value": 120.0, "unit": "px" },
            "height": { "kind": "literal", "value": 80.0, "unit": "px" },
            "fill": "#111827"
          }
        ]
      },
      {
        "kind": "rectangle",
        "id": "untouched",
        "width": { "kind": "literal", "value": 40.0, "unit": "px" },
        "height": { "kind": "literal", "value": 40.0, "unit": "px" },
        "fill": "#3B82F6"
      }
    ]
  },
  "motion": {},
  "behavior": {}
}
"##;

fn spec() -> AuthoringSpec {
    serde_json::from_str(BASE_DOCUMENT).expect("valid AuthoringSpec fixture")
}

fn dependency_spec() -> AuthoringSpec {
    serde_json::from_value(json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 240.0, "unit": "px" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "rectangle",
                    "id": "panel",
                    "width": { "kind": "literal", "value": 120.0, "unit": "px" },
                    "height": { "kind": "literal", "value": 80.0, "unit": "px" },
                    "fill": "#111827"
                }
            ]
        },
        "motion": {
            "poses": [
                {
                    "id": "rest",
                    "targets": [
                        {
                            "target": "panel",
                            "transform": {
                                "x": { "kind": "literal", "value": 0.0, "unit": "px" }
                            }
                        }
                    ]
                },
                {
                    "id": "moved",
                    "targets": [
                        {
                            "target": "panel",
                            "transform": {
                                "x": { "kind": "literal", "value": 40.0, "unit": "px" }
                            }
                        }
                    ]
                }
            ],
            "tracks": [
                {
                    "id": "pulse",
                    "fps": 60,
                    "duration_frames": {
                        "kind": "literal",
                        "value": 10.0,
                        "unit": "scalar"
                    },
                    "keyframes": [
                        {
                            "frame": { "kind": "literal", "value": 0.0, "unit": "scalar" },
                            "pose": "rest"
                        },
                        {
                            "frame": { "kind": "literal", "value": 10.0, "unit": "scalar" },
                            "pose": "moved"
                        }
                    ]
                }
            ]
        },
        "behavior": {
            "statecharts": [
                {
                    "id": "machine",
                    "initial": "idle",
                    "states": [
                        { "id": "idle", "motion": "pulse" }
                    ]
                }
            ]
        }
    }))
    .expect("valid dependency fixture")
}

fn ellipse(id: &str) -> VisualNode {
    serde_json::from_value(json!({
        "kind": "ellipse",
        "id": id,
        "width": { "kind": "literal", "value": 24.0, "unit": "px" },
        "height": { "kind": "literal", "value": 24.0, "unit": "px" },
        "fill": "#22C55E"
    }))
    .expect("valid visual node")
}

fn source_entry<'a>(lowered: &'a LoweredAuthoring, authored_id: &str) -> &'a SourceMapEntry {
    lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == authored_id)
        .unwrap_or_else(|| panic!("missing source-map entry for {authored_id}"))
}

fn has_diagnostic(error: &rive_cli::authoring::AuthoringError, code: &str, path: &str) -> bool {
    error
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == code && diagnostic.path == path)
}

fn insert_badge(document: &AuthoringSpec) -> AuthoringSpec {
    apply_operation(
        document,
        &AuthoringOperation::Insert {
            entity: AuthoringEntity::VisualNode(ellipse("badge")),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::VisualGroup {
                    target_id: "frame".to_string(),
                },
            },
        },
    )
    .expect("badge insert applies")
    .spec
}

#[test]
fn insert_visual_node_uses_authored_containment_and_preserves_unaffected_identity() {
    let document = spec();
    let before = lower_authoring(&document).expect("base document lowers");
    let untouched = source_entry(&before, "untouched").clone();
    let operation = AuthoringOperation::Insert {
        entity: AuthoringEntity::VisualNode(ellipse("badge")),
        placement: AuthoringPlacement::Into {
            container: AuthoringContainer::VisualGroup {
                target_id: "frame".to_string(),
            },
        },
    };

    let applied = apply_operation(&document, &operation).expect("valid insert applies");

    assert!(
        applied
            .lowered
            .source_map
            .entries
            .iter()
            .any(|entry| entry.authored_id == "frame/badge")
    );
    assert_eq!(source_entry(&applied.lowered, "untouched"), &untouched);
    assert_builds(applied.lowered.scene);
}

#[test]
fn invalid_insert_rolls_back_at_the_authored_boundary() {
    let document = spec();
    let before = serde_json::to_value(&document).expect("serialize input");
    let operation = AuthoringOperation::Insert {
        entity: AuthoringEntity::VisualNode(ellipse("panel")),
        placement: AuthoringPlacement::Into {
            container: AuthoringContainer::VisualGroup {
                target_id: "frame".to_string(),
            },
        },
    };

    let error = apply_operation(&document, &operation).expect_err("duplicate insert must fail");

    assert!(has_diagnostic(
        &error,
        "duplicate_id",
        "$.visual.nodes[0].children[1].id"
    ));
    assert_eq!(
        serde_json::to_value(&document).expect("serialize input after failure"),
        before
    );
}

#[test]
fn move_reorders_by_scoped_authored_id_without_changing_runtime_bindings() {
    let document = insert_badge(&spec());
    let before = lower_authoring(&document).expect("inserted document lowers");
    let badge_runtime = source_entry(&before, "frame/badge").runtime_names.clone();
    let panel_runtime = source_entry(&before, "frame/panel").runtime_names.clone();
    let operation = AuthoringOperation::Move {
        target: AuthoringTarget::VisualNode {
            target_id: "frame/badge".to_string(),
        },
        placement: AuthoringPlacement::Before {
            anchor: AuthoringTarget::VisualNode {
                target_id: "frame/panel".to_string(),
            },
        },
    };

    let applied = apply_operation(&document, &operation).expect("valid move applies");
    let value = serde_json::to_value(&applied.spec).expect("serialize moved spec");

    assert_eq!(value["visual"]["nodes"][0]["children"][0]["id"], "badge");
    assert_eq!(value["visual"]["nodes"][0]["children"][1]["id"], "panel");
    assert_eq!(
        source_entry(&applied.lowered, "frame/badge").runtime_names,
        badge_runtime
    );
    assert_eq!(
        source_entry(&applied.lowered, "frame/panel").runtime_names,
        panel_runtime
    );
    assert_builds(applied.lowered.scene);
}

#[test]
fn remove_restores_the_same_canonical_output_when_the_inserted_node_is_unreferenced() {
    let base = spec();
    let base_lowered = lower_authoring(&base).expect("base document lowers");
    let document = insert_badge(&base);
    let operation = AuthoringOperation::Remove {
        target: AuthoringTarget::VisualNode {
            target_id: "frame/badge".to_string(),
        },
    };

    let applied = apply_operation(&document, &operation).expect("valid remove applies");

    assert_eq!(applied.lowered, base_lowered);
}

#[test]
fn removing_a_visual_motion_target_fails_transactionally_with_authored_diagnostic() {
    let document = dependency_spec();
    let before = lower_authoring(&document).expect("dependency fixture lowers");
    let operation = AuthoringOperation::Remove {
        target: AuthoringTarget::VisualNode {
            target_id: "panel".to_string(),
        },
    };

    let error = apply_operation(&document, &operation).expect_err("dangling motion target must fail");

    assert!(has_diagnostic(
        &error,
        "unknown_motion_target",
        "$.motion.poses[0].targets[0].target"
    ));
    assert_eq!(
        lower_authoring(&document).expect("original document still lowers"),
        before
    );
}

#[test]
fn removing_motion_referenced_by_behavior_fails_without_silent_retargeting() {
    let document = dependency_spec();
    let operation = AuthoringOperation::Remove {
        target: AuthoringTarget::MotionTrack {
            target_id: "pulse".to_string(),
        },
    };

    let error =
        apply_operation(&document, &operation).expect_err("dangling behavior motion must fail");

    assert!(has_diagnostic(
        &error,
        "unknown_behavior_motion",
        "$.behavior.statecharts[0].states[0].motion"
    ));
}

#[test]
fn one_transaction_can_insert_typed_visual_motion_and_behavior_concepts() {
    let document = spec();
    let pose: PoseSpec = serde_json::from_value(json!({
        "id": "badge-pose",
        "targets": [
            {
                "target": "frame/badge",
                "opacity": { "kind": "literal", "value": 1.0, "unit": "scalar" }
            }
        ]
    }))
    .expect("valid pose");
    let model: BehaviorModelSpec = serde_json::from_value(json!({
        "id": "preferences",
        "properties": []
    }))
    .expect("valid behavior model");
    let operations = vec![
        AuthoringOperation::Insert {
            entity: AuthoringEntity::VisualNode(ellipse("badge")),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::VisualGroup {
                    target_id: "frame".to_string(),
                },
            },
        },
        AuthoringOperation::Insert {
            entity: AuthoringEntity::MotionPose(pose),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::MotionPoses,
            },
        },
        AuthoringOperation::Insert {
            entity: AuthoringEntity::BehaviorModel(model),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::BehaviorModels,
            },
        },
    ];

    let applied = apply_operations(&document, &operations).expect("cross-domain transaction applies");
    let value = serde_json::to_value(&applied.spec).expect("serialize applied spec");

    assert_eq!(value["motion"]["poses"][0]["id"], "badge-pose");
    assert_eq!(value["behavior"]["models"][0]["id"], "preferences");
    assert_builds(applied.lowered.scene);
}

#[test]
fn a_failing_multi_operation_sequence_exposes_no_partial_document() {
    let document = spec();
    let before = serde_json::to_value(&document).expect("serialize input");
    let operations = vec![
        AuthoringOperation::Insert {
            entity: AuthoringEntity::VisualNode(ellipse("badge")),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::VisualGroup {
                    target_id: "frame".to_string(),
                },
            },
        },
        AuthoringOperation::Insert {
            entity: AuthoringEntity::VisualNode(ellipse("panel")),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::VisualGroup {
                    target_id: "frame".to_string(),
                },
            },
        },
    ];

    let error = apply_operations(&document, &operations).expect_err("second operation must fail");

    assert!(has_diagnostic(
        &error,
        "duplicate_id",
        "$.visual.nodes[0].children[2].id"
    ));
    assert_eq!(
        serde_json::to_value(&document).expect("serialize input after failure"),
        before
    );
}

#[test]
fn placement_types_cannot_cross_authoring_domains() {
    let document = spec();
    let operation = AuthoringOperation::Move {
        target: AuthoringTarget::VisualNode {
            target_id: "frame/panel".to_string(),
        },
        placement: AuthoringPlacement::Into {
            container: AuthoringContainer::MotionPoses,
        },
    };

    let error = apply_operation(&document, &operation).expect_err("cross-domain move must fail");

    assert!(has_diagnostic(
        &error,
        "invalid_operation_placement",
        "$.visual.nodes"
    ));
}

#[test]
fn repeated_multi_operation_sequences_are_deterministic() {
    let document = spec();
    let operations = vec![
        AuthoringOperation::Insert {
            entity: AuthoringEntity::VisualNode(ellipse("badge")),
            placement: AuthoringPlacement::Into {
                container: AuthoringContainer::VisualGroup {
                    target_id: "frame".to_string(),
                },
            },
        },
        AuthoringOperation::Move {
            target: AuthoringTarget::VisualNode {
                target_id: "frame/badge".to_string(),
            },
            placement: AuthoringPlacement::Before {
                anchor: AuthoringTarget::VisualNode {
                    target_id: "frame/panel".to_string(),
                },
            },
        },
        AuthoringOperation::Remove {
            target: AuthoringTarget::VisualNode {
                target_id: "untouched".to_string(),
            },
        },
    ];

    let first = apply_operations(&document, &operations).expect("first sequence applies");
    let second = apply_operations(&document, &operations).expect("second sequence applies");

    assert_eq!(first.lowered, second.lowered);
    assert_eq!(
        serde_json::to_value(&first.spec).expect("serialize first spec"),
        serde_json::to_value(&second.spec).expect("serialize second spec")
    );
}
