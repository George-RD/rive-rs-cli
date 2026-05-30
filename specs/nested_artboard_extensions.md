# Nested Artboard Extensions

Spec for eight additional nested artboard object types that extend the existing NestedArtboard (92), NestedStateMachine (95), and NestedSimpleAnimation (96) system.

## Background

The Rive runtime supports composing artboards by nesting one inside another. The current CLI implements the base NestedArtboard object (which references a source artboard) and two animation controllers (NestedStateMachine, NestedSimpleAnimation). This spec covers the remaining nested types: two additional animation controllers, four nested input types, and two artboard-level variants.

---

## 1. NestedLinearAnimation

**typeKey: 97**

A nested animation controller that plays a LinearAnimation from the nested artboard. Unlike NestedSimpleAnimation (96), this type represents a direct linear animation reference without the speed/mix/isPlaying convenience properties -- those are controlled externally through the state machine or host artboard.

### Inheritance

```
Component (name=4, parentId=5)
  -> NestedAnimation (animationId=198)
    -> NestedLinearAnimation (typeKey=97)
```

### Properties

| Property | Key | Type | Required | Default | Description |
|----------|-----|------|----------|---------|-------------|
| name | 4 | String | yes | -- | Component name |
| parentId | 5 | UInt | yes | -- | Artboard-local index of parent (the NestedArtboard) |
| animationId | 198 | UInt | yes | -- | Index of the LinearAnimation in the nested artboard |
| mix | 200 | Float | no | 1.0 | Blend mix factor (0.0-1.0) |

### Parent-child relationship

NestedLinearAnimation is a child of a NestedArtboard object. It must appear sequentially after its parent NestedArtboard in the object list, with parentId pointing to the NestedArtboard's artboard-local index.

---

## 2. NestedRemapAnimation

**typeKey: 98**

A nested animation controller that allows time remapping of a LinearAnimation. Instead of playing at a fixed speed, the animation's current time is driven by an explicit `time` property (typically bound to a state machine number input or data binding), enabling scrubbing and non-linear playback.

### Inheritance

```
Component (name=4, parentId=5)
  -> NestedAnimation (animationId=198)
    -> NestedRemapAnimation (typeKey=98, time=202)
```

### Properties

| Property | Key | Type | Required | Default | Description |
|----------|-----|------|----------|---------|-------------|
| name | 4 | String | yes | -- | Component name |
| parentId | 5 | UInt | yes | -- | Artboard-local index of parent (the NestedArtboard) |
| animationId | 198 | UInt | yes | -- | Index of the LinearAnimation in the nested artboard |
| time | 202 | Float | no | 0.0 | Remapped time value in seconds; drives the animation position directly |

### Parent-child relationship

NestedRemapAnimation is a child of a NestedArtboard object, same as other NestedAnimation subtypes.

---

## 3. NestedInput (base)

**typeKey: 121**

Abstract base type for inputs that are forwarded into a nested artboard's state machine. Each NestedInput subtype corresponds to one of the state machine input types (trigger, bool, number). NestedInput itself defines the `nestedInputId` property that identifies which state machine input is being targeted.

### Inheritance

```
Component (name=4, parentId=5)
  -> NestedInput (typeKey=121, nestedInputId=400)
    -> NestedTrigger (typeKey=122)
    -> NestedBool (typeKey=123, nestedValue=238)
    -> NestedNumber (typeKey=124, nestedValue=239)
```

### Properties

| Property | Key | Type | Required | Default | Description |
|----------|-----|------|----------|---------|-------------|
| name | 4 | String | yes | -- | Component name |
| parentId | 5 | UInt | yes | -- | Artboard-local index of parent (the NestedArtboard) |
| nestedInputId | 400 | UInt | yes | -- | ID of the state machine input in the nested artboard's state machine being targeted |

### Parent-child relationship

NestedInput objects are children of a NestedArtboard. They appear after the NestedArtboard in the object list and reference it via parentId.

### Notes

- NestedInput (121) may appear in .riv files as a concrete object (acting as a base-only reference), but typically one of the three subtypes is used instead.
- The `nestedInputId` maps to a StateMachineInput index within the nested artboard's state machine.

---

## 4. NestedTrigger

**typeKey: 122**

A trigger input forwarded into a nested artboard's state machine. Triggers have no persistent value -- they fire once when activated.

### Inheritance

```
Component (name=4, parentId=5)
  -> NestedInput (nestedInputId=400)
    -> NestedTrigger (typeKey=122)
```

### Properties

| Property | Key | Type | Required | Default | Description |
|----------|-----|------|----------|---------|-------------|
| name | 4 | String | yes | -- | Component name |
| parentId | 5 | UInt | yes | -- | Artboard-local index of parent (the NestedArtboard) |
| nestedInputId | 400 | UInt | yes | -- | ID of the target StateMachineTrigger input |

### Parent-child relationship

Child of NestedArtboard.

---

## 5. NestedBool

**typeKey: 123**

A boolean input forwarded into a nested artboard's state machine.

### Inheritance

```
Component (name=4, parentId=5)
  -> NestedInput (nestedInputId=400)
    -> NestedBool (typeKey=123, nestedValue=238)
```

### Properties

| Property | Key | Type | Required | Default | Description |
|----------|-----|------|----------|---------|-------------|
| name | 4 | String | yes | -- | Component name |
| parentId | 5 | UInt | yes | -- | Artboard-local index of parent (the NestedArtboard) |
| nestedInputId | 400 | UInt | yes | -- | ID of the target StateMachineBool input |
| nestedValue | 238 | UInt (bool) | no | 0 (false) | Boolean value (0=false, 1=true). Backing type is UInt; encodes as CoreBoolType (single raw byte). |

### Encoding note

Property 238 has backing type UInt but represents a boolean. Like other CoreBoolType properties (isVisible=41, enableWorkArea=62, etc.), it encodes as a single raw byte, NOT LEB128 varuint.

### Parent-child relationship

Child of NestedArtboard.

---

## 6. NestedNumber

**typeKey: 124**

A numeric input forwarded into a nested artboard's state machine.

### Inheritance

```
Component (name=4, parentId=5)
  -> NestedInput (nestedInputId=400)
    -> NestedNumber (typeKey=124, nestedValue=239)
```

### Properties

| Property | Key | Type | Required | Default | Description |
|----------|-----|------|----------|---------|-------------|
| name | 4 | String | yes | -- | Component name |
| parentId | 5 | UInt | yes | -- | Artboard-local index of parent (the NestedArtboard) |
| nestedInputId | 400 | UInt | yes | -- | ID of the target StateMachineNumber input |
| nestedValue | 239 | Float | no | 0.0 | Numeric value to set on the nested state machine input |

### Parent-child relationship

Child of NestedArtboard.

---

## 7. NestedArtboardLeaf

**typeKey: 451**

A leaf-level variant of NestedArtboard. Used when the nested artboard does not participate in the layout system -- it renders as a simple embedded artboard with fixed dimensions from its source. This is the default nesting mode for non-layout artboards.

### Inheritance

```
Component (name=4, parentId=5)
  -> Node (x=13, y=14)
    -> TransformComponent (rotation=15, scaleX=16, scaleY=17, opacity=18)
      -> WorldTransformComponent
        -> Drawable (blendMode=23, flags=129)
          -> NestedArtboard (artboardId=197)
            -> NestedArtboardLeaf (typeKey=451)
```

### Properties

All properties inherited from NestedArtboard, no additional properties.

| Property | Key | Type | Required | Default | Description |
|----------|-----|------|----------|---------|-------------|
| name | 4 | String | yes | -- | Component name |
| parentId | 5 | UInt | yes | -- | Artboard-local index of parent |
| artboardId | 197 | UInt | yes | -- | Index of the source artboard (0-based across all artboards in the file) |
| x | 13 | Float | no | 0.0 | X position |
| y | 14 | Float | no | 0.0 | Y position |

### Parent-child relationship

NestedArtboardLeaf is a direct child of an Artboard (or a layout container within an artboard). It functions identically to NestedArtboard (92) but is explicitly typed as a leaf node for the runtime's layout system to distinguish from layout-participating nested artboards.

### Usage

Use NestedArtboardLeaf (451) instead of NestedArtboard (92) when the runtime version supports the leaf/layout distinction. The base NestedArtboard (92) is the legacy type that predates this split.

---

## 8. NestedArtboardLayout

**typeKey: 452**

A layout-aware variant of NestedArtboard. Used when the nested artboard participates in the Yoga/flexbox layout system. Unlike NestedArtboardLeaf, this variant responds to layout constraints (width/height units, flex properties, alignment) and can resize based on parent layout rules.

### Inheritance

```
Component (name=4, parentId=5)
  -> Node (x=13, y=14)
    -> TransformComponent (rotation=15, scaleX=16, scaleY=17, opacity=18)
      -> WorldTransformComponent
        -> Drawable (blendMode=23, flags=129)
          -> NestedArtboard (artboardId=197)
            -> LayoutComponent (width=7, height=8, clip=196, styleId=494)
              -> NestedArtboardLayout (typeKey=452)
```

### Properties

Inherits from both NestedArtboard and LayoutComponent.

| Property | Key | Type | Required | Default | Description |
|----------|-----|------|----------|---------|-------------|
| name | 4 | String | yes | -- | Component name |
| parentId | 5 | UInt | yes | -- | Artboard-local index of parent |
| artboardId | 197 | UInt | yes | -- | Index of the source artboard |
| x | 13 | Float | no | 0.0 | X position |
| y | 14 | Float | no | 0.0 | Y position |
| width | 7 | Float | no | 0.0 | Layout width |
| height | 8 | Float | no | 0.0 | Layout height |
| clip | 196 | UInt (bool) | no | 0 | Whether to clip content to bounds |
| styleId | 494 | UInt | no | -- | Reference to a LayoutComponentStyle object for flex/alignment properties |
| instanceWidth | 663 | Float | no | -- | Instance-level width override |
| instanceHeight | 664 | Float | no | -- | Instance-level height override |
| instanceWidthUnitsValue | 665 | UInt | no | -- | Units for instance width |
| instanceHeightUnitsValue | 666 | UInt | no | -- | Units for instance height |
| instanceWidthScaleType | 667 | UInt | no | -- | Scale type for instance width |
| instanceHeightScaleType | 668 | UInt | no | -- | Scale type for instance height |
| fractionalWidth | 706 | Float | no | -- | Fractional width (0.0-1.0) |
| fractionalHeight | 707 | Float | no | -- | Fractional height (0.0-1.0) |

### Parent-child relationship

NestedArtboardLayout is a child of an Artboard or LayoutComponent. It participates in the layout system and may itself contain child NestedInput/NestedAnimation objects.

### Usage

Use NestedArtboardLayout (452) when the nested artboard should respect flexbox layout constraints from its parent. Typically paired with a LayoutComponentStyle object for detailed styling.

---

## Object Emission Order

When emitting a nested artboard with inputs and animations, objects must appear in this order:

```
NestedArtboard (92) / NestedArtboardLeaf (451) / NestedArtboardLayout (452)
  NestedLinearAnimation (97)        -- child, parentId -> NestedArtboard
  NestedRemapAnimation (98)         -- child, parentId -> NestedArtboard
  NestedStateMachine (95)           -- child, parentId -> NestedArtboard
  NestedSimpleAnimation (96)        -- child, parentId -> NestedArtboard
  NestedTrigger (122)               -- child, parentId -> NestedArtboard
  NestedBool (123)                  -- child, parentId -> NestedArtboard
  NestedNumber (124)                -- child, parentId -> NestedArtboard
```

All children reference the NestedArtboard via artboard-local parentId. The NestedArtboard itself references its parent (typically the Artboard root at index 0).

---

## JSON Schema

### NestedArtboard with animations and inputs

```json
{
  "type": "nested_artboard",
  "name": "my_component",
  "source_artboard": "ButtonComponent",
  "x": 100,
  "y": 50,
  "children": [
    {
      "type": "nested_linear_animation",
      "name": "intro",
      "animation": "intro_anim"
    },
    {
      "type": "nested_remap_animation",
      "name": "scrubber",
      "animation": "progress_anim",
      "time": 0.5
    },
    {
      "type": "nested_state_machine",
      "name": "sm_controller",
      "animation": "MainSM"
    },
    {
      "type": "nested_simple_animation",
      "name": "idle_loop",
      "animation": "idle",
      "speed": 1.0,
      "is_playing": true,
      "mix": 1.0
    },
    {
      "type": "nested_trigger",
      "name": "fire_click",
      "nested_input_id": 0
    },
    {
      "type": "nested_bool",
      "name": "is_active",
      "nested_input_id": 1,
      "value": true
    },
    {
      "type": "nested_number",
      "name": "progress",
      "nested_input_id": 2,
      "value": 0.75
    }
  ]
}
```

### NestedArtboardLeaf

```json
{
  "type": "nested_artboard_leaf",
  "name": "static_icon",
  "source_artboard": "IconComponent",
  "x": 20,
  "y": 20
}
```

### NestedArtboardLayout

```json
{
  "type": "nested_artboard_layout",
  "name": "card_content",
  "source_artboard": "CardComponent",
  "width": 300,
  "height": 200,
  "style_id": 5,
  "children": [
    {
      "type": "nested_state_machine",
      "name": "card_sm",
      "animation": "Interactions"
    }
  ]
}
```

---

## ObjectSpec additions (builder/spec.rs)

The following new variants should be added to the `ObjectSpec` enum:

```rust
NestedLinearAnimation {
    name: String,
    animation: String,
    mix: Option<f32>,
},
NestedRemapAnimation {
    name: String,
    animation: String,
    time: Option<f32>,
},
NestedTrigger {
    name: String,
    nested_input_id: u64,
},
NestedBool {
    name: String,
    nested_input_id: u64,
    value: Option<bool>,
},
NestedNumber {
    name: String,
    nested_input_id: u64,
    value: Option<f32>,
},
NestedArtboardLeaf {
    name: String,
    source_artboard: String,
    x: Option<f32>,
    y: Option<f32>,
    children: Option<Vec<ObjectSpec>>,
},
NestedArtboardLayout {
    name: String,
    source_artboard: String,
    x: Option<f32>,
    y: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    style_id: Option<u64>,
    children: Option<Vec<ObjectSpec>>,
},
```

The existing `NestedArtboard` variant should be extended with an optional `children` field to support inline nested inputs and animations:

```rust
NestedArtboard {
    name: String,
    source_artboard: String,
    x: Option<f32>,
    y: Option<f32>,
    children: Option<Vec<ObjectSpec>>,  // NEW: nested inputs and animations
},
```

---

## Property key constants to add (objects/core.rs)

```rust
pub const NESTED_LINEAR_ANIMATION: u16 = 97;      // type_keys
pub const NESTED_REMAP_ANIMATION: u16 = 98;        // type_keys
pub const NESTED_INPUT: u16 = 121;                  // type_keys
pub const NESTED_TRIGGER: u16 = 122;                // type_keys
pub const NESTED_BOOL: u16 = 123;                   // type_keys
pub const NESTED_NUMBER: u16 = 124;                  // type_keys
pub const NESTED_ARTBOARD_LEAF: u16 = 451;          // type_keys
pub const NESTED_ARTBOARD_LAYOUT: u16 = 452;        // type_keys

pub const NESTED_REMAP_TIME: u16 = 202;             // property_keys
pub const NESTED_INPUT_INPUT_ID: u16 = 237;          // property_keys (StateMachineNestedInput.inputId)
pub const NESTED_BOOL_VALUE: u16 = 238;              // property_keys (nestedValue, backing: UInt/bool)
pub const NESTED_NUMBER_VALUE: u16 = 239;            // property_keys (nestedValue, backing: Float)
```

Property 400 (`NESTED_INPUT_ID`) already exists in core.rs.

---

## Property backing types

| Property Key | Name | Backing Type | Notes |
|-------------|------|--------------|-------|
| 202 | time | Float | NestedRemapAnimation time remap |
| 237 | inputId | UInt | StateMachineNestedInput reference |
| 238 | nestedValue | UInt | NestedBool value (CoreBoolType: encode as raw byte) |
| 239 | nestedValue | Float | NestedNumber value |
| 400 | nestedInputId | UInt | Already registered |

Property 238 is a CoreBoolType -- it must encode as a single raw byte, not LEB128 varuint.

---

## Acceptance Criteria

### Type registration
- [ ] All eight type key constants added to `type_keys` in `core.rs`
- [ ] All new property key constants added to `property_keys` in `core.rs`
- [ ] `property_backing_type()` updated for keys 202, 237, 238, 239
- [ ] Property 238 listed in `is_bool_property()` (CoreBoolType encoding)

### Struct implementations
- [ ] `NestedLinearAnimation` struct with RiveObject impl (type_key=97)
- [ ] `NestedRemapAnimation` struct with RiveObject impl (type_key=98)
- [ ] `NestedInput` struct with RiveObject impl (type_key=121)
- [ ] `NestedTrigger` struct with RiveObject impl (type_key=122)
- [ ] `NestedBool` struct with RiveObject impl (type_key=123)
- [ ] `NestedNumber` struct with RiveObject impl (type_key=124)
- [ ] `NestedArtboardLeaf` struct with RiveObject impl (type_key=451)
- [ ] `NestedArtboardLayout` struct with RiveObject impl (type_key=452)
- [ ] Default-valued properties omitted (time=0.0, nestedValue=0/0.0, x=0.0, y=0.0, mix=1.0)

### Builder integration
- [ ] ObjectSpec variants added to `builder/spec.rs` for all eight types
- [ ] Existing `NestedArtboard` variant extended with `children` field
- [ ] `build_scene()` match arms handle new ObjectSpec variants
- [ ] NestedInput children resolve `nested_input_id` correctly
- [ ] Animation children resolve `animation` name to animation index
- [ ] NestedArtboardLeaf/Layout resolve `source_artboard` with same cycle-detection as NestedArtboard

### Encoding
- [ ] NestedBool nestedValue (238) encodes as raw byte (CoreBoolType), not LEB128
- [ ] All properties write with correct backing types per generated_registry
- [ ] parentId uses artboard-local indexing

### Tests
- [ ] Unit tests for each struct's type_key and properties
- [ ] Unit tests for default-value omission
- [ ] Builder test: NestedArtboard with nested inputs and animation children
- [ ] Builder test: NestedArtboardLeaf and NestedArtboardLayout resolve source correctly
- [ ] E2E test: generate a .riv with nested inputs, validate and inspect
- [ ] E2E test: NestedRemapAnimation with time remap value
