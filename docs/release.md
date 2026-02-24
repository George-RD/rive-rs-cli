# Release Process

This project ships prebuilt binaries from Git tags via GitHub Actions.

## Supported Release Targets

- `x86_64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`

Artifacts are uploaded to the GitHub Release with SHA256 checksums.

## Maintainer Flow

```bash
# 1) Ensure local main is clean and up to date
git checkout main
git pull origin main

# 2) Bump Cargo.toml version
# edit version = "X.Y.Z"

# 2b) Update CHANGELOG.md
# move items from [Unreleased] into [X.Y.Z] with date

# 3) Run quality checks
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# 4) Commit the version bump
git add Cargo.toml CHANGELOG.md
git commit -m "chore: release vX.Y.Z"
git push origin main

# 5) Create and push tag (must match Cargo.toml version)
git tag vX.Y.Z
git push origin vX.Y.Z
```

## Workflow Behavior

- Workflow file: `.github/workflows/release.yml`
- Trigger: push tags matching `v*`
- Guardrail: tag version must match `Cargo.toml` package version
- Build mode: `cargo build --release --locked --target <target>`
- Packaging:
  - Unix targets: `tar.gz` containing `rive-cli`
  - Windows target: `.zip` containing `rive-cli.exe`
- Publishing: `softprops/action-gh-release` uploads all target archives and `checksums.txt`

## Verifying a Published Release

```bash
# Download one artifact and checksum file from the release page, then verify
sha256sum -c checksums.txt
```

On macOS, use:

```bash
shasum -a 256 -c checksums.txt
```
