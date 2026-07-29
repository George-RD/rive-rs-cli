use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn rive_cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rive-cli"))
        .args(args)
        .output()
        .expect("failed to run rive-cli")
}

fn temp_dir(name: &str) -> PathBuf {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "rive_eval_contract_{}_{}_{}",
        name,
        std::process::id(),
        id
    ))
}

fn write_suite(root: &Path, contents: &str) -> PathBuf {
    std::fs::create_dir_all(root).expect("failed to create temp root");
    let path = root.join("suite.json");
    std::fs::write(&path, contents).expect("failed to write suite");
    path
}

#[test]
fn template_suite_passes_declared_quality_gates() {
    let root = temp_dir("pass");
    let suite = write_suite(
        &root,
        r#"{
  "suite_name": "integration-pass",
  "suite_version": 1,
  "cases": [{
    "id": "bounce",
    "input_kind": "template",
    "input": "bounce",
    "expected_traits": ["has_animation"]
  }]
}"#,
    );
    let runs = root.join("runs");
    let output = rive_cli(&[
        "ai",
        "lab",
        "--suite",
        suite.to_str().unwrap(),
        "--output-dir",
        runs.to_str().unwrap(),
        "--json",
    ]);
    assert!(
        output.status.success(),
        "ai lab failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid report JSON");
    assert_eq!(report["passed"], true);
    assert_eq!(report["validity_rate"], 1.0);
    assert_eq!(report["trait_adherence_rate"], 1.0);
    assert_eq!(report["pipeline_reproducibility_rate"], 1.0);
    assert_eq!(report["provider"], "template");
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn breached_quality_gate_fails_the_command() {
    let root = temp_dir("gate");
    let suite = write_suite(
        &root,
        r#"{
  "suite_name": "integration-gate",
  "suite_version": 1,
  "cases": [{
    "id": "bounce",
    "input_kind": "template",
    "input": "bounce",
    "expected_traits": ["has_state_machine"]
  }]
}"#,
    );
    let output = rive_cli(&[
        "ai",
        "lab",
        "--suite",
        suite.to_str().unwrap(),
        "--output-dir",
        root.join("runs").to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("evaluation gate failed"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn prompt_case_rejects_the_template_provider() {
    let root = temp_dir("provider");
    let suite = write_suite(
        &root,
        r#"{
  "suite_name": "integration-provider",
  "suite_version": 1,
  "cases": [{
    "id": "prompt",
    "input_kind": "prompt",
    "input": "a bouncing ball",
    "expected_traits": ["has_animation"]
  }]
}"#,
    );
    let output = rive_cli(&[
        "ai",
        "lab",
        "--suite",
        suite.to_str().unwrap(),
        "--provider",
        "template",
        "--output-dir",
        root.join("runs").to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("prompt cases require"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn ai_lab_help_exposes_prompt_provider_controls() {
    let output = rive_cli(&["ai", "lab", "--help"]);
    assert!(output.status.success());
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(help.contains("--provider"));
    assert!(help.contains("--model"));
}
