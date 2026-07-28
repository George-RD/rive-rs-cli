use std::collections::HashSet;
use std::path::{Component, Path};

use crate::ai::config::ProviderKind;
use crate::ai::templates;
use crate::ai::AiConfig;

use super::model::{EvalBaseline, EvalGates, EvalSuite, InputKind};
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
    if suite
        .gates
        .max_average_retries
        .is_some_and(|value| !value.is_finite() || value < 0.0)
    {
        return Err("max_average_retries must be a finite non-negative number".to_string());
    }

    let supported_traits = SUPPORTED_TRAITS.iter().copied().collect::<HashSet<_>>();
    let templates = templates::list_templates().iter().copied().collect::<HashSet<_>>();
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
        if case
            .image_path
            .as_deref()
            .is_some_and(|path| Path::new(path).is_absolute() || has_parent_component(path))
        {
            return Err(format!("case '{}' image_path must stay within the repository", case.id));
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
            return Err(format!("baseline hash for case '{}' must be 64 hex characters", case_id));
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
        return Err(
            "prompt cases require --provider openai and an OPENAI_API_KEY".to_string(),
        );
    }
    Ok(config)
}

pub fn evaluate_gates(
    gates: &EvalGates,
    validity_rate: f64,
    trait_adherence_rate: f64,
    pipeline_reproducibility_rate: f64,
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
