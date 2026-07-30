mod support;

use rive_cli::authoring::{authoring_schema, lower_authoring_json};
use serde_json::{Map, Value, json};
use support::assert_builds;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn parameter(name: &str) -> Value {
    json!({ "kind": "parameter", "name": name })
}

fn base_document(nodes: Vec<Value>) -> String {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "textstage",
            "width": { "value": 480.0, "unit": "px" },
            "height": { "value": 240.0, "unit": "px" }
        },
        "visual": { "nodes": nodes },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

fn simple_text(id: &str, extra: Map<String, Value>) -> Value {
    let mut node = json!({
        "kind": "text",
        "id": id,
        "text": "Hello Rive",
        "font_size": literal(24.0, "px"),
        "fill": "#0F172A"
    });
    node.as_object_mut()
        .expect("text node object")
        .extend(extra);
    node
}

fn component_document() -> String {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "textstage",
            "width": { "value": 480.0, "unit": "px" },
            "height": { "value": 240.0, "unit": "px" }
        },
        "components": [
            {
                "id": "label",
                "parameters": {
                    "font_size": { "value": 24.0, "unit": "px" },
                    "box_width": { "value": 280.0, "unit": "px" },
                    "box_height": { "value": 90.0, "unit": "px" },
                    "line_height": { "value": 1.25, "unit": "scalar" },
                    "tracking": { "value": 1.5, "unit": "px" },
                    "paragraph_spacing": { "value": 6.0, "unit": "px" },
                    "origin_x": { "value": 0.5, "unit": "scalar" },
                    "origin_y": { "value": 0.5, "unit": "scalar" },
                    "gradient_width": { "value": 280.0, "unit": "px" }
                },
                "visual": [
                    {
                        "kind": "text",
                        "id": "copy",
                        "text": "Rive from data",
                        "font_size": parameter("font_size"),
                        "fill": {
                            "kind": "linear_gradient",
                            "start_x": literal(0.0, "px"),
                            "start_y": literal(0.0, "px"),
                            "end_x": parameter("gradient_width"),
                            "end_y": literal(0.0, "px"),
                            "stops": [
                                {
                                    "color": "#22D3EE",
                                    "position": literal(0.0, "scalar")
                                },
                                {
                                    "color": "#7C3AED",
                                    "position": literal(1.0, "scalar")
                                }
                            ]
                        },
                        "width": parameter("box_width"),
                        "height": parameter("box_height"),
                        "line_height": parameter("line_height"),
                        "letter_spacing": parameter("tracking"),
                        "paragraph_spacing": parameter("paragraph_spacing"),
                        "origin_x": parameter("origin_x"),
                        "origin_y": parameter("origin_y"),
                        "align": "center",
                        "overflow": "ellipsis",
                        "transform": {
                            "x": literal(12.0, "px"),
                            "y": literal(-4.0, "px"),
                            "rotation": literal(5.0, "degrees")
                        }
                    }
                ]
            }
        ],
        "visual": {
            "nodes": [
                {
                    "kind": "instance",
                    "id": "hero",
                    "component": "label",
                    "overrides": {
                        "font_size": { "value": 32.0, "unit": "px" },
                        "box_width": { "value": 320.0, "unit": "px" },
                        "box_height": { "value": 100.0, "unit": "px" },
                        "line_height": { "value": 1.4, "unit": "scalar" },
                        "tracking": { "value": 2.0, "unit": "px" },
                        "paragraph_spacing": { "value": 8.0, "unit": "px" },
                        "gradient_width": { "value": 320.0, "unit": "px" }
                    },
                    "transform": {
                        "x": literal(240.0, "px"),
                        "y": literal(120.0, "px")
                    }
                }
            ]
        },
        "motion": {},
        "behavior": {}
    })
    .to_string()
}

#[test]
fn text_nodes_lower_through_components_deterministically_and_build() {
    let input = component_document();
    let first = lower_authoring_json(&input).expect("first text lowering");
    let second = lower_authoring_json(&input).expect("second text lowering");

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let anchor = &first.scene["artboard"]["children"][0]["children"][0];
    assert_eq!(anchor["type"], "node");
    assert_eq!(anchor["name"], "auth__textstage__hero__copy__text_anchor");
    assert_eq!(anchor["x"], 12.0);
    assert_eq!(anchor["y"], -4.0);

    let text = &anchor["children"][0];
    assert_eq!(text["type"], "text");
    assert_eq!(text["name"], "auth__textstage__hero__copy__text");
    assert_eq!(text["align_value"], 2);
    assert_eq!(text["sizing_value"], 2);
    assert_eq!(text["overflow_value"], 3);
    assert_eq!(text["width"], 320.0);
    assert_eq!(text["height"], 100.0);
    assert_eq!(text["origin_x"], 0.5);
    assert_eq!(text["origin_y"], 0.5);
    assert_eq!(text["paragraph_spacing"], 8.0);

    let style = &text["children"][0];
    assert_eq!(style["type"], "text_style");
    assert_eq!(style["name"], "auth__textstage__hero__copy__text_style");
    assert_eq!(style["font_size"], 32.0);
    assert_eq!(style["line_height"], 1.4);
    assert_eq!(style["letter_spacing"], 2.0);

    let fill = &style["children"][0];
    assert_eq!(fill["type"], "fill");
    assert_eq!(fill["name"], "auth__textstage__hero__copy__text_fill");
    let gradient = &fill["children"][0];
    assert_eq!(gradient["type"], "linear_gradient");
    assert_eq!(
        gradient["name"],
        "auth__textstage__hero__copy__text_gradient"
    );
    assert_eq!(gradient["end_x"], 320.0);
    assert_eq!(gradient["children"].as_array().map(Vec::len), Some(2));

    let run = &text["children"][1];
    assert_eq!(run["type"], "text_value_run");
    assert_eq!(run["name"], "auth__textstage__hero__copy__text_run");
    assert_eq!(run["text"], "Rive from data");
    assert_eq!(run["style"], "auth__textstage__hero__copy__text_style");

    let expanded = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "hero/copy")
        .expect("expanded text source-map entry");
    assert_eq!(expanded.runtime_names.len(), 8);
    assert_eq!(expanded.scene_paths.len(), 8);
    assert!(
        expanded
            .runtime_names
            .iter()
            .any(|name| name == "auth__textstage__hero__copy__text_run")
    );
    assert!(expanded.scene_paths.iter().any(|path| {
        path == "/artboard/children/0/children/0/children/0/children/1"
    }));

    assert_builds(first.scene);
}

#[test]
fn text_sizing_is_derived_from_authored_dimensions() {
    let auto_width = simple_text("auto", Map::new());

    let mut auto_height_fields = Map::new();
    auto_height_fields.insert("width".to_string(), literal(220.0, "px"));
    let auto_height = simple_text("wrap", auto_height_fields);

    let mut fixed_fields = Map::new();
    fixed_fields.insert("width".to_string(), literal(220.0, "px"));
    fixed_fields.insert("height".to_string(), literal(80.0, "px"));
    let fixed = simple_text("fixed", fixed_fields);

    let lowered = lower_authoring_json(&base_document(vec![auto_width, auto_height, fixed]))
        .expect("derived text sizing");

    let children = lowered.scene["artboard"]["children"]
        .as_array()
        .expect("artboard children");
    let auto_width_text = &children[0]["children"][0];
    let auto_height_text = &children[1]["children"][0];
    let fixed_text = &children[2]["children"][0];

    assert_eq!(auto_width_text["sizing_value"], 0);
    assert!(auto_width_text.get("width").is_none());
    assert!(auto_width_text.get("height").is_none());

    assert_eq!(auto_height_text["sizing_value"], 1);
    assert_eq!(auto_height_text["width"], 220.0);
    assert!(auto_height_text.get("height").is_none());

    assert_eq!(fixed_text["sizing_value"], 2);
    assert_eq!(fixed_text["width"], 220.0);
    assert_eq!(fixed_text["height"], 80.0);

    assert_builds(lowered.scene);
}

#[test]
fn text_contract_errors_point_to_authored_fields() {
    let cases = [
        (
            "font_size",
            literal(0.0, "px"),
            "invalid_dimension",
            "$.visual.nodes[0].font_size",
        ),
        (
            "font_size",
            literal(24.0, "scalar"),
            "unit_mismatch",
            "$.visual.nodes[0].font_size",
        ),
        (
            "width",
            literal(0.0, "px"),
            "invalid_dimension",
            "$.visual.nodes[0].width",
        ),
        (
            "line_height",
            literal(0.0, "scalar"),
            "invalid_dimension",
            "$.visual.nodes[0].line_height",
        ),
        (
            "line_height",
            literal(1.2, "px"),
            "unit_mismatch",
            "$.visual.nodes[0].line_height",
        ),
        (
            "origin_x",
            literal(1.1, "scalar"),
            "invalid_ratio",
            "$.visual.nodes[0].origin_x",
        ),
        (
            "origin_y",
            literal(0.5, "degrees"),
            "unit_mismatch",
            "$.visual.nodes[0].origin_y",
        ),
    ];

    for (field, value, expected_code, expected_path) in cases {
        let mut fields = Map::new();
        fields.insert(field.to_string(), value);
        let input = base_document(vec![simple_text("bad", fields)]);
        let error = lower_authoring_json(&input).expect_err("invalid text must fail");

        assert!(
            error.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == expected_code && diagnostic.path == expected_path
            }),
            "missing {expected_code} at {expected_path}; diagnostics: {:#?}",
            error.diagnostics
        );
    }

    let mut height_without_width = Map::new();
    height_without_width.insert("height".to_string(), literal(80.0, "px"));
    let error = lower_authoring_json(&base_document(vec![simple_text(
        "bad-layout",
        height_without_width,
    )]))
    .expect_err("height without width must fail");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "invalid_text_layout"
            && diagnostic.path == "$.visual.nodes[0].height"
    }));
}

#[test]
fn text_schema_exposes_semantic_fields_without_runtime_indices() {
    let schema = authoring_schema();
    assert_eq!(
        schema["$defs"]["TextAlign"]["enum"],
        json!(["left", "right", "center"])
    );
    assert_eq!(
        schema["$defs"]["TextOverflow"]["enum"],
        json!(["visible", "hidden", "clipped", "ellipsis", "fit", "fit_font_size"])
    );

    let text = schema["$defs"]["VisualNode"]["oneOf"]
        .as_array()
        .expect("visual variants")
        .iter()
        .find(|variant| variant["properties"]["kind"]["const"] == "text")
        .expect("text visual variant");
    let properties = text["properties"].as_object().expect("text properties");

    assert_eq!(properties["font_size"]["$ref"], "#/$defs/ScalarExpr");
    assert_eq!(properties["fill"]["$ref"], "#/$defs/PaintSpec");
    assert_eq!(properties["align"]["$ref"], "#/$defs/TextAlign");
    assert_eq!(properties["overflow"]["$ref"], "#/$defs/TextOverflow");
    assert!(!properties.contains_key("align_value"));
    assert!(!properties.contains_key("sizing_value"));
    assert!(!properties.contains_key("overflow_value"));

    let required = text["required"].as_array().expect("required text fields");
    for field in ["kind", "id", "text", "font_size", "fill"] {
        assert!(required.iter().any(|candidate| candidate == field));
    }
}
