---
node: rive-cli.intelligence.ai
status: open
created: 2026-07-29
---

# P4 — Split the repair engine behind characterization tests

Decompose the existing repair engine only after the authoring and evaluation
boundaries are stable. Preserve behavior with characterization tests and avoid
optimizing a direct-SceneSpec generation path that is no longer the product
priority.

## Immediate correctness gate

A bounded P0 defect interrupted the deferred P4 refactor. `deduplicate_names` kept
the first object named `foo`, renamed later duplicates, then recorded multiple
`foo -> foo_N` rewrites in a map keyed by the original name. The final duplicate
won, so a recognized reference to `foo` encountered by the legacy walker could be
redirected to the last renamed object even though the first object still owned
`foo`.

The intended duplicate cannot be inferred from an ambiguous pre-repair reference.
The accepted safe policy is therefore:

- the first object retains the original name;
- later duplicates receive deterministic suffixes;
- an existing reference to the duplicated original name remains on the first object;
- repair must not silently infer that the reference meant a later duplicate.

PR #166 closes this gate as a focused defect fix rather than beginning the deferred
module split:

- the first end-to-end keyframe contract established that duplicate-name normalization ran and generated `foo`, `foo_2`, and `foo_3`; it also showed that the legacy reference walker did not descend into animation `keyframes`, narrowing the architecture report's original example;
- the direct characterization was therefore moved to an `object` reference inside the traversal owned by `deduplicate_names`;
- exact test-only head `52ee952` passed Rust 1.88, rustfmt, Clippy, browser contracts, and 619 existing unit tests in runs `31182212085` and `31182211147`; only `test_deduplicate_names_preserves_ambiguous_reference` failed;
- the RED failure observed three fixes instead of the two deterministic renames, proving that the global rewrite added a third reference mutation before the exact-value assertion;
- implementation `c53717b` removes the ambiguous rename map and the now-dead recursive reference-rewrite helper, while retaining deterministic duplicate suffixes and existing repair messages;
- the focused unit regression and end-to-end RepairEngine contract passed before the implementation commit was persisted;
- exact implementation head `eefe2cd` passed the Rust 1.88 minimum in run `31182568911` and the complete repository suite in run `31182568545`: rustfmt, Clippy, all Rust tests, browser contracts, Cairn architecture validation, official-runtime evidence, demo, site, Playwright, and visual regression all passed before this durable evidence was committed.

A late review on merged PR #166 found that the end-to-end contract also pinned the
current retry count and indexed the second attempt directly. PR #167 hardens the
same completed correctness gate without changing production behavior:

- the contract now searches all recorded attempts for the deterministic `foo_3` rename rather than requiring exactly one retry and indexing `attempts[1]`;
- the repaired names and the final ambiguous-reference policy remain the primary assertions;
- exact test-hardening head `85dabaa` passed the Rust 1.88 minimum in run `31184278616` and the complete repository suite in run `31184279807`: rustfmt, Clippy, all Rust tests, browser contracts, Cairn architecture validation, official-runtime evidence, demo, site, Playwright, and visual regression all passed before this evidence was committed;
- this is review remediation inside the existing P0 gate, not a new roadmap item or the beginning of the deferred P4 module split.

## Acceptance criteria

- The immediate duplicate-reference regression is fixed without changing unrelated repair behavior.
- Existing repair outputs and diagnostics remain compatible.
- Passes have explicit ownership and bounded inputs/outputs.
- Repair results map back to AuthoringSpec source paths when the frontend exists.

The broader P4 modularization remains open and deferred until the Authoring compiler
architecture is stable. Line count alone is not sufficient reason to split a
cohesive module.
