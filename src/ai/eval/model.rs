use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

fn one() -> f64 {
    1.0
}

fn runtime_fps() -> f64 {
    60.0
}

fn runtime_dimension() -> u32 {
    256
}

fn runtime_scale() -> u32 {
    1
}

fn one_frame() -> usize {
    1
}

fn two_colours() -> usize {
    2
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
    #[serde(default = "one")]
    pub min_runtime_pass_rate: f64,
    #[serde(default = "one")]
    pub min_semantic_static_pass_rate: f64,
    #[serde(default = "one")]
    pub min_semantic_animated_pass_rate: f64,
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
            min_runtime_pass_rate: 1.0,
            min_semantic_static_pass_rate: 1.0,
            min_semantic_animated_pass_rate: 1.0,
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
    #[serde(default)]
    pub runtime: Option<RuntimeExpectations>,
    #[serde(default)]
    pub semantic: Option<SemanticExpectations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticExpectations {
    #[serde(default)]
    pub static_checks: Vec<StaticSemanticCheck>,
    #[serde(default)]
    pub animated_checks: Vec<AnimatedSemanticCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum StaticSemanticCheck {
    AuthoredIdPresent {
        authored_id: String,
    },
    AuthoredIdHasRuntimeType {
        authored_id: String,
        object_type: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnimatedSemanticCheck {
    FramesDiffer { from: u32, to: u32 },
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticCheckEvidence {
    pub check: String,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticEvidence {
    pub static_passed: Option<bool>,
    pub animated_passed: Option<bool>,
    pub static_checks: Vec<SemanticCheckEvidence>,
    pub animated_checks: Vec<SemanticCheckEvidence>,
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeExpectations {
    #[serde(default)]
    pub frames: Vec<u32>,
    #[serde(default = "runtime_fps")]
    pub fps: f64,
    #[serde(default)]
    pub animation: Option<String>,
    #[serde(default)]
    pub state_machine: Option<String>,
    #[serde(default)]
    pub artboard: Option<String>,
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default = "runtime_dimension")]
    pub width: u32,
    #[serde(default = "runtime_dimension")]
    pub height: u32,
    #[serde(default = "runtime_scale")]
    pub scale: u32,
    #[serde(default = "one_frame")]
    pub min_non_blank_frames: usize,
    #[serde(default = "two_colours")]
    pub min_distinct_colors: usize,
}

impl Default for RuntimeExpectations {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            fps: runtime_fps(),
            animation: None,
            state_machine: None,
            artboard: None,
            background: None,
            width: runtime_dimension(),
            height: runtime_dimension(),
            scale: runtime_scale(),
            min_non_blank_frames: one_frame(),
            min_distinct_colors: two_colours(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeEvidence {
    pub passed: bool,
    pub rendered_frame_count: usize,
    pub non_blank_frame_count: usize,
    pub minimum_distinct_colors_observed: usize,
    pub manifest_path: Option<String>,
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputKind {
    Template,
    Prompt,
    AuthoringExample,
}

impl InputKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Template => "template",
            Self::Prompt => "prompt",
            Self::AuthoringExample => "authoring_example",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvalFailureStage {
    AuthoringSchema,
    Lowering,
    Structural,
    Runtime,
    SemanticMismatch,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvalDiagnosticEvidence {
    pub path: String,
    pub code: String,
    pub message: String,
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
    pub runtime_case_count: usize,
    pub runtime_pass_count: usize,
    pub runtime_pass_rate: f64,
    pub semantic_static_case_count: usize,
    pub semantic_static_pass_count: usize,
    pub semantic_static_pass_rate: f64,
    pub semantic_animated_case_count: usize,
    pub semantic_animated_pass_count: usize,
    pub semantic_animated_pass_rate: f64,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_stage: Option<EvalFailureStage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<EvalDiagnosticEvidence>,
    pub artifact_dir: String,
    pub text_hint: Option<String>,
    pub image_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<RuntimeEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic: Option<SemanticEvidence>,
}
