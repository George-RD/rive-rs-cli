# objects/ — Rive Object Type Definitions

All Rive runtime object types. Every struct implements `RiveObject` trait from `core.rs`.

## STRUCTURE

| File | Types | LOC | Domain |
|------|-------|-----|--------|
| `core.rs` | RiveObject, PropertyValue, BackingType, type_keys, property_keys | 291 | Foundation |
| `shapes.rs` | Node, TransformComponent, Shape, Ellipse, Rectangle, Path, Fill, Stroke, SolidColor, LinearGradient, RadialGradient, GradientStop | 1125 | Drawing |
| `state_machine.rs` | StateMachine, SMLayer, SMInput/Number/Bool/Trigger, EntryState, ExitState, AnyState, AnimationState, LayerState, StateTransition, TransitionCondition variants | 660 | Interactivity |
| `animation.rs` | LinearAnimation, KeyedObject, KeyedProperty, KeyFrameDouble, KeyFrameColor | 409 | Animation |
| `artboard.rs` | Artboard, Backboard | 154 | Scene root |

## HOW TO ADD A NEW OBJECT TYPE

1. **Look up in C++ runtime**: Find `*_base.hpp` in `rive-runtime/include/rive/generated/` for exact typeKey and propertyKeys
2. **Add type_key constant** to `core.rs` → `type_keys` module
3. **Add property_key constants** to `core.rs` → `property_keys` module (if new properties)
4. **Update `property_backing_type()`** in `core.rs` for any new property keys
5. **Create struct** in the appropriate file, implement `RiveObject`:
   ```rust
   impl RiveObject for MyType {
       fn type_key(&self) -> u16 { type_keys::MY_TYPE }
       fn properties(&self) -> Vec<Property> { vec![...] }
   }
   ```
6. **Add to builder** in `builder/scene.rs` if JSON-constructable
7. **Add unit tests** in the same file (`#[cfg(test)] mod tests`)

## CRITICAL RULES

- **Type keys and property IDs MUST match C++ generated headers exactly** — one wrong ID = file loads but renders incorrectly or crashes
- **Property backing types**: `property_backing_type()` in `core.rs` must return the correct type for every property key used. The encoder uses this for ToC generation; the validator uses it for deserialization.
- **CoreBoolType properties** (isVisible=41, enableWorkArea=62, quantize=376, smBoolValue=141, linkCornerRadius=164): backing type is `UInt` (0) but **encode as single raw byte**, NOT LEB128 varuint. The C++ `CoreBoolType::deserialize` calls `reader.readByte()`.
- **CoreDoubleType properties** (all floats): despite the name "Double", they encode as **4-byte float32** LE, not 8-byte double.
- **Only write non-default properties** — default values waste bytes and can confuse runtimes.
- **Artboard property emission order** must be `width(7)` → `height(8)` → `name(4)` and must NOT include `parentId(5)`.
- **LinearAnimation property slimming**: always write `name/fps/duration`; write `speed/loop/workStart/workEnd` only when non-default.

## HIERARCHY (inheritance in C++ runtime)

```
Component (name=4, parentId=5)
├── Node (x=13, y=14)
│   └── TransformComponent (rotation=15, scaleX=16, scaleY=17, opacity=18)
│       ├── Shape (blendMode=23)
│       └── WorldTransformComponent
├── Drawable (blendMode=23, flags=129)
├── ContainerComponent
└── Artboard (width=7, height=8, originX=11, originY=12)

Animation (name=55)
├── LinearAnimation (fps=56, duration=57, speed=58, loop=59, ...)
└── StateMachine

StateMachineComponent (name=138)
├── StateMachineInput → SMNumber(value=140), SMBool(value=141), SMTrigger
└── StateMachineLayer
    └── [contains] LayerState (flags=536)
        ├── EntryState, ExitState, AnyState
        └── AdvanceableState (speed=292)
            └── AnimationState (animationId=149)

StateMachineLayerComponent
└── StateTransition (stateToId=151, flags=152, duration=158, exitTime=160, randomWeight=537)
    └── TransitionCondition → Input(inputId=155), Trigger, Value(op=156), Number(value=157), Bool
```

## WHERE TO FIND C++ DEFINITIONS

| Our file | C++ reference |
|----------|---------------|
| `core.rs` type_keys | `core_registry.hpp` → `makeCoreInstance()` switch |
| `core.rs` property_keys | `*_base.hpp` files → `static const uint16_t *PropertyKey` |
| `core.rs` property_backing_type | `core_registry.hpp` → `propertyFieldId()` switch |
| `shapes.rs` | `generated/shapes/*.hpp`, `generated/shapes/paint/*.hpp` |
| `animation.rs` | `generated/animation/linear_animation_base.hpp`, `generated/animation/keyed_*_base.hpp` |
| `state_machine.rs` | `generated/animation/state_machine_*_base.hpp`, `generated/animation/*_state_base.hpp` |
| `artboard.rs` | `generated/artboard_base.hpp`, `generated/backboard_base.hpp` |
