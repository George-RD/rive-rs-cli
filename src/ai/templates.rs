use crate::ai::AiError;

const BOUNCE_TEMPLATE: &str = r#"
{
  "scene_format_version": 1,
  "artboard": {
    "name": "BounceArtboard",
    "width": 500,
    "height": 500,
    "children": [
      {
        "type": "shape",
        "name": "BounceBall",
        "x": 150,
        "y": 250,
        "children": [
          {
            "type": "ellipse",
            "name": "BounceBallPath",
            "width": 100,
            "height": 100,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "BounceBallFill",
            "children": [
              {
                "type": "solid_color",
                "name": "BounceBallColor",
                "color": "FFFF0000"
              }
            ]
          }
        ]
      }
    ],
    "animations": [
      {
        "name": "bounce",
        "fps": 60,
        "duration": 120,
        "keyframes": [
          {
            "object": "BounceBallPath",
            "property": "x",
            "frames": [
              { "frame": 0, "value": 0.0 },
              { "frame": 60, "value": 200.0 },
              { "frame": 119, "value": 0.0 }
            ]
          }
        ]
      }
    ]
  }
}
"#;

const SPINNER_TEMPLATE: &str = r#"
{
  "scene_format_version": 1,
  "artboard": {
    "name": "SpinnerArtboard",
    "width": 500,
    "height": 500,
    "children": [
      {
        "type": "shape",
        "name": "SpinnerShape",
        "x": 250,
        "y": 250,
        "children": [
          {
            "type": "rectangle",
            "name": "SpinnerRect",
            "width": 120,
            "height": 30,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "SpinnerFill",
            "children": [
              {
                "type": "solid_color",
                "name": "SpinnerColor",
                "color": "FFFF8800"
              }
            ]
          }
        ]
      }
    ],
    "animations": [
      {
        "name": "spinner",
        "fps": 60,
        "duration": 60,
        "keyframes": [
          {
            "object": "SpinnerShape",
            "property": "x",
            "frames": [
              { "frame": 0, "value": 80.0 },
              { "frame": 15, "value": 0.0 },
              { "frame": 30, "value": -80.0 },
              { "frame": 45, "value": 0.0 },
              { "frame": 59, "value": 80.0 }
            ]
          },
          {
            "object": "SpinnerShape",
            "property": "y",
            "frames": [
              { "frame": 0, "value": 0.0 },
              { "frame": 15, "value": 80.0 },
              { "frame": 30, "value": 0.0 },
              { "frame": 45, "value": -80.0 },
              { "frame": 59, "value": 0.0 }
            ]
          }
        ]
      }
    ]
  }
}
"#;

const PULSE_TEMPLATE: &str = r#"
{
  "scene_format_version": 1,
  "artboard": {
    "name": "PulseArtboard",
    "width": 500,
    "height": 500,
    "children": [
      {
        "type": "shape",
        "name": "PulseShape",
        "x": 250,
        "y": 250,
        "children": [
          {
            "type": "ellipse",
            "name": "PulseCircle",
            "width": 80,
            "height": 80,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "PulseFill",
            "children": [
              {
                "type": "solid_color",
                "name": "PulseColor",
                "color": "FF0066FF"
              }
            ]
          }
        ]
      }
    ],
    "animations": [
      {
        "name": "pulse",
        "fps": 60,
        "duration": 60,
        "keyframes": [
          {
            "object": "PulseCircle",
            "property": "width",
            "frames": [
              { "frame": 0, "value": 80.0 },
              { "frame": 30, "value": 150.0 },
              { "frame": 59, "value": 80.0 }
            ]
          },
          {
            "object": "PulseCircle",
            "property": "height",
            "frames": [
              { "frame": 0, "value": 80.0 },
              { "frame": 30, "value": 150.0 },
              { "frame": 59, "value": 80.0 }
            ]
          }
        ]
      }
    ]
  }
}
"#;

const FADE_TEMPLATE: &str = r#"
{
  "scene_format_version": 1,
  "artboard": {
    "name": "FadeArtboard",
    "width": 500,
    "height": 500,
    "children": [
      {
        "type": "shape",
        "name": "FadeShape",
        "x": 250,
        "y": 250,
        "children": [
          {
            "type": "rectangle",
            "name": "FadeRect",
            "width": 180,
            "height": 120,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "FadeFill",
            "children": [
              {
                "type": "solid_color",
                "name": "FadeColor",
                "color": "FF00FF00"
              }
            ]
          }
        ]
      }
    ],
    "animations": [
      {
        "name": "fade",
        "fps": 60,
        "duration": 120,
        "keyframes": [
          {
            "object": "FadeColor",
            "property": "color",
            "frames": [
              { "frame": 0, "value": "FF00FF00" },
              { "frame": 60, "value": "6600FF00" },
              { "frame": 119, "value": "0000FF00" }
            ]
          }
        ]
      }
    ]
  }
}
"#;

const MINIMAL_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/minimal.json"
));
const SHAPES_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/shapes.json"
));
const ANIMATION_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/animation.json"
));
const STATE_MACHINE_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/state_machine.json"
));
const GRADIENTS_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/gradients.json"
));
const TEXT_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/text.json"
));
const LAYOUT_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/layout.json"
));
const DATA_BINDING_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/data_binding.json"
));
const BONES_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/bones.json"
));
const CONSTRAINTS_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/constraints.json"
));

pub fn get_template(name: &str) -> Result<serde_json::Value, AiError> {
    let json_str = match name {
        "bounce" => BOUNCE_TEMPLATE,
        "spinner" => SPINNER_TEMPLATE,
        "pulse" => PULSE_TEMPLATE,
        "fade" => FADE_TEMPLATE,
        "minimal" => MINIMAL_TEMPLATE,
        "shapes" => SHAPES_TEMPLATE,
        "animation" => ANIMATION_TEMPLATE,
        "state_machine" => STATE_MACHINE_TEMPLATE,
        "gradients" => GRADIENTS_TEMPLATE,
        "text" => TEXT_TEMPLATE,
        "layout" => LAYOUT_TEMPLATE,
        "data_binding" => DATA_BINDING_TEMPLATE,
        "bones" => BONES_TEMPLATE,
        "constraints" => CONSTRAINTS_TEMPLATE,
        _ => return Err(AiError::UnknownTemplate(name.to_string())),
    };
    serde_json::from_str(json_str).map_err(|e| AiError::InvalidResponse(e.to_string()))
}

pub fn list_templates() -> &'static [&'static str] {
    &[
        "bounce",
        "spinner",
        "pulse",
        "fade",
        "minimal",
        "shapes",
        "animation",
        "state_machine",
        "gradients",
        "text",
        "layout",
        "data_binding",
        "bones",
        "constraints",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::{SceneSpec, build_scene};

    #[test]
    fn test_all_templates_are_valid_scene_specs() {
        for name in list_templates() {
            let json = get_template(name).unwrap();
            let spec: SceneSpec = serde_json::from_value(json).unwrap();
            let scene = build_scene(&spec).unwrap();
            assert!(
                !scene.is_empty(),
                "template '{}' produced empty scene",
                name
            );
        }
    }

    #[test]
    fn test_unknown_template() {
        assert!(get_template("nonexistent").is_err());
    }

    #[test]
    fn test_template_catalog_has_minimum_size() {
        assert!(list_templates().len() >= 10);
    }
}
