# Retained snapshot review gate

`capabilities.py check` checks current source metadata and candidate evidence.
It cannot by itself detect replacing both a retained archive and its checksum:
that requires a pre-change source of truth. The schema workflow therefore checks
out the pull request's base commit separately, without executing its code, and
runs this additional read-only gate:

```sh
python3 parity/cli-reference/schema_retention.py --base-root /path/to/base-checkout
```

The command validates both retained snapshots and manifests, then reports schema,
compiler, provenance and manifest differences. It exits 0 for unchanged evidence,
1 for an unreviewed change, and 2 for invalid evidence. Recomputing the candidate's
manifest digest cannot make a changed type key pass against the base snapshot.
An incomplete old snapshot is an error, not an initial retention.

An intentional replacement has an explicit reviewed pair:

```sh
python3 parity/cli-reference/schema_retention.py --base-root /path/to/base-checkout \
  --reviewed-change OLD_SHA256:NEW_SHA256
```

The pair must match both actual archive identities. It does not erase the diff:
output still has `changed: true`, every difference and `reviewed_change: true`.
This is an explicit review input, not authentication or execution attestation.
A maintainer updates the workflow's pair only after examining the reported facts,
provenance and retained artifacts. There is no automatic approval from the new
manifest or a pull-request body marker.

For this first retention, the old identity is `absent` and the workflow allows
only the independently verified archive
`6a879379e557eb45ecbdc4da958aa4032f7b4ca3ebef884ca3426dd79eb8b61f`.
Once a baseline exists, that initial exception cannot authorize a replacement.
A modified first archive also fails even when its checksum is updated. The initial
snapshot's bounded canonical-root correction and historical identities remain in
[schema-baseline/README.md](schema-baseline/README.md).

Snapshot JSON rejects duplicate members at every depth, including compressed
candidates. Repeated enum or bit continuations fail instead of overwriting earlier
facts. The stricter parser was run against the actual 351-type capture; all facts
and provenance match the retained snapshot. Neither retained archive was changed
by these review fixes.
