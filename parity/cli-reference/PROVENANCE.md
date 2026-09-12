# Sources and audit scope

Reviewed on 2026-09-12. This is a public-interface audit, not a description of
undocumented implementation. CLI command observations belong in the actual run;
public documentation alone is not experimental evidence.

| Material | Source / ownership | Retention |
| --- | --- | --- |
| `fixtures/*/scene.rml`, `rive.yaml` | Original minimal inputs written for this repository; MIT under root `LICENSE` | Retained, not copied from Rive samples |
| Runner and contract tests | Original code for this repository; root MIT license | Retained |
| Official CLI 1.0.2 | [Rive's cask](https://github.com/rive-app/homebrew-tap/blob/main/Casks/rive-cli.rb), source blob `d9770d76f2439190bec057ec89bdfcc44b0f4024`; exact versioned archive and digest in `lock.json` | Proprietary executable/installation never vendored or uploaded |
| Existing canvas runtime | `@rive-app/canvas` 2.39.1; MIT as recorded in `assets/README.md`; git blob pins in `lock.json` | Existing assets reused unchanged |
| Official docs, samples, scaffolded AGENTS.md | [Public docs](https://rive.app/docs/cli/overview); no separate sample/scaffold redistribution grant established in this audit | No samples, full docs or generated guidance copied; command output digests only for audit probes |
| Retained compiled outputs, inspection and PNGs | Derived solely from the original fixtures, without Rive sample artwork, scripts, fonts or shaders | Actual successful capture retained for this development test; no claim about signed or commercial distribution rights |

The cask supplies an archive checksum, not a separate executable checksum. The
runner validates that archive first, hashes its regular `rive` member, matches the
supplied executable, and records both digests. A same-version replacement binary
cannot bypass this check. Changing the archive pin is an explicit source change.

## Commands relevant to these fixtures

The public [command reference](https://rive.app/docs/cli/reference/commands)
describes `--verify`, unsigned `--once`, and resolved-scene `inspect --json`.
Its structured JSON includes command identity and error information; exit zero
alone is insufficient because unknown project flags can be ignored.

`schema Rectangle`, `schema Shape --animatable`, `schema Fill`, `schema SolidColor`,
`schema LinearAnimation`, and `schema KeyFrameDouble` cover the two fixtures.
`docs format`, `docs --list`, `samples --path` and `--help` are non-copying audit
probes. The runner records whether each actually works offline on the pinned
binary and records output digests, without vendoring those materials. Any failed
audit probe remains an observed unsupported/unavailable command, not a pass.

The [RML format](https://rive.app/docs/runtimes/advanced-topic/rml) describes
fragment documents, the version attribute, typed nodes, IDs, keyed properties and
rotation property key 15. The [project configuration](https://rive.app/docs/cli/reference/project-config)
requires a name in `rive.yaml` and discovers source files by extension. Therefore
original and perturbed projects are isolated from each other and from evidence
logs; arbitrary files are not placed in either project's source directory.

The [agent guidance](https://rive.app/docs/cli/agents) describes consulting the
CLI's schema/docs/samples and the guidance created by `rive create`. No scaffold
or sample is needed for these originals, so none is copied. LSP, Luau protocols,
layout, state machines, images/fonts, shaders, screenshot/benchmark commands,
editor conversion and authenticated distribution are outside this seed.

Public documentation describes account/network requirements for signed outputs.
Only the recorded no-credential/offline probe outcomes count as observations in
this experiment; they do not establish licensing or commercial deployment rights.

## Retained capture review

`baseline/capture.tar.xz` contains the original 87 files from successful run
34693899414, repacked without changing their contents. The retention scope is
original RML/config inputs, their unsigned RIV outputs, inspection/build reports,
PNG captures and command evidence. It includes no executable, installation tree,
account files, credentials, fonts, Rive sample artwork or copied schema/docs
output. Audit probes retain only output digests and exit status. The original
source fixtures and project code use the root MIT license; that does not relicense
Rive's tools or establish production distribution rights.

[baseline/RESULTS.md](baseline/RESULTS.md) records actual outcomes separately from
public documentation. Every retained file is inventoried and SHA-256 checked by
the offline baseline contract, with subprocess execution forbidden.

## B2 normalized schema facts

The separate schema capture for #266 ran at source
`41d8a7969c6db3036f5642496596e86daed5e101`, workflow run 34715128718. It reused
`lock.json`'s CLI 1.0.2 archive and the existing isolated HOME/network runner.
All 351 listed types were queried with the normal and `--all` views. The
[normalized snapshot](schema-baseline/README.md) retains factual type/property
identities, inheritance, literal defaults, accepted enum labels and flags, with
source/executable/recording/output-inventory digests. It does not retain property
explanations, documentation, samples, scaffolds or the proprietary executable.

Attribution remains Rive, Inc. The root MIT license covers our original tooling;
it does not relicense Rive's tools or establish signing, commercial distribution
or artwork rights. The original B1 archive still contains audit hashes only.
The new snapshot is a development compatibility observation, not a replacement
for its runtime evidence. Synthetic test captures are explicitly named and are
never retained as actual reference runs.
