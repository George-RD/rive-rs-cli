---
name: rive-animation
description: Animation and state machine reference for Rive scene JSON. Covers keyframes, interpolation, loop types, state machine inputs/layers/transitions/conditions.
---

# Rive Animation and State Machines

Reference for writing animation and state machine specs in Rive scene JSON. Use with rive-scene-schema skill.

---

## A. AnimationSpec

```json
{
  "name": "string",        // REQUIRED, unique within artboard
  "fps": 60,              // REQUIRED, frames per second (typically 60)
  "duration": 120,         // REQUIRED, total frames (e.g., 120 = 2 seconds at 60fps)
  "speed": 1.0,           // Optional, default 1.0
  "loop_type": "oneshot",  // Optional: "oneshot" (default), "loop", "pingpong"
  "interpolators": [...],  // Optional, cubic easing definitions
  "keyframes": [...]       // REQUIRED, animation channels
}
```

---

## B. KeyframeGroupSpec

Each entry animates one property on one object.

```json
{
  "object": "ObjectName",   // Name of the object to animate (must match a child name)
  "property": "x",          // Property to animate
  "frames": [
    {"frame": 0, "value": 0.0},
    {"frame": 60, "value": 200.0, "interpolation": "cubic", "interpolator": "EaseInOut"}
  ]
}
```

---

## C. Animatable Properties

Exact property name strings accepted by the animation system.

**Position:**
- `"x"` — horizontal position in pixels
- `"y"` — vertical position in pixels

**Transform:**
- `"rotation"` — angle in degrees (0-360)
- `"scale_x"` — horizontal scale factor
- `"scale_y"` — vertical scale factor

**Opacity:**
- `"opacity"` — alpha value (0.0 to 1.0)

**Color:**
- `"color"` — hex string `"#RRGGBB"` or `"#AARRGGBB"`

**Size:**
- `"width"` — width in pixels
- `"height"` — height in pixels

---

## D. Interpolation

**Linear (default):**
No `interpolation` field needed. Values change evenly between keyframes.

**Cubic easing:**
Set `"interpolation": "cubic"` and reference an interpolator by name.

**InterpolatorSpec:**
```json
{
  "name": "EaseInOut",
  "x1": 0.42,
  "y1": 0.0,
  "x2": 0.58,
  "y2": 1.0
}
```

x1/y1/x2/y2 are cubic bezier control points (default: 0.42, 0.0, 0.58, 1.0).

**Common curves:**
| Name | x1 | y1 | x2 | y2 | Effect |
|------|----|----|----|----|--------|
| ease-in | 0.42 | 0.0 | 1.0 | 1.0 | starts slow, ends fast |
| ease-out | 0.0 | 0.0 | 0.58 | 1.0 | starts fast, ends slow |
| ease-in-out | 0.42 | 0.0 | 0.58 | 1.0 | starts and ends slow |

---

## E. Loop Types

| Value | Name | Behavior |
|-------|------|----------|
| 0 | `"oneshot"` | plays once and stops (default) |
| 1 | `"loop"` | repeats from start |
| 2 | `"pingpong"` | plays forward then backward |

---

## F. StateMachineSpec

```json
{
  "name": "string",        // REQUIRED
  "inputs": [...],         // Optional, defines external controls
  "layers": [...]          // REQUIRED, at least one layer
}
```

---

## G. Inputs

Discriminated by `"type"` field.

**Number input:**
```json
{"type": "number", "name": "progress", "value": 0.0}
```

**Boolean input:**
```json
{"type": "bool", "name": "isOn", "value": false}
```

**Trigger input:**
```json
{"type": "trigger", "name": "onClick"}
```

---

## H. LayerSpec

```json
{
  "states": [              // REQUIRED, defines available states — ORDER MATTERS
    {"type": "entry"},     // REQUIRED — starting point, MUST be index 0
    {"type": "any"},       // REQUIRED — wildcard source, MUST be index 1
    {"type": "exit"},      // REQUIRED — ending point, MUST be index 2
    {"type": "animation", "animation": "AnimationName"}  // index 3+, references animation by name
  ],
  "transitions": [         // Optional, defines state changes
    {
      "from": 0,           // State index (0-based into states array)
      "to": 3,             // Target state index
      "duration": 0,       // Optional, transition duration in milliseconds
      "conditions": [...]  // Optional, when to trigger
    }
  ]
}
```

**CRITICAL: State ordering is enforced by the Rive runtime:**
1. `entry` — ALWAYS index 0
2. `any` — ALWAYS index 1 (include even if no any-state transitions)
3. `exit` — ALWAYS index 2 (include even if no exit transitions)
4. Animation states — index 3 onwards

Missing `any`/`exit` or wrong ordering causes `RuntimeError: table index is out of bounds`.

---

## I. ConditionSpec

**Boolean condition:**
```json
{"input": "isOn", "op": "==", "value": true}
```

**Number condition:**
```json
{"input": "progress", "op": ">", "value": 50.0}
```

**Trigger condition:**
```json
{"input": "onClick"}
```

**Operators:** `"=="`, `"!="`, `"<"`, `">"`, `"<="`, `">="`

---

## J. State Machine Design Patterns

### Pattern 1 — Toggle (bool input)

```json
{
  "states": [
    {"type": "entry"},
    {"type": "any"},
    {"type": "exit"},
    {"type": "animation", "animation": "idle_anim"},
    {"type": "animation", "animation": "active_anim"}
  ],
  "transitions": [
    {"from": 0, "to": 3},
    {"from": 3, "to": 4, "conditions": [{"input": "isOn", "op": "==", "value": true}]},
    {"from": 4, "to": 3, "conditions": [{"input": "isOn", "op": "==", "value": false}]}
  ]
}
```

### Pattern 2 — Progress (number input)

```json
{
  "states": [
    {"type": "entry"},
    {"type": "any"},
    {"type": "exit"},
    {"type": "animation", "animation": "phase1_anim"},
    {"type": "animation", "animation": "phase2_anim"},
    {"type": "animation", "animation": "phase3_anim"}
  ],
  "transitions": [
    {"from": 0, "to": 3},
    {"from": 3, "to": 4, "conditions": [{"input": "progress", "op": ">=", "value": 33.0}]},
    {"from": 4, "to": 5, "conditions": [{"input": "progress", "op": ">=", "value": 66.0}]}
  ]
}
```

### Pattern 3 — Trigger (fire-once)

```json
{
  "states": [
    {"type": "entry"},
    {"type": "any"},
    {"type": "exit"},
    {"type": "animation", "animation": "idle_anim"},
    {"type": "animation", "animation": "reaction_anim"}
  ],
  "transitions": [
    {"from": 0, "to": 3},
    {"from": 3, "to": 4, "conditions": [{"input": "onClick"}]},
    {"from": 4, "to": 3}
  ]
}
```

---

## K. Complete Animated Example

Bouncing ball with loop animation:

```json
{
  "scene_format_version": 1,
  "artboard": {
    "name": "Main",
    "width": 500,
    "height": 500,
    "children": [
      {
        "type": "shape",
        "name": "Ball",
        "x": 250,
        "y": 250,
        "children": [
          {
            "type": "ellipse",
            "name": "BallPath",
            "width": 80,
            "height": 80
          },
          {
            "type": "fill",
            "name": "BallFill",
            "children": [
              {
                "type": "solid_color",
                "name": "BallColor",
                "color": "#FF4444"
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
        "duration": 60,
        "loop_type": "loop",
        "keyframes": [
          {
            "object": "Ball",
            "property": "y",
            "frames": [
              {"frame": 0, "value": 150.0},
              {"frame": 30, "value": 350.0},
              {"frame": 59, "value": 150.0}
            ]
          }
        ]
      }
    ]
  }
}
```

---

## L. Complete Interactive Example

Toggle with state machine switching between two animation states:

```json
{
  "scene_format_version": 1,
  "artboard": {
    "name": "Main",
    "width": 500,
    "height": 500,
    "children": [
      {
        "type": "shape",
        "name": "Box",
        "x": 250,
        "y": 250,
        "width": 100,
        "height": 100,
        "children": [
          {
            "type": "rectangle",
            "name": "BoxPath",
            "width": 100,
            "height": 100
          },
          {
            "type": "fill",
            "name": "BoxFill",
            "children": [
              {
                "type": "solid_color",
                "name": "BoxColor",
                "color": "#4444FF"
              }
            ]
          }
        ]
      }
    ],
    "animations": [
      {
        "name": "idle",
        "fps": 60,
        "duration": 60,
        "loop_type": "loop",
        "keyframes": [
          {
            "object": "Box",
            "property": "scale_x",
            "frames": [
              {"frame": 0, "value": 1.0},
              {"frame": 30, "value": 1.05},
              {"frame": 59, "value": 1.0}
            ]
          },
          {
            "object": "Box",
            "property": "scale_y",
            "frames": [
              {"frame": 0, "value": 1.0},
              {"frame": 30, "value": 1.05},
              {"frame": 59, "value": 1.0}
            ]
          }
        ]
      },
      {
        "name": "active",
        "fps": 60,
        "duration": 60,
        "loop_type": "loop",
        "keyframes": [
          {
            "object": "Box",
            "property": "rotation",
            "frames": [
              {"frame": 0, "value": 0.0},
              {"frame": 60, "value": 360.0}
            ]
          },
          {
            "object": "BoxColor",
            "property": "color",
            "frames": [
              {"frame": 0, "value": "#44FF44"},
              {"frame": 30, "value": "#FF4444"},
              {"frame": 59, "value": "#44FF44"}
            ]
          }
        ]
      }
    ],
    "state_machines": [
      {
        "name": "ToggleMachine",
        "inputs": [
          {"type": "bool", "name": "isActive", "value": false}
        ],
        "layers": [
          {
            "states": [
              {"type": "entry"},
              {"type": "any"},
              {"type": "exit"},
              {"type": "animation", "animation": "idle"},
              {"type": "animation", "animation": "active"}
            ],
            "transitions": [
              {"from": 0, "to": 3},
              {
                "from": 1,
                "to": 4,
                "conditions": [{"input": "isActive", "op": "==", "value": true}]
              },
              {
                "from": 2,
                "to": 3,
                "conditions": [{"input": "isActive", "op": "==", "value": false}]
              }
            ]
          }
        ]
      }
    ]
  }
}
```
