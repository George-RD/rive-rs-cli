# Implementation Plan: ~104 Missing Rive Object Types

## Overview

This plan adds ~104 missing Rive object types across 15 domain areas. The work is organized into 17 tasks: one foundation task (T001) for all core.rs constants, 15 domain implementation tasks (T002-T016) that can be parallelized after T001, and one final integration/validation task (T017).

### Task Dependency Graph

```
T001 (Foundation: core.rs constants)
  |
  +-- T002 (Assets)              -- can parallelize
  +-- T003 (Effects)             -- can parallelize
  +-- T004 (Events)              -- can parallelize
  +-- T005 (Graphics Misc)       -- can parallelize
  +-- T006 (Layout Extensions)   -- can parallelize
  +-- T007 (Listener/SM Ext)     -- can parallelize
  +-- T008 (Mesh)                -- can parallelize
  +-- T009 (Nested Artboard Ext) -- can parallelize
  +-- T010 (New Constraints)     -- can parallelize
  +-- T011 (NSlicer)             -- can parallelize
  +-- T012 (Data Binding Ext)    -- can parallelize
  +-- T013 (Data Converters)     -- can parallelize
  +-- T014 (Scripting)           -- can parallelize
  +-- T015 (Text Extensions)     -- can parallelize
  +-- T016 (Transition Comp.)    -- can parallelize
  |
  v
T017 (Integration & Validation)  -- depends on ALL above
```

---

### T001: Foundation -- All type_keys, property_keys, property_backing_type, is_bool_property

**Description:** Add ALL new constants for all ~104 types across all 15 spec files to `src/objects/core.rs`. This is the single foundation task that every domain task depends on.

**Types included:** None (constants only -- no struct implementations)

**Files to modify:**
- `src/objects/core.rs` -- type_keys module (~80 new constants), property_keys module (~120 new constants), `property_backing_type()` function, `is_bool_property()` function

**Work breakdown:**

1. **type_keys** -- Add constants from ALL spec files:
   - Assets (8): FOLDER(102), LAYERED_ASSET(119), LAYER_IMAGE_ASSET(120), SVG_ASSET(132), LOTTIE_ASSET(133), EXPORT_AUDIO(422), SCRIPT_ASSET(529), BLOB_ASSET(649)
   - Effects (3): DASH_PATH(506), DASH(507), FEATHER(533)
   - Events (9): OPEN_URL_EVENT(131), AUDIO_EVENT(407), CUSTOM_PROPERTY(167), CUSTOM_PROPERTY_NUMBER(127), CUSTOM_PROPERTY_BOOLEAN(129), CUSTOM_PROPERTY_STRING(130), CUSTOM_PROPERTY_COLOR(592), CUSTOM_PROPERTY_TRIGGER(613), CUSTOM_PROPERTY_ENUM(616), CUSTOM_PROPERTY_GROUP(548)
   - Graphics Misc (9): TARGET_EFFECT(644), GROUP_EFFECT(645), LIST_PATH(619), POINTS_COMMON_PATH(620), GUIDE(140), ARTBOARD_COMPONENT_LIST(559), ARTBOARD_COMPONENT_LIST_OVERRIDE(606), ARTBOARD_LIST_MAP_RULE(648), KEY_FRAME_UINT(450) -- already exists, verify
   - Layout (4): FOREGROUND_LAYOUT_DRAWABLE(513), SCROLL_PHYSICS(523), CLAMPED_SCROLL_PHYSICS(524), ELASTIC_SCROLL_PHYSICS(525)
   - Listener/SM (9): LISTENER_ALIGN_TARGET(126), LISTENER_FIRE_EVENT(168), LISTENER_VIEW_MODEL_CHANGE(487), STATE_MACHINE_FIRE_EVENT(169), STATE_MACHINE_FIRE_TRIGGER(614), STATE_MACHINE_FIRE_ACTION(615), STATE_MACHINE_COMPONENT_NESTED_ARTBOARD(172), STATE_MACHINE_NESTED_INPUT(173), BLEND_STATE_1D_VIEW_MODEL(528)
   - Mesh (4): MESH_VERTEX(108), MESH(109), CONTOUR_MESH_VERTEX(111), FORCED_EDGE(112)
   - Nested Artboard (8): NESTED_LINEAR_ANIMATION(97), NESTED_REMAP_ANIMATION(98), NESTED_INPUT(121), NESTED_TRIGGER(122), NESTED_BOOL(123), NESTED_NUMBER(124), NESTED_ARTBOARD_LEAF(451), NESTED_ARTBOARD_LAYOUT(452)
   - Constraints (4): DRAGGABLE_CONSTRAINT(520), SCROLL_CONSTRAINT(521), SCROLL_BAR_CONSTRAINT(522), LIST_FOLLOW_PATH_CONSTRAINT(625)
   - NSlicer (5): NSLICER_TILE_MODE(491), NSLICER(493), AXIS_Y(494), AXIS_X(495), N_SLICED_NODE(508)
   - Data Binding (34): VIEW_MODEL_PROPERTY_NUMBER(431), VIEW_MODEL_PROPERTY_LIST(434), DATA_ENUM_CUSTOM(438), VIEW_MODEL_PROPERTY_ENUM_CUSTOM(439), VIEW_MODEL_PROPERTY_COLOR(440), VIEW_MODEL_INSTANCE_LIST(441) -- verify, VIEW_MODEL_PROPERTY_STRING(443), DATA_ENUM_VALUE(445), VIEW_MODEL_PROPERTY_BOOLEAN(448), VIEW_MODEL_INSTANCE_TRIGGER(501), VIEW_MODEL_PROPERTY_TRIGGER(502), BINDABLE_PROPERTY_TRIGGER(503), VIEW_MODEL_PROPERTY_ENUM(509), DATA_ENUM(510), VIEW_MODEL_PROPERTY_ENUM_SYSTEM(511), DATA_ENUM_SYSTEM(512), VIEW_MODEL_PROPERTY_SYMBOL(563), VIEW_MODEL_PROPERTY_SYMBOL_LIST_INDEX(564), VIEW_MODEL_INSTANCE_SYMBOL(565), VIEW_MODEL_INSTANCE_SYMBOL_LIST_INDEX(566), BINDABLE_PROPERTY_INTEGER(567), VIEW_MODEL_PROPERTY_ASSET_IMAGE(585), VIEW_MODEL_INSTANCE_ASSET_IMAGE(587), BINDABLE_PROPERTY_LIST(590), BINDABLE_PROPERTY_ID(596), BINDABLE_PROPERTY_ARTBOARD(597), VIEW_MODEL_PROPERTY_ARTBOARD(598), VIEW_MODEL_INSTANCE_ARTBOARD(599), DATA_BIND_PATH(643), BINDABLE_PROPERTY_STRING(471), BINDABLE_PROPERTY_BOOLEAN(472), BINDABLE_PROPERTY_NUMBER(473), BINDABLE_PROPERTY_ENUM(474), BINDABLE_PROPERTY_COLOR(475), VIEW_MODEL_PROPERTY_VIEW_MODEL(436) -- verify collision with existing
   - Data Converters (30): DATA_CONVERTER(488), DATA_CONVERTER_ROUNDER(489), DATA_CONVERTER_TO_STRING(490), DATA_CONVERTER_GROUP_ITEM(498), DATA_CONVERTER_GROUP(499), DATA_CONVERTER_OPERATION_VALUE(500), DATA_CONVERTER_TRIGGER(504), DATA_CONVERTER_SYSTEM_DEGS_TO_RADS(514), DATA_CONVERTER_SYSTEM_NORMALIZER(515), DATA_CONVERTER_OPERATION(516), DATA_CONVERTER_OPERATION_VIEW_MODEL(517), DATA_CONVERTER_RANGE_MAPPER(519), DATA_CONVERTER_STRING_PAD(530), DATA_CONVERTER_STRING_REMOVE_ZEROS(531), DATA_CONVERTER_STRING_TRIM(532), DATA_CONVERTER_INTERPOLATOR(534), DATA_CONVERTER_BOOLEAN_NEGATE(535), DATA_CONVERTER_FORMULA(536), FORMULA_TOKEN(537), FORMULA_TOKEN_ARGUMENT_SEPARATOR(538), FORMULA_TOKEN_PARENTHESIS(539), FORMULA_TOKEN_PARENTHESIS_CLOSE(540), FORMULA_TOKEN_OPERATION(541), FORMULA_TOKEN_FUNCTION(542), FORMULA_TOKEN_VALUE(543), FORMULA_TOKEN_PARENTHESIS_OPEN(544), FORMULA_TOKEN_INPUT(545), DATA_CONVERTER_NUMBER_TO_LIST(568), DATA_CONVERTER_LIST_TO_LENGTH(591), DATA_CONVERTER_TO_NUMBER(617)
   - Scripting (13): SCRIPTED_DRAWABLE(603), SCRIPT_INPUT_NUMBER(611), SCRIPT_INPUT_VIEW_MODEL_PROPERTY(612), SCRIPTED_DATA_CONVERTER(629), SCRIPT_INPUT_TRIGGER(618), SCRIPT_INPUT_ARTBOARD(621), SCRIPT_INPUT_COLOR(626), SCRIPT_INPUT_STRING(627), SCRIPT_INPUT_BOOLEAN(631), SCRIPTED_LAYOUT(637), SCRIPTED_PATH_EFFECT(640), SCRIPTED_LISTENER_ACTION(646), SCRIPTED_TRANSITION_CONDITION(647)
   - Text (10): TEXT_STYLE_PAINT(137), TEXT_STYLE_AXIS(144), TEXT_TARGET_MODIFIER(546), TEXT_FOLLOW_PATH_MODIFIER(547), TEXT_INPUT(569), TEXT_INPUT_DRAWABLE(570), TEXT_INPUT_CURSOR(571), TEXT_INPUT_TEXT(572), TEXT_INPUT_SELECTION(574), TEXT_INPUT_SELECTED_TEXT(575)
   - Transition Comparators (7): TRANSITION_PROPERTY_VIEW_MODEL_COMPARATOR(479), TRANSITION_PROPERTY_ARTBOARD_COMPARATOR(496), TRANSITION_ARTBOARD_CONDITION(497), TRANSITION_SELF_COMPARATOR(593), TRANSITION_VALUE_ID_COMPARATOR(601), TRANSITION_VALUE_ASSET_COMPARATOR(602), TRANSITION_VALUE_ARTBOARD_COMPARATOR(630)

2. **property_keys** -- Add ALL new property key constants from every spec file (see each spec's "Property Key Summary" section)

3. **property_backing_type()** -- Register every new property key with its correct backing type (String/Float/UInt/Color)

4. **is_bool_property()** -- Add all new CoreBoolType properties: 238(nestedBoolValue), 676(normalized), 691(offsetIsPercentage), 693(lengthIsPercentage), 724(snap), 734(autoSize), 752(inner/feather), 782(orient), 891(interactive), 895(isPaused), 921(isRelative), 951(isStateful)

**Acceptance criteria:**
- `cargo build` succeeds
- `cargo test` passes (existing tests still pass)
- `cargo clippy -- -D warnings` clean
- Every type_key and property_key constant from all 15 specs is present
- `property_backing_type()` returns `Some(...)` for every new runtime property key
- `is_bool_property()` returns true for all new CoreBoolType properties

**Dependencies:** None

---

### T002: Asset Extensions

**Description:** Implement 8 asset types: Folder, LayeredAsset, LayerImageAsset, SVGAsset, LottieAsset, ExportAudio, ScriptAsset, BlobAsset.

**Spec:** `specs/assets_extensions.md`

**Types (8):** Folder(102), LayeredAsset(119), LayerImageAsset(120), SVGAsset(132), LottieAsset(133), ExportAudio(422), ScriptAsset(529), BlobAsset(649)

**Files to modify:**
- `src/objects/assets.rs` -- Add 8 structs with RiveObject impls
- `src/objects/mod.rs` -- Ensure exports
- `src/builder/spec.rs` -- Add 8 ObjectSpec variants
- `src/builder/objects.rs` -- Add 8 match arms in append_object()
- `tests/fixtures/` -- Add test fixture JSON
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- All 8 types encode with correct type keys
- FileAsset-derived types emit name(203), assetId(204), cdnBaseUrl(362)
- Folder uses Component properties (name=4, parentId=5), NOT Asset properties
- ExportAudio emits volume(530) only when != 1.0
- ScriptAsset emits isModule(914) when true
- Unit tests for each struct
- E2E generate+validate round-trip

**Dependencies:** T001

---

### T003: Dash & Feather Effects

**Description:** Implement 3 paint effect types: DashPath, Dash, Feather.

**Spec:** `specs/effects.md`

**Types (3):** DashPath(506), Dash(507), Feather(533)

**Files to modify:**
- `src/objects/paint.rs` (or new `src/objects/effects.rs`) -- Add 3 structs
- `src/objects/mod.rs` -- Ensure exports
- `src/builder/spec.rs` -- Add 3 ObjectSpec variants (dash_path, dash, feather)
- `src/builder/objects.rs` -- Add 3 match arms, enforce parent constraints (DashPath under Stroke, Feather under Fill/Stroke, Dash under DashPath)
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- DashPath emits offset(690), offsetIsPercentage(691)
- Dash emits length(692), lengthIsPercentage(693)
- Feather emits strength(749), offsetX(750), offsetY(751), spaceValue(748), inner(752)
- Parent constraint enforcement (DashPath requires Stroke parent)
- Default value suppression
- E2E round-trip

**Dependencies:** T001

---

### T004: Event System Extensions

**Description:** Implement 9 event and custom property types: OpenUrlEvent, AudioEvent, CustomPropertyNumber/Boolean/String/Color/Trigger/Enum, CustomPropertyGroup.

**Spec:** `specs/events.md`

**Types (9):** OpenUrlEvent(131), AudioEvent(407), CustomPropertyNumber(127), CustomPropertyBoolean(129), CustomPropertyString(130), CustomPropertyColor(592), CustomPropertyTrigger(613), CustomPropertyEnum(616), CustomPropertyGroup(548)

**Files to modify:**
- `src/objects/state_machine.rs` (or new `src/objects/events.rs`) -- Add 9 structs
- `src/objects/mod.rs` -- Ensure exports
- `src/builder/spec.rs` -- Add 9 ObjectSpec variants
- `src/builder/objects.rs` -- Add 9 match arms, including child recursion for container types
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- OpenUrlEvent emits url(248), targetValue(249)
- AudioEvent emits assetId(408)
- CustomPropertyBoolean encodes propertyValue(245) as UInt (NOT CoreBoolType)
- CustomPropertyColor encodes propertyValue(836) as Color
- Container types (OpenUrlEvent, AudioEvent, CustomPropertyGroup) support children
- E2E round-trip

**Dependencies:** T001

---

### T005: Graphics Miscellaneous

**Description:** Implement 9 miscellaneous types: TargetEffect, GroupEffect, ListPath, PointsCommonPath, Guide, ArtboardComponentList, ArtboardComponentListOverride, ArtboardListMapRule, KeyFrameUint.

**Spec:** `specs/graphics_misc.md`

**Types (9):** TargetEffect(644), GroupEffect(645), ListPath(619), PointsCommonPath(620), Guide(140), ArtboardComponentList(559), ArtboardComponentListOverride(606), ArtboardListMapRule(648), KeyFrameUint(450)

**Files to modify:**
- `src/objects/shapes.rs` or `src/objects/base.rs` -- Add TargetEffect, GroupEffect, ListPath, PointsCommonPath, Guide structs
- `src/objects/data_binding.rs` or new file -- Add ArtboardComponentList, ArtboardComponentListOverride, ArtboardListMapRule
- `src/objects/animation.rs` -- Add KeyFrameUint
- `src/builder/spec.rs` -- Add ObjectSpec variants
- `src/builder/objects.rs` -- Add match arms
- `src/builder/scene.rs` -- Add KeyFrameUint handling in animation builder
- `tests/fixtures/` -- Add test fixtures
- `tests/e2e.rs` -- Add integration tests

**Acceptance criteria:**
- GroupEffect does NOT emit design-time properties (917, 918)
- PointsCommonPath does NOT emit isClockwise(753)
- Guide is design-time only; encoded for editor fidelity but skipped by runtime
- isClosed(32) on ListPath/PointsCommonPath encodes as CoreBoolType (raw byte)
- KeyFrameUint emits value(631) as UInt
- E2E round-trip

**Dependencies:** T001

---

### T006: Layout Extensions

**Description:** Implement 3 layout types: ForegroundLayoutDrawable, ClampedScrollPhysics, ElasticScrollPhysics.

**Spec:** `specs/layout_extensions.md`

**Types (3):** ForegroundLayoutDrawable(513), ClampedScrollPhysics(524), ElasticScrollPhysics(525)

**Files to modify:**
- `src/objects/layout.rs` -- Add 3 structs
- `src/builder/spec.rs` -- Add 3 ObjectSpec variants
- `src/builder/objects.rs` -- Add 3 match arms
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- ForegroundLayoutDrawable emits with typeKey 513, inherits Component properties
- ClampedScrollPhysics emits friction(728), speedMultiplier(729)
- ElasticScrollPhysics also emits elasticFactor(730)
- Default suppression
- E2E round-trip

**Dependencies:** T001

---

### T007: Listener & State Machine Extensions

**Description:** Implement 9 listener/SM types: ListenerAlignTarget, ListenerFireEvent, ListenerViewModelChange, StateMachineFireEvent/Trigger/Action, StateMachineComponentNestedArtboard, StateMachineNestedInput, BlendState1DViewModel.

**Spec:** `specs/listener_sm_extensions.md`

**Types (9):** ListenerAlignTarget(126), ListenerFireEvent(168), ListenerViewModelChange(487), StateMachineFireEvent(169), StateMachineFireTrigger(614), StateMachineFireAction(615), StateMachineComponentNestedArtboard(172), StateMachineNestedInput(173), BlendState1DViewModel(528)

**Files to modify:**
- `src/objects/state_machine.rs` -- Add 9 structs
- `src/builder/spec.rs` -- Add relevant ObjectSpec / ListenerActionSpec / StateSpec variants
- `src/builder/objects.rs` and/or `src/builder/state_machines.rs` -- Add match arms
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- Listener action types follow existing ListenerBoolChange pattern (no name/parentId)
- StateMachine types follow existing StateMachineComponent pattern (name=138)
- StateMachineFireEvent emits eventId(392), occursValue(393)
- BlendState1DViewModel is a zero-property type (type key only)
- E2E round-trip

**Dependencies:** T001

---

### T008: Mesh Deformation

**Description:** Implement 4 mesh types: Mesh, MeshVertex, ContourMeshVertex, ForcedEdge. Extend Image ObjectSpec to support children.

**Spec:** `specs/mesh.md`

**Types (4):** MeshVertex(108), Mesh(109), ContourMeshVertex(111), ForcedEdge(112)

**Files to modify:**
- New `src/objects/mesh.rs` -- Add 4 structs
- `src/objects/mod.rs` -- Register new module
- `src/builder/spec.rs` -- Add 4 ObjectSpec variants, extend Image variant with children, add ParentKind::Mesh
- `src/builder/objects.rs` -- Add 4 match arms, name resolution for ForcedEdge from_vertex/to_vertex
- `tests/fixtures/mesh.json` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- MeshVertex and ContourMeshVertex emit u(215), v(216) as Float
- ForcedEdge emits fromId(219), toId(220) as UInt via name resolution
- Mesh is a container (name+parentId only)
- Image ObjectSpec now supports children
- Hierarchy: Mesh under Image, vertices/edges under Mesh
- E2E round-trip

**Dependencies:** T001

---

### T009: Nested Artboard Extensions

**Description:** Implement 8 nested artboard types and extend existing NestedArtboard with children support.

**Spec:** `specs/nested_artboard_extensions.md`

**Types (8):** NestedLinearAnimation(97), NestedRemapAnimation(98), NestedInput(121), NestedTrigger(122), NestedBool(123), NestedNumber(124), NestedArtboardLeaf(451), NestedArtboardLayout(452)

**Files to modify:**
- `src/objects/artboard.rs` -- Add 8 structs
- `src/builder/spec.rs` -- Add 8 ObjectSpec variants, extend NestedArtboard with children
- `src/builder/objects.rs` -- Add 8 match arms, animation name resolution for nested animations
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- NestedBool nestedValue(238) encodes as CoreBoolType (raw byte)
- NestedRemapAnimation emits time(202) as Float
- NestedArtboardLeaf/Layout resolve source_artboard with same cycle-detection as NestedArtboard
- NestedArtboardLayout emits layout properties (width, height, clip, styleId, etc.)
- Default suppression (time=0.0, mix=1.0, etc.)
- E2E round-trip

**Dependencies:** T001

---

### T010: New Constraints

**Description:** Implement 4 constraint types: DraggableConstraint, ScrollConstraint, ScrollBarConstraint, ListFollowPathConstraint.

**Spec:** `specs/new_constraints.md`

**Types (4):** DraggableConstraint(520), ScrollConstraint(521), ScrollBarConstraint(522), ListFollowPathConstraint(625)

**Files to modify:**
- `src/objects/constraints.rs` -- Add 4 structs
- `src/builder/spec.rs` -- Add 4 ObjectSpec variants
- `src/builder/objects.rs` -- Add 4 match arms
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- DraggableConstraint emits strength(172), directionValue(722)
- ScrollConstraint emits snap(724) as CoreBoolType (raw byte)
- ScrollBarConstraint emits autoSize(734) as CoreBoolType
- ListFollowPathConstraint emits orient(782) as CoreBoolType, is a TargetedConstraint with targetId(173)
- Default suppression (strength=1.0, etc.)
- E2E round-trip

**Dependencies:** T001

---

### T011: 9-Slice (NSlicer)

**Description:** Implement 5 NSlicer types: NSlicerTileMode, NSlicer, AxisY, AxisX, NSlicedNode.

**Spec:** `specs/nslicer.md`

**Types (5):** NSlicerTileMode(491), NSlicer(493), AxisY(494), AxisX(495), NSlicedNode(508)

**Files to modify:**
- New `src/objects/nslicer.rs` (or add to shapes.rs) -- Add 5 structs
- `src/objects/mod.rs` -- Register module if new file
- `src/builder/spec.rs` -- Add 5 ObjectSpec variants (nslicer, axis_x, axis_y, nslicer_tile_mode, n_sliced_node)
- `src/builder/objects.rs` -- Add 5 match arms
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- AxisX/AxisY share property keys: offset(675), normalized(676)
- normalized(676) encodes as CoreBoolType (raw byte)
- NSlicer emits initialWidth(697), initialHeight(698), width(699), height(700) as Float
- NSlicerTileMode emits patchIndex(672) as UInt, style(673) as UInt
- NSlicedNode inherits Node/TransformComponent properties
- Patch index 0-8 bounds validation
- E2E round-trip

**Dependencies:** T001

---

### T012: Data Binding Extensions

**Description:** Implement ~34 data binding types: 14 ViewModelProperty subtypes, 5 ViewModelInstance subtypes, 4 DataEnum types, 10 BindableProperty types, DataBindPath.

**Spec:** `specs/data_binding_extensions.md`

**Types (34):** ViewModelPropertyNumber(431), ViewModelPropertyBoolean(448), ViewModelPropertyString(443), ViewModelPropertyColor(440), ViewModelPropertyList(434), ViewModelPropertyViewModel(436), ViewModelPropertyEnum(509), ViewModelPropertyEnumCustom(439), ViewModelPropertyEnumSystem(511), ViewModelPropertyTrigger(502), ViewModelPropertyAssetImage(585), ViewModelPropertyArtboard(598), ViewModelPropertySymbol(563), ViewModelPropertySymbolListIndex(564), ViewModelInstanceTrigger(501), ViewModelInstanceSymbol(565), ViewModelInstanceSymbolListIndex(566), ViewModelInstanceAssetImage(587), ViewModelInstanceArtboard(599), DataEnum(510), DataEnumCustom(438), DataEnumValue(445), DataEnumSystem(512), BindablePropertyString(471), BindablePropertyBoolean(472), BindablePropertyNumber(473), BindablePropertyEnum(474), BindablePropertyColor(475), BindablePropertyTrigger(503), BindablePropertyInteger(567), BindablePropertyList(590), BindablePropertyId(596), BindablePropertyArtboard(597), DataBindPath(643)

**Files to modify:**
- `src/objects/data_binding.rs` -- Add 34 structs (follows existing patterns closely)
- `src/builder/spec.rs` -- Add 34 ObjectSpec variants
- `src/builder/objects.rs` -- Add 34 match arms
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- ViewModelProperty subtypes emit name(4), parentId(5), and subtype-specific extras
- ViewModelInstance subtypes emit viewModelPropertyId(554) and type-specific propertyValue
- DataEnumCustom uses name property 572 (NOT 4)
- DataEnumValue emits key(578), value(579) as Strings
- BindableProperty types emit their typed propertyValue property
- BindablePropertyColor(475) emits value(638) as Color
- BindablePropertyId(596) emits value(836) as Color
- E2E round-trip

**Dependencies:** T001

---

### T013: Data Converters

**Description:** Implement ~26 data converter and formula token types.

**Spec:** `specs/data_converters.md`

**Types (26):** DataConverterRounder(489), DataConverterToString(490), DataConverterGroupItem(498), DataConverterGroup(499), DataConverterOperationValue(500), DataConverterTrigger(504), DataConverterSystemDegsToRads(514), DataConverterSystemNormalizer(515), DataConverterOperationViewModel(517), DataConverterRangeMapper(519), DataConverterStringPad(530), DataConverterStringRemoveZeros(531), DataConverterStringTrim(532), DataConverterInterpolator(534), DataConverterBooleanNegate(535), DataConverterFormula(536), FormulaTokenArgumentSeparator(538), FormulaTokenParenthesisClose(540), FormulaTokenOperation(541), FormulaTokenFunction(542), FormulaTokenValue(543), FormulaTokenParenthesisOpen(544), FormulaTokenInput(545), DataConverterNumberToList(568), DataConverterListToLength(591), DataConverterToNumber(617)

**Files to modify:**
- New `src/objects/data_converters.rs` -- Add 26 structs
- `src/objects/mod.rs` -- Register module
- `src/builder/spec.rs` -- Add ObjectSpec variants
- `src/builder/objects.rs` -- Add match arms
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- All DataConverter subtypes inherit name(662) from base
- DataConverterGroupItem is NOT a DataConverter subclass (no name(662) property; has converterId(679))
- DataConverterOperationViewModel handles sourcePathIds(711) as Bytes type (special encoding, NOT in ToC)
- FormulaToken types are leaf children of DataConverterFormula
- Default value suppression per spec defaults (operationValue=1.0, duration=1.0, etc.)
- Abstract types (488, 516, 537, 539) are NOT instantiable
- E2E round-trip

**Dependencies:** T001

---

### T014: Scripting Types

**Description:** Implement 13 scripting types: 6 scripted component types and 7 script input types.

**Spec:** `specs/scripting.md`

**Types (13):** ScriptedDrawable(603), ScriptedDataConverter(629), ScriptedLayout(637), ScriptedPathEffect(640), ScriptedListenerAction(646), ScriptedTransitionCondition(647), ScriptInputNumber(611), ScriptInputViewModelProperty(612), ScriptInputTrigger(618), ScriptInputArtboard(621), ScriptInputColor(626), ScriptInputString(627), ScriptInputBoolean(631)

**Files to modify:**
- New `src/objects/scripting.rs` -- Add 13 structs
- `src/objects/mod.rs` -- Register module
- `src/builder/spec.rs` -- Add 13 ObjectSpec variants
- `src/builder/objects.rs` -- Add 13 match arms
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- Each scripted component uses its own scriptAssetId property key (848, 892, 912, 930, 931)
- ScriptedDrawable has the most properties (isPaused=895 as CoreBoolType, interactive=891 as CoreBoolType)
- ScriptInput types are simple Component-derived (name+parentId)
- ScriptedListenerAction/ScriptedTransitionCondition follow listener/transition patterns (no name/parentId)
- E2E round-trip

**Dependencies:** T001

---

### T015: Text Extensions

**Description:** Implement 10 text types: TextStylePaint, TextStyleAxis, TextTargetModifier, TextFollowPathModifier, TextInput, TextInputDrawable/Cursor/Text/Selection/SelectedText.

**Spec:** `specs/text_extensions.md`

**Types (10):** TextStylePaint(137), TextStyleAxis(144), TextTargetModifier(546), TextFollowPathModifier(547), TextInput(569), TextInputDrawable(570), TextInputCursor(571), TextInputText(572), TextInputSelection(574), TextInputSelectedText(575)

**Files to modify:**
- `src/objects/text.rs` -- Add 10 structs
- `src/builder/spec.rs` -- Add 10 ObjectSpec variants
- `src/builder/objects.rs` -- Add 10 match arms
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- TextStyleAxis emits tag(289) as UInt, axisValue(288) as Float; no name property
- TextFollowPathModifier emits orient(782) as CoreBoolType
- TextInput emits text(817) as String, interactive(891) as CoreBoolType
- TextInputDrawable/Cursor/Text/Selection/SelectedText are minimal (name+parentId only)
- TextStylePaint is a container type (name+parentId, children define paint)
- E2E round-trip

**Dependencies:** T001

---

### T016: Transition Comparators

**Description:** Implement 7 transition comparator types.

**Spec:** `specs/transition_comparators.md`

**Types (7):** TransitionPropertyViewModelComparator(479), TransitionPropertyArtboardComparator(496), TransitionArtboardCondition(497), TransitionSelfComparator(593), TransitionValueIdComparator(601), TransitionValueAssetComparator(602), TransitionValueArtboardComparator(630)

**Files to modify:**
- `src/objects/state_machine.rs` -- Add 7 structs
- `src/builder/spec.rs` -- Add TransitionChildSpec variants
- `src/builder/state_machines.rs` -- Add match arms for transition child building
- `tests/fixtures/` -- Add test fixture
- `tests/e2e.rs` -- Add integration test

**Acceptance criteria:**
- Unit structs (TransitionPropertyViewModelComparator, TransitionPropertyArtboardComparator, TransitionSelfComparator) emit empty properties
- TransitionArtboardCondition reuses opValue(650) property key
- TransitionValueIdComparator emits propertyValue(823), TransitionValueAssetComparator emits propertyValue(824), TransitionValueArtboardComparator emits propertyValue(870)
- Added to TransitionChildSpec enum for JSON deserialization
- E2E round-trip

**Dependencies:** T001

---

### T017: Integration & Validation

**Description:** Final validation pass across all domain tasks. Run full test suite, clippy, fmt, and validate all generated .riv files load in the runtime.

**Types included:** None (validation only)

**Files to modify:**
- `tests/e2e.rs` -- Add comprehensive cross-domain integration test
- `src/objects/generated_registry.rs` -- Verify all new types appear in type_name() and property_name()

**Work breakdown:**
1. Run `cargo build` -- verify clean compilation
2. Run `cargo clippy -- -D warnings` -- verify no lint warnings
3. Run `cargo fmt --check` -- verify formatting
4. Run `cargo test` -- all unit and integration tests pass
5. Generate a comprehensive .riv fixture with types from every domain
6. Run `cargo run -- validate` on all new fixtures
7. Run `cargo run -- inspect` on all new fixtures to verify object trees
8. Verify decompile round-trip for key fixtures
9. Run Playwright runtime regression if available

**Acceptance criteria:**
- All tests pass
- Clippy and fmt clean
- All new fixtures validate successfully
- No regressions in existing tests
- Cross-domain fixture (artboard with types from multiple domains) generates and validates

**Dependencies:** T001-T016 (all domain tasks)
