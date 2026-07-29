use rive_cli::authoring::lower_authoring_json;

#[test]
fn out_of_range_authoring_number_reports_the_authored_value_path() {
    let input = r#"
    {
      "authoring_format_version": 0,
      "artboard": {
        "id": "overflow",
        "width": { "value": 1e100, "unit": "px" },
        "height": { "value": 100.0, "unit": "px" }
      },
      "visual": { "nodes": [] },
      "motion": {},
      "behavior": {}
    }
    "#;

    let error = lower_authoring_json(input).expect_err("f32 overflow must fail before lowering");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "numeric_out_of_range"
            && diagnostic.path == "$.artboard.width.value"
    }));
}

#[test]
fn unused_component_references_are_validated() {
    let input = r#"
    {
      "authoring_format_version": 0,
      "artboard": {
        "id": "unused-reference",
        "width": { "value": 100.0, "unit": "px" },
        "height": { "value": 100.0, "unit": "px" }
      },
      "components": [
        {
          "id": "unused",
          "visual": [
            { "kind": "instance", "id": "broken", "component": "missing" }
          ]
        }
      ],
      "visual": { "nodes": [] },
      "motion": {},
      "behavior": {}
    }
    "#;

    let error = lower_authoring_json(input).expect_err("unused component must be validated");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unknown_component"
            && diagnostic.path == "$.components[0].visual[0].component"
    }));
}

#[test]
fn unused_component_cycles_are_validated() {
    let input = r#"
    {
      "authoring_format_version": 0,
      "artboard": {
        "id": "unused-cycle",
        "width": { "value": 100.0, "unit": "px" },
        "height": { "value": 100.0, "unit": "px" }
      },
      "components": [
        {
          "id": "a",
          "visual": [
            { "kind": "instance", "id": "to-b", "component": "b" }
          ]
        },
        {
          "id": "b",
          "visual": [
            { "kind": "instance", "id": "to-a", "component": "a" }
          ]
        }
      ],
      "visual": { "nodes": [] },
      "motion": {},
      "behavior": {}
    }
    "#;

    let error = lower_authoring_json(input).expect_err("unused cycle must be validated");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "component_cycle"
            && diagnostic.path == "$.components[1].visual[0].component"
    }));
}

#[test]
fn unused_raw_component_objects_pass_canonical_scene_validation() {
    let input = r#"
    {
      "authoring_format_version": 0,
      "artboard": {
        "id": "unused-raw",
        "width": { "value": 100.0, "unit": "px" },
        "height": { "value": 100.0, "unit": "px" }
      },
      "components": [
        {
          "id": "unused",
          "visual": [
            {
              "kind": "raw_scene_object",
              "id": "bad-raw",
              "object": { "type": "not_a_scene_object", "name": "BadRaw" }
            }
          ]
        }
      ],
      "visual": { "nodes": [] },
      "motion": {},
      "behavior": {}
    }
    "#;

    let error = lower_authoring_json(input).expect_err("unused raw object must be validated");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "invalid_component_scene"
            && diagnostic.path == "$.components[0].visual"
    }));
}

#[test]
fn instantiated_component_errors_report_the_definition_path() {
    let input = r##"
    {
      "authoring_format_version": 0,
      "artboard": {
        "id": "definition-path",
        "width": { "value": 100.0, "unit": "px" },
        "height": { "value": 100.0, "unit": "px" }
      },
      "components": [
        {
          "id": "broken-box",
          "visual": [
            {
              "kind": "rectangle",
              "id": "box",
              "width": {
                "kind": "add",
                "left": { "kind": "literal", "value": 40.0, "unit": "px" },
                "right": { "kind": "literal", "value": 2.0, "unit": "scalar" }
              },
              "height": { "kind": "literal", "value": 40.0, "unit": "px" },
              "fill": "#111827"
            }
          ]
        }
      ],
      "visual": {
        "nodes": [
          { "kind": "instance", "id": "broken", "component": "broken-box" }
        ]
      },
      "motion": {},
      "behavior": {}
    }
    "##;

    let error = lower_authoring_json(input).expect_err("invalid component expression must fail");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unit_mismatch"
            && diagnostic.path == "$.components[0].visual[0].width.right"
    }));
}
