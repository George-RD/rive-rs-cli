use crate::ai::authoring::{AuthoringRepairRequest, GenerationTarget};
use crate::ai::config::ProviderKind;
use crate::ai::templates;
use crate::ai::{AiConfig, AiError};

pub trait AiProvider {
    fn generate(
        &self,
        input: &str,
        config: &AiConfig,
        target: GenerationTarget,
    ) -> Result<serde_json::Value, AiError>;

    fn repair_authoring(
        &self,
        _request: &AuthoringRepairRequest,
        _config: &AiConfig,
    ) -> Result<serde_json::Value, AiError> {
        Err(AiError::ProviderNotConfigured(
            "this provider does not support incremental AuthoringSpec repair".to_string(),
        ))
    }
}

pub fn create_provider(
    config: &AiConfig,
    is_template: bool,
) -> Result<Box<dyn AiProvider>, AiError> {
    if is_template {
        return Ok(Box::new(TemplateProvider));
    }
    match config.provider {
        ProviderKind::Template => Err(AiError::ProviderNotConfigured(
            "no API key set; use --template for built-in templates, or set OPENAI_API_KEY for prompt mode".to_string(),
        )),
        ProviderKind::OpenAi => {
            let api_key = match config.api_key.clone() {
                Some(key) => key,
                None => return Err(AiError::ApiKeyMissing("OPENAI_API_KEY".to_string())),
            };
            Ok(Box::new(crate::ai::openai::OpenAiProvider::new(
                api_key,
                config.base_url.clone(),
            )))
        }
    }
}

struct TemplateProvider;

impl AiProvider for TemplateProvider {
    fn generate(
        &self,
        input: &str,
        _config: &AiConfig,
        target: GenerationTarget,
    ) -> Result<serde_json::Value, AiError> {
        if target != GenerationTarget::Scene {
            return Err(AiError::ProviderNotConfigured(
                "built-in templates are SceneSpec escape-hatch inputs; use the scene target"
                    .to_string(),
            ));
        }
        templates::get_template(input)
    }
}
