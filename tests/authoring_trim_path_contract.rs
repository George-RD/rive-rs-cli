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

fn trim(start: Value, end: Value, offset: Option<Value>, mode: &str) -> Value {
    let mut trim = json!({
        "start": start,
        "end": end,
        "mode": mode
    });
    if let Some(offset) = offset {
        trim["offset"] = offset;
    }
    trim
}

fn component_scene(trim: Value) -> String {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "trimstage",
            "width": { "value": 240.0, "unit": "px" },
            "height": { "value": 180.0, "unit": "px" }
        },
        "components": [
            {
                "id": "ring",
                "parameters": {
                    "diameter": { "value": 72.0, "unit": "px" },
                    "outline": { "value": 4.0, "unit": "px" },
                    "trim_start": { "value": 0.1, "unit": "scalar" },
                    "trim_end": { "value": 0.8, "unit": "scalar" },
                    "trim_offset": { "value": 0.05, "unit": "scalar" }
                },
                "visual": [
                    {
                        "kind": "ellipse",
                        "id": "ring",
                        "width": parameter("diameter"),
                        "height": parameter("diameter"),
                        "fill": "#0F172A",
                        "stroke": {
                            "paint": "#F8FAFC",
                            "width": parameter("outline"),
                            "trim": trim
                        }
                    }
                ]
            }
        ],
        "visual": {
            "nodes": [
                {
                    "kind": "instance",
                    "id": "left",
                    "component": "ring",
                    "overrides": {
                        "diameter": { "value": 88.0, "unit": "px" },
                        "outline": { "value": 6.0, "unit": "px" },
                        "trim_start": { "value": 0.2, "unit": "scalar" },
                        "trim_end": { "value": 0.9, "unit": "scalar" },
                        "trim_offset": { "value": 1.25, "unit": "scalar" }
                    },
                    "transform": {
                        "x": literal(120.0, "px"),
                        "y": literal(90.0, "px")
                    }
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

fn standard_trim() -> Value {
    trim(
        parameter("trim_start"),
        parameter("trim_end"),
        Some(parameter("trim_offset")),
        "synchronized",
    )
}

#[test]
fn trim_paths_lower_through_components_deterministically_and_build() {
    let input = component_scene(standard_trim());
    let first = lower_authoring_json(&input).expect("first trim-path lowering");
    let second = lower_authoring_json(&input).expect("second trim-path lowering");

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let stroke = &first.scene["artboard"]["children"][0]["children"][0]["children"][2];
    assert_eq!(stroke["type"], "stroke");
    assert_eq!(stroke["children"].as_array().map(Vec::len), Some(2));

    let trim = &stroke["children"][1];
    assert_eq!(trim["type"], "trim_path");
    assert_eq!(trim["name"], "auth__trimstage__left__ring__stroke_trim");
    assert_eq!(trim["start"], 0.2);
    assert_eq!(trim["end"], 0.9);
    assert_eq!(trim["offset"], 1.25);
    assert_eq!(trim["mode"], "synchronized");

    let expanded = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "left/ring")
        .expect("expanded trimmed shape source-map entry");
    assert_eq!(expanded.runtime_names.len(), 7);
    assert_eq!(expanded.scene_paths.len(), 7);
    assert!(
        expanded
            .runtime_names
            .iter()
            .any(|name| name == "auth__trimstage__left__ring__stroke_trim")
    );
    assert!(
        expanded
            .scene_paths
            .iter()
            .any(|path| path == "/artboard/children/0/children/0/children/2/children/1")
    );

    assert_builds(first.scene);
}

#[test]
fn trim_offset_defaults_to_zero() {
    let input = component_scene(trim(
        parameter("trim_start"),
        parameter("trim_end"),
        None,
        "sequential",
    ));
    let lowered = lower_authoring_json(&input).expect("trim path without offset");
    let trim =
        &lowered.scene["artboard"]["children"][0]["children"][0]["children"][2]["children"][1];

    assert_eq!(trim["offset"], 0.0);
    assert_eq!(trim["mode"], "sequential");
    assert_builds(lowered.scene);
}

#[test]
fn trim_contract_errors_point_to_authored_expressions() {
    let cases = [
        (
            trim(
                literal(-0.1, "scalar"),
                parameter("trim_end"),
                None,
                "sequential",
            ),
            "invalid_ratio",
            "$.components[0].visual[0].stroke.trim.start",
        ),
        (
            trim(
                parameter("trim_start"),
                literal(1.1, "scalar"),
                None,
                "sequential",
            ),
            "invalid_ratio",
            "$.components[0].visual[0].stroke.trim.end",
        ),
        (
            trim(
                literal(10.0, "px"),
                parameter("trim_end"),
                None,
                "sequential",
            ),
            "unit_mismatch",
            "$.components[0].visual[0].stroke.trim.start",
        ),
        (
            trim(
                parameter("trim_start"),
                parameter("trim_end"),
                Some(literal(90.0, "degrees")),
                "sequential",
            ),
            "unit_mismatch",
            "$.components[0].visual[0].stroke.trim.offset",
        ),
    ];

    for (trim, expected_code, expected_path) in cases {
        let input = component_scene(trim);
        let error = lower_authoring_json(&input).expect_err("invalid trim path must fail");

        assert!(
            error.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == expected_code && diagnostic.path == expected_path
            }),
            "missing {expected_code} at {expected_path}; diagnostics: {:#?}",
            error.diagnostics
        );
    }
}

#[test]
fn stroke_schema_exposes_optional_typed_trim_paths() {
    let schema = authoring_schema();
    let stroke = &schema["$defs"]["StrokeSpec"];
    assert!(stroke["properties"].get("trim").is_some());
    assert!(
        stroke["required"]
            .as_array()
            .is_some_and(|required| required.iter().all(|field| field.as_str() != Some("trim")))
    );

    let trim = &schema["$defs"]["TrimPathSpec"];
    assert_eq!(trim["properties"]["start"]["$ref"], "#/$defs/ScalarExpr");
    assert_eq!(trim["properties"]["end"]["$ref"], "#/$defs/ScalarExpr");
    assert_eq!(trim["properties"]["mode"]["$ref"], "#/$defs/TrimPathMode");
    assert_eq!(
        schema["$defs"]["TrimPathMode"]["enum"],
        json!(["sequential", "synchronized"])
    );
}
