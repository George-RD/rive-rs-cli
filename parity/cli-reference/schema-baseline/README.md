# Retained public-schema facts

`facts.json.xz` is deterministic UTF-8 JSON compressed with xz. `snapshot.json` pins
its digest and the actual source capture. The `report --json` command exposes the
facts directly; `xz -dc facts.json.xz` is also sufficient to inspect the file.

The source is CLI 1.0.2 at archive/executable identities retained inside the
snapshot, captured from `41d8a7969c6db3036f5642496596e86daed5e101` in run
34715128718, artifact 10304268395. There are 351 types and 3,729 property occurrences,
including inherited and schema-filtered properties. All listed types have both
normal and `--all` responses. Type occurrences are not parity coverage.

Raw capture ZIP SHA-256:
`72905a44b8d2a955d7124c9c2b72098b1f0205e472c4d837e967c121dd99828a`.
The snapshot provenance retains the recording and raw-output-inventory digests.
Compiler source metadata is captured separately: 331 registered types, 201 canonical
ObjectSpec variants, the SceneSpec root and the ObjectSpec envelope. Neither
inventory proves runtime behavior. Live reports read current sources and detect
drift independently of JSON formatting or object-key order.

## Retention history

The first candidate was 33,972 bytes and was normalized independently in the
development container and the retention job, with identical SHA-256
`cb46f6bd7e1e8636ba7bfc00975ee7a2357ef9bacdb55285982391c3883d8bcf`.
The retention job required that digest, refused existing files and committed only
the two candidate files on the draft branch. Its temporary writer is not shipped.

Review of `0ca6046` then found that the normalizer omitted the canonical root
contract. Removing a root requirement, changing a root field constraint/reference,
or adding an ObjectSpec envelope constraint could incorrectly pass the drift check.
The completed snapshot adds only `compiler_metadata.canonical.root` and
`compiler_metadata.canonical.object_spec`, from the original canonical source whose
SHA-256 was already recorded as
`53ac8ddf5bb732e2fb06a59e8fae63d6ac86dc1be243e67a0be1c99a83c835b6`.
That source was hash-checked before normalization. Removing the two added members
reproduces the entire previous snapshot document, including all official facts and
provenance. This is a correction to the initial snapshot's coverage, not a new
capture or an acceptance of changed compiler behavior. The first candidate remains
in Git history.

The completed archive is 34,100 bytes, SHA-256
`6a879379e557eb45ecbdc4da958aa4032f7b4ca3ebef884ca3426dd79eb8b61f`.
Subsequent refreshes are explicit new candidates for review, never automatic updates.

Attribution: Rive, Inc. These are normalized public-interface facts, not copied
property descriptions, documentation, sample artwork, tools or installation files.
Original normalizer/report/test code is under the repository MIT license. That
license does not relicense Rive's executable or establish distribution rights.
The actual B1 runtime capture remains separately pinned and unchanged.
