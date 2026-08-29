use std::path::PathBuf;
use std::process::Command;

fn temp_output() -> PathBuf {
    std::env::temp_dir().join(format!("rive_showcase_artifact_{}.riv", std::process::id()))
}

struct CleanupOnDrop(PathBuf);

impl Drop for CleanupOnDrop {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn committed_authoring_showcase_matches_public_authoring_compile() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source = root.join("examples/authoring/complex-animated-showcase.v0.json");
    let committed = root.join("examples/authoring/complex-animated-showcase.v0.riv");
    let output = temp_output();
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
        "authoring compile failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );

    let expected = std::fs::read(&committed).expect("committed showcase artifact must exist");
    let actual =
        std::fs::read(&output).expect("authoring compile must produce a showcase artifact");
    assert_eq!(
        actual, expected,
        "committed AuthoringSpec showcase drifted; regenerate with `cargo run --quiet -- authoring compile examples/authoring/complex-animated-showcase.v0.json -o examples/authoring/complex-animated-showcase.v0.riv` and intentionally replace the committed artifact"
    );
}
