# Compile with caller-owned assets

`compile_scene_with_assets` accepts explicit image and font sources without a
scene directory. It uses the same SceneSpec builder and binary encoder as existing
CLI and application callers. [The evidence record](../tests/evidence/embedded-assets/README.md)
separates legacy byte compatibility from official-runtime image/font proof.

```rust
use rive_cli::builder::SceneSpec;
use rive_cli::compile::{CompileOptions, compile_scene_with_assets};
use rive_cli::compile::assets::{AssetLimits, MemoryAssets};

fn compile(
    scene: &SceneSpec,
    font: Vec<u8>,
    image: Vec<u8>,
) -> Result<Vec<u8>, rive_cli::compile::EmbeddedCompileError> {
    let mut assets = MemoryAssets::new();
    assets.insert("memory://font", font);
    assets.insert("memory://image", image);
    compile_scene_with_assets(
        scene,
        CompileOptions {
            file_id: 42,
            asset_limits: AssetLimits {
                per_asset_bytes: 8 * 1024 * 1024,
                total_bytes: 16 * 1024 * 1024,
            },
        },
        &assets,
    )
}
```

The SceneSpec's font and image declarations use those exact strings in `source`.
They are lookup keys, not URLs or filesystem paths, when using `MemoryAssets`.
There is no key normalization, disk lookup, network fallback, process exit,
logging, or output writing in this path. A missing key returns `asset-missing`.
The caller decides where the returned `.riv` bytes go.

A declaration without `source` still means an external/unembedded asset, as it did
before this change. The compiler emits its descriptor without invoking a resolver.
That is not a failed lookup fallback, and it is not a promise that a downstream
runtime will avoid fetching explicitly external assets. Use a `source` for every
font/image that must be self-contained. Audio authoring is unchanged.

## Ownership and repeatability

`MemoryAssets` owns inserted buffers. A direct resolver call borrows them; a
compilation session returns owned payloads to the object builder. Neither call
consumes or mutates the registered bytes. The resolver is borrowed for the duration
of a compilation, and `.riv` output has no dependency on its lifetime.

An `AssetResolver` implementation must return the same bytes for a source during
one compilation. Repeated compilations are deterministic for the same SceneSpec,
file ID, limits, and asset bytes. Custom resolvers and filesystem inputs must be
stable while a compile runs. Snapshot mutable external inputs before compiling
when repeatability matters. The library does not promise deterministic output
from a deliberately stateful custom resolver.

## Limits and diagnostics

Defaults are 16 MiB per embedded payload and 64 MiB across the file. Limits are
inclusive. Reusing a source in two distinct asset declarations counts both emitted
payloads; the total applies across every artboard. Failed resolutions do not
consume the session's total. When both limits are equally restrictive, the
per-asset error takes precedence.

The session passes the tighter remaining limit to the resolver and checks returned
lengths independently before copying a borrowed payload. The filesystem adapter
checks metadata and reads at most the requested limit plus one sentinel byte.
These limits bound supplied asset bytes, not total compiler/process memory or the
allocations and side effects of a custom resolver.

Errors retain the SceneSpec asset name and source key, not a runtime ordinal or a
canonical host path. Codes distinguish missing, unavailable, empty, known
image/font kind mismatch, per-asset limits, total limits, relative-source policy,
project containment, and a missing filesystem base directory. Signature checks
recognize common image/font families only; they are not full file validation.
Unknown signatures retain legacy behavior and require runtime validation.

## Existing callers and host responsibilities

`compile_scene(scene, base_dir, file_id)` keeps its signature and legacy
`CompileError::Build` error type and `invalid-scene` classification.
`builder::build_scene` keeps its string-error interface. Both forward through the
same resolver-aware builder; there is no second scene construction or encoding
pipeline. The new compilation entry point exposes typed asset diagnostics as
`EmbeddedCompileError`, without adding variants to the legacy error enum.

The builder completes its existing structural and object-construction checks
before invoking the resolver. Explicit sources reserve contents positions in that
same graph, then bounded byte resolution fills them without changing indices.
No unresolved contents slot can be returned or encoded. Invalid placements and
late reference failures therefore cannot trigger caller asset-loading side effects.
Custom resolvers remain responsible for their own effects on otherwise valid
scenes, including failures partway through resolving several assets.

`FilesystemAssets` owns path canonicalization, project-root discovery and
containment. Object construction performs no asset I/O. The adapter requires an
explicit base directory. It allows parent-relative sources inside the discovered
project, rejects absolute/drive-rooted sources and symlink escapes, and never
falls back after a failed resolution. Its containment check assumes the filesystem
is not being maliciously mutated between resolution and open; it is not a
race-proof filesystem sandbox.

AuthoringSpec still lowers through the existing compiler-owned SceneSpec and
source-map state. No schema, identity, animation index, RML adapter, workspace
layout, or consumer-specific compiler is introduced here. Portable crate extraction
is the subsequent #259 work, not a capability claimed by this API addition.

## Verification

`tests/asset_resolution_contract.rs` covers resolver ownership, literal lookup,
limits, diagnostics and filesystem policies. `tests/embedded_compile_contract.rs`
checks the public memory API, file IDs, asset order, byte equality against the old
path entry, global budgets, validation/callback order, and successful legacy fixture
hashes captured before the builder change. Invalid nested assets, deep nodes and
unresolved nested-artboard targets must all fail without a resolver callback.

`cargo run --locked --quiet --example embedded_asset_compatibility` writes the
compatibility manifest to stdout. The committed baseline was captured with the
unchanged compilation path from `dfdf746e923a4935ab1d78c30425588d95bebcd5`;
do not refresh it from the implementation merely to make a mismatch pass. Skipped
malformed or unsupported inputs are listed separately from successful hashes.

`tests/embedded_assets_runtime.rs` compiles the image/font scene from memory and
renders through the bundled official runtime. The image-byte control keeps the
entire scene and image dimensions fixed and varies only the supplied image buffer.
Every image pixel must take that buffer's replacement color; every non-image pixel
must stay identical. The font-byte control must match the removed-text control,
while the supplied image remains visible. Repeat captures must match exactly.

Set `RIVE_MEMORY_ASSET_EVIDENCE` to retain PNGs, `.riv` artifacts and input/runtime
digests. The ordinary Rust CI job also retains those artifacts plus source,
toolchain and browser identity. A compiled test or structural parse alone is not
runtime proof.
