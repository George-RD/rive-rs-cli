# Asset Extensions

Spec for new asset types: Folder, LayeredAsset, LayerImageAsset, SVGAsset, LottieAsset, ExportAudio, ScriptAsset, and BlobAsset.

## Type Keys

| Type | Key | C++ base |
|------|-----|----------|
| Folder | 102 | Component (10) |
| LayeredAsset | 119 | DrawableAsset (104) |
| LayerImageAsset | 120 | FileAsset (103) |
| SVGAsset | 132 | FileAsset (103) |
| LottieAsset | 133 | FileAsset (103) |
| ExportAudio | 422 | Component (10) |
| ScriptAsset | 529 | FileAsset (103) |
| BlobAsset | 649 | FileAsset (103) |

## Property Keys

From `generated_registry.rs`:

| Property | Key | Backing | Used by |
|----------|-----|---------|---------|
| name | 203 | String | All assets (inherited from Asset) |
| assetId | 204 | UInt | FileAsset-derived types |
| cdnBaseUrl | 362 | String | FileAsset-derived types |
| volume | 530 | Float | ExportAudio |
| folderPath | 926 | String | Folder |
| isModule | 914 | UInt (bool) | ScriptAsset |

### Existing property keys already in core.rs

- `ASSET_NAME` = 203
- `FILE_ASSET_ASSET_ID` = 204
- `FILE_ASSET_CDN_BASE_URL` = 362

## Implementation Details

### Folder (102)

Asset organization folder. Groups assets in the asset hierarchy.

```
Hierarchy: Component -> Folder
```

Properties:
- `name` (4, String) - folder name (uses Component.name, not Asset.name)
- `parentId` (5, UInt) - parent folder or root

Note: Folder uses Component properties (name=4, parentId=5), NOT Asset properties (name=203). This is because Folder inherits from Component in the C++ hierarchy, not from Asset.

### LayeredAsset (119)

Multi-layer asset container. Holds multiple LayerImageAsset children.

```
Hierarchy: Asset -> DrawableAsset -> LayeredAsset
```

Properties:
- `name` (203, String) - asset name

LayeredAsset is a container; the actual image layers are LayerImageAsset children.

### LayerImageAsset (120)

A single image layer within a LayeredAsset.

```
Hierarchy: Asset -> FileAsset -> LayerImageAsset
```

Properties:
- `name` (203, String) - layer/asset name
- `assetId` (204, UInt) - unique asset ID
- `cdnBaseUrl` (362, String) - CDN base URL for hosted assets

Follows the exact same pattern as ImageAsset (105) already implemented in `assets.rs`.

### SVGAsset (132)

SVG vector asset for embedding scalable vector graphics.

```
Hierarchy: Asset -> FileAsset -> SVGAsset
```

Properties:
- `name` (203, String) - asset name
- `assetId` (204, UInt) - unique asset ID
- `cdnBaseUrl` (362, String) - CDN base URL

Same FileAsset property pattern as ImageAsset/FontAsset/AudioAsset.

### LottieAsset (133)

Lottie animation asset for embedding Lottie JSON animations.

```
Hierarchy: Asset -> FileAsset -> LottieAsset
```

Properties:
- `name` (203, String) - asset name
- `assetId` (204, UInt) - unique asset ID
- `cdnBaseUrl` (362, String) - CDN base URL

Same FileAsset property pattern.

### ExportAudio (422)

Audio export configuration object. Controls how audio is exported/rendered.

```
Hierarchy: Component -> ExportAudio
```

Properties:
- `name` (4, String) - component name (Component inheritance)
- `parentId` (5, UInt) - parent component
- `volume` (530, Float) - audio volume level

Note: ExportAudio inherits from Component, so it uses `name` (4) and `parentId` (5), not the Asset name property (203).

### ScriptAsset (529)

Script/code asset for embedding executable scripts.

```
Hierarchy: Asset -> FileAsset -> ScriptAsset
```

Properties:
- `name` (203, String) - asset name
- `assetId` (204, UInt) - unique asset ID
- `cdnBaseUrl` (362, String) - CDN base URL
- `isModule` (914, UInt/Bool) - whether the script is a module (backing: UInt)

Extends the standard FileAsset pattern with a module flag.

### BlobAsset (649)

Generic binary blob asset for arbitrary data.

```
Hierarchy: Asset -> FileAsset -> BlobAsset
```

Properties:
- `name` (203, String) - asset name
- `assetId` (204, UInt) - unique asset ID
- `cdnBaseUrl` (362, String) - CDN base URL

Same FileAsset property pattern. Used for data that doesn't fit other asset categories.

## New Constants Needed in core.rs

### type_keys

```rust
pub const FOLDER: u16 = 102;
pub const LAYERED_ASSET: u16 = 119;
pub const LAYER_IMAGE_ASSET: u16 = 120;
pub const SVG_ASSET: u16 = 132;
pub const LOTTIE_ASSET: u16 = 133;
pub const EXPORT_AUDIO: u16 = 422;
pub const SCRIPT_ASSET: u16 = 529;
pub const BLOB_ASSET: u16 = 649;
```

### property_keys

```rust
pub const EXPORT_AUDIO_VOLUME: u16 = 530;
pub const SCRIPT_ASSET_IS_MODULE: u16 = 914;
pub const FOLDER_PATH: u16 = 926;
```

Note: `ASSET_NAME` (203), `FILE_ASSET_ASSET_ID` (204), `FILE_ASSET_CDN_BASE_URL` (362) already exist.

## File Location

All types go in `src/objects/assets.rs` alongside ImageAsset, FontAsset, AudioAsset, and FileAssetContents.

## Struct Designs

### FileAsset-derived types (LayerImageAsset, SVGAsset, LottieAsset, BlobAsset)

All follow the exact same struct pattern as ImageAsset:

```rust
pub struct SVGAsset {
    pub name: String,
    pub asset_id: u64,
    pub cdn_base_url: String,
}
```

With the `new(name)` constructor defaulting `asset_id` to 0 and `cdn_base_url` to empty string. Properties emitted: always `name`, conditionally `asset_id` (when != 0) and `cdn_base_url` (when non-empty).

### ScriptAsset

```rust
pub struct ScriptAsset {
    pub name: String,
    pub asset_id: u64,
    pub cdn_base_url: String,
    pub is_module: bool,
}
```

Same as FileAsset pattern plus `is_module` emitted as UInt (0/1) when true.

### Folder

```rust
pub struct Folder {
    pub name: String,
    pub parent_id: u64,
}
```

Uses Component properties (4, 5), NOT Asset properties.

### LayeredAsset

```rust
pub struct LayeredAsset {
    pub name: String,
}
```

Name-only; children are LayerImageAsset objects.

### ExportAudio

```rust
pub struct ExportAudio {
    pub name: String,
    pub parent_id: u64,
    pub volume: f32,
}
```

Emit volume only when != 1.0 (default volume).

## Test Coverage

Each type needs:
1. Type key assertion
2. Default properties test (name only for FileAsset types)
3. Full properties test with asset_id and cdn_base_url set
4. Verify no parentId emission for FileAsset-derived types (they are top-level assets, not in the component tree)
5. Verify Folder uses Component properties (4, 5), not Asset properties (203)
6. Verify ExportAudio uses Component properties (4, 5) plus volume (530)
