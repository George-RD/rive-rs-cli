---
node: rive-cli.verification.parity
status: done
created: 2026-09-12
---

# B1 — Pinned official CLI reference seed

Implemented in [PR #274](https://github.com/George-RD/rive-rs-cli/pull/274) for
[#258](https://github.com/George-RD/rive-rs-cli/issues/258), parent #256.
Independent of #257. Planning base: `dfdf746e923a4935ab1d78c30425588d95bebcd5`.
No compiler, runtime, asset API, shared workflow or #256 changes.

## Acceptance and evidence

The actual reference ran at `f878465e8d3b493e567d1556130880b84a7ec8df` in successful
run 34693899414. CLI 1.0.2 verified/built/inspected both original projects without
credentials/network; repeated outputs matched. The existing canvas 2.39.1 renderer
loaded static and animated bytes. Self-comparisons were exact; paint perturbations
produced 8.59% and 9.28% maximum pixel differences. Both distribution probes reported
missing login; scripts, shaders, signing and production rights remain untested.

The complete original evidence is committed in `parity/cli-reference/baseline/`
with independent source/artifact/archive pins and human-readable results.
40 local Python tests pass, including a no-subprocess check of that actual capture.
They do not claim local official CLI, browser, Rust or Cairn execution.
The source-head CI run 34693872297 and MSRV run 34693872295 passed. Final retention
commit gates and any fresh exact-head capture are recorded on PR #274 before merge.

## Decisions and review

The adapter reuses public render/compare commands; no second renderer, parser or
production dependency. Acquisition/capture is deliberately opt-in. Default tests
check retained evidence offline; missing/wrong binaries and malformed evidence fail.
Baselines are immutable historical observations, not silently refreshed output.

TDD covered missing binaries, retained process failures, evidence/output bindings,
network-isolation claims, independent expected source heads and the actual raw
render JSON contract. The baseline contract failed before the capture was retained.
Earlier runs failed on missing libEGL and an incorrect render JSON assumption;
actual failures informed fixes and were not counted as successful runtime evidence.

Standards and spec self-review completed. CodeRabbit verified and resolved the
expected-head finding; the Linux library prerequisite was documented and retained
based on the actual executable failure. Merge remains subject to fresh exact-head
CI/review, not this task status alone. See PR #274 for the final verification ledger.
