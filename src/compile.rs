use std::path::Path;

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
        CompileOptions {
            file_id,
            ..CompileOptions::default()
        },
        &FilesystemAssets::new(base_dir),
    )
    .map_err(|error| CompileError::Build(error.to_string()))
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
