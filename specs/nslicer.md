# 9-Slice (NSlicer) Types Specification

## Overview

9-slice scaling divides an image or node into a 3x3 grid and scales each section independently, preserving corners and stretching edges/center. This is commonly used for resizable UI elements like buttons, panels, and dialog boxes.

The NSlicer system in the Rive format consists of five object types that work together to define slicing axes, tile behavior, and the resulting sliced node.

---

## Type: NSlicerTileMode

**typeKey: 491**

Controls how a specific patch (cell) within the 9-slice grid is rendered. Each patch can be stretched, tiled, or hidden independently.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent NSlicer object (artboard-local index) |
| patchIndex | 672 | UInt | yes | - | Index of the patch this mode applies to (0-8, row-major) |
| style | 673 | UInt | no | 0 | Tile mode: 0 = stretch, 1 = tile, 2 = hidden |

### Hierarchy

- **Parent**: NSlicer (typeKey 493)
- **Children**: none

### Notes

- The 9-slice grid has 9 patches indexed 0-8 in row-major order: top-left(0), top-center(1), top-right(2), middle-left(3), center(4), middle-right(5), bottom-left(6), bottom-center(7), bottom-right(8).
- Default behavior (stretch) does not require a NSlicerTileMode object; only create these to override specific patches to tile or hide.
- The `style` value determines rendering: stretch maps the patch texture to fill, tile repeats it, and hidden omits it entirely.

---

## Type: NSlicer

**typeKey: 493**

The main 9-slice container. Defines the source dimensions and references axis children that partition the image into the 3x3 grid. An NSlicer is placed as a child of an Image or Node and drives the slicing behavior.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent Image or Node (artboard-local index) |
| initialWidth | 697 | Float | no | 0.0 | Original (unscaled) width of the source content |
| initialHeight | 698 | Float | no | 0.0 | Original (unscaled) height of the source content |
| width | 699 | Float | no | 0.0 | Current/target width for the sliced output |
| height | 700 | Float | no | 0.0 | Current/target height for the sliced output |

### Hierarchy

- **Parent**: Image (typeKey 100) or Node (typeKey 2) -- the visual element being sliced
- **Children**: AxisX (typeKey 495), AxisY (typeKey 494), NSlicerTileMode (typeKey 491)

### Notes

- Exactly two AxisX children define the two vertical cut lines (left and right boundaries of the center column).
- Exactly two AxisY children define the two horizontal cut lines (top and bottom boundaries of the center row).
- NSlicerTileMode children are optional; they override the default stretch behavior for specific patches.
- `initialWidth`/`initialHeight` represent the content dimensions at author time. The runtime computes scale factors as `width / initialWidth` and `height / initialHeight`.

---

## Type: AxisY

**typeKey: 494**

Defines a horizontal cut line (Y-axis position) for the 9-slice grid. Two AxisY children on an NSlicer divide the content into three horizontal bands.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent NSlicer (artboard-local index) |
| offset | 675 | Float | yes | 0.0 | Position of the cut line |
| normalized | 676 | UInt (bool) | no | 0 | If 1, offset is a 0..1 fraction of height; if 0, offset is in pixels |

### Hierarchy

- **Parent**: NSlicer (typeKey 493)
- **Children**: none

### Notes

- The first AxisY (smaller offset) defines the top boundary of the center row; the second defines the bottom boundary.
- When `normalized` is 1, offset 0.25 means the cut is at 25% of the source height.
- Axis children must be emitted before NSlicerTileMode children.

---

## Type: AxisX

**typeKey: 495**

Defines a vertical cut line (X-axis position) for the 9-slice grid. Two AxisX children on an NSlicer divide the content into three vertical columns.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent NSlicer (artboard-local index) |
| offset | 675 | Float | yes | 0.0 | Position of the cut line |
| normalized | 676 | UInt (bool) | no | 0 | If 1, offset is a 0..1 fraction of width; if 0, offset is in pixels |

### Hierarchy

- **Parent**: NSlicer (typeKey 493)
- **Children**: none

### Notes

- The first AxisX (smaller offset) defines the left boundary of the center column; the second defines the right boundary.
- AxisX and AxisY share the same property keys (675, 676) since they are structurally identical; only their typeKey differs.

---

## Type: NSlicedNode

**typeKey: 508**

A drawable node that renders content with 9-slice scaling applied. This is the output node that the runtime creates and renders. It inherits from Node and adds transform/drawable capabilities.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent artboard or group node (artboard-local index) |
| x | 13 | Float | no | 0.0 | X translation |
| y | 14 | Float | no | 0.0 | Y translation |
| rotation | 15 | Float | no | 0.0 | Rotation in radians |
| scaleX | 16 | Float | no | 1.0 | Horizontal scale |
| scaleY | 17 | Float | no | 1.0 | Vertical scale |
| opacity | 18 | Float | no | 1.0 | Opacity 0..1 |

### Hierarchy

- **Parent**: Artboard (typeKey 1) or any container node
- **Children**: NSlicer (typeKey 493), plus paint children (Fill, Stroke) if needed

### Notes

- NSlicedNode is the top-level drawable that hosts the NSlicer and its axes. The visual content (typically an Image) is a child of the NSlicedNode.
- Inherits all TransformComponent properties (x, y, rotation, scaleX, scaleY, opacity).
- Only write non-default properties (same convention as other Node-derived types).

---

## Parent-Child Relationships

```
Artboard
  |
  +-- NSlicedNode (typeKey 508)
        |
        +-- Image (typeKey 100)    <-- the source image being sliced
        |
        +-- NSlicer (typeKey 493)  <-- defines the 9-slice grid
              |
              +-- AxisX (typeKey 495)  <-- left vertical cut (e.g., offset=30)
              +-- AxisX (typeKey 495)  <-- right vertical cut (e.g., offset=70)
              +-- AxisY (typeKey 494)  <-- top horizontal cut (e.g., offset=30)
              +-- AxisY (typeKey 494)  <-- bottom horizontal cut (e.g., offset=70)
              +-- NSlicerTileMode (typeKey 491)  <-- optional: override patch 4 to tile
```

---

## JSON Schema

```json
{
  "type": "n_sliced_node",
  "name": "sliced_button",
  "x": 100,
  "y": 50,
  "children": [
    {
      "type": "image",
      "name": "button_bg",
      "asset_id": 1
    },
    {
      "type": "nslicer",
      "name": "slicer",
      "initial_width": 200,
      "initial_height": 80,
      "width": 400,
      "height": 80,
      "children": [
        {
          "type": "axis_x",
          "name": "left_cut",
          "offset": 30.0,
          "normalized": false
        },
        {
          "type": "axis_x",
          "name": "right_cut",
          "offset": 170.0,
          "normalized": false
        },
        {
          "type": "axis_y",
          "name": "top_cut",
          "offset": 20.0,
          "normalized": false
        },
        {
          "type": "axis_y",
          "name": "bottom_cut",
          "offset": 60.0,
          "normalized": false
        },
        {
          "type": "nslicer_tile_mode",
          "name": "center_tile",
          "patch_index": 4,
          "style": 1
        }
      ]
    }
  ]
}
```

### JSON Field Reference

**n_sliced_node**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| type | `"n_sliced_node"` | yes | Discriminator |
| name | string | yes | Component name |
| x | float | no | X position |
| y | float | no | Y position |
| children | array | yes | Must contain Image + NSlicer |

**nslicer**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| type | `"nslicer"` | yes | Discriminator |
| name | string | yes | Component name |
| initial_width | float | no | Source content width at design time |
| initial_height | float | no | Source content height at design time |
| width | float | no | Target output width |
| height | float | no | Target output height |
| children | array | yes | AxisX, AxisY, and optional NSlicerTileMode |

**axis_x / axis_y**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| type | `"axis_x"` or `"axis_y"` | yes | Discriminator |
| name | string | no | Component name |
| offset | float | yes | Cut position (pixels or normalized) |
| normalized | bool | no | If true, offset is 0..1 fraction |

**nslicer_tile_mode**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| type | `"nslicer_tile_mode"` | yes | Discriminator |
| name | string | no | Component name |
| patch_index | int | yes | Patch index 0-8 |
| style | int | no | 0=stretch, 1=tile, 2=hidden |

---

## Acceptance Criteria

1. **Axis encoding**: Two AxisX and two AxisY objects are emitted as children of NSlicer with correct property keys (675 for offset, 676 for normalized).
2. **NSlicer encoding**: NSlicer is emitted with typeKey 493 and writes initialWidth(697), initialHeight(698), width(699), height(700) as Float properties.
3. **TileMode encoding**: NSlicerTileMode objects are emitted with typeKey 491, patchIndex(672) as UInt, and style(673) as UInt.
4. **NSlicedNode encoding**: NSlicedNode is emitted with typeKey 508 and inherits Node/TransformComponent properties.
5. **Parent-child hierarchy**: parentId values correctly chain NSlicedNode -> NSlicer -> Axes/TileModes using artboard-local indices.
6. **Round-trip validation**: A generated .riv containing 9-slice types passes `validate` and `inspect` without errors.
7. **Runtime loading**: The .riv loads in the Rive WASM runtime without errors (Playwright regression test).
8. **Default suppression**: Properties at their default values (0.0 for floats, 0 for uints) are not written to the binary.
9. **Patch index bounds**: patch_index values outside 0-8 produce a builder error.
10. **Axis count validation**: The builder emits a warning or error if NSlicer does not have exactly 2 AxisX and 2 AxisY children.
