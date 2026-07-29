use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::ai::config::ProviderKind;
use crate::ai::{AiConfig, AiError, RepairEngine, create_provider};
use crate::validator::{InspectFilter, ValidationReport, parse_riv, validate_riv};

use super::model::{EvalBaseline, EvalCase, EvalCaseReport, EvalReport, EvalSuite, InputKind};
use super::traits::trait_score;
use super::validation::{evaluate_gates, resolve_eval_config, validate_baseline, validate_suite};

fn run_id() -> Result<String, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("system time error: {}", error))?;
    let suffix: u64 = rand::thread_rng().r#gen();
    Ok(format!("{}_{:016x}", now.as_millis(), suffix))
}

fn hash_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{:064x}", digest)
}

fn write_json<P: AsRef<Path>, T: Serialize>(path: P, value: &T) -> Result<(), String> {
    let pretty = serde_json::to_string_pretty(value).map_err(|error| {
        format!(
            "failed to serialize JSON for {}: {}",
            path.as_ref().display(),
            error
        )
    })?;
    fs::write(path.as_ref(), pretty)
        .map_err(|error| format!("failed to write {}: {}", path.as_ref().display(), error))
}

fn provider_name(config: &AiConfig) -> &'static str {
    match &config.provider {
        ProviderKind::Template => "template",
        ProviderKind::OpenAi => "openai",
    }
}

fn report_model(config: &AiConfig) -> String {
    match &config.provider {
        ProviderKind::Template => "built-in".to_string(),
        ProviderKind::OpenAi => config.model.clone(),
    }
}

fn failed_case(case: &EvalCase, case_dir: &Path, error: String) -> EvalCaseReport {
    EvalCaseReport {
        id: case.id.clone(),
        input_kind: case.input_kind.as_str().to_string(),
        input: case.input.clone(),
        expected_traits: case.expected_traits.clone(),
        style_matched_traits: Vec::new(),
        style_score: 0.0,
        valid: false,
        retries: 0,
        reproducible: false,
        output_hash: None,
        drifted: false,
        failure_reason: Some(error),
        artifact_dir: case_dir.display().to_string(),
        text_hint: case.text_hint.clone(),
        image_path: case.image_path.clone(),
    }
}

fn run_case(
    case: &EvalCase,
    case_dir: &Path,
    file_id: u64,
    max_retries: u8,
    baseline_hash: Option<&String>,
    config: &AiConfig,
) -> Result<EvalCaseReport, String> {
    fs::create_dir_all(case_dir)
        .map_err(|error| format!("failed to create {}: {}", case_dir.display(), error))?;
    fs::write(case_dir.join("input.txt"), &case.input)
        .map_err(|error| format!("failed to write input.txt: {}", error))?;

    let provider = create_provider(config, case.input_kind == InputKind::Template)
        .map_err(|error| format!("AI provider error: {}", error))?;
    let generated_scene = provider
        .generate(&case.input, config)
        .map_err(|error| format!("generation failed: {}", error))?;
    write_json(case_dir.join("generated-scene.json"), &generated_scene)?;

    let engine = RepairEngine::new(max_retries);
    let repaired =
        engine
            .repair(generated_scene.clone(), file_id)
            .map_err(|error| match error {
                AiError::RepairFailed {
                    attempts,
                    final_error,
                } => format!(
                    "repair failed after {} attempts: {}",
                    attempts.len(),
                    final_error
                ),
                other => format!("repair failed: {}", other),
            })?;
    write_json(case_dir.join("scene.json"), &repaired.scene_json)?;
    fs::write(case_dir.join("output.riv"), &repaired.riv_bytes)
        .map_err(|error| format!("failed to write output.riv: {}", error))?;

    let validation: ValidationReport =
        validate_riv(&repaired.riv_bytes).map_err(|error| format!("validate failed: {}", error))?;
    write_json(case_dir.join("validate.json"), &validation)?;
    let parsed = parse_riv(&repaired.riv_bytes, &InspectFilter::default())
        .map_err(|error| format!("inspect parse failed: {}", error))?;
    write_json(case_dir.join("inspect.json"), &parsed)?;

    let repeat = engine
        .repair(generated_scene, file_id)
        .map_err(|error| format!("pipeline reproducibility check failed: {}", error))?;
    let reproducible = repaired.riv_bytes == repeat.riv_bytes;
    let output_hash = hash_bytes(&repaired.riv_bytes);
    let (style_score, matched_traits) = trait_score(&repaired.scene_json, &case.expected_traits);
    let drifted = baseline_hash.is_some_and(|hash| hash != &output_hash);

    Ok(EvalCaseReport {
        id: case.id.clone(),
        input_kind: case.input_kind.as_str().to_string(),
        input: case.input.clone(),
        expected_traits: case.expected_traits.clone(),
        style_matched_traits: matched_traits,
        style_score,
        valid: validation.valid,
        retries: repaired.total_retries,
        reproducible,
        output_hash: Some(output_hash),
        drifted,
        failure_reason: (!validation.valid).then(|| validation.errors.join("; ")),
        artifact_dir: case_dir.display().to_string(),
        text_hint: case.text_hint.clone(),
        image_path: case.image_path.clone(),
    })
}

fn read_suite(path: &Path) -> Result<EvalSuite, String> {
    let contents = fs::read_to_string(path)
        .map_err(|error| format!("failed to read suite {}: {}", path.display(), error))?;
    serde_json::from_str(&contents)
        .map_err(|error| format!("failed to parse suite {}: {}", path.display(), error))
}

fn read_baseline(path: &Path) -> Result<EvalBaseline, String> {
    let contents = fs::read_to_string(path)
        .map_err(|error| format!("failed to read baseline {}: {}", path.display(), error))?;
    serde_json::from_str(&contents)
        .map_err(|error| format!("failed to parse baseline {}: {}", path.display(), error))
}

pub fn run_eval_suite(
    suite_path: &Path,
    output_root: &Path,
    file_id: u64,
    max_retries: u8,
    baseline_path: Option<&Path>,
    write_baseline_path: Option<&Path>,
) -> Result<EvalReport, String> {
    run_eval_suite_configured(
        suite_path,
        output_root,
        file_id,
        max_retries,
        baseline_path,
        write_baseline_path,
        None,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn run_eval_suite_configured(
    suite_path: &Path,
    output_root: &Path,
    file_id: u64,
    max_retries: u8,
    baseline_path: Option<&Path>,
    write_baseline_path: Option<&Path>,
    model_override: Option<String>,
    provider_override: Option<String>,
) -> Result<EvalReport, String> {
    if baseline_path.is_some() && write_baseline_path.is_some() {
        return Err("--baseline and --write-baseline cannot be used together".to_string());
    }

    let suite = read_suite(suite_path)?;
    validate_suite(&suite)?;
    let baseline = baseline_path.map(read_baseline).transpose()?;
    if let Some(baseline) = &baseline {
        validate_baseline(&suite, baseline)?;
    }
    let config = resolve_eval_config(&suite, model_override, provider_override)?;

    let run_id = run_id()?;
    let run_dir = output_root.join(&run_id);
    let samples_dir = run_dir.join("samples");
    fs::create_dir_all(&samples_dir)
        .map_err(|error| format!("failed to create {}: {}", samples_dir.display(), error))?;
    write_json(run_dir.join("suite.json"), &suite)?;

    let cases = suite
        .cases
        .iter()
        .map(|case| {
            let case_dir = samples_dir.join(&case.id);
            let baseline_hash = baseline
                .as_ref()
                .and_then(|value| value.case_hashes.get(&case.id));
            run_case(
                case,
                &case_dir,
                file_id,
                max_retries,
                baseline_hash,
                &config,
            )
            .unwrap_or_else(|error| failed_case(case, &case_dir, error))
        })
        .collect::<Vec<_>>();

    let case_count = cases.len();
    let valid_count = cases.iter().filter(|case| case.valid).count();
    let validity_rate = valid_count as f64 / case_count as f64;
    let average_retries = cases
        .iter()
        .map(|case| f64::from(case.retries))
        .sum::<f64>()
        / case_count as f64;
    let trait_adherence_rate =
        cases.iter().map(|case| case.style_score).sum::<f64>() / case_count as f64;
    let pipeline_reproducibility_rate =
        cases.iter().filter(|case| case.reproducible).count() as f64 / case_count as f64;
    let drift_count = cases.iter().filter(|case| case.drifted).count();
    let gate_failures = evaluate_gates(
        &suite.gates,
        validity_rate,
        trait_adherence_rate,
        pipeline_reproducibility_rate,
        average_retries,
        drift_count,
    );

    let report = EvalReport {
        run_id: run_id.clone(),
        suite_name: suite.suite_name.clone(),
        suite_version: suite.suite_version,
        output_dir: run_dir.display().to_string(),
        provider: provider_name(&config).to_string(),
        model: report_model(&config),
        baseline_used: baseline.is_some(),
        passed: gate_failures.is_empty(),
        gate_failures,
        case_count,
        valid_count,
        validity_rate,
        average_retries,
        trait_adherence_rate,
        style_adherence_rate: trait_adherence_rate,
        pipeline_reproducibility_rate,
        reproducibility_rate: pipeline_reproducibility_rate,
        drift_count,
        cases,
    };
    write_json(run_dir.join("report.json"), &report)?;

    if let Some(path) = write_baseline_path {
        if !report.passed {
            return Err(format!(
                "refusing to write a baseline from a failing run; see {}",
                run_dir.join("report.json").display()
            ));
        }
        let case_hashes = report
            .cases
            .iter()
            .map(|case| {
                case.output_hash
                    .as_ref()
                    .map(|hash| (case.id.clone(), hash.clone()))
                    .ok_or_else(|| format!("case '{}' has no output hash", case.id))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        write_json(
            path,
            &EvalBaseline {
                suite_name: suite.suite_name,
                suite_version: suite.suite_version,
                case_hashes,
            },
        )?;
    }

    Ok(report)
}

#[cfg(test)]
pub(super) fn test_hash_bytes(bytes: &[u8]) -> String {
    hash_bytes(bytes)
}

#[cfg(test)]
pub(super) fn test_run_id() -> Result<String, String> {
    run_id()
}
