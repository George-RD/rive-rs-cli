# Pinned official CLI reference

Development-only work for [#258](https://github.com/George-RD/rive-rs-cli/issues/258).
The runner is implemented; a successful official compilation/runtime recording is
not yet retained. Offline contract tests are not runtime evidence. Do not close
#258 or call this a compatibility baseline until the real capture passes.

## Run

Use Linux x86_64, Python 3.11+, Rust, Git, Chromium and `unshare` with user/network
namespaces enabled, plus the CLI's Linux shared libraries (Ubuntu: `libegl1 libgles2`). Start from a clean committed checkout. Provision tools before
the experiment; compilation itself is tested without network or credentials.

Acquire the archive named by `lock.json` from its versioned source. Verify its
SHA-256, extract it outside the repository, and keep the archive for verification.
The runner checks the supplied executable against the archive member as well as
checking its reported version. It never installs, downloads or updates the CLI.

```sh
python3 parity/cli-reference/reference.py capture \
  --official-cli /absolute/path/to/rive \
  --archive /absolute/path/to/rive-linux-x64.tar.gz \
  --browser /absolute/path/to/chromium \
  --output target/official-reference

python3 parity/cli-reference/reference.py verify target/official-reference \
  --expected-head "$(git rev-parse HEAD)"
python3 -m unittest discover -s parity/cli-reference -p 'test_*.py' -v
```

`capture` refuses an existing output directory. There is no update-baseline flag.
`verify` only checks retained files and their measurements; it executes no tool and
makes no claim to have rerun the experiment. Supply the independently recorded
commit SHA for a historical capture; a different expected head is rejected. Normal Cargo builds/tests do not call
the official CLI. Ordinary CI runs only the Python contract tests.

The separate `CLI reference` workflow provisions the exact locked archive only
when explicitly requested. Use workflow dispatch, or deliberately edit a same-repo
PR body to add `<!-- run-official-reference:1.0.2 -->`. Only adding the marker
starts a capture: retaining it during a bot/description edit does not. Remove and
re-add it to request another capture. A synchronize or PR creation does not opt in. The workflow retains the evidence, never
the downloaded executable, archive, installation files or isolated HOME.

## What the experiment measures

Two original RML fragments contain a filled rectangle and the same rectangle with
a quarter-turn timeline. Each is verified, built twice to check byte repeatability,
and inspected by the official CLI. Every official command runs in a new network
namespace, with an empty temporary HOME/XDG configuration and an environment
allowlist. Analytics are disabled. No user session is read or login attempted.

The adapter builds this checkout's `rive-cli`, checks the existing runtime and
harness pins, and calls its `render` and `compare` commands. Static frames must
remain identical; three animation samples must be nonblank and distinct. Each
reference must compare exactly with itself. An independently compiled copy with
one changed paint must produce a measured visible difference above 0.1 percent.
A failed renderer/comparator is an error, never a successful negative control.

The runtime is deliberately pinned to the repository's existing canvas 2.39.1
assets. Compatibility with CLI 1.0.2 is **unverified until capture succeeds**. An
unsupported file, empty artboard, blank frame or missing animation fails the run.
There is no fallback renderer or automatic runtime upgrade. An intentional pin
change needs review and a separately identified capture, not a silent replacement.
Browser rendering uses local bundled assets but is not network-isolated; only the
official CLI's command executions establish offline behavior.

## Evidence and limits

`recording.json` includes source head, planning base, tool versions and digests,
runtime/harness identities, RML version, commands, exit codes, input/output hashes,
frame captures and self/perturbed comparisons. A partial run is marked failed.
The offline verifier requires complete command/output/artifact records and rejects
changed inputs, missing outputs, escaping paths, nonfinite metrics and unmeasured
pass claims. These records establish provenance and internal consistency, not a
cryptographic attestation that a trusted machine executed them.

The no-credential/no-network `--publish` and `--rev` probes must return a documented
authentication/network failure. That combined experiment cannot distinguish an
account restriction from network availability and does not establish what a paid
account can export. It does not call `login`, bind a project, publish successfully
or create an editor file. Scripts, shaders, signing, watermark-free distribution
and deployment rights remain untested. Successful playback of these two unsigned
files must not be generalized to those capabilities.

See [PROVENANCE.md](PROVENANCE.md) for source and sample licensing boundaries and
[the Cairn todo](../../meta/todos/todo.official-cli-reference-seed.md) for status.
