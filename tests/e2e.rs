use std::path::PathBuf;
use std::process::Command;

const ARTBOARD_TYPE_KEY: u64 = 1;
const ARTBOARD_WIDTH_KEY: u64 = 7;
const ARTBOARD_HEIGHT_KEY: u64 = 8;

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

#[test]
fn test_generate_cubic_easing() {
    let input = fixture_path("cubic_easing.json");
    let output = temp_output("cubic_easing");
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
fn test_generate_trim_path() {
    let input = fixture_path("trim_path.json");
    let output = temp_output("trim_path");
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
fn test_generate_multi_artboard() {
    let input = fixture_path("multi_artboard.json");
    let output = temp_output("multi_artboard");
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
fn test_validate_multi_artboard() {
    let input = fixture_path("multi_artboard.json");
    let output = temp_output("validate_multi_artboard");
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
fn test_inspect_multi_artboard() {
    let input = fixture_path("multi_artboard.json");
    let output = temp_output("inspect_multi_artboard");
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
        stdout.contains("Artboards: 2"),
        "expected multi-artboard count in output, got: {}",
        stdout
    );
    assert!(
        stdout.contains("Screen A"),
        "expected Screen A in output, got: {}",
        stdout
    );
    assert!(
        stdout.contains("Screen B"),
        "expected Screen B in output, got: {}",
        stdout
    );
    cleanup(&output);
}

#[test]
fn test_inspect_json_multi_artboard() {
    let input = fixture_path("multi_artboard.json");
    let output = temp_output("inspect_json_multi");
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
    let objects = parsed.get("objects").unwrap().as_array().unwrap();
    let artboard_count = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == ARTBOARD_TYPE_KEY)
        .count();
    assert_eq!(artboard_count, 2);
    cleanup(&output);
}

#[test]
fn test_generate_nested_artboard() {
    let input = fixture_path("nested_artboard.json");
    let output = temp_output("nested_artboard");
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
fn test_inspect_nested_artboard() {
    let input = fixture_path("nested_artboard.json");
    let output = temp_output("inspect_nested_artboard");
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
        stdout.contains("NestedArtboard"),
        "expected 'NestedArtboard' in output, got: {}",
        stdout
    );

    cleanup(&output);
}

#[test]
fn test_generate_artboard_preset() {
    let input = fixture_path("artboard_preset.json");
    let output = temp_output("artboard_preset");
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

    let inspect = cargo_run(&["inspect", "--json", output.to_str().unwrap()]);
    assert!(
        inspect.status.success(),
        "inspect --json failed: {}",
        String::from_utf8_lossy(&inspect.stderr)
    );

    let parsed: serde_json::Value =
        serde_json::from_slice(&inspect.stdout).expect("inspect --json output is not valid JSON");
    let objects = parsed
        .get("objects")
        .and_then(|v| v.as_array())
        .expect("objects array missing");
    let artboard = objects
        .iter()
        .find(|o| o.get("type_key").and_then(|k| k.as_u64()) == Some(ARTBOARD_TYPE_KEY))
        .expect("artboard object missing");
    let properties = artboard
        .get("properties")
        .and_then(|v| v.as_array())
        .expect("artboard properties missing");

    let width = properties
        .iter()
        .find(|p| p.get("key").and_then(|k| k.as_u64()) == Some(ARTBOARD_WIDTH_KEY))
        .and_then(|p| p.get("value"))
        .and_then(|v| v.get("Float"))
        .and_then(|v| v.as_f64())
        .expect("artboard width property missing");
    let height = properties
        .iter()
        .find(|p| p.get("key").and_then(|k| k.as_u64()) == Some(ARTBOARD_HEIGHT_KEY))
        .and_then(|p| p.get("value"))
        .and_then(|v| v.get("Float"))
        .and_then(|v| v.as_f64())
        .expect("artboard height property missing");

    assert_eq!(width, 390.0);
    assert_eq!(height, 844.0);
    cleanup(&output);
}

#[test]
fn test_list_presets_flag() {
    let result = cargo_run(&["--list-presets"]);
    let stdout = String::from_utf8_lossy(&result.stdout);

    assert!(
        result.status.success(),
        "--list-presets failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        stdout.contains("mobile: 390x844"),
        "expected mobile preset in output, got: {}",
        stdout
    );
    assert!(
        stdout.contains("story: 1080x1920"),
        "expected story preset in output, got: {}",
        stdout
    );
}
