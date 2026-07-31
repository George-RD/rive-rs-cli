mod support;

use std::{collections::HashSet, fs, path::Path};

use rive_cli::authoring::{authoring_schema, lower_authoring_json};
use support::assert_builds;

const COMPONENT_SCENE: &str = r##"
{
  "authoring_format_version": 0,
  "artboard": {
    "id": "main-stage",
    "width": { "value": 320.0, "unit": "px" },
    "height": { "value": 240.0, "unit": "px" }
  },
  "parameters": {
    "gap": { "value": 24.0, "unit": "px" }
  },
  "components": [
    {
      "id": "badge",
      "parameters": {
        "diameter": { "value": 64.0, "unit": "px" }
      },
      "visual": [
        {
          "kind": "ellipse",
          "id": "disc",
          "width": { "kind": "parameter", "name": "diameter" },
          "height": { "kind": "parameter", "name": "diameter" },
          "fill": "#246BFD"
        }
      ]
    }
  ],
  "visual": {
    "nodes": [
      {
        "kind": "instance",
        "id": "left",
        "component": "badge",
        "overrides": {
          "diameter": { "value": 72.0, "unit": "px" }
        },
        "transform": {
          "x": { "kind": "literal", "value": 80.0, "unit": "px" },
          "y": { "kind": "literal", "value": 120.0, "unit": "px" }
        }
      },
      {
        "kind": "instance",
        "id": "right",
        "component": "badge",
        "transform": {
          "x": {
            "kind": "add",
            "left": { "kind": "literal", "value": 200.0, "unit": "px" },
            "right": { "kind": "parameter", "name": "gap" }
          },
          "y": { "kind": "literal", "value": 120.0, "unit": "px" }
        }
      }
    ]
  },
  "motion": {},
  "behavior": {}
}
"##;

const RAW_ESCAPE_SCENE: &str = r##"
{
  "authoring_format_version": 0,
  "artboard": {
    "id": "raw-stage",
    "width": { "value": 240.0, "unit": "px" },
    "height": { "value": 240.0, "unit": "px" }
  },
  "visual": {
    "nodes": [
      {
        "kind": "raw_scene_object",
        "id": "raw-ball",
        "object": {
          "type": "shape",
          "name": "RawBall",
          "x": 120.0,
          "y": 120.0,
          "children": [
            {
              "type": "ellipse",
              "name": "RawBallGeometry",
              "width": 80.0,
              "height": 80.0,
              "origin_x": 0.5,
              "origin_y": 0.5
            },
            {
              "type": "fill",
              "name": "RawBallFill",
              "children": [
                {
                  "type": "solid_color",
                  "name": "RawBallColor",
                  "color": "#EF4444"
                }
              ]
            }
          ]
        }
      }
    ]
  },
  "motion": {
    "raw_animations": [
      {
        "id": "pulse-motion",
        "value": {
          "name": "pulse",
          "fps": 60,
          "duration": 60,
          "keyframes": [
            {
              "object": "RawBallGeometry",
              "property": "width",
              "frames": [
                { "frame": 0, "value": 80.0 },
                { "frame": 30, "value": 120.0 },
                { "frame": 59, "value": 80.0 }
              ]
            }
          ]
        }
      }
    ]
  },
  "behavior": {}
}
"##;

#[test]
fn component_scene_lowers_deterministically_and_builds() {
    let first = lower_authoring_json(COMPONENT_SCENE).expect("first lowering");
    let second = lower_authoring_json(COMPONENT_SCENE).expect("second lowering");

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);
    assert_builds(first.scene);
}

#[test]
fn raw_escape_scene_lowers_deterministically_and_builds() {
    let first = lower_authoring_json(RAW_ESCAPE_SCENE).expect("first lowering");
    let second = lower_authoring_json(RAW_ESCAPE_SCENE).expect("second lowering");

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);
    assert_eq!(first.scene["artboard"]["animations"][0]["name"], "pulse");
    assert_builds(first.scene);
}

#[test]
fn source_map_tracks_generated_runtime_names_and_expansion_paths() {
    let lowered = lower_authoring_json(COMPONENT_SCENE).expect("lowering");
    let expanded = lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "left/disc")
        .expect("expanded component source-map entry");

    assert!(
        expanded
            .definition_path
            .as_deref()
            .is_some_and(|path| path.contains("components[0]"))
    );
    assert!(expanded.authored_path.contains("visual.nodes[0]"));
    assert_eq!(expanded.runtime_names.len(), 4);
    assert_eq!(expanded.scene_paths.len(), 4);

    let names = lowered
        .source_map
        .entries
        .iter()
        .flat_map(|entry| entry.runtime_names.iter())
        .collect::<Vec<_>>();
    let unique = names.iter().copied().collect::<HashSet<_>>();
    assert_eq!(names.len(), unique.len());
}

#[test]
fn schema_is_versioned_and_exposes_explicit_authored_graphs() {
    let schema = authoring_schema();
    assert_eq!(
        schema["$id"],
        "https://github.com/George-RD/rive-rs-cli/docs/authoring.schema.v0.json"
    );
    assert_eq!(schema["title"], "rive-cli AuthoringSpec v0");

    let text = serde_json::to_string(&schema).expect("serialize schema");
    for required in [
        "authoring_format_version",
        "visual",
        "motion",
        "behavior",
        "additionalProperties",
    ] {
        assert!(text.contains(required), "schema is missing {required}");
    }
}

fn authoring_schema_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/authoring.schema.v0.json")
}

#[test]
fn published_authoring_schema_matches_generated_contract() {
    let schema_path = authoring_schema_path();
    let published: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&schema_path).expect("read published authoring schema"),
    )
    .expect("published authoring schema must be valid JSON");
    let generated = authoring_schema();

    assert_eq!(
        published,
        generated,
        "published authoring schema differs from authoring_schema(); regenerate {}",
        schema_path.display()
    );
}

#[test]
#[ignore = "regenerates docs/authoring.schema.v0.json"]
fn regenerate_published_authoring_schema() {
    fs::write(
        authoring_schema_path(),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&authoring_schema())
                .expect("serialize generated authoring schema")
        ),
    )
    .expect("write published authoring schema");
}

#[test]
fn unknown_component_reports_the_authored_path() {
    let input = COMPONENT_SCENE.replace("\"component\": \"badge\"", "\"component\": \"missing\"");
    let error = lower_authoring_json(&input).expect_err("unknown component must fail");

    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unknown_component" && diagnostic.path == "$.visual.nodes[0].component"
    }));
}

#[test]
fn incompatible_expression_units_report_the_operand_path() {
    let input = r##"
    {
      "authoring_format_version": 0,
      "artboard": {
        "id": "units",
        "width": { "value": 100.0, "unit": "px" },
        "height": { "value": 100.0, "unit": "px" }
      },
      "visual": {
        "nodes": [
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
      },
      "motion": {},
      "behavior": {}
    }
    "##;
    let error = lower_authoring_json(input).expect_err("unit mismatch must fail");

    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "unit_mismatch" && diagnostic.path == "$.visual.nodes[0].width.right"
    }));
}

#[test]
fn component_cycles_report_the_instance_path() {
    let input = r##"
    {
      "authoring_format_version": 0,
      "artboard": {
        "id": "cycle",
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
      "visual": {
        "nodes": [
          { "kind": "instance", "id": "root", "component": "a" }
        ]
      },
      "motion": {},
      "behavior": {}
    }
    "##;
    let error = lower_authoring_json(input).expect_err("component cycle must fail");

    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "component_cycle" && diagnostic.path.contains("component")
    }));
}

#[test]
fn unknown_fields_are_rejected_by_the_strict_frontend() {
    let input = COMPONENT_SCENE.replace(
        "\"authoring_format_version\": 0,",
        "\"authoring_format_version\": 0, \"surprise\": true,",
    );
    let error = lower_authoring_json(&input).expect_err("unknown field must fail");

    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "invalid_json"
            && diagnostic.path == "$"
            && diagnostic.message.contains("unknown field")
    }));
}
