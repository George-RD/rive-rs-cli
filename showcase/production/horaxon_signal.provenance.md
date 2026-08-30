# Horaxon signal-to-action production provenance

This record documents a production-consumer proof. It does not claim independent customer endorsement, adoption, or performance results.

## Consumer evidence

The Horaxon consumer repository is private, so this public repository retains only the minimum provenance metadata needed to support the public claim; it does not copy the private consumer source.

- Consumer repository: `George-RD/horaxon-web`
- Consumer commit inspected for this record: `feb944c3ec0e4004d57d41215c8a1110d2c8a3d1`
- Consumer path: `src/scripts/rive-signal.ts`
- Consumer Git blob: `7566c6494d99db242ec142e3cb20a85d56a5d9a0`
- Public attestation: `showcase/production/horaxon_consumer.attestation.json`
- The inspected consumer pins the Rive file to rive-cli commit `b5b5823b994aa175e4a24983d913f40386c0ef9b` at `scratch/horaxon-signal-to-action.riv`.
- The inspected consumer requests animation `auth__horaxon_2dsignal_2dstage__signal_2dto_2daction__animation`.

The relevant production pin recorded from that consumer is:

```text
https://raw.githubusercontent.com/George-RD/rive-rs-cli/b5b5823b994aa175e4a24983d913f40386c0ef9b/scratch/horaxon-signal-to-action.riv
```

Public CI validates the attestation against this manifest and the immutable rive-cli origin. It does not claim to re-fetch or independently prove the private repository contents without private-repository credentials.

## rive-cli origin evidence

The pinned rive-cli commit is `b5b5823b994aa175e4a24983d913f40386c0ef9b`, whose commit message is `scratch: compile continuous Horaxon signal-to-action proof`.

At that commit:

- artifact: `scratch/horaxon-signal-to-action.riv`
- artifact Git blob: `31114fbe834aeb72c527152bd74ee5420e649588`
- AuthoringSpec source: `scratch/horaxon-signal.authoring.json`
- source Git blob: `d53549270d624e170beaca846036aec266fe4d9c`
- generator helper: `scratch/build_horaxon_signal.py`
- retained runtime evidence: `scratch/horaxon-render/`

The public showcase stages the exact artifact and AuthoringSpec blobs above under stable `showcase/production/` paths. Normal site playback therefore does not depend on the historical raw-GitHub URL.

## Claim boundary

This Horaxon-specific AuthoringSpec and `.riv` are separate from `examples/authoring/complex-animated-showcase.v0.json` and its generated artifact. Both express a signal-to-action story, but they are not presented as byte-identical or as the same final authoring path.
