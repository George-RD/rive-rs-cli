# New Text Types

Spec for text-related types not yet implemented in `src/objects/text.rs`.

## Existing Text Types (already implemented)

| Type | typeKey | File | Notes |
|------|---------|------|-------|
| Text | 134 | text.rs | Text container node |
| TextValueRun | 135 | text.rs | A run of text with a style reference |
| TextStyle | 573 | text.rs | Font/style definition for text runs |
| TextModifierRange | 158 | text.rs | Range selection for text modifiers |
| TextModifierGroup | 159 | text.rs | Group of text modifications |
| TextVariationModifier | 162 | text.rs | Font variation axis modifier |
| TextStyleFeature | 164 | text.rs | OpenType feature on a style |

## Hierarchy Context

```
Text (134) — root text component
  ├── TextValueRun (135) — runs of text content
  │   └── references a TextStyle by styleId
  ├── TextModifierGroup (159) — modifier groups
  │   ├── TextModifierRange (158) — range definitions
  │   └── TextVariationModifier (162) — variation modifiers
  ├── TextTargetModifier (546) — target modifier (NEW)
  └── TextFollowPathModifier (547) — follow-path modifier (NEW)

TextStyle (573) — style definition
  ├── TextStylePaint (137) — paint on style (NEW)
  ├── TextStyleAxis (144) — font variation axis (NEW)
  └── TextStyleFeature (164) — OpenType features

TextInput (569) — interactive text input (NEW)
  ├── TextInputDrawable (570) — drawable container (NEW)
  │   ├── TextInputText (572) — text content area (NEW)
  │   ├── TextInputCursor (571) — blinking cursor (NEW)
  │   ├── TextInputSelection (574) — selection highlight (NEW)
  │   └── TextInputSelectedText (575) — selected text rendering (NEW)
```

## New Types

---

### TextStylePaint

**typeKey: 137**

A paint (fill or stroke) applied to a TextStyle. This is how text color/gradient is defined — it is a child of TextStyle in the object hierarchy.

#### Inheritance

```
Component (name=4, parentId=5)
  -> TextStylePaint
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component (typically empty for paints) |
| parentId | 5 | UInt | - | Reference to parent TextStyle (artboard-local index) |

#### Implementation Notes

- TextStylePaint itself has NO unique properties — it serves as a container type.
- The actual paint properties (color, gradient, etc.) come from child objects: SolidColor (typeKey 18), LinearGradient (typeKey 22), RadialGradient (typeKey 17), Fill (typeKey 20), or Stroke (typeKey 24).
- A typical hierarchy is: TextStyle -> TextStylePaint -> Fill -> SolidColor.
- The parentId MUST reference a TextStyle object.
- Name is often omitted (empty string) for paint children.

#### Struct Skeleton

```rust
pub struct TextStylePaint {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for TextStylePaint {
    fn type_key(&self) -> u16 { 137 }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property { key: 4, value: PropertyValue::String(self.name.clone()) },
            Property { key: 5, value: PropertyValue::UInt(self.parent_id) },
        ]
    }
}
```

---

### TextStyleAxis

**typeKey: 144**

A font variation axis value applied to a TextStyle. Defines variable font axis settings (e.g., weight, width, slant) at the style level.

#### Inheritance

```
Component (parentId=5)
  -> TextStyleAxis
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| parentId | 5 | UInt | - | Reference to parent TextStyle |
| tag | 289 | UInt | 0 | 4-byte font variation axis tag (e.g., 0x77676874 = "wght") |
| axisValue | 288 | Float | 0.0 | The axis value (e.g., 700.0 for bold weight) |

#### Implementation Notes

- Similar to TextStyleFeature but for variable font axes instead of OpenType features.
- The `tag` (289) is a 4-byte uint32 representing the OpenType axis tag (e.g., `wght`, `wdth`, `ital`, `slnt`).
- `axisValue` (288) is a float representing the axis value.
- No name property emitted — just parentId + tag + value.
- Omit properties at default values (tag=0, axisValue=0.0).

#### Struct Skeleton

```rust
pub struct TextStyleAxis {
    pub parent_id: u64,
    pub tag: u64,         // tag (289), default 0
    pub axis_value: f32,  // axisValue (288), default 0.0
}

impl RiveObject for TextStyleAxis {
    fn type_key(&self) -> u16 { 144 }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property { key: 5, value: PropertyValue::UInt(self.parent_id) },
        ];
        if self.tag != 0 {
            props.push(Property { key: 289, value: PropertyValue::UInt(self.tag) });
        }
        if self.axis_value != 0.0 {
            props.push(Property { key: 288, value: PropertyValue::Float(self.axis_value) });
        }
        props
    }
}
```

---

### TextTargetModifier

**typeKey: 546**

A modifier that targets specific text elements. Child of a Text node, it applies modifications to targeted text runs or ranges.

#### Inheritance

```
Component (name=4, parentId=5)
  -> TextTargetModifier
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Reference to parent Text node |
| targetId | 778 | UInt | 0 | Target element to modify |

#### Implementation Notes

- The `targetId` (778) references the target text element by artboard-local index.
- Omit targetId when 0 (default).
- This is a concrete modifier type that directs text modifications to a specific target.

#### Struct Skeleton

```rust
pub struct TextTargetModifier {
    pub name: String,
    pub parent_id: u64,
    pub target_id: u64,  // targetId (778), default 0
}

impl RiveObject for TextTargetModifier {
    fn type_key(&self) -> u16 { 546 }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property { key: 4, value: PropertyValue::String(self.name.clone()) },
            Property { key: 5, value: PropertyValue::UInt(self.parent_id) },
        ];
        if self.target_id != 0 {
            props.push(Property { key: 778, value: PropertyValue::UInt(self.target_id) });
        }
        props
    }
}
```

---

### TextFollowPathModifier

**typeKey: 547**

Makes text follow a path shape. The text is laid out along a referenced path instead of in a straight line.

#### Inheritance

```
Component (name=4, parentId=5)
  -> TextTargetModifier (targetId=778)
    -> TextFollowPathModifier
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Reference to parent Text node |
| targetId | 778 | UInt | 0 | Inherited from TextTargetModifier — the path to follow |
| orient | 782 | UInt | 0 | Bool — orient text glyphs to path tangent |
| start | 783 | Float | 0.0 | Start position along path (0.0-1.0) |
| end | 784 | Float | 1.0 | End position along path (0.0-1.0) |
| strength | 785 | Float | 1.0 | Strength of the follow-path effect |
| offset | 786 | Float | 0.0 | Offset along the path |

#### Implementation Notes

- Extends TextTargetModifier — the `targetId` (778) points to the Path shape the text follows.
- `orient` (782) is a CoreBoolType — encode as single raw byte.
- `start`/`end` define the parametric range of the path used.
- Omit all properties at their defaults.

#### Struct Skeleton

```rust
pub struct TextFollowPathModifier {
    pub name: String,
    pub parent_id: u64,
    pub target_id: u64,  // 778, default 0
    pub orient: bool,    // 782, default false
    pub start: f32,      // 783, default 0.0
    pub end: f32,        // 784, default 1.0
    pub strength: f32,   // 785, default 1.0
    pub offset: f32,     // 786, default 0.0
}
```

---

### TextInput

**typeKey: 569**

An interactive text input field. Root component for text input functionality — supports user typing, selection, and cursor display.

#### Inheritance

```
Component (name=4, parentId=5)
  -> Node (x=13, y=14)
    -> TransformComponent (rotation=15, scaleX=16, scaleY=17, opacity=18)
      -> Text (alignValue=281, sizingValue=284, etc.)
        -> TextInput
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Inherited from Component |
| alignValue | 281 | UInt | 0 | Inherited from Text — text alignment |
| sizingValue | 284 | UInt | 0 | Inherited from Text — sizing mode |
| overflowValue | 287 | UInt | 0 | Inherited from Text — overflow behavior |
| width | 285 | Float | 0.0 | Inherited from Text — input field width |
| height | 286 | Float | 0.0 | Inherited from Text — input field height |
| text | 817 | String | "" | Input text content (the current value) |
| selectionRadius | 818 | Float | 0.0 | Radius for selection hit detection |
| interactive | 891 | UInt | 0 | Bool — whether input accepts user interaction |

#### Implementation Notes

- TextInput extends Text — it inherits all Text properties.
- `text` (817) is a String — the current text content of the input field. This is distinct from TextValueRun's `text` (268).
- `interactive` (891) is a CoreBoolType — encode as single raw byte.
- `selectionRadius` (818) is a Float for cursor/selection hit testing.
- TextInput children include TextInputDrawable, TextInputText, TextInputCursor, TextInputSelection, TextInputSelectedText.
- Omit properties at default values.

#### Struct Skeleton

```rust
pub struct TextInput {
    pub name: String,
    pub parent_id: u64,
    pub align_value: u64,      // 281, default 0
    pub sizing_value: u64,     // 284, default 0
    pub overflow_value: u64,   // 287, default 0
    pub width: f32,            // 285, default 0.0
    pub height: f32,           // 286, default 0.0
    pub text: String,          // 817, default ""
    pub selection_radius: f32, // 818, default 0.0
    pub interactive: bool,     // 891, default false
}
```

---

### TextInputDrawable

**typeKey: 570**

Drawable container for a TextInput. Manages the rendering of the input's visual elements (text, cursor, selection).

#### Inheritance

```
Component (name=4, parentId=5)
  -> TextInputDrawable
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Reference to parent TextInput |

#### Implementation Notes

- TextInputDrawable is a container — it has no unique properties.
- It is a child of TextInput and parent to TextInputText, TextInputCursor, TextInputSelection, and TextInputSelectedText.
- Serves as the drawable root for the text input's visual hierarchy.

#### Struct Skeleton

```rust
pub struct TextInputDrawable {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for TextInputDrawable {
    fn type_key(&self) -> u16 { 570 }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property { key: 4, value: PropertyValue::String(self.name.clone()) },
            Property { key: 5, value: PropertyValue::UInt(self.parent_id) },
        ]
    }
}
```

---

### TextInputCursor

**typeKey: 571**

The blinking cursor in a text input field. Represents the insertion point.

#### Inheritance

```
Component (name=4, parentId=5)
  -> TextInputCursor
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Reference to parent TextInputDrawable |

#### Implementation Notes

- TextInputCursor has no unique properties — its position is computed at runtime from the text state.
- The cursor appearance (color, width) is controlled by child ShapePaint objects.
- Parent must be a TextInputDrawable.

#### Struct Skeleton

```rust
pub struct TextInputCursor {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for TextInputCursor {
    fn type_key(&self) -> u16 { 571 }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property { key: 4, value: PropertyValue::String(self.name.clone()) },
            Property { key: 5, value: PropertyValue::UInt(self.parent_id) },
        ]
    }
}
```

---

### TextInputText

**typeKey: 572**

The visible text content area within a text input. Renders the text the user has typed.

#### Inheritance

```
Component (name=4, parentId=5)
  -> TextInputText
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Reference to parent TextInputDrawable |

#### Implementation Notes

- TextInputText is the drawable that renders the actual text content.
- No unique properties — it reads from the parent TextInput's text state at runtime.
- Parent must be a TextInputDrawable.

#### Struct Skeleton

```rust
pub struct TextInputText {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for TextInputText {
    fn type_key(&self) -> u16 { 572 }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property { key: 4, value: PropertyValue::String(self.name.clone()) },
            Property { key: 5, value: PropertyValue::UInt(self.parent_id) },
        ]
    }
}
```

---

### TextInputSelection

**typeKey: 574**

The selection highlight region in a text input. Renders the background highlight behind selected text.

#### Inheritance

```
Component (name=4, parentId=5)
  -> TextInputSelection
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Reference to parent TextInputDrawable |

#### Implementation Notes

- TextInputSelection has no unique properties — selection range is computed at runtime.
- The selection color is controlled by child ShapePaint objects (typically a Fill with SolidColor).
- Parent must be a TextInputDrawable.

#### Struct Skeleton

```rust
pub struct TextInputSelection {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for TextInputSelection {
    fn type_key(&self) -> u16 { 574 }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property { key: 4, value: PropertyValue::String(self.name.clone()) },
            Property { key: 5, value: PropertyValue::UInt(self.parent_id) },
        ]
    }
}
```

---

### TextInputSelectedText

**typeKey: 575**

Renders the selected text within a text input with distinct styling (e.g., white text on blue background).

#### Inheritance

```
Component (name=4, parentId=5)
  -> TextInputSelectedText
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Reference to parent TextInputDrawable |

#### Implementation Notes

- TextInputSelectedText has no unique properties — it renders the same text content as TextInputText but only for the selected range, with alternate styling.
- The selected text color is controlled by child ShapePaint objects.
- Parent must be a TextInputDrawable.

#### Struct Skeleton

```rust
pub struct TextInputSelectedText {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for TextInputSelectedText {
    fn type_key(&self) -> u16 { 575 }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property { key: 4, value: PropertyValue::String(self.name.clone()) },
            Property { key: 5, value: PropertyValue::UInt(self.parent_id) },
        ]
    }
}
```

---

## Property Key Summary (new keys not in core.rs)

| Constant Name | Key | Backing | Used By |
|---------------|-----|---------|---------|
| TEXT_STYLE_AXIS_AXIS_VALUE | 288 | Float | TextStyleAxis |
| TEXT_STYLE_AXIS_TAG | 289 | UInt | TextStyleAxis |
| TEXT_TARGET_MODIFIER_TARGET_ID | 778 | UInt | TextTargetModifier, TextFollowPathModifier |
| TEXT_FOLLOW_PATH_MODIFIER_ORIENT | 782 | UInt (bool) | TextFollowPathModifier |
| TEXT_FOLLOW_PATH_MODIFIER_START | 783 | Float | TextFollowPathModifier |
| TEXT_FOLLOW_PATH_MODIFIER_END | 784 | Float | TextFollowPathModifier |
| TEXT_FOLLOW_PATH_MODIFIER_STRENGTH | 785 | Float | TextFollowPathModifier |
| TEXT_FOLLOW_PATH_MODIFIER_OFFSET | 786 | Float | TextFollowPathModifier |
| TEXT_INPUT_TEXT | 817 | String | TextInput |
| TEXT_INPUT_SELECTION_RADIUS | 818 | Float | TextInput |
| TEXT_INPUT_INTERACTIVE | 891 | UInt (bool) | TextInput |

## Type Key Summary

| Constant Name | Key |
|---------------|-----|
| TEXT_STYLE_PAINT | 137 |
| TEXT_STYLE_AXIS | 144 |
| TEXT_TARGET_MODIFIER | 546 |
| TEXT_FOLLOW_PATH_MODIFIER | 547 |
| TEXT_INPUT | 569 |
| TEXT_INPUT_DRAWABLE | 570 |
| TEXT_INPUT_CURSOR | 571 |
| TEXT_INPUT_TEXT | 572 |
| TEXT_INPUT_SELECTION | 574 |
| TEXT_INPUT_SELECTED_TEXT | 575 |

## Bool Properties Requiring is_bool_property() Registration

The following new property keys are CoreBoolType and must be added to `is_bool_property()` in `core.rs`:

- 782 (TEXT_FOLLOW_PATH_MODIFIER_ORIENT)
- 891 (TEXT_INPUT_INTERACTIVE)

## Implementation Checklist

1. Add type_key constants to `core.rs` -> `type_keys` module
2. Add property_key constants (288, 289, 778, 782, 783, 784, 785, 786, 817, 818, 891) to `core.rs` -> `property_keys` module
3. Update `is_bool_property()` in `core.rs` for keys 782 and 891
4. Update `property_backing_type()` in `core.rs` for all new property keys (cross-reference with generated_registry.rs backing types)
5. Add structs + `RiveObject` impls to `text.rs`
6. Add unit tests following existing patterns
7. Add to builder in `scene.rs` if JSON-constructable

## Complete TextInput Object Tree Example

For a functional TextInput, the minimal object tree is:

```
TextInput (569)           // parentId -> Artboard
  TextInputDrawable (570) // parentId -> TextInput
    TextInputText (572)   // parentId -> TextInputDrawable
    TextInputCursor (571) // parentId -> TextInputDrawable
    TextInputSelection (574)    // parentId -> TextInputDrawable
    TextInputSelectedText (575) // parentId -> TextInputDrawable
  TextValueRun (135)      // parentId -> TextInput, with text content
    TextStyle (573)       // referenced by styleId
      TextStylePaint (137) // parentId -> TextStyle
        Fill (20)
          SolidColor (18)
```
