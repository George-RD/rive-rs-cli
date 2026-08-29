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
        "rive_interactive_eval_contract_{}_{}_{}",
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
fn interactive_semantic_checks_require_runtime_evidence() {
    let root = temp_dir("runtime-required");
    let suite = write_suite(
        &root,
        r#"{
  "suite_name": "interactive-runtime-required",
  "suite_version": 1,
  "cases": [{
    "id": "interactive",
    "input_kind": "authoring_example",
    "input": "examples/authoring/complex-interactive-showcase.v0.json",
    "semantic": {
      "interactive_checks": [{
        "kind": "transition",
        "statechart_id": "flow",
        "transition_id": "focus",
        "from_state": "queued-state",
        "to_state": "focused-state",
        "input_id": "selected",
        "equals": true
      }]
    }
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
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("interactive semantic checks require runtime expectations")
    );
    let _ = std::fs::remove_dir_all(root);
}
