# New Constraint Types

Spec for constraint types not yet implemented in `src/objects/constraints.rs`.

## Existing Constraints (already implemented)

| Type | typeKey | Inherits |
|------|---------|----------|
| Constraint | 79 | Component |
| TargetedConstraint | 80 | Constraint |
| IKConstraint | 81 | TargetedConstraint |
| DistanceConstraint | 82 | TargetedConstraint |
| TransformConstraint | 83 | TargetedConstraint |
| TransformComponentConstraint | 85 | TransformSpaceConstraint |
| TranslationConstraint | 87 | TransformComponentConstraintY |
| ScaleConstraint | 88 | TransformComponentConstraintY |
| RotationConstraint | 89 | TransformComponentConstraint |
| FollowPathConstraint | 165 | TargetedConstraint, TransformSpaceConstraint |

## New Types

---

### DraggableConstraint

**typeKey: 520**

Makes a node draggable at runtime (user can click/drag to reposition). Child of a Node.

#### Inheritance

```
Component (name=4, parentId=5)
  -> Constraint (strength=172)
    -> DraggableConstraint
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Inherited from Component (artboard-local index) |
| strength | 172 | Float | 1.0 | Inherited from Constraint |
| directionValue | 722 | UInt | 0 | Direction of allowed drag: 0=both, 1=horizontal, 2=vertical |

#### Implementation Notes

- No target — this is NOT a TargetedConstraint. It acts on the parent node itself.
- The `directionValue` property (722) controls which axes allow dragging.
- Omit properties at default values (strength=1.0, directionValue=0).

#### Struct Skeleton

```rust
pub struct DraggableConstraint {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,       // Constraint::strength (172), default 1.0
    pub direction_value: u64, // directionValue (722), default 0
}
```

---

### ScrollConstraint

**typeKey: 521**

Adds scroll behavior to a node/layout. Enables scrollable content areas.

#### Inheritance

```
Component (name=4, parentId=5)
  -> Constraint (strength=172)
    -> ScrollConstraint
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Inherited from Component |
| strength | 172 | Float | 1.0 | Inherited from Constraint |
| directionValue | 722 | UInt | 0 | Scroll direction: 0=both, 1=horizontal, 2=vertical |
| snap | 724 | UInt | 0 | Bool — whether scroll snaps to items |
| physicsId | 726 | UInt | 0 | Reference to a ScrollPhysics object (0 = none) |
| scrollOffsetX | 759 | Float | 0.0 | Initial horizontal scroll offset |
| scrollOffsetY | 760 | Float | 0.0 | Initial vertical scroll offset |
| scrollPercentX | 761 | Float | 0.0 | Horizontal scroll percent (0.0-1.0) |
| scrollPercentY | 762 | Float | 0.0 | Vertical scroll percent (0.0-1.0) |
| scrollIndex | 763 | Float | 0.0 | Scroll index for snapping mode |

#### Implementation Notes

- No target — acts on the parent node.
- `snap` (724) is a CoreBoolType — encode as single raw byte, not LEB128.
- `physicsId` references a child ScrollPhysics (typeKey 523), ClampedScrollPhysics (524), or ElasticScrollPhysics (525) object.
- Scroll offset/percent/index are mutually exclusive conceptually but all can be serialized.
- Omit properties at default values.

#### Related Types (not in this spec — lower priority)

- ScrollPhysics (typeKey: 523) — base physics for scroll
- ClampedScrollPhysics (typeKey: 524) — clamped scroll physics
- ElasticScrollPhysics (typeKey: 525) — elastic/bounce scroll physics

These physics types have properties:
- physicsTypeValue (727, UInt)
- friction (728, Float)
- speedMultiplier (729, Float)
- elasticFactor (730, Float)

#### Struct Skeleton

```rust
pub struct ScrollConstraint {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,         // 172, default 1.0
    pub direction_value: u64,  // 722, default 0
    pub snap: bool,            // 724, default false
    pub physics_id: u64,       // 726, default 0
    pub scroll_offset_x: f32,  // 759, default 0.0
    pub scroll_offset_y: f32,  // 760, default 0.0
    pub scroll_percent_x: f32, // 761, default 0.0
    pub scroll_percent_y: f32, // 762, default 0.0
    pub scroll_index: f32,     // 763, default 0.0
}
```

---

### ScrollBarConstraint

**typeKey: 522**

Scroll bar linked to a ScrollConstraint. Provides a visual scroll indicator/handle.

#### Inheritance

```
Component (name=4, parentId=5)
  -> Constraint (strength=172)
    -> ScrollBarConstraint
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Inherited from Component |
| strength | 172 | Float | 1.0 | Inherited from Constraint |
| scrollConstraintId | 725 | UInt | 0 | Reference to the ScrollConstraint this bar controls |
| autoSize | 734 | UInt | 0 | Bool — whether bar auto-sizes to content ratio |

#### Implementation Notes

- `scrollConstraintId` (725) references a ScrollConstraint (typeKey 521) by artboard-local index.
- `autoSize` (734) is a CoreBoolType — encode as single raw byte.
- The scroll bar is typically a child of a node that serves as the scroll track.
- Omit properties at default values.

#### Struct Skeleton

```rust
pub struct ScrollBarConstraint {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,             // 172, default 1.0
    pub scroll_constraint_id: u64, // 725, default 0
    pub auto_size: bool,           // 734, default false
}
```

---

### ListFollowPathConstraint

**typeKey: 625**

Constrains list items to follow a path. Positions items from a list along a path shape.

#### Inheritance

```
Component (name=4, parentId=5)
  -> Constraint (strength=172)
    -> TargetedConstraint (targetId=173)
      -> ListFollowPathConstraint
```

#### Properties

| Property | propertyKey | Backing Type | Default | Notes |
|----------|-------------|--------------|---------|-------|
| name | 4 | String | "" | Inherited from Component |
| parentId | 5 | UInt | - | Inherited from Component |
| strength | 172 | Float | 1.0 | Inherited from Constraint |
| targetId | 173 | UInt | 0xFFFFFFFF | Inherited from TargetedConstraint — path to follow |
| orient | 782 | UInt | 0 | Bool — orient items along path tangent |
| start | 783 | Float | 0.0 | Start position along path (0.0-1.0) |
| end | 784 | Float | 1.0 | End position along path (0.0-1.0) |
| listSource | 800 | UInt | 0 | Reference to list data source |
| distanceEnd | 888 | Float | 0.0 | Distance end value |
| distanceOffset | 889 | Float | 0.0 | Distance offset along path |
| randomModeValue | 887 | UInt | 0 | Random distribution mode |

#### Implementation Notes

- This IS a TargetedConstraint — it has a targetId pointing to the Path the items follow.
- `orient` (782) is a CoreBoolType — encode as single raw byte.
- `start`/`end` define the parametric range of the path to distribute items along.
- `listSource` (800) references the list providing the items.
- Omit properties at default values (strength=1.0, targetId=0xFFFFFFFF, start=0.0, end=1.0, etc.).

#### Struct Skeleton

```rust
pub struct ListFollowPathConstraint {
    pub name: String,
    pub parent_id: u64,
    pub strength: f32,          // 172, default 1.0
    pub target_id: u64,         // 173, default u32::MAX
    pub orient: bool,           // 782, default false
    pub start: f32,             // 783, default 0.0
    pub end: f32,               // 784, default 1.0
    pub list_source: u64,       // 800, default 0
    pub distance_end: f32,      // 888, default 0.0
    pub distance_offset: f32,   // 889, default 0.0
    pub random_mode_value: u64, // 887, default 0
}
```

---

## Property Key Summary (new keys not in core.rs)

| Constant Name | Key | Backing | Used By |
|---------------|-----|---------|---------|
| DRAGGABLE_CONSTRAINT_DIRECTION_VALUE / SCROLL_CONSTRAINT_DIRECTION_VALUE | 722 | UInt | DraggableConstraint, ScrollConstraint |
| SCROLL_CONSTRAINT_SNAP | 724 | UInt (bool) | ScrollConstraint |
| SCROLL_BAR_CONSTRAINT_SCROLL_CONSTRAINT_ID | 725 | UInt | ScrollBarConstraint |
| SCROLL_CONSTRAINT_PHYSICS_ID | 726 | UInt | ScrollConstraint |
| SCROLL_BAR_CONSTRAINT_AUTO_SIZE | 734 | UInt (bool) | ScrollBarConstraint |
| SCROLL_CONSTRAINT_SCROLL_OFFSET_X | 759 | Float | ScrollConstraint |
| SCROLL_CONSTRAINT_SCROLL_OFFSET_Y | 760 | Float | ScrollConstraint |
| SCROLL_CONSTRAINT_SCROLL_PERCENT_X | 761 | Float | ScrollConstraint |
| SCROLL_CONSTRAINT_SCROLL_PERCENT_Y | 762 | Float | ScrollConstraint |
| SCROLL_CONSTRAINT_SCROLL_INDEX | 763 | Float | ScrollConstraint |
| LIST_FOLLOW_PATH_CONSTRAINT_ORIENT | 782 | UInt (bool) | ListFollowPathConstraint |
| LIST_FOLLOW_PATH_CONSTRAINT_START | 783 | Float | ListFollowPathConstraint |
| LIST_FOLLOW_PATH_CONSTRAINT_END | 784 | Float | ListFollowPathConstraint |
| LIST_FOLLOW_PATH_CONSTRAINT_LIST_SOURCE | 800 | UInt | ListFollowPathConstraint |
| LIST_FOLLOW_PATH_CONSTRAINT_RANDOM_MODE_VALUE | 887 | UInt | ListFollowPathConstraint |
| LIST_FOLLOW_PATH_CONSTRAINT_DISTANCE_END | 888 | Float | ListFollowPathConstraint |
| LIST_FOLLOW_PATH_CONSTRAINT_DISTANCE_OFFSET | 889 | Float | ListFollowPathConstraint |

## Type Key Summary

| Constant Name | Key |
|---------------|-----|
| DRAGGABLE_CONSTRAINT | 520 |
| SCROLL_CONSTRAINT | 521 |
| SCROLL_BAR_CONSTRAINT | 522 |
| LIST_FOLLOW_PATH_CONSTRAINT | 625 |
