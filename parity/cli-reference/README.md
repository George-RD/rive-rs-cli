# Pinned official CLI reference

Development-only reference for [#258](https://github.com/George-RD/rive-rs-cli/issues/258).
The retained run compiled and inspected both original RML fixtures without login
or network, then exercised their unsigned bytes in the existing canvas 2.39.1
runtime. Both self-comparisons and paint-perturbed negative controls passed.
See [baseline/RESULTS.md](baseline/RESULTS.md) for measured results and limitations.

## Check the retained baseline offline

```sh
python3 -m unittest discover -s parity/cli-reference -p 'test_*.py' -v
```

This includes the actual capture in `baseline/capture.tar.xz`, pinned by
`baseline/capture.json`. Its original files are unchanged; the test checks the
archive digest, extracts into a temporary directory and verifies the evidence.
It forbids subprocess execution during verification. No official CLI, Rust,
browser, account or network is needed for these offline checks (Python 3.11+).
They establish retained evidence consistency, not a fresh runtime execution.

## Make a new reference capture

Use Linux x86_64, Python 3.11+, Rust, Git, Chromium and `unshare` with user/network
namespaces enabled, plus the CLI's Linux libraries (Ubuntu: `libegl1 libgles2`).
Start from a clean committed checkout. Provision tools before the experiment.
Acquire the archive named by `lock.json`, verify its SHA-256, and extract it
outside the repository. Keep the archive: the runner checks the supplied
executable against its member as well as checking the reported CLI version.

```sh
python3 parity/cli-reference/reference.py capture \
  --official-cli /absolute/path/to/rive \
  --archive /absolute/path/to/rive-linux-x64.tar.gz \
  --browser /absolute/path/to/chromium \
  --output target/official-reference

python3 parity/cli-reference/reference.py verify target/official-reference \
  --expected-head "$(git rev-parse HEAD)"
```

The runner never installs or downloads tools and refuses an existing output
folder. There is no update-baseline flag. Use an independently recorded commit
SHA to verify a historical capture; a different expected head is rejected.
Normal Cargo builds/tests do not call the official CLI.

The separate `CLI reference` workflow provisions the locked archive only on
workflow dispatch or when a same-repo PR description newly adds
`<!-- run-official-reference:1.0.2 -->`. An existing marker, bot edit, ordinary PR
creation or synchronize event does not opt in. Remove and re-add the marker for
another run. CI uploads only evidence, not the downloaded executable, archive,
installation files or isolated HOME. Retain reviewed baselines in Git before
short-lived CI artifacts expire; do not silently replace an established pin.

## Assertions and boundaries

The runner verifies, builds twice and inspects original static and quarter-turn
projects. Every official command uses an isolated network namespace, empty
HOME/XDG directories and an environment allowlist with analytics disabled.
No user session is read or login attempted.

The adapter builds this checkout's `rive-cli` and reuses its `render` and `compare`
commands with the pinned existing runtime/harness. Static frames must be identical;
three animation samples must be nonblank and distinct. Each file must compare
exactly with itself. Changing one paint must produce a visible difference above
0.1 percent. A failed renderer/comparator is an error, not a negative-control pass.
No fallback renderer or automatic runtime upgrade can mask an unsupported file.
Browser rendering uses bundled local assets but is not network-isolated.

Records include source/tool identities, commands, exit codes, inputs, inspection,
output digests and PNG frames. Partial captures fail. The offline verifier rejects
missing/changed evidence, escaping paths, inconsistent source heads and invalid
measurements. It checks provenance and consistency, not execution attestation.

The retained no-credential `--publish` and `--rev` probes reported missing login.
Scripts, shaders, signing, watermark-free production use and distribution rights
remain untested. Playback of these two unsigned files establishes none of those
capabilities. See [PROVENANCE.md](PROVENANCE.md) for retention boundaries and
[the Cairn todo](../../meta/todos/todo.official-cli-reference-seed.md) for tracking.
