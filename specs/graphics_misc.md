# Graphics Miscellaneous Types Specification

## Overview

This spec covers remaining object types that do not fit neatly into other categories: graphics effects (TargetEffect, GroupEffect), specialized paths (ListPath, PointsCommonPath), scene/artboard management types (Guide, ArtboardComponentList, ArtboardComponentListOverride, ArtboardListMapRule), and an animation keyframe variant (KeyFrameUint). These types span several domains but share the trait of being less commonly used in basic animations and more relevant to advanced features like data-driven lists, design tooling, and effect stacking.

---

## Type: TargetEffect

**typeKey: 644**

A reference object that points to a GroupEffect within the effect stack. TargetEffect acts as a pointer from a shape's paint hierarchy to a specific GroupEffect, enabling shapes to participate in grouped rendering effects. It is a child of Component.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent component (artboard-local index) |
| targetId | 922 | UInt | yes | -1 (missing) | ID of the GroupEffect this effect references in the effect stack |

### Hierarchy

- **Extends**: Component
- **Parent**: Shape paint or shape component in the artboard hierarchy
- **Children**: none

### Notes

- TargetEffect extends Component directly (not ContainerComponent), so it cannot have children.
- The `targetId` property is an Id type (uint at runtime) that references a GroupEffect by its artboard-local index. A value of `Core.missingId` (-1 / `u32::MAX`) means no target is assigned.
- TargetEffect and GroupEffect work together: TargetEffect is the "use site" that binds a shape or paint to a specific GroupEffect definition.
- Property 922 (`targetId`) has backing type UInt.

---

## Type: GroupEffect

**typeKey: 645**

A container that groups rendering effects to be applied collectively to shapes that reference it via TargetEffect. GroupEffect extends ContainerComponent, meaning it can hold child objects that define the effect behavior.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent component (artboard-local index) |

GroupEffect also defines two design-time-only properties that are NOT written to the .riv binary:

| Property | Key | Runtime | Default | Description |
|----------|-----|---------|---------|-------------|
| x | 917 | no | 0.0 | Design-time x position (not serialized) |
| y | 918 | no | 0.0 | Design-time y position (not serialized) |

### Hierarchy

- **Extends**: ContainerComponent (can have children)
- **Parent**: Artboard or a container within the artboard
- **Children**: Effect-defining objects

### Notes

- GroupEffect is a container; its children define what effects are applied when a TargetEffect references it.
- The `x` and `y` properties (keys 917, 918) are marked `runtime: false` in the C++ definitions. They exist for design tooling (editor positioning) and must NOT be written to the binary output. The encoder should skip these properties entirely.
- At the binary level, GroupEffect emits only `name(4)` and `parentId(5)` as inherited from Component/ContainerComponent. No additional runtime properties exist on the type itself.
- Property keys 917 and 918 are not present in the generated property backing type registry, confirming they are non-runtime.

---

## Type: ListPath

**typeKey: 619**

A path type used for list-driven item rendering. ListPath is a specialized PointsCommonPath that sources its path data from a data-bound list. It enables dynamic path generation where the number and position of vertices is driven by a ViewModel list property.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent shape (artboard-local index) |
| isClosed | 32 | UInt (bool) | no | 0 (false) | Whether the path closes back to its first vertex |
| listSource | 874 | UInt | no | -1 (missing) | ID referencing the bound list property that drives this path |

### Hierarchy

- **Extends**: PointsCommonPath (typeKey 620) -> Path -> Node -> TransformComponent -> WorldTransformComponent -> ContainerComponent -> Component
- **Parent**: Shape (typeKey 3) or similar drawable
- **Children**: Vertex objects (StraightVertex, CubicDetachedVertex, etc.)

### Notes

- ListPath extends PointsCommonPath, inheriting the `isClosed` property (key 32). It adds `listSource` (key 874) which references a ViewModel list that provides data for generating path points.
- The `listSource` property is an Id type (uint at runtime). A value of -1 (`Core.missingId`) means no list is bound, and the path behaves like a standard PointsCommonPath.
- The `listSource` property is bindable, meaning it can be connected to data binding contexts.
- Property 874 (`listSource`) has backing type UInt. Property 32 (`isClosed`) has backing type UInt (bool encoding: single raw byte, not LEB128).
- Because ListPath inherits from the full Node hierarchy, it also inherits transform properties (x=13, y=14, rotation=15, scaleX=16, scaleY=17, opacity=18) which should be written only when non-default.

---

## Type: PointsCommonPath

**typeKey: 620**

An abstract base path type that provides the common `isClosed` property shared by all point-based paths. PointsCommonPath is the parent class for PointsPath (typeKey 16) and ListPath (typeKey 619).

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent shape (artboard-local index) |
| isClosed | 32 | UInt (bool) | no | 0 (false) | Whether the path closes back to its first vertex |

PointsCommonPath also defines a design-time-only property:

| Property | Key | Runtime | Default | Description |
|----------|-----|---------|---------|-------------|
| isClockwise | 753 | no | true | Tracks whether user-designed paths are clockwise (not serialized) |

### Hierarchy

- **Extends**: Path -> Node -> TransformComponent -> WorldTransformComponent -> ContainerComponent -> Component
- **Parent**: Shape (typeKey 3) or similar drawable
- **Children**: Vertex objects

### Notes

- PointsCommonPath is typically used as a base class rather than instantiated directly, but the runtime does register typeKey 620 in `makeCoreInstance()`.
- The existing PointsPath (typeKey 16) in the codebase is a sibling concept. PointsCommonPath is the shared base that was extracted to also serve ListPath.
- `isClockwise` (key 753) is marked `runtime: false` and `journal: false` -- it must NOT be written to the binary. It exists purely for editor tooling to track winding order.
- Property 32 (`isClosed`) is a CoreBoolType: its backing type is UInt (0) but it encodes as a single raw byte, NOT LEB128 varuint. This is the same encoding rule as `isVisible(41)` and other bool properties.
- Property 753 is not present in the generated property backing type registry.

---

## Type: Guide

**typeKey: 140**

A design-time guide line used in the Rive editor for aligning objects. Guides are non-runtime objects: they exist in the .riv file for editor state persistence but are ignored by the runtime when loading and rendering.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent artboard (artboard-local index) |

Guide also defines two design-time-only properties:

| Property | Key | Runtime | Default | Description |
|----------|-----|---------|---------|-------------|
| axisPosition | 276 | no | 0.0 | Position of the guide along the axis (not serialized to runtime) |
| axisValue | 277 | no | 0 | Axis the guide relates to: 0 = x (vertical line), 1 = y (horizontal line) (not serialized to runtime) |

### Hierarchy

- **Extends**: Component
- **Parent**: Artboard (typeKey 1)
- **Children**: none

### Notes

- Guide is marked `runtime: false` at the type level in the C++ definitions. The entire object (including its properties) is a design-time artifact. The runtime's `makeCoreInstance()` does NOT instantiate Guide objects -- they are skipped during .riv import.
- Property keys 276 and 277 are not present in the generated property backing type registry, confirming they are non-runtime.
- When generating .riv files intended only for runtime playback (not round-tripping back to the editor), Guide objects can be omitted entirely.
- If Guide objects ARE written (for editor round-trip fidelity), `axisPosition(276)` would encode as Float and `axisValue(277)` as UInt, but the runtime will skip over the entire object during import since typeKey 140 is not recognized.
- Guide objects do not affect rendering, animation, or state machine behavior in any way.

---

## Type: ArtboardComponentList

**typeKey: 559**

A drawable component that renders a list of items within an artboard. ArtboardComponentList sources its items from a data-bound list property and stamps out nested artboard instances for each item in the list.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent component (artboard-local index) |
| listSource | 800 | UInt | yes | -1 (missing) | ID referencing the ViewModel list property that drives this component list |

### Hierarchy

- **Extends**: Drawable -> Node -> TransformComponent -> WorldTransformComponent -> ContainerComponent -> Component
- **Parent**: Artboard (typeKey 1) or LayoutComponent (typeKey 409)
- **Children**: ArtboardListMapRule (typeKey 648), ArtboardComponentListOverride (typeKey 606)

### Notes

- ArtboardComponentList inherits the full Drawable hierarchy, giving it access to `blendModeValue(23)`, `drawableFlags(129)`, and all Node/TransformComponent properties. Only write these when non-default.
- The `listSource` property (key 800) is an Id type referencing a ViewModel list property. It uses backing type UInt. A value of -1 (`Core.missingId` / `u32::MAX`) means no list is bound.
- ArtboardComponentList works with the data binding system: the list property it references determines how many items to render and what data each item receives.
- Child ArtboardListMapRule objects define which artboard template to use for each list item, and ArtboardComponentListOverride objects customize the size/layout of stamped instances.

---

## Type: ArtboardComponentListOverride

**typeKey: 606**

Provides per-artboard overrides for the size and layout of nested artboard instances stamped by an ArtboardComponentList. Each override targets a specific artboard template (or all templates when `artboardId` is -1) and specifies instance dimensions and scaling behavior.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent ArtboardComponentList (artboard-local index) |
| artboardId | 858 | UInt | no | -1 (missing) | Target artboard ID; -1 means apply to all artboards |
| instanceWidth | 859 | Float | no | -1.0 | Width of the nested instance in points or percent |
| instanceHeight | 860 | Float | no | -1.0 | Height of the nested instance in points or percent |
| instanceWidthUnitsValue | 856 | UInt | no | 1 | Width units: 0 = points, 1 = percent |
| instanceHeightUnitsValue | 861 | UInt | no | 1 | Height units: 0 = points, 1 = percent |
| instanceWidthScaleType | 862 | UInt | no | 0 | Width scale type: 0 = fixed, 1 = fill |
| instanceHeightScaleType | 863 | UInt | no | 0 | Height scale type: 0 = fixed, 1 = fill |

### Hierarchy

- **Extends**: Component
- **Parent**: ArtboardComponentList (typeKey 559)
- **Children**: none

### Notes

- ArtboardComponentListOverride is a child of ArtboardComponentList. It customizes how nested artboard instances are sized within the list.
- The `artboardId` property (key 858) specifies which artboard template this override applies to. When set to -1 (`Core.missingId`), the override applies to all artboard templates in the list.
- `instanceWidth(859)` and `instanceHeight(860)` are Float properties that define the size. A value of -1.0 means "use the artboard's intrinsic size."
- `instanceWidthUnitsValue(856)` and `instanceHeightUnitsValue(861)` control whether the dimensions are in points (0) or percent (1). Default is 1 (percent).
- `instanceWidthScaleType(862)` and `instanceHeightScaleType(863)` choose between fixed sizing (0) and fill-to-parent sizing (1).
- The `instanceWidth` and `instanceHeight` properties are animatable and bindable.
- Property backing types: 858=UInt, 859=Float, 860=Float, 856=UInt, 861=UInt, 862=UInt, 863=UInt.

---

## Type: ArtboardListMapRule

**typeKey: 648**

Defines a mapping rule within an ArtboardComponentList that associates a ViewModel type with an artboard template. When the list iterates over items, each item's ViewModel type is matched against the map rules to determine which artboard to stamp for that item.

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| name | 4 | String | no | "" | Component name |
| parentId | 5 | UInt | yes | - | Parent ArtboardComponentList (artboard-local index) |
| artboardId | 934 | UInt | yes | -1 (missing) | Artboard template ID to instantiate for matching items |
| viewModelId | 935 | UInt | yes | -1 (missing) | ViewModel type ID to match against list items |

### Hierarchy

- **Extends**: Component
- **Parent**: ArtboardComponentList (typeKey 559)
- **Children**: none

### Notes

- ArtboardListMapRule extends Component directly, so it has no transform or drawable properties.
- Both `artboardId(934)` and `viewModelId(935)` are Id types (UInt at runtime). They map a ViewModel type to a specific artboard template.
- When ArtboardComponentList processes a list, it examines each item's ViewModel, looks up the matching ArtboardListMapRule, and instantiates the corresponding artboard template.
- Multiple ArtboardListMapRule children can exist under one ArtboardComponentList to handle heterogeneous lists where different items use different artboard templates.
- Property backing types: 934=UInt, 935=UInt.

---

## Type: KeyFrameUint

**typeKey: 450**

A keyframe that stores an unsigned integer value. KeyFrameUint is the uint counterpart to KeyFrameDouble (typeKey 30), KeyFrameColor (typeKey 37), KeyFrameBool (typeKey 84), and KeyFrameString (typeKey 142). It is used to animate properties with UInt backing type (enum values, flags, IDs, etc.).

### Properties

| Property | Key | Backing Type | Required | Default | Description |
|----------|-----|-------------|----------|---------|-------------|
| frame | 67 | UInt | yes | - | Frame number within the animation timeline |
| interpolationType | 68 | UInt | yes | 1 | Interpolation type: 0=hold, 1=linear, 2=cubic |
| interpolatorId | 69 | UInt | no | 4294967295 | ID of the CubicInterpolator (only for cubic interpolation); u32::MAX means no interpolator |
| value | 631 | UInt | yes | 0 | The unsigned integer value at this keyframe |

### Hierarchy

- **Extends**: InterpolatingKeyFrame (typeKey 170) -> KeyFrame (typeKey 29)
- **Parent**: KeyedProperty (typeKey 26) -- keyframes are children of keyed properties within the animation hierarchy
- **Children**: none

### Notes

- KeyFrameUint follows the exact same pattern as the existing KeyFrameDouble, KeyFrameColor, KeyFrameBool, and KeyFrameString implementations. The only difference is the value property type (UInt) and key (631).
- The `value` property (key 631) has backing type UInt. It is written as a LEB128 varuint.
- The `frame`, `interpolationType`, and `interpolatorId` properties are inherited from InterpolatingKeyFrame and KeyFrame and use the same keys and encoding as all other keyframe types.
- When `interpolationType` is 0 (hold), the value snaps to the keyframe value with no transition. When 1 (linear), it linearly interpolates between integer values. When 2 (cubic), the `interpolatorId` must reference a valid CubicInterpolator.
- For `interpolatorId`, the default sentinel value `u32::MAX` (4294967295) indicates no custom interpolator. This matches the pattern used by other keyframe types.
- KeyFrameUint is used for animating enum-typed properties (e.g., layout alignment values, blend modes, fill rules) and other uint-backed properties that change over time.
- Property key 631 is registered in `property_backing_type()` in `core.rs` as UInt.

---

## Parent-Child Relationships

```
Artboard (typeKey 1)
  |
  +-- Guide (typeKey 140)                          <-- design-time only
  |
  +-- Shape (typeKey 3)
  |     |
  |     +-- Fill (typeKey 20)
  |     |     |
  |     |     +-- TargetEffect (typeKey 644)       <-- references a GroupEffect
  |     |
  |     +-- Stroke (typeKey 24)
  |           |
  |           +-- TargetEffect (typeKey 644)       <-- references a GroupEffect
  |
  +-- GroupEffect (typeKey 645)                    <-- effect container
  |
  +-- ArtboardComponentList (typeKey 559)          <-- data-driven list
  |     |
  |     +-- ArtboardListMapRule (typeKey 648)      <-- ViewModel->Artboard mapping
  |     +-- ArtboardListMapRule (typeKey 648)      <-- additional mapping rules
  |     +-- ArtboardComponentListOverride (606)    <-- size/layout overrides
  |
  +-- Shape (typeKey 3)
        |
        +-- ListPath (typeKey 619)                 <-- data-driven path
        |     |
        |     +-- StraightVertex / CubicVertex...  <-- vertices
        |
        +-- PointsCommonPath (typeKey 620)         <-- base for point paths
              |
              +-- StraightVertex / CubicVertex...  <-- vertices

Animation hierarchy:
  KeyedObject (typeKey 25)
    +-- KeyedProperty (typeKey 26)
          +-- KeyFrameUint (typeKey 450)           <-- uint keyframe
```

---

## Encoding Notes

### Property Key Summary

| Property Key | Name | Backing Type | Used By |
|-------------|------|-------------|---------|
| 32 | isClosed | UInt (bool) | PointsCommonPath, ListPath |
| 67 | frame | UInt | KeyFrameUint (inherited) |
| 68 | interpolationType | UInt | KeyFrameUint (inherited) |
| 69 | interpolatorId | UInt | KeyFrameUint (inherited) |
| 631 | value | UInt | KeyFrameUint |
| 800 | listSource | UInt | ArtboardComponentList |
| 856 | instanceWidthUnitsValue | UInt | ArtboardComponentListOverride |
| 858 | artboardId | UInt | ArtboardComponentListOverride |
| 859 | instanceWidth | Float | ArtboardComponentListOverride |
| 860 | instanceHeight | Float | ArtboardComponentListOverride |
| 861 | instanceHeightUnitsValue | UInt | ArtboardComponentListOverride |
| 862 | instanceWidthScaleType | UInt | ArtboardComponentListOverride |
| 863 | instanceHeightScaleType | UInt | ArtboardComponentListOverride |
| 874 | listSource | UInt | ListPath |
| 922 | targetId | UInt | TargetEffect |
| 934 | artboardId | UInt | ArtboardListMapRule |
| 935 | viewModelId | UInt | ArtboardListMapRule |

### Non-Runtime Properties (DO NOT encode)

| Property Key | Name | Type (design-time) | Used By |
|-------------|------|-------------------|---------|
| 276 | axisPosition | double | Guide |
| 277 | axisValue | uint | Guide |
| 753 | isClockwise | bool | PointsCommonPath |
| 917 | x | double | GroupEffect |
| 918 | y | double | GroupEffect |

### Bool Property Encoding

Property 32 (`isClosed`) is a CoreBoolType. It must be encoded as a single raw byte (0x00 or 0x01), NOT as LEB128 varuint. The same rule applies as for `isVisible(41)`, `enableWorkArea(62)`, and all other bool properties.

### Default Value Suppression

- TargetEffect: skip `targetId(922)` if value is u32::MAX (-1)
- GroupEffect: no runtime properties to suppress (only inherited name/parentId)
- ListPath: skip `isClosed(32)` if false, skip `listSource(874)` if -1
- PointsCommonPath: skip `isClosed(32)` if false
- Guide: design-time only; entire object can be omitted for runtime-only output
- ArtboardComponentList: skip `listSource(800)` if -1
- ArtboardComponentListOverride: skip properties at their default values (-1 for artboardId, -1.0 for instanceWidth/Height, 1 for units, 0 for scale types)
- ArtboardListMapRule: skip `artboardId(934)` and `viewModelId(935)` if -1
- KeyFrameUint: skip `value(631)` if 0, skip `interpolatorId(69)` if u32::MAX

---

## Acceptance Criteria

1. **TargetEffect encoding**: TargetEffect emits typeKey 644 with `targetId(922)` as UInt. The parent must be a valid component.
2. **GroupEffect encoding**: GroupEffect emits typeKey 645 with only inherited Component properties (name, parentId). Design-time properties 917 and 918 are NOT written.
3. **ListPath encoding**: ListPath emits typeKey 619 with `isClosed(32)` as UInt (bool encoding) and `listSource(874)` as UInt.
4. **PointsCommonPath encoding**: PointsCommonPath emits typeKey 620 with `isClosed(32)` as UInt (bool encoding). Design-time property 753 is NOT written.
5. **Guide handling**: Guide (typeKey 140) is a design-time object. For runtime-only output it should be omitted. If written for editor fidelity, the runtime will skip it during import.
6. **ArtboardComponentList encoding**: ArtboardComponentList emits typeKey 559 with `listSource(800)` as UInt. Inherits drawable properties.
7. **ArtboardComponentListOverride encoding**: Emits typeKey 606 with `artboardId(858)` as UInt, `instanceWidth(859)` as Float, `instanceHeight(860)` as Float, `instanceWidthUnitsValue(856)` as UInt, `instanceHeightUnitsValue(861)` as UInt, `instanceWidthScaleType(862)` as UInt, `instanceHeightScaleType(863)` as UInt.
8. **ArtboardListMapRule encoding**: Emits typeKey 648 with `artboardId(934)` as UInt and `viewModelId(935)` as UInt.
9. **KeyFrameUint encoding**: KeyFrameUint emits typeKey 450 with `frame(67)`, `interpolationType(68)`, `interpolatorId(69)`, and `value(631)` all as UInt. Follows the same encoding pattern as KeyFrameDouble/KeyFrameColor.
10. **Property key registration**: All new runtime property keys (631, 800, 856, 858, 859, 860, 861, 862, 863, 874, 922, 934, 935) must be registered in `property_backing_type()` in `core.rs`. Property 32 must be in `is_bool_property()`.
11. **Round-trip validation**: Generated .riv files containing these types pass `validate` and `inspect` without errors.
12. **Default suppression**: Properties at their default values are not written to the binary (except required properties like frame on keyframes).
13. **Hierarchy enforcement**: ArtboardListMapRule and ArtboardComponentListOverride require ArtboardComponentList as parent. ListPath requires a Shape-like parent. KeyFrameUint requires KeyedProperty as parent.
