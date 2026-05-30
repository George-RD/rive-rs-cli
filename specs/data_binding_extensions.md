# Data Binding Extensions Spec

Extends the CLI's existing data binding support (ViewModel, ViewModelProperty, DataBind, ViewModelInstance, and typed ViewModelInstance subtypes) with typed ViewModelProperty subtypes, additional ViewModelInstance subtypes, DataEnum types, BindableProperty types, and DataBindPath.

## Reference

- Type keys: `src/objects/generated_registry.rs` -> `type_name()`
- Property keys: `src/objects/generated_registry.rs` -> `property_name()`
- Property backing types: `src/objects/generated_registry.rs` -> `property_backing_type_generated()`
- Existing data binding code: `src/objects/data_binding.rs`
- Existing spec types: `src/builder/spec.rs`

---

## 1. ViewModelProperty Subtypes

The CLI currently has a generic `ViewModelProperty` (type 430) with fields `name`, `parent_id`, and `property_type_value`. The runtime defines typed subtypes that inherit from ViewModelProperty. Each subtype has the same base properties (name=4, parentId=5) but uses its own type key. Some subtypes add additional properties.

### Inheritance chain (C++ runtime)

```
ViewModelComponent (name=4, parentId=5)
  -> ViewModelProperty (propertyTypeValue=875)
       -> ViewModelPropertyNumber   (431) -- no extra properties
       -> ViewModelPropertyBoolean  (448) -- no extra properties
       -> ViewModelPropertyString   (443) -- no extra properties
       -> ViewModelPropertyColor    (440) -- no extra properties
       -> ViewModelPropertyList     (434) -- viewModelReferenceId=565
       -> ViewModelPropertyViewModel(436) -- viewModelReferenceId=565
       -> ViewModelPropertyEnum     (509) -- enumId=873
       -> ViewModelPropertyEnumCustom(439) -- enumId=574
       -> ViewModelPropertyEnumSystem(511) -- enumId=873
       -> ViewModelPropertyTrigger  (502) -- no extra properties
       -> ViewModelPropertyAssetImage(585) -- no extra properties (via ViewModelPropertyAsset=584)
       -> ViewModelPropertyArtboard (598) -- artboardId=876
       -> ViewModelPropertySymbol   (563) -- symbolTypeValue=875, artboardId=876
       -> ViewModelPropertySymbolListIndex(564) -- symbolTypeValue=875, artboardId=876, listSource=874
```

### 1.1 ViewModelPropertyNumber

| Field | Description |
|-------|-------------|
| **Type key** | 431 |
| **Properties** | name (4, String), parentId (5, UInt) |
| **Notes** | No extra properties beyond base ViewModelProperty. Does NOT emit propertyTypeValue since the type key itself distinguishes it. |

### 1.2 ViewModelPropertyBoolean

| Field | Description |
|-------|-------------|
| **Type key** | 448 |
| **Properties** | name (4, String), parentId (5, UInt) |

### 1.3 ViewModelPropertyString

| Field | Description |
|-------|-------------|
| **Type key** | 443 |
| **Properties** | name (4, String), parentId (5, UInt) |

### 1.4 ViewModelPropertyColor

| Field | Description |
|-------|-------------|
| **Type key** | 440 |
| **Properties** | name (4, String), parentId (5, UInt) |

### 1.5 ViewModelPropertyList

| Field | Description |
|-------|-------------|
| **Type key** | 434 |
| **Properties** | name (4, String), parentId (5, UInt), viewModelReferenceId (565, UInt) |
| **viewModelReferenceId** | References the ViewModel type that list items conform to. |

### 1.6 ViewModelPropertyViewModel

| Field | Description |
|-------|-------------|
| **Type key** | 436 |
| **Properties** | name (4, String), parentId (5, UInt), viewModelReferenceId (565, UInt) |
| **viewModelReferenceId** | References the ViewModel type of the nested view model. |

### 1.7 ViewModelPropertyEnum

| Field | Description |
|-------|-------------|
| **Type key** | 509 |
| **Properties** | name (4, String), parentId (5, UInt), enumId (873, UInt) |
| **enumId (873)** | References a DataEnum (510) object. Generic enum property -- use ViewModelPropertyEnumCustom or ViewModelPropertyEnumSystem for specific enum sources. |

### 1.8 ViewModelPropertyEnumCustom

| Field | Description |
|-------|-------------|
| **Type key** | 439 |
| **Properties** | name (4, String), parentId (5, UInt), enumId (574, UInt) |
| **enumId (574)** | References a DataEnumCustom (438) object. Note: this uses property key 574, not 873. |

### 1.9 ViewModelPropertyEnumSystem

| Field | Description |
|-------|-------------|
| **Type key** | 511 |
| **Properties** | name (4, String), parentId (5, UInt), enumId (873, UInt) |
| **enumId (873)** | References a DataEnumSystem (512) object. |

### 1.10 ViewModelPropertyTrigger

| Field | Description |
|-------|-------------|
| **Type key** | 502 |
| **Properties** | name (4, String), parentId (5, UInt) |

### 1.11 ViewModelPropertyAssetImage

| Field | Description |
|-------|-------------|
| **Type key** | 585 |
| **Properties** | name (4, String), parentId (5, UInt) |
| **Inheritance** | Extends ViewModelPropertyAsset (584, abstract). No extra properties. |

### 1.12 ViewModelPropertyArtboard

| Field | Description |
|-------|-------------|
| **Type key** | 598 |
| **Properties** | name (4, String), parentId (5, UInt), artboardId (876, UInt) |
| **artboardId (876)** | References the target artboard. |

### 1.13 ViewModelPropertySymbol

| Field | Description |
|-------|-------------|
| **Type key** | 563 |
| **Properties** | name (4, String), parentId (5, UInt), symbolTypeValue (875, UInt), artboardId (876, UInt) |
| **symbolTypeValue (875)** | Defines the symbol type. |
| **artboardId (876)** | References the artboard source. |

### 1.14 ViewModelPropertySymbolListIndex

| Field | Description |
|-------|-------------|
| **Type key** | 564 |
| **Properties** | name (4, String), parentId (5, UInt), symbolTypeValue (875, UInt), artboardId (876, UInt), listSource (874, UInt) |
| **listSource (874)** | References the list that this symbol indexes into. |

---

## 2. ViewModelInstance Subtypes (Missing)

The CLI already implements ViewModelInstanceColor (426), ViewModelInstanceString (433), ViewModelInstanceNumber (442), ViewModelInstanceBoolean (449), ViewModelInstanceEnum (432), ViewModelInstanceList (441), ViewModelInstanceListItem (427), and ViewModelInstanceViewModel (444). The following subtypes are missing.

All ViewModelInstance subtypes inherit viewModelPropertyId (554, UInt) from ViewModelInstanceValue (428).

### 2.1 ViewModelInstanceTrigger

| Field | Description |
|-------|-------------|
| **Type key** | 501 |
| **Properties** | viewModelPropertyId (554, UInt), propertyValue (814, UInt) |
| **propertyValue (814)** | Backing type: UInt. Trigger state value. |

### 2.2 ViewModelInstanceSymbol

| Field | Description |
|-------|-------------|
| **Type key** | 565 |
| **Properties** | viewModelPropertyId (554, UInt), propertyValue (870, UInt) |
| **propertyValue (870)** | Backing type: UInt. References the symbol instance value. |

### 2.3 ViewModelInstanceSymbolListIndex

| Field | Description |
|-------|-------------|
| **Type key** | 566 |
| **Properties** | viewModelPropertyId (554, UInt), propertyValue (872, UInt) |
| **propertyValue (872)** | Backing type: UInt. Index into the symbol list. |

### 2.4 ViewModelInstanceAssetImage

| Field | Description |
|-------|-------------|
| **Type key** | 587 |
| **Properties** | viewModelPropertyId (554, UInt), propertyValue (846, UInt) |
| **propertyValue (846)** | Backing type: UInt. References the asset image. Via ViewModelInstanceAsset (586, abstract). |

### 2.5 ViewModelInstanceArtboard

| Field | Description |
|-------|-------------|
| **Type key** | 599 |
| **Properties** | viewModelPropertyId (554, UInt), propertyValue (835, UInt), artboardId (858, UInt) |
| **propertyValue (835)** | Backing type: UInt. Artboard property value. |
| **artboardId (858)** | Backing type: UInt. References the artboard. |

---

## 3. Data Enums

Data enums define enumeration types used by ViewModelPropertyEnum variants. They are children of ViewModel objects.

### 3.1 DataEnum

| Field | Description |
|-------|-------------|
| **Type key** | 510 |
| **Properties** | name (4, String), parentId (5, UInt) |
| **Notes** | Abstract base for enum definitions. Children are DataEnumValue objects. |

### 3.2 DataEnumCustom

| Field | Description |
|-------|-------------|
| **Type key** | 438 |
| **Properties** | name (572, String), parentId (5, UInt) |
| **name property** | Uses property key 572 (not 4). Backing type: String. |
| **Notes** | User-defined enum. Children are DataEnumValue objects. |

### 3.3 DataEnumValue

| Field | Description |
|-------|-------------|
| **Type key** | 445 |
| **Properties** | key (578, String), value (579, String) |
| **key (578)** | Backing type: String. The enum value identifier. |
| **value (579)** | Backing type: String. The enum value display text. |
| **Notes** | Individual enum option. Child of DataEnumCustom or DataEnumSystem. |

### 3.4 DataEnumSystem

| Field | Description |
|-------|-------------|
| **Type key** | 512 |
| **Properties** | name (4, String), parentId (5, UInt), enumType (708, UInt) |
| **enumType (708)** | Backing type: UInt. Identifies the system-provided enum type. |
| **Notes** | System-defined enum (e.g., built-in enum types like blend modes). |

---

## 4. Bindable Properties

BindableProperty types define the property interface for data binding targets. They are leaf objects with no children. All inherit from BindableProperty (type 9, abstract). They carry no user-facing properties -- they serve as typed markers that the runtime uses to resolve data bind targets.

### 4.1 BindablePropertyString

| Field | Description |
|-------|-------------|
| **Type key** | 471 |
| **Properties** | propertyValue (635, String) |
| **propertyValue (635)** | Backing type: String. |

### 4.2 BindablePropertyBoolean

| Field | Description |
|-------|-------------|
| **Type key** | 472 |
| **Properties** | propertyValue (634, UInt) |
| **propertyValue (634)** | Backing type: UInt. Boolean stored as uint. |

### 4.3 BindablePropertyNumber

| Field | Description |
|-------|-------------|
| **Type key** | 473 |
| **Properties** | propertyValue (636, Float) |
| **propertyValue (636)** | Backing type: Float. |

### 4.4 BindablePropertyEnum

| Field | Description |
|-------|-------------|
| **Type key** | 474 |
| **Properties** | propertyValue (637, UInt) |
| **propertyValue (637)** | Backing type: UInt. Enum index value. |

### 4.5 BindablePropertyColor

| Field | Description |
|-------|-------------|
| **Type key** | 475 |
| **Properties** | propertyValue (638, Color) |
| **propertyValue (638)** | Backing type: Color. ARGB uint32. |

### 4.6 BindablePropertyTrigger

| Field | Description |
|-------|-------------|
| **Type key** | 503 |
| **Properties** | propertyValue (686, UInt) |
| **propertyValue (686)** | Backing type: UInt. |

### 4.7 BindablePropertyInteger

| Field | Description |
|-------|-------------|
| **Type key** | 567 |
| **Properties** | propertyValue (823, UInt) |
| **propertyValue (823)** | Backing type: UInt. Integer value. |

### 4.8 BindablePropertyList

| Field | Description |
|-------|-------------|
| **Type key** | 590 |
| **Properties** | propertyValue (824, UInt) |
| **propertyValue (824)** | Backing type: UInt. List reference. |

### 4.9 BindablePropertyId

| Field | Description |
|-------|-------------|
| **Type key** | 596 |
| **Properties** | propertyValue (836, Color) |
| **propertyValue (836)** | Backing type: Color. ID stored as color value. |

### 4.10 BindablePropertyArtboard

| Field | Description |
|-------|-------------|
| **Type key** | 597 |
| **Properties** | propertyValue (687, UInt) |
| **propertyValue (687)** | Backing type: UInt. Artboard reference. |

---

## 5. DataBindPath

DataBindPath extends the existing DataBind (446) system to support property paths for nested data binding resolution.

### 5.1 DataBindPath

| Field | Description |
|-------|-------------|
| **Type key** | 643 |
| **Properties** | propertyKey (586, UInt), flags (587, UInt), converterId (660, UInt), pathId (TBD -- verify against C++ headers) |
| **Inheritance** | Extends DataBind. Inherits propertyKey, flags, converterId. |
| **Notes** | Used for binding through nested ViewModel paths. Verify additional properties against `data_bind_path_base.hpp` before implementing. |

---

## 6. Implementation Plan

### 6.1 New type_keys constants needed in `core.rs`

```
VIEW_MODEL_PROPERTY_NUMBER = 431
VIEW_MODEL_PROPERTY_BOOLEAN = 448
VIEW_MODEL_PROPERTY_STRING = 443
VIEW_MODEL_PROPERTY_COLOR = 440
VIEW_MODEL_PROPERTY_LIST = 434
VIEW_MODEL_PROPERTY_VIEW_MODEL = 436
VIEW_MODEL_PROPERTY_ENUM = 509
VIEW_MODEL_PROPERTY_ENUM_CUSTOM = 439
VIEW_MODEL_PROPERTY_ENUM_SYSTEM = 511
VIEW_MODEL_PROPERTY_TRIGGER = 502
VIEW_MODEL_PROPERTY_ASSET_IMAGE = 585
VIEW_MODEL_PROPERTY_ARTBOARD = 598
VIEW_MODEL_PROPERTY_SYMBOL = 563
VIEW_MODEL_PROPERTY_SYMBOL_LIST_INDEX = 564
VIEW_MODEL_INSTANCE_TRIGGER = 501
VIEW_MODEL_INSTANCE_SYMBOL = 565
VIEW_MODEL_INSTANCE_SYMBOL_LIST_INDEX = 566
VIEW_MODEL_INSTANCE_ASSET_IMAGE = 587
VIEW_MODEL_INSTANCE_ARTBOARD = 599
DATA_ENUM = 510
DATA_ENUM_CUSTOM = 438
DATA_ENUM_VALUE = 445
DATA_ENUM_SYSTEM = 512
BINDABLE_PROPERTY_STRING = 471
BINDABLE_PROPERTY_BOOLEAN = 472
BINDABLE_PROPERTY_NUMBER = 473
BINDABLE_PROPERTY_ENUM = 474
BINDABLE_PROPERTY_COLOR = 475
BINDABLE_PROPERTY_TRIGGER = 503
BINDABLE_PROPERTY_INTEGER = 567
BINDABLE_PROPERTY_LIST = 590
BINDABLE_PROPERTY_ID = 596
BINDABLE_PROPERTY_ARTBOARD = 597
DATA_BIND_PATH = 643
```

### 6.2 New property_keys constants needed in `core.rs`

```
VIEW_MODEL_REFERENCE_ID = 565           (UInt) -- used by ViewModelPropertyList, ViewModelPropertyViewModel
DATA_ENUM_CUSTOM_NAME = 572             (String) -- used by DataEnumCustom
DATA_ENUM_CUSTOM_ENUM_ID = 574          (UInt) -- used by ViewModelPropertyEnumCustom
DATA_ENUM_VALUE_KEY = 578               (String) -- used by DataEnumValue
DATA_ENUM_VALUE_VALUE = 579             (String) -- used by DataEnumValue
DATA_ENUM_ENUM_TYPE = 708               (UInt) -- used by DataEnumSystem
VIEW_MODEL_INSTANCE_TRIGGER_PROPERTY_VALUE = 814  (UInt)
VIEW_MODEL_INSTANCE_ARTBOARD_PROPERTY_VALUE = 835 (UInt)
BINDABLE_PROPERTY_BOOLEAN_VALUE = 634   (UInt)
BINDABLE_PROPERTY_STRING_VALUE = 635    (String)
BINDABLE_PROPERTY_NUMBER_VALUE = 636    (Float)
BINDABLE_PROPERTY_ENUM_VALUE = 637      (UInt)
BINDABLE_PROPERTY_COLOR_VALUE = 638     (Color)
BINDABLE_PROPERTY_TRIGGER_VALUE = 686   (UInt)
BINDABLE_PROPERTY_ARTBOARD_VALUE = 687  (UInt)
BINDABLE_PROPERTY_INTEGER_VALUE = 823   (UInt)
BINDABLE_PROPERTY_LIST_VALUE = 824      (UInt)
BINDABLE_PROPERTY_ID_VALUE = 836        (Color)
VIEW_MODEL_INSTANCE_ASSET_IMAGE_PROPERTY_VALUE = 846  (UInt)
VIEW_MODEL_INSTANCE_ARTBOARD_ARTBOARD_ID = 858   (UInt)
VIEW_MODEL_INSTANCE_SYMBOL_PROPERTY_VALUE = 870   (UInt)
VIEW_MODEL_INSTANCE_SYMBOL_LIST_INDEX_PROPERTY_VALUE = 872  (UInt)
DATA_ENUM_ENUM_ID = 873                 (UInt) -- used by ViewModelPropertyEnum, ViewModelPropertyEnumSystem
LIST_SOURCE = 874                       (UInt) -- used by ViewModelPropertySymbolListIndex
SYMBOL_TYPE_VALUE = 875                 (UInt) -- used by ViewModelPropertySymbol, ViewModelPropertySymbolListIndex
VM_PROPERTY_ARTBOARD_ID = 876           (UInt) -- used by ViewModelPropertyArtboard, ViewModelPropertySymbol, ViewModelPropertySymbolListIndex
```

### 6.3 Files to modify

1. **`src/objects/core.rs`** -- Add type_keys and property_keys constants. Add new property keys to `property_backing_type()` and `is_bool_property()` if applicable.
2. **`src/objects/data_binding.rs`** -- Add struct + `RiveObject` impl for each new type. Follow existing patterns (ViewModelInstanceColor etc.).
3. **`src/builder/spec.rs`** -- Add ObjectSpec variants for JSON input support.
4. **`src/builder/scene.rs`** -- Add match arms in `append_object()` for each new ObjectSpec variant.
5. **`src/objects/mod.rs`** -- Ensure new types are re-exported.

### 6.4 Implementation patterns

**ViewModelProperty subtypes without extra properties** (Number, Boolean, String, Color, Trigger, AssetImage):
```rust
pub struct ViewModelPropertyNumber {
    pub name: String,
    pub parent_id: u64,
}

impl RiveObject for ViewModelPropertyNumber {
    fn type_key(&self) -> u16 { type_keys::VIEW_MODEL_PROPERTY_NUMBER }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property { key: property_keys::COMPONENT_NAME, value: PropertyValue::String(self.name.clone()) },
            Property { key: property_keys::COMPONENT_PARENT_ID, value: PropertyValue::UInt(self.parent_id) },
        ]
    }
}
```

**ViewModelProperty subtypes with reference ID** (List, ViewModel):
```rust
pub struct ViewModelPropertyList {
    pub name: String,
    pub parent_id: u64,
    pub view_model_reference_id: u64,
}

impl RiveObject for ViewModelPropertyList {
    fn type_key(&self) -> u16 { type_keys::VIEW_MODEL_PROPERTY_LIST }
    fn properties(&self) -> Vec<Property> {
        let mut props = vec![
            Property { key: property_keys::COMPONENT_NAME, value: PropertyValue::String(self.name.clone()) },
            Property { key: property_keys::COMPONENT_PARENT_ID, value: PropertyValue::UInt(self.parent_id) },
        ];
        if self.view_model_reference_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_REFERENCE_ID,
                value: PropertyValue::UInt(self.view_model_reference_id),
            });
        }
        props
    }
}
```

**ViewModelInstance subtypes** (follow existing ViewModelInstanceEnum pattern):
```rust
pub struct ViewModelInstanceTrigger {
    pub view_model_property_id: u64,
    pub property_value: u64,
}

impl RiveObject for ViewModelInstanceTrigger {
    fn type_key(&self) -> u16 { type_keys::VIEW_MODEL_INSTANCE_TRIGGER }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.view_model_property_id != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_VALUE_VIEW_MODEL_PROPERTY_ID,
                value: PropertyValue::UInt(self.view_model_property_id),
            });
        }
        if self.property_value != 0 {
            props.push(Property {
                key: property_keys::VIEW_MODEL_INSTANCE_TRIGGER_PROPERTY_VALUE,
                value: PropertyValue::UInt(self.property_value),
            });
        }
        props
    }
}
```

**BindableProperty types** (typed markers):
```rust
pub struct BindablePropertyNumber {
    pub property_value: f32,
}

impl RiveObject for BindablePropertyNumber {
    fn type_key(&self) -> u16 { type_keys::BINDABLE_PROPERTY_NUMBER }
    fn properties(&self) -> Vec<Property> {
        vec![Property {
            key: property_keys::BINDABLE_PROPERTY_NUMBER_VALUE,
            value: PropertyValue::Float(self.property_value),
        }]
    }
}
```

**DataEnumValue** (key-value pair):
```rust
pub struct DataEnumValue {
    pub key: String,
    pub value: String,
}

impl RiveObject for DataEnumValue {
    fn type_key(&self) -> u16 { type_keys::DATA_ENUM_VALUE }
    fn properties(&self) -> Vec<Property> {
        vec![
            Property { key: property_keys::DATA_ENUM_VALUE_KEY, value: PropertyValue::String(self.key.clone()) },
            Property { key: property_keys::DATA_ENUM_VALUE_VALUE, value: PropertyValue::String(self.value.clone()) },
        ]
    }
}
```

### 6.5 JSON schema additions (ObjectSpec variants)

```
ViewModelPropertyNumber { name, children? }
ViewModelPropertyBoolean { name, children? }
ViewModelPropertyString { name, children? }
ViewModelPropertyColor { name, children? }
ViewModelPropertyList { name, view_model_reference_id?, children? }
ViewModelPropertyViewModel { name, view_model_reference_id?, children? }
ViewModelPropertyEnum { name, enum_id?, children? }
ViewModelPropertyEnumCustom { name, enum_id?, children? }
ViewModelPropertyEnumSystem { name, enum_id?, children? }
ViewModelPropertyTrigger { name, children? }
ViewModelPropertyAssetImage { name, children? }
ViewModelPropertyArtboard { name, artboard_id?, children? }
ViewModelPropertySymbol { name, symbol_type_value?, artboard_id?, children? }
ViewModelPropertySymbolListIndex { name, symbol_type_value?, artboard_id?, list_source?, children? }

ViewModelInstanceTrigger { view_model_property_id?, value? }
ViewModelInstanceSymbol { view_model_property_id?, value? }
ViewModelInstanceSymbolListIndex { view_model_property_id?, value? }
ViewModelInstanceAssetImage { view_model_property_id?, value? }
ViewModelInstanceArtboard { view_model_property_id?, value?, artboard_id? }

DataEnum { name, children? }
DataEnumCustom { name, children? }
DataEnumValue { key, value }
DataEnumSystem { name, enum_type? }

BindablePropertyString { value? }
BindablePropertyBoolean { value? }
BindablePropertyNumber { value? }
BindablePropertyEnum { value? }
BindablePropertyColor { value? }
BindablePropertyTrigger { value? }
BindablePropertyInteger { value? }
BindablePropertyList { value? }
BindablePropertyId { value? }
BindablePropertyArtboard { value? }

DataBindPath { property_key, flags, converter_id? }
```

### 6.6 Verification checklist

Before shipping, verify each type by:

1. Cross-reference type keys against `generated_registry.rs` -> `type_name()`
2. Cross-reference property keys against `generated_registry.rs` -> `property_name()` and `property_backing_type_generated()`
3. Verify property keys against C++ `*_base.hpp` headers in `rive-runtime/include/rive/generated/`
4. Generate a test .riv with each new type, validate with `cargo run -- validate`, and inspect with `cargo run -- inspect`
5. Load in Rive editor or WASM runtime to confirm no InvalidObject errors
