---
node: rive-cli.verification.parity
status: open
created: 2026-09-12
---

# B1 — Pinned official CLI reference seed

Execution issue: [#258](https://github.com/George-RD/rive-rs-cli/issues/258).
Parent: #256. Independent of #257 and compiler extraction. Planning base:
`dfdf746e923a4935ab1d78c30425588d95bebcd5`.

The adapter under `parity/cli-reference/` wraps the existing public render/compare
commands. It introduces no parser, renderer, compiler dependency or asset API.
Default tests are offline contracts; real capture is explicitly opt-in.

## Acceptance and evidence

- Pinned source/archive, executable identity and exact reported version: implemented.
- Original static and transform-animation projects, provenance and command audit: implemented.
- Isolated no-login/network official commands, immutable capture and bounded claims: implemented.
- Existing-runtime render/self-comparison/paint-negative adapter: implemented.
- Successful official builds, inspected outputs and actual runtime evidence: pending execution.
- Exact-head Rust/MSRV, runtime and Cairn gates: pending execution.

Local Python contract tests passed (36 tests). Behavioral red evidence was observed
for missing-binary rejection and retaining process launch failures before their
implementations. Synthetic contract fixtures are not official execution evidence.
Local Rust and Cairn are unavailable; outbound DNS prevented binary acquisition.
The separate opt-in workflow provides a reproducible remote execution route.
The first remote capture at `086a648` (run 34693047769, artifact 10297871878)
verified the archive/member and built the local CLI, then failed loading the
official executable because Ubuntu lacked `libEGL.so.1`. No compilation/runtime
pass was inferred. Provisioning now installs `libegl1 libgles2` explicitly.
Review red/green also covers missing command/output binding, missing network
namespace evidence and disagreement between the recorded and observed source head.
A PR-description bot exposed duplicate capture triggers; opt-in now requires the
marker to be newly added, not merely present.

## Review

Standards self-review: bounded parity adapter, standard library only, explicit
process boundaries, no comments/docstrings, no production compiler edits.
Spec self-review: the runtime/retained-artifact acceptance gate is not yet met;
#258 must remain open and the PR must remain unmerged until actual capture and
required verification pass. No independent sub-agent review was available.

Record actual execution and exact-head results here before closing this todo.
Do not change #256 or the parallel #257 scope to make the checks pass.
