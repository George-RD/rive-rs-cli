#![allow(clippy::len_without_is_empty, clippy::new_without_default)]

pub mod ai;
pub mod authoring;
pub mod builder;
pub mod compare;
pub mod discovery;
pub mod encoder;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod objects;
pub mod render;
pub mod scaffold;
pub mod validator;
