---
name: rive-animation
description: Author Rive (.riv) vector animations from JSON scene specs using the rive-cli tool. Use when creating, editing, validating, or visually verifying Rive animations, loaders, interactive buttons, animated icons, HUDs, or motion graphics.
---

# Rive Animation Authoring

You write a JSON **scene spec**, compile it to a `.riv` binary with `rive-cli`, then **render frames to PNG
and look at them**. Never claim an animation works until you have rendered it and inspected the frames.

## Start from a working scene

Do not invent a scene spec from a blank file. Scaffold one, then edit it:

```bash
rive-cli new --list          # shape, animated, gradient, spinner, button, multi
rive-cli new spinner -o scene.json
```

Every template compiles, validates and renders non-blank, and already encodes the conventions the runtime
requires. Start from the closest one and modify it.

## The loop

```bash
rive-cli generate scene.json -o out.riv     # compile
rive-cli validate out.riv                   # structural check of the binary
rive-cli render out.riv --frames 0,15,30,45 --preview -o frames/   # SEE IT
```

`render` drives headless Chromium with the real Rive runtime. If the runtime refuses your file, `render`
fails with the runtime's own message — this catches whole classes of defect that `validate` cannot.

**`validate` passing does not mean the file works.** It only checks binary structure. A file can validate
and still be rejected by the runtime. `render` is the real gate.

### Seeing your render without a vision model

`--preview` prints an ASCII coverage map per frame, and writes it to `preview.txt` and into `manifest.json`.
Use it always — it is how you catch composition mistakes you cannot otherwise see:

- A solid block of `#` filling the grid, with a dominant color near 100%, means **one opaque shape is
  covering your whole scene**. This is the single most common showcase failure: check draw order — in Rive
  the FIRST declared sibling paints on top (see Composition and draw order below).
- An almost empty grid means your content is off-screen or unfilled.
- The reported bounding box tells you whether the composition is centred or clipped at an edge.

### Reading render output

`render` prints a table and writes `manifest.json` plus one PNG per frame:

```
artboard 'Loader' | animation 'spin' | 512x512 @2x | 60 fps
  frame   seconds  file                 colors
      0     0.000  frame_00000.png         902
     30     0.500  frame_00030.png        2613
```

- `colors` is the count of distinct pixel colors. Use it as a cheap automated check.
- A frame marked `BLANK` is a single flat color — your artboard is empty, your shapes are off-screen,
  or nothing is filled. `render` warns when every frame is blank.
- **If every frame has an identical `colors` value, suspect nothing is animating.** Confirm by comparing
  file hashes: identical PNGs across different frames means no motion.
- Rendering is deterministic: the same file and frame always produce byte-identical PNGs.

Useful flags:

```bash
--frames 0,15,30        # explicit list
--frames 0..120:10      # range, end-exclusive, step 10
--width 800 --height 600 --scale 2      # logical size; output is width*scale x height*scale
--background '#101014'  # default is transparent, which is hard to judge
--contact-sheet         # also write one filmstrip PNG of all frames
--animation NAME        # pick a linear animation (default: the first one)
--state-machine NAME    # drive a state machine instead
--input isOn=true       # set a state machine input; repeatable
--json                  # machine-readable manifest
```

## Discovering the schema

Do not guess type names or property names. Ask the tool:

```bash
rive-cli types                 # every object type, grouped by category
rive-cli types --category paint
rive-cli describe ellipse      # fields, valid parents, animatable properties, example
rive-cli schema                # the complete JSON Schema
```

`rive-cli describe <type>` is authoritative — its animatable-property list is generated from the same code
that compiles keyframes, so it cannot drift from reality.

## Scene structure

```json
{
  "scene_format_version": 1,
  "artboard": {
    "name": "Main",
    "width": 500,
    "height": 500,
    "children": [ ... ],
    "animations": [ ... ],
    "state_machines": [ ... ]
  }
}
```

Object nesting:

```text
artboard
  └── shape            (this is what carries x / y / rotation / scale)
        ├── ellipse | rectangle | triangle | polygon | star | points_path   (the geometry)
        ├── fill
        │     └── solid_color | linear_gradient | radial_gradient
        └── stroke     (thickness, cap, join)
              ├── solid_color | linear_gradient | radial_gradient
              └── trim_path
```

Key rule: **geometry does not move — the parent `shape` moves.** An `ellipse` has `width`, `height`,
`origin_x`, `origin_y`. Its position comes from the enclosing `shape`'s `x`/`y`. To move a circle, keyframe
`x` on the shape, not on the ellipse.

Set `origin_x: 0.5, origin_y: 0.5` so a shape rotates and scales about its own centre.

## Animation

```json
{
  "name": "spin",
  "fps": 60,
  "duration": 120,
  "loop_type": "loop",
  "interpolators": [
    { "name": "ease_in_out", "x1": 0.42, "y1": 0.0, "x2": 0.58, "y2": 1.0 }
  ],
  "keyframes": [
    {
      "object": "Ring",
      "property": "rotation",
      "frames": [
        { "frame": 0, "value": 0.0, "interpolation": "cubic", "interpolator": "ease_in_out" },
        { "frame": 119, "value": 6.28318 }
      ]
    }
  ]
}
```

- `duration` is in frames. At `fps: 60`, `duration: 120` is two seconds.
- `loop_type`: `"loop"`, `"oneshot"`, or `"pingpong"` (no underscores).
- `rotation` is in **radians**. A full turn is `6.28318`.
- Keyframe `value` is a number, or a `"#RRGGBB"` string when the property is `color`.
- `interpolation` is `"linear"`, `"cubic"`, or `"hold"`. `"cubic"` requires a named `interpolator`.
  Without an interpolator everything is linear, which looks mechanical — use easing for quality work.

### Animatable properties

**Animatable properties are per-type.** `rive-cli describe <type>` reports exactly the set that `generate`
will accept for that type — the two cannot disagree, so trust it over any list including this one.

| Target type | Animatable |
|---|---|
| `shape`, `node` | `x`, `y`, `rotation`, `scale_x`, `scale_y`, `opacity` |
| `ellipse`, `rectangle`, `triangle`, `polygon`, `star` | the transforms above plus `width`, `height` |
| `solid_color`, `gradient_stop` | `color` |
| `trim_path` | `trim_start`, `trim_end`, `trim_offset` |
| `fill`, `stroke` | `is_visible` |
| `text_style` | `font_size`, `line_height`, `letter_spacing` |

The traps this table exists to prevent:

- **`width`/`height` live on the geometry, not the shape.** Keyframing `width` on a `shape` is rejected;
  target the child `ellipse`/`rectangle`, or animate `scale_x`/`scale_y` on the shape instead.
- **A `fill` or `stroke` has no `opacity`.** To show and hide a paint, animate `is_visible` (0 or 1), or
  animate `opacity` on the enclosing `shape`. This one silently produced no motion before it was rejected.
- **`stroke.thickness` is not animatable.** Use a second stroke toggled with `is_visible`, or scale the shape.
- Trim animation names are `trim_start`/`trim_end`/`trim_offset`. The *static* field names on the object are
  `start`/`end`/`offset` — they differ deliberately.

If `generate` rejects a pairing it names the object, its type, and lists exactly what that type does allow.

## State machines

A state machine layer **must contain both an `entry` state and an `exit` state**. If either is missing the
Rive runtime rejects the entire file — and `validate` will still call it valid. `generate` now refuses this,
but remember the rule when hand-writing layers.

```json
"state_machines": [
  {
    "name": "Logic",
    "inputs": [
      { "type": "bool", "name": "isOn", "value": false },
      { "type": "trigger", "name": "press" },
      { "type": "number", "name": "level", "value": 0 }
    ],
    "layers": [
      {
        "states": [
          { "type": "entry" },
          { "type": "animation", "animation": "idle" },
          { "type": "animation", "animation": "active" },
          { "type": "exit" }
        ],
        "transitions": [
          { "from": 0, "to": 1 },
          { "from": 1, "to": 2, "duration": 150,
            "conditions": [ { "input": "isOn", "value": true } ] },
          { "from": 2, "to": 1,
            "conditions": [ { "input": "isOn", "value": false } ] }
        ]
      }
    ]
  }
]
```

`from`/`to` are indices into that layer's `states` array. `duration` is the blend time in milliseconds.

Verify interactive behaviour by driving inputs and rendering each state. `--input` is repeatable and takes
three value forms: `true`/`false` for a bool, a number for a number input, and the literal word `trigger`
to fire a trigger (NOT `true` — a trigger is fired, not set):

```bash
rive-cli render out.riv --state-machine Logic --frames 0,20,40 -o idle/
rive-cli render out.riv --state-machine Logic --input isOn=true --frames 0,20,40 -o active/
rive-cli render out.riv --state-machine Logic --input press=trigger --frames 0,20,40 -o pressed/
rive-cli render out.riv --state-machine Logic --input level=0.75 --input isOn=true --frames 0,20 -o both/
```

The directories must differ. If they are identical your conditions are not firing — check that the input
names match exactly and that the transition `from`/`to` indices point at the states you meant.

## Colors and gradients

Colors are `"#RRGGBB"` or `"#RRGGBBAA"` (alpha last).

```json
{
  "type": "linear_gradient",
  "name": "Sky",
  "start_x": 0, "start_y": 0, "end_x": 0, "end_y": 400,
  "children": [
    { "type": "gradient_stop", "color": "#1E3A8A", "position": 0.0 },
    { "type": "gradient_stop", "color": "#38BDF8", "position": 1.0 }
  ]
}
```

`position` runs 0..1. `radial_gradient` uses the same shape.

## What works well, and what to avoid

Strong, proven ground for showcase work:

- Vector geometry: ellipse, rectangle (with `corner_radius`), triangle, polygon, star, `points_path`
- Solid fills, linear and radial gradients with multiple stops
- Strokes with `cap` (`butt`/`round`/`square`), `join` (`miter`/`round`/`bevel`), and `trim_path`
- Transform animation with cubic easing, looping, ping-pong
- State machines with bool/number/trigger inputs
- Clipping, multiple artboards, nested artboards

Avoid in showcase work — these compile but render **blank**:

- **Text.** Requires an embedded font, which this tool cannot embed. `text` objects render as nothing.
  Draw letterforms as vector paths instead, or omit text.
- **Images / raster assets.** No asset bytes can be embedded, so images render as nothing.
- **Scripting** objects and extended **transition comparators** — known runtime gaps.
- **Bones / skinning** — they serialise, but no fixture demonstrates actual mesh deformation.
  Build characters from separate animated shapes instead.

## Composition and draw order

**Rive draws the FIRST declared sibling on top.** This is the reverse of HTML, SVG and most design tools,
and it is the single most expensive mistake to make here. Verified: two overlapping squares, red declared
first and blue second, render with red covering blue at the overlap; swapping the declaration order swaps
which one is visible.

So a background plate must be the **LAST** child of the artboard, not the first. Put your foreground
artwork first and work backwards.

If a large opaque shape is declared before your content, it covers everything: the file compiles, validates,
renders non-blank and even animates, but every frame looks identical because the moving parts are hidden
behind it. Several showcase attempts lost a full iteration to exactly this.

`--preview` catches it immediately: a solid `#` grid with a dominant color above ~95% means one shape owns
the canvas. Check the bounding box too — content filling the whole frame when you expected a centred motif
is the same bug.

Simplest way to avoid the whole problem: do not paint a full-artboard backdrop at all. Pass `--background`
to `render` instead, and keep the scene to actual artwork.

## Common failures

| Symptom | Cause |
|---|---|
| every frame `BLANK` | no fill, shape outside the artboard, or an all-transparent color |
| `render` fails "file failed to load" | runtime rejected it — usually a state machine layer missing entry/exit |
| shape does not move | you keyframed `x` on the geometry instead of the parent `shape` |
| rotation barely moves | `rotation` is radians, not degrees |
| `unknown property referenced in keyframes` | run `rive-cli describe <type>` for the real names |
| trim path animation ignored | use `trim_start`/`trim_end`, not `start`/`end` |
| identical frames, scene looks solid | an opaque shape declared **earlier** covers everything — first sibling paints on top; check `--preview` |
| identical frames, `fill`/`stroke` keyframed | a paint has no `opacity`; animate `is_visible` instead |
| identical frames otherwise | nothing is actually animating; check keyframe values really differ |
| rotating a circle shows nothing | a circle is rotationally symmetric; rotate an asymmetric shape or a group |

## Quality bar

Before you call an animation done:

1. `rive-cli validate` exits 0.
2. `rive-cli render` succeeds with `--background` set, and no frame is `BLANK`.
3. Frames across the timeline are visibly different from each other.
4. You have **looked at** the PNGs (or the `--contact-sheet`) and they show what you intended.
5. Motion uses easing rather than raw linear interpolation, and shapes are composed deliberately
   — deliberate palette, balanced composition, nothing clipped at the artboard edge unintentionally.
