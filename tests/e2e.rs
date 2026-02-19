use std::path::PathBuf;
use std::process::Command;

fn cargo_run(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .output()
        .expect("failed to run cargo")
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn temp_output(test_name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("rive_e2e_{}.riv", test_name))
}

fn cleanup(path: &PathBuf) {
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_generate_minimal() {
    let input = fixture_path("minimal.json");
    let output = temp_output("minimal");
    cleanup(&output);

    let result = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
    let bytes = std::fs::read(&output).unwrap();
    assert!(bytes.len() > 4);
    assert_eq!(&bytes[0..4], b"RIVE");
    cleanup(&output);
}

#[test]
fn test_generate_shapes() {
    let input = fixture_path("shapes.json");
    let output = temp_output("shapes");
    cleanup(&output);

    let result = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
    let bytes = std::fs::read(&output).unwrap();
    assert_eq!(&bytes[0..4], b"RIVE");
    cleanup(&output);
}

#[test]
fn test_generate_animation() {
    let input = fixture_path("animation.json");
    let output = temp_output("animation");
    cleanup(&output);

    let result = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
    let bytes = std::fs::read(&output).unwrap();
    assert_eq!(&bytes[0..4], b"RIVE");
    cleanup(&output);
}

#[test]
fn test_generate_state_machine() {
    let input = fixture_path("state_machine.json");
    let output = temp_output("state_machine");
    cleanup(&output);

    let result = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
    let bytes = std::fs::read(&output).unwrap();
    assert_eq!(&bytes[0..4], b"RIVE");
    cleanup(&output);
}

#[test]
fn test_generate_path() {
    let input = fixture_path("path.json");
    let output = temp_output("path");
    cleanup(&output);

    let result = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
    let bytes = std::fs::read(&output).unwrap();
    assert_eq!(&bytes[0..4], b"RIVE");
    cleanup(&output);
}

#[test]
fn test_validate_generated_file() {
    let input = fixture_path("minimal.json");
    let output = temp_output("validate_gen");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let val = cargo_run(&["validate", output.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&val.stdout);
    assert!(
        val.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&val.stderr)
    );
    assert!(
        stdout.contains("valid"),
        "expected 'valid' in stdout, got: {}",
        stdout
    );
    cleanup(&output);
}

#[test]
fn test_inspect_generated_file() {
    let input = fixture_path("minimal.json");
    let output = temp_output("inspect_gen");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let insp = cargo_run(&["inspect", output.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&insp.stdout);
    assert!(
        insp.status.success(),
        "inspect failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );
    assert!(
        stdout.contains("Artboard"),
        "expected 'Artboard' in output, got: {}",
        stdout
    );
    cleanup(&output);
}

#[test]
fn test_inspect_json_flag() {
    let input = fixture_path("minimal.json");
    let output = temp_output("inspect_json");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let insp = cargo_run(&["inspect", "--json", output.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&insp.stdout);
    assert!(
        insp.status.success(),
        "inspect --json failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("inspect --json output is not valid JSON");
    assert!(parsed.get("header").is_some());
    assert!(parsed.get("objects").is_some());
    cleanup(&output);
}

#[test]
fn test_generate_invalid_input() {
    let result = cargo_run(&["generate", "nonexistent_file.json"]);
    assert!(!result.status.success());
}

#[test]
fn test_validate_invalid_file() {
    let path = std::env::temp_dir().join("rive_e2e_invalid.bin");
    std::fs::write(&path, b"not a riv file").unwrap();

    let result = cargo_run(&["validate", path.to_str().unwrap()]);
    assert!(!result.status.success());

    let _ = std::fs::remove_file(&path);
}
