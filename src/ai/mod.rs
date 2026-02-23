pub mod config;
pub mod error;
pub mod provider;
pub mod repair;
pub mod templates;
pub use config::AiConfig;
pub use error::AiError;
pub use provider::{AiProvider, create_provider};
pub use repair::{RepairEngine, RepairResult, format_repair_summary, remediation_hints};
