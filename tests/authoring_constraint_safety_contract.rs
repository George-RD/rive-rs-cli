use rive_cli::authoring::{AuthoringError, authoring_schema, lower_authoring_json};
use serde_json::{Value, json};

const MAX_CONSTRAINTS: usize = 100;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn rectangle(id: &str) -> Value {
    rectangle_at(id, 0.0, 0.0)
}

fn rectangle_at(id: &str, x: f64, y: f64) -> Value {
    json!({
        "kind": "rectangle",
        "id": id,
        "width": literal(16.0, "px"),
        "height": literal(12.0, "px"),
        "fill": "#2563EB",
        "transform": {
            "x": literal(x, "px"),
            "y": literal(y, "px")
        }
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

fn diagnostic<'a>(
    error: &'a AuthoringError,
    code: &str,
) -> &'a rive_cli::authoring::AuthoringDiagnostic {
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
fn axis_constraints_apply_to_y_without_changing_x() {
    let children = vec![
        rectangle_at("top", 5.0, 10.0),
        rectangle_at("bottom", 100.0, 90.0),
        rectangle_at("aligned", 20.0, 0.0),
        rectangle_at("centered", 30.0, 0.0),
        rectangle_at("spaced-a", 40.0, 20.0),
        rectangle_at("spaced-b", 50.0, 0.0),
        rectangle_at("spaced-c", 60.0, 0.0),
    ];
    let constraints = vec![
        json!({
            "kind": "align",
            "id": "align-y",
            "subject": "aligned",
            "target": "top",
            "axis": "y"
        }),
        json!({
            "kind": "center",
            "id": "center-y",
            "subject": "centered",
            "start": "top",
            "end": "bottom",
            "axis": "y"
        }),
        json!({
            "kind": "spacing",
            "id": "spacing-y",
            "items": ["spaced-a", "spaced-b", "spaced-c"],
            "axis": "y",
            "gap": literal(15.0, "px")
        }),
    ];

    let lowered = lower_authoring_json(&document(children, constraints))
        .expect("y-axis constraints must lower");
    let nodes = lowered.scene["artboard"]["children"][0]["children"]
        .as_array()
        .expect("lowered group children");
    let expected = [
        (5.0, 10.0),
        (100.0, 90.0),
        (20.0, 10.0),
        (30.0, 50.0),
        (40.0, 20.0),
        (50.0, 35.0),
        (60.0, 50.0),
    ];
    for (index, (x, y)) in expected.into_iter().enumerate() {
        assert_eq!(nodes[index]["x"], x);
        assert_eq!(nodes[index]["y"], y);
    }
}

#[test]
fn group_constraint_count_is_bounded_in_schema_and_runtime() {
    let schema = authoring_schema();
    let group = schema["$defs"]["VisualNode"]["oneOf"]
        .as_array()
        .expect("visual variants")
        .iter()
        .find(|variant| variant["properties"]["kind"]["const"] == "group")
        .expect("group schema variant");
    assert_eq!(
        group["properties"]["constraints"]["maxItems"],
        json!(MAX_CONSTRAINTS)
    );

    let children = (0..=MAX_CONSTRAINTS)
        .map(|index| rectangle(&format!("node-{index}")))
        .collect::<Vec<_>>();
    let constraints = (0..=MAX_CONSTRAINTS)
        .map(|index| {
            json!({
                "kind": "align",
                "id": format!("constraint-{index}"),
                "subject": format!("node-{index}"),
                "target": format!("node-{MAX_CONSTRAINTS}"),
                "axis": "x"
            })
        })
        .collect::<Vec<_>>();

    let error = lower_authoring_json(&document(children, constraints))
        .expect_err("oversized constraint lists must fail");
    let diagnostic = diagnostic(&error, "invalid_constraint_count");
    assert_eq!(diagnostic.path, "$.visual.nodes[0].constraints");
    assert!(diagnostic.message.contains("100"));
}

#[test]
fn constraint_dependency_depth_is_bounded() {
    let children = (0..=101)
        .map(|index| rectangle(&format!("node-{index}")))
        .collect::<Vec<_>>();
    let spacing_items = (2..=101)
        .rev()
        .map(|index| format!("node-{index}"))
        .collect::<Vec<_>>();
    let constraints = vec![
        json!({
            "kind": "spacing",
            "id": "long-chain",
            "items": spacing_items,
            "axis": "x",
            "gap": literal(1.0, "px")
        }),
        json!({
            "kind": "align",
            "id": "link-one",
            "subject": "node-1",
            "target": "node-2",
            "axis": "x"
        }),
        json!({
            "kind": "align",
            "id": "link-zero",
            "subject": "node-0",
            "target": "node-1",
            "axis": "x"
        }),
    ];

    let error = lower_authoring_json(&document(children, constraints))
        .expect_err("deep dependency chains must be bounded");
    let diagnostic = diagnostic(&error, "constraint_resolution_depth_limit");
    assert_eq!(diagnostic.path, "$.visual.nodes[0].constraints[0].items[1]");
    assert!(diagnostic.message.contains("100"));
}

#[test]
fn constraint_dependency_depth_is_bounded_after_memoized_prefixes() {
    let children = (0..=101)
        .map(|index| rectangle(&format!("node-{index}")))
        .collect::<Vec<_>>();
    let spacing_items = (0..100)
        .map(|index| format!("node-{index}"))
        .collect::<Vec<_>>();
    let constraints = vec![
        json!({
            "kind": "spacing",
            "id": "memoized-prefix",
            "items": spacing_items,
            "axis": "x",
            "gap": literal(1.0, "px")
        }),
        json!({
            "kind": "align",
            "id": "link-one-hundred",
            "subject": "node-100",
            "target": "node-99",
            "axis": "x"
        }),
        json!({
            "kind": "align",
            "id": "link-one-hundred-one",
            "subject": "node-101",
            "target": "node-100",
            "axis": "x"
        }),
    ];

    let error = lower_authoring_json(&document(children, constraints))
        .expect_err("memoized prefixes must not hide an overlong dependency chain");
    let diagnostic = diagnostic(&error, "constraint_resolution_depth_limit");
    assert_eq!(diagnostic.path, "$.visual.nodes[0].constraints[0].items[1]");
    assert!(diagnostic.message.contains("100"));
}
