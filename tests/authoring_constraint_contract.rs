mod support;

use rive_cli::authoring::{authoring_schema, lower_authoring_json};
use serde_json::{Value, json};
use support::assert_builds;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn parameter(name: &str) -> Value {
    json!({ "kind": "parameter", "name": name })
}

fn rectangle(id: &str, x: Value, y: Value) -> Value {
    json!({
        "kind": "rectangle",
        "id": id,
        "width": literal(16.0, "px"),
        "height": literal(12.0, "px"),
        "fill": "#2563EB",
        "transform": { "x": x, "y": y }
    })
}

fn group(id: &str, children: Vec<Value>, constraints: Vec<Value>) -> Value {
    json!({
        "kind": "group",
        "id": id,
        "children": children,
        "constraints": constraints
    })
}

fn document(nodes: Vec<Value>) -> String {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 240.0, "unit": "px" }
        },
        "visual": { "nodes": nodes },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

fn diagnostic<'a>(
    error: &'a rive_cli::authoring::AuthoringError,
    code: &str,
) -> &'a rive_cli::authoring::AuthoringDiagnostic {
    error
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == code)
        .unwrap_or_else(|| panic!("missing diagnostic {code}: {:?}", error.diagnostics))
}

#[test]
fn group_constraints_lower_deterministically_and_build() {
    let children = vec![
        rectangle("left", literal(20.0, "px"), literal(40.0, "px")),
        rectangle("right", literal(140.0, "px"), literal(90.0, "px")),
        rectangle("aligned", literal(0.0, "px"), literal(25.0, "px")),
        rectangle("centered", literal(0.0, "px"), literal(60.0, "px")),
        rectangle("offset", literal(0.0, "px"), literal(0.0, "px")),
        rectangle("spaced-a", literal(10.0, "px"), literal(120.0, "px")),
        rectangle("spaced-b", literal(-5.0, "px"), literal(130.0, "px")),
        rectangle("spaced-c", literal(-10.0, "px"), literal(140.0, "px")),
    ];
    let constraints = vec![
        json!({
            "kind": "align",
            "id": "align-x",
            "subject": "aligned",
            "target": "left",
            "axis": "x"
        }),
        json!({
            "kind": "center",
            "id": "center-x",
            "subject": "centered",
            "start": "left",
            "end": "right",
            "axis": "x"
        }),
        json!({
            "kind": "offset",
            "id": "offset-center",
            "subject": "offset",
            "target": "centered",
            "x": literal(10.0, "px"),
            "y": literal(-15.0, "px")
        }),
        json!({
            "kind": "spacing",
            "id": "space-row",
            "items": ["spaced-a", "spaced-b", "spaced-c"],
            "axis": "x",
            "gap": literal(30.0, "px")
        }),
    ];
    let mut constrained_group = group("layout", children, constraints);
    constrained_group["transform"] = json!({
        "x": literal(5.0, "px"),
        "y": literal(6.0, "px")
    });
    let input = document(vec![constrained_group]);

    let first = lower_authoring_json(&input).expect("first constrained lowering");
    let second = lower_authoring_json(&input).expect("second constrained lowering");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let wrapper = &first.scene["artboard"]["children"][0];
    assert_eq!(wrapper["name"], "auth__stage__layout__group");
    assert_eq!(wrapper["x"], 5.0);
    assert_eq!(wrapper["y"], 6.0);
    let lowered = wrapper["children"].as_array().expect("group children");
    let expected = [
        (20.0, 40.0),
        (140.0, 90.0),
        (20.0, 25.0),
        (80.0, 60.0),
        (90.0, 45.0),
        (10.0, 120.0),
        (40.0, 130.0),
        (70.0, 140.0),
    ];
    for (index, (x, y)) in expected.into_iter().enumerate() {
        assert_eq!(lowered[index]["x"], x);
        assert_eq!(lowered[index]["y"], y);
    }

    assert_builds(first.scene);
}

#[test]
fn constraints_use_component_parameters_and_instance_overrides() {
    let input = json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 240.0, "unit": "px" }
        },
        "components": [
            {
                "id": "row",
                "parameters": {
                    "start": { "value": 15.0, "unit": "px" },
                    "gap": { "value": 12.0, "unit": "px" }
                },
                "visual": [
                    group(
                        "layout",
                        vec![
                            rectangle("a", parameter("start"), literal(20.0, "px")),
                            rectangle("b", literal(0.0, "px"), literal(40.0, "px")),
                            rectangle("c", literal(0.0, "px"), literal(60.0, "px"))
                        ],
                        vec![json!({
                            "kind": "spacing",
                            "id": "row-gap",
                            "items": ["a", "b", "c"],
                            "axis": "x",
                            "gap": parameter("gap")
                        })]
                    )
                ]
            }
        ],
        "visual": {
            "nodes": [
                {
                    "kind": "instance",
                    "id": "hero",
                    "component": "row",
                    "overrides": {
                        "start": { "value": 25.0, "unit": "px" },
                        "gap": { "value": 20.0, "unit": "px" }
                    }
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string();

    let lowered = lower_authoring_json(&input).expect("component constraint lowering");
    let children = lowered.scene["artboard"]["children"][0]["children"][0]["children"]
        .as_array()
        .expect("expanded constrained group");
    assert_eq!(children[0]["x"], 25.0);
    assert_eq!(children[1]["x"], 45.0);
    assert_eq!(children[2]["x"], 65.0);
    assert_eq!(children[0]["y"], 20.0);
    assert_eq!(children[1]["y"], 40.0);
    assert_eq!(children[2]["y"], 60.0);

    let expanded = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "hero/layout/b")
        .expect("expanded constrained child source map");
    assert_eq!(
        expanded.definition_path.as_deref(),
        Some("$.components[0].visual[0].children[1]")
    );

    assert_builds(lowered.scene);
}

#[test]
fn constraint_cycles_report_the_authored_chain() {
    let constrained_group = group(
        "cycle",
        vec![
            rectangle("a", literal(0.0, "px"), literal(0.0, "px")),
            rectangle("b", literal(20.0, "px"), literal(0.0, "px")),
        ],
        vec![
            json!({
                "kind": "align",
                "id": "a-from-b",
                "subject": "a",
                "target": "b",
                "axis": "x"
            }),
            json!({
                "kind": "align",
                "id": "b-from-a",
                "subject": "b",
                "target": "a",
                "axis": "x"
            }),
        ],
    );

    let error = lower_authoring_json(&document(vec![constrained_group]))
        .expect_err("constraint cycle must fail");
    let diagnostic = diagnostic(&error, "constraint_cycle");
    assert!(diagnostic.path.starts_with("$.visual.nodes[0].constraints["));
    assert!(diagnostic.message.contains("a.x"));
    assert!(diagnostic.message.contains("b.x"));
}

#[test]
fn constraint_diagnostics_are_actionable() {
    let cases = vec![
        (
            "unknown_constraint_node",
            ".target",
            vec![json!({
                "kind": "align",
                "id": "missing-target",
                "subject": "a",
                "target": "missing",
                "axis": "x"
            })],
            vec![rectangle(
                "a",
                literal(0.0, "px"),
                literal(0.0, "px")
            )],
        ),
        (
            "constraint_conflict",
            ".subject",
            vec![
                json!({
                    "kind": "align",
                    "id": "first",
                    "subject": "a",
                    "target": "b",
                    "axis": "x"
                }),
                json!({
                    "kind": "align",
                    "id": "second",
                    "subject": "a",
                    "target": "c",
                    "axis": "x"
                }),
            ],
            vec![
                rectangle("a", literal(0.0, "px"), literal(0.0, "px")),
                rectangle("b", literal(10.0, "px"), literal(0.0, "px")),
                rectangle("c", literal(20.0, "px"), literal(0.0, "px")),
            ],
        ),
        (
            "invalid_constraint_items",
            ".items",
            vec![json!({
                "kind": "spacing",
                "id": "too-short",
                "items": ["a"],
                "axis": "x",
                "gap": literal(10.0, "px")
            })],
            vec![rectangle(
                "a",
                literal(0.0, "px"),
                literal(0.0, "px")
            )],
        ),
        (
            "duplicate_constraint_node",
            ".items[1]",
            vec![json!({
                "kind": "spacing",
                "id": "duplicate",
                "items": ["a", "a"],
                "axis": "x",
                "gap": literal(10.0, "px")
            })],
            vec![rectangle(
                "a",
                literal(0.0, "px"),
                literal(0.0, "px")
            )],
        ),
        (
            "unit_mismatch",
            ".gap",
            vec![json!({
                "kind": "spacing",
                "id": "bad-gap",
                "items": ["a", "b"],
                "axis": "x",
                "gap": literal(10.0, "degrees")
            })],
            vec![
                rectangle("a", literal(0.0, "px"), literal(0.0, "px")),
                rectangle("b", literal(0.0, "px"), literal(0.0, "px")),
            ],
        ),
    ];

    for (code, path_suffix, constraints, children) in cases {
        let error = lower_authoring_json(&document(vec![group(
            "invalid",
            children,
            constraints,
        )]))
        .expect_err("invalid constraint must fail");
        let diagnostic = diagnostic(&error, code);
        assert!(
            diagnostic.path.ends_with(path_suffix),
            "unexpected path for {code}: {}",
            diagnostic.path
        );
    }

    let raw_group = group(
        "raw",
        vec![
            json!({
                "kind": "raw_scene_object",
                "id": "raw-node",
                "object": {
                    "type": "node",
                    "name": "raw-node",
                    "x": 0.0,
                    "y": 0.0,
                    "rotation": 0.0,
                    "scale_x": 1.0,
                    "scale_y": 1.0,
                    "children": []
                }
            }),
            rectangle("typed", literal(0.0, "px"), literal(0.0, "px")),
        ],
        vec![json!({
            "kind": "align",
            "id": "raw-align",
            "subject": "raw-node",
            "target": "typed",
            "axis": "x"
        })],
    );
    let error = lower_authoring_json(&document(vec![raw_group]))
        .expect_err("raw nodes cannot participate in typed constraints");
    let diagnostic = diagnostic(&error, "unsupported_constraint_node");
    assert!(diagnostic.path.ends_with(".subject"));
}

#[test]
fn constraint_schema_exposes_only_semantic_fields() {
    let schema = authoring_schema();
    let group = schema["$defs"]["VisualNode"]["oneOf"]
        .as_array()
        .expect("visual variants")
        .iter()
        .find(|variant| variant["properties"]["kind"]["const"] == "group")
        .expect("group schema variant");
    let group_properties = group["properties"].as_object().expect("group properties");
    assert!(group_properties.contains_key("constraints"));
    assert_eq!(group_properties["constraints"]["default"], json!([]));
    assert!(!group["required"]
        .as_array()
        .expect("group required fields")
        .iter()
        .any(|field| field == "constraints"));

    let variants = schema["$defs"]["ConstraintSpec"]["oneOf"]
        .as_array()
        .expect("constraint variants");
    let kinds = variants
        .iter()
        .map(|variant| {
            variant["properties"]["kind"]["const"]
                .as_str()
                .expect("constraint kind")
        })
        .collect::<Vec<_>>();
    assert_eq!(kinds, vec!["align", "center", "offset", "spacing"]);

    for variant in variants {
        let kind = variant["properties"]["kind"]["const"]
            .as_str()
            .expect("constraint kind");
        let properties = variant["properties"]
            .as_object()
            .expect("constraint properties");
        assert!(properties.contains_key("id"));
        assert!(properties.contains_key("subject") || kind == "spacing");
        for field in ["runtime_name", "scene_path", "resolved_x", "resolved_y"] {
            assert!(!properties.contains_key(field));
        }
    }

    assert_eq!(
        schema["$defs"]["ConstraintAxis"]["enum"],
        json!(["x", "y"])
    );
}
