# Listener & State Machine Extensions

Spec for listener action extensions, state machine fire/trigger/action types, nested artboard/input components, and ViewModel-driven blend states.

## Type Keys

| Type | Key | C++ base |
|------|-----|----------|
| ListenerAlignTarget | 126 | ListenerAction (125) |
| ListenerFireEvent | 168 | ListenerAction (125) |
| ListenerViewModelChange | 487 | ListenerAction (125) |
| StateMachineFireEvent | 169 | StateMachineComponent (54) |
| StateMachineFireTrigger | 614 | StateMachineComponent (54) |
| StateMachineFireAction | 615 | StateMachineComponent (54) |
| StateMachineComponentNestedArtboard | 172 | StateMachineComponent (54) |
| StateMachineNestedInput | 173 | StateMachineComponent (54) |
| BlendState1DViewModel | 528 | BlendState1D (527) |

## Property Keys

From `generated_registry.rs` property_name and property_backing_type_generated:

| Property | Key | Backing | Used by |
|----------|-----|---------|---------|
| targetId | 240 | UInt | ListenerAlignTarget |
| eventId | 389 | UInt | ListenerFireEvent (fire event from listener) |
| eventId | 392 | UInt | StateMachineFireEvent |
| occursValue | 393 | UInt | StateMachineFireEvent |
| eventId | 399 | UInt | ListenerViewModelChange (if used as event ref) |
| nestedInputId | 400 | UInt | StateMachineNestedInput |
| artboardId | 197 | UInt | StateMachineComponentNestedArtboard |
| viewModelPropertyId | 554 | UInt | ListenerViewModelChange |

### Existing property keys already in core.rs

- `LISTENER_TARGET_ID` = 224 (already used by StateMachineListener)
- `LISTENER_INPUT_ID` = 227
- `LISTENER_ACTION_ID` = 226
- `STATE_MACHINE_COMPONENT_NAME` = 138
- `NESTED_ARTBOARD_ARTBOARD_ID` = 197
- `NESTED_INPUT_ID` = 400

## Implementation Details

### ListenerAlignTarget (126)

Listener action that aligns to a target object.

```
Hierarchy: ListenerAction -> ListenerAlignTarget
```

Properties:
- `targetId` (240, UInt) - ID of the target object to align to

Pattern: follows ListenerBoolChange/ListenerNumberChange pattern from `state_machine.rs`.

### ListenerFireEvent (168)

Listener action that fires an event.

```
Hierarchy: ListenerAction -> ListenerFireEvent
```

Properties:
- `eventId` (389, UInt) - ID of the event to fire

Note: property key 389 is `eventId` per the generated registry, backing type UInt. This is the same property used by AudioEvent and similar event-referencing types.

### ListenerViewModelChange (487)

Listener action that changes a ViewModel property value.

```
Hierarchy: ListenerAction -> ListenerViewModelChange
```

Properties:
- `viewModelPropertyId` (554, UInt) - the ViewModel property to change

This type bridges the listener system with the ViewModel/data-binding system.

### StateMachineFireEvent (169)

Fires an event from the state machine.

```
Hierarchy: StateMachineComponent -> StateMachineFireEvent
```

Properties:
- `eventId` (392, UInt) - ID of the event to fire
- `occursValue` (393, UInt) - when the event occurs (enum: entry/exit/etc)

Inherits `name` (138) from StateMachineComponent.

### StateMachineFireTrigger (614)

Fires a trigger from the state machine.

```
Hierarchy: StateMachineComponent -> StateMachineFireTrigger
```

Properties:
- Inherits `name` (138) from StateMachineComponent
- May reference a trigger input by ID

Minimal-property type similar to StateMachineTrigger pattern.

### StateMachineFireAction (615)

Fires an action from the state machine.

```
Hierarchy: StateMachineComponent -> StateMachineFireAction
```

Properties:
- Inherits `name` (138) from StateMachineComponent

Minimal-property type; the action itself is determined by the component hierarchy.

### StateMachineComponentNestedArtboard (172)

Nested artboard reference within a state machine.

```
Hierarchy: StateMachineComponent -> StateMachineComponentNestedArtboard
```

Properties:
- `artboardId` (197, UInt) - references the nested artboard

This enables state machines to reference nested artboards for nested state machine inputs.

### StateMachineNestedInput (173)

Nested input reference in a state machine, forwarding inputs to nested artboards.

```
Hierarchy: StateMachineComponent -> StateMachineNestedInput
```

Properties:
- `nestedInputId` (400, UInt) - the input ID within the nested artboard

Works in conjunction with StateMachineComponentNestedArtboard to route inputs.

### BlendState1DViewModel (528)

A 1D blend state driven by a ViewModel property instead of a state machine input.

```
Hierarchy: BlendState -> BlendState1D -> BlendState1DViewModel
```

Properties:
- Same as BlendState1D (no extra properties beyond the base)
- The ViewModel binding is established through the data binding system, not through direct properties

Similar to BlendState1D (527) which is already implemented as a zero-property type. The ViewModel variant uses data binding to determine the blend value instead of `BlendState1DInput.inputId`.

## New Constants Needed in core.rs

### type_keys

```rust
pub const LISTENER_ALIGN_TARGET: u16 = 126;
pub const LISTENER_FIRE_EVENT: u16 = 168;
pub const LISTENER_VIEW_MODEL_CHANGE: u16 = 487;
pub const STATE_MACHINE_FIRE_EVENT: u16 = 169;
pub const STATE_MACHINE_FIRE_TRIGGER: u16 = 614;
pub const STATE_MACHINE_FIRE_ACTION: u16 = 615;
pub const STATE_MACHINE_COMPONENT_NESTED_ARTBOARD: u16 = 172;
pub const STATE_MACHINE_NESTED_INPUT: u16 = 173;
pub const BLEND_STATE_1D_VIEW_MODEL: u16 = 528;
```

### property_keys (new ones not already in core.rs)

```rust
pub const LISTENER_ALIGN_TARGET_ID: u16 = 240;
pub const LISTENER_FIRE_EVENT_EVENT_ID: u16 = 389;
pub const STATE_MACHINE_FIRE_EVENT_EVENT_ID: u16 = 392;
pub const STATE_MACHINE_FIRE_EVENT_OCCURS_VALUE: u16 = 393;
pub const LISTENER_VIEW_MODEL_CHANGE_PROPERTY_ID: u16 = 554;
```

Note: `NESTED_ARTBOARD_ARTBOARD_ID` (197) and `NESTED_INPUT_ID` (400) already exist in core.rs.

## File Location

All types go in `src/objects/state_machine.rs` alongside the existing listener and state machine types.

## Test Coverage

Each type needs:
1. Type key assertion matching the numeric constant
2. Properties test verifying correct keys and values
3. Default/zero omission tests where applicable (e.g., eventId=0 should still be emitted since 0 is a valid reference)
