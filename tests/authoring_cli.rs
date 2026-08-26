use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::{compile, validator};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn rive_cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rive-cli"))
}

fn cargo_run(args: &[&str]) -> std::process::Output {
    rive_cli()
        .args(args)
        .output()
        .expect("failed to run rive-cli binary")
}

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("authoring")
        .join("typed-motion.v0.json")
}

fn temp_path(test_name: &str, extension: &str) -> PathBuf {
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "rive_authoring_cli_{test_name}_{}_{}.{}",
        std::process::id(),
        counter,
        extension
    ))
}

struct CleanupOnDrop(PathBuf);

impl Drop for CleanupOnDrop {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn typed_motion_fixture_compiles_through_shared_seam() {
    let fixture = fixture_path();
    let input = std::fs::read_to_string(&fixture).expect("typed motion fixture must exist");
    let first = lower_authoring_json(&input).expect("first lowering must succeed");
    let second = lower_authoring_json(&input).expect("second lowering must succeed");

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let scene: SceneSpec =
        serde_json::from_value(first.scene).expect("lowered SceneSpec must deserialize");
    let bytes = compile::compile_scene(&scene, fixture.parent(), 7)
        .expect("shared compilation seam must accept the lowered scene");
    let report = validator::validate_riv(&bytes).expect("compiled bytes must parse");

    assert!(report.valid, "compiled fixture errors: {:?}", report.errors);
    assert_eq!(report.header.file_id, 7);
    assert!(report.object_count > 10);
}

#[test]
fn authoring_compile_is_a_public_end_to_end_command() {
    let fixture = fixture_path();
    let output = temp_path("compile", "riv");
    let _output_guard = CleanupOnDrop(output.clone());

    let result = cargo_run(&[
        "authoring",
        "compile",
        fixture.to_str().expect("UTF-8 fixture path"),
        "-o",
        output.to_str().expect("UTF-8 output path"),
        "--file-id",
        "42",
        "--json",
    ]);
    assert!(
        result.status.success(),
        "authoring compile failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );

    let payload: serde_json::Value =
        serde_json::from_slice(&result.stdout).expect("compile output must be JSON");
    assert_eq!(payload["ok"].as_bool(), Some(true));
    assert_eq!(
        payload["output_path"].as_str(),
        Some(output.to_str().expect("UTF-8 output path"))
    );
    assert!(payload["bytes_written"].as_u64().is_some_and(|size| size > 4));
    assert!(
        payload["source_map"]["entries"]
            .as_array()
            .is_some_and(|entries| entries.len() >= 4)
    );

    let bytes = std::fs::read(&output).expect("authoring compile must write output");
    assert_eq!(&bytes[..4], b"RIVE");

    let validate = cargo_run(&[
        "validate",
        output.to_str().expect("UTF-8 output path"),
        "--json",
    ]);
    assert!(
        validate.status.success(),
        "validate failed: {}",
        String::from_utf8_lossy(&validate.stderr)
    );
    let validation: serde_json::Value =
        serde_json::from_slice(&validate.stdout).expect("validate output must be JSON");
    assert_eq!(validation["ok"].as_bool(), Some(true));
    assert_eq!(validation["valid"].as_bool(), Some(true));
    assert_eq!(validation["header"]["file_id"].as_u64(), Some(42));

    let inspect = cargo_run(&[
        "inspect",
        output.to_str().expect("UTF-8 output path"),
        "--json",
    ]);
    assert!(
        inspect.status.success(),
        "inspect failed: {}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    let inspection: serde_json::Value =
        serde_json::from_slice(&inspect.stdout).expect("inspect output must be JSON");
    assert!(
        inspection["objects"]
            .as_array()
            .is_some_and(|objects| !objects.is_empty())
    );
}

#[test]
fn authoring_schema_is_discoverable_from_the_cli() {
    let result = cargo_run(&["authoring", "schema", "--compact"]);
    assert!(
        result.status.success(),
        "authoring schema failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );

    let schema: serde_json::Value =
        serde_json::from_slice(&result.stdout).expect("schema output must be JSON");
    assert_eq!(
        schema["$id"].as_str(),
        Some("https://github.com/George-RD/rive-rs-cli/docs/authoring.schema.v0.json")
    );
    assert_eq!(schema["title"].as_str(), Some("rive-cli AuthoringSpec v0"));
    assert!(schema["properties"]["authoring_format_version"].is_object());
}

#[test]
fn authoring_json_failures_preserve_the_full_diagnostic_set() {
    let input = temp_path("invalid", "json");
    let output = temp_path("invalid", "riv");
    let _input_guard = CleanupOnDrop(input.clone());
    let _output_guard = CleanupOnDrop(output.clone());
    std::fs::write(
        &input,
        r#"{
          "authoring_format_version": 0,
          "artboard": {
            "id": "bad/id",
            "width": { "value": 100.0, "unit": "px" },
            "height": { "value": 100.0, "unit": "px" }
          },
          "image_assets": { "bad.name": "" },
          "visual": {
            "nodes": [
              { "kind": "group", "id": "child/id", "children": [] }
            ]
          },
          "motion": {},
          "behavior": {}
        }"#,
    )
    .expect("write invalid AuthoringSpec");

    let result = cargo_run(&[
        "--json",
        "authoring",
        "compile",
        input.to_str().expect("UTF-8 input path"),
        "-o",
        output.to_str().expect("UTF-8 output path"),
    ]);
    assert!(!result.status.success());
    assert!(!output.exists());

    let payload: serde_json::Value =
        serde_json::from_slice(&result.stderr).expect("error output must be JSON");
    assert_eq!(payload["ok"].as_bool(), Some(false));
    assert_eq!(payload["command"].as_str(), Some("authoring"));
    assert_eq!(payload["code"].as_str(), Some("lowering-failed"));
    let diagnostics = payload["diagnostics"]
        .as_array()
        .expect("diagnostics must be an array");
    assert!(diagnostics.len() >= 4);
    assert!(diagnostics.iter().all(|diagnostic| {
        diagnostic["path"].as_str().is_some_and(|value| !value.is_empty())
            && diagnostic["code"]
                .as_str()
                .is_some_and(|value| !value.is_empty())
            && diagnostic["message"]
                .as_str()
                .is_some_and(|value| !value.is_empty())
    }));
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic["path"].as_str() == Some("$.artboard.id")
            && diagnostic["code"].as_str() == Some("invalid_id")
    }));
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic["path"].as_str() == Some("$.image_assets")
            && diagnostic["code"].as_str() == Some("invalid_asset_id")
    }));
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic["path"].as_str() == Some("$.image_assets.bad.name")
            && diagnostic["code"].as_str() == Some("invalid_asset_source")
    }));
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic["path"].as_str() == Some("$.visual.nodes[0].id")
            && diagnostic["code"].as_str() == Some("invalid_id")
    }));
}

#[test]
fn authoring_usage_errors_honor_json_mode_before_clap_dispatch() {
    let result = cargo_run(&["authoring", "compile", "--json"]);
    assert!(!result.status.success());

    let payload: serde_json::Value =
        serde_json::from_slice(&result.stderr).expect("usage error must be JSON");
    assert_eq!(payload["ok"].as_bool(), Some(false));
    assert_eq!(payload["command"].as_str(), Some("authoring"));
    assert_eq!(payload["code"].as_str(), Some("usage"));
    assert!(payload["message"].as_str().is_some_and(|message| {
        message.contains("required arguments were not provided")
    }));
}
