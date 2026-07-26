window.__RIVE_FIXTURE_MANIFEST = {
  "generatedAt": "2026-07-26T18:11:07.297Z",
  "fixtures": [
    {
      "name": "animation",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "Red ball bounce one-shot. Use Replay to run it again.",
      "animations": [
        "bounce"
      ],
      "animationsByArtboard": {
        "Animated": [
          "bounce"
        ]
      },
      "replay": true
    },
    {
      "name": "artboard_preset",
      "category": "nonvisual",
      "scope": "nonvisual",
      "tags": [
        "schema"
      ],
      "expectation": "Preset sizing encoding fixture. No drawable objects, so canvas stays blank.",
      "replay": false
    },
    {
      "name": "asset_extensions",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "asset_extensions fixture loaded.",
      "replay": false
    },
    {
      "name": "assets",
      "category": "nonvisual",
      "scope": "nonvisual",
      "tags": [
        "schema"
      ],
      "expectation": "Asset declaration fixture (image/font/audio metadata). No visual output — assets have no embedded data.",
      "replay": false
    },
    {
      "name": "blend_animation",
      "category": "interactive",
      "scope": "visual",
      "tags": [
        "interactive",
        "animated"
      ],
      "expectation": "blend_animation fixture loaded.",
      "stateMachine": "BlendMachine",
      "animations": [
        "anim_a",
        "anim_b"
      ],
      "animationsByArtboard": {
        "BlendDemo": [
          "anim_a",
          "anim_b"
        ]
      },
      "replay": true
    },
    {
      "name": "bones",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static",
        "structure"
      ],
      "expectation": "Bone/rig scaffold preview; currently minimal visual output.",
      "replay": false
    },
    {
      "name": "button_states",
      "category": "interactive",
      "scope": "visual",
      "tags": [
        "interactive",
        "animated"
      ],
      "expectation": "Button with hover/press/loading states. Toggle isHovered/isPressed/isLoading to switch visual states.",
      "stateMachine": "ButtonStateMachine",
      "animations": [
        "idle",
        "hover_in",
        "press",
        "loading"
      ],
      "animationsByArtboard": {
        "ButtonArtboard": [
          "idle",
          "hover_in",
          "press",
          "loading"
        ]
      },
      "replay": true
    },
    {
      "name": "clipping_shape",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "clipping_shape fixture loaded.",
      "replay": false
    },
    {
      "name": "color_animation",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "Color transitions over time.",
      "animations": [
        "color_shift"
      ],
      "animationsByArtboard": {
        "ColorAnim": [
          "color_shift"
        ]
      },
      "replay": true
    },
    {
      "name": "comparison_clip_tests",
      "category": "interactive",
      "scope": "visual",
      "tags": [
        "interactive",
        "structure",
        "animated"
      ],
      "expectation": "comparison_clip_tests fixture loaded.",
      "artboards": [
        "Empty-Shape",
        "One-Clipping-Shape-Visible-One-Hidden",
        "Hidden-Path-Visible-Path"
      ],
      "stateMachine": "State Machine 1",
      "animations": [
        "Timeline 1"
      ],
      "animationsByArtboard": {
        "Empty-Shape": [
          "Timeline 1"
        ],
        "One-Clipping-Shape-Visible-One-Hidden": [
          "Timeline 1"
        ],
        "Hidden-Path-Visible-Path": [
          "Timeline 1"
        ]
      },
      "replay": true
    },
    {
      "name": "comparison_official_test",
      "category": "interactive",
      "scope": "visual",
      "tags": [
        "interactive",
        "animated"
      ],
      "expectation": "comparison_official_test fixture loaded.",
      "stateMachine": "State Machine 1",
      "animations": [
        "Fire",
        "Off",
        "On",
        "OffOn",
        "OnOff"
      ],
      "animationsByArtboard": {
        "New Artboard": [
          "Fire",
          "Off",
          "On",
          "OffOn",
          "OnOff"
        ]
      },
      "replay": true
    },
    {
      "name": "comparison_quantize_test",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated",
        "comparison"
      ],
      "expectation": "Recreation of official quantize_test.riv. Animated ellipse with state machine.",
      "stateMachine": "State Machine 1",
      "animations": [
        "Timeline 1"
      ],
      "animationsByArtboard": {
        "New Artboard": [
          "Timeline 1"
        ]
      },
      "replay": true,
      "hasReference": true,
      "referenceSource": "riv/reference/quantize_test.riv"
    },
    {
      "name": "comparison_trim",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static",
        "comparison"
      ],
      "expectation": "Recreation of official trim.riv using PointsPath and StraightVertex. Near-exact match.",
      "replay": false,
      "hasReference": true,
      "referenceSource": "riv/reference/trim.riv"
    },
    {
      "name": "constraints",
      "category": "nonvisual",
      "scope": "nonvisual",
      "tags": [
        "schema",
        "known-issue"
      ],
      "expectation": "Constraint types fixture. Known runtime load failure — constraint validation not yet supported.",
      "replay": false
    },
    {
      "name": "cubic_asymmetric",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "cubic_asymmetric fixture loaded.",
      "replay": false
    },
    {
      "name": "cubic_easing",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "Width animation with cubic easing curve.",
      "animations": [
        "ease_move"
      ],
      "animationsByArtboard": {
        "CubicEasing": [
          "ease_move"
        ]
      },
      "replay": true
    },
    {
      "name": "data_binding",
      "category": "nonvisual",
      "scope": "nonvisual",
      "tags": [
        "schema"
      ],
      "expectation": "Schema/encoding fixture: validated structurally, not a visual showcase.",
      "replay": false
    },
    {
      "name": "data_converters",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "data_converters fixture loaded.",
      "replay": false
    },
    {
      "name": "draw_rules",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "draw_rules fixture loaded.",
      "replay": false
    },
    {
      "name": "effects",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "effects fixture loaded.",
      "replay": false
    },
    {
      "name": "elastic_interpolator",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "elastic_interpolator fixture loaded.",
      "animations": [
        "bounce"
      ],
      "animationsByArtboard": {
        "ElasticInterpolator": [
          "bounce"
        ]
      },
      "replay": true
    },
    {
      "name": "empty_artboard",
      "category": "nonvisual",
      "scope": "nonvisual",
      "tags": [
        "schema",
        "structure"
      ],
      "expectation": "Intentionally empty structural fixture; blank canvas is expected.",
      "replay": false
    },
    {
      "name": "event_test",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "event_test fixture loaded.",
      "animations": [
        "timeline"
      ],
      "animationsByArtboard": {
        "EventTimeline": [
          "timeline"
        ]
      },
      "replay": true
    },
    {
      "name": "events_extended",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "events_extended fixture loaded.",
      "replay": false
    },
    {
      "name": "follow_path_constraint",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "follow_path_constraint fixture loaded.",
      "replay": false
    },
    {
      "name": "game_hud",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "game_hud fixture loaded.",
      "animations": [
        "drain_health"
      ],
      "animationsByArtboard": {
        "GameHUD": [
          "drain_health"
        ]
      },
      "replay": true
    },
    {
      "name": "gradients",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "Linear gradient is diagonal (red->magenta->blue). Oval uses radial yellow->green; replay re-runs orientation motion.",
      "animations": [
        "gradient_motion"
      ],
      "animationsByArtboard": {
        "Gradients": [
          "gradient_motion"
        ]
      },
      "replay": true
    },
    {
      "name": "graphics_misc",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "graphics_misc fixture loaded.",
      "replay": false
    },
    {
      "name": "icon_set",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static",
        "structure"
      ],
      "expectation": "icon_set fixture loaded.",
      "artboards": [
        "Home",
        "Settings",
        "Profile"
      ],
      "replay": false
    },
    {
      "name": "image_node",
      "category": "nonvisual",
      "scope": "nonvisual",
      "tags": [
        "schema"
      ],
      "expectation": "Image node fixture. No visual output — image asset has no embedded pixel data.",
      "replay": false
    },
    {
      "name": "joystick",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "joystick fixture loaded.",
      "replay": false
    },
    {
      "name": "keyframe_types",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "keyframe_types fixture loaded.",
      "animations": [
        "toggle_visibility",
        "change_text"
      ],
      "animationsByArtboard": {
        "KeyframeTypesDemo": [
          "toggle_visibility",
          "change_text"
        ]
      },
      "replay": true
    },
    {
      "name": "layout",
      "category": "nonvisual",
      "scope": "nonvisual",
      "tags": [
        "schema"
      ],
      "expectation": "Schema/encoding fixture: validated structurally, not a visual showcase.",
      "replay": false
    },
    {
      "name": "layout_extensions",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "layout_extensions fixture loaded.",
      "replay": false
    },
    {
      "name": "listener_test",
      "category": "interactive",
      "scope": "visual",
      "tags": [
        "interactive"
      ],
      "expectation": "listener_test fixture loaded.",
      "stateMachine": "InputMachine",
      "replay": false
    },
    {
      "name": "loader",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "Spinning loader animation — cyan arc rotates on dark track rings.",
      "animations": [
        "spin"
      ],
      "animationsByArtboard": {
        "Loader": [
          "spin"
        ]
      },
      "replay": true
    },
    {
      "name": "loop_animation",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "Continuous looping motion.",
      "animations": [
        "loop_spin"
      ],
      "animationsByArtboard": {
        "LoopAnim": [
          "loop_spin"
        ]
      },
      "replay": true
    },
    {
      "name": "mascot",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "mascot fixture loaded.",
      "animations": [
        "idle_bob"
      ],
      "animationsByArtboard": {
        "RobotMascot": [
          "idle_bob"
        ]
      },
      "replay": true
    },
    {
      "name": "mesh",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "mesh fixture loaded.",
      "replay": false
    },
    {
      "name": "minimal",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "Tiny baseline file; expected to look mostly empty.",
      "replay": false
    },
    {
      "name": "multi_artboard",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated",
        "interactive",
        "structure"
      ],
      "expectation": "Two artboards with per-artboard animations. Replay checks timeline behavior; Artboard switch checks scoping.",
      "artboards": [
        "Screen A",
        "Screen B"
      ],
      "animations": [
        "fade_in"
      ],
      "animationsByArtboard": {
        "Screen A": [
          "fade_in"
        ],
        "Screen B": [
          "slide_in"
        ]
      },
      "replay": true
    },
    {
      "name": "nested_artboard",
      "category": "interactive",
      "scope": "visual",
      "tags": [
        "interactive",
        "structure"
      ],
      "expectation": "Nested-artboard wiring check. No timeline animation; use Artboard switch to verify Main vs Component.",
      "artboards": [
        "Main",
        "Component"
      ],
      "replay": false
    },
    {
      "name": "nested_extensions",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated",
        "structure"
      ],
      "expectation": "nested_extensions fixture loaded.",
      "artboards": [
        "Component",
        "Main"
      ],
      "animations": [
        "intro"
      ],
      "animationsByArtboard": {
        "Component": [
          "intro"
        ],
        "Main": [
          "main_anim"
        ]
      },
      "replay": true
    },
    {
      "name": "nested_simple_animation",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated",
        "structure"
      ],
      "expectation": "nested_simple_animation fixture loaded.",
      "artboards": [
        "Inner",
        "Outer"
      ],
      "animations": [
        "pulse"
      ],
      "animationsByArtboard": {
        "Inner": [
          "pulse"
        ],
        "Outer": [
          "outer_idle"
        ]
      },
      "replay": true
    },
    {
      "name": "new_constraints",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "new_constraints fixture loaded.",
      "replay": false
    },
    {
      "name": "nslicer",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static",
        "structure"
      ],
      "expectation": "nslicer fixture loaded.",
      "artboards": [
        "ImageSlice",
        "VectorSlice"
      ],
      "replay": false
    },
    {
      "name": "path",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "Path-based drawing rendered as static art.",
      "replay": false
    },
    {
      "name": "points_path",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "points_path fixture loaded.",
      "replay": false
    },
    {
      "name": "polygon_star",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "polygon_star fixture loaded.",
      "replay": false
    },
    {
      "name": "scripting",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "scripting fixture loaded.",
      "replay": false
    },
    {
      "name": "shapes",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "Basic visible geometry (shape and fill sanity check).",
      "replay": false
    },
    {
      "name": "solo_test",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "solo_test fixture loaded.",
      "animations": [
        "toggle_active"
      ],
      "animationsByArtboard": {
        "SoloDemo": [
          "toggle_active"
        ]
      },
      "replay": true
    },
    {
      "name": "state_machine",
      "category": "interactive",
      "scope": "visual",
      "tags": [
        "interactive",
        "animated"
      ],
      "expectation": "Toggle isOn to switch magenta dot into active state (moves right + turns green).",
      "stateMachine": "Logic",
      "animations": [
        "idle",
        "active"
      ],
      "animationsByArtboard": {
        "Interactive": [
          "idle",
          "active"
        ]
      },
      "replay": true
    },
    {
      "name": "stroke_styles",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "Stroke thickness/style showcase around paths.",
      "replay": false
    },
    {
      "name": "text",
      "category": "nonvisual",
      "scope": "nonvisual",
      "tags": [
        "schema"
      ],
      "expectation": "Text schema fixture; renderer output may be limited in this harness.",
      "replay": false
    },
    {
      "name": "text_modifiers",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "text_modifiers fixture loaded.",
      "replay": false
    },
    {
      "name": "transition_comparators",
      "category": "interactive",
      "scope": "visual",
      "tags": [
        "interactive",
        "animated"
      ],
      "expectation": "transition_comparators fixture loaded.",
      "stateMachine": "Logic",
      "animations": [
        "idle",
        "active"
      ],
      "animationsByArtboard": {
        "ComparatorDemo": [
          "idle",
          "active"
        ]
      },
      "replay": true
    },
    {
      "name": "triangle",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "triangle fixture loaded.",
      "replay": false
    },
    {
      "name": "trim_path",
      "category": "animated",
      "scope": "visual",
      "tags": [
        "animated"
      ],
      "expectation": "Magenta arc sweeps around the ring over 2s; Replay restarts the sweep.",
      "animations": [
        "trim_sweep"
      ],
      "animationsByArtboard": {
        "TrimDemo": [
          "trim_sweep"
        ]
      },
      "replay": true
    },
    {
      "name": "view_model_instances",
      "category": "static",
      "scope": "visual",
      "tags": [
        "static"
      ],
      "expectation": "view_model_instances fixture loaded.",
      "replay": false
    },
    {
      "name": "official_test",
      "category": "interactive",
      "expectation": "Official fire_button.riv from rive-runtime repo. NOT generated by rive-cli — runtime compatibility test.",
      "scope": "visual",
      "tags": [
        "interactive",
        "animated",
        "official"
      ],
      "stateMachine": "State Machine 1",
      "hasReference": true,
      "referenceSource": "riv/reference/official_test.riv"
    }
  ]
};
