use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::{SceneSpec, build_scene};

const STROKED_COMPONENT_SCENE: &str = r##"
{
  "authoring_format_version": 0,
  "artboard": {
    "id": "stroke-stage",
    "width": { "value": 240.0, "unit": "px" },
    "height": { "value": 180.0, "unit": "px" }
  },
  "components": [
    {
      "id": "badge",
      "parameters": {
        "diameter": { "value": 72.0, "unit": "px" },
        "outline": { "value": 4.0, "unit": "px" }
      },
      "visual": [
        {
          "kind": "star",
          "id": "star",
          "width": { "kind": "parameter", "name": "diameter" },
          "height": { "kind": "parameter", "name": "diameter" },
          "points": 5,
          "inner_radius": { "kind": "literal", "value": 0.45, "unit": "scalar" },
          "fill": "#F59E0B",
          "stroke": {
            "color": "#0F172A",
            "width": { "kind": "parameter", "name": "outline" }
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
        "component": "badge",
        "overrides": {
          "diameter": { "value": 88.0, "unit": "px" },
          "outline": { "value": 6.0, "unit": "px" }
        },
        "transform": {
          "x": { "kind": "literal", "value": 120.0, "unit": "px" },
          "y": { "kind": "literal", "value": 90.0, "unit": "px" }
        }
      }
    ]
  },
  "motion": {},
  "behavior": {}
}
"##;

fn assert_builds(scene: serde_json::Value) {
    let scene: SceneSpec =
        serde_json::from_value(scene).expect("lowered SceneSpec must deserialize");
    build_scene(&scene, None).expect("lowered SceneSpec must pass the canonical builder");
}

#[test]
fn typed_stroke_lowers_through_components_deterministically_and_builds() {
    let first = lower_authoring_json(STROKED_COMPONENT_SCENE).expect("first stroke lowering");
    let second = lower_authoring_json(STROKED_COMPONENT_SCENE).expect("second stroke lowering");

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let children = first.scene["artboard"]["children"][0]["children"]
        .as_array()
        .expect("shape children");
    assert_eq!(children[2]["type"], "stroke");
    assert_eq!(children[2]["thickness"], 6.0);
    assert_eq!(children[2]["children"][0]["type"], "solid_color");
    assert_eq!(children[2]["children"][0]["color"], "#0F172A");

    let expanded = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "left/star")
        .expect("expanded stroked shape source-map entry");
    assert_eq!(expanded.runtime_names.len(), 6);
    assert_eq!(expanded.scene_paths.len(), 6);

    assert_builds(first.scene);
}

#[test]
fn stroke_width_requires_positive_pixels_at_the_authored_path() {
    for (width, expected_code) in [
        (
            r#"{ "kind": "literal", "value": 0.0, "unit": "px" }"#,
            "invalid_dimension",
        ),
        (
            r#"{ "kind": "literal", "value": 2.0, "unit": "scalar" }"#,
            "unit_mismatch",
        ),
    ] {
        let input =
            STROKED_COMPONENT_SCENE.replace(r#"{ "kind": "parameter", "name": "outline" }"#, width);
        let error = lower_authoring_json(&input).expect_err("invalid stroke width must fail");

        assert!(error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == expected_code
                && diagnostic.path == "$.components[0].visual[0].stroke.width"
        }));
    }
}
