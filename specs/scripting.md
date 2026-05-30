# Scripting Types

Spec for scripted component types and script input types that enable Rive's scripting system.

## Type Keys

### Scripted Components

| Type | Key | C++ base |
|------|-----|----------|
| ScriptedDrawable | 603 | Drawable (13) |
| ScriptedDataConverter | 629 | DataConverter (488) |
| ScriptedLayout | 637 | LayoutComponent (409) |
| ScriptedPathEffect | 640 | Component (10) |
| ScriptedListenerAction | 646 | ListenerAction (125) |
| ScriptedTransitionCondition | 647 | TransitionCondition (476) |

### Script Inputs

| Type | Key | C++ base |
|------|-----|----------|
| ScriptInputNumber | 611 | Component (10) |
| ScriptInputViewModelProperty | 612 | Component (10) |
| ScriptInputTrigger | 618 | Component (10) |
| ScriptInputArtboard | 621 | Component (10) |
| ScriptInputColor | 626 | Component (10) |
| ScriptInputString | 627 | Component (10) |
| ScriptInputBoolean | 631 | Component (10) |

## Property Keys

From `generated_registry.rs`:

| Property | Key | Backing | Used by |
|----------|-----|---------|---------|
| name | 4 | String | All (inherited from Component) |
| parentId | 5 | UInt | All (inherited from Component) |
| scriptAssetId | 848 | UInt | ScriptedDrawable |
| scriptAssetId | 892 | UInt | ScriptedDataConverter |
| scriptAssetId | 912 | UInt | ScriptedLayout |
| scriptAssetId | 930 | UInt | ScriptedListenerAction |
| scriptAssetId | 931 | UInt | ScriptedTransitionCondition |
| generatorFunctionRef | 893 | UInt | ScriptedDrawable |
| threshold | 894 | Float | ScriptedDrawable |
| isPaused | 895 | UInt (bool) | ScriptedDrawable |
| speed | 907 | Float | ScriptedDrawable |
| quantize | 908 | Float | ScriptedDrawable |
| interactive | 891 | UInt (bool) | ScriptedDrawable |
| isStateful | 951 | UInt (bool) | ScriptedListenerAction, ScriptedTransitionCondition |
| isRelative | 921 | UInt (bool) | ScriptedPathEffect |
| targetId | 922 | UInt | ScriptedPathEffect |
| artboardId | 934 | UInt | ScriptInputArtboard |
| viewModelId | 935 | UInt | ScriptInputViewModelProperty |

### Property disambiguation

Several types use `scriptAssetId` but with different property keys. The C++ runtime generates separate property keys per type to allow the core registry to dispatch correctly:
- ScriptedDrawable: scriptAssetId = 848
- ScriptedDataConverter: scriptAssetId = 892
- ScriptedLayout: scriptAssetId = 912
- ScriptedListenerAction: scriptAssetId = 930
- ScriptedTransitionCondition: scriptAssetId = 931

ScriptedPathEffect does NOT appear to have its own scriptAssetId in the registry.

## Implementation Details

### ScriptedDrawable (603)

A drawable whose rendering is controlled by a script.

```
Hierarchy: Component -> Drawable -> ScriptedDrawable
```

Properties:
- `name` (4, String) - component name
- `parentId` (5, UInt) - parent component
- `scriptAssetId` (848, UInt) - reference to the ScriptAsset containing the code
- `generatorFunctionRef` (893, UInt) - function reference within the script
- `threshold` (894, Float) - rendering threshold
- `isPaused` (895, UInt/Bool) - whether the script execution is paused
- `speed` (907, Float) - script execution speed multiplier
- `quantize` (908, Float) - quantization value for frame stepping
- `interactive` (891, UInt/Bool) - whether the drawable responds to interaction

The most property-rich scripted type. Most properties should be conditionally emitted (only when non-default).

### ScriptedDataConverter (629)

A data converter whose transformation logic is defined by a script.

```
Hierarchy: DataConverter (488) -> ScriptedDataConverter
```

Properties:
- `scriptAssetId` (892, UInt) - reference to the ScriptAsset

Note: DataConverter types may not inherit from Component, so they may not have name/parentId. Check C++ hierarchy. The converter is referenced by data bindings via `converterId` (660).

### ScriptedLayout (637)

A layout component whose layout algorithm is defined by a script.

```
Hierarchy: Component -> LayoutComponent -> ScriptedLayout
```

Properties:
- `name` (4, String) - component name
- `parentId` (5, UInt) - parent component
- `scriptAssetId` (912, UInt) - reference to the ScriptAsset

Inherits layout properties from LayoutComponent. The script controls custom layout logic beyond flexbox.

### ScriptedPathEffect (640)

A path effect (like dash, trim) whose behavior is defined by a script.

```
Hierarchy: Component -> ScriptedPathEffect
```

Properties:
- `name` (4, String) - component name
- `parentId` (5, UInt) - parent component
- `isRelative` (921, UInt/Bool) - whether effect coordinates are relative
- `targetId` (922, UInt) - target path/shape to apply the effect to

### ScriptedListenerAction (646)

A listener action whose behavior is defined by a script.

```
Hierarchy: ListenerAction (125) -> ScriptedListenerAction
```

Properties:
- `scriptAssetId` (930, UInt) - reference to the ScriptAsset
- `isStateful` (951, UInt/Bool) - whether the action maintains state between invocations

ListenerAction types are children of StateMachineListener in the object hierarchy.

### ScriptedTransitionCondition (647)

A transition condition whose evaluation is defined by a script.

```
Hierarchy: TransitionCondition (476) -> ScriptedTransitionCondition
```

Properties:
- `scriptAssetId` (931, UInt) - reference to the ScriptAsset
- `isStateful` (951, UInt/Bool) - whether the condition maintains state

TransitionCondition types are children of StateTransition in the object hierarchy.

### ScriptInputNumber (611)

Numeric input parameter for a scripted component.

```
Hierarchy: Component -> ScriptInputNumber
```

Properties:
- `name` (4, String) - input parameter name
- `parentId` (5, UInt) - parent scripted component

ScriptInput types are children of their parent scripted component (ScriptedDrawable, ScriptedLayout, etc.) and define the input interface. The actual value is set at runtime or via data binding.

### ScriptInputViewModelProperty (612)

ViewModel property input for a scripted component, allowing scripts to read ViewModel data.

```
Hierarchy: Component -> ScriptInputViewModelProperty
```

Properties:
- `name` (4, String) - input parameter name
- `parentId` (5, UInt) - parent scripted component
- `viewModelId` (935, UInt) - reference to the ViewModel

### ScriptInputTrigger (618)

Trigger input for a scripted component.

```
Hierarchy: Component -> ScriptInputTrigger
```

Properties:
- `name` (4, String) - input parameter name
- `parentId` (5, UInt) - parent scripted component

### ScriptInputArtboard (621)

Artboard reference input for a scripted component.

```
Hierarchy: Component -> ScriptInputArtboard
```

Properties:
- `name` (4, String) - input parameter name
- `parentId` (5, UInt) - parent scripted component
- `artboardId` (934, UInt) - reference to the artboard

### ScriptInputColor (626)

Color input for a scripted component.

```
Hierarchy: Component -> ScriptInputColor
```

Properties:
- `name` (4, String) - input parameter name
- `parentId` (5, UInt) - parent scripted component

### ScriptInputString (627)

String input for a scripted component.

```
Hierarchy: Component -> ScriptInputString
```

Properties:
- `name` (4, String) - input parameter name
- `parentId` (5, UInt) - parent scripted component

### ScriptInputBoolean (631)

Boolean input for a scripted component.

```
Hierarchy: Component -> ScriptInputBoolean
```

Properties:
- `name` (4, String) - input parameter name
- `parentId` (5, UInt) - parent scripted component

## New Constants Needed in core.rs

### type_keys

```rust
pub const SCRIPTED_DRAWABLE: u16 = 603;
pub const SCRIPTED_DATA_CONVERTER: u16 = 629;
pub const SCRIPTED_LAYOUT: u16 = 637;
pub const SCRIPTED_PATH_EFFECT: u16 = 640;
pub const SCRIPTED_LISTENER_ACTION: u16 = 646;
pub const SCRIPTED_TRANSITION_CONDITION: u16 = 647;
pub const SCRIPT_INPUT_NUMBER: u16 = 611;
pub const SCRIPT_INPUT_VIEW_MODEL_PROPERTY: u16 = 612;
pub const SCRIPT_INPUT_TRIGGER: u16 = 618;
pub const SCRIPT_INPUT_ARTBOARD: u16 = 621;
pub const SCRIPT_INPUT_COLOR: u16 = 626;
pub const SCRIPT_INPUT_STRING: u16 = 627;
pub const SCRIPT_INPUT_BOOLEAN: u16 = 631;
```

### property_keys

```rust
pub const SCRIPTED_DRAWABLE_SCRIPT_ASSET_ID: u16 = 848;
pub const SCRIPTED_DRAWABLE_GENERATOR_FUNCTION_REF: u16 = 893;
pub const SCRIPTED_DRAWABLE_THRESHOLD: u16 = 894;
pub const SCRIPTED_DRAWABLE_IS_PAUSED: u16 = 895;
pub const SCRIPTED_DRAWABLE_INTERACTIVE: u16 = 891;
pub const SCRIPTED_DRAWABLE_SPEED: u16 = 907;
pub const SCRIPTED_DRAWABLE_QUANTIZE: u16 = 908;
pub const SCRIPTED_DATA_CONVERTER_SCRIPT_ASSET_ID: u16 = 892;
pub const SCRIPTED_LAYOUT_SCRIPT_ASSET_ID: u16 = 912;
pub const SCRIPTED_PATH_EFFECT_IS_RELATIVE: u16 = 921;
pub const SCRIPTED_PATH_EFFECT_TARGET_ID: u16 = 922;
pub const SCRIPTED_LISTENER_ACTION_SCRIPT_ASSET_ID: u16 = 930;
pub const SCRIPTED_TRANSITION_CONDITION_SCRIPT_ASSET_ID: u16 = 931;
pub const SCRIPTED_IS_STATEFUL: u16 = 951;
pub const SCRIPT_INPUT_ARTBOARD_ARTBOARD_ID: u16 = 934;
pub const SCRIPT_INPUT_VIEW_MODEL_PROPERTY_VIEW_MODEL_ID: u16 = 935;
```

## File Location

All types go in a new file `src/objects/scripting.rs` or can be added to an existing file. Given the number of types (13), a dedicated file is recommended.

The file must be registered in `src/objects/mod.rs`.

## Struct Designs

### Simple ScriptInput types (Number, Trigger, Color, String, Boolean)

All share the same minimal struct:

```rust
pub struct ScriptInputNumber {
    pub name: String,
    pub parent_id: u64,
}
```

Constructor: `new(name, parent_id)`. Properties: always emit name (4) and parent_id (5).

### ScriptInputViewModelProperty

```rust
pub struct ScriptInputViewModelProperty {
    pub name: String,
    pub parent_id: u64,
    pub view_model_id: u64,
}
```

Emit `view_model_id` (935) only when != 0.

### ScriptInputArtboard

```rust
pub struct ScriptInputArtboard {
    pub name: String,
    pub parent_id: u64,
    pub artboard_id: u64,
}
```

Emit `artboard_id` (934) only when != 0.

### ScriptedDrawable

```rust
pub struct ScriptedDrawable {
    pub name: String,
    pub parent_id: u64,
    pub script_asset_id: u64,
    pub generator_function_ref: u64,
    pub threshold: f32,
    pub is_paused: bool,
    pub speed: f32,
    pub quantize: f32,
    pub interactive: bool,
}
```

Defaults: `script_asset_id=0`, `generator_function_ref=0`, `threshold=0.0`, `is_paused=false`, `speed=1.0`, `quantize=0.0`, `interactive=false`.

### ScriptedDataConverter

```rust
pub struct ScriptedDataConverter {
    pub script_asset_id: u64,
}
```

Minimal type; may not have name/parentId if DataConverter base doesn't inherit from Component.

### ScriptedLayout

```rust
pub struct ScriptedLayout {
    pub name: String,
    pub parent_id: u64,
    pub script_asset_id: u64,
}
```

### ScriptedPathEffect

```rust
pub struct ScriptedPathEffect {
    pub name: String,
    pub parent_id: u64,
    pub is_relative: bool,
    pub target_id: u64,
}
```

### ScriptedListenerAction

```rust
pub struct ScriptedListenerAction {
    pub script_asset_id: u64,
    pub is_stateful: bool,
}
```

ListenerAction types don't typically have name/parentId (see ListenerBoolChange pattern in `state_machine.rs`).

### ScriptedTransitionCondition

```rust
pub struct ScriptedTransitionCondition {
    pub script_asset_id: u64,
    pub is_stateful: bool,
}
```

TransitionCondition types don't typically have name/parentId (see TransitionInputCondition pattern).

## Test Coverage

Each type needs:
1. Type key assertion matching the numeric constant
2. Default properties test
3. Non-default properties test verifying correct keys and values
4. For ScriptedDrawable: test that default values are omitted (speed=1.0, is_paused=false, etc.)
5. For ScriptInput types: verify name and parentId are always emitted
6. For ScriptedListenerAction/ScriptedTransitionCondition: verify no name/parentId emission (matches listener action pattern)

## Relationship Diagram

```
ScriptAsset (529)
  ^
  | referenced by scriptAssetId
  |
  +-- ScriptedDrawable (603) --has-children--> ScriptInputNumber (611)
  |                                            ScriptInputViewModelProperty (612)
  |                                            ScriptInputTrigger (618)
  |                                            ScriptInputArtboard (621)
  |                                            ScriptInputColor (626)
  |                                            ScriptInputString (627)
  |                                            ScriptInputBoolean (631)
  |
  +-- ScriptedDataConverter (629)
  +-- ScriptedLayout (637) ---has-children--> ScriptInput* types
  +-- ScriptedPathEffect (640) (may use different script ref mechanism)
  +-- ScriptedListenerAction (646)
  +-- ScriptedTransitionCondition (647)
```
