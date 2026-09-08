---
id: res.number-model-bindings
nodes:
  - rive-cli.intelligence.authoring
  - rive-cli.core.builder
  - rive-cli.core.objects
  - rive-cli.verification.rust
  - rive-cli.verification.browser
date: 2026-09-08
method: primary
---

# Numeric model bindings

Issue #237 / PR #238 continues the existing behavior todo under #175.
Fixed base: `a1f88b18b2f6f43fd2402c53581ba0bf43f1356c`.

## Observed verification

- Run `34251480151`, head `4eaa83a306d5153ad10db264ac7d3451a999693a`:
  the test compiled and failed because model kind number was unknown.
- Run `34251879833`, executed source `bdf7bffd6a349fae4399a5eb0c7301b853944afd`:
  the first contract, formatting, Clippy and full Rust suite passed. Downloaded
  artifact `10066439736` pins the executed source and clean working tree.
- Run `34252234877`, head `10d2ac01b3a8814328b6a2da8b24309667a9e841`:
  the original contract passed; four new invalid-kind/condition contracts failed.
  Artifact finalization separately failed with HTTP 403 in this and the first red
  run; job logs `102149214532` and `102146722769` retain the behavioral failures.
- Run `34253194379`: kind guards and full Rust checks passed, but runtime lookup
  failed for the numeric model property. Artifact `10066948039` retains the cause.
- Run `34253812557`, executed source `0b3a35d9b5eae062046a87122de4c0fc91341811`:
  thirteen contracts passed; name-field and canonical overflow contracts failed.
  Downloaded artifact `10067059056` was inspected before those fixes.
- Run `34254207250`, executed source `600052b77ae3f4eddd864ef7f0def4011154df12`:
  all fifteen contracts, formatting, all-target/all-feature Clippy, locked
  all-feature Rust tests and the numeric model runtime proof passed. Downloaded
  artifact `10067304424` retains source, logs, binary, six PNGs and evidence JSON.
  All six PNG hashes were checked. Panel centres are 39.5px initially, after
  input-only mutation and at model 59; 159.5px at model 60 and 90; 39.5px after
  reversal to 59. The synthesized input remains 90 during those model changes.

Final exact-head CI/MSRV and Standards/Spec review are recorded in PR #238; these
intermediate results are not a substitute for the final merge gate.

## Pinned runtime format provenance

Official `rive-app/rive-runtime` commit:
`25a2dc10786955df879ebc295e7c67923b8bde90`.

- `include/rive/generated/viewmodel/viewmodel_property_number_base.hpp`: type 431
  inherits ViewModelProperty/ViewModelComponent, not Component.
- `include/rive/generated/viewmodel/viewmodel_component_base.hpp`: name property
  557. Numeric properties previously wrote Component fields 4/5, leaving their
  names undiscoverable. Corrected emission follows the boolean property pattern.
  The legacy public parent_id field remains for API compatibility but is not
  serialized, as with the boolean type.
- `include/rive/generated/data_bind/bindable_property_number_base.hpp`: type 473,
  value field 636.
- `include/rive/generated/animation/transition_value_number_comparator_base.hpp`:
  type 484, value field 652.
- `src/animation/transition_viewmodel_condition.cpp`: model-backed numeric
  comparands. Existing object types and vendored runtime assets are reused.

The authored property value initializes the synthesized input only. The host
initializes and binds its model instance. This slice does not synchronize inputs,
add model-bound blends or conversions, or complete the broader behavior roadmap.
Local cloning failed DNS resolution and local Cargo is unavailable. Source review
and JavaScript syntax checks are local; Rust/browser execution is in Actions.
