use std::path::PathBuf;
use std::process::Command;

const ARTBOARD_TYPE_KEY: u64 = 1;
const BACKBOARD_TYPE_KEY: u64 = 23;
const COMPONENT_NAME_KEY: u64 = 4;
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
fn test_inspect_filter_type_key_json() {
    let input = fixture_path("minimal.json");
    let output = temp_output("inspect_filter_type_key_json");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let insp = cargo_run(&[
        "inspect",
        "--json",
        "--type-key",
        "1",
        output.to_str().unwrap(),
    ]);
    assert!(
        insp.status.success(),
        "inspect --json --type-key failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );

    let parsed: serde_json::Value =
        serde_json::from_slice(&insp.stdout).expect("inspect output is not valid JSON");
    let objects = parsed
        .get("objects")
        .and_then(|v| v.as_array())
        .expect("objects array missing");
    assert_eq!(objects.len(), 1);
    assert_eq!(
        objects[0].get("type_key").and_then(|v| v.as_u64()),
        Some(ARTBOARD_TYPE_KEY)
    );
    cleanup(&output);
}

#[test]
fn test_inspect_filter_type_name_case_insensitive_json() {
    let input = fixture_path("minimal.json");
    let output = temp_output("inspect_filter_type_name_json");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let insp = cargo_run(&[
        "inspect",
        "--json",
        "--type-name",
        "artboard",
        output.to_str().unwrap(),
    ]);
    assert!(
        insp.status.success(),
        "inspect --json --type-name failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );

    let parsed: serde_json::Value =
        serde_json::from_slice(&insp.stdout).expect("inspect output is not valid JSON");
    let objects = parsed
        .get("objects")
        .and_then(|v| v.as_array())
        .expect("objects array missing");
    assert_eq!(objects.len(), 1);
    assert_eq!(
        objects[0].get("type_key").and_then(|v| v.as_u64()),
        Some(ARTBOARD_TYPE_KEY)
    );
    cleanup(&output);
}

#[test]
fn test_inspect_filter_object_index_json() {
    let input = fixture_path("minimal.json");
    let output = temp_output("inspect_filter_object_index_json");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let insp = cargo_run(&[
        "inspect",
        "--json",
        "--object-index",
        "0",
        output.to_str().unwrap(),
    ]);
    assert!(
        insp.status.success(),
        "inspect --json --object-index failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );

    let parsed: serde_json::Value =
        serde_json::from_slice(&insp.stdout).expect("inspect output is not valid JSON");
    let objects = parsed
        .get("objects")
        .and_then(|v| v.as_array())
        .expect("objects array missing");
    assert_eq!(objects.len(), 1);
    assert_eq!(
        objects[0].get("type_key").and_then(|v| v.as_u64()),
        Some(BACKBOARD_TYPE_KEY)
    );
    cleanup(&output);
}

#[test]
fn test_inspect_filter_property_key_json() {
    let input = fixture_path("minimal.json");
    let output = temp_output("inspect_filter_property_key_json");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let insp = cargo_run(&[
        "inspect",
        "--json",
        "--type-key",
        "1",
        "--property-key",
        "4",
        output.to_str().unwrap(),
    ]);
    assert!(
        insp.status.success(),
        "inspect --json --property-key failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );

    let parsed: serde_json::Value =
        serde_json::from_slice(&insp.stdout).expect("inspect output is not valid JSON");
    let objects = parsed
        .get("objects")
        .and_then(|v| v.as_array())
        .expect("objects array missing");
    assert_eq!(objects.len(), 1);
    let properties = objects[0]
        .get("properties")
        .and_then(|v| v.as_array())
        .expect("properties array missing");
    assert_eq!(properties.len(), 1);
    assert_eq!(
        properties[0].get("key").and_then(|v| v.as_u64()),
        Some(COMPONENT_NAME_KEY)
    );
    cleanup(&output);
}

#[test]
fn test_inspect_filter_combined_and_logic_json() {
    let input = fixture_path("minimal.json");
    let output = temp_output("inspect_filter_combined_json");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let insp = cargo_run(&[
        "inspect",
        "--json",
        "--type-name",
        "ARTBOARD",
        "--object-index",
        "1",
        "--property-key",
        "4",
        output.to_str().unwrap(),
    ]);
    assert!(
        insp.status.success(),
        "inspect --json combined filters failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );

    let parsed: serde_json::Value =
        serde_json::from_slice(&insp.stdout).expect("inspect output is not valid JSON");
    let objects = parsed
        .get("objects")
        .and_then(|v| v.as_array())
        .expect("objects array missing");
    assert_eq!(objects.len(), 1);
    assert_eq!(
        objects[0].get("type_key").and_then(|v| v.as_u64()),
        Some(ARTBOARD_TYPE_KEY)
    );
    let properties = objects[0]
        .get("properties")
        .and_then(|v| v.as_array())
        .expect("properties array missing");
    assert_eq!(properties.len(), 1);
    assert_eq!(
        properties[0].get("key").and_then(|v| v.as_u64()),
        Some(COMPONENT_NAME_KEY)
    );
    cleanup(&output);
}

#[test]
fn test_inspect_filter_no_match_human_output() {
    let input = fixture_path("minimal.json");
    let output = temp_output("inspect_filter_no_match");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let insp = cargo_run(&["inspect", "--object-index", "99", output.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&insp.stdout);
    assert!(
        insp.status.success(),
        "inspect with non-matching filters failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );
    assert!(
        stdout.contains("No objects matched the provided filters."),
        "expected no-match message in output, got: {}",
        stdout
    );
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
fn test_generate_gradients() {
    let input = fixture_path("gradients.json");
    let output = temp_output("gradients");
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
fn test_validate_gradients() {
    let input = fixture_path("gradients.json");
    let output = temp_output("validate_gradients");
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
fn test_inspect_gradients() {
    let input = fixture_path("gradients.json");
    let output = temp_output("inspect_gradients");
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
fn test_generate_color_animation() {
    let input = fixture_path("color_animation.json");
    let output = temp_output("color_animation");
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
fn test_validate_color_animation() {
    let input = fixture_path("color_animation.json");
    let output = temp_output("validate_color_animation");
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
fn test_inspect_color_animation() {
    let input = fixture_path("color_animation.json");
    let output = temp_output("inspect_color_animation");
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
fn test_generate_loop_animation() {
    let input = fixture_path("loop_animation.json");
    let output = temp_output("loop_animation");
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
fn test_validate_loop_animation() {
    let input = fixture_path("loop_animation.json");
    let output = temp_output("validate_loop_animation");
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
fn test_inspect_loop_animation() {
    let input = fixture_path("loop_animation.json");
    let output = temp_output("inspect_loop_animation");
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
fn test_generate_stroke_styles() {
    let input = fixture_path("stroke_styles.json");
    let output = temp_output("stroke_styles");
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
fn test_validate_stroke_styles() {
    let input = fixture_path("stroke_styles.json");
    let output = temp_output("validate_stroke_styles");
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
fn test_inspect_stroke_styles() {
    let input = fixture_path("stroke_styles.json");
    let output = temp_output("inspect_stroke_styles");
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
fn test_generate_empty_artboard() {
    let input = fixture_path("empty_artboard.json");
    let output = temp_output("empty_artboard");
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
fn test_validate_empty_artboard() {
    let input = fixture_path("empty_artboard.json");
    let output = temp_output("validate_empty_artboard");
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
fn test_inspect_empty_artboard() {
    let input = fixture_path("empty_artboard.json");
    let output = temp_output("inspect_empty_artboard");
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
fn test_validate_artboard_preset() {
    let input = fixture_path("artboard_preset.json");
    let output = temp_output("validate_artboard_preset");
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
fn test_ai_generate_template_bounce() {
    let output = temp_output("ai_template_bounce");
    cleanup(&output);

    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "ai generate failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let val = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        val.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&val.stderr)
    );
    cleanup(&output);
}

#[test]
fn test_ai_generate_template_spinner() {
    let output = temp_output("ai_template_spinner");
    cleanup(&output);

    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "spinner",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "ai generate failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let val = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        val.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&val.stderr)
    );
    cleanup(&output);
}

#[test]
fn test_ai_generate_template_pulse() {
    let output = temp_output("ai_template_pulse");
    cleanup(&output);

    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "pulse",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "ai generate failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let val = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        val.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&val.stderr)
    );
    cleanup(&output);
}

#[test]
fn test_ai_generate_template_fade() {
    let output = temp_output("ai_template_fade");
    cleanup(&output);

    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "fade",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "ai generate failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let val = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        val.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&val.stderr)
    );
    cleanup(&output);
}

#[test]
fn test_ai_generate_dry_run() {
    let result = cargo_run(&["ai", "generate", "--template", "bounce", "--dry-run"]);
    assert!(
        result.status.success(),
        "ai dry-run failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );

    let stdout = String::from_utf8_lossy(&result.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("dry-run JSON should parse");
    assert_eq!(
        parsed.get("scene_format_version").and_then(|v| v.as_u64()),
        Some(1)
    );
}

#[test]
fn test_ai_dry_run_pipe_to_generate() {
    let json_path = std::env::temp_dir().join("rive_ai_dry_run_pipe.json");
    let output = temp_output("ai_dry_run_pipe");
    let _ = std::fs::remove_file(&json_path);
    cleanup(&output);

    let dry_run = cargo_run(&["ai", "generate", "--template", "bounce", "--dry-run"]);
    assert!(
        dry_run.status.success(),
        "ai dry-run failed: {}",
        String::from_utf8_lossy(&dry_run.stderr)
    );

    std::fs::write(&json_path, &dry_run.stdout).expect("failed to write dry-run output");

    let generate = cargo_run(&[
        "generate",
        json_path.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        generate.status.success(),
        "generate from dry-run failed: {}",
        String::from_utf8_lossy(&generate.stderr)
    );

    let val = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        val.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&val.stderr)
    );

    let _ = std::fs::remove_file(&json_path);
    cleanup(&output);
}

#[test]
fn test_ai_unknown_template_error() {
    let result = cargo_run(&["ai", "generate", "--template", "nonexistent"]);
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!result.status.success());
    assert!(
        stderr.contains("unknown template"),
        "stderr was: {}",
        stderr
    );
}

#[test]
fn test_ai_no_prompt_or_template() {
    let result = cargo_run(&["ai", "generate"]);
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!result.status.success());
    assert!(
        stderr.contains("provide --prompt or --template"),
        "stderr was: {}",
        stderr
    );
}

#[test]
fn test_ai_generate_prompt_without_api_key() {
    let result = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "ai",
            "generate",
            "--prompt",
            "make a bounce",
        ])
        .env_remove("OPENAI_API_KEY")
        .output()
        .expect("failed to run cargo");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!result.status.success());
    assert!(
        stderr.contains("no API key set")
            || stderr.contains("API key missing")
            || stderr.contains("OpenAI provider not yet implemented"),
        "stderr was: {}",
        stderr
    );
}

#[test]
fn test_ai_rejects_both_template_and_prompt() {
    let result = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "ai",
            "generate",
            "--template",
            "bounce",
            "--prompt",
            "make something",
        ])
        .env_remove("OPENAI_API_KEY")
        .output()
        .expect("failed to run cargo");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!result.status.success());
    assert!(
        stderr.contains("cannot use both --template and --prompt"),
        "expected conflict error, got: {}",
        stderr
    );
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

#[test]
fn test_ai_generate_with_repair_retries() {
    let output = temp_output("ai_generate_with_repair_retries");
    cleanup(&output);

    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "--max-retries",
        "3",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "ai generate with repair failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
    let bytes = std::fs::read(&output).unwrap();
    assert_eq!(&bytes[0..4], b"RIVE");

    let val = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        val.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&val.stderr)
    );
    cleanup(&output);
}

#[test]
fn test_ai_repair_zero_retries_still_succeeds_valid_template() {
    let output = temp_output("ai_repair_zero_retries");
    cleanup(&output);

    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "--max-retries",
        "0",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "ai generate with --max-retries 0 should succeed for valid templates: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
    let bytes = std::fs::read(&output).unwrap();
    assert_eq!(&bytes[0..4], b"RIVE");
    cleanup(&output);
}

#[test]
fn test_ai_repair_dry_run_skips_repair() {
    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "bounce",
        "--dry-run",
        "--max-retries",
        "0",
    ]);
    assert!(
        result.status.success(),
        "ai dry-run should skip repair: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout = String::from_utf8_lossy(&result.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("dry-run JSON should parse");
    assert_eq!(
        parsed.get("scene_format_version").and_then(|v| v.as_u64()),
        Some(1)
    );
}

#[test]
fn test_ai_max_retries_flag_accepted() {
    let output = temp_output("ai_max_retries_flag");
    cleanup(&output);

    let result = cargo_run(&[
        "ai",
        "generate",
        "--template",
        "spinner",
        "--max-retries",
        "5",
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "ai generate with --max-retries 5 failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(output.exists());
    let bytes = std::fs::read(&output).unwrap();
    assert_eq!(&bytes[0..4], b"RIVE");
    cleanup(&output);
}

#[test]
fn test_ai_lab_generates_report_and_artifacts() {
    let root = std::env::temp_dir().join("rive_ai_lab_e2e_run");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("failed to create temp root");

    let suite_path = root.join("suite.json");
    let suite_json = r#"{
  "suite_name": "e2e-suite",
  "suite_version": 1,
  "cases": [
    {
      "id": "bounce-case",
      "input_kind": "template",
      "input": "bounce",
      "expected_traits": ["has_animation"]
    }
  ]
}"#;
    std::fs::write(&suite_path, suite_json).expect("failed to write suite json");

    let output_dir = root.join("runs");
    let result = cargo_run(&[
        "ai",
        "lab",
        "--suite",
        suite_path.to_str().unwrap(),
        "--output-dir",
        output_dir.to_str().unwrap(),
    ]);

    assert!(
        result.status.success(),
        "ai lab failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );

    let stdout = String::from_utf8_lossy(&result.stdout);
    let run_id_line = stdout
        .lines()
        .find(|line| line.starts_with("run_id="))
        .expect("missing run_id output");
    let run_id = run_id_line.trim_start_matches("run_id=");

    let run_dir = output_dir.join(run_id);
    assert!(run_dir.join("report.json").exists());
    assert!(run_dir.join("samples/bounce-case/scene.json").exists());
    assert!(run_dir.join("samples/bounce-case/output.riv").exists());
    assert!(run_dir.join("samples/bounce-case/validate.json").exists());
    assert!(run_dir.join("samples/bounce-case/inspect.json").exists());

    let report_str = std::fs::read_to_string(run_dir.join("report.json")).unwrap();
    let report: serde_json::Value = serde_json::from_str(&report_str).unwrap();
    assert_eq!(report["case_count"].as_u64(), Some(1));
    assert_eq!(report["valid_count"].as_u64(), Some(1));

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn test_ai_lab_regression_flags_drift() {
    let root = std::env::temp_dir().join("rive_ai_lab_e2e_drift");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("failed to create temp root");

    let suite_path = root.join("suite.json");
    let suite_json = r#"{
  "suite_name": "e2e-drift-suite",
  "suite_version": 1,
  "cases": [
    {
      "id": "bounce-case",
      "input_kind": "template",
      "input": "bounce",
      "expected_traits": ["has_animation"]
    }
  ]
}"#;
    std::fs::write(&suite_path, suite_json).expect("failed to write suite json");

    let baseline_path = root.join("baseline.json");
    let baseline_json = r#"{
  "suite_name": "e2e-drift-suite",
  "suite_version": 1,
  "case_hashes": {
    "bounce-case": "deadbeefdeadbeef"
  }
}"#;
    std::fs::write(&baseline_path, baseline_json).expect("failed to write baseline json");

    let output_dir = root.join("runs");
    let result = cargo_run(&[
        "ai",
        "lab",
        "--suite",
        suite_path.to_str().unwrap(),
        "--output-dir",
        output_dir.to_str().unwrap(),
        "--baseline",
        baseline_path.to_str().unwrap(),
    ]);

    assert!(!result.status.success(), "expected drift failure");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("regression drift detected"));

    let _ = std::fs::remove_dir_all(&root);
}

const BONE_TYPE_KEY: u64 = 40;
const ROOT_BONE_TYPE_KEY: u64 = 41;
const SKIN_TYPE_KEY: u64 = 43;
const TENDON_TYPE_KEY: u64 = 44;
const WEIGHT_TYPE_KEY: u64 = 45;
const CUBIC_WEIGHT_TYPE_KEY: u64 = 46;
const IK_CONSTRAINT_TYPE_KEY: u64 = 81;
const DISTANCE_CONSTRAINT_TYPE_KEY: u64 = 82;
const TRANSFORM_CONSTRAINT_TYPE_KEY: u64 = 83;
const TRANSLATION_CONSTRAINT_TYPE_KEY: u64 = 87;
const SCALE_CONSTRAINT_TYPE_KEY: u64 = 88;
const ROTATION_CONSTRAINT_TYPE_KEY: u64 = 89;

#[test]
fn test_generate_bones() {
    let input = fixture_path("bones.json");
    let output = temp_output("bones");
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
fn test_validate_bones() {
    let input = fixture_path("bones.json");
    let output = temp_output("validate_bones");
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
fn test_inspect_bones() {
    let input = fixture_path("bones.json");
    let output = temp_output("inspect_bones");
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
        stdout.contains("Rigging"),
        "expected artboard name 'Rigging' in output, got: {}",
        stdout
    );
    cleanup(&output);
}

#[test]
fn test_inspect_json_bones() {
    let input = fixture_path("bones.json");
    let output = temp_output("inspect_json_bones");
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

    let root_bones: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == ROOT_BONE_TYPE_KEY)
        .collect();
    assert_eq!(root_bones.len(), 1, "expected 1 RootBone");

    let bones: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == BONE_TYPE_KEY)
        .collect();
    assert!(
        bones.len() >= 4,
        "expected at least 4 Bones (Torso, Neck, LeftArm, LeftForearm), got {}",
        bones.len()
    );

    let skins: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == SKIN_TYPE_KEY)
        .collect();
    assert_eq!(skins.len(), 1, "expected 1 Skin");

    let tendons: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == TENDON_TYPE_KEY)
        .collect();
    assert_eq!(tendons.len(), 2, "expected 2 Tendons");

    let weights: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == WEIGHT_TYPE_KEY)
        .collect();
    assert_eq!(weights.len(), 1, "expected 1 Weight");

    let cubic_weights: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == CUBIC_WEIGHT_TYPE_KEY)
        .collect();
    assert_eq!(cubic_weights.len(), 1, "expected 1 CubicWeight");

    cleanup(&output);
}

#[test]
fn test_generate_constraints() {
    let input = fixture_path("constraints.json");
    let output = temp_output("constraints");
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
fn test_validate_constraints() {
    let input = fixture_path("constraints.json");
    let output = temp_output("validate_constraints");
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
fn test_inspect_constraints() {
    let input = fixture_path("constraints.json");
    let output = temp_output("inspect_constraints");
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
        stdout.contains("Constraints"),
        "expected artboard name 'Constraints' in output, got: {}",
        stdout
    );
    cleanup(&output);
}

#[test]
fn test_inspect_json_constraints() {
    let input = fixture_path("constraints.json");
    let output = temp_output("inspect_json_constraints");
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

    let ik: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == IK_CONSTRAINT_TYPE_KEY)
        .collect();
    assert_eq!(ik.len(), 1, "expected 1 IKConstraint");

    let dist: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == DISTANCE_CONSTRAINT_TYPE_KEY)
        .collect();
    assert_eq!(dist.len(), 1, "expected 1 DistanceConstraint");

    let transform: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == TRANSFORM_CONSTRAINT_TYPE_KEY)
        .collect();
    assert_eq!(transform.len(), 1, "expected 1 TransformConstraint");

    let translation: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == TRANSLATION_CONSTRAINT_TYPE_KEY)
        .collect();
    assert_eq!(translation.len(), 1, "expected 1 TranslationConstraint");

    let scale: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == SCALE_CONSTRAINT_TYPE_KEY)
        .collect();
    assert_eq!(scale.len(), 1, "expected 1 ScaleConstraint");

    let rotation: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == ROTATION_CONSTRAINT_TYPE_KEY)
        .collect();
    assert_eq!(rotation.len(), 1, "expected 1 RotationConstraint");

    cleanup(&output);
}

const TEXT_TYPE_KEY: u64 = 134;
const TEXT_STYLE_TYPE_KEY: u64 = 573;
const TEXT_VALUE_RUN_TYPE_KEY: u64 = 135;
const IMAGE_TYPE_KEY: u64 = 100;
const IMAGE_ASSET_TYPE_KEY: u64 = 105;
const FONT_ASSET_TYPE_KEY: u64 = 141;
const AUDIO_ASSET_TYPE_KEY: u64 = 406;
const LAYOUT_COMPONENT_TYPE_KEY: u64 = 409;
const LAYOUT_COMPONENT_STYLE_TYPE_KEY: u64 = 420;
const VIEW_MODEL_TYPE_KEY: u64 = 435;
const VIEW_MODEL_PROPERTY_TYPE_KEY: u64 = 430;
const DATA_BIND_TYPE_KEY: u64 = 446;

#[test]
fn test_generate_text() {
    let input = fixture_path("text.json");
    let output = temp_output("text");
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
fn test_validate_text() {
    let input = fixture_path("text.json");
    let output = temp_output("validate_text");
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
fn test_inspect_text() {
    let input = fixture_path("text.json");
    let output = temp_output("inspect_text");
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
        stdout.contains("TextDemo"),
        "expected artboard name 'TextDemo' in output, got: {}",
        stdout
    );
    cleanup(&output);
}

#[test]
fn test_inspect_json_text() {
    let input = fixture_path("text.json");
    let output = temp_output("inspect_json_text");
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

    let text: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == TEXT_TYPE_KEY)
        .collect();
    assert_eq!(text.len(), 1, "expected 1 Text");

    let text_styles: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == TEXT_STYLE_TYPE_KEY)
        .collect();
    assert_eq!(text_styles.len(), 1, "expected 1 TextStyle");

    let text_runs: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == TEXT_VALUE_RUN_TYPE_KEY)
        .collect();
    assert_eq!(text_runs.len(), 1, "expected 1 TextValueRun");

    cleanup(&output);
}

#[test]
fn test_generate_assets() {
    let input = fixture_path("assets.json");
    let output = temp_output("assets");
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
fn test_validate_assets() {
    let input = fixture_path("assets.json");
    let output = temp_output("validate_assets");
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
fn test_inspect_json_assets() {
    let input = fixture_path("assets.json");
    let output = temp_output("inspect_json_assets");
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

    let images: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == IMAGE_ASSET_TYPE_KEY)
        .collect();
    assert_eq!(images.len(), 1, "expected 1 ImageAsset");

    let fonts: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == FONT_ASSET_TYPE_KEY)
        .collect();
    assert_eq!(fonts.len(), 1, "expected 1 FontAsset");

    let audio: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == AUDIO_ASSET_TYPE_KEY)
        .collect();
    assert_eq!(audio.len(), 1, "expected 1 AudioAsset");

    cleanup(&output);
}

#[test]
fn test_generate_image_node() {
    let input = fixture_path("image_node.json");
    let output = temp_output("image_node");
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
fn test_validate_image_node() {
    let input = fixture_path("image_node.json");
    let output = temp_output("validate_image_node");
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
fn test_inspect_json_image_node() {
    let input = fixture_path("image_node.json");
    let output = temp_output("inspect_json_image_node");
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

    let images: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == IMAGE_TYPE_KEY)
        .collect();
    assert_eq!(images.len(), 1, "expected 1 Image");

    let image_assets: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == IMAGE_ASSET_TYPE_KEY)
        .collect();
    assert_eq!(image_assets.len(), 1, "expected 1 ImageAsset");

    cleanup(&output);
}

#[test]
fn test_generate_layout() {
    let input = fixture_path("layout.json");
    let output = temp_output("layout");
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
fn test_validate_layout() {
    let input = fixture_path("layout.json");
    let output = temp_output("validate_layout");
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
fn test_inspect_json_layout() {
    let input = fixture_path("layout.json");
    let output = temp_output("inspect_json_layout");
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

    let layout_components: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == LAYOUT_COMPONENT_TYPE_KEY)
        .collect();
    assert_eq!(layout_components.len(), 1, "expected 1 LayoutComponent");

    let layout_styles: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == LAYOUT_COMPONENT_STYLE_TYPE_KEY)
        .collect();
    assert_eq!(layout_styles.len(), 1, "expected 1 LayoutComponentStyle");

    cleanup(&output);
}

#[test]
fn test_generate_data_binding() {
    let input = fixture_path("data_binding.json");
    let output = temp_output("data_binding");
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
fn test_validate_data_binding() {
    let input = fixture_path("data_binding.json");
    let output = temp_output("validate_data_binding");
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
fn test_inspect_json_data_binding() {
    let input = fixture_path("data_binding.json");
    let output = temp_output("inspect_json_data_binding");
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

    let view_models: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == VIEW_MODEL_TYPE_KEY)
        .collect();
    assert_eq!(view_models.len(), 1, "expected 1 ViewModel");

    let view_model_props: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == VIEW_MODEL_PROPERTY_TYPE_KEY)
        .collect();
    assert_eq!(view_model_props.len(), 2, "expected 2 ViewModelProperty");

    let data_binds: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == DATA_BIND_TYPE_KEY)
        .collect();
    assert_eq!(data_binds.len(), 1, "expected 1 DataBind");

    cleanup(&output);
}

#[test]
fn test_generate_button_states() {
    let input = fixture_path("button_states.json");
    let output = temp_output("button_states");
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
fn test_validate_button_states() {
    let input = fixture_path("button_states.json");
    let output = temp_output("button_states_validate");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let v = cargo_run(&["validate", output.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&v.stdout);
    assert!(v.status.success());
    assert!(stdout.contains("valid"));
    assert!(stdout.contains("90 objects"));
    cleanup(&output);
}

#[test]
fn test_inspect_button_states() {
    let input = fixture_path("button_states.json");
    let output = temp_output("button_states_inspect");
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
    assert!(insp.status.success());
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("inspect --json output is not valid JSON");
    let objects = parsed.get("objects").unwrap().as_array().unwrap();
    assert_eq!(objects.len(), 90);

    let state_machines: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == 53)
        .collect();
    assert_eq!(state_machines.len(), 1, "expected 1 StateMachine");

    let animations: Vec<_> = objects
        .iter()
        .filter(|o| o.get("type_key").unwrap().as_u64().unwrap() == 31)
        .collect();
    assert_eq!(animations.len(), 4, "expected 4 LinearAnimations");

    cleanup(&output);
}

#[test]
fn test_generate_loader() {
    let input = fixture_path("loader.json");
    let output = temp_output("loader");
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
fn test_validate_loader() {
    let input = fixture_path("loader.json");
    let output = temp_output("loader_validate");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let v = cargo_run(&["validate", output.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&v.stdout);
    assert!(v.status.success());
    assert!(stdout.contains("valid"));
    assert!(stdout.contains("27 objects"));
    cleanup(&output);
}

#[test]
fn test_inspect_loader() {
    let input = fixture_path("loader.json");
    let output = temp_output("loader_inspect");
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
    assert!(insp.status.success());
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("inspect --json output is not valid JSON");
    let objects = parsed.get("objects").unwrap().as_array().unwrap();
    assert_eq!(objects.len(), 27);
    cleanup(&output);
}
