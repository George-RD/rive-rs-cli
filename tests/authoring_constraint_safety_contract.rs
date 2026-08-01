mod support;

use rive_cli::authoring::{AuthoringError, lower_authoring_json};
use serde_json::{Value, json};

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn rectangle(id: &str) -> Value {
    json!({
        "kind": "rectangle",
        "id": id,
        "width": literal(16.0, "px"),
        "height": literal(12.0, "px"),
        "fill": "#2563EB"
    })
}

fn document(children: Vec<Value>, constraints: Vec<Value>) -> String {
    json!({
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
                    "id": "layout",
                    "children": children,
                    "constraints": constraints
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

fn diagnostic<'a>(error: &'a AuthoringError, code: &str) -> &'a rive_cli::authoring::AuthoringDiagnostic {
    error
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == code)
        .unwrap_or_else(|| panic!("missing diagnostic {code}: {:?}", error.diagnostics))
}

#[test]
fn constraint_ids_have_stable_validation_diagnostics() {
    let children = vec![rectangle("a"), rectangle("b")];

    let error = lower_authoring_json(&document(
        children.clone(),
        vec![json!({
            "kind": "align",
            "id": "bad/id",
            "subject": "a",
            "target": "b",
            "axis": "x"
        })],
    ))
    .expect_err("reserved separators must fail");
    assert!(
        diagnostic(&error, "invalid_constraint_id")
            .path
            .ends_with(".constraints[0].id")
    );

    let error = lower_authoring_json(&document(
        children.clone(),
        vec![json!({
            "kind": "align",
            "id": "   ",
            "subject": "a",
            "target": "b",
            "axis": "x"
        })],
    ))
    .expect_err("blank ids must fail");
    assert!(
        diagnostic(&error, "invalid_constraint_id")
            .path
            .ends_with(".constraints[0].id")
    );

    let error = lower_authoring_json(&document(
        children,
        vec![
            json!({
                "kind": "align",
                "id": "same",
                "subject": "a",
                "target": "b",
                "axis": "x"
            }),
            json!({
                "kind": "align",
                "id": "same",
                "subject": "a",
                "target": "b",
                "axis": "y"
            }),
        ],
    ))
    .expect_err("duplicate ids must fail");
    assert!(
        diagnostic(&error, "duplicate_constraint_id")
            .path
            .ends_with(".constraints[1].id")
    );
}

#[test]
fn constraint_dependency_depth_is_bounded() {
    const ASSIGNMENTS: usize = 101;

    let children = (0..=ASSIGNMENTS)
        .map(|index| rectangle(&format!("node-{index}")))
        .collect::<Vec<_>>();
    let constraints = (0..ASSIGNMENTS)
        .map(|index| {
            json!({
                "kind": "align",
                "id": format!("link-{index}"),
                "subject": format!("node-{index}"),
                "target": format!("node-{}", index + 1),
                "axis": "x"
            })
        })
        .collect::<Vec<_>>();

    let error = lower_authoring_json(&document(children, constraints))
        .expect_err("deep dependency chains must be bounded");
    let diagnostic = diagnostic(&error, "constraint_resolution_depth_limit");
    assert_eq!(
        diagnostic.path,
        "$.visual.nodes[0].constraints[100].subject"
    );
    assert!(diagnostic.message.contains("100"));
}
