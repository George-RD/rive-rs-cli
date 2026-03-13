use crate::ai::repair::RepairAttempt;

#[derive(Debug, thiserror::Error)]
pub enum AiError {
    #[error("provider not configured: {0}")]
    ProviderNotConfigured(String),
    #[error("API key missing: set {0} environment variable")]
    ApiKeyMissing(String),
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("invalid API response: {0}")]
    InvalidResponse(String),
    #[allow(dead_code)] // reserved for future schema validation errors
    #[error("schema validation failed:\n{0}")]
    SchemaValidation(String),
    #[error("unknown template '{name}'; available: {available}")]
    UnknownTemplate { name: String, available: String },
    #[error("repair failed after {n} attempt(s): {final_error}", n = attempts.len())]
    RepairFailed {
        attempts: Vec<RepairAttempt>,
        final_error: String,
    },
}
