mod support;

use rive_cli::authoring::{
    AuthoringOperation, AuthoringSpec, VisualNode, apply_operation, lower_authoring,
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
          },
          {
            "kind": "ellipse",
            "id": "badge",
            "width": { "kind": "literal", "value": 24.0, "unit": "px" },
            "height": { "kind": "literal", "value": 24.0, "unit": "px" },
            "fill": "#22C55E"
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

fn replacement(id: &str) -> VisualNode {
    serde_json::from_value(json!({
        "kind": "ellipse",
        "id": id,
        "width": { "kind": "literal", "value": 96.0, "unit": "px" },
        "height": { "kind": "literal", "value": 64.0, "unit": "px" },
        "fill": "#F59E0B"
    }))
    .expect("valid replacement node")
}

fn invalid_replacement() -> VisualNode {
    serde_json::from_value(json!({
        "kind": "ellipse",
        "id": "panel",
        "width": { "kind": "literal", "value": 96.0, "unit": "scalar" },
        "height": { "kind": "literal", "value": 64.0, "unit": "px" },
        "fill": "#F59E0B"
    }))
    .expect("syntactically valid replacement with invalid semantic units")
}

fn replace(target_id: &str, node: VisualNode) -> AuthoringOperation {
    AuthoringOperation::ReplaceVisualNode {
        target_id: target_id.to_string(),
        node,
    }
}

#[test]
fn replace_visual_node_validates_through_the_canonical_authoring_path() {
    let document = spec();
    let before = lower_authoring(&document).expect("base document lowers");

    let applied = apply_operation(&document, &replace("frame/panel", replacement("panel")))
        .expect("valid replacement applies");

    assert_ne!(applied.lowered.scene, before.scene);
    assert_builds(applied.lowered.scene);
}

#[test]
fn invalid_replace_rolls_back_without_mutating_the_input_document() {
    let document = spec();
    let before = serde_json::to_value(&document).expect("serialize input before operation");

    let error = apply_operation(&document, &replace("frame/panel", invalid_replacement()))
        .expect_err("invalid replacement must be rejected by canonical lowering");

    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unit_mismatch"
            && diagnostic.path == "$.visual.nodes[0].children[0].width"
    }));
    assert_eq!(
        serde_json::to_value(&document).expect("serialize input after failed operation"),
        before
    );
}

#[test]
fn replace_preserves_unaffected_source_map_identity() {
    let document = spec();
    let before = lower_authoring(&document).expect("base document lowers");
    let before_entry = before
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "untouched")
        .cloned()
        .expect("unaffected source-map entry before replace");

    let applied = apply_operation(&document, &replace("frame/panel", replacement("panel")))
        .expect("valid replacement applies");
    let after_entry = applied
        .lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "untouched")
        .cloned()
        .expect("unaffected source-map entry after replace");

    assert_eq!(after_entry, before_entry);
}

#[test]
fn replacing_the_same_target_is_deterministic_from_the_same_input() {
    let document = spec();
    let operation = replace("frame/panel", replacement("panel"));

    let first = apply_operation(&document, &operation).expect("first replacement applies");
    let second = apply_operation(&document, &operation).expect("second replacement applies");

    assert_eq!(first.lowered.scene, second.lowered.scene);
    assert_eq!(first.lowered.source_map, second.lowered.source_map);
    assert_eq!(
        serde_json::to_value(first.spec).expect("serialize first result"),
        serde_json::to_value(second.spec).expect("serialize second result")
    );
}

#[test]
fn nested_targets_require_their_scoped_authored_id() {
    let error = apply_operation(&spec(), &replace("panel", replacement("panel")))
        .expect_err("local nested id is not a stable authored target");

    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unknown_authored_id" && diagnostic.path == "$.visual.nodes"
    }));
}

#[test]
fn unknown_target_reports_an_authored_id_diagnostic() {
    let error = apply_operation(&spec(), &replace("missing", replacement("replacement")))
        .expect_err("unknown authored target must fail");

    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unknown_authored_id" && diagnostic.path == "$.visual.nodes"
    }));
}

#[test]
fn ambiguous_target_reports_an_authored_id_diagnostic() {
    let duplicate = BASE_DOCUMENT.replace("\"id\": \"badge\"", "\"id\": \"panel\"");
    let document: AuthoringSpec =
        serde_json::from_str(&duplicate).expect("duplicate IDs remain syntactically valid");

    let error = apply_operation(
        &document,
        &replace("frame/panel", replacement("replacement")),
    )
    .expect_err("ambiguous authored target must fail");

    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ambiguous_authored_id" && diagnostic.path == "$.visual.nodes"
    }));
}
