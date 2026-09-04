use std::path::PathBuf;
use std::process::Command;

const SHOWCASES: [&str; 2] = ["complex-animated-showcase", "interactive-console"];

fn temp_output(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "rive_showcase_artifact_{}_{name}.riv",
        std::process::id()
    ))
}

struct CleanupOnDrop(PathBuf);

impl Drop for CleanupOnDrop {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn committed_authoring_showcases_match_public_authoring_compile() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for name in SHOWCASES {
        let source = root.join(format!("examples/authoring/{name}.v0.json"));
        let committed = root.join(format!("examples/authoring/{name}.v0.riv"));
        let output = temp_output(name);
        let _guard = CleanupOnDrop(output.clone());

        let result = Command::new(env!("CARGO_BIN_EXE_rive-cli"))
            .args([
                "authoring",
                "compile",
                source.to_str().expect("UTF-8 source path"),
                "-o",
                output.to_str().expect("UTF-8 output path"),
            ])
            .output()
            .expect("failed to run rive-cli authoring compile");

        assert!(
            result.status.success(),
            "authoring compile failed for {name}: {}",
            String::from_utf8_lossy(&result.stderr)
        );

        let expected = std::fs::read(&committed).expect("committed showcase artifact must exist");
        let actual =
            std::fs::read(&output).expect("authoring compile must produce a showcase artifact");
        assert_eq!(
            actual, expected,
            "committed AuthoringSpec showcase '{name}' drifted; regenerate with `cargo run --quiet -- authoring compile examples/authoring/{name}.v0.json -o examples/authoring/{name}.v0.riv` and intentionally replace the committed artifact"
        );
    }
}
