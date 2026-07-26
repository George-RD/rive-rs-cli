use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct TemplateInfo {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownTemplate;

impl std::fmt::Display for UnknownTemplate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("unknown scaffold template")
    }
}

impl std::error::Error for UnknownTemplate {}

const TEMPLATES: [TemplateInfo; 6] = [
    TemplateInfo {
        name: "shape",
        description: "Centered ellipse with a solid fill.",
    },
    TemplateInfo {
        name: "animated",
        description: "Looping cubic-eased translation with a named interpolator.",
    },
    TemplateInfo {
        name: "gradient",
        description: "Centered rectangle with a multi-stop linear gradient.",
    },
    TemplateInfo {
        name: "spinner",
        description: "Looping stroked ring with trim-path and rotation animation.",
    },
    TemplateInfo {
        name: "button",
        description: "Bool-driven state machine with entry and exit states.",
    },
    TemplateInfo {
        name: "multi",
        description: "Composed shapes with staggered looping animation.",
    },
];

const SHAPE: &str = r##"{
  "scene_format_version": 1,
  "artboard": {
    "name": "Shape",
    "width": 400,
    "height": 400,
    "children": [
      {
        "type": "shape",
        "name": "EllipseShape",
        "x": 200,
        "y": 200,
        "children": [
          {
            "type": "ellipse",
            "name": "EllipsePath",
            "width": 180,
            "height": 180,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "EllipseFill",
            "children": [
              {
                "type": "solid_color",
                "name": "EllipseColor",
                "color": "#4F46E5"
              }
            ]
          }
        ]
      }
    ],
    "animations": [],
    "state_machines": []
  }
}"##;

const ANIMATED: &str = r##"{
  "scene_format_version": 1,
  "artboard": {
    "name": "Animated",
    "width": 400,
    "height": 400,
    "children": [
      {
        "type": "shape",
        "name": "MovingEllipse",
        "x": 80,
        "y": 200,
        "children": [
          {
            "type": "ellipse",
            "name": "MovingEllipsePath",
            "width": 96,
            "height": 96,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "MovingEllipseFill",
            "children": [
              {
                "type": "solid_color",
                "name": "MovingEllipseColor",
                "color": "#06B6D4"
              }
            ]
          }
        ]
      }
    ],
    "animations": [
      {
        "name": "move",
        "fps": 60,
        "duration": 60,
        "loop_type": "loop",
        "interpolators": [
          {
            "name": "ease_in_out",
            "x1": 0.42,
            "y1": 0.0,
            "x2": 0.58,
            "y2": 1.0
          }
        ],
        "keyframes": [
          {
            "object": "MovingEllipse",
            "property": "x",
            "frames": [
              {
                "frame": 0,
                "value": 80.0,
                "interpolation": "cubic",
                "interpolator": "ease_in_out"
              },
              {
                "frame": 30,
                "value": 320.0,
                "interpolation": "cubic",
                "interpolator": "ease_in_out"
              },
              {
                "frame": 59,
                "value": 80.0
              }
            ]
          }
        ]
      }
    ],
    "state_machines": []
  }
}"##;

const GRADIENT: &str = r##"{
  "scene_format_version": 1,
  "artboard": {
    "name": "Gradient",
    "width": 400,
    "height": 400,
    "children": [
      {
        "type": "shape",
        "name": "GradientRectangle",
        "x": 200,
        "y": 200,
        "children": [
          {
            "type": "rectangle",
            "name": "GradientRectanglePath",
            "width": 280,
            "height": 180,
            "corner_radius": 24,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "GradientFill",
            "children": [
              {
                "type": "linear_gradient",
                "name": "SunsetGradient",
                "start_x": 0,
                "start_y": 0,
                "end_x": 280,
                "end_y": 180,
                "children": [
                  {
                    "type": "gradient_stop",
                    "color": "#F97316",
                    "position": 0.0
                  },
                  {
                    "type": "gradient_stop",
                    "color": "#EC4899",
                    "position": 0.5
                  },
                  {
                    "type": "gradient_stop",
                    "color": "#6366F1",
                    "position": 1.0
                  }
                ]
              }
            ]
          }
        ]
      }
    ],
    "animations": [],
    "state_machines": []
  }
}"##;

const SPINNER: &str = r##"{
  "scene_format_version": 1,
  "artboard": {
    "name": "Spinner",
    "width": 400,
    "height": 400,
    "children": [
      {
        "type": "shape",
        "name": "SpinnerShape",
        "x": 200,
        "y": 200,
        "children": [
          {
            "type": "ellipse",
            "name": "SpinnerPath",
            "width": 220,
            "height": 220,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "stroke",
            "name": "SpinnerStroke",
            "thickness": 14,
            "cap": "round",
            "children": [
              {
                "type": "solid_color",
                "name": "SpinnerColor",
                "color": "#22D3EE"
              },
              {
                "type": "trim_path",
                "name": "SpinnerTrim",
                "start": 0.0,
                "end": 0.7,
                "offset": 0.0,
                "mode": "sequential"
              }
            ]
          }
        ]
      }
    ],
    "animations": [
      {
        "name": "spin",
        "fps": 60,
        "duration": 60,
        "loop_type": "loop",
        "keyframes": [
          {
            "object": "SpinnerShape",
            "property": "rotation",
            "frames": [
              {
                "frame": 0,
                "value": 0.0
              },
              {
                "frame": 59,
                "value": 6.283185
              }
            ]
          },
          {
            "object": "SpinnerTrim",
            "property": "trim_offset",
            "frames": [
              {
                "frame": 0,
                "value": 0.0
              },
              {
                "frame": 59,
                "value": 1.0
              }
            ]
          }
        ]
      }
    ],
    "state_machines": []
  }
}"##;

const BUTTON: &str = r##"{
  "scene_format_version": 1,
  "artboard": {
    "name": "Button",
    "width": 400,
    "height": 240,
    "children": [
      {
        "type": "shape",
        "name": "ButtonBody",
        "x": 200,
        "y": 120,
        "children": [
          {
            "type": "rectangle",
            "name": "ButtonPath",
            "width": 280,
            "height": 104,
            "corner_radius": 24,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "ButtonFill",
            "children": [
              {
                "type": "solid_color",
                "name": "ButtonColor",
                "color": "#334155"
              }
            ]
          }
        ]
      },
      {
        "type": "shape",
        "name": "ButtonIndicator",
        "x": 200,
        "y": 120,
        "children": [
          {
            "type": "ellipse",
            "name": "ButtonIndicatorPath",
            "width": 40,
            "height": 40,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "ButtonIndicatorFill",
            "children": [
              {
                "type": "solid_color",
                "name": "ButtonIndicatorColor",
                "color": "#94A3B8"
              }
            ]
          }
        ]
      }
    ],
    "animations": [
      {
        "name": "off",
        "fps": 60,
        "duration": 60,
        "loop_type": "loop",
        "keyframes": [
          {
            "object": "ButtonColor",
            "property": "color",
            "frames": [
              {
                "frame": 0,
                "value": "#334155"
              },
              {
                "frame": 59,
                "value": "#334155"
              }
            ]
          },
          {
            "object": "ButtonIndicator",
            "property": "scale_x",
            "frames": [
              {
                "frame": 0,
                "value": 1.0
              },
              {
                "frame": 59,
                "value": 1.0
              }
            ]
          }
        ]
      },
      {
        "name": "on",
        "fps": 60,
        "duration": 60,
        "loop_type": "loop",
        "keyframes": [
          {
            "object": "ButtonColor",
            "property": "color",
            "frames": [
              {
                "frame": 0,
                "value": "#16A34A"
              },
              {
                "frame": 59,
                "value": "#16A34A"
              }
            ]
          },
          {
            "object": "ButtonIndicator",
            "property": "scale_x",
            "frames": [
              {
                "frame": 0,
                "value": 2.0
              },
              {
                "frame": 59,
                "value": 2.0
              }
            ]
          },
          {
            "object": "ButtonIndicator",
            "property": "scale_y",
            "frames": [
              {
                "frame": 0,
                "value": 2.0
              },
              {
                "frame": 59,
                "value": 2.0
              }
            ]
          }
        ]
      }
    ],
    "state_machines": [
      {
        "name": "ButtonMachine",
        "inputs": [
          {
            "type": "bool",
            "name": "enabled",
            "value": false
          }
        ],
        "layers": [
          {
            "states": [
              {
                "type": "entry"
              },
              {
                "type": "exit"
              },
              {
                "type": "animation",
                "animation": "off"
              },
              {
                "type": "animation",
                "animation": "on"
              }
            ],
            "transitions": [
              {
                "from": 0,
                "to": 2
              },
              {
                "from": 2,
                "to": 3,
                "conditions": [
                  {
                    "input": "enabled",
                    "op": "==",
                    "value": true
                  }
                ]
              },
              {
                "from": 3,
                "to": 2,
                "conditions": [
                  {
                    "input": "enabled",
                    "op": "==",
                    "value": false
                  }
                ]
              }
            ]
          }
        ]
      }
    ]
  }
}"##;

const MULTI: &str = r##"{
  "scene_format_version": 1,
  "artboard": {
    "name": "Multi",
    "width": 400,
    "height": 400,
    "children": [
      {
        "type": "shape",
        "name": "LeftCircle",
        "x": 110,
        "y": 200,
        "children": [
          {
            "type": "ellipse",
            "name": "LeftCirclePath",
            "width": 96,
            "height": 96,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "LeftCircleFill",
            "children": [
              {
                "type": "solid_color",
                "name": "LeftCircleColor",
                "color": "#F97316"
              }
            ]
          }
        ]
      },
      {
        "type": "shape",
        "name": "CenterSquare",
        "x": 200,
        "y": 200,
        "children": [
          {
            "type": "rectangle",
            "name": "CenterSquarePath",
            "width": 88,
            "height": 88,
            "corner_radius": 16,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "CenterSquareFill",
            "children": [
              {
                "type": "solid_color",
                "name": "CenterSquareColor",
                "color": "#8B5CF6"
              }
            ]
          }
        ]
      },
      {
        "type": "shape",
        "name": "RightCircle",
        "x": 290,
        "y": 200,
        "children": [
          {
            "type": "ellipse",
            "name": "RightCirclePath",
            "width": 96,
            "height": 96,
            "origin_x": 0.5,
            "origin_y": 0.5
          },
          {
            "type": "fill",
            "name": "RightCircleFill",
            "children": [
              {
                "type": "solid_color",
                "name": "RightCircleColor",
                "color": "#22C55E"
              }
            ]
          }
        ]
      }
    ],
    "animations": [
      {
        "name": "stagger",
        "fps": 60,
        "duration": 60,
        "loop_type": "loop",
        "keyframes": [
          {
            "object": "LeftCircle",
            "property": "y",
            "frames": [
              {
                "frame": 0,
                "value": 200.0
              },
              {
                "frame": 20,
                "value": 130.0
              },
              {
                "frame": 40,
                "value": 200.0
              },
              {
                "frame": 59,
                "value": 200.0
              }
            ]
          },
          {
            "object": "CenterSquare",
            "property": "y",
            "frames": [
              {
                "frame": 0,
                "value": 200.0
              },
              {
                "frame": 10,
                "value": 200.0
              },
              {
                "frame": 30,
                "value": 130.0
              },
              {
                "frame": 50,
                "value": 200.0
              },
              {
                "frame": 59,
                "value": 200.0
              }
            ]
          },
          {
            "object": "RightCircle",
            "property": "y",
            "frames": [
              {
                "frame": 0,
                "value": 200.0
              },
              {
                "frame": 20,
                "value": 200.0
              },
              {
                "frame": 40,
                "value": 130.0
              },
              {
                "frame": 59,
                "value": 200.0
              }
            ]
          }
        ]
      }
    ],
    "state_machines": []
  }
}"##;

pub fn templates() -> &'static [TemplateInfo] {
    &TEMPLATES
}

pub fn template_json(name: &str) -> Result<&'static str, UnknownTemplate> {
    match name {
        "shape" => Ok(SHAPE),
        "animated" => Ok(ANIMATED),
        "gradient" => Ok(GRADIENT),
        "spinner" => Ok(SPINNER),
        "button" => Ok(BUTTON),
        "multi" => Ok(MULTI),
        _ => Err(UnknownTemplate),
    }
}
