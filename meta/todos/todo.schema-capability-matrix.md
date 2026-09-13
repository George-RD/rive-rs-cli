---
node: rive-cli.verification.parity
status: done
created: 2026-09-12
---

# B2 — Capability matrix and official-schema drift

Implemented for #266 in PR #277, parent #256. Review base:
`92968573a3e171ea1ed83026e24f9bc02909271f`. This is independent of #259 and the
Pages work in #276. No production compiler definitions, crates or binary semantics
change. Parents #256 and #175 and the linked correctness issues remain open.

The public developer commands and support boundaries are in
`parity/cli-reference/CAPABILITIES.md`. The schema capture at
`41d8a7969c6db3036f5642496596e86daed5e101`, run 34715128718, supplies 351 type
records and 3,729 property occurrences. Version/archive/executable/source hashes,
owner-qualified properties, inheritance, literal defaults/enums and flags are
retained; unavailable facts stay unknown. Current canonical metadata comes from
existing generated sources, not a new production registry.

The report distinguishes declaration, parse, lowering, encoding, runtime and
semantic stages, with the B1 original static/animated proof in its own official
pipeline. Every requested family and the known #123-#128/#252/#254/#255 obligations
remain visible. #175's remaining behavior scope is not promoted by this tooling.
Discovery/compile/inspect/render failure lessons are recorded in the developer guide.

TDD observed failures before fixes for property-key/default/enum drift, shadowed
inherited names, singular/empty property inventories, source-command identity,
malformed candidate fields, stale fixture evidence and directly inspectable report
facts. The suite also checks snapshot immutability, current registry drift and
path-only crate moves. The retained snapshot matched independent normalization
byte for byte. No local Rust, browser or Cairn execution is claimed.

Standards and Spec author review use the base above and remain separate from
external review. Findings fixed during review include lost inherited property
identity and malformed snapshots accepted as ordinary drift. Exact final-head
CI, external review and merge evidence belong on PR #277; this implementation
status is not permission to skip those gates. The first retention writer and
source-archive tooling are removed from the final diff. Normal tests neither
execute nor download the official CLI. Refresh writes a new candidate only.


External review follow-up: Codex reproduced missing candidate compiler metadata
passing as clean. CodeRabbit also identified incomplete provenance comparison,
indented continuation ambiguity and a missing positive opt-in test assertion.
The fixes require complete snapshot inventories/provenance, bind the retained
manifest source head, compare candidate facts and source identities separately,
reject malformed enum/bit continuations and test both marker predicates. The CLI's
actual property descriptions remain excluded from retained facts but included in
raw-output digest comparison. Rejecting all such prose was not adopted: the pinned
capture contains 659 distinct description lines. All 351 types were re-normalized
with identical retained bytes after the stricter parser; no baseline refresh was
needed. The expanded 70-contract suite and final-head gates are recorded on #277.

Resume review: `0ca6046` rejects unknown top-level snapshot members, including
misleading runtime-test claims, with JSON/xz regressions. Its 71-test Python suite
and full repository CI passed, but fresh review found root SceneSpec changes could
still be omitted. Four public-check mutations reproduced that blind spot: root
requirements, field constraints, references and an ObjectSpec envelope constraint.

The metadata reader now retains both schema envelopes and requires them in
candidates. Those four mutations, candidate-only root drift and formatting-only
controls pass after correction; the 18 capability/snapshot tests ran locally.
The initial snapshot was deliberately extended using the original hash-verified
canonical source, adding only the missing root and ObjectSpec envelope. This is
not an upstream refresh or acceptance of changed production behavior. All official
facts and prior metadata/provenance are unchanged. The original and completed
archive hashes and source hash are recorded in `schema-baseline/README.md`.
The B1 runtime archive remains untouched. Fresh checks and review of this correction
must pass before merge; the earlier `0ca6046` checks are not substitutes.

September 13 resume: `a8bab6a` handles malformed/truncated XZ, validates type-key
ranges and inheritance, and checks the complete opt-in condition. `1ece11a` adds
pre-change retained-baseline comparison and rejects duplicate JSON members and
repeated enum/bit facts. Its full CI passed, but manifest validation still accepted
missing capture fields and arbitrary claims during initial retention.

The manifest reader now requires the exact v1 field set, a valid source head and
digests, and positive integer capture run/artifact identifiers before evaluating
any reviewed archive exception. Both base and candidate manifests use this rule.
Two new public-command regressions exposed 22 false passes before correction;
all nine retention tests pass afterward. The source modules used locally matched
their Git blobs, and normalization of the actual capture reproduced the completed
34,100-byte snapshot and its existing SHA-256. No baseline or production source
was changed. Final-head verification and external review remain recorded on #277.
