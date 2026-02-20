use crate::ai::AiError;

pub enum ProviderKind {
    Template,
    OpenAi,
}

pub struct AiConfig {
    pub provider: ProviderKind,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: String,
}

impl AiConfig {
    pub fn resolve(
        model_override: Option<String>,
        provider_override: Option<String>,
    ) -> Result<Self, AiError> {
        let api_key = std::env::var("OPENAI_API_KEY").ok();
        let base_url = std::env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let model = model_override
            .or_else(|| std::env::var("OPENAI_MODEL").ok())
            .unwrap_or_else(|| "gpt-4o".to_string());

        let provider = if let Some(ref p) = provider_override {
            match p.as_str() {
                "template" => ProviderKind::Template,
                "openai" => ProviderKind::OpenAi,
                other => {
                    return Err(AiError::ProviderNotConfigured(format!(
                        "unknown provider '{}'; available: template, openai",
                        other
                    )));
                }
            }
        } else if api_key.is_some() {
            ProviderKind::OpenAi
        } else {
            ProviderKind::Template
        };

        Ok(AiConfig {
            provider,
            model,
            api_key,
            base_url,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_default_config() {
        unsafe {
            std::env::remove_var("OPENAI_API_KEY");
            std::env::remove_var("OPENAI_MODEL");
            std::env::remove_var("OPENAI_BASE_URL");
        }
        let config = AiConfig::resolve(None, None).unwrap();
        assert!(matches!(config.provider, ProviderKind::Template));
    }

    #[test]
    fn test_resolve_unknown_provider_error() {
        let result = AiConfig::resolve(None, Some("unknown".to_string()));
        assert!(result.is_err());
    }
}
