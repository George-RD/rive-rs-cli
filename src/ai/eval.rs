use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ai::{AiConfig, AiError, RepairEngine, create_provider};
use crate::validator::{InspectFilter, ValidationReport, parse_riv, validate_riv};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalSuite {
    pub suite_name: String,
    pub suite_version: u32,
    pub cases: Vec<EvalCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalCase {
    pub id: String,
    pub input_kind: InputKind,
    pub input: String,
    #[serde(default)]
    pub expected_traits: Vec<String>,
    #[serde(default)]
    pub text_hint: Option<String>,
    #[serde(default)]
    pub image_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputKind {
    Template,
    Prompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalBaseline {
    pub suite_name: String,
    pub suite_version: u32,
    pub case_hashes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvalReport {
    pub run_id: String,
    pub suite_name: String,
    pub suite_version: u32,
    pub output_dir: String,
    pub case_count: usize,
    pub valid_count: usize,
    pub validity_rate: f64,
    pub average_retries: f64,
    pub style_adherence_rate: f64,
    pub reproducibility_rate: f64,
    pub drift_count: usize,
    pub cases: Vec<EvalCaseReport>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvalCaseReport {
    pub id: String,
    pub input_kind: String,
    pub input: String,
    pub expected_traits: Vec<String>,
    pub style_matched_traits: Vec<String>,
    pub style_score: f64,
    pub valid: bool,
    pub retries: u8,
    pub reproducible: bool,
    pub output_hash: Option<String>,
    pub drifted: bool,
    pub failure_reason: Option<String>,
    pub artifact_dir: String,
    pub text_hint: Option<String>,
    pub image_path: Option<String>,
}

fn run_id() -> Result<String, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("system time error: {}", e))?;
    Ok(format!("{}", now.as_secs()))
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn write_json<P: AsRef<Path>, T: Serialize>(path: P, value: &T) -> Result<(), String> {
    let pretty = serde_json::to_string_pretty(value).map_err(|e| {
        format!(
            "failed to serialize JSON for {}: {}",
            path.as_ref().display(),
            e
        )
    })?;
    fs::write(path.as_ref(), pretty)
        .map_err(|e| format!("failed to write {}: {}", path.as_ref().display(), e))
}

fn collect_object_types(value: &Value, out: &mut HashSet<String>) {
    if let Some(obj) = value.as_object() {
        if let Some(t) = obj.get("type").and_then(Value::as_str) {
            out.insert(t.to_string());
        }
        for child in obj.values() {
            collect_object_types(child, out);
        }
    }
    if let Some(arr) = value.as_array() {
        for child in arr {
            collect_object_types(child, out);
        }
    }
}

fn has_animations(scene: &Value) -> bool {
    if let Some(artboard) = scene.get("artboard")
        && artboard
            .get("animations")
            .and_then(Value::as_array)
            .is_some_and(|a| !a.is_empty())
    {
        return true;
    }
    scene
        .get("artboards")
        .and_then(Value::as_array)
        .is_some_and(|boards| {
            boards.iter().any(|b| {
                b.get("animations")
                    .and_then(Value::as_array)
                    .is_some_and(|a| !a.is_empty())
            })
        })
}

fn has_state_machines(scene: &Value) -> bool {
    if let Some(artboard) = scene.get("artboard")
        && artboard
            .get("state_machines")
            .and_then(Value::as_array)
            .is_some_and(|a| !a.is_empty())
    {
        return true;
    }
    scene
        .get("artboards")
        .and_then(Value::as_array)
        .is_some_and(|boards| {
            boards.iter().any(|b| {
                b.get("state_machines")
                    .and_then(Value::as_array)
                    .is_some_and(|a| !a.is_empty())
            })
        })
}

fn case_traits(scene: &Value) -> HashSet<String> {
    let mut object_types = HashSet::new();
    collect_object_types(scene, &mut object_types);

    let mut traits = HashSet::new();
    if has_animations(scene) {
        traits.insert("has_animation".to_string());
    }
    if has_state_machines(scene) {
        traits.insert("has_state_machine".to_string());
    }
    if scene
        .get("artboards")
        .and_then(Value::as_array)
        .is_some_and(|a| a.len() > 1)
    {
        traits.insert("multi_artboard".to_string());
    }

    let type_to_trait = [
        ("text", "has_text"),
        ("text_style", "has_text"),
        ("text_value_run", "has_text"),
        ("layout_component", "has_layout"),
        ("layout_component_style", "has_layout"),
        ("view_model", "has_data_binding"),
        ("view_model_property", "has_data_binding"),
        ("data_bind", "has_data_binding"),
        ("image_asset", "has_assets"),
        ("font_asset", "has_assets"),
        ("audio_asset", "has_assets"),
        ("file_asset_contents", "has_assets"),
        ("bone", "has_bones"),
        ("root_bone", "has_bones"),
        ("skin", "has_bones"),
        ("tendon", "has_bones"),
        ("weight", "has_bones"),
        ("cubic_weight", "has_bones"),
        ("ik_constraint", "has_constraints"),
        ("distance_constraint", "has_constraints"),
        ("transform_constraint", "has_constraints"),
        ("translation_constraint", "has_constraints"),
        ("scale_constraint", "has_constraints"),
        ("rotation_constraint", "has_constraints"),
        ("linear_gradient", "has_gradients"),
        ("radial_gradient", "has_gradients"),
        ("trim_path", "has_trim_path"),
    ];

    for (t, tag) in type_to_trait {
        if object_types.contains(t) {
            traits.insert(tag.to_string());
        }
    }
    traits
}

fn style_score(scene: &Value, expected_traits: &[String]) -> (f64, Vec<String>) {
    if expected_traits.is_empty() {
        return (1.0, Vec::new());
    }
    let traits = case_traits(scene);
    let matched: Vec<String> = expected_traits
        .iter()
        .filter(|t| traits.contains(t.as_str()))
        .cloned()
        .collect();
    (matched.len() as f64 / expected_traits.len() as f64, matched)
}

fn run_case(
    case: &EvalCase,
    case_dir: &Path,
    file_id: u64,
    max_retries: u8,
    baseline_hash: Option<&String>,
) -> Result<EvalCaseReport, String> {
    fs::create_dir_all(case_dir)
        .map_err(|e| format!("failed to create {}: {}", case_dir.display(), e))?;

    let provider_input = &case.input;
    fs::write(case_dir.join("input.txt"), provider_input)
        .map_err(|e| format!("failed to write input.txt: {}", e))?;

    let config = AiConfig::resolve(None, Some("template".to_string()))
        .map_err(|e| format!("AI config error: {}", e))?;
    let provider = create_provider(&config, matches!(case.input_kind, InputKind::Template))
        .map_err(|e| format!("AI provider error: {}", e))?;

    let scene_json = provider
        .generate(provider_input, &config)
        .map_err(|e| format!("generation failed: {}", e))?;
    write_json(case_dir.join("scene.json"), &scene_json)?;

    let engine = RepairEngine::new(max_retries);
    let repaired = engine
        .repair(scene_json.clone(), file_id)
        .map_err(|e| match e {
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

    fs::write(case_dir.join("output.riv"), &repaired.riv_bytes)
        .map_err(|e| format!("failed to write output.riv: {}", e))?;

    let validation: ValidationReport =
        validate_riv(&repaired.riv_bytes).map_err(|e| format!("validate failed: {}", e))?;
    write_json(case_dir.join("validate.json"), &validation)?;

    let parsed = parse_riv(&repaired.riv_bytes, &InspectFilter::default())
        .map_err(|e| format!("inspect parse failed: {}", e))?;
    write_json(case_dir.join("inspect.json"), &parsed)?;

    let repeat = engine
        .repair(scene_json.clone(), file_id)
        .map_err(|e| format!("repro check repair failed: {}", e))?;
    let reproducible = repaired.riv_bytes == repeat.riv_bytes;

    let output_hash = hash_bytes(&repaired.riv_bytes);
    let (style_score_value, matched_traits) = style_score(&scene_json, &case.expected_traits);

    let drifted = baseline_hash.is_some_and(|h| h != &output_hash);

    Ok(EvalCaseReport {
        id: case.id.clone(),
        input_kind: match case.input_kind {
            InputKind::Template => "template".to_string(),
            InputKind::Prompt => "prompt".to_string(),
        },
        input: case.input.clone(),
        expected_traits: case.expected_traits.clone(),
        style_matched_traits: matched_traits,
        style_score: style_score_value,
        valid: validation.valid,
        retries: repaired.total_retries,
        reproducible,
        output_hash: Some(output_hash),
        drifted,
        failure_reason: if validation.valid {
            None
        } else {
            Some(validation.errors.join("; "))
        },
        artifact_dir: case_dir.display().to_string(),
        text_hint: case.text_hint.clone(),
        image_path: case.image_path.clone(),
    })
}

pub fn run_eval_suite(
    suite_path: &Path,
    output_root: &Path,
    file_id: u64,
    max_retries: u8,
    baseline_path: Option<&Path>,
    write_baseline_path: Option<&Path>,
) -> Result<EvalReport, String> {
    let suite_str = fs::read_to_string(suite_path)
        .map_err(|e| format!("failed to read suite {}: {}", suite_path.display(), e))?;
    let suite: EvalSuite = serde_json::from_str(&suite_str)
        .map_err(|e| format!("failed to parse suite {}: {}", suite_path.display(), e))?;

    let baseline: Option<EvalBaseline> = if let Some(path) = baseline_path {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("failed to read baseline {}: {}", path.display(), e))?;
        Some(
            serde_json::from_str(&contents)
                .map_err(|e| format!("failed to parse baseline {}: {}", path.display(), e))?,
        )
    } else {
        None
    };

    let run_id = run_id()?;
    let run_dir = output_root.join(&run_id);
    let samples_dir = run_dir.join("samples");
    fs::create_dir_all(&samples_dir)
        .map_err(|e| format!("failed to create {}: {}", samples_dir.display(), e))?;

    write_json(run_dir.join("suite.json"), &suite)?;

    let mut cases: Vec<EvalCaseReport> = Vec::new();
    for case in &suite.cases {
        let case_dir = samples_dir.join(&case.id);
        let baseline_hash = baseline.as_ref().and_then(|b| b.case_hashes.get(&case.id));

        let result = run_case(case, &case_dir, file_id, max_retries, baseline_hash);
        match result {
            Ok(r) => cases.push(r),
            Err(err) => {
                let id = case.id.clone();
                cases.push(EvalCaseReport {
                    id,
                    input_kind: match case.input_kind {
                        InputKind::Template => "template".to_string(),
                        InputKind::Prompt => "prompt".to_string(),
                    },
                    input: case.input.clone(),
                    expected_traits: case.expected_traits.clone(),
                    style_matched_traits: Vec::new(),
                    style_score: 0.0,
                    valid: false,
                    retries: 0,
                    reproducible: false,
                    output_hash: None,
                    drifted: false,
                    failure_reason: Some(err),
                    artifact_dir: case_dir.display().to_string(),
                    text_hint: case.text_hint.clone(),
                    image_path: case.image_path.clone(),
                });
            }
        }
    }

    let valid_count = cases.iter().filter(|c| c.valid).count();
    let validity_rate = if cases.is_empty() {
        0.0
    } else {
        valid_count as f64 / cases.len() as f64
    };

    let average_retries = if cases.is_empty() {
        0.0
    } else {
        cases.iter().map(|c| c.retries as f64).sum::<f64>() / cases.len() as f64
    };

    let style_adherence_rate = if cases.is_empty() {
        0.0
    } else {
        cases.iter().map(|c| c.style_score).sum::<f64>() / cases.len() as f64
    };

    let reproducibility_rate = if cases.is_empty() {
        0.0
    } else {
        cases.iter().filter(|c| c.reproducible).count() as f64 / cases.len() as f64
    };

    let drift_count = cases.iter().filter(|c| c.drifted).count();

    let report = EvalReport {
        run_id: run_id.clone(),
        suite_name: suite.suite_name.clone(),
        suite_version: suite.suite_version,
        output_dir: run_dir.display().to_string(),
        case_count: cases.len(),
        valid_count,
        validity_rate,
        average_retries,
        style_adherence_rate,
        reproducibility_rate,
        drift_count,
        cases,
    };

    write_json(run_dir.join("report.json"), &report)?;

    if let Some(path) = write_baseline_path {
        let mut case_hashes = BTreeMap::new();
        for case in &report.cases {
            if let Some(hash) = &case.output_hash {
                case_hashes.insert(case.id.clone(), hash.clone());
            }
        }
        let baseline = EvalBaseline {
            suite_name: suite.suite_name,
            suite_version: suite.suite_version,
            case_hashes,
        };
        write_json(path, &baseline)?;
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_score_empty_expectations() {
        let scene = serde_json::json!({"scene_format_version":1});
        let (score, matched) = style_score(&scene, &[]);
        assert_eq!(score, 1.0);
        assert!(matched.is_empty());
    }

    #[test]
    fn test_style_score_matches_animation_trait() {
        let scene = serde_json::json!({
            "scene_format_version": 1,
            "artboard": {
                "name": "A",
                "width": 100,
                "height": 100,
                "children": [],
                "animations": [{"name": "anim", "fps": 60, "duration": 10, "keyframes": []}]
            }
        });
        let expected = vec!["has_animation".to_string(), "has_state_machine".to_string()];
        let (score, matched) = style_score(&scene, &expected);
        assert!(score > 0.4 && score < 0.6);
        assert_eq!(matched, vec!["has_animation".to_string()]);
    }
}
