# Capability report and schema drift

This is developer tooling for #266, not an RML parser or a new production registry.
It reads the committed CLI 1.0.2 schema facts, the existing generated registry and
`docs/scene.schema.v1.json`, and the unchanged B1 runtime capture. Python 3.11+
is sufficient for ordinary reports and tests. No Rust, browser, official CLI,
account or network is needed.

## Commands

```sh
python3 parity/cli-reference/capabilities.py report
python3 parity/cli-reference/capabilities.py report --json > /tmp/capabilities.json
python3 parity/cli-reference/capabilities.py check --json
python3 -m unittest discover -s parity/cli-reference -p 'test_*.py' -v
```

JSON includes all normalized official property facts, owner-qualified identities,
current canonical fields, source digests, declaration differences, family inventory,
known work and scoped evidence. Output is deterministic for identical inputs.
The registry reader allows its single authoritative file to move from `src/` into
`crates/`; a path-only move is not semantic drift. A new unrecognized registry
format fails rather than silently returning an empty inventory. The existing
Rust schema-generation guard remains responsible for keeping the published schema
aligned with production types.

`check` returns 0 when the reviewed schema/compiler metadata is unchanged and the
retained fixture evidence verifies, 1 for semantic drift, and 2 for invalid or
stale evidence. The normal workflow runs that check. A failing check is not fixed
by regenerating a baseline without review. Its JSON names added, removed and
changed fields; keys, defaults, enum labels and metadata changes remain distinct.

## What the stages mean

Per-type stages concern the independent RML path: declared, parsed, lowered,
encoded, runtime-tested and semantic-tested. `declared: true` means the pinned
official schema describes the type. The other fields are currently `null`:
unassessed, not proven unsupported. Current registry keys and canonical-name
candidates are reported separately. Similar names do not prove a field mapping,
parser, lowering rule or working binary.

The B1 static and animated cases have their own `official-cli-reference` pipeline.
Their retained parse/build/runtime observations do not promote our RML support.
Lowering and semantic evaluation remain unknown for those cases. Their files,
commands, source fixtures and comparison measurements are revalidated offline;
this is historical evidence consistency, not a fresh runtime execution or
execution attestation. Existing SceneSpec/Authoring runtime and semantic suites
continue independently; this report does not automatically turn their suite-level
results into per-type support.

All requested families remain visible, including scripts, shaders and a 3D row.
The captured type names do not identify a separate 3D family; that row is unassessed,
not a claim that Rive cannot do 3D. Family assignment is a reporting heuristic,
not an authoritative type registry. No complete family has runtime parity proof.

Gap classes distinguish RML parser, canonical representation, builder/encoder,
runtime/version, editor-only and external compiler/signing/tool requirements.
They identify investigation boundaries as well as known defects. A field hidden
without `--all` is recorded as filtered, not independently proved runtime-irrelevant.
No missing default or enum value is guessed. Numeric enum discriminants, containment
rules and runtime behavior are not inferred from literal labels or inheritance.

## Refresh without changing the baseline

Provision the archive pinned by `lock.json` outside the repository as described in
the reference README. Use a clean committed checkout and an empty output location.
The capture runner validates the archive and executable and runs every official
command without credentials or network. Acquisition is not part of normal tests.

```sh
python3 parity/cli-reference/schema_capture.py \
  --official-cli /absolute/path/to/rive \
  --archive /absolute/path/to/rive-linux-x64.tar.gz \
  --output /tmp/schema-capture

python3 parity/cli-reference/capabilities.py refresh /tmp/schema-capture \
  --expected-head COMMIT_RECORDED_BEFORE_CAPTURE \
  --output /tmp/schema-candidate.json.xz

python3 parity/cli-reference/capabilities.py check \
  --candidate /tmp/schema-candidate.json.xz --json
```

A future version can use an explicitly reviewed separate `--lock` file for both
capture and refresh. That does not rewrite the current lock or B1 runtime baseline.
`refresh` validates complete command/output bindings and writes only a new candidate;
it refuses an existing destination. It includes compiler metadata from the checkout
performing normalization, identified by source digests, separately from the official
capture's source head. Review the reported schema, reference identity and current
compiler changes before intentionally retaining a new snapshot/pin in a PR.
If schema and compiler snapshots are refreshed together, compare against the old
pin first; replacing the expected input is not itself proof of correctness.

## Findings that change the authoring workflow

**Discover:** the captured `LibraryArtboard` has both its own `name` (key 794) and
inherited `Asset.name` (key 203). Bare property-name maps lose a real declaration.
The normalizer keys by owner and name. Single-property and zero-property schemas
also use different summary shapes. Tests reproduce those observations. Defaults
and accepted enum labels are literal facts, not inferred numeric encodings.

**Compile:** B1's first actual execution failed for a missing Linux `libEGL` library.
That was a tool prerequisite, not an RML or encoder defect. The corrected reference
verified and built two original projects offline; successful exit status alone was
insufficient, so B1 checks the structured command/error result. Its `--publish` and
`--rev` probes required the specific missing-login diagnostic. That does not prove
unsigned production distribution rights or script/shader compilation.

**Inspect:** a valid object inventory is not working behavior. #252 records how a
wrong view-model name key survived structural validation but could not be resolved
by name in the runtime. The matrix therefore links #123-#128 and #252/#254/#255 to
specific remaining obligations rather than declaring them fixed through schema
agreement. Parent #175's unfinished behavior scope stays visible.

**Render:** B1's identical-file controls and paint perturbations prove that its
comparison pipeline detects a visual difference on two original scenes. They do
not exercise every declared type or evaluate broad authored intent. The new stale
fixture mutation is rejected instead of reusing that old runtime pass. Continue
through discover, compile, inspect and runtime/semantic checks; no evidence here
establishes that our agent guidance is superior to the official workflow.

#259 remains independently unblocked. #267 supplies the first independent static
RML compile and scoped differential proof after its compiler prerequisite lands.
This report neither closes known correctness issues nor blocks all progress on the
historical 104-type backlog.
