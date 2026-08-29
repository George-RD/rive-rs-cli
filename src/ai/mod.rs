pub mod authoring;
pub mod config;
pub mod error;
pub mod eval;
pub mod openai;
pub mod provider;
pub mod repair;
pub mod templates;
pub use authoring::{
    AuthoringRepairAttempt, AuthoringRepairFailure, AuthoringRepairRequest, AuthoringRepairResult,
    GenerationTarget, authoring_repair_schema, format_authoring_repair_summary,
    repair_authoring_spec,
};
pub use config::AiConfig;
pub use error::AiError;
pub use eval::{run_eval_suite, run_eval_suite_configured};
pub use provider::create_provider;
pub use repair::{RepairAttempt, RepairEngine, format_repair_summary, remediation_hints};
