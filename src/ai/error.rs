#[derive(Debug)]
pub enum AiError {
    ProviderNotConfigured(String),
    ApiKeyMissing(String),
    RequestFailed(String),
    InvalidResponse(String),
    SchemaValidation(String),
    UnknownTemplate(String),
}

impl std::fmt::Display for AiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiError::ProviderNotConfigured(msg) => write!(f, "provider not configured: {}", msg),
            AiError::ApiKeyMissing(msg) => {
                write!(f, "API key missing: set {} environment variable", msg)
            }
            AiError::RequestFailed(msg) => write!(f, "API request failed: {}", msg),
            AiError::InvalidResponse(msg) => write!(f, "invalid API response: {}", msg),
            AiError::SchemaValidation(msg) => write!(f, "schema validation failed:\n{}", msg),
            AiError::UnknownTemplate(msg) => {
                let available = crate::ai::templates::list_templates().join(", ");
                write!(f, "unknown template '{}'; available: {}", msg, available)
            }
        }
    }
}

impl std::error::Error for AiError {}
