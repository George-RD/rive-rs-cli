---
name: rive-scene-schema
description: Complete JSON schema reference for generating valid Rive .riv scene files. Covers all ObjectSpec types, ArtboardSpec, hierarchy rules, color format, and field types.
---

# Rive Scene JSON Schema

Reference for writing valid Rive scene JSON files consumed by `rive-cli generate`.

---

## Root Structure

```json
{
  "scene_format_version": 1,
  "artboard": {...}
}
```

OR for multi-artboard:

```json
{
  "scene_format_version": 1,
  "artboards": [{...}, {...}]
}
```

- `scene_format_version`: REQUIRED, must be exactly `1`
- `artboard`: Single artboard object (use this OR `artboards`, not both)
- `artboards`: Array of artboard objects for multi-artboard files

---

## ArtboardSpec

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `name` | String | Yes | Must be unique across artboards |
| `preset` | String | No | One of: `mobile`, `tablet`, `desktop`, `square`, `banner`, `story` |
| `width` | f32 | If no preset | Must be > 0. Ignored if preset used |
| `height` | f32 | If no preset | Must be > 0. Ignored if preset used |
| `children` | Vec<ObjectSpec> | Yes | Can be empty `[]` |
| `animations` | Vec<AnimationSpec> | No | See animation skill |
| `state_machines` | Vec<StateMachineSpec> | No | See state machine skill |

**Preset dimensions:**
- `mobile`: 390x844
- `tablet`: 768x1024
- `desktop`: 1440x900
- `square`: 500x500
- `banner`: 728x90
- `story`: 1080x1920

---

## ObjectSpec Types

All objects use `"type"` field with snake_case value. Names must be unique per artboard.

### Shape

Container for paths and paints.

```json
{
  "type": "shape",
  "name": "MyShape",
  "x": 250,
  "y": 250,
  "children": [...]
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `x` | f32 | No | 0.0 |
| `y` | f32 | No | 0.0 |
| `children` | Vec<ObjectSpec> | No | [] |

### Ellipse

Parametric ellipse path.

```json
{
  "type": "ellipse",
  "name": "Circle",
  "width": 200,
  "height": 200,
  "origin_x": 0.5,
  "origin_y": 0.5
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `width` | f32 | Yes | - |
| `height` | f32 | Yes | - |
| `origin_x` | f32 | No | 0.0 |
| `origin_y` | f32 | No | 0.0 |

### Rectangle

Parametric rectangle path.

```json
{
  "type": "rectangle",
  "name": "Box",
  "width": 150,
  "height": 100,
  "corner_radius": 10,
  "origin_x": 0.5,
  "origin_y": 0.5
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `width` | f32 | Yes | - |
| `height` | f32 | Yes | - |
| `corner_radius` | f32 | No | 0.0 |
| `origin_x` | f32 | No | 0.0 |
| `origin_y` | f32 | No | 0.0 |

### Fill

Shape paint for filling interior.

```json
{
  "type": "fill",
  "name": "MyFill",
  "fill_rule": "nonzero",
  "children": [...]
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `fill_rule` | String/u64 | No | 0 (nonzero) |
| `children` | Vec<ObjectSpec> | No | [] |

`fill_rule` values: `"nonzero"` (0) or `"evenodd"` (1)

### Stroke

Shape paint for outlining.

```json
{
  "type": "stroke",
  "name": "MyStroke",
  "thickness": 2.0,
  "cap": "round",
  "join": "miter",
  "children": [...]
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `thickness` | f32 | No | 1.0 |
| `cap` | String/u64 | No | 0 (butt) |
| `join` | String/u64 | No | 0 (miter) |
| `children` | Vec<ObjectSpec> | No | [] |

`cap` values: `"butt"` (0), `"round"` (1), `"square"` (2)

`join` values: `"miter"` (0), `"round"` (1), `"bevel"` (2)

### SolidColor

Color source for fills/strokes.

```json
{
  "type": "solid_color",
  "name": "Red",
  "color": "#FF0000"
}
```

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `name` | String | Yes | - |
| `color` | String | Yes | Hex format, see Color Format section |

### LinearGradient

Linear gradient paint source.

```json
{
  "type": "linear_gradient",
  "name": "LG",
  "start_x": 0,
  "start_y": 0,
  "end_x": 100,
  "end_y": 100,
  "children": [...]
}
```

| Field | Type | Required |
|-------|------|----------|
| `name` | String | Yes |
| `start_x` | f32 | Yes |
| `start_y` | f32 | Yes |
| `end_x` | f32 | Yes |
| `end_y` | f32 | Yes |
| `children` | Vec<ObjectSpec> | No |

Children must be `gradient_stop` objects.

### RadialGradient

Radial gradient paint source.

```json
{
  "type": "radial_gradient",
  "name": "RG",
  "start_x": 50,
  "start_y": 50,
  "end_x": 150,
  "end_y": 150,
  "children": [...]
}
```

| Field | Type | Required |
|-------|------|----------|
| `name` | String | Yes |
| `start_x` | f32 | Yes |
| `start_y` | f32 | Yes |
| `end_x` | f32 | Yes |
| `end_y` | f32 | Yes |
| `children` | Vec<ObjectSpec> | No |

### GradientStop

Color stop for gradients. Name is auto-generated if omitted.

```json
{
  "type": "gradient_stop",
  "name": "Stop1",
  "color": "#FF0000",
  "position": 0.0
}
```

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `name` | String | No | Auto-generated as `gradient_stop_N` |
| `color` | String | Yes | Hex color |
| `position` | f32 | Yes | Range 0.0 to 1.0 |

### Node

Basic transform node.

```json
{
  "type": "node",
  "name": "MyNode",
  "x": 100,
  "y": 100
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `x` | f32 | No | 0.0 |
| `y` | f32 | No | 0.0 |

### Image

Image reference node.

```json
{
  "type": "image",
  "name": "MyImage",
  "asset_id": 0,
  "x": 100,
  "y": 100
}
```

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `name` | String | Yes | - |
| `asset_id` | u64 | Yes | Index into ImageAsset objects |
| `x` | f32 | No | 0.0 |
| `y` | f32 | No | 0.0 |

### Path

Vector path object.

```json
{
  "type": "path",
  "name": "MyPath",
  "path_flags": 0
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `path_flags` | u64 | No | 0 |

### TrimPath

Path trimming effect. Must be child of Fill or Stroke, NOT Shape.

```json
{
  "type": "trim_path",
  "name": "Trimmer",
  "start": 0.0,
  "end": 0.75,
  "offset": 0.0,
  "mode": "sequential"
}
```

| Field | Type | Required | Default | Notes |
|-------|------|----------|---------|-------|
| `name` | String | Yes | - | - |
| `start` | f32 | No | 0.0 | Start trim position |
| `end` | f32 | No | 1.0 | End trim position |
| `offset` | f32 | No | 0.0 | Trim offset |
| `mode` | String/u64 | No | 1 | `sequential` (1) or `synchronized` (2) |

### NestedArtboard

Reference to another artboard in the same file.

```json
{
  "type": "nested_artboard",
  "name": "Nested",
  "source_artboard": "OtherArtboard",
  "x": 100,
  "y": 100
}
```

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `name` | String | Yes | - |
| `source_artboard` | String | Yes | Name of referenced artboard |
| `x` | f32 | No | 0.0 |
| `y` | f32 | No | 0.0 |

Cannot reference its own containing artboard.

### Bone

Skeletal bone with optional children.

```json
{
  "type": "bone",
  "name": "Arm",
  "length": 50.0,
  "children": [...]
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `length` | f32 | No | 0.0 |
| `children` | Vec<ObjectSpec> | No | [] |

### RootBone

Root bone with position.

```json
{
  "type": "root_bone",
  "name": "Root",
  "x": 250,
  "y": 250,
  "length": 80.0,
  "children": [...]
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `x` | f32 | No | 0.0 |
| `y` | f32 | No | 0.0 |
| `length` | f32 | No | 0.0 |
| `children` | Vec<ObjectSpec> | No | [] |

### Skin

Mesh skinning container.

```json
{
  "type": "skin",
  "name": "MySkin",
  "xx": 1.0,
  "yx": 0.0,
  "xy": 0.0,
  "yy": 1.0,
  "tx": 0.0,
  "ty": 0.0,
  "children": [...]
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `xx` | f32 | No | 1.0 |
| `yx` | f32 | No | 0.0 |
| `xy` | f32 | No | 0.0 |
| `yy` | f32 | No | 1.0 |
| `tx` | f32 | No | 0.0 |
| `ty` | f32 | No | 0.0 |
| `children` | Vec<ObjectSpec> | No | [] |

Children must be `tendon` objects.

### Tendon

Bone attachment for skinning.

```json
{
  "type": "tendon",
  "name": "ArmTendon",
  "bone": "Arm",
  "xx": 1.0,
  "yx": 0.0,
  "xy": 0.0,
  "yy": 1.0,
  "tx": 0.0,
  "ty": 0.0
}
```

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `name` | String | Yes | - |
| `bone` | String | Yes | Name of bone to attach to |
| `xx` | f32 | No | 1.0 |
| `yx` | f32 | No | 0.0 |
| `xy` | f32 | No | 0.0 |
| `yy` | f32 | No | 1.0 |
| `tx` | f32 | No | 0.0 |
| `ty` | f32 | No | 0.0 |

### Weight

Vertex weight data.

```json
{
  "type": "weight",
  "name": "MyWeight",
  "values": 255,
  "indices": 1
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `values` | u64 | No | 0 |
| `indices` | u64 | No | 0 |

### CubicWeight

Cubic vertex weight data.

```json
{
  "type": "cubic_weight",
  "name": "SmoothWeight",
  "in_values": 128,
  "in_indices": 0,
  "out_values": 128,
  "out_indices": 0
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `in_values` | u64 | No | 0 |
| `in_indices` | u64 | No | 0 |
| `out_values` | u64 | No | 0 |
| `out_indices` | u64 | No | 0 |

### IkConstraint

Inverse kinematics constraint.

```json
{
  "type": "ik_constraint",
  "name": "ArmIK",
  "target": "Target",
  "strength": 1.0,
  "invert_direction": false,
  "parent_bone_count": 2
}
```

| Field | Type | Required | Default | Notes |
|-------|------|----------|---------|-------|
| `name` | String | Yes | - | - |
| `target` | String | Yes | - | Target object name |
| `strength` | f32 | No | 1.0 | 0.0 to 1.0 |
| `invert_direction` | bool | No | false | - |
| `parent_bone_count` | u64 | No | 0 | Number of parent bones |

### DistanceConstraint

Distance-maintaining constraint.

```json
{
  "type": "distance_constraint",
  "name": "KeepDistance",
  "target": "Target",
  "strength": 0.8,
  "distance": 50.0,
  "mode_value": 0
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `target` | String | Yes | - |
| `strength` | f32 | No | 1.0 |
| `distance` | f32 | No | 0.0 |
| `mode_value` | u64 | No | 0 |

### TransformConstraint

Full transform copying constraint.

```json
{
  "type": "transform_constraint",
  "name": "Follow",
  "target": "Target",
  "strength": 0.5,
  "source_space_value": 0,
  "dest_space_value": 0,
  "origin_x": 0.0,
  "origin_y": 0.0
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `target` | String | Yes | - |
| `strength` | f32 | No | 1.0 |
| `source_space_value` | u64 | No | 0 |
| `dest_space_value` | u64 | No | 0 |
| `origin_x` | f32 | No | 0.0 |
| `origin_y` | f32 | No | 0.0 |

### TranslationConstraint

Position copying constraint.

```json
{
  "type": "translation_constraint",
  "name": "FollowPos",
  "target": "Target",
  "strength": 1.0,
  "source_space_value": 0,
  "dest_space_value": 0,
  "copy_factor": 1.0,
  "min_value": 0.0,
  "max_value": 0.0,
  "offset": false,
  "does_copy": true,
  "min": false,
  "max": false,
  "min_max_space_value": 0,
  "copy_factor_y": 1.0,
  "min_value_y": 0.0,
  "max_value_y": 0.0,
  "does_copy_y": true,
  "min_y": false,
  "max_y": false
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `target` | String | Yes | - |
| `strength` | f32 | No | 1.0 |
| `source_space_value` | u64 | No | 0 |
| `dest_space_value` | u64 | No | 0 |
| `copy_factor` | f32 | No | 1.0 |
| `min_value` | f32 | No | 0.0 |
| `max_value` | f32 | No | 0.0 |
| `offset` | bool | No | false |
| `does_copy` | bool | No | true |
| `min` | bool | No | false |
| `max` | bool | No | false |
| `min_max_space_value` | u64 | No | 0 |
| `copy_factor_y` | f32 | No | 1.0 |
| `min_value_y` | f32 | No | 0.0 |
| `max_value_y` | f32 | No | 0.0 |
| `does_copy_y` | bool | No | true |
| `min_y` | bool | No | false |
| `max_y` | bool | No | false |

### ScaleConstraint

Scale copying constraint. Same fields as TranslationConstraint.

### RotationConstraint

Rotation copying constraint.

```json
{
  "type": "rotation_constraint",
  "name": "FollowRot",
  "target": "Target",
  "strength": 0.9,
  "source_space_value": 0,
  "dest_space_value": 0,
  "copy_factor": 1.0,
  "min_value": 0.0,
  "max_value": 0.0,
  "offset": false,
  "does_copy": true,
  "min": false,
  "max": false,
  "min_max_space_value": 0
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `target` | String | Yes | - |
| `strength` | f32 | No | 1.0 |
| `source_space_value` | u64 | No | 0 |
| `dest_space_value` | u64 | No | 0 |
| `copy_factor` | f32 | No | 1.0 |
| `min_value` | f32 | No | 0.0 |
| `max_value` | f32 | No | 0.0 |
| `offset` | bool | No | false |
| `does_copy` | bool | No | true |
| `min` | bool | No | false |
| `max` | bool | No | false |
| `min_max_space_value` | u64 | No | 0 |

### Text

Text object container.

```json
{
  "type": "text",
  "name": "MyText",
  "align_value": 1,
  "sizing_value": 2,
  "overflow_value": 1,
  "width": 400,
  "height": 200,
  "origin_x": 0.5,
  "origin_y": 0.5,
  "paragraph_spacing": 12,
  "origin_value": 0,
  "children": [...]
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `align_value` | u64 | No | 0 |
| `sizing_value` | u64 | No | 0 |
| `overflow_value` | u64 | No | 0 |
| `width` | f32 | No | 0.0 |
| `height` | f32 | No | 0.0 |
| `origin_x` | f32 | No | 0.0 |
| `origin_y` | f32 | No | 0.0 |
| `paragraph_spacing` | f32 | No | 0.0 |
| `origin_value` | u64 | No | 0 |
| `children` | Vec<ObjectSpec> | No | [] |

Children: `text_style`, `text_value_run`

### TextStyle

Text styling definition.

```json
{
  "type": "text_style",
  "name": "Heading",
  "font_size": 24,
  "line_height": 1.5,
  "letter_spacing": 0.5,
  "font_asset_id": 1
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `font_size` | f32 | No | 16.0 |
| `line_height` | f32 | No | 1.2 |
| `letter_spacing` | f32 | No | 0.0 |
| `font_asset_id` | u64 | No | 0 |

### TextValueRun

Text content run.

```json
{
  "type": "text_value_run",
  "name": "Content",
  "text": "Hello World",
  "style_id": 0
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `text` | String | Yes | - |
| `style_id` | u64 | No | 0 |

### ImageAsset

Image asset reference.

```json
{
  "type": "image_asset",
  "name": "Photo",
  "asset_id": 1,
  "cdn_base_url": "https://cdn.example.com/"
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `asset_id` | u64 | No | 0 |
| `cdn_base_url` | String | No | "" |

### FontAsset

Font asset reference.

```json
{
  "type": "font_asset",
  "name": "Roboto",
  "asset_id": 1,
  "cdn_base_url": "https://cdn.example.com/"
}
```

Same fields as ImageAsset.

### AudioAsset

Audio asset reference.

```json
{
  "type": "audio_asset",
  "name": "Sound",
  "asset_id": 1,
  "cdn_base_url": "https://cdn.example.com/"
}
```

Same fields as ImageAsset.

### LayoutComponent

Layout container.

```json
{
  "type": "layout_component",
  "name": "Container",
  "clip": true,
  "width": 500,
  "height": 300,
  "style_id": 0,
  "fractional_width": 1.0,
  "fractional_height": 1.0,
  "children": [...]
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `clip` | bool | No | false |
| `width` | f32 | No | 0.0 |
| `height` | f32 | No | 0.0 |
| `style_id` | u64 | No | 0 |
| `fractional_width` | f32 | No | 0.0 |
| `fractional_height` | f32 | No | 0.0 |
| `children` | Vec<ObjectSpec> | No | [] |

### LayoutComponentStyle

Layout styling (flexbox properties).

All fields optional (default 0/false):

```json
{
  "type": "layout_component_style",
  "name": "FlexStyle",
  "gap_horizontal": 10,
  "gap_vertical": 10,
  "max_width": 500,
  "max_height": 400,
  "min_width": 100,
  "min_height": 100,
  "border_left": 1,
  "border_right": 1,
  "border_top": 1,
  "border_bottom": 1,
  "margin_left": 10,
  "margin_right": 10,
  "margin_top": 10,
  "margin_bottom": 10,
  "padding_left": 20,
  "padding_right": 20,
  "padding_top": 20,
  "padding_bottom": 20,
  "position_left": 0,
  "position_right": 0,
  "position_top": 0,
  "position_bottom": 0,
  "flex_direction": 0,
  "flex_wrap": 0,
  "align_items": 0,
  "align_content": 0,
  "justify_content": 0,
  "display": 0,
  "position_type": 0,
  "overflow": 0,
  "intrinsically_sized": false,
  "width_units": 0,
  "height_units": 0,
  "flex_grow": 0,
  "flex_shrink": 1,
  "flex_basis": 0,
  "aspect_ratio": 0
}
```

### ViewModel

Data binding view model.

```json
{
  "type": "view_model",
  "name": "UserData",
  "children": [...]
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `children` | Vec<ObjectSpec> | No | [] |

Children: `view_model_property`

### ViewModelProperty

View model property definition.

```json
{
  "type": "view_model_property",
  "name": "score",
  "property_type_value": 0
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `name` | String | Yes | - |
| `property_type_value` | u64 | No | 0 |

### DataBind

Data binding connection.

```json
{
  "type": "data_bind",
  "property_key": 100,
  "flags": 0,
  "converter_id": 0
}
```

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `property_key` | u64 | Yes | - |
| `flags` | u64 | Yes | - |
| `converter_id` | u64 | No | 0 |

---

## Color Format

Hex string in one of two formats:

- `#RRGGBB` - 6-digit, alpha defaults to FF (fully opaque)
- `#AARRGGBB` - 8-digit with alpha channel

Examples:
- `"#FF0000"` - Solid red
- `"#00FF00"` - Solid green
- `"#0000FF"` - Solid blue
- `"#80FF0000"` - 50% transparent red
- `"#FFFF00FF"` - Fully opaque magenta

---

## Object Hierarchy Rules

Valid parent-child relationships:

| Parent Type | Valid Children |
|-------------|----------------|
| Artboard | shape, node, image, path, bone, root_bone, skin, weight, cubic_weight, constraint types, text, layout_component, view_model, image_asset, font_asset, audio_asset |
| Shape | ellipse, rectangle, path, fill, stroke |
| Fill | solid_color, linear_gradient, radial_gradient, trim_path |
| Stroke | solid_color, linear_gradient, radial_gradient, trim_path |
| LinearGradient | gradient_stop |
| RadialGradient | gradient_stop |
| Bone | bone, tendon |
| RootBone | bone, tendon |
| Skin | tendon |
| Text | text_style, text_value_run |
| LayoutComponent | Any valid artboard child |
| ViewModel | view_model_property |

**Critical:** `trim_path` must be child of Fill or Stroke. Putting it directly under Shape causes runtime errors.

---

## Minimal Complete Example

Red circle at center:

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
        "name": "Circle",
        "x": 250,
        "y": 250,
        "children": [
          {
            "type": "ellipse",
            "name": "CirclePath",
            "width": 100,
            "height": 100
          },
          {
            "type": "fill",
            "name": "CircleFill",
            "children": [
              {
                "type": "solid_color",
                "name": "CircleColor",
                "color": "#FF0000"
              }
            ]
          }
        ]
      }
    ]
  }
}
```

---

## Validation Rules

- All object names must be unique within an artboard
- `gradient_stop` names are auto-generated if omitted
- `width` and `height` must be non-negative
- `trim_path` mode must be `"sequential"` (1) or `"synchronized"` (2), never 0
- Nested artboard references cannot be circular
- Image `asset_id` must reference a prior `image_asset`
- Constraint `target` must reference an existing object by name
- Tendon `bone` must reference an existing bone by name
