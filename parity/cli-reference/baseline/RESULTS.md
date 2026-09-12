# Observed reference: CLI 1.0.2

Captured on 2026-09-12 from `f878465e8d3b493e567d1556130880b84a7ec8df`.
[Run 34693899414](https://github.com/George-RD/rive-rs-cli/actions/runs/34693899414)
passed; source artifact 10297913092 is retained here as `capture.tar.xz`.
`capture.json` pins the original ZIP digest, repacked archive digest and source
head. All 87 source files (recording plus 86 artifacts) are unchanged.
This is historical execution evidence, not a claim that later checkouts reran it.

## Environment

Official `rive 1.0.2`, RML version 1, Linux x86_64 (glibc 2.39), repository canvas
2.39.1 and Google Chrome for Testing 151.0.7922.34. The executable SHA-256 was
`4d7a219c00f6978b91fb3d4d7f2db694ffc11afe2f1ae7c2b6e7a386a3642fef`.
The locked archive, runtime/harness and browser digests, Rust toolchain, exact
commands and clean source-head checks are in the retained `recording.json` and
`commands/` files. `lock.json` gives the acquisition source and runtime blob pins.

All official commands ran without network access in empty HOME/XDG directories,
with credential/proxy variables excluded. Tool provisioning and the browser
renderer were not network-isolated; do not generalize the offline claim to them.

## Compiled bytes and runtime results

| Observation | Static shape/paint | Quarter-turn animation |
| --- | --- | --- |
| Unsigned output | 107 bytes, 6 objects | 149 bytes, 11 objects |
| Repeated build | Byte-identical | Byte-identical |
| Frames at 0, 15, 30 | Nonblank and identical | Nonblank and distinct; QuarterTurn selected |
| Self-comparison | 0% pixels; zero type/count deltas | 0% pixels; zero type/count deltas |
| Paint-perturbed comparison | 8.59375% at each frame | 8.59375%, 9.27734375%, 8.968098958333332% |

Both used 128x96 pixels, scale 1, 60 fps. Changing only the paint kept structure
unchanged and produced the expected visible differences. PNG captures were also
visually inspected: the static rectangle stays fixed, and the animated rectangle
rotates through the three samples. Inspection reported `no-default-artboard` and
`artboard-without-style` warnings, not errors; these fixed-position, single-artboard
fixtures rendered successfully without introducing layout or state machines.

Original output SHA-256:

- Static: `1ddd2ceb6fa49ed3cfdc712cc91a3020c6050ed5842df1e0c39ba3c2482c37f3`.
- Animated: `5a26b749125353c2349ec947b925a4a8cdd893ca0251ee4e3e940cb14dc69a6d`.

## Public command and account observations

All ten audit probes exited zero offline: `schema Rectangle`, `schema Shape
--animatable`, `schema Fill`, `schema SolidColor`, `schema LinearAnimation`,
`schema KeyFrameDouble`, `docs format`, `docs --list`, `samples --path`, and
`--help`. Their output digests are retained, not copies of proprietary docs or
samples. Agent-scaffold guidance was reviewed from public documentation only;
`rive create`, copied sample projects and generated guidance were not exercised.

`--publish` and `--rev=build/probe.rev` both exited 3 and reported
`Not logged in. Run: rive login`. No login, account binding, successful publishing
or editor export occurred. This combined no-login/no-network probe does not
separate network requirements from account restrictions or establish paid rights.
Scripts, shaders, signing and production distribution remain untested.

## Earlier attempts

Run 34693047769 verified the archive and built our CLI but failed loading the
official binary because `libEGL.so.1` was absent. Explicit Linux provisioning fixed
that observed dependency. Run 34693535512 compiled and rendered the static fixture
but failed our adapter's incorrect expectation that render used compare's `ok`
JSON field. The corrected adapter accepts render's existing raw manifest, rejects
explicit failures, and passes scale 1 explicitly. Neither partial run is presented
as a successful baseline.
