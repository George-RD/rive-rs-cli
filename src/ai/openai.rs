use crate::ai::provider::AiProvider;
use crate::ai::{AiConfig, AiError};

pub struct OpenAiProvider {
    api_key: String,
    base_url: String,
}

impl OpenAiProvider {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self { api_key, base_url }
    }

    fn build_system_prompt(&self) -> String {
        let schema = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/scene.schema.v1.json"
        ));
        let example = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/shapes.json"
        ));
        format!(
            "You are a Rive animation generator. You produce SceneSpec JSON that the rive-cli tool converts to .riv binary files.\n\n\
             ## Schema\n\n{}\n\n\
             ## Example\n\n{}\n\n\
             ## Rules\n\
             - Output ONLY valid JSON matching the schema above.\n\
             - scene_format_version must be 1.\n\
             - Every object must have a unique name within its artboard.\n\
             - Colors use #RRGGBB format (e.g. \"#FF0000\" for red).\n\
             - Enum fields use string names: fill_rule (nonzero/evenodd), cap (butt/round/square), join (miter/round/bevel), loop_type (oneshot/loop/pingpong), mode (sequential/synchronized).\n\
             - Do NOT include any explanation, markdown, or text outside the JSON object.\n\
             - Artboard dimensions should be reasonable (100-2000 pixels).\n\
             - Use origin_x: 0.5, origin_y: 0.5 to center shapes at their position.",
            schema, example
        )
    }
}

impl AiProvider for OpenAiProvider {
    fn generate(&self, input: &str, config: &AiConfig) -> Result<serde_json::Value, AiError> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = serde_json::json!({
            "model": config.model,
            "messages": [
                {"role": "system", "content": self.build_system_prompt()},
                {"role": "user", "content": input}
            ],
            "temperature": 0.7,
            "response_format": {"type": "json_object"}
        });

        let response = ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_json(&body)
            .map_err(|e| AiError::RequestFailed(e.to_string()))?;

        let response_json: serde_json::Value = response.into_json().map_err(|e| {
            AiError::InvalidResponse(format!("failed to parse response JSON: {}", e))
        })?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| {
                AiError::InvalidResponse("missing choices[0].message.content".to_string())
            })?;

        extract_json(content)
    }
}

fn extract_json(content: &str) -> Result<serde_json::Value, AiError> {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(content)
        && v.is_object()
    {
        return Ok(v);
    }

    if let Some(start) = content.find("```json") {
        let json_start = start + 7;
        if let Some(end) = content[json_start..].find("```") {
            let json_str = content[json_start..json_start + end].trim();
            return serde_json::from_str(json_str).map_err(|e| {
                AiError::InvalidResponse(format!("invalid JSON in code fence: {}", e))
            });
        }
    }

    if let Some(start) = content.find("```") {
        let json_start = start + 3;
        let json_start = content[json_start..]
            .find('\n')
            .map(|n| json_start + n + 1)
            .unwrap_or(json_start);
        if let Some(end) = content[json_start..].find("```") {
            let json_str = content[json_start..json_start + end].trim();
            return serde_json::from_str(json_str).map_err(|e| {
                AiError::InvalidResponse(format!("invalid JSON in code fence: {}", e))
            });
        }
    }

    if let (Some(start), Some(end)) = (content.find('{'), content.rfind('}'))
        && start < end
    {
        let json_str = &content[start..=end];
        return serde_json::from_str(json_str)
            .map_err(|e| AiError::InvalidResponse(format!("extracted JSON invalid: {}", e)));
    }

    Err(AiError::InvalidResponse(
        "response contains no valid JSON object".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_direct() {
        let json = r#"{"scene_format_version": 1, "artboard": {"name": "Test", "width": 100, "height": 100, "children": []}}"#;
        let result = extract_json(json).unwrap();
        assert_eq!(result["scene_format_version"], 1);
    }

    #[test]
    fn test_extract_json_from_code_fence() {
        let content = "Here is the JSON:\n```json\n{\"scene_format_version\": 1}\n```\nDone.";
        let result = extract_json(content).unwrap();
        assert_eq!(result["scene_format_version"], 1);
    }

    #[test]
    fn test_extract_json_from_plain_fence() {
        let content = "```\n{\"scene_format_version\": 1}\n```";
        let result = extract_json(content).unwrap();
        assert_eq!(result["scene_format_version"], 1);
    }

    #[test]
    fn test_extract_json_with_surrounding_text() {
        let content =
            "Sure! Here's the animation:\n{\"scene_format_version\": 1}\nHope that helps!";
        let result = extract_json(content).unwrap();
        assert_eq!(result["scene_format_version"], 1);
    }

    #[test]
    fn test_extract_json_no_json() {
        let result = extract_json("no json here");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_json_array_rejected() {
        let result = extract_json("[1, 2, 3]");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_system_prompt_contains_schema() {
        let provider = OpenAiProvider::new(
            "test-key".to_string(),
            "https://api.openai.com/v1".to_string(),
        );
        let prompt = provider.build_system_prompt();
        assert!(prompt.contains("scene_format_version"));
        assert!(prompt.contains("#RRGGBB"));
    }
}
