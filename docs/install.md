# Installation

## From Prebuilt Release Artifacts

1. Open the latest release page: https://github.com/George-RD/rive-rs-cli/releases
2. Download the archive for your platform:
   - macOS Intel: `x86_64-apple-darwin.tar.gz`
   - macOS Apple Silicon: `aarch64-apple-darwin.tar.gz`
   - Linux glibc x64: `x86_64-unknown-linux-gnu.tar.gz`
   - Windows x64: `x86_64-pc-windows-msvc.zip`
3. Extract and place the binary on your `PATH`.

## From Source (Cargo)

```bash
# Clone and build release binary
git clone https://github.com/George-RD/rive-rs-cli.git
cd rive-rs-cli
cargo build --release

# Binary path
./target/release/rive-cli --help
```

## Verify Installation

```bash
rive-cli --help
rive-cli inspect --help
rive-cli validate --help
rive-cli generate --help
```
