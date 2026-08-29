use std::collections::HashSet;
use std::path::{Component, Path};

use crate::ai::AiConfig;
use crate::ai::config::ProviderKind;
use crate::ai::templates;

use super::model::{
    AnimatedSemanticCheck, EvalBaseline, EvalGates, EvalSuite, InputKind, StaticSemanticCheck,
};
use super::traits::SUPPORTED_TRAITS;

fn validate_rate(name: &str, value: f64) -> Result<(), String> {
    if value.is_finite() && (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(format!("{} must be between 0 and 1", name))
    }
}

fn is_safe_case_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
}

fn has_parent_component(path: &str) -> bool {
    Path::new(path)
        .components()
        .any(|component| matches!(component, Component::ParentDir))
}

fn validate_repo_relative_path(case_id: &str, label: &str, path: &str) -> Result<(), String> {
    if Path::new(path).is_absolute() || has_parent_component(path) {
        Err(format!(
            "case '{}' {} must stay within the repository",
            case_id, label
        ))
    } else {
        Ok(())
    }
}

pub fn validate_suite(suite: &EvalSuite) -> Result<(), String> {
    if suite.suite_name.trim().is_empty() {
        return Err("suite name must not be empty".to_string());
    }
    if suite.suite_version == 0 {
        return Err("suite version must be greater than zero".to_string());
    }
    if suite.cases.is_empty() {
        return Err("suite must contain at least one case".to_string());
    }

    validate_rate("min_validity_rate", suite.gates.min_validity_rate)?;
    validate_rate(
        "min_trait_adherence_rate",
        suite.gates.min_trait_adherence_rate,
    )?;
    validate_rate(
        "min_pipeline_reproducibility_rate",
        suite.gates.min_pipeline_reproducibility_rate,
    )?;
    validate_rate("min_runtime_pass_rate", suite.gates.min_runtime_pass_rate)?;
    validate_rate(
        "min_semantic_static_pass_rate",
        suite.gates.min_semantic_static_pass_rate,
    )?;
    validate_rate(
        "min_semantic_animated_pass_rate",
        suite.gates.min_semantic_animated_pass_rate,
    )?;
    if suite
        .gates
        .max_average_retries
        .is_some_and(|value| !value.is_finite() || value < 0.0)
    {
        return Err("max_average_retries must be a finite non-negative number".to_string());
    }

    let supported_traits = SUPPORTED_TRAITS.iter().copied().collect::<HashSet<_>>();
    let templates = templates::list_templates()
        .iter()
        .copied()
        .collect::<HashSet<_>>();
    let mut case_ids = HashSet::new();

    for case in &suite.cases {
        if !is_safe_case_id(&case.id) {
            return Err(format!(
                "case id '{}' must contain only ASCII letters, digits, '-' or '_' and be at most 64 characters",
                case.id
            ));
        }
        if !case_ids.insert(case.id.as_str()) {
            return Err(format!("duplicate case id '{}'", case.id));
        }
        if case.input.trim().is_empty() {
            return Err(format!("case '{}' input must not be empty", case.id));
        }
        if case.input_kind == InputKind::Template && !templates.contains(case.input.as_str()) {
            return Err(format!(
                "case '{}' references unknown template '{}'",
                case.id, case.input
            ));
        }
        if case.input_kind == InputKind::AuthoringExample {
            validate_repo_relative_path(&case.id, "authoring input path", &case.input)?;
        }
        if let Some(image_path) = case.image_path.as_deref() {
            validate_repo_relative_path(&case.id, "image_path", image_path)?;
        }
        if let Some(runtime) = &case.runtime {
            if runtime.frames.is_empty() {
                return Err(format!(
                    "case '{}' runtime frames must not be empty",
                    case.id
                ));
            }
            if runtime.frames.iter().copied().collect::<HashSet<_>>().len() != runtime.frames.len()
            {
                return Err(format!("case '{}' runtime frames must be unique", case.id));
            }
            if !runtime.fps.is_finite() || runtime.fps <= 0.0 {
                return Err(format!(
                    "case '{}' runtime fps must be finite and greater than zero",
                    case.id
                ));
            }
            if runtime.width == 0 || runtime.height == 0 || runtime.scale == 0 {
                return Err(format!(
                    "case '{}' runtime width, height and scale must be greater than zero",
                    case.id
                ));
            }
            if runtime.min_non_blank_frames == 0
                || runtime.min_non_blank_frames > runtime.frames.len()
            {
                return Err(format!(
                    "case '{}' runtime min_non_blank_frames must be between 1 and the frame count",
                    case.id
                ));
            }
            if runtime.min_distinct_colors == 0 {
                return Err(format!(
                    "case '{}' runtime min_distinct_colors must be greater than zero",
                    case.id
                ));
            }
            if runtime.animation.is_some() && runtime.state_machine.is_some() {
                return Err(format!(
                    "case '{}' runtime cannot select both an animation and a state machine",
                    case.id
                ));
            }
        }

        if let Some(semantic) = &case.semantic {
            if case.input_kind != InputKind::AuthoringExample {
                return Err(format!(
                    "case '{}' semantic expectations require input_kind 'authoring_example'",
                    case.id
                ));
            }
            if semantic.static_checks.is_empty() && semantic.animated_checks.is_empty() {
                return Err(format!(
                    "case '{}' semantic expectations must contain at least one check",
                    case.id
                ));
            }
            for check in &semantic.static_checks {
                match check {
                    StaticSemanticCheck::AuthoredIdPresent { authored_id } => {
                        if authored_id.trim().is_empty() {
                            return Err(format!(
                                "case '{}' semantic authored_id must not be empty",
                                case.id
                            ));
                        }
                    }
                    StaticSemanticCheck::AuthoredIdHasRuntimeType {
                        authored_id,
                        object_type,
                    } => {
                        if authored_id.trim().is_empty() || object_type.trim().is_empty() {
                            return Err(format!(
                                "case '{}' semantic authored_id and object_type must not be empty",
                                case.id
                            ));
                        }
                    }
                }
            }
            if !semantic.animated_checks.is_empty() && case.runtime.is_none() {
                return Err(format!(
                    "case '{}' animated semantic checks require runtime expectations",
                    case.id
                ));
            }
            if let Some(runtime) = &case.runtime {
                for check in &semantic.animated_checks {
                    match check {
                        AnimatedSemanticCheck::FramesDiffer { from, to } => {
                            if from == to {
                                return Err(format!(
                                    "case '{}' animated semantic frame pair must use different frames",
                                    case.id
                                ));
                            }
                            if !runtime.frames.contains(from) || !runtime.frames.contains(to) {
                                return Err(format!(
                                    "case '{}' animated semantic frames {} and {} must both be rendered",
                                    case.id, from, to
                                ));
                            }
                        }
                    }
                }
            }
        }

        let mut expected = HashSet::new();
        for trait_name in &case.expected_traits {
            if !supported_traits.contains(trait_name.as_str()) {
                return Err(format!(
                    "case '{}' has unsupported expected trait '{}'",
                    case.id, trait_name
                ));
            }
            if !expected.insert(trait_name.as_str()) {
                return Err(format!(
                    "case '{}' repeats expected trait '{}'",
                    case.id, trait_name
                ));
            }
        }
    }
    Ok(())
}

pub fn validate_baseline(suite: &EvalSuite, baseline: &EvalBaseline) -> Result<(), String> {
    if baseline.suite_name != suite.suite_name {
        return Err(format!(
            "baseline suite name '{}' does not match '{}'",
            baseline.suite_name, suite.suite_name
        ));
    }
    if baseline.suite_version != suite.suite_version {
        return Err(format!(
            "baseline suite version {} does not match {}",
            baseline.suite_version, suite.suite_version
        ));
    }

    let case_ids = suite
        .cases
        .iter()
        .map(|case| case.id.as_str())
        .collect::<HashSet<_>>();
    for case_id in &case_ids {
        let Some(hash) = baseline.case_hashes.get(*case_id) else {
            return Err(format!("baseline is missing case '{}'", case_id));
        };
        if hash.len() != 64 || !hash.chars().all(|character| character.is_ascii_hexdigit()) {
            return Err(format!(
                "baseline hash for case '{}' must be 64 hex characters",
                case_id
            ));
        }
    }
    for case_id in baseline.case_hashes.keys() {
        if !case_ids.contains(case_id.as_str()) {
            return Err(format!("baseline contains unknown case '{}'", case_id));
        }
    }
    Ok(())
}

pub fn resolve_eval_config(
    suite: &EvalSuite,
    model_override: Option<String>,
    provider_override: Option<String>,
) -> Result<AiConfig, String> {
    let has_prompt_cases = suite
        .cases
        .iter()
        .any(|case| case.input_kind == InputKind::Prompt);
    let provider_override = if has_prompt_cases || provider_override.is_some() {
        provider_override
    } else {
        Some("template".to_string())
    };
    let config = AiConfig::resolve(model_override, provider_override)
        .map_err(|error| format!("AI config error: {}", error))?;
    if has_prompt_cases && matches!(config.provider, ProviderKind::Template) {
        return Err("prompt cases require --provider openai and an OPENAI_API_KEY".to_string());
    }
    Ok(config)
}

#[allow(clippy::too_many_arguments)]
pub fn evaluate_gates(
    gates: &EvalGates,
    validity_rate: f64,
    trait_adherence_rate: f64,
    pipeline_reproducibility_rate: f64,
    runtime_pass_rate: f64,
    semantic_static_pass_rate: f64,
    semantic_animated_pass_rate: f64,
    average_retries: f64,
    drift_count: usize,
) -> Vec<String> {
    let mut failures = Vec::new();
    if validity_rate < gates.min_validity_rate {
        failures.push(format!(
            "validity rate {:.3} is below {:.3}",
            validity_rate, gates.min_validity_rate
        ));
    }
    if trait_adherence_rate < gates.min_trait_adherence_rate {
        failures.push(format!(
            "trait adherence rate {:.3} is below {:.3}",
            trait_adherence_rate, gates.min_trait_adherence_rate
        ));
    }
    if pipeline_reproducibility_rate < gates.min_pipeline_reproducibility_rate {
        failures.push(format!(
            "pipeline reproducibility rate {:.3} is below {:.3}",
            pipeline_reproducibility_rate, gates.min_pipeline_reproducibility_rate
        ));
    }
    if runtime_pass_rate < gates.min_runtime_pass_rate {
        failures.push(format!(
            "runtime pass rate {:.3} is below {:.3}",
            runtime_pass_rate, gates.min_runtime_pass_rate
        ));
    }
    if semantic_static_pass_rate < gates.min_semantic_static_pass_rate {
        failures.push(format!(
            "static semantic pass rate {:.3} is below {:.3}",
            semantic_static_pass_rate, gates.min_semantic_static_pass_rate
        ));
    }
    if semantic_animated_pass_rate < gates.min_semantic_animated_pass_rate {
        failures.push(format!(
            "animated semantic pass rate {:.3} is below {:.3}",
            semantic_animated_pass_rate, gates.min_semantic_animated_pass_rate
        ));
    }
    if gates
        .max_average_retries
        .is_some_and(|maximum| average_retries > maximum)
    {
        failures.push(format!(
            "average retries {:.3} exceeds {:.3}",
            average_retries,
            gates.max_average_retries.unwrap_or_default()
        ));
    }
    if drift_count > gates.max_drift_count {
        failures.push(format!(
            "drift count {} exceeds {}",
            drift_count, gates.max_drift_count
        ));
    }
    failures
}
