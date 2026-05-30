# Layout Extensions

Spec for foreground layout drawable and scroll physics types.

## Type Keys

| Type | Key | C++ base |
|------|-----|----------|
| ForegroundLayoutDrawable | 513 | LayoutComponent (409) |
| ClampedScrollPhysics | 524 | ScrollPhysics (523) |
| ElasticScrollPhysics | 525 | ScrollPhysics (523) |

## Property Keys

From `generated_registry.rs`:

| Property | Key | Backing | Used by |
|----------|-----|---------|---------|
| name | 4 | String | ForegroundLayoutDrawable (inherited from Component) |
| parentId | 5 | UInt | ForegroundLayoutDrawable (inherited from Component) |
| scrollConstraintId | 725 | UInt | ClampedScrollPhysics, ElasticScrollPhysics |
| physicsId | 726 | UInt | ClampedScrollPhysics, ElasticScrollPhysics |
| physicsTypeValue | 727 | UInt | ClampedScrollPhysics, ElasticScrollPhysics |
| friction | 728 | Float | ClampedScrollPhysics, ElasticScrollPhysics |
| speedMultiplier | 729 | Float | ClampedScrollPhysics, ElasticScrollPhysics |
| elasticFactor | 730 | Float | ElasticScrollPhysics |

## Implementation Details

### ForegroundLayoutDrawable (513)

A drawable that renders in the foreground layer of a layout component. Used to draw content above the layout's children (e.g., borders, overlays).

```
Hierarchy: Component -> LayoutComponent -> ForegroundLayoutDrawable
```

Properties:
- `name` (4, String) - component name
- `parentId` (5, UInt) - parent component ID

This is a layout component variant. It inherits the same base properties as LayoutComponent (409) already implemented in `layout.rs`. May also carry layout-specific properties like `clip` (196), `width` (7), `height` (8), etc.

Implementation should follow the LayoutComponent pattern from `layout.rs`, with the same field set but a different type key.

### ClampedScrollPhysics (524)

Scroll physics that clamps scrolling at boundaries (no over-scroll bounce).

```
Hierarchy: ScrollPhysics (523) -> ClampedScrollPhysics
```

Properties:
- `friction` (728, Float) - deceleration friction coefficient
- `speedMultiplier` (729, Float) - scroll speed multiplier

Both ScrollPhysics types are children of a ScrollConstraint (521) component. They define the momentum/deceleration behavior after the user stops scrolling.

Default behavior: scrolling stops precisely at content boundaries.

### ElasticScrollPhysics (525)

Scroll physics with elastic/bouncy over-scroll behavior (iOS-style rubber-banding).

```
Hierarchy: ScrollPhysics (523) -> ElasticScrollPhysics
```

Properties:
- `friction` (728, Float) - deceleration friction coefficient
- `speedMultiplier` (729, Float) - scroll speed multiplier
- `elasticFactor` (730, Float) - bounce elasticity factor (higher = bouncier)

Extends ClampedScrollPhysics with an additional elastic factor that controls how far content can over-scroll and how it springs back.

## New Constants Needed in core.rs

### type_keys

```rust
pub const FOREGROUND_LAYOUT_DRAWABLE: u16 = 513;
pub const SCROLL_PHYSICS: u16 = 523;
pub const CLAMPED_SCROLL_PHYSICS: u16 = 524;
pub const ELASTIC_SCROLL_PHYSICS: u16 = 525;
```

### property_keys

```rust
pub const SCROLL_PHYSICS_FRICTION: u16 = 728;
pub const SCROLL_PHYSICS_SPEED_MULTIPLIER: u16 = 729;
pub const ELASTIC_SCROLL_PHYSICS_ELASTIC_FACTOR: u16 = 730;
```

## File Location

All types go in `src/objects/layout.rs` alongside LayoutComponent and LayoutComponentStyle.

## Struct Designs

### ForegroundLayoutDrawable

```rust
pub struct ForegroundLayoutDrawable {
    pub name: String,
    pub parent_id: u64,
}
```

Follows the same minimal pattern as other component types. Layout-specific properties (clip, width, height, style_id, fractional_width, fractional_height) may be added as optional fields mirroring LayoutComponent if needed.

### ClampedScrollPhysics

```rust
pub struct ClampedScrollPhysics {
    pub friction: f32,
    pub speed_multiplier: f32,
}
```

Emit properties only when non-default (friction != 0.0, speed_multiplier != 1.0).

### ElasticScrollPhysics

```rust
pub struct ElasticScrollPhysics {
    pub friction: f32,
    pub speed_multiplier: f32,
    pub elastic_factor: f32,
}
```

Same non-default emission pattern, plus elasticFactor.

## Test Coverage

Each type needs:
1. Type key assertion
2. Default properties test (empty or minimal)
3. Non-default properties test verifying correct keys, values, and backing types
4. Verify ForegroundLayoutDrawable uses same property keys as LayoutComponent base (name=4, parentId=5)
