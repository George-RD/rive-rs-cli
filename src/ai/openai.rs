use crate::ai::authoring::{AuthoringRepairRequest, GenerationTarget, authoring_repair_schema};
use crate::ai::provider::AiProvider;
use crate::ai::{AiConfig, AiError};

pub struct OpenAiProvider {
    api_key: String,
    base_url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthoringTask {
    Static,
    Animated,
    Interactive,
}

impl OpenAiProvider {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self { api_key, base_url }
    }

    fn build_scene_system_prompt(&self) -> String {
        let schema = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/ai/scene-prompt-schema.json"
        ));
        let example = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/shapes.json"
        ));
        format!(
            "You are a Rive animation generator. You produce SceneSpec JSON that the rive-cli tool converts to .riv binary files. SceneSpec is the explicit expert escape hatch, so author runtime-facing structure only when this target is requested.\n\n\
             ## Schema (the object types below are the complete set you may emit)\n\n{}\n\n\
             ## Example\n\n{}\n\n\
             ## Rules\n\
             - Output ONLY valid JSON matching the schema above.\n\
             - Do NOT invent object types; use only the types defined in the schema above.\n\
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

    fn build_authoring_system_prompt(&self, input: &str) -> String {
        let task = authoring_task(input);
        let schema = crate::authoring::authoring_schema();
        let schema_json =
            serde_json::to_string_pretty(&schema).unwrap_or_else(|_| "{}".to_string());
        let schema_slice = authoring_schema_slice(&schema, task);
        let schema_slice_json =
            serde_json::to_string_pretty(&schema_slice).unwrap_or_else(|_| "{}".to_string());
        let example = authoring_example(task);
        let source_map = crate::authoring::lower_authoring_json(example)
            .ok()
            .and_then(|lowered| serde_json::to_string_pretty(&lowered.source_map).ok())
            .unwrap_or_else(|| "{}".to_string());
        format!(
            "You are a Rive animation author. Produce high-level AuthoringSpec JSON for rive-cli. AuthoringSpec expresses visual, motion, and behavior intent through stable authored IDs; rive-cli deterministically lowers it to SceneSpec and .riv.\n\n\
             ## Task-specific AuthoringSpec schema focus\n\n{}\n\n\
             This focus lists the top-level AuthoringSpec sections relevant to this task. Follow referenced definitions from the complete current schema below.\n\n\
             ## Complete current AuthoringSpec schema\n\n{}\n\n\
             ## Relevant checked-in showcase\n\n{}\n\n\
             ## Relevant authored-to-runtime source-map example\n\n{}\n\n\
             ## Rules\n\
             - Output ONLY one valid AuthoringSpec JSON object matching the current schema.\n\
             - authoring_format_version must be 0.\n\
             - Use stable, descriptive authored IDs and reference those IDs from constraints, poses, tracks, bindings, states, and transitions.\n\
             - Express intent through typed AuthoringSpec concepts. Do NOT author runtime indices, SceneSpec containment bookkeeping, generated runtime names, or state-array indices.\n\
             - Prefer reusable parameters/components, named poses/tracks/easings, and named behavior over repeated low-level objects.\n\
             - Use raw SceneSpec escapes only when the requested Rive concept is not represented by the typed schema: visual raw_scene_object nodes, motion.raw_animations, or behavior.raw_state_machines. Never use a raw escape merely for convenience.\n\
             - Do NOT invent fields or claim unsupported behavior.\n\
             - Do NOT include explanation, markdown, or text outside the JSON object.",
            schema_slice_json, schema_json, example, source_map
        )
    }

    fn build_authoring_repair_system_prompt(&self) -> String {
        let schema = serde_json::to_string_pretty(&authoring_repair_schema())
            .unwrap_or_else(|_| "{}".to_string());
        format!(
            "You repair a Rive AuthoringSpec by returning exactly one incremental stable-ID operation. The operation is applied atomically by rive-cli and the entire document is validated after it. Never regenerate or return the whole AuthoringSpec.\n\n\
             ## Repair operation schema\n\n{}\n\n\
             ## Rules\n\
             - Output ONLY one JSON object matching the repair schema.\n\
             - Choose the smallest authored concept that resolves the supplied diagnostic.\n\
             - Address concepts only by stable authored ID; never use runtime indices, SceneSpec paths, generated runtime names, or array positions as edit identity.\n\
             - Preserve unrelated authored content.\n\
             - Prefer typed AuthoringSpec entities. Use raw escapes only for concepts outside the typed schema.\n\
             - The provided source-map entries, when present, are evidence for mapping runtime symptoms back to authored concepts; they are context, not edit targets.\n\
             - Do NOT include explanation, markdown, or text outside the JSON object.",
            schema
        )
    }

    fn request_json(
        &self,
        system_prompt: String,
        user_prompt: String,
        config: &AiConfig,
    ) -> Result<serde_json::Value, AiError> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = serde_json::json!({
            "model": config.model,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_prompt}
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

impl AiProvider for OpenAiProvider {
    fn generate(
        &self,
        input: &str,
        config: &AiConfig,
        target: GenerationTarget,
    ) -> Result<serde_json::Value, AiError> {
        let system_prompt = match target {
            GenerationTarget::Authoring => self.build_authoring_system_prompt(input),
            GenerationTarget::Scene => self.build_scene_system_prompt(),
        };
        self.request_json(system_prompt, input.to_string(), config)
    }

    fn repair_authoring(
        &self,
        request: &AuthoringRepairRequest,
        config: &AiConfig,
    ) -> Result<serde_json::Value, AiError> {
        let user_prompt = serde_json::to_string_pretty(request).map_err(|error| {
            AiError::InvalidResponse(format!("failed to serialize repair context: {error}"))
        })?;
        self.request_json(
            self.build_authoring_repair_system_prompt(),
            user_prompt,
            config,
        )
    }
}

fn authoring_task(input: &str) -> AuthoringTask {
    let normalized = input.to_ascii_lowercase();
    if [
        "interactive",
        "click",
        "tap",
        "hover",
        "toggle",
        "state",
        "input",
        "button",
    ]
    .iter()
    .any(|needle| normalized.contains(needle))
    {
        AuthoringTask::Interactive
    } else if [
        "animate",
        "animation",
        "motion",
        "move",
        "spin",
        "pulse",
        "bounce",
        "transition",
    ]
    .iter()
    .any(|needle| normalized.contains(needle))
    {
        AuthoringTask::Animated
    } else {
        AuthoringTask::Static
    }
}

fn authoring_example(task: AuthoringTask) -> &'static str {
    match task {
        AuthoringTask::Interactive => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/authoring/complex-interactive-showcase.v0.json"
        )),
        AuthoringTask::Animated => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/authoring/complex-animated-showcase.v0.json"
        )),
        AuthoringTask::Static => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/authoring/complex-static-showcase.v0.json"
        )),
    }
}

fn authoring_schema_slice(
    schema: &serde_json::Value,
    task: AuthoringTask,
) -> serde_json::Value {
    let mut names = vec![
        "authoring_format_version",
        "artboard",
        "parameters",
        "components",
        "visual",
    ];
    if matches!(task, AuthoringTask::Animated | AuthoringTask::Interactive) {
        names.push("motion");
    }
    if task == AuthoringTask::Interactive {
        names.push("behavior");
    }

    let properties = schema
        .get("properties")
        .and_then(serde_json::Value::as_object);
    let focused = names
        .into_iter()
        .filter_map(|name| {
            properties
                .and_then(|properties| properties.get(name))
                .map(|value| (name.to_string(), value.clone()))
        })
        .collect::<serde_json::Map<_, _>>();
    serde_json::json!({ "properties": focused })
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

    fn provider() -> OpenAiProvider {
        OpenAiProvider::new(
            "test-key".to_string(),
            "https://api.openai.com/v1".to_string(),
        )
    }

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
    fn scene_prompt_keeps_scene_spec_escape_hatch() {
        let prompt = provider().build_scene_system_prompt();
        assert!(prompt.contains("scene_format_version"));
        assert!(prompt.contains("expert escape hatch"));
        assert!(prompt.contains("#RRGGBB"));
    }

    #[test]
    fn authoring_prompt_teaches_stable_intent_not_runtime_indices() {
        let prompt = provider().build_authoring_system_prompt("interactive toggle button");
        assert!(prompt.contains("authoring_format_version"));
        assert!(prompt.contains("stable authored IDs"));
        assert!(prompt.contains("Do NOT author runtime indices"));
        assert!(prompt.contains("raw_state_machines"));
        assert!(prompt.contains("Relevant authored-to-runtime source-map example"));
    }

    #[test]
    fn authoring_repair_prompt_requires_incremental_stable_id_edit() {
        let prompt = provider().build_authoring_repair_system_prompt();
        assert!(prompt.contains("exactly one incremental stable-ID operation"));
        assert!(prompt.contains("replace_visual_node"));
        assert!(prompt.contains("Never regenerate"));
    }

    #[test]
    fn task_selection_tracks_requested_semantic_dimension() {
        assert_eq!(authoring_task("static dashboard"), AuthoringTask::Static);
        assert_eq!(
            authoring_task("animate the signal"),
            AuthoringTask::Animated
        );
        assert_eq!(
            authoring_task("interactive toggle"),
            AuthoringTask::Interactive
        );
    }

    #[test]
    fn schema_focus_only_adds_motion_and_behavior_when_relevant() {
        let schema = crate::authoring::authoring_schema();
        let static_slice = authoring_schema_slice(&schema, AuthoringTask::Static);
        let animated_slice = authoring_schema_slice(&schema, AuthoringTask::Animated);
        let interactive_slice = authoring_schema_slice(&schema, AuthoringTask::Interactive);

        assert!(static_slice["properties"].get("visual").is_some());
        assert!(static_slice["properties"].get("motion").is_none());
        assert!(static_slice["properties"].get("behavior").is_none());
        assert!(animated_slice["properties"].get("motion").is_some());
        assert!(animated_slice["properties"].get("behavior").is_none());
        assert!(interactive_slice["properties"].get("motion").is_some());
        assert!(interactive_slice["properties"].get("behavior").is_some());
    }

    #[test]
    fn representative_showcases_provide_source_map_context() {
        for task in [
            AuthoringTask::Static,
            AuthoringTask::Animated,
            AuthoringTask::Interactive,
        ] {
            let lowered = crate::authoring::lower_authoring_json(authoring_example(task))
                .expect("checked-in AuthoringSpec showcase should lower");
            assert!(!lowered.source_map.entries.is_empty());
        }
    }
}
