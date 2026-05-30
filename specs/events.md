# Event System Extensions Spec

## Overview

This spec covers specialized Event types and CustomProperty types that extend the
existing `Event` (typeKey 128) support. Events are children of an Artboard.
CustomProperty types are children of Events (or CustomPropertyGroup containers).
OpenUrlEvent and AudioEvent are specialized Event subtypes.

### Inheritance hierarchy (C++ runtime)

```
Component (name=4, parentId=5)
  CustomProperty (typeKey 167, abstract base)
    CustomPropertyNumber   (typeKey 127, propertyValue=243)
    CustomPropertyBoolean  (typeKey 129, propertyValue=245)
    CustomPropertyString   (typeKey 130, propertyValue=246)
    CustomPropertyColor    (typeKey 592, propertyValue=836)
    CustomPropertyTrigger  (typeKey 613, fire=869, propertyValue=870)
    CustomPropertyEnum     (typeKey 616, propertyValue=872, enumId=873)
  CustomPropertyGroup (typeKey 548, no own properties)
    Event (typeKey 128, trigger=395)
      OpenUrlEvent (typeKey 131, url=248, targetValue=249)
      AudioEvent   (typeKey 407, assetId=408)
```

Event extends CustomPropertyGroup, which means an Event can itself contain
CustomProperty children. CustomPropertyGroup extends Component.

---

## Types

### 1. OpenUrlEvent (typeKey 131)

An Event that opens a URL when triggered. Extends Event (typeKey 128).

**Inheritance chain:** Component -> CustomProperty -> CustomPropertyGroup -> Event -> OpenUrlEvent

#### Properties

| Property     | Key | Backing Type | Required | Default | Description                                              |
|-------------|-----|-------------|----------|---------|----------------------------------------------------------|
| name        | 4   | String      | yes      | -       | Inherited from Component                                 |
| parentId    | 5   | UInt        | yes      | -       | Inherited from Component (artboard-local index)          |
| url         | 248 | String      | yes      | ""      | The URL to open                                          |
| targetValue | 249 | UInt        | no       | 0       | Target window: 0 = blank/new, 1 = parent, 2 = self, 3 = top |

Note: The inherited `trigger` property (key 395) from Event is a runtime-only
callback and is not written to the binary.

#### Parent-child relationships

- **Parent:** Artboard (direct child of artboard, like Event)
- **Children:** CustomProperty types (CustomPropertyNumber, CustomPropertyBoolean,
  CustomPropertyString, CustomPropertyColor, CustomPropertyTrigger,
  CustomPropertyEnum), CustomPropertyGroup

---

### 2. AudioEvent (typeKey 407)

An Event that plays audio when triggered. Extends Event (typeKey 128).

**Inheritance chain:** Component -> CustomProperty -> CustomPropertyGroup -> Event -> AudioEvent

#### Properties

| Property  | Key | Backing Type | Required | Default | Description                                       |
|----------|-----|-------------|----------|---------|---------------------------------------------------|
| name     | 4   | String      | yes      | -       | Inherited from Component                          |
| parentId | 5   | UInt        | yes      | -       | Inherited from Component (artboard-local index)   |
| assetId  | 408 | UInt        | no       | -1 (u32::MAX) | Reference to an AudioAsset (typeKey 406) object |

The AudioEvent references an AudioAsset by its asset ID. The AudioAsset must
exist in the file's asset list for the audio to play at runtime.

#### Parent-child relationships

- **Parent:** Artboard (direct child of artboard, like Event)
- **Children:** CustomProperty types, CustomPropertyGroup

---

### 3. CustomPropertyNumber (typeKey 127)

A numeric property attached to an Event. Used to pass number data with events.

**Inheritance chain:** Component -> CustomProperty -> CustomPropertyNumber

#### Properties

| Property      | Key | Backing Type | Required | Default | Description                        |
|--------------|-----|-------------|----------|---------|-------------------------------------|
| name         | 4   | String      | yes      | -       | Inherited from Component            |
| parentId     | 5   | UInt        | yes      | -       | Inherited from Component            |
| propertyValue| 243 | Float       | no       | 0.0     | The numeric value of this property  |

#### Parent-child relationships

- **Parent:** Event, OpenUrlEvent, AudioEvent, or CustomPropertyGroup
- **Children:** None

---

### 4. CustomPropertyBoolean (typeKey 129)

A boolean property attached to an Event.

**Inheritance chain:** Component -> CustomProperty -> CustomPropertyBoolean

#### Properties

| Property      | Key | Backing Type | Required | Default | Description                         |
|--------------|-----|-------------|----------|---------|--------------------------------------|
| name         | 4   | String      | yes      | -       | Inherited from Component             |
| parentId     | 5   | UInt        | yes      | -       | Inherited from Component             |
| propertyValue| 245 | UInt        | no       | 0       | Boolean value (0 = false, 1 = true). Backing type is UInt but represents a bool. |

**Encoding note:** Despite being boolean semantically, this property uses UInt
backing (not CoreBoolType). It is encoded as a LEB128 varuint, NOT as a raw byte.
This differs from CoreBoolType properties like `isVisible` (key 41).

#### Parent-child relationships

- **Parent:** Event, OpenUrlEvent, AudioEvent, or CustomPropertyGroup
- **Children:** None

---

### 5. CustomPropertyString (typeKey 130)

A string property attached to an Event.

**Inheritance chain:** Component -> CustomProperty -> CustomPropertyString

#### Properties

| Property      | Key | Backing Type | Required | Default | Description                        |
|--------------|-----|-------------|----------|---------|-------------------------------------|
| name         | 4   | String      | yes      | -       | Inherited from Component            |
| parentId     | 5   | UInt        | yes      | -       | Inherited from Component            |
| propertyValue| 246 | String      | no       | ""      | The string value of this property   |

#### Parent-child relationships

- **Parent:** Event, OpenUrlEvent, AudioEvent, or CustomPropertyGroup
- **Children:** None

---

### 6. CustomPropertyColor (typeKey 592)

A color property attached to an Event.

**Inheritance chain:** Component -> CustomProperty -> CustomPropertyColor

#### Properties

| Property      | Key | Backing Type | Required | Default      | Description                           |
|--------------|-----|-------------|----------|--------------|----------------------------------------|
| name         | 4   | String      | yes      | -            | Inherited from Component               |
| parentId     | 5   | UInt        | yes      | -            | Inherited from Component               |
| propertyValue| 836 | Color       | no       | 0xFF000000   | ARGB color value, encoded as uint32 LE |

**Encoding note:** Color backing type encodes as 4-byte uint32 little-endian
(same as SolidColor value, GradientStop color, etc.).

#### Parent-child relationships

- **Parent:** Event, OpenUrlEvent, AudioEvent, or CustomPropertyGroup
- **Children:** None

---

### 7. CustomPropertyTrigger (typeKey 613)

A trigger property attached to an Event. Triggers have no persistent value --
they represent a one-shot action signal.

**Inheritance chain:** Component -> CustomProperty -> CustomPropertyTrigger

#### Properties

| Property      | Key | Backing Type | Required | Default | Description                                |
|--------------|-----|-------------|----------|---------|--------------------------------------------|
| name         | 4   | String      | yes      | -       | Inherited from Component                   |
| parentId     | 5   | UInt        | yes      | -       | Inherited from Component                   |
| propertyValue| 870 | UInt        | no       | 0       | Trigger property value (runtime use only)  |

Note: The `fire` property (key 869) is a runtime-only callback and is not
written to the binary file.

#### Parent-child relationships

- **Parent:** Event, OpenUrlEvent, AudioEvent, or CustomPropertyGroup
- **Children:** None

---

### 8. CustomPropertyEnum (typeKey 616)

An enum property attached to an Event. References an enum definition and stores
the selected enum value.

**Inheritance chain:** Component -> CustomProperty -> CustomPropertyEnum

#### Properties

| Property      | Key | Backing Type | Required | Default | Description                                  |
|--------------|-----|-------------|----------|---------|-----------------------------------------------|
| name         | 4   | String      | yes      | -       | Inherited from Component                      |
| parentId     | 5   | UInt        | yes      | -       | Inherited from Component                      |
| propertyValue| 872 | UInt        | no       | 0       | The selected enum value index                 |
| enumId       | 873 | UInt        | no       | 0       | Reference to the enum type definition         |

#### Parent-child relationships

- **Parent:** Event, OpenUrlEvent, AudioEvent, or CustomPropertyGroup
- **Children:** None

---

### 9. CustomPropertyGroup (typeKey 548)

A container that groups CustomProperty children. Event itself extends
CustomPropertyGroup, so this type is primarily used when you need an additional
level of grouping within an Event's custom properties.

**Inheritance chain:** Component -> CustomProperty -> CustomPropertyGroup

#### Properties

| Property  | Key | Backing Type | Required | Default | Description                        |
|----------|-----|-------------|----------|---------|-------------------------------------|
| name     | 4   | String      | yes      | -       | Inherited from Component            |
| parentId | 5   | UInt        | yes      | -       | Inherited from Component            |

CustomPropertyGroup has no own properties beyond those inherited from Component.

#### Parent-child relationships

- **Parent:** Event, OpenUrlEvent, AudioEvent, or another CustomPropertyGroup
- **Children:** CustomProperty types (Number, Boolean, String, Color, Trigger, Enum),
  other CustomPropertyGroup

---

## JSON Schema

The following ObjectSpec variants should be added to support these types. They
are children within an Artboard's `children` array (for event types) or within
an Event's `children` array (for custom property types).

```json
{
  "type": "open_url_event",
  "name": "OpenLink",
  "url": "https://rive.app",
  "target_value": 0,
  "children": [
    {
      "type": "custom_property_string",
      "name": "campaign",
      "property_value": "summer_2024"
    }
  ]
}
```

```json
{
  "type": "audio_event",
  "name": "PlaySound",
  "asset_id": 1,
  "children": [
    {
      "type": "custom_property_number",
      "name": "volume_multiplier",
      "property_value": 0.8
    }
  ]
}
```

### Full ObjectSpec variants

```jsonc
// OpenUrlEvent - extends Event with URL navigation
{
  "type": "open_url_event",
  "name": "<string, required>",
  "url": "<string, required>",
  "target_value": "<uint, optional, default 0>",
  "children": ["<CustomProperty | CustomPropertyGroup, optional>"]
}

// AudioEvent - extends Event with audio playback
{
  "type": "audio_event",
  "name": "<string, required>",
  "asset_id": "<uint, optional, references AudioAsset>",
  "children": ["<CustomProperty | CustomPropertyGroup, optional>"]
}

// CustomPropertyNumber
{
  "type": "custom_property_number",
  "name": "<string, required>",
  "property_value": "<float, optional, default 0.0>"
}

// CustomPropertyBoolean
{
  "type": "custom_property_boolean",
  "name": "<string, required>",
  "property_value": "<bool, optional, default false>"
}

// CustomPropertyString
{
  "type": "custom_property_string",
  "name": "<string, required>",
  "property_value": "<string, optional, default \"\">"
}

// CustomPropertyColor
{
  "type": "custom_property_color",
  "name": "<string, required>",
  "property_value": "<color string e.g. \"#FF0000FF\", optional>"
}

// CustomPropertyTrigger
{
  "type": "custom_property_trigger",
  "name": "<string, required>"
}

// CustomPropertyEnum
{
  "type": "custom_property_enum",
  "name": "<string, required>",
  "property_value": "<uint, optional, default 0>",
  "enum_id": "<uint, optional, default 0>"
}

// CustomPropertyGroup
{
  "type": "custom_property_group",
  "name": "<string, required>",
  "children": ["<CustomProperty | CustomPropertyGroup, optional>"]
}
```

### Complete Example

```json
{
  "scene_format_version": 1,
  "artboards": [
    {
      "name": "MainArtboard",
      "width": 500,
      "height": 500,
      "children": [
        {
          "type": "open_url_event",
          "name": "VisitWebsite",
          "url": "https://rive.app",
          "target_value": 0,
          "children": [
            {
              "type": "custom_property_string",
              "name": "source",
              "property_value": "animation"
            },
            {
              "type": "custom_property_boolean",
              "name": "is_external",
              "property_value": true
            }
          ]
        },
        {
          "type": "audio_event",
          "name": "ButtonClickSound",
          "asset_id": 42,
          "children": [
            {
              "type": "custom_property_number",
              "name": "delay_ms",
              "property_value": 100.0
            }
          ]
        },
        {
          "type": "event",
          "name": "CustomAction",
          "children": [
            {
              "type": "custom_property_group",
              "name": "metadata",
              "children": [
                {
                  "type": "custom_property_string",
                  "name": "action_type",
                  "property_value": "purchase"
                },
                {
                  "type": "custom_property_number",
                  "name": "item_id",
                  "property_value": 42.0
                },
                {
                  "type": "custom_property_color",
                  "name": "highlight_color",
                  "property_value": "#FF6600FF"
                },
                {
                  "type": "custom_property_enum",
                  "name": "priority",
                  "property_value": 1,
                  "enum_id": 5
                },
                {
                  "type": "custom_property_trigger",
                  "name": "on_confirm"
                }
              ]
            }
          ]
        }
      ],
      "animations": [],
      "state_machines": []
    }
  ]
}
```

---

## Implementation Notes

### Property keys to add to `core.rs`

```
property_keys::OPEN_URL_EVENT_URL: u16 = 248;
property_keys::OPEN_URL_EVENT_TARGET_VALUE: u16 = 249;
property_keys::AUDIO_EVENT_ASSET_ID: u16 = 408;
property_keys::CUSTOM_PROPERTY_NUMBER_PROPERTY_VALUE: u16 = 243;
property_keys::CUSTOM_PROPERTY_BOOLEAN_PROPERTY_VALUE: u16 = 245;
property_keys::CUSTOM_PROPERTY_STRING_PROPERTY_VALUE: u16 = 246;
property_keys::CUSTOM_PROPERTY_COLOR_PROPERTY_VALUE: u16 = 836;
property_keys::CUSTOM_PROPERTY_TRIGGER_PROPERTY_VALUE: u16 = 870;
property_keys::CUSTOM_PROPERTY_ENUM_PROPERTY_VALUE: u16 = 872;
property_keys::CUSTOM_PROPERTY_ENUM_ENUM_ID: u16 = 873;
```

### Type keys to add to `core.rs`

```
type_keys::OPEN_URL_EVENT: u16 = 131;
type_keys::AUDIO_EVENT: u16 = 407;
type_keys::CUSTOM_PROPERTY: u16 = 167;
type_keys::CUSTOM_PROPERTY_NUMBER: u16 = 127;
type_keys::CUSTOM_PROPERTY_BOOLEAN: u16 = 129;
type_keys::CUSTOM_PROPERTY_STRING: u16 = 130;
type_keys::CUSTOM_PROPERTY_COLOR: u16 = 592;
type_keys::CUSTOM_PROPERTY_TRIGGER: u16 = 613;
type_keys::CUSTOM_PROPERTY_ENUM: u16 = 616;
type_keys::CUSTOM_PROPERTY_GROUP: u16 = 548;
```

### Backing types for `property_backing_type()`

| Key | Name                          | Backing  |
|-----|-------------------------------|----------|
| 243 | CustomPropertyNumber value    | Float    |
| 245 | CustomPropertyBoolean value   | UInt     |
| 246 | CustomPropertyString value    | String   |
| 248 | OpenUrlEvent url              | String   |
| 249 | OpenUrlEvent targetValue      | UInt     |
| 408 | AudioEvent assetId            | UInt     |
| 836 | CustomPropertyColor value     | Color    |
| 870 | CustomPropertyTrigger value   | UInt     |
| 872 | CustomPropertyEnum value      | UInt     |
| 873 | CustomPropertyEnum enumId     | UInt     |

### Boolean encoding caveat

CustomPropertyBoolean's `propertyValue` (key 245) has UInt backing and is NOT in
the `is_bool_property()` list. It encodes as a normal LEB128 varuint (0 or 1),
not as a raw byte. This is different from runtime bool properties like
`isVisible` (key 41) which use CoreBoolType encoding.

### ParentKind extension

Add a new `ParentKind::Event` variant (or reuse the existing approach) to
validate that CustomProperty types are only placed as children of Event,
OpenUrlEvent, AudioEvent, or CustomPropertyGroup.

---

## Acceptance Criteria

### Core implementation

- [ ] Type key constants added to `core.rs` for all 9 types (OpenUrlEvent,
      AudioEvent, CustomPropertyNumber, CustomPropertyBoolean,
      CustomPropertyString, CustomPropertyColor, CustomPropertyTrigger,
      CustomPropertyEnum, CustomPropertyGroup)
- [ ] Property key constants added to `core.rs` for all new properties
      (248, 249, 408, 243, 245, 246, 836, 870, 872, 873)
- [ ] `property_backing_type()` updated for all new property keys
- [ ] Struct definitions with `RiveObject` trait implementations for all 9 types
- [ ] Property emission order follows C++ runtime convention

### Builder (spec.rs + scene.rs/objects.rs)

- [ ] `ObjectSpec` variants added for all 9 types with correct serde rename
      (`open_url_event`, `audio_event`, `custom_property_number`, etc.)
- [ ] `append_object()` match arms handle all 9 types
- [ ] OpenUrlEvent and AudioEvent support `children` for nested CustomProperty types
- [ ] CustomPropertyGroup supports `children` for nested CustomProperty types
- [ ] CustomPropertyNumber/Boolean/String/Color/Trigger/Enum are leaf nodes (no children)

### Validation

- [ ] Parent-child validation: CustomProperty types can only be children of
      Event, OpenUrlEvent, AudioEvent, or CustomPropertyGroup
- [ ] OpenUrlEvent, AudioEvent can only be children of Artboard
- [ ] CustomPropertyBoolean `property_value` accepts JSON `true`/`false` and
      converts to UInt 0/1
- [ ] CustomPropertyColor `property_value` parses hex color string to u32

### Testing

- [ ] Unit tests for each RiveObject implementation (type_key, properties output)
- [ ] E2E test: generate .riv with OpenUrlEvent containing custom properties,
      validate with `validate` command
- [ ] E2E test: generate .riv with AudioEvent referencing an AudioAsset,
      validate with `validate` command
- [ ] E2E test: generate .riv with nested CustomPropertyGroup containing
      mixed CustomProperty types
- [ ] Inspect/decompile round-trip: events and custom properties survive
      encode -> inspect -> verify cycle
- [ ] Playwright runtime test: .riv with OpenUrlEvent loads without error
      in Rive WASM runtime

### Edge cases

- [ ] OpenUrlEvent with empty URL string (should encode, runtime decides behavior)
- [ ] AudioEvent without asset_id (omit property, use default)
- [ ] CustomPropertyBoolean with integer 0/1 input (accept both bool and int)
- [ ] CustomPropertyColor with various color formats (#RGB, #RRGGBB, #RRGGBBAA)
- [ ] Event with deeply nested CustomPropertyGroup hierarchy
- [ ] Multiple events on same artboard with overlapping custom property names
