use std::path::PathBuf;
use std::process::Command;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("authoring")
        .join("typed-motion.v0.json")
}

fn temp_path(name: &str, extension: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "rive_authoring_cli_{name}_{}.{}",
        std::process::id(),
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
fn final_builder_failures_retain_authored_diagnostics() {
    let input = temp_path("missing_asset", "json");
    let output = temp_path("missing_asset", "riv");
    let _input_guard = CleanupOnDrop(input.clone());
    let _output_guard = CleanupOnDrop(output.clone());

    let fixture = std::fs::read_to_string(fixture_path()).expect("fixture must exist");
    let mut spec: serde_json::Value =
        serde_json::from_str(&fixture).expect("fixture must be valid JSON");
    spec["image_assets"]["aurora"] = serde_json::Value::String("missing-aurora.png".to_string());
    std::fs::write(
        &input,
        serde_json::to_vec_pretty(&spec).expect("serialize invalid asset fixture"),
    )
    .expect("write invalid asset fixture");

    let result = Command::new(env!("CARGO_BIN_EXE_rive-cli"))
        .args([
            "authoring",
            "compile",
            input.to_str().expect("UTF-8 input path"),
            "-o",
            output.to_str().expect("UTF-8 output path"),
            "--json",
        ])
        .output()
        .expect("run rive-cli");

    assert!(!result.status.success());
    assert!(!output.exists());

    let payload: serde_json::Value =
        serde_json::from_slice(&result.stderr).expect("error output must be JSON");
    assert_eq!(payload["ok"].as_bool(), Some(false));
    assert_eq!(payload["command"].as_str(), Some("authoring"));
    assert_eq!(payload["code"].as_str(), Some("invalid-scene"));

    let diagnostics = payload["diagnostics"]
        .as_array()
        .expect("final builder failure must retain Authoring diagnostics");
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0]["path"].as_str(), Some("$.lowered_scene"));
    assert_eq!(diagnostics[0]["code"].as_str(), Some("invalid-scene"));
    assert!(
        diagnostics[0]["message"]
            .as_str()
            .is_some_and(|message| message.contains("missing-aurora.png"))
    );
}
