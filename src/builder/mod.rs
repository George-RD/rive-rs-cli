mod animations;
mod objects;
mod parsers;
pub mod scene;
pub(crate) mod spec;
mod state_machines;
mod validation;

pub use scene::{artboard_presets, build_scene};
pub use spec::SceneSpec;
