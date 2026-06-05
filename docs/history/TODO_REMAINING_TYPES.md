# TODO: Remaining Rive Type Implementations

> Generated 2026-03-22 — All 104 missing types now have ObjectSpec, RiveObject struct, builder, and test coverage.

## Status Summary

| Metric | Count |
|--------|-------|
| Types in Rive format (generated_registry.rs) | 251 |
| Types supported before this work | ~147 (decoder/encoder) / ~66 (JSON builder) |
| Types added in this sprint | 104 new builder types |
| Types now JSON-creatable | ~170 |
| Tests | 1183 (520 lib + 520 bin + 143 e2e) |
| Clippy warnings | 0 |

## What Was Implemented

### T001: Foundation (core.rs)
- Added ~160 type_key constants, ~140 property_key constants
- Added all property_backing_type() entries
- Added 13 is_bool_property() entries for CoreBoolType properties

### T002: Asset Extensions (8 types)
- Folder, LayeredAsset, LayerImageAsset, SVGAsset, LottieAsset, ExportAudio, ScriptAsset, BlobAsset
- File: `src/objects/assets.rs`

### T003: Effects (3 types)
- DashPath, Dash, Feather
- File: `src/objects/paint.rs`

### T004: Events (9 types)
- OpenUrlEvent, AudioEvent, CustomPropertyNumber/Boolean/String/Color/Trigger/Enum, CustomPropertyGroup
- File: `src/objects/state_machine.rs`

### T005: Graphics Misc (9 types)
- TargetEffect, GroupEffect, ListPath, PointsCommonPath, Guide, ArtboardComponentList/Override, ArtboardListMapRule, KeyFrameUint
- Files: `shapes.rs`, `base.rs`, `animation.rs`

### T006: Layout Extensions (3 types)
- ForegroundLayoutDrawable, ClampedScrollPhysics, ElasticScrollPhysics
- File: `src/objects/layout.rs`

### T007: Listener & SM Extensions (9 types)
- ListenerAlignTarget, ListenerFireEvent, ListenerViewModelChange
- StateMachineFireEvent/Trigger/Action, StateMachineComponentNestedArtboard, StateMachineNestedInput
- BlendState1DViewModel
- File: `src/objects/state_machine.rs` + `src/builder/state_machines.rs`

### T008: Mesh (4 types)
- Mesh, MeshVertex, ContourMeshVertex, ForcedEdge
- File: `src/objects/mesh.rs` (new)

### T009: Nested Artboard Extensions (8 types)
- NestedLinearAnimation, NestedRemapAnimation, NestedInput/Trigger/Bool/Number
- NestedArtboardLeaf, NestedArtboardLayout
- File: `src/objects/artboard.rs`

### T010: New Constraints (4 types)
- DraggableConstraint, ScrollConstraint, ScrollBarConstraint, ListFollowPathConstraint
- File: `src/objects/constraints.rs`

### T011: NSlicer (5 types)
- NSlicerTileMode, NSlicer, AxisX, AxisY, NSlicedNode
- File: `src/objects/nslicer.rs` (new)

### T012: Data Binding Extensions (34 types)
- 14 ViewModelProperty subtypes, 5 ViewModelInstance subtypes
- 4 DataEnum types, 10 BindableProperty types, DataBindPath
- File: `src/objects/data_binding.rs`

### T013: Data Converters (26 types)
- 19 DataConverter subtypes, 7 FormulaToken types
- File: `src/objects/data_converters.rs` (new)

### T014: Scripting (13 types)
- 6 Scripted component types, 7 ScriptInput types
- File: `src/objects/scripting.rs` (new)

### T015: Text Extensions (10 types)
- TextStylePaint, TextStyleAxis, TextTargetModifier, TextFollowPathModifier
- TextInput, TextInputDrawable/Cursor/Text/Selection/SelectedText
- File: `src/objects/text.rs`

### T016: Transition Comparators (7 types)
- TransitionPropertyViewModelComparator, TransitionPropertyArtboardComparator
- TransitionArtboardCondition, TransitionSelfComparator
- TransitionValueIdComparator, TransitionValueAssetComparator, TransitionValueArtboardComparator
- File: `src/objects/state_machine.rs` + `src/builder/state_machines.rs`

---

## Remaining TODO Items

### HIGH PRIORITY — Correctness & Completeness

- [ ] **Verify property keys against C++ runtime headers**: Cross-check every new property key ID against `rive-runtime/dev/defs/` JSON definitions. Some specs may have inferred property keys from `generated_registry.rs` which could have ambiguous mappings (same property name used across different types with different keys).

- [ ] **DataConverterOperationViewModel sourcePathIds(711)**: This property uses `CoreBytesType` encoding which is NOT one of the standard 4 backing types (UInt/String/Float/Color). Currently skipped. Need to either:
  - Add Bytes backing type support to the encoder
  - Or document this as a known limitation

- [ ] **Decompile round-trip testing**: Verify that all 104 new types survive a generate → decompile → re-generate round-trip. The decompiler needs to be updated to emit JSON for the new types.

- [ ] **Default value accuracy**: Some default values in the specs were inferred. Verify against C++ runtime defaults:
  - DataConverterGroupItem converterId default (0xFFFFFFFF vs 0)
  - DataConverterInterpolator interpolatorId default
  - AudioEvent assetId default
  - Various other sentinel values

### MEDIUM PRIORITY — Builder UX

- [ ] **Enum value parsing for new types**: Several new types accept enum values as raw UInt IDs. Add human-readable string parsing (like existing `fill_rule`, `cap`, `join` support) for:
  - NSlicerTileMode `style` (stretch/tile/hidden)
  - DashPath `mode`
  - Feather `spaceValue`
  - ScrollConstraint `directionValue`
  - TransitionArtboardCondition `opValue`
  - DataConverter operation types

- [ ] **Name resolution for new reference properties**: Several new types reference other objects by ID. Add name-to-ID resolution (like existing constraint `target` resolution) for:
  - ForcedEdge `fromId`/`toId` → vertex names
  - ScrollBarConstraint `scrollConstraintId` → constraint name
  - DataConverterGroupItem `converterId` → converter name
  - ListenerAlignTarget `targetId` → node name
  - ListenerFireEvent `eventId` → event name

- [ ] **Validation rules for new parent-child constraints**:
  - DashPath must be child of Stroke
  - Dash must be child of DashPath
  - Feather must be child of Fill or Stroke
  - FormulaTokens must be children of DataConverterFormula
  - ScriptInputs must be children of Scripted components
  - MeshVertex/ContourMeshVertex/ForcedEdge must be children of Mesh
  - Mesh must be child of Image

- [ ] **Preset support**: Add presets for commonly used new type combinations:
  - Dashed stroke preset (Stroke + DashPath + Dash children)
  - Blurred shape preset (Fill/Stroke + Feather)
  - 9-slice image preset (NSlicedNode + NSlicer + Axes + TileModes)
  - Text input preset (TextInput + cursor/selection children)

### LOW PRIORITY — Polish & Documentation

- [ ] **AI generate prompt coverage**: Update the `ai generate` command's knowledge base to include the new types so AI-generated .riv files can use them.

- [ ] **Spec file documentation**: Some spec files have property keys that were inferred rather than verified. Mark these with confidence levels or verify against the C++ runtime.

- [ ] **Integration tests for complex hierarchies**: Add E2E tests that combine multiple new type domains:
  - Mesh deformation on nested artboard
  - Data-bound text input with converters
  - Scrollable layout with clamped/elastic physics
  - 9-slice with dashed strokes
  - Scripted components with data converters

- [ ] **Abstract type validation**: The generated_registry.rs has abstract types (Constraint, TargetedConstraint, DataConverter, etc.) that should never appear in user JSON. Add validation to reject abstract type keys.

- [ ] **Performance**: With ~170 ObjectSpec variants, the serde deserialization and match statements are large. Consider if this impacts compile time or runtime performance.

---

## Files Created/Modified in This Sprint

### New Files (4)
```
src/objects/mesh.rs
src/objects/nslicer.rs
src/objects/data_converters.rs
src/objects/scripting.rs
```

### New Test Fixtures (9)
```
tests/fixtures/asset_extensions.json
tests/fixtures/effects.json
tests/fixtures/events_extended.json
tests/fixtures/graphics_misc.json
tests/fixtures/layout_extensions.json
tests/fixtures/mesh.json
tests/fixtures/nested_extensions.json
tests/fixtures/new_constraints.json
tests/fixtures/nslicer.json
tests/fixtures/data_converters.json
tests/fixtures/scripting.json
```

### Modified Files
```
src/objects/core.rs          — +160 type_keys, +140 property_keys, backing types, bool props
src/objects/mod.rs           — +4 module declarations
src/objects/assets.rs        — +8 types
src/objects/paint.rs         — +3 types
src/objects/state_machine.rs — +25 types (events, listeners, SM, transitions)
src/objects/artboard.rs      — +8 types
src/objects/constraints.rs   — +4 types
src/objects/text.rs          — +10 types
src/objects/layout.rs        — +3 types
src/objects/data_binding.rs  — +34 types
src/objects/shapes.rs        — +7 types
src/objects/base.rs          — +1 type (Guide)
src/objects/animation.rs     — +1 type (KeyFrameUint)
src/builder/spec.rs          — +104 ObjectSpec variants, TransitionChildSpec, ListenerActionSpec
src/builder/objects.rs       — +104 match arms
src/builder/validation.rs    — Updated all exhaustive matches
src/builder/state_machines.rs — +10 listener/transition handlers
src/builder/scene.rs         — Image/NestedArtboard children support
```
