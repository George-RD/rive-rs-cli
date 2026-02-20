# Post-PR #26 Development Roadmap: rive-rs-cli

## TL;DR

> **Quick Summary**: Harden the rive-rs-cli foundation (CI/CD, encoder correctness, validation) in PR #27, then expand animation capability and object coverage in PR #28. This is an infrastructure-then-features sequence that prevents regression debt.
>
> **Deliverables**:
> - PR #27: GitHub Actions CI, generalized ToC encoder, boolean encoding fix, typed error model (builder), stale-doc/issue cleanup
> - PR #28: CubicInterpolator + TrimPath objects, expanded animatable property set, new fixtures with playwright regression
>
> **Estimated Effort**: Medium (each PR is ~1 day of focused work)
> **Parallel Execution**: YES — 3 waves per PR
> **Critical Path**: CI setup → ToC refactor → bool fix → e2e verify → new objects → new fixtures → playwright

---

## Context

### Original Request

Produce a concrete next-phase development plan for rive-rs-cli after PR #26 merged. Prioritized roadmap with milestones, acceptance criteria, sequencing, and risk mitigation. Focus on: (1) scene schema evolution, (2) semantic validation hardening, (3) exporter boundary design, (4) object coverage expansion, (5) verification gates.

### Current State (Post-PR #26)

- **Branch**: `main` at `4bd8171` (PR #26 merged)
- **Tests**: 133 passing (123 unit + 10 e2e), 0 failures
- **Object Coverage**: 30+ type_key constants defined, 11 types JSON-constructable via scene schema v1
- **Animatable Properties**: 9 mapped in `property_key_from_name()` (x, y, rotation, scale_x, scale_y, opacity, width, height, color)
- **Verification**: cargo test + playwright regression harness (5 fixtures × Rive WASM runtime)
- **Schema**: v1 with JSON Schema 2020-12, `unevaluatedProperties: false`

### Metis Review

**Critical finding — StateMachineLayer bug is ALREADY FIXED:**
PRs #21-25 resolved the SM sentinel state issue. The `builder/scene.rs` auto-injects AnyState and interleaves transitions correctly. Playwright regression passes for `state_machine.json`. The AGENTS.md "Active bug" note is stale.

**Boolean encoding is functionally correct:**
`write_varuint(1)` → `[0x01]` = same bytes as `write_byte(1)`. For values 0/1 the encoding is byte-identical. This is code hygiene, not a runtime blocker.

**CI/CD is the real priority:**
20 open GitHub issues, no automation. Every PR requires manual `cargo test` + `clippy` + `fmt` + playwright. CI has compound ROI for every future change.

**Lottie exporter abstraction is premature:**
`RiveObject` trait returns `type_key() -> u16` (Rive-specific). `PropertyValue` enum has exactly 4 .riv backing types. Designing abstractions before the second consumer exists produces wrong abstractions.

### Gaps Addressed

- **SM bug stale docs**: Auto-resolved — will update AGENTS.md (MINOR)
- **Lottie scope**: Auto-resolved — deferred with ADR note, no code changes (DEFAULT)
- **Schema version**: Auto-resolved — extend v1 (additive), no v2 needed yet (DEFAULT)
- **thiserror migration scope**: Auto-resolved — scope to builder/scene.rs only (DEFAULT)
- **CubicInterpolator key verification**: MUST verify type_key=139 against C++ `cubic_interpolator_base.hpp` before implementing

---

## Work Objectives

### Core Objective

Deliver two focused PRs that (1) eliminate infrastructure debt and encoding edge cases, then (2) meaningfully expand what users can create — measured by automated gates at every step.

### Concrete Deliverables

- `.github/workflows/ci.yml` — automated test/lint/format/playwright pipeline
- Generalized ToC encoder collecting all property keys from all objects
- Boolean encoding fix (encoder + validator in lockstep)
- Typed error enum for `builder/scene.rs` via `thiserror`
- CubicInterpolator object type (easing curves for animations)
- TrimPath object type (path trimming effect)
- 5+ new animatable properties in `property_key_from_name()`
- Updated JSON schema v1 with new animatable property names
- 2+ new test fixtures with full e2e + playwright coverage

### Definition of Done

- [ ] `cargo test` passes with 0 failures (count ≥ 150 after both PRs)
- [ ] `cargo clippy -- -D warnings` clean
- [ ] `cargo fmt --check` clean
- [ ] Playwright regression passes for all fixtures (5 existing + new)
- [ ] GitHub Actions CI passes on push to any branch

### Must Have

- CI/CD pipeline (PR #27)
- Generalized ToC encoder (PR #27)
- At least 1 new interpolation type (CubicInterpolator, PR #28)
- At least 1 new visual object type (TrimPath, PR #28)
- All new types verified against C++ runtime headers

### Must NOT Have (Guardrails)

- NO Lottie exporter code or `Encoder` trait abstraction — premature, defer to future PR
- NO schema version bump to v2 — all changes are additive within v1
- NO bones/skeletal/constraints/text/assets — too complex for 2 PRs, defer to PR #29+
- NO StateMachineLayer "fix" — bug is already resolved
- NO full thiserror migration across all modules — scope to `builder/scene.rs` only
- NO byte-level snapshot tests for .riv files — use `inspect --json` structural comparison
- NO removal of `#![allow(dead_code, unused_imports)]` — still needed for incremental build
- NO vendoring of Rive JS runtime (harness pins `@rive-app/canvas@2.35.0` via CDN — acceptable for now)

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision

- **Infrastructure exists**: YES — cargo test (133 tests), playwright regression harness
- **Automated tests**: YES (tests-after) — each task adds unit + e2e tests
- **Framework**: `cargo test` (built-in) + `npx playwright` (regression)

### QA Policy

Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Encoder changes**: Bash — `cargo run -- inspect output.riv --json` before/after comparison
- **New object types**: Bash — `cargo test test_{type_name}` + e2e fixture generation
- **CI pipeline**: Bash — verify workflow YAML structure, then `gh workflow view`
- **Schema changes**: Bash — validate fixture JSON against schema

---

## Execution Strategy

### Parallel Execution Waves

```text
=== PR #27: Infrastructure & Correctness ===

Wave 1 (Start Immediately — independent foundations):
├── Task 1: GitHub Actions CI pipeline [quick]
├── Task 2: Generalize ToC encoder [unspecified-high]
├── Task 3: Boolean encoding fix (encoder + validator) [deep]
└── Task 4: Update AGENTS.md — remove stale SM bug note [quick]

Wave 2 (After Wave 1 — depends on encoder changes):
├── Task 5: Typed error model for builder/scene.rs [unspecified-high]
├── Task 6: Semantic validation hardening (validator) [deep]
└── Task 7: GitHub issue cleanup + exporter boundary ADR [writing]

Wave 3 (After Wave 2 — integration verification):
└── Task 8: PR #27 integration verification + playwright [unspecified-high]

=== PR #28: Object Coverage & Animation Enhancement ===

Wave 4 (Start Immediately — independent additions):
├── Task 9: CubicInterpolator object type [deep]
├── Task 10: TrimPath object type [unspecified-high]
└── Task 11: Expand animatable property set [unspecified-high]

Wave 5 (After Wave 4 — depends on new objects):
├── Task 12: New fixtures + e2e tests [unspecified-high]
├── Task 13: Update JSON schema v1 [quick]
└── Task 14: CubicInterpolator scene builder integration [deep]

Wave 6 (After Wave 5 — integration verification):
└── Task 15: PR #28 integration verification + playwright [unspecified-high]

Wave FINAL (After ALL tasks — independent review, 4 parallel):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
└── Task F4: Scope fidelity check (deep)

Critical Path: Task 1 → Task 8 → Task 9 → Task 14 → Task 15 → F1-F4
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 4 (Waves 1 and 4)
```

### Dependency Matrix

| Task | Depends On | Blocks | Wave |
|------|-----------|--------|------|
| 1 | None | 8 | 1 |
| 2 | None | 6, 8 | 1 |
| 3 | None | 6, 8 | 1 |
| 4 | None | 7 | 1 |
| 5 | None | 8 | 2 |
| 6 | 2, 3 | 8 | 2 |
| 7 | 4 | 8 | 2 |
| 8 | 1, 2, 3, 5, 6, 7 | 9 | 3 |
| 9 | 8 | 12, 14 | 4 |
| 10 | 8 | 12 | 4 |
| 11 | 8 | 12, 13 | 4 |
| 12 | 9, 10, 11 | 15 | 5 |
| 13 | 11 | 15 | 5 |
| 14 | 9 | 15 | 5 |
| 15 | 12, 13, 14 | F1-F4 | 6 |

### Agent Dispatch Summary

- **Wave 1**: 4 tasks — T1 → `quick`, T2 → `unspecified-high`, T3 → `deep`, T4 → `quick`
- **Wave 2**: 3 tasks — T5 → `unspecified-high`, T6 → `deep`, T7 → `writing`
- **Wave 3**: 1 task — T8 → `unspecified-high`
- **Wave 4**: 3 tasks — T9 → `deep`, T10 → `unspecified-high`, T11 → `unspecified-high`
- **Wave 5**: 3 tasks — T12 → `unspecified-high`, T13 → `quick`, T14 → `deep`
- **Wave 6**: 1 task — T15 → `unspecified-high`
- **FINAL**: 4 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

> Implementation + Test = ONE Task. Never separate.
> EVERY task MUST have: Recommended Agent Profile + Parallelization info + QA Scenarios.

- [ ] 1. GitHub Actions CI Pipeline

  **What to do**:
  - Create `.github/workflows/ci.yml` with jobs: `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
  - Add a playwright regression job that runs `npx -y -p playwright node tests/playwright/regression.js` (install chromium first)
  - Trigger on push to any branch and on pull_request to main
  - Use `actions/checkout@v4`, `dtolnay/rust-toolchain@stable`, caching via `Swatinem/rust-cache@v2`
  - Playwright job should depend on the build job (reuse compiled binary)

  **Must NOT do**:
  - Do not add deployment steps or release automation
  - Do not install unnecessary toolchains (nightly, etc.)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single file creation with well-known patterns; GitHub Actions YAML is templated work
  - **Skills**: []
    - No specialized skills needed — standard YAML authoring
  - **Skills Evaluated but Omitted**:
    - `playwright`: Not needed — we're configuring CI to run existing playwright script, not writing new browser automation
    - `git-master`: Not needed — no git history operations

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4)
  - **Blocks**: Task 8 (integration verification)
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `tests/playwright/regression.js:1-126` — The full playwright script that CI must execute; understand its deps (chromium, python3 http.server)
  - `Cargo.toml:1-19` — Package config, needed for CI cache keys

  **Test References**:
  - `tests/e2e.rs:1-245` — All 10 e2e tests that `cargo test` must pass in CI

  **External References**:
  - GitHub Actions docs: https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions
  - dtolnay/rust-toolchain: standard Rust CI action
  - Swatinem/rust-cache: cargo build cache action

  **WHY Each Reference Matters**:
  - `regression.js` needs chromium binary + python3 for http.server — CI job must install these
  - `Cargo.toml` edition=2024 requires Rust 1.84+ — CI must use stable (≥1.84)

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```text
  Scenario: CI workflow file is valid YAML and contains required jobs
    Tool: Bash
    Preconditions: .github/workflows/ci.yml exists
    Steps:
      1. cat .github/workflows/ci.yml — verify YAML parses
      2. grep -c "cargo test" .github/workflows/ci.yml — should be ≥ 1
      3. grep -c "cargo clippy" .github/workflows/ci.yml — should be ≥ 1
      4. grep -c "cargo fmt" .github/workflows/ci.yml — should be ≥ 1
      5. grep -c "playwright" .github/workflows/ci.yml — should be ≥ 1
    Expected Result: All 4 commands present in workflow
    Failure Indicators: Missing any of the 4 commands
    Evidence: .sisyphus/evidence/task-1-ci-yaml-validation.txt

  Scenario: All existing tests pass locally (baseline before CI)
    Tool: Bash
    Preconditions: Clean working tree on main
    Steps:
      1. cargo test 2>&1 | tail -5
      2. cargo clippy -- -D warnings 2>&1 | tail -3
      3. cargo fmt --check 2>&1
    Expected Result: All 3 commands exit 0
    Failure Indicators: Any non-zero exit code
    Evidence: .sisyphus/evidence/task-1-baseline-tests.txt
  ```

  **Commit**: YES
  - Message: `ci: add GitHub Actions workflow for test/lint/format/playwright`
  - Files: `.github/workflows/ci.yml`
  - Pre-commit: `cargo test && cargo clippy -- -D warnings`

- [ ] 2. Generalize ToC Encoder

  **What to do**:
  - In `src/encoder/mod.rs:27-36`, replace the hardcoded `if prop.key == 236` check with a general collection: iterate all objects, collect ALL unique property keys, EXCLUDE baseline properties (4=name, 5=parentId, 7=width, 8=height) that runtimes resolve via built-in registry
  - Ensure `toc::encode_toc()` handles >16 property keys correctly (multiple uint32 chunks)
  - Add unit test for ToC with 0, 1, 16, and 17+ property keys
  - Verify existing fixtures produce valid output by comparing `inspect --json` before/after

  **Must NOT do**:
  - Do not include baseline properties (4, 5, 7, 8) in ToC — these cause WASM runtime import failures
  - Do not change the ToC encoding format itself (2-bit backing types, 16 per uint32 LE)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Requires understanding of binary format semantics, ToC backing type resolution, and runtime compatibility constraints
  - **Skills**: []
    - No specialized skills needed — pure Rust logic change
  - **Skills Evaluated but Omitted**:
    - `playwright`: Not needed — playwright is QA gate, not implementation tool
    - `git-master`: Not needed — no git operations

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 4)
  - **Blocks**: Tasks 6, 8
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `src/encoder/mod.rs:24-47` — Current `encode_riv()` with hardcoded ToC key 236; this is the function to refactor
  - `src/encoder/toc.rs:1-34` — ToC encoding: 2-bit backing types packed into LE uint32; must handle multi-chunk correctly
  - `src/objects/core.rs:171-256` — `property_backing_type()` maps every known key to its backing type; ToC entries must match
  - `docs/format-spec.md:14-22` — ToC format documentation: property keys terminated by 0, backing bits packed 16 per uint32

  **API/Type References**:
  - `src/objects/core.rs:1-7` — `BackingType` enum: UInt=0, String=1, Float=2, Color=3
  - `src/objects/core.rs:34-37` — `RiveObject` trait: `properties()` returns Vec<Property> with key + value

  **Anti-Pattern References**:
  - `docs/format-spec.md:37` — "Do not include baseline properties in ToC: name (4), parentId (5), width (7), height (8)"
  - `AGENTS.md` anti-patterns — "NEVER include baseline properties (name=4, parentId=5, width=7, height=8) in ToC"

  **WHY Each Reference Matters**:
  - `encode_riv()` is the only place ToC keys are collected — the entire change is here
  - `toc.rs` handles the bit-packing; with >16 keys it must emit multiple uint32 words
  - `property_backing_type()` is the source of truth for what backing type each key uses
  - Baseline property exclusion is a hard runtime constraint — including them causes "may be corrupt" errors

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```text
  Scenario: ToC now includes non-baseline property keys from shapes fixture
    Tool: Bash
    Preconditions: shapes.json fixture exists, ToC encoder refactored
    Steps:
      1. cargo run --quiet -- generate tests/fixtures/shapes.json -o /tmp/toc_shapes.riv
      2. cargo run --quiet -- inspect /tmp/toc_shapes.riv --json | python3 -c "import json,sys; d=json.load(sys.stdin); keys=d.get('toc_property_keys',[]); print(f'ToC keys: {len(keys)} — {keys}')"
      3. Verify ToC contains property keys like 37 (solid_color_value), 38 (gradient_stop_color), etc.
      4. Verify ToC does NOT contain keys 4, 5, 7, 8 (baseline)
    Expected Result: ToC has >0 keys, none are baseline
    Failure Indicators: ToC empty OR contains 4/5/7/8
    Evidence: .sisyphus/evidence/task-2-toc-shapes.txt

  Scenario: ToC multi-chunk encoding (17+ properties)
    Tool: Bash (cargo test)
    Preconditions: Unit test added for 17+ key ToC
    Steps:
      1. cargo test test_toc_encode_17_keys 2>&1
    Expected Result: Test passes — 17 keys require 2 uint32 words, decoded correctly
    Failure Indicators: Panic or assertion failure on second uint32 chunk
    Evidence: .sisyphus/evidence/task-2-toc-multichunk.txt

  Scenario: All 5 existing fixtures still validate after ToC change
    Tool: Bash
    Preconditions: ToC refactored
    Steps:
      1. for f in minimal shapes animation state_machine path; do cargo run --quiet -- generate tests/fixtures/$f.json -o /tmp/toc_$f.riv && cargo run --quiet -- validate /tmp/toc_$f.riv; done
    Expected Result: All 5 report "valid"
    Failure Indicators: Any "invalid" or parse error
    Evidence: .sisyphus/evidence/task-2-toc-validation.txt
  ```

  **Commit**: YES
  - Message: `fix: generalize ToC encoder to collect all property keys`
  - Files: `src/encoder/mod.rs`, `src/encoder/toc.rs` (if test added there)
  - Pre-commit: `cargo test`

- [ ] 3. Boolean Encoding Fix (Encoder + Validator)

  **What to do**:
  - In `src/encoder/binary_writer.rs`, change `write_bool()` (currently calls `write_varuint`) to directly push a single raw byte: `self.buffer.push(if value { 1 } else { 0 })`
  - In `src/encoder/mod.rs:encode_object()`, identify CoreBoolType properties (isVisible=41, enableWorkArea=62, quantize=376, smBoolValue=141, linkCornerRadius=164) and call `writer.write_bool()` instead of `writer.write_varuint()`
  - In `src/validator/mod.rs:212-240`, for CoreBoolType properties, use `reader.read_byte()` instead of `reader.read_varuint()`
  - Add a helper function `is_bool_property(key: u16) -> bool` in `objects/core.rs` that returns true for the 5 CoreBoolType property keys
  - Add unit tests verifying single-byte encoding/decoding roundtrip for bool properties
  - **CRITICAL**: Encoder and validator changes MUST be in the SAME commit to prevent desync

  **Must NOT do**:
  - Do not change `BackingType` enum (bools still report as `UInt` in the ToC — this matches C++ runtime behavior)
  - Do not modify properties() implementations — they already return `PropertyValue::UInt(0/1)` for bools, which is correct

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Requires understanding of LEB128 encoding edge cases, C++ runtime deserialization behavior, and encoder/validator symmetry
  - **Skills**: []
    - No specialized skills needed — low-level binary format work
  - **Skills Evaluated but Omitted**:
    - `playwright`: Verification gate only, not implementation
    - `git-master`: No git operations

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 4)
  - **Blocks**: Tasks 6, 8
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `src/encoder/binary_writer.rs:52-54` — Current `write_bool()` implementation that calls `write_varuint` (the line to change)
  - `src/encoder/mod.rs:8-22` — `encode_object()` match on PropertyValue variants; must dispatch bools differently
  - `src/validator/mod.rs:212-240` — Property value reading; must read bools with `read_byte()` not `read_varuint()`

  **API/Type References**:
  - `src/objects/core.rs:90-169` — Property key constants; identify which are CoreBoolType
  - C++ reference: `core_registry.hpp` → `CoreBoolType::deserialize` calls `reader.readByte()`

  **WHY Each Reference Matters**:
  - `write_bool` is the exact function to fix — currently delegates to varuint
  - `encode_object` dispatches based on PropertyValue variant, but bools are `UInt(0/1)` — need to check property key to determine bool encoding
  - Validator must mirror encoder exactly — asymmetry = corruption

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```text
  Scenario: Bool properties encode as single raw byte
    Tool: Bash (cargo test)
    Preconditions: write_bool fixed to use buffer.push
    Steps:
      1. cargo test test_write_bool_raw_byte 2>&1
    Expected Result: Test passes — write_bool(true) produces [0x01], write_bool(false) produces [0x00], NOT LEB128
    Failure Indicators: Extra bytes in output (LEB128 continuation bits)
    Evidence: .sisyphus/evidence/task-3-bool-encoding.txt

  Scenario: Roundtrip encode/decode preserves bool values
    Tool: Bash (cargo test)
    Preconditions: Both encoder and validator updated
    Steps:
      1. cargo test test_roundtrip_bool_property 2>&1
    Expected Result: Encode a Fill with isVisible=1, parse it back, verify isVisible=UInt(1)
    Failure Indicators: Parse failure or wrong value
    Evidence: .sisyphus/evidence/task-3-bool-roundtrip.txt

  Scenario: All fixtures still validate after bool encoding change
    Tool: Bash
    Steps:
      1. for f in minimal shapes animation state_machine path; do cargo run --quiet -- generate tests/fixtures/$f.json -o /tmp/bool_$f.riv && cargo run --quiet -- validate /tmp/bool_$f.riv; done
    Expected Result: All 5 "valid"
    Failure Indicators: Parse error on bool property deserialization
    Evidence: .sisyphus/evidence/task-3-bool-fixtures.txt
  ```

  **Commit**: YES
  - Message: `fix: use raw byte encoding for CoreBoolType properties`
  - Files: `src/encoder/binary_writer.rs`, `src/encoder/mod.rs`, `src/validator/mod.rs`, `src/objects/core.rs`
  - Pre-commit: `cargo test`

- [ ] 4. Update AGENTS.md — Remove Stale SM Bug Note

  **What to do**:
  - In `AGENTS.md` NOTES section, remove or update the line: "**Active bug**: StateMachineLayer (type 57) causes WASM runtime 'may be corrupt' error — under investigation"
  - Replace with: "StateMachineLayer bug resolved in PRs #21-25 — builder auto-injects sentinel states (AnyState, EntryState, ExitState) and interleaves transitions after their source state"
  - In `src/objects/AGENTS.md` KNOWN ISSUE section, update similarly
  - Update test count from 131 to current count (133)

  **Must NOT do**:
  - Do not rewrite entire AGENTS.md — only update the specific stale sections

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Two small documentation edits with known content
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `writing`: Overkill for 2-line doc updates

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: Task 7
  - **Blocked By**: None

  **References**:
  - `AGENTS.md:91` — "Active bug" line to update
  - `src/objects/AGENTS.md:39-40` — KNOWN ISSUE section to update

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```text
  Scenario: Stale SM bug note removed
    Tool: Bash (grep)
    Steps:
      1. grep -c "Active bug" AGENTS.md — should be 0
      2. grep -c "under investigation" AGENTS.md — should be 0
      3. grep -c "resolved" AGENTS.md — should be ≥ 1
    Expected Result: No stale bug references, resolution noted
    Evidence: .sisyphus/evidence/task-4-agents-md.txt
  ```

  **Commit**: YES (groups with Task 7)
  - Message: `docs: update AGENTS.md, close stale issues, add exporter ADR`
  - Files: `AGENTS.md`, `src/objects/AGENTS.md`

---

## Final Verification Wave (MANDATORY — after ALL implementation tasks)

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, curl endpoint, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo clippy -- -D warnings` + `cargo fmt --check` + `cargo test`. Review all changed files for: `as any` equivalent (`as u16` without bounds check), empty catches, `unwrap()` in library code, commented-out code, unused imports beyond main.rs allow. Check AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Lint [PASS/FAIL] | Tests [N pass/N fail] | Files [N clean/N issues] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high` (+ `playwright` skill)
  Start from clean state. Execute EVERY QA scenario from EVERY task — follow exact steps, capture evidence. Test cross-task integration (e.g., CubicInterpolator keyframe in animation fixture, TrimPath in shape fixture). Run full playwright regression. Save to `.sisyphus/evidence/final-qa/`.
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

Each PR should contain atomic commits following the repo's existing `feat:` / `fix:` convention:

**PR #27 commits:**
- `ci: add GitHub Actions workflow for test/lint/format/playwright`
- `fix: generalize ToC encoder to collect all property keys`
- `fix: use raw byte encoding for CoreBoolType properties`
- `refactor: introduce typed error enum for builder/scene.rs`
- `fix: add semantic validation rules to validator`
- `docs: update AGENTS.md, close stale issues, add exporter ADR`

**PR #28 commits:**
- `feat: add CubicInterpolator object type`
- `feat: add TrimPath object type`
- `feat: expand animatable property set in builder`
- `feat: add cubic interpolator integration in scene builder`
- `test: add fixtures and e2e tests for new object types`
- `docs: update scene schema v1 with new animatable properties`

---

## Success Criteria

### Verification Commands

```bash
# PR #27
cargo test                                    # ≥ 140 tests, 0 failures
cargo clippy -- -D warnings                   # clean
cargo fmt --check                             # clean
npx -y -p playwright node tests/playwright/regression.js  # exits 0
cat .github/workflows/ci.yml | head -1        # file exists

# PR #28
cargo test                                    # ≥ 155 tests, 0 failures
cargo test test_cubic_interpolator            # passes
cargo test test_trim_path                     # passes
ls tests/fixtures/ | wc -l                    # > 5
npx -y -p playwright node tests/playwright/regression.js  # exits 0, all fixtures pass
```

### Final Checklist

- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] All tests pass
- [ ] CI pipeline operational
- [ ] Playwright regression green for ALL fixtures
- [ ] No stale documentation (AGENTS.md SM bug note removed)
