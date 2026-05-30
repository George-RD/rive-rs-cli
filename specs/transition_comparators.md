# New Transition Comparator Types

Spec for transition comparator types not yet implemented in `src/objects/state_machine.rs`.

## Existing Comparators (already implemented)

| Type | typeKey | Properties | Notes |
|------|---------|------------|-------|
| TransitionCondition | 476 | (none) | Abstract base |
| TransitionPropertyComparator | 478 | (none) | Base property comparator |
| TransitionViewModelCondition | 482 | opValue (650, UInt) | ViewModel condition |
| TransitionValueBooleanComparator | 481 | value (647, UInt/bool) | Boolean value comparison |
| TransitionValueColorComparator | 483 | value (651, Color) | Color value comparison |
| TransitionValueNumberComparator | 484 | value (652, Float) | Number value comparison |
| TransitionValueEnumComparator | 485 | (none) | Enum value comparison |
| TransitionValueStringComparator | 486 | value (654, String) | String value comparison |
| TransitionValueTriggerComparator | 505 | value (689, UInt) | Trigger value comparison |

## Hierarchy Context

Transition comparators are children of a TransitionViewModelCondition in the object tree. They define how to compare a ViewModel property's value for state transitions.

```
StateTransition (typeKey 65)
  -> TransitionViewModelCondition (typeKey 482)
    -> TransitionPropertyComparator (typeKey 478)    // identifies which property
    -> TransitionValue*Comparator (typeKey 481-486)  // compares the value
```

The new comparator types extend this system for artboard properties, self-references, and asset comparisons.

## New Types

---

### TransitionPropertyViewModelComparator

**typeKey: 479**

Compares a property that is itself a ViewModel reference. Used when the ViewModel property being compared is a nested ViewModel (not a primitive value).

#### Inheritance

```
TransitionComparator (typeKey 477)
  -> TransitionPropertyComparator (typeKey 478)
    -> TransitionPropertyViewModelComparator
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| (none own) | - | - | - | Inherits behavior from TransitionPropertyComparator |

#### Implementation Notes

- This type has no additional properties beyond what TransitionPropertyComparator provides.
- It acts as a type marker — the typeKey (479) tells the runtime to resolve the property as a ViewModel reference rather than a primitive.
- Implement as a unit struct, like TransitionPropertyComparator and TransitionValueEnumComparator.

#### Struct Skeleton

```rust
pub struct TransitionPropertyViewModelComparator;

impl RiveObject for TransitionPropertyViewModelComparator {
    fn type_key(&self) -> u16 { 479 }
    fn properties(&self) -> Vec<Property> { vec![] }
}
```

---

### TransitionPropertyArtboardComparator

**typeKey: 496**

Compares a property that references an artboard. Used when the ViewModel property being compared is an artboard reference.

#### Inheritance

```
TransitionComparator (typeKey 477)
  -> TransitionPropertyComparator (typeKey 478)
    -> TransitionPropertyArtboardComparator
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| (none own) | - | - | - | Type marker only |

#### Implementation Notes

- Unit struct, same pattern as TransitionPropertyViewModelComparator.
- The typeKey (496) tells the runtime to resolve the property comparator for artboard-typed ViewModel properties.

#### Struct Skeleton

```rust
pub struct TransitionPropertyArtboardComparator;

impl RiveObject for TransitionPropertyArtboardComparator {
    fn type_key(&self) -> u16 { 496 }
    fn properties(&self) -> Vec<Property> { vec![] }
}
```

---

### TransitionArtboardCondition

**typeKey: 497**

A condition that evaluates artboard-level state for transitions. Analogous to TransitionViewModelCondition but for artboard properties rather than ViewModel properties.

#### Inheritance

```
TransitionCondition (typeKey 476)
  -> TransitionArtboardCondition
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| opValue | 650 | UInt | 0 | Comparison operator (same semantics as TransitionViewModelCondition) |

#### Implementation Notes

- Same property (`opValue` at 650) as TransitionViewModelCondition, but with a different typeKey.
- This conditions on artboard-level properties instead of ViewModel properties.
- Omit opValue when 0 (default).

#### Struct Skeleton

```rust
pub struct TransitionArtboardCondition {
    pub op_value: u64, // opValue (650), default 0
}

impl RiveObject for TransitionArtboardCondition {
    fn type_key(&self) -> u16 { 497 }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.op_value != 0 {
            props.push(Property {
                key: 650, // TRANSITION_VIEW_MODEL_CONDITION_OP_VALUE
                value: PropertyValue::UInt(self.op_value),
            });
        }
        props
    }
}
```

---

### TransitionSelfComparator

**typeKey: 593**

Compares a ViewModel property against itself (self-comparison). Used for "has changed" style conditions where the comparison target is the property's own previous value.

#### Inheritance

```
TransitionComparator (typeKey 477)
  -> TransitionPropertyComparator (typeKey 478)
    -> TransitionSelfComparator
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| (none own) | - | - | - | Type marker only |

#### Implementation Notes

- Unit struct — the typeKey alone communicates "compare property to itself."
- This enables "value changed" transitions: the runtime detects when a property's current value differs from its previous value.

#### Struct Skeleton

```rust
pub struct TransitionSelfComparator;

impl RiveObject for TransitionSelfComparator {
    fn type_key(&self) -> u16 { 593 }
    fn properties(&self) -> Vec<Property> { vec![] }
}
```

---

### TransitionValueIdComparator

**typeKey: 601**

Compares a ViewModel property by its ID value. Used for ViewModel properties that reference entities by ID (e.g., list items, symbols).

#### Inheritance

```
TransitionComparator (typeKey 477)
  -> TransitionValueComparator (typeKey 480)
    -> TransitionValueIdComparator
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| propertyValue | 823 | UInt | 0 | The ID value to compare against |

#### Implementation Notes

- Has a single `propertyValue` (key 823) of type UInt — the ID being compared.
- Omit when value is 0 (default).

#### Struct Skeleton

```rust
pub struct TransitionValueIdComparator {
    pub value: u64, // propertyValue (823), default 0
}

impl RiveObject for TransitionValueIdComparator {
    fn type_key(&self) -> u16 { 601 }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.value != 0 {
            props.push(Property {
                key: 823,
                value: PropertyValue::UInt(self.value),
            });
        }
        props
    }
}
```

---

### TransitionValueAssetComparator

**typeKey: 602**

Compares a ViewModel property that holds an asset reference. Used for ViewModel properties of type asset (e.g., image assets).

#### Inheritance

```
TransitionComparator (typeKey 477)
  -> TransitionValueComparator (typeKey 480)
    -> TransitionValueAssetComparator
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| propertyValue | 824 | UInt | 0 | The asset reference value to compare against |

#### Implementation Notes

- Has a single `propertyValue` (key 824) of type UInt — the asset reference being compared.
- Omit when value is 0 (default).

#### Struct Skeleton

```rust
pub struct TransitionValueAssetComparator {
    pub value: u64, // propertyValue (824), default 0
}

impl RiveObject for TransitionValueAssetComparator {
    fn type_key(&self) -> u16 { 602 }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.value != 0 {
            props.push(Property {
                key: 824,
                value: PropertyValue::UInt(self.value),
            });
        }
        props
    }
}
```

---

### TransitionValueArtboardComparator

**typeKey: 630**

Compares a ViewModel property that holds an artboard reference. Used for ViewModel properties of type artboard.

#### Inheritance

```
TransitionComparator (typeKey 477)
  -> TransitionValueComparator (typeKey 480)
    -> TransitionValueArtboardComparator
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| propertyValue | 870 | UInt | 0 | The artboard reference value to compare against |

#### Implementation Notes

- Has a single `propertyValue` (key 870) of type UInt — the artboard reference being compared.
- Omit when value is 0 (default).

#### Struct Skeleton

```rust
pub struct TransitionValueArtboardComparator {
    pub value: u64, // propertyValue (870), default 0
}

impl RiveObject for TransitionValueArtboardComparator {
    fn type_key(&self) -> u16 { 630 }
    fn properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if self.value != 0 {
            props.push(Property {
                key: 870,
                value: PropertyValue::UInt(self.value),
            });
        }
        props
    }
}
```

---

## Property Key Summary (new keys not in core.rs)

| Constant Name | Key | Backing | Used By |
|---------------|-----|---------|---------|
| TRANSITION_VALUE_ID_COMPARATOR_VALUE | 823 | UInt | TransitionValueIdComparator |
| TRANSITION_VALUE_ASSET_COMPARATOR_VALUE | 824 | UInt | TransitionValueAssetComparator |
| TRANSITION_VALUE_ARTBOARD_COMPARATOR_VALUE | 870 | UInt | TransitionValueArtboardComparator |

Note: `opValue` (650) is already defined as `TRANSITION_VIEW_MODEL_CONDITION_OP_VALUE` in `core.rs` and is reused by TransitionArtboardCondition.

## Type Key Summary

| Constant Name | Key |
|---------------|-----|
| TRANSITION_PROPERTY_VIEW_MODEL_COMPARATOR | 479 |
| TRANSITION_PROPERTY_ARTBOARD_COMPARATOR | 496 |
| TRANSITION_ARTBOARD_CONDITION | 497 |
| TRANSITION_SELF_COMPARATOR | 593 |
| TRANSITION_VALUE_ID_COMPARATOR | 601 |
| TRANSITION_VALUE_ASSET_COMPARATOR | 602 |
| TRANSITION_VALUE_ARTBOARD_COMPARATOR | 630 |

## Implementation Checklist

1. Add type_key constants to `core.rs` -> `type_keys` module
2. Add property_key constants for 823, 824, 870 to `core.rs` -> `property_keys` module
3. No new entries needed in `property_backing_type()` — property 650 is already covered, and 823/824/870 are in the generated registry
4. Add structs + `RiveObject` impls to `state_machine.rs`
5. Add unit tests following existing patterns (test type_key, test default props, test custom props)
6. Add to builder in `scene.rs` if JSON-constructable
