# Data Converters Spec

Data converters transform data between view model properties and bound UI elements. They are children of DataBind objects, referenced via the DataBind's `converterId` property (660, UInt). Each converter inherits from the abstract DataConverter base (type 488) which provides a `name` property (662, String).

## Reference

- Type keys: `src/objects/generated_registry.rs` -> `type_name()`
- Property keys: `src/objects/generated_registry.rs` -> `property_name()`
- Property backing types: `src/objects/generated_registry.rs` -> `property_backing_type_generated()`
- Existing data binding code: `src/objects/data_binding.rs`
- C++ base headers: `rive-runtime/include/rive/generated/data_bind/converters/`
- C++ def files: `rive-runtime/dev/defs/data_bind/converters/`

---

## Inheritance Hierarchy

```
DataConverter (488, abstract) -- name (662, String)
├── DataConverterRounder (489)       -- decimals (669, UInt)
├── DataConverterToString (490)      -- flags (764, UInt), decimals (765, UInt), colorFormat (766, String)
├── DataConverterToNumber (617)      -- (no extra properties)
├── DataConverterGroup (499)         -- (no extra properties)
├── DataConverterTrigger (504)       -- (no extra properties)
├── DataConverterBooleanNegate (535) -- (no extra properties)
├── DataConverterListToLength (591)  -- (no extra properties)
├── DataConverterStringPad (530)     -- length (743, UInt), text (744, String), padType (745, UInt)
├── DataConverterStringRemoveZeros (531) -- (no extra properties)
├── DataConverterStringTrim (532)    -- trimType (746, UInt)
├── DataConverterInterpolator (534)  -- duration (756, Float), interpolationType (757, UInt), interpolatorId (758, UInt)
├── DataConverterRangeMapper (519)   -- interpolationType (713, UInt), interpolatorId (714, UInt),
│                                       flags (715, UInt), minInput (716, Float), maxInput (717, Float),
│                                       minOutput (718, Float), maxOutput (719, Float)
├── DataConverterFormula (536)       -- randomModeValue (887, UInt)
├── DataConverterNumberToList (568)  -- viewModelId (816, UInt)
├── DataConverterOperation (516, abstract) -- operationType (682, UInt)
│   ├── DataConverterOperationValue (500)        -- operationValue (681, Float)
│   ├── DataConverterSystemDegsToRads (514)      -- (no extra properties beyond inherited operationType)
│   ├── DataConverterSystemNormalizer (515)       -- (no extra properties beyond inherited operationType/operationValue)
│   └── DataConverterOperationViewModel (517)     -- sourcePathIds (711, Bytes/encoded)
└── DataConverterGroupItem (498, not a DataConverter subclass) -- converterId (679, UInt)
```

**Note on DataConverterGroupItem**: This type does NOT extend DataConverter. It is a standalone object that references a converter via `converterId`. It has two editor-only properties (order=678, FractionalIndex, runtime: false; groupId=680, Id, runtime: false) that are not serialized in .riv files.

**Note on DataConverterOperation subtypes**: DataConverterSystemDegsToRads inherits from DataConverterOperation (gaining operationType=682), and DataConverterSystemNormalizer inherits from DataConverterOperationValue (gaining both operationType=682 and operationValue=681). Their own type keys introduce no new properties.

**Note on DataConverterOperationViewModel**: The `sourcePathIds` property (711) is a `Bytes` type (`CoreBytesType`), encoded as a list of IDs. This is not one of the standard 4 backing types (UInt/String/Float/Color) and requires special handling (similar to weight values/indices in bones).

---

## 1. DataConverterRounder

Rounds numeric values to a specified number of decimal places.

| Field | Description |
|-------|-------------|
| **Type key** | 489 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | decimals (669, UInt) |
| **decimals default** | 0 |
| **Notes** | Bindable. Rounds input number to `decimals` decimal places. |

---

## 2. DataConverterToString

Converts values to their string representation with formatting options.

| Field | Description |
|-------|-------------|
| **Type key** | 490 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | flags (764, UInt), decimals (765, UInt), colorFormat (766, String) |
| **flags default** | 0 |
| **decimals default** | 0 |
| **colorFormat default** | "" (empty string) |
| **Notes** | The `flags` property controls conversion behavior. `decimals` specifies rounding precision for numeric-to-string conversion. `colorFormat` specifies a format string for color-to-string conversion. Both `decimals` and `colorFormat` are bindable. |

---

## 3. DataConverterToNumber

Converts values (typically strings) to numeric representation.

| Field | Description |
|-------|-------------|
| **Type key** | 617 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | (none) |
| **Notes** | No additional properties. Performs type coercion to number. |

---

## 4. DataConverterGroup

Groups multiple converters into a sequential pipeline.

| Field | Description |
|-------|-------------|
| **Type key** | 499 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | (none) |
| **Notes** | Contains DataConverterGroupItem children that reference individual converters in order. The group applies them sequentially. |

---

## 5. DataConverterGroupItem

References a converter within a DataConverterGroup. This is NOT a DataConverter subclass.

| Field | Description |
|-------|-------------|
| **Type key** | 498 |
| **Extends** | (none / standalone) |
| **Own properties** | converterId (679, UInt) |
| **converterId default** | 4294967295 (0xFFFFFFFF, Core.missingId) |
| **Notes** | Child of DataConverterGroup. References a DataConverter by its artboard-local object index. The `order` (678) and `groupId` (680) properties are editor-only (runtime: false) and must NOT be serialized. |

---

## 6. DataConverterOperationValue

Applies a binary arithmetic operation between the input and a static value.

| Field | Description |
|-------|-------------|
| **Type key** | 500 |
| **Extends** | DataConverterOperation (516) -> DataConverter (488) |
| **Inherited properties** | name (662, String), operationType (682, UInt) |
| **Own properties** | operationValue (681, Float) |
| **operationType default** | 0 |
| **operationValue default** | 1.0 |
| **operationType values** | Defines the arithmetic operation (e.g., add, subtract, multiply, divide) |
| **Notes** | Bindable. The `operationValue` is the second operand; the input is the first. |

---

## 7. DataConverterTrigger

Converts any value change into a trigger event.

| Field | Description |
|-------|-------------|
| **Type key** | 504 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | (none) |
| **Notes** | Fires a trigger whenever the bound input value changes. |

---

## 8. DataConverterOperationViewModel

Applies a binary arithmetic operation between the input and a value from a view model property.

| Field | Description |
|-------|-------------|
| **Type key** | 517 |
| **Extends** | DataConverterOperation (516) -> DataConverter (488) |
| **Inherited properties** | name (662, String), operationType (682, UInt) |
| **Own properties** | sourcePathIds (711, Bytes) |
| **operationType default** | 0 |
| **sourcePathIds default** | [] (empty) |
| **Notes** | The `sourcePathIds` property is a `CoreBytesType` (encoded list of IDs defining the path to a view model property). This is NOT one of the 4 standard backing types and requires special encode/decode handling. It is similar to how weight values/indices work in the bones system. At encode time, the bytes are written as a length-prefixed byte array. |

### Encoding sourcePathIds

The `sourcePathIds` property uses `CoreBytesType`, which is serialized as:
1. Length of the byte array (varuint)
2. Raw bytes

The byte content is an encoded sequence of uint32 IDs representing the path through view model properties to reach the operand value.

---

## 9. DataConverterStringPad

Pads a string to a target length with a given fill string.

| Field | Description |
|-------|-------------|
| **Type key** | 530 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | length (743, UInt), text (744, String), padType (745, UInt) |
| **length default** | 1 |
| **text default** | "" (empty string) |
| **padType default** | 0 |
| **padType values** | 0 = pad start, 1 = pad end |
| **Notes** | All properties are bindable. Pads the input string with `text` until it reaches `length` characters. |

---

## 10. DataConverterStringRemoveZeros

Removes trailing zeros from numeric string representations.

| Field | Description |
|-------|-------------|
| **Type key** | 531 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | (none) |
| **Notes** | No configuration. Strips trailing zeros (and optionally the decimal point) from stringified numbers. |

---

## 11. DataConverterStringTrim

Trims whitespace from strings.

| Field | Description |
|-------|-------------|
| **Type key** | 532 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | trimType (746, UInt) |
| **trimType default** | 1 |
| **trimType values** | 0 = none, 1 = start, 2 = end, 3 = both |
| **Notes** | Bindable. Trims leading/trailing whitespace based on trimType. |

---

## 12. DataConverterInterpolator

Smoothly interpolates between value changes over a duration.

| Field | Description |
|-------|-------------|
| **Type key** | 534 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | duration (756, Float), interpolationType (757, UInt), interpolatorId (758, UInt) |
| **duration default** | 1.0 |
| **interpolationType default** | 1 |
| **interpolatorId default** | 4294967295 (0xFFFFFFFF, Core.missingId) |
| **interpolationType values** | Index into KeyframeInterpolation enum (0=hold, 1=linear, 2=cubic) |
| **Notes** | When `interpolationType` is 2 (cubic), the `interpolatorId` references a CubicInterpolator object. Duration is bindable and controls how long the transition takes in seconds. |

---

## 13. DataConverterBooleanNegate

Inverts a boolean value.

| Field | Description |
|-------|-------------|
| **Type key** | 535 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | (none) |
| **Notes** | No configuration. Simply negates the boolean input (true -> false, false -> true). |

---

## 14. DataConverterRangeMapper

Maps an input value from one numeric range to another with optional interpolation.

| Field | Description |
|-------|-------------|
| **Type key** | 519 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | interpolationType (713, UInt), interpolatorId (714, UInt), flags (715, UInt), minInput (716, Float), maxInput (717, Float), minOutput (718, Float), maxOutput (719, Float) |
| **interpolationType default** | 1 |
| **interpolatorId default** | 4294967295 (0xFFFFFFFF, Core.missingId) |
| **flags default** | 0 |
| **minInput default** | 1.0 |
| **maxInput default** | 1.0 |
| **minOutput default** | 1.0 |
| **maxOutput default** | 1.0 |
| **Notes** | Maps input from [minInput, maxInput] to [minOutput, maxOutput]. The `flags` property controls clamping behavior. The `interpolationType` (0=hold, 1=linear, 2=cubic) defines the easing curve; when cubic, `interpolatorId` references a CubicInterpolator. `maxOutput` is bindable. |

---

## 15. DataConverterFormula

Evaluates a mathematical formula composed of FormulaToken children.

| Field | Description |
|-------|-------------|
| **Type key** | 536 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | randomModeValue (887, UInt) |
| **randomModeValue default** | 0 |
| **Notes** | The formula is defined by its child FormulaToken objects, which are emitted in order as children of this object. The tokens form a postfix (RPN) expression tree. |

---

## 16. DataConverterSystemDegsToRads

Converts degrees to radians. Inherits from DataConverterOperation.

| Field | Description |
|-------|-------------|
| **Type key** | 514 |
| **Extends** | DataConverterOperation (516) -> DataConverter (488) |
| **Inherited properties** | name (662, String), operationType (682, UInt) |
| **Own properties** | (none) |
| **Notes** | System converter with a fixed operation. The inherited `operationType` should be left at default (0). Multiplies input by pi/180. |

---

## 17. DataConverterSystemNormalizer

Normalizes a value within a range. Inherits from DataConverterOperationValue.

| Field | Description |
|-------|-------------|
| **Type key** | 515 |
| **Extends** | DataConverterOperationValue (500) -> DataConverterOperation (516) -> DataConverter (488) |
| **Inherited properties** | name (662, String), operationType (682, UInt), operationValue (681, Float) |
| **Own properties** | (none) |
| **Notes** | System converter. Inherits both `operationType` and `operationValue` from the DataConverterOperationValue chain. |

---

## 18. DataConverterNumberToList

Converts a number to a list by using it as a count to generate instances.

| Field | Description |
|-------|-------------|
| **Type key** | 568 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | viewModelId (816, UInt) |
| **viewModelId default** | 4294967295 (0xFFFFFFFF, Core.missingId) |
| **Notes** | References the ViewModel (by ID) to instantiate for each list item. |

---

## 19. DataConverterListToLength

Converts a list to its length (number of items).

| Field | Description |
|-------|-------------|
| **Type key** | 591 |
| **Extends** | DataConverter (488) |
| **Inherited properties** | name (662, String) |
| **Own properties** | (none) |
| **Notes** | No configuration. Returns the count of items in the input list. |

---

## Formula Tokens

Formula tokens are children of DataConverterFormula (536). They define the individual elements of a formula expression. All tokens inherit from the abstract FormulaToken base (type 537), which has no properties of its own.

### Token Hierarchy

```
FormulaToken (537, abstract) -- (no properties)
├── FormulaTokenArgumentSeparator (538) -- (no properties)
├── FormulaTokenOperation (541)         -- operationType (775, UInt)
├── FormulaTokenValue (543)             -- operationValue (777, Float)
├── FormulaTokenInput (545)             -- (no properties)
├── FormulaTokenParenthesis (539, abstract) -- (no properties)
│   ├── FormulaTokenParenthesisClose (540) -- (no properties)
│   ├── FormulaTokenParenthesisOpen (544)  -- (no properties)
│   └── FormulaTokenFunction (542)         -- functionType (776, UInt)
```

---

### 20. FormulaTokenArgumentSeparator

Represents a comma/argument separator in a function call.

| Field | Description |
|-------|-------------|
| **Type key** | 538 |
| **Extends** | FormulaToken (537) |
| **Own properties** | (none) |

---

### 21. FormulaTokenParenthesisClose

Represents a closing parenthesis `)`.

| Field | Description |
|-------|-------------|
| **Type key** | 540 |
| **Extends** | FormulaTokenParenthesis (539) -> FormulaToken (537) |
| **Own properties** | (none) |

---

### 22. FormulaTokenOperation

Represents a mathematical/logical operation (+, -, *, /, etc.).

| Field | Description |
|-------|-------------|
| **Type key** | 541 |
| **Extends** | FormulaToken (537) |
| **Own properties** | operationType (775, UInt) |
| **operationType default** | 0 |
| **Notes** | The `operationType` value selects the operation (e.g., add, subtract, multiply, divide, modulo, power, comparison operators, logical operators). |

---

### 23. FormulaTokenFunction

Represents a function call (sin, cos, abs, min, max, etc.).

| Field | Description |
|-------|-------------|
| **Type key** | 542 |
| **Extends** | FormulaTokenParenthesis (539) -> FormulaToken (537) |
| **Own properties** | functionType (776, UInt) |
| **functionType default** | 0 |
| **Notes** | The `functionType` value selects the function. A FormulaTokenFunction also acts as an opening parenthesis for its arguments. FormulaTokenArgumentSeparator separates arguments within. |

---

### 24. FormulaTokenValue

Represents a literal numeric value in the formula.

| Field | Description |
|-------|-------------|
| **Type key** | 543 |
| **Extends** | FormulaToken (537) |
| **Own properties** | operationValue (777, Float) |
| **operationValue default** | 1.0 |
| **Notes** | A constant number embedded in the formula. |

---

### 25. FormulaTokenParenthesisOpen

Represents an opening parenthesis `(`.

| Field | Description |
|-------|-------------|
| **Type key** | 544 |
| **Extends** | FormulaTokenParenthesis (539) -> FormulaToken (537) |
| **Own properties** | (none) |

---

### 26. FormulaTokenInput

Represents a reference to the formula's input value (the data-bound value).

| Field | Description |
|-------|-------------|
| **Type key** | 545 |
| **Extends** | FormulaToken (537) |
| **Own properties** | (none) |
| **Notes** | When evaluated, this token resolves to the current input value of the DataConverterFormula. |

---

## Property Key Summary

### New property keys required (not yet in `core.rs`)

| Constant name | Key | Backing type | Used by |
|---------------|-----|-------------|---------|
| DATA_CONVERTER_NAME | 662 | String | DataConverter (base, all subtypes) |
| DATA_CONVERTER_ROUNDER_DECIMALS | 669 | UInt | DataConverterRounder |
| DATA_CONVERTER_GROUP_ITEM_CONVERTER_ID | 679 | UInt | DataConverterGroupItem |
| DATA_CONVERTER_OPERATION_VALUE | 681 | Float | DataConverterOperationValue, DataConverterSystemNormalizer |
| DATA_CONVERTER_OPERATION_TYPE | 682 | UInt | DataConverterOperation (base), all Operation subtypes |
| DATA_CONVERTER_RANGE_MAPPER_INTERPOLATION_TYPE | 713 | UInt | DataConverterRangeMapper |
| DATA_CONVERTER_RANGE_MAPPER_INTERPOLATOR_ID | 714 | UInt | DataConverterRangeMapper |
| DATA_CONVERTER_RANGE_MAPPER_FLAGS | 715 | UInt | DataConverterRangeMapper |
| DATA_CONVERTER_RANGE_MAPPER_MIN_INPUT | 716 | Float | DataConverterRangeMapper |
| DATA_CONVERTER_RANGE_MAPPER_MAX_INPUT | 717 | Float | DataConverterRangeMapper |
| DATA_CONVERTER_RANGE_MAPPER_MIN_OUTPUT | 718 | Float | DataConverterRangeMapper |
| DATA_CONVERTER_RANGE_MAPPER_MAX_OUTPUT | 719 | Float | DataConverterRangeMapper |
| DATA_CONVERTER_STRING_PAD_LENGTH | 743 | UInt | DataConverterStringPad |
| DATA_CONVERTER_STRING_PAD_TEXT | 744 | String | DataConverterStringPad |
| DATA_CONVERTER_STRING_PAD_TYPE | 745 | UInt | DataConverterStringPad |
| DATA_CONVERTER_STRING_TRIM_TYPE | 746 | UInt | DataConverterStringTrim |
| DATA_CONVERTER_INTERPOLATOR_DURATION | 756 | Float | DataConverterInterpolator |
| DATA_CONVERTER_INTERPOLATOR_INTERPOLATION_TYPE | 757 | UInt | DataConverterInterpolator |
| DATA_CONVERTER_INTERPOLATOR_INTERPOLATOR_ID | 758 | UInt | DataConverterInterpolator |
| DATA_CONVERTER_TO_STRING_FLAGS | 764 | UInt | DataConverterToString |
| DATA_CONVERTER_TO_STRING_DECIMALS | 765 | UInt | DataConverterToString |
| DATA_CONVERTER_TO_STRING_COLOR_FORMAT | 766 | String | DataConverterToString |
| FORMULA_TOKEN_OPERATION_TYPE | 775 | UInt | FormulaTokenOperation |
| FORMULA_TOKEN_FUNCTION_TYPE | 776 | UInt | FormulaTokenFunction |
| FORMULA_TOKEN_VALUE_OPERATION_VALUE | 777 | Float | FormulaTokenValue |
| DATA_CONVERTER_NUMBER_TO_LIST_VIEW_MODEL_ID | 816 | UInt | DataConverterNumberToList |
| DATA_CONVERTER_FORMULA_RANDOM_MODE_VALUE | 887 | UInt | DataConverterFormula |

### New type key constants required (not yet in `core.rs`)

| Constant name | Key |
|---------------|-----|
| DATA_CONVERTER | 488 |
| DATA_CONVERTER_ROUNDER | 489 |
| DATA_CONVERTER_TO_STRING | 490 |
| DATA_CONVERTER_GROUP_ITEM | 498 |
| DATA_CONVERTER_GROUP | 499 |
| DATA_CONVERTER_OPERATION_VALUE | 500 |
| DATA_CONVERTER_TRIGGER | 504 |
| DATA_CONVERTER_SYSTEM_DEGS_TO_RADS | 514 |
| DATA_CONVERTER_SYSTEM_NORMALIZER | 515 |
| DATA_CONVERTER_OPERATION | 516 |
| DATA_CONVERTER_OPERATION_VIEW_MODEL | 517 |
| DATA_CONVERTER_RANGE_MAPPER | 519 |
| DATA_CONVERTER_STRING_PAD | 530 |
| DATA_CONVERTER_STRING_REMOVE_ZEROS | 531 |
| DATA_CONVERTER_STRING_TRIM | 532 |
| DATA_CONVERTER_INTERPOLATOR | 534 |
| DATA_CONVERTER_BOOLEAN_NEGATE | 535 |
| DATA_CONVERTER_FORMULA | 536 |
| FORMULA_TOKEN | 537 |
| FORMULA_TOKEN_ARGUMENT_SEPARATOR | 538 |
| FORMULA_TOKEN_PARENTHESIS | 539 |
| FORMULA_TOKEN_PARENTHESIS_CLOSE | 540 |
| FORMULA_TOKEN_OPERATION | 541 |
| FORMULA_TOKEN_FUNCTION | 542 |
| FORMULA_TOKEN_VALUE | 543 |
| FORMULA_TOKEN_PARENTHESIS_OPEN | 544 |
| FORMULA_TOKEN_INPUT | 545 |
| DATA_CONVERTER_NUMBER_TO_LIST | 568 |
| DATA_CONVERTER_LIST_TO_LENGTH | 591 |
| DATA_CONVERTER_TO_NUMBER | 617 |

---

## Implementation Notes

### Parent-child relationships

- DataConverterGroup (499) is the parent of DataConverterGroupItem (498) children.
- DataConverterFormula (536) is the parent of FormulaToken children (538-545).
- All data converters are referenced by DataBind objects via `converterId` (660). They are emitted as top-level objects within the artboard (not as children of the DataBind).

### Defaults and omission

Following project convention, only emit properties when their values differ from defaults. For example:
- DataConverterRounder with decimals=0 needs no decimals property.
- DataConverterStringTrim with trimType=1 needs no trimType property.
- Properties with Core.missingId defaults (0xFFFFFFFF) should not be emitted when the value equals 0xFFFFFFFF.

### CoreBytesType (sourcePathIds on DataConverterOperationViewModel)

Property 711 (`sourcePathIds`) uses `CoreBytesType` which is not one of the 4 standard backing types. It does NOT appear in `property_backing_type_generated()` and must NOT be added to the Table of Contents. Encoding:
1. Property key is written as a varuint.
2. Length of the byte payload is written as a varuint.
3. Raw bytes follow.

This is the same pattern used for `Weight.values` (102) and `Weight.indices` (103).

### Abstract types

DataConverter (488), DataConverterOperation (516), FormulaToken (537), and FormulaTokenParenthesis (539) are abstract base types. They must NOT be instantiated directly. Only their concrete subtypes should appear in .riv files.
