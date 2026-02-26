---
name: rive-anti-patterns
description: Guardrails skill listing all known pitfalls when generating Rive scene JSON. WRONG/RIGHT format. Always load alongside creation skills.
---

# Rive Anti-Patterns and Guardrails

This skill documents common pitfalls when creating Rive scene JSON. Always verify against this list before generating scene files.

---

## A. scene_format_version is REQUIRED

**WRONG:**
```json
{
  "artboard": { "name": "Main" }
}
```

**RIGHT:**
```json
{
  "scene_format_version": 1,
  "artboard": { "name": "Main" }
}
```

Always include `scene_format_version: 1` at the root level.

---

## B. All objects MUST have unique names

**WRONG:**
```json
{
  "children": [
    { "type": "shape", "name": "Shape" },
    { "type": "shape", "name": "Shape" }
  ]
}
```

**RIGHT:**
```json
{
  "children": [
    { "type": "shape", "name": "LeftShape" },
    { "type": "shape", "name": "RightShape" }
  ]
}
```

Exception: `gradient_stop` name is optional.

---

## C. TrimPath parent must be Stroke or Fill, NOT Shape

**WRONG:**
```json
{
  "type": "shape",
  "name": "MyShape",
  "children": [
    { "type": "ellipse", "name": "Body" },
    { "type": "trim_path", "name": "Trim", "mode": "sequential" }
  ]
}
```

**RIGHT:**
```json
{
  "type": "shape",
  "name": "MyShape",
  "children": [
    { "type": "ellipse", "name": "Body" },
    {
      "type": "stroke",
      "name": "Outline",
      "children": [
        { "type": "solid_color", "name": "OutlineColor", "color": "#000000" },
        { "type": "trim_path", "name": "Trim", "mode": "sequential" }
      ]
    }
  ]
}
```

Why: EffectsContainer::from() in the runtime only accepts ShapePaint types.

---

## D. TrimPath mode must be 1 or 2, NEVER 0

**WRONG:**
```json
{ "type": "trim_path", "mode": 0 }
{ "type": "trim_path", "mode": "none" }
```

**RIGHT:**
```json
{ "type": "trim_path", "mode": "sequential" }
{ "type": "trim_path", "mode": "synchronized" }
```

Why: Mode 0 causes InvalidObject in the Rive runtime.

---

## E. Shape hierarchy must follow Shape → Path + Paint pattern

**WRONG:**
```json
{
  "type": "fill",
  "children": [
    { "type": "ellipse", "name": "Body" }
  ]
}
```

**RIGHT:**
```json
{
  "type": "shape",
  "name": "BodyShape",
  "children": [
    { "type": "ellipse", "name": "Body" },
    {
      "type": "fill",
      "name": "BodyFill",
      "children": [
        { "type": "solid_color", "name": "BodyColor", "color": "#FF0000" }
      ]
    }
  ]
}
```

The correct tree: shape > [ellipse, fill > [solid_color]]

---

## F. Color format must be hex string with # prefix

**WRONG:**
```json
{ "color": 0xFF0000 }
{ "color": "red" }
{ "color": "rgb(255,0,0)" }
{ "color": "FF0000" }
```

**RIGHT:**
```json
{ "color": "#FF0000" }
{ "color": "#80FF0000" }
```

Use 6-char RGB or 8-char ARGB hex strings with # prefix.

---

## G. Width and height must be positive, non-zero for artboards

**WRONG:**
```json
{ "width": 0, "height": 0 }
{ "width": -100, "height": 200 }
```

**RIGHT:**
```json
{ "width": 500, "height": 500 }
{ "width": 1920, "height": 1080 }
```

---

## H. Use EITHER 'artboard' OR 'artboards', not both

**WRONG:**
```json
{
  "artboard": { "name": "Main" },
  "artboards": [{ "name": "Secondary" }]
}
```

**RIGHT (single artboard):**
```json
{
  "artboard": { "name": "Main" }
}
```

**RIGHT (multiple artboards):**
```json
{
  "artboards": [
    { "name": "Main" },
    { "name": "Secondary" }
  ]
}
```

---

## I. Animation keyframe 'object' must reference an existing child name

**WRONG:**
```json
{
  "animations": [{
    "name": "Bounce",
    "keyframe_groups": [{
      "object": "NonexistentObject",
      "property": "x",
      "keyframes": [{ "frame": 0, "value": 0 }]
    }]
  }]
}
```

**RIGHT:**
```json
{
  "children": [{ "type": "ellipse", "name": "Ball" }],
  "animations": [{
    "name": "Bounce",
    "keyframe_groups": [{
      "object": "Ball",
      "property": "x",
      "keyframes": [{ "frame": 0, "value": 0 }]
    }]
  }]
}
```

The `object` field must match a child's `name` exactly.

---

## J. State machine animation references must match animation names

**WRONG:**
```json
{
  "animations": [{ "name": "Bounce" }],
  "state_machines": [{
    "layers": [{
      "states": [{ "type": "animation", "animation": "bounce" }]
    }]
  }]
}
```

**RIGHT:**
```json
{
  "animations": [{ "name": "Bounce" }],
  "state_machines": [{
    "layers": [{
      "states": [{ "type": "animation", "animation": "Bounce" }]
    }]
  }]
}
```

Names are case-sensitive and must match exactly.

---

## K. Transition state indices must be valid

**WRONG:**
```json
{
  "states": [
    { "type": "entry" },
    { "type": "animation", "animation": "Idle" }
  ],
  "transitions": [{ "from": 0, "to": 5 }]
}
```

**RIGHT:**
```json
{
  "states": [
    { "type": "entry" },
    { "type": "animation", "animation": "Idle" }
  ],
  "transitions": [{ "from": 0, "to": 1 }]
}
```

Indices are 0-based into the states array.

---

## L. State machine layers MUST have at least entry state

**WRONG:**
```json
{
  "layers": [{
    "states": [{ "type": "animation", "animation": "Idle" }]
  }]
}
```

**RIGHT:**
```json
{
  "layers": [{
    "states": [
      { "type": "entry" },
      { "type": "animation", "animation": "Idle" }
    ]
  }]
}
```

Every layer needs an entry state as the starting point.

---

## M. Multi-artboard: nested_artboard source must exist

**WRONG:**
```json
{
  "artboards": [
    { "name": "Main" },
    {
      "name": "Scene",
      "children": [{ "type": "nested_artboard", "source_artboard": "Nonexistent" }]
    }
  ]
}
```

**RIGHT:**
```json
{
  "artboards": [
    { "name": "Main" },
    {
      "name": "Scene",
      "children": [{ "type": "nested_artboard", "source_artboard": "Main" }]
    }
  ]
}
```

source_artboard must match a sibling artboard's name.

---

## N. Gradient stops need position values 0.0 to 1.0

**WRONG:**
```json
{
  "type": "linear_gradient",
  "stops": [
    { "position": 0, "color": "#FF0000" },
    { "position": 50, "color": "#0000FF" }
  ]
}
```

**RIGHT:**
```json
{
  "type": "linear_gradient",
  "stops": [
    { "position": 0.0, "color": "#FF0000" },
    { "position": 1.0, "color": "#0000FF" }
  ]
}
```

Use normalized values (0.0 to 1.0), not percentages.

---

## O. Interpolation requires both fields

**WRONG:**
```json
{
  "keyframes": [
    { "frame": 0, "value": 0 },
    { "frame": 30, "value": 100, "interpolation": "cubic" }
  ]
}
```

**RIGHT:**
```json
{
  "interpolators": [{ "name": "EaseInOut", "type": "cubic" }],
  "keyframes": [
    { "frame": 0, "value": 0 },
    { "frame": 30, "value": 100, "interpolation": "cubic", "interpolator": "EaseInOut" }
  ]
}
```

Both `interpolation` and `interpolator` fields are required, and the interpolator must be defined.

---

## P. Loop type values are strings

**WRONG:**
```json
{ "name": "Walk", "loop_type": 1 }
{ "name": "Walk", "loop_type": true }
```

**RIGHT:**
```json
{ "name": "Walk", "loop_type": "loop" }
{ "name": "Walk", "loop_type": "oneshot" }
{ "name": "Walk", "loop_type": "pingpong" }
```

---

## Q. Input conditions — triggers don't take op/value

**WRONG:**
```json
{
  "transitions": [{
    "from": 0,
    "to": 1,
    "conditions": [{ "input": "onClick", "op": "==", "value": true }]
  }]
}
```

**RIGHT:**
```json
{
  "transitions": [{
    "from": 0,
    "to": 1,
    "conditions": [{ "input": "onClick" }]
  }]
}
```

Triggers fire on invocation, no comparison needed.

---

## R. Children arrays should be omitted or non-null

**WRONG:**
```json
{ "type": "shape", "name": "Empty", "children": null }
```

**RIGHT:**
```json
{ "type": "shape", "name": "Empty" }
{ "type": "shape", "name": "Empty", "children": [] }
```

---

## S. State machine states MUST include ExitState and follow strict ordering

**WRONG (missing exit, any after animations):**
```json
{
  "states": [
    {"type": "entry"},
    {"type": "animation", "animation": "Idle"},
    {"type": "animation", "animation": "Active"},
    {"type": "any"}
  ]
}
```

**RIGHT (exit present, ordering: entry → any → exit → animations):**
```json
{
  "states": [
    {"type": "entry"},
    {"type": "any"},
    {"type": "exit"},
    {"type": "animation", "animation": "Idle"},
    {"type": "animation", "animation": "Active"}
  ]
}
```

The Rive runtime requires strict state ordering within a layer:
1. `entry` — ALWAYS first (index 0)
2. `any` — ALWAYS second (index 1) when present
3. `exit` — ALWAYS third (index 2) when present
4. Animation states — ALWAYS after system states

Missing ExitState or wrong ordering causes `RuntimeError: table index is out of bounds` in the WASM runtime. The file will pass structural validation but fail at load time.

---

## Quick Checklist

Before generating any Rive scene JSON, verify:

- [ ] scene_format_version: 1 present
- [ ] All objects have unique names
- [ ] Shape hierarchy: shape > [path + paint > [color]]
- [ ] Colors are hex strings with #
- [ ] Positive non-zero artboard dimensions
- [ ] Animation object names match children names
- [ ] State machine has entry state, exit state, and correct ordering (entry → any → exit → animations)
- [ ] Transition indices valid
- [ ] TrimPath under Stroke/Fill, mode 1 or 2
- [ ] Interpolation has both interpolation + interpolator fields
- [ ] State machine includes ExitState even if unused
