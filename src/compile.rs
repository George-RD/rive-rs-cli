use std::path::Path;

use thiserror::Error;

use crate::builder::{self, SceneSpec};
use crate::encoder;
use crate::objects::core::RiveObject;

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

pub fn compile_scene(
    spec: &SceneSpec,
    base_dir: Option<&Path>,
    file_id: u64,
) -> Result<Vec<u8>, CompileError> {
    let scene = builder::build_scene(spec, base_dir).map_err(CompileError::Build)?;
    let objects: Vec<&dyn RiveObject> = scene.iter().map(|object| &**object).collect();

    Ok(encoder::encode_riv(&objects, file_id))
}
