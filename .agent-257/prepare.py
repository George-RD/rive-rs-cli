from hashlib import sha1
from pathlib import Path

BASE_BLOBS = {
    "src/builder/mod.rs": "e52c7b5f500da259bca307a1088aae5444a1a3c9",
    "src/builder/scene.rs": "fdda81bdbf96676e2a35d14d0b2457d84978fff4",
    "src/builder/objects.rs": "8cf5642d9ca3be3bde6640baeba01fb38015e69f",
}


def checked(path):
    data = Path(path).read_bytes()
    actual = sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()
    if actual != BASE_BLOBS[path]:
        raise RuntimeError(f"{path}: expected source blob {BASE_BLOBS[path]}, got {actual}")
    return data.decode()


def replace_once(text, old, new):
    if text.count(old) != 1:
        raise RuntimeError(f"expected exactly one patch anchor: {old[:100]!r}")
    return text.replace(old, new, 1)


objects = checked("src/builder/objects.rs")
objects = replace_once(objects, "use std::path::{Component, Path, PathBuf};\n", "")
start = objects.index("pub(crate) fn append_file_asset(")
end = objects.index("#[allow(clippy::too_many_arguments)]\npub(crate) fn append_object(", start)
objects = objects[:start] + '''pub(crate) fn append_file_asset(
    spec: &ObjectSpec,
    objects: &mut Vec<Box<dyn RiveObject>>,
    contents: Option<Vec<u8>>,
) {
    match spec {
        ObjectSpec::ImageAsset {
            name,
            asset_id,
            cdn_base_url,
            ..
        } => {
            let mut asset = ImageAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
        }
        ObjectSpec::FontAsset {
            name,
            asset_id,
            cdn_base_url,
            ..
        } => {
            let mut asset = FontAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
        }
        ObjectSpec::AudioAsset {
            name,
            asset_id,
            cdn_base_url,
        } => {
            let mut asset = AudioAsset::new(name.clone());
            if let Some(v) = asset_id {
                asset.asset_id = *v;
            }
            if let Some(v) = cdn_base_url {
                asset.cdn_base_url = v.clone();
            }
            objects.push(Box::new(asset));
        }
        _ => return,
    }
    if let Some(bytes) = contents {
        objects.push(Box::new(FileAssetContents::new(bytes)));
    }
}

''' + objects[end:]

scene = checked("src/builder/scene.rs")
scene = replace_once(scene, "use super::animations::{build_animations, register_interpolators};", '''use super::BuildError;
use super::animations::{build_animations, register_interpolators};
use super::assets::{AssetKind, AssetLimits, AssetRequest, AssetResolver, AssetSession, FilesystemAssets};''')
scene = replace_once(scene, '''pub fn build_scene(
    spec: &SceneSpec,
    base_dir: Option<&Path>,
) -> Result<Vec<Box<dyn RiveObject>>, String> {
    let indexes = validate_scene_spec(spec)?;''', '''pub fn build_scene(
    spec: &SceneSpec,
    base_dir: Option<&Path>,
) -> Result<Vec<Box<dyn RiveObject>>, String> {
    build_scene_with_assets(spec, &FilesystemAssets::new(base_dir), AssetLimits::default())
        .map_err(|error| error.to_string())
}

pub(crate) fn build_scene_with_assets(
    spec: &SceneSpec,
    resolver: &dyn AssetResolver,
    limits: AssetLimits,
) -> Result<Vec<Box<dyn RiveObject>>, BuildError> {
    let indexes = validate_scene_spec(spec)?;
    let mut assets = AssetSession::new(resolver, limits);''')
scene = replace_once(scene, "for (artboard_spec, index) in artboard_specs.iter().zip(&indexes) {", "for index in &indexes {")
scene = replace_once(scene, '''                    "asset '{asset_name}' is declared more than once; Rive stores assets at file scope, so their names must be unique across every artboard"
                ));''', '''                    "asset '{asset_name}' is declared more than once; Rive stores assets at file scope, so their names must be unique across every artboard"
                ).into());''')
scene = replace_once(scene, '''        for child in &artboard_spec.children {
            if file_asset(child).is_some() {
                append_file_asset(child, &mut objects, base_dir)?;
            }
        }
    }
    let mut view_model_id_base''', '''    }
    for artboard_spec in &artboard_specs {
        for child in &artboard_spec.children {
            if file_asset(child).is_some() {
                let contents = embedded_asset_request(child)
                    .map(|request| assets.resolve(request))
                    .transpose()?;
                append_file_asset(child, &mut objects, contents);
            }
        }
    }
    let mut view_model_id_base''')
scene = replace_once(scene, "#[cfg(test)]\nmod tests {", '''fn embedded_asset_request(spec: &ObjectSpec) -> Option<AssetRequest<'_>> {
    match spec {
        ObjectSpec::ImageAsset { name, source: Some(source), .. } => Some(AssetRequest {
            name,
            source,
            kind: AssetKind::Image,
        }),
        ObjectSpec::FontAsset { name, source: Some(source), .. } => Some(AssetRequest {
            name,
            source,
            kind: AssetKind::Font,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {''')

builder = checked("src/builder/mod.rs")
builder = replace_once(builder, "mod animations;\n", "mod animations;\npub mod assets;\n")
builder = replace_once(builder, "pub use spec::SceneSpec;\n", '''pub use spec::SceneSpec;
pub(crate) use scene::build_scene_with_assets;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BuildError {
    #[error("{0}")]
    Build(String),
    #[error(transparent)]
    Asset(#[from] assets::AssetError),
}

impl From<String> for BuildError {
    fn from(error: String) -> Self {
        Self::Build(error)
    }
}

impl BuildError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Build(_) => "invalid-scene",
            Self::Asset(error) => error.code(),
        }
    }
}

''')

compile_source = '''use std::path::Path;

use thiserror::Error;

use crate::builder::{self, SceneSpec};
use crate::encoder;
use crate::objects::core::RiveObject;

pub use crate::builder::BuildError as EmbeddedCompileError;
pub use crate::builder::assets;

use assets::{AssetLimits, AssetResolver, FilesystemAssets};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CompileError {
    #[error("{0}")]
    Build(String),
}

impl CompileError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Build(_) => "invalid-scene",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CompileOptions {
    pub file_id: u64,
    pub asset_limits: AssetLimits,
}

pub fn compile_scene(
    spec: &SceneSpec,
    base_dir: Option<&Path>,
    file_id: u64,
) -> Result<Vec<u8>, CompileError> {
    compile_scene_with_assets(
        spec,
        CompileOptions { file_id, ..CompileOptions::default() },
        &FilesystemAssets::new(base_dir),
    ).map_err(|error| CompileError::Build(error.to_string()))
}

pub fn compile_scene_with_assets(
    spec: &SceneSpec,
    options: CompileOptions,
    assets: &dyn AssetResolver,
) -> Result<Vec<u8>, EmbeddedCompileError> {
    let scene = builder::build_scene_with_assets(spec, assets, options.asset_limits)?;
    let objects: Vec<&dyn RiveObject> = scene.iter().map(|object| &**object).collect();
    Ok(encoder::encode_riv(&objects, options.file_id))
}
'''

blueprint = Path("cairn.blueprint").read_text()
blueprint = replace_once(blueprint, '                "./tests/compile_contract.rs",', '''                "./tests/asset_resolution_contract.rs",
                "./tests/compile_contract.rs",
                "./tests/embedded_compile_contract.rs",
                "./tests/embedded_assets_runtime.rs",
                "./tests/evidence/embedded-assets",''')
blueprint = replace_once(blueprint, 'path ["./showcase", "./demo", "./examples/authoring"]', 'path ["./showcase", "./demo", "./examples/authoring", "./examples/embedded_asset_compatibility.rs"]')

blueprint += '\nrive-cli.delivery.examples -> rive-cli.core.compile "Exercises legacy fixture byte compatibility"\nrive-cli.delivery.examples -> rive-cli.core.builder "Parses canonical fixture scenes"\nrive-cli.delivery.examples -> rive-cli.intelligence.authoring "Exercises lowered authoring fixture compatibility"\n'

roadmap = Path("ROADMAP.md").read_text()
roadmap = replace_once(roadmap, "## Current implementation frontier\n", '''## Current implementation frontier

Embedding and official-reference spec: [#256](https://github.com/George-RD/rive-rs-cli/issues/256).
The [execution graph](https://github.com/George-RD/rive-rs-cli/issues/257#issuecomment-5644596712)
records the stacked PR units and actual blockers. [#257](https://github.com/George-RD/rive-rs-cli/issues/257)
is in progress on `agent/256-a1-memory-assets`; its [Cairn todo](meta/todos/todo.embedded-memory-assets.md)
records the implementation and outstanding verification gates. #259 remains blocked
until #257 is verified and merged. #258 is an independent reference-tooling unit
and can proceed in parallel; this compiler change does not implement or close it.

''')

Path("src/builder/assets").mkdir(parents=True, exist_ok=True)
Path("src/compile/assets.rs").rename("src/builder/assets.rs")
Path("src/compile/assets/filesystem.rs").rename("src/builder/assets/filesystem.rs")
for path, text in {
    "src/builder/objects.rs": objects,
    "src/builder/scene.rs": scene,
    "src/builder/mod.rs": builder,
    "src/compile.rs": compile_source,
    "cairn.blueprint": blueprint,
    "ROADMAP.md": roadmap,
}.items():
    Path(path).write_text(text)
Path(".agent-257/prepare.py").unlink()
print("Applied the #257 source patch; no compiler code generation or temporary job is required at runtime.")
