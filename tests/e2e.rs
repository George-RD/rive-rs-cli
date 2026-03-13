use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

const ARTBOARD_TYPE_KEY: u64 = 1;
const BACKBOARD_TYPE_KEY: u64 = 23;
const COMPONENT_NAME_KEY: u64 = 4;
const ARTBOARD_WIDTH_KEY: u64 = 7;
const ARTBOARD_HEIGHT_KEY: u64 = 8;

static TEMP_OUTPUT_COUNTER: AtomicU64 = AtomicU64::new(0);

fn rive_cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rive-cli"))
}

fn cargo_run(args: &[&str]) -> std::process::Output {
    rive_cli()
        .args(args)
        .output()
        .expect("failed to run rive-cli binary")
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn temp_output(test_name: &str) -> PathBuf {
    let counter = TEMP_OUTPUT_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "rive_e2e_{}_{}_{}.riv",
        test_name,
        std::process::id(),
        counter
    ))
}

fn cleanup(path: &PathBuf) {
    let _ = std::fs::remove_file(path);
}

struct CleanupOnDrop(PathBuf);

impl Drop for CleanupOnDrop {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn generate_and_validate_output(fixture: &str, suffix: &str) -> (PathBuf, CleanupOnDrop) {
    let input = fixture_path(&format!("{}.json", fixture));
    let output = temp_output(&format!("{}_{}", fixture, suffix));
    cleanup(&output);
    let guard = CleanupOnDrop(output.clone());

    let generate = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        generate.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&generate.stderr)
    );

    let validate = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        validate.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&validate.stderr)
    );

    (output, guard)
}

fn assert_generate_validate_inspect(fixture: &str, expected: &[&str]) {
    let (output, _guard) = generate_and_validate_output(fixture, "inspect");

    let inspect = cargo_run(&["inspect", output.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&inspect.stdout);
    assert!(
        inspect.status.success(),
        "inspect failed: {}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    for s in expected {
        assert!(stdout.contains(s), "expected '{}' in inspect output", s);
    }
}

fn generate_and_inspect_json(fixture: &str) -> serde_json::Value {
    let (output, _guard) = generate_and_validate_output(fixture, "json");

    let inspect = cargo_run(&["inspect", "--json", output.to_str().unwrap()]);
    assert!(
        inspect.status.success(),
        "inspect --json failed: {}",
        String::from_utf8_lossy(&inspect.stderr)
    );

    serde_json::from_slice(&inspect.stdout).expect("inspect --json output is not valid JSON")
}

fn json_objects(parsed: &serde_json::Value) -> &[serde_json::Value] {
    parsed["objects"]
        .as_array()
        .expect("objects should be an array")
}

fn json_properties(object: &serde_json::Value) -> &[serde_json::Value] {
    object["properties"]
        .as_array()
        .expect("properties should be an array")
}

fn find_object_by_type<'a>(
    objects: &'a [serde_json::Value],
    type_name: &str,
) -> &'a serde_json::Value {
    objects
        .iter()
        .find(|object| object["type_name"] == type_name)
        .unwrap_or_else(|| panic!("missing object type {}", type_name))
}

fn find_objects_by_type<'a>(
    objects: &'a [serde_json::Value],
    type_name: &str,
) -> Vec<&'a serde_json::Value> {
    objects
        .iter()
        .filter(|object| object["type_name"] == type_name)
        .collect()
}

fn find_property<'a>(object: &'a serde_json::Value, property_name: &str) -> &'a serde_json::Value {
    json_properties(object)
        .iter()
        .find(|property| property["name"] == property_name)
        .unwrap_or_else(|| panic!("missing property {}", property_name))
}

fn uint_property(object: &serde_json::Value, property_name: &str) -> u64 {
    find_property(object, property_name)["value"]["UInt"]
        .as_u64()
        .unwrap_or_else(|| panic!("{} should be a UInt", property_name))
}

fn float_property(object: &serde_json::Value, property_name: &str) -> f64 {
    find_property(object, property_name)["value"]["Float"]
        .as_f64()
        .unwrap_or_else(|| panic!("{} should be a Float", property_name))
}

fn string_property<'a>(object: &'a serde_json::Value, property_name: &str) -> &'a str {
    find_property(object, property_name)["value"]["String"]
        .as_str()
        .unwrap_or_else(|| panic!("{} should be a String", property_name))
}

fn color_property(object: &serde_json::Value, property_name: &str) -> u64 {
    find_property(object, property_name)["value"]["Color"]
        .as_u64()
        .unwrap_or_else(|| panic!("{} should be a Color", property_name))
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
fn test_generate_validate_solo_test() {
    let input = fixture_path("solo_test.json");
    let output = temp_output("solo_test");
    cleanup(&output);

    let generate = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        generate.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&generate.stderr)
    );

    let validate = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        validate.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&validate.stderr)
    );

    let inspect = cargo_run(&["inspect", output.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&inspect.stdout);
    assert!(
        inspect.status.success(),
        "inspect failed: {}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    assert!(
        stdout.contains("Solo"),
        "expected 'Solo' in inspect output, got: {}",
        stdout
    );

    cleanup(&output);
}

#[test]
fn test_generate_validate_listener_test() {
    let input = fixture_path("listener_test.json");
    let output = temp_output("listener_test");
    cleanup(&output);

    let generate = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        generate.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&generate.stderr)
    );

    let validate = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        validate.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&validate.stderr)
    );

    let inspect = cargo_run(&["inspect", output.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&inspect.stdout);
    assert!(
        inspect.status.success(),
        "inspect failed: {}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    assert!(
        stdout.contains("StateMachineListener"),
        "expected 'StateMachineListener' in inspect output, got: {}",
        stdout
    );
    assert!(
        stdout.contains("ListenerBoolChange"),
        "expected 'ListenerBoolChange' in inspect output, got: {}",
        stdout
    );
    assert!(
        stdout.contains("ListenerTriggerChange"),
        "expected 'ListenerTriggerChange' in inspect output, got: {}",
        stdout
    );
    assert!(
        stdout.contains("ListenerNumberChange"),
        "expected 'ListenerNumberChange' in inspect output, got: {}",
        stdout
    );

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
fn test_generate_validate_points_path() {
    let input = fixture_path("points_path.json");
    let output = temp_output("points_path");
    cleanup(&output);

    let generate = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        generate.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&generate.stderr)
    );
    assert!(output.exists());

    let validate = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        validate.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&validate.stderr)
    );
    let validate_stdout = String::from_utf8_lossy(&validate.stdout);
    assert!(
        validate_stdout.contains("valid"),
        "expected 'valid' in validate stdout, got: {}",
        validate_stdout
    );

    let inspect = cargo_run(&["inspect", output.to_str().unwrap()]);
    assert!(
        inspect.status.success(),
        "inspect failed: {}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    let inspect_stdout = String::from_utf8_lossy(&inspect.stdout);
    assert!(
        inspect_stdout.contains("PointsPath") || inspect_stdout.contains("Points Path"),
        "expected PointsPath object in inspect output, got: {}",
        inspect_stdout
    );
    assert!(
        inspect_stdout.contains("StraightVertex") || inspect_stdout.contains("Straight Vertex"),
        "expected StraightVertex object in inspect output, got: {}",
        inspect_stdout
    );

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
    let objects = parsed
        .get("objects")
        .and_then(|v| v.as_array())
        .expect("objects array missing");
    assert!(
        objects
            .iter()
            .any(|o| o.get("type_name").and_then(|v| v.as_str()).is_some()),
        "expected at least one resolved type_name"
    );
    let artboard = objects
        .iter()
        .find(|o| o.get("type_key").and_then(|v| v.as_u64()) == Some(ARTBOARD_TYPE_KEY))
        .expect("artboard object missing");
    let properties = artboard
        .get("properties")
        .and_then(|v| v.as_array())
        .expect("properties array missing");
    assert!(
        properties
            .iter()
            .any(|p| p.get("name").and_then(|v| v.as_str()) == Some("name")),
        "expected resolved property name for component name"
    );
    cleanup(&output);
}

#[test]
fn test_decompile_outputs_resolved_names_json() {
    let input = fixture_path("minimal.json");
    let output = temp_output("decompile_minimal");
    cleanup(&output);

    let g = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(g.status.success());

    let decompile = cargo_run(&["decompile", output.to_str().unwrap()]);
    assert!(
        decompile.status.success(),
        "decompile failed: {}",
        String::from_utf8_lossy(&decompile.stderr)
    );

    let parsed: serde_json::Value =
        serde_json::from_slice(&decompile.stdout).expect("decompile output is not valid JSON");
    let objects = parsed
        .get("objects")
        .and_then(|v| v.as_array())
        .expect("objects array missing");

    assert!(
        objects
            .iter()
            .any(|o| o.get("type_name").and_then(|v| v.as_str()) == Some("Artboard"))
    );
    let artboard = objects
        .iter()
        .find(|o| o.get("type_key").and_then(|v| v.as_u64()) == Some(ARTBOARD_TYPE_KEY))
        .expect("artboard object missing");
    let properties = artboard
        .get("properties")
        .and_then(|v| v.as_array())
        .expect("properties array missing");
    assert!(
        properties
            .iter()
            .any(|p| p.get("name").and_then(|v| v.as_str()) == Some("name"))
    );

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
fn test_inspect_filter_artboard_name_and_local_index_json() {
    let (output, _guard) =
        generate_and_validate_output("multi_artboard", "inspect_filter_artboard_name_local_index");

    let insp = cargo_run(&[
        "inspect",
        "--json",
        "--artboard-name",
        "screen b",
        "--local-index",
        "2",
        output.to_str().unwrap(),
    ]);
    assert!(
        insp.status.success(),
        "inspect --json artboard filters failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );

    let parsed: serde_json::Value =
        serde_json::from_slice(&insp.stdout).expect("inspect output is not valid JSON");
    let objects = parsed
        .get("objects")
        .and_then(|value| value.as_array())
        .expect("objects array missing");
    assert_eq!(objects.len(), 1);
    assert_eq!(
        objects[0]
            .get("object_index")
            .and_then(|value| value.as_u64()),
        Some(13)
    );
    assert_eq!(
        objects[0]
            .get("artboard_index")
            .and_then(|value| value.as_u64()),
        Some(1)
    );
    assert_eq!(
        objects[0]
            .get("artboard_name")
            .and_then(|value| value.as_str()),
        Some("Screen B")
    );
    assert_eq!(
        objects[0]
            .get("local_index")
            .and_then(|value| value.as_u64()),
        Some(2)
    );
    assert_eq!(
        objects[0].get("type_key").and_then(|value| value.as_u64()),
        Some(7)
    );
}

#[test]
fn test_inspect_filter_artboard_index_human_output() {
    let (output, _guard) =
        generate_and_validate_output("multi_artboard", "inspect_filter_artboard_index_human");

    let insp = cargo_run(&[
        "inspect",
        "--artboard-index",
        "1",
        "--local-index",
        "2",
        output.to_str().unwrap(),
    ]);
    let stdout = String::from_utf8_lossy(&insp.stdout);
    assert!(
        insp.status.success(),
        "inspect artboard filters failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );
    assert!(
        stdout.contains("Artboard 1 (Screen B)"),
        "expected Screen B context in output, got: {}",
        stdout
    );
    assert!(
        stdout.contains("[13:2] type=7 (Rectangle)"),
        "expected filtered rectangle in output, got: {}",
        stdout
    );
    assert!(
        !stdout.contains("Screen A"),
        "did not expect Screen A in filtered output, got: {}",
        stdout
    );
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
        stderr.contains("--prompt") || stderr.contains("--template"),
        "stderr was: {}",
        stderr
    );
}

#[test]
fn test_ai_generate_prompt_without_api_key() {
    let result = rive_cli()
        .args(["ai", "generate", "--prompt", "make a bounce"])
        .env_remove("OPENAI_API_KEY")
        .output()
        .expect("failed to run rive-cli binary");
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
    let result = rive_cli()
        .args([
            "ai",
            "generate",
            "--template",
            "bounce",
            "--prompt",
            "make something",
        ])
        .env_remove("OPENAI_API_KEY")
        .output()
        .expect("failed to run rive-cli binary");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!result.status.success());
    assert!(
        stderr.contains("cannot be used with"),
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
    assert!(stdout.contains("20 objects"));
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
    assert_eq!(objects.len(), 20);
    cleanup(&output);
}

#[test]
fn test_generate_validate_inspect_elastic_interpolator() {
    assert_generate_validate_inspect("elastic_interpolator", &["ElasticInterpolator"]);
}

#[test]
fn test_generate_validate_inspect_triangle() {
    assert_generate_validate_inspect("triangle", &["Triangle"]);
}

#[test]
fn test_generate_validate_inspect_event_test() {
    assert_generate_validate_inspect("event_test", &["Event", "KeyFrameCallback"]);
}

#[test]
fn test_generate_validate_inspect_nested_simple_animation() {
    assert_generate_validate_inspect("nested_simple_animation", &["NestedSimpleAnimation"]);
}

#[test]
fn test_generate_validate_inspect_icon_set() {
    assert_generate_validate_inspect("icon_set", &["Home", "Settings", "Profile"]);
}

#[test]
fn test_generate_validate_inspect_game_hud() {
    assert_generate_validate_inspect("game_hud", &["HealthBar", "ScoreText", "MiniMapFrame"]);
}

#[test]
fn test_generate_validate_inspect_mascot() {
    assert_generate_validate_inspect("mascot", &["Spine", "Torso", "Neck"]);
}

#[test]
fn test_generate_validate_inspect_polygon_star() {
    assert_generate_validate_inspect("polygon_star", &["Polygon", "Star"]);
}

#[test]
fn test_generate_validate_inspect_clipping_shape() {
    assert_generate_validate_inspect("clipping_shape", &["ClippingShape"]);
}

#[test]
fn test_generate_validate_inspect_follow_path_constraint() {
    assert_generate_validate_inspect("follow_path_constraint", &["FollowPathConstraint"]);
}

#[test]
fn test_generate_validate_inspect_cubic_asymmetric() {
    assert_generate_validate_inspect("cubic_asymmetric", &["CubicAsymmetricVertex"]);
}

#[test]
fn test_generate_validate_inspect_draw_rules() {
    assert_generate_validate_inspect("draw_rules", &["DrawTarget", "DrawRules"]);
}

#[test]
fn test_generate_validate_inspect_joystick() {
    assert_generate_validate_inspect("joystick", &["Joystick"]);
}

#[test]
fn test_generate_validate_inspect_blend_animation() {
    let parsed = generate_and_inspect_json("blend_animation");
    let objects = json_objects(&parsed);

    find_object_by_type(objects, "BlendState1D");
    let blend_state_input = find_object_by_type(objects, "BlendState1DInput");
    assert_eq!(uint_property(blend_state_input, "inputId"), 0);

    let blend_animations_1d = find_objects_by_type(objects, "BlendAnimation1D");
    assert_eq!(blend_animations_1d.len(), 2);
    assert_eq!(uint_property(blend_animations_1d[0], "animationId"), 0);
    assert!(
        json_properties(blend_animations_1d[0])
            .iter()
            .all(|property| property["name"] != "value")
    );
    assert_eq!(uint_property(blend_animations_1d[1], "animationId"), 1);
    assert_eq!(float_property(blend_animations_1d[1], "value"), 1.0);

    find_object_by_type(objects, "BlendStateDirect");
    let blend_animation_direct = find_object_by_type(objects, "BlendAnimationDirect");
    assert_eq!(uint_property(blend_animation_direct, "animationId"), 1);
    assert_eq!(uint_property(blend_animation_direct, "inputId"), 0);
    assert_eq!(float_property(blend_animation_direct, "mixValue"), 0.5);
    assert_eq!(uint_property(blend_animation_direct, "blendSource"), 1);
}

#[test]
fn test_generate_validate_inspect_transition_comparators() {
    let parsed = generate_and_inspect_json("transition_comparators");
    let objects = json_objects(&parsed);

    let bool_comparator = find_object_by_type(objects, "TransitionValueBooleanComparator");
    assert_eq!(uint_property(bool_comparator, "value"), 1);

    let number_comparator = find_object_by_type(objects, "TransitionValueNumberComparator");
    assert_eq!(float_property(number_comparator, "value"), 1.0);

    let string_comparator = find_object_by_type(objects, "TransitionValueStringComparator");
    assert_eq!(string_property(string_comparator, "value"), "on");

    let color_comparator = find_object_by_type(objects, "TransitionValueColorComparator");
    assert_eq!(color_property(color_comparator, "value"), 0xFFFF0000);

    let condition_ops: Vec<u64> = find_objects_by_type(objects, "TransitionBoolCondition")
        .into_iter()
        .map(|object| uint_property(object, "opValue"))
        .collect();
    assert_eq!(condition_ops, vec![0, 1]);
}

#[test]
fn test_generate_validate_inspect_view_model_instances() {
    let parsed = generate_and_inspect_json("view_model_instances");
    let objects = json_objects(&parsed);

    let view_model = find_object_by_type(objects, "ViewModel");
    assert_eq!(string_property(view_model, "name"), "ProfileModel");

    let properties = find_objects_by_type(objects, "ViewModelProperty");
    assert_eq!(properties.len(), 4);
    assert_eq!(string_property(properties[0], "name"), "Username");
    assert_eq!(uint_property(properties[0], "symbolTypeValue"), 1);
    assert_eq!(string_property(properties[3], "name"), "ThemeColor");
    assert_eq!(uint_property(properties[3], "symbolTypeValue"), 4);

    let instance = find_object_by_type(objects, "ViewModelInstance");
    assert_eq!(uint_property(instance, "viewModelId"), 1);
    assert_eq!(
        string_property(
            find_object_by_type(objects, "ViewModelInstanceString"),
            "propertyValue"
        ),
        "Alice"
    );
    assert_eq!(
        float_property(
            find_object_by_type(objects, "ViewModelInstanceNumber"),
            "propertyValue"
        ),
        42.5
    );
    assert_eq!(
        uint_property(
            find_object_by_type(objects, "ViewModelInstanceBoolean"),
            "propertyValue"
        ),
        1
    );
    assert_eq!(
        color_property(
            find_object_by_type(objects, "ViewModelInstanceColor"),
            "propertyValue"
        ),
        0xFF3366FF
    );
    assert_eq!(
        uint_property(
            find_object_by_type(objects, "ViewModelInstanceEnum"),
            "propertyValue"
        ),
        2
    );
    assert_eq!(
        uint_property(
            find_object_by_type(objects, "ViewModelInstanceValue"),
            "viewModelPropertyId"
        ),
        6
    );
    find_object_by_type(objects, "ViewModelInstanceList");
    let list_item = find_object_by_type(objects, "ViewModelInstanceListItem");
    assert_eq!(uint_property(list_item, "viewModelId"), 1);
    assert_eq!(uint_property(list_item, "viewModelInstanceId"), 2);
    let view_model_value = find_object_by_type(objects, "ViewModelInstanceViewModel");
    assert_eq!(uint_property(view_model_value, "viewModelPropertyId"), 7);
    assert_eq!(uint_property(view_model_value, "propertyValue"), 3);
}

#[test]
fn test_generate_validate_inspect_keyframe_types() {
    let parsed = generate_and_inspect_json("keyframe_types");
    let objects = json_objects(&parsed);

    let bool_frames = find_objects_by_type(objects, "KeyFrameBool");
    assert_eq!(bool_frames.len(), 3);
    assert_eq!(uint_property(bool_frames[0], "frame"), 0);
    assert_eq!(uint_property(bool_frames[0], "value"), 1);
    assert_eq!(uint_property(bool_frames[1], "frame"), 30);
    assert_eq!(uint_property(bool_frames[1], "value"), 0);
    assert_eq!(uint_property(bool_frames[2], "frame"), 59);
    assert_eq!(uint_property(bool_frames[2], "value"), 1);

    let string_frames = find_objects_by_type(objects, "KeyFrameString");
    assert_eq!(string_frames.len(), 3);
    assert_eq!(uint_property(string_frames[0], "frame"), 0);
    assert_eq!(string_property(string_frames[0], "value"), "Hello");
    assert_eq!(uint_property(string_frames[1], "frame"), 30);
    assert_eq!(string_property(string_frames[1], "value"), "World");
    assert_eq!(uint_property(string_frames[2], "frame"), 59);
    assert_eq!(string_property(string_frames[2], "value"), "Done");
}

#[test]
fn test_generate_validate_inspect_text_modifiers() {
    let parsed = generate_and_inspect_json("text_modifiers");
    let objects = json_objects(&parsed);

    let text_style = find_object_by_type(objects, "TextStyle");
    assert_eq!(string_property(text_style, "name"), "BaseStyle");
    assert_eq!(float_property(text_style, "fontSize"), 32.0);

    let style_feature = find_object_by_type(objects, "TextStyleFeature");
    assert_eq!(uint_property(style_feature, "parentId"), 2);
    assert_eq!(uint_property(style_feature, "tag"), 1818847073);
    assert_eq!(uint_property(style_feature, "featureValue"), 1);

    let modifier_group = find_object_by_type(objects, "TextModifierGroup");
    assert_eq!(string_property(modifier_group, "name"), "WaveEffect");
    assert_eq!(uint_property(modifier_group, "modifierFlags"), 1);
    assert_eq!(float_property(modifier_group, "y"), 10.0);

    let modifier_range = find_object_by_type(objects, "TextModifierRange");
    assert_eq!(uint_property(modifier_range, "parentId"), 5);
    assert_eq!(float_property(modifier_range, "falloffTo"), 1.0);

    let variation = find_object_by_type(objects, "TextVariationModifier");
    assert_eq!(uint_property(variation, "parentId"), 5);
    assert_eq!(uint_property(variation, "axisTag"), 2003265652);
    assert_eq!(float_property(variation, "axisValue"), 700.0);
}

#[test]
fn test_validate_json_output() {
    let (output, _guard) = generate_and_validate_output("minimal", "validate_json");
    let result = cargo_run(&["validate", "--json", output.to_str().unwrap()]);
    assert!(
        result.status.success(),
        "validate --json failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let json: serde_json::Value =
        serde_json::from_slice(&result.stdout).expect("validate --json should output valid JSON");
    assert!(json["valid"].as_bool().unwrap(), "report should be valid");
    assert!(
        json["object_count"].as_u64().unwrap() > 0,
        "should have objects"
    );
    assert!(
        json["header"]["major_version"].as_u64().is_some(),
        "should have header version"
    );
}

#[test]
fn test_generate_json_output() {
    let input = fixture_path("minimal.json");
    let output = temp_output("generate_json");
    cleanup(&output);
    let _guard = CleanupOnDrop(output.clone());
    let result = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
        "--json",
    ]);
    assert!(
        result.status.success(),
        "generate --json failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let json: serde_json::Value =
        serde_json::from_slice(&result.stdout).expect("generate --json should output valid JSON");
    assert!(
        json["bytes_written"].as_u64().unwrap() > 0,
        "should have bytes_written"
    );
    assert!(
        json["output_path"].as_str().is_some(),
        "should have output_path"
    );
}

#[test]
fn test_list_presets_json() {
    let result = cargo_run(&["--list-presets", "--json"]);
    assert!(
        result.status.success(),
        "--list-presets --json failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&result.stdout)
        .expect("--list-presets --json should output valid JSON");
    let presets = json.as_array().expect("should be an array");
    assert!(!presets.is_empty(), "should have presets");
    assert!(
        presets[0]["name"].as_str().is_some(),
        "preset should have name"
    );
    assert!(
        presets[0]["width"].as_f64().is_some(),
        "preset should have width"
    );
    assert!(
        presets[0]["height"].as_f64().is_some(),
        "preset should have height"
    );
}

#[test]
fn test_version_flag() {
    let output = cargo_run(&["--version"]);
    assert!(output.status.success(), "--version failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.trim(),
        format!("rive-cli {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn test_generate_missing_input_file() {
    let result = cargo_run(&["generate", "/tmp/nonexistent_rive_test_file.json"]);
    assert!(!result.status.success(), "should fail on missing input");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("error"),
        "stderr should contain error message"
    );
}

#[test]
fn test_generate_malformed_json() {
    let bad_json = temp_output("malformed").with_extension("json");
    std::fs::write(&bad_json, "{ this is not valid json }").unwrap();
    let _guard = CleanupOnDrop(bad_json.clone());
    let result = cargo_run(&["generate", bad_json.to_str().unwrap()]);
    assert!(!result.status.success(), "should fail on malformed JSON");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("error"),
        "stderr should contain error message"
    );
}

#[test]
fn test_generate_invalid_scene_spec() {
    let bad_spec = temp_output("invalid_spec").with_extension("json");
    std::fs::write(&bad_spec, r#"{"valid": "json", "but": "not a scene spec"}"#).unwrap();
    let _guard = CleanupOnDrop(bad_spec.clone());
    let result = cargo_run(&["generate", bad_spec.to_str().unwrap()]);
    assert!(
        !result.status.success(),
        "should fail on invalid scene spec"
    );
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!stderr.is_empty(), "stderr should have error output");
}

#[test]
fn test_validate_missing_file() {
    let result = cargo_run(&["validate", "/tmp/nonexistent_rive_test.riv"]);
    assert!(!result.status.success(), "should fail on missing file");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("error"),
        "stderr should contain error message"
    );
}

#[test]
fn test_validate_truncated_file() {
    let truncated = temp_output("truncated");
    // Write just the RIVE header bytes but nothing else — truncated file
    std::fs::write(&truncated, b"RIVE").unwrap();
    let _guard = CleanupOnDrop(truncated.clone());
    let result = cargo_run(&["validate", truncated.to_str().unwrap()]);
    assert!(!result.status.success(), "should fail on truncated file");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!stderr.is_empty(), "stderr should have error output");
}

#[test]
fn test_inspect_missing_file() {
    let result = cargo_run(&["inspect", "/tmp/nonexistent_rive_test.riv"]);
    assert!(!result.status.success(), "should fail on missing file");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("error"),
        "stderr should contain error message"
    );
}

#[test]
fn test_decompile_missing_file() {
    let result = cargo_run(&["decompile", "/tmp/nonexistent_rive_test.riv"]);
    assert!(!result.status.success(), "should fail on missing file");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("error"),
        "stderr should contain error message"
    );
}

#[test]
fn test_decompile_corrupt_file() {
    let corrupt = temp_output("corrupt");
    std::fs::write(&corrupt, b"not a riv file at all").unwrap();
    let _guard = CleanupOnDrop(corrupt.clone());
    let result = cargo_run(&["decompile", corrupt.to_str().unwrap()]);
    assert!(!result.status.success(), "should fail on corrupt file");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!stderr.is_empty(), "stderr should have error output");
}

#[test]
fn test_generate_no_args() {
    let result = cargo_run(&["generate"]);
    assert!(!result.status.success(), "should fail with no input file");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(!stderr.is_empty(), "stderr should have usage/error output");
}

#[test]
fn test_inspect_nonexistent_type_key_graceful() {
    let (output, _guard) = generate_and_validate_output("minimal", "inspect_nokey");
    let result = cargo_run(&["inspect", "--type-key", "65535", output.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        result.status.success(),
        "should succeed even with no matching type key: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        stdout.contains("No objects matched the provided filters."),
        "output should report no matches when type key 65535 matches nothing, got: {}",
        stdout
    );
}

#[test]
fn test_generate_then_validate_then_inspect() {
    let input = fixture_path("minimal.json");
    let output = temp_output("full_pipeline");
    cleanup(&output);
    let _guard = CleanupOnDrop(output.clone());

    // Step 1: Generate
    let gen_out = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        gen_out.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&gen_out.stderr)
    );
    assert!(output.exists(), "output file should exist");

    // Step 2: Validate
    let val = cargo_run(&["validate", output.to_str().unwrap()]);
    assert!(
        val.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&val.stderr)
    );
    let val_stdout = String::from_utf8_lossy(&val.stdout);
    assert!(val_stdout.contains("valid"), "validate should report valid");

    // Step 3: Inspect
    let insp = cargo_run(&["inspect", output.to_str().unwrap()]);
    assert!(
        insp.status.success(),
        "inspect failed: {}",
        String::from_utf8_lossy(&insp.stderr)
    );
    let insp_stdout = String::from_utf8_lossy(&insp.stdout);
    assert!(
        insp_stdout.contains("Backboard"),
        "inspect should show Backboard"
    );
    assert!(
        insp_stdout.contains("Artboard"),
        "inspect should show Artboard"
    );
}

#[test]
fn test_generate_then_decompile_roundtrip() {
    let input = fixture_path("shapes.json");
    let output = temp_output("decompile_roundtrip");
    cleanup(&output);
    let _guard = CleanupOnDrop(output.clone());

    // Generate
    let gen_out = cargo_run(&[
        "generate",
        input.to_str().unwrap(),
        "-o",
        output.to_str().unwrap(),
    ]);
    assert!(
        gen_out.status.success(),
        "generate failed: {}",
        String::from_utf8_lossy(&gen_out.stderr)
    );

    // Decompile
    let dec = cargo_run(&["decompile", output.to_str().unwrap()]);
    assert!(
        dec.status.success(),
        "decompile failed: {}",
        String::from_utf8_lossy(&dec.stderr)
    );
    let json: serde_json::Value =
        serde_json::from_slice(&dec.stdout).expect("decompile output should be valid JSON");

    // Verify structure
    assert!(json["header"].is_object(), "should have header");
    assert!(json["objects"].is_array(), "should have objects array");
    let objects = json["objects"].as_array().unwrap();
    assert!(
        objects.len() >= 2,
        "should have at least Backboard + Artboard"
    );

    // First object should be Backboard (type_key 23)
    assert_eq!(
        objects[0]["type_key"].as_u64().unwrap(),
        23,
        "first object should be Backboard"
    );
    // Second object should be Artboard (type_key 1)
    assert_eq!(
        objects[1]["type_key"].as_u64().unwrap(),
        1,
        "second object should be Artboard"
    );
}

#[test]
fn test_validate_warns_on_version_mismatch() {
    let v8_riv = temp_output("v8_version_warning");
    cleanup(&v8_riv);
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"RIVE");
    bytes.push(8);
    bytes.push(0);
    bytes.push(0);
    bytes.push(0);
    bytes.push(23);
    bytes.push(0);
    bytes.push(1);
    bytes.push(0);
    std::fs::write(&v8_riv, &bytes).unwrap();
    let _guard = CleanupOnDrop(v8_riv.clone());

    let result = cargo_run(&["validate", v8_riv.to_str().unwrap()]);
    assert!(result.status.success(), "validate should succeed for v8");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("major version"),
        "should output major version warning: stderr={}",
        stderr
    );
}

#[test]
fn test_help_output() {
    let output = cargo_run(&["--help"]);
    assert!(output.status.success(), "--help failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("generate"),
        "help should mention generate command"
    );
    assert!(
        stdout.contains("validate"),
        "help should mention validate command"
    );
    assert!(
        stdout.contains("inspect"),
        "help should mention inspect command"
    );
    assert!(
        stdout.contains("decompile"),
        "help should mention decompile command"
    );
    assert!(stdout.contains("ai"), "help should mention ai command");
    assert!(
        stdout.contains("JSON scene spec"),
        "help should include generate description"
    );
    assert!(
        stdout.contains("AI-assisted"),
        "help should include ai description"
    );
}

#[test]
fn test_inspect_help() {
    let output = cargo_run(&["inspect", "--help"]);
    assert!(output.status.success(), "inspect --help failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("objects and properties"),
        "inspect help should include description"
    );
    assert!(
        stdout.contains("Examples:"),
        "inspect help should include examples"
    );
}

#[test]
fn test_generate_help() {
    let output = cargo_run(&["generate", "--help"]);
    assert!(output.status.success(), "generate --help failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("scene"),
        "generate help should mention scene input"
    );
    assert!(
        stdout.contains("Examples:"),
        "generate help should contain examples"
    );
}

#[test]
fn test_ai_help() {
    let output = cargo_run(&["ai", "--help"]);
    assert!(output.status.success(), "ai --help failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("generate"),
        "ai help should mention generate subcommand"
    );
    assert!(
        stdout.contains("lab"),
        "ai help should mention lab subcommand"
    );
    assert!(
        stdout.contains("evaluation suites"),
        "ai help should include lab description"
    );
}

#[test]
fn test_ai_generate_help() {
    let output = cargo_run(&["ai", "generate", "--help"]);
    assert!(output.status.success(), "ai generate --help failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("natural language prompt"),
        "ai generate help should include description"
    );
    assert!(
        stdout.contains("Examples:"),
        "ai generate help should include examples"
    );
}

#[test]
fn test_decompile_roundtrip_verify_objects() {
    let (output, _guard) = generate_and_validate_output("minimal", "decompile_rt");
    let dec = cargo_run(&["decompile", output.to_str().unwrap()]);
    assert!(
        dec.status.success(),
        "decompile failed: {}",
        String::from_utf8_lossy(&dec.stderr)
    );
    let json: serde_json::Value =
        serde_json::from_slice(&dec.stdout).expect("should be valid JSON");
    let objects = json["objects"].as_array().expect("should have objects");
    assert!(
        objects.len() >= 2,
        "should have at least 2 objects (Backboard + Artboard)"
    );
    assert_eq!(objects[0]["type_key"].as_u64().unwrap(), 23);
    assert_eq!(objects[1]["type_key"].as_u64().unwrap(), 1);
}

#[test]
fn test_multiple_fixtures_validate() {
    let fixtures = ["minimal", "shapes", "animation"];
    for fixture in &fixtures {
        let input = fixture_path(&format!("{}.json", fixture));
        assert!(
            input.exists(),
            "missing required fixture for this test: {}",
            input.display()
        );
        let output = temp_output(&format!("multi_{}", fixture));
        cleanup(&output);
        let _guard = CleanupOnDrop(output.clone());

        let gen_out = cargo_run(&[
            "generate",
            input.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ]);
        assert!(
            gen_out.status.success(),
            "generate {} failed: {}",
            fixture,
            String::from_utf8_lossy(&gen_out.stderr)
        );

        let val = cargo_run(&["validate", output.to_str().unwrap()]);
        assert!(
            val.status.success(),
            "validate {} failed: {}",
            fixture,
            String::from_utf8_lossy(&val.stderr)
        );
    }
}
