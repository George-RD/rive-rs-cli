# Dash and Feather Effects Specification

## Overview

Dash and feather effects modify the rendering of shapes and paths. DashPath and Dash create dashed-line patterns on strokes, while Feather applies a blur/feather effect to shape edges. All three are paint-level effects that attach as children of ShapePaint objects (Stroke or Fill).

---

## Type: DashPath

**typeKey: 506**

A dashed-path effect applied to a Stroke. When present as a child of a Stroke, it causes the stroke to render as a dashed line. DashPath contains Dash children that define the individual dash and gap segments.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent Stroke (artboard-local index) |
| offset | 690 | Float | no | 0.0 | Starting offset along the path before dashes begin |
| offsetIsPercentage | 691 | UInt (bool) | no | 0 | If 1, offset is a percentage of path length |

### Hierarchy

- **Parent**: Stroke (typeKey 24) -- must be a child of a Stroke, similar to TrimPath
- **Children**: Dash (typeKey 507) -- one or more dash/gap segment definitions

### Notes

- DashPath is a sibling concept to TrimPath: both are effects containers that live under a ShapePaint. The runtime's `EffectsContainer::from()` resolves them from ShapePaint types only.
- The `offset` property shifts the entire dash pattern along the path. When `offsetIsPercentage` is 1, offset 50.0 means start the pattern at 50% of the path length.
- At least one Dash child is required for the effect to be visible.
- Animating the `offset` property creates a "marching ants" or flowing-dash animation.

---

## Type: Dash

**typeKey: 507**

Defines a single segment within a DashPath pattern. Dash objects are emitted in sequence -- the first defines a visible dash length, the second a gap length, the third a dash, and so on, alternating.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent DashPath (artboard-local index) |
| length | 692 | Float | yes | 0.0 | Length of this dash or gap segment |
| lengthIsPercentage | 693 | UInt (bool) | no | 0 | If 1, length is a percentage of path length |

### Hierarchy

- **Parent**: DashPath (typeKey 506)
- **Children**: none

### Notes

- Dash segments alternate between visible (dash) and invisible (gap). The first Dash child is a visible segment, the second is a gap, and so on.
- If only one Dash is provided, the runtime uses the same length for both dash and gap (symmetric pattern).
- If an odd number of Dash children exist, the pattern wraps (the last dash's gap uses the first dash's length on the next cycle).
- A Dash with `length` of 0.0 produces a dot (when combined with round stroke caps, this creates a dotted line).
- When `lengthIsPercentage` is 1, the dash length scales with the path length, maintaining a consistent visual ratio regardless of path size.

---

## Type: Feather

**typeKey: 533**

Applies a blur/feather effect to a shape's edges. Feather softens the boundary of a fill or stroke, creating a gradient falloff at edges. It can be applied to both Fill and Stroke paints.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent Fill or Stroke (artboard-local index) |
| strength | 749 | Float | no | 0.0 | Blur radius / feather strength in pixels |
| offsetX | 750 | Float | no | 0.0 | Horizontal offset of the feather effect |
| offsetY | 751 | Float | no | 0.0 | Vertical offset of the feather effect |
| spaceValue | 748 | UInt | no | 0 | Coordinate space for the effect (0 = local, 1 = world) |
| inner | 752 | UInt (bool) | no | 0 | If 1, feather inward (inside the shape); if 0, feather outward |

### Hierarchy

- **Parent**: Fill (typeKey 20) or Stroke (typeKey 24) -- must be a child of a ShapePaint
- **Children**: none

### Notes

- Feather is functionally similar to CSS `box-shadow` or SVG `feGaussianBlur`. The `strength` property controls the blur radius.
- `offsetX`/`offsetY` shift the blur relative to the shape, enabling drop-shadow-like effects when combined with a second fill.
- The `inner` flag controls directionality: outward feathering (default) blurs the outer edges into transparency; inward feathering blurs from the edges toward the interior.
- Feather must be a child of a ShapePaint (Fill or Stroke), not directly under a Shape. This follows the same parent constraint as TrimPath and DashPath.
- Setting `strength` to 0.0 disables the effect; non-zero values enable the Gaussian blur.
- The `spaceValue` property determines whether the blur is applied in local (object) or world (scene) coordinate space, which affects how the blur responds to transforms.

---

## Parent-Child Relationships

```
Shape (typeKey 3)
  |
  +-- Ellipse / Rectangle / Path     <-- geometry
  |
  +-- Fill (typeKey 20)               <-- paint
  |     |
  |     +-- SolidColor (typeKey 18)   <-- color source
  |     +-- Feather (typeKey 533)     <-- feather on fill (optional)
  |
  +-- Stroke (typeKey 24)             <-- paint
        |
        +-- SolidColor (typeKey 18)   <-- color source
        +-- DashPath (typeKey 506)    <-- dash effect (optional)
        |     |
        |     +-- Dash (typeKey 507)  <-- dash segment
        |     +-- Dash (typeKey 507)  <-- gap segment
        |
        +-- TrimPath (typeKey 47)     <-- trim effect (optional, existing)
        +-- Feather (typeKey 533)     <-- feather on stroke (optional)
```

---

## JSON Schema

### Dashed Stroke Example

```json
{
  "type": "shape",
  "name": "dashed_line",
  "children": [
    {
      "type": "points_path",
      "name": "line",
      "is_closed": false,
      "children": [
        {
          "type": "straight_vertex",
          "name": "start",
          "x": 0,
          "y": 0
        },
        {
          "type": "straight_vertex",
          "name": "end",
          "x": 200,
          "y": 0
        }
      ]
    },
    {
      "type": "stroke",
      "name": "dashed_stroke",
      "thickness": 3.0,
      "cap": "round",
      "children": [
        {
          "type": "solid_color",
          "name": "stroke_color",
          "color": "#000000"
        },
        {
          "type": "dash_path",
          "name": "dash_effect",
          "offset": 0.0,
          "offset_is_percentage": false,
          "children": [
            {
              "type": "dash",
              "name": "dash_segment",
              "length": 10.0,
              "length_is_percentage": false
            },
            {
              "type": "dash",
              "name": "gap_segment",
              "length": 5.0,
              "length_is_percentage": false
            }
          ]
        }
      ]
    }
  ]
}
```

### Feathered Fill Example

```json
{
  "type": "shape",
  "name": "soft_circle",
  "children": [
    {
      "type": "ellipse",
      "name": "circle",
      "width": 100,
      "height": 100
    },
    {
      "type": "fill",
      "name": "soft_fill",
      "children": [
        {
          "type": "solid_color",
          "name": "fill_color",
          "color": "#FF6600"
        },
        {
          "type": "feather",
          "name": "blur",
          "strength": 8.0,
          "offset_x": 0.0,
          "offset_y": 2.0,
          "inner": false
        }
      ]
    }
  ]
}
```

### JSON Field Reference

**dash_path**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| type | `"dash_path"` | yes | Discriminator |
| name | string | no | Component name |
| offset | float | no | Dash pattern offset along path |
| offset_is_percentage | bool | no | Whether offset is a percentage |
| children | array | yes | One or more Dash segments |

**dash**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| type | `"dash"` | yes | Discriminator |
| name | string | no | Component name |
| length | float | yes | Dash or gap length |
| length_is_percentage | bool | no | Whether length is a percentage |

**feather**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| type | `"feather"` | yes | Discriminator |
| name | string | no | Component name |
| strength | float | no | Blur radius in pixels |
| offset_x | float | no | Horizontal blur offset |
| offset_y | float | no | Vertical blur offset |
| space_value | int | no | 0=local, 1=world coordinate space |
| inner | bool | no | If true, feather inward instead of outward |

---

## Acceptance Criteria

1. **DashPath encoding**: DashPath is emitted with typeKey 506, offset(690) as Float, and offsetIsPercentage(691) as UInt.
2. **Dash encoding**: Dash is emitted with typeKey 507, length(692) as Float, and lengthIsPercentage(693) as UInt.
3. **Feather encoding**: Feather is emitted with typeKey 533 and writes strength(749) as Float, offsetX(750) as Float, offsetY(751) as Float, spaceValue(748) as UInt, inner(752) as UInt.
4. **Parent constraint enforcement**: The builder rejects DashPath/Feather if the parent is not a ShapePaint (Fill or Stroke). DashPath specifically requires Stroke as parent.
5. **Dash parent constraint**: Dash objects must be children of DashPath, not directly under a Stroke.
6. **DashPath requires children**: The builder emits an error if a DashPath has no Dash children.
7. **Round-trip validation**: Generated .riv files containing dash and feather effects pass `validate` and `inspect` without errors.
8. **Runtime loading**: The .riv loads in the Rive WASM runtime without errors (Playwright regression test).
9. **Default suppression**: Properties at their default values are not written to the binary.
10. **Property key registration**: All new property keys (690, 691, 692, 693, 748, 749, 750, 751, 752) are registered in `property_backing_type()` and `is_bool_property()` (for 691, 693, and 752) in `core.rs`.
11. **Dash animation**: Animating DashPath offset (property 690) produces a smooth marching-dash effect when keyframed.
12. **Feather on Fill and Stroke**: Feather works correctly as a child of both Fill and Stroke paints.
