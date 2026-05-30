window.__RIVE_FIXTURE_MANIFEST = {
  "generatedAt": "2026-02-27T06:30:29.256Z",
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
      "referenceSource": "riv/reference/quantize_test.riv",
      "gapTypes": []
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
      "referenceSource": "riv/reference/trim.riv",
      "gapTypes": []
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
