use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

fn one() -> f64 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalSuite {
    pub suite_name: String,
    pub suite_version: u32,
    #[serde(default)]
    pub gates: EvalGates,
    pub cases: Vec<EvalCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalGates {
    #[serde(default = "one")]
    pub min_validity_rate: f64,
    #[serde(default = "one")]
    pub min_trait_adherence_rate: f64,
    #[serde(default = "one")]
    pub min_pipeline_reproducibility_rate: f64,
    #[serde(default)]
    pub max_average_retries: Option<f64>,
    #[serde(default)]
    pub max_drift_count: usize,
}

impl Default for EvalGates {
    fn default() -> Self {
        Self {
            min_validity_rate: 1.0,
            min_trait_adherence_rate: 1.0,
            min_pipeline_reproducibility_rate: 1.0,
            max_average_retries: None,
            max_drift_count: 0,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputKind {
    Template,
    Prompt,
}

impl InputKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Template => "template",
            Self::Prompt => "prompt",
        }
    }
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
    pub provider: String,
    pub model: String,
    pub baseline_used: bool,
    pub passed: bool,
    pub gate_failures: Vec<String>,
    pub case_count: usize,
    pub valid_count: usize,
    pub validity_rate: f64,
    pub average_retries: f64,
    pub trait_adherence_rate: f64,
    pub style_adherence_rate: f64,
    pub pipeline_reproducibility_rate: f64,
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
