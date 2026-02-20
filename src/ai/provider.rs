use crate::ai::config::ProviderKind;
use crate::ai::templates;
use crate::ai::{AiConfig, AiError};

pub trait AiProvider {
    fn generate(&self, input: &str, config: &AiConfig) -> Result<serde_json::Value, AiError>;
}

pub fn create_provider(
    config: &AiConfig,
    is_template: bool,
) -> Result<Box<dyn AiProvider>, AiError> {
    if is_template {
        return Ok(Box::new(TemplateProvider));
    }
    match config.provider {
        ProviderKind::Template => Err(AiError::ApiKeyMissing("OPENAI_API_KEY".to_string())),
        ProviderKind::OpenAi => {
            if config.api_key.is_none() {
                return Err(AiError::ApiKeyMissing("OPENAI_API_KEY".to_string()));
            }
            Err(AiError::ProviderNotConfigured(
                "OpenAI provider not yet implemented; use --template for now, or set OPENAI_API_KEY when available"
                    .to_string(),
            ))
        }
    }
}

struct TemplateProvider;

impl AiProvider for TemplateProvider {
    fn generate(&self, input: &str, _config: &AiConfig) -> Result<serde_json::Value, AiError> {
        templates::get_template(input)
    }
}
