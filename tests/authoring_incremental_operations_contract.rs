mod support;

use rive_cli::authoring::{
    AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement, AuthoringSpec,
    VisualNode, apply_operation, lower_authoring,
};
use serde_json::json;
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

#[test]
fn insert_visual_node_uses_authored_containment_and_canonical_validation() {
    let document = spec();
    let before = lower_authoring(&document).expect("base document lowers");
    let operation = AuthoringOperation::Insert {
        entity: AuthoringEntity::VisualNode(ellipse("badge")),
        placement: AuthoringPlacement::Into {
            container: AuthoringContainer::VisualGroup {
                target_id: "frame".to_string(),
            },
        },
    };

    let applied = apply_operation(&document, &operation).expect("valid insert applies");

    assert_ne!(applied.lowered.scene, before.scene);
    assert!(
        applied
            .lowered
            .source_map
            .entries
            .iter()
            .any(|entry| entry.authored_id == "frame/badge")
    );
    assert_builds(applied.lowered.scene);
}
