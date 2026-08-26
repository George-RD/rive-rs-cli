use std::path::{Path, PathBuf};

use rive_cli::{
    builder::SceneSpec,
    compile::{CompileError, compile_scene},
    validator::{InspectFilter, PropertyValueRead, parse_riv, validate_riv},
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn fixture_spec(name: &str) -> SceneSpec {
    let path = fixture_dir().join(name);
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    serde_json::from_str(&input)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}

#[test]
fn public_compile_seam_encodes_valid_scene_spec() {
    let bytes = compile_scene(&fixture_spec("minimal.json"), None, 0)
        .expect("valid SceneSpec should compile");
    let report = validate_riv(&bytes).expect("compiled bytes should parse");

    assert_eq!(&bytes[..4], b"RIVE");
    assert!(report.valid, "compiled bytes should validate: {report:?}");
}

#[test]
fn public_compile_seam_preserves_file_id() {
    let bytes = compile_scene(&fixture_spec("minimal.json"), None, 0x1234_5678)
        .expect("valid SceneSpec should compile");
    let report = validate_riv(&bytes).expect("compiled bytes should parse");

    assert_eq!(report.header.file_id, 0x1234_5678);
}

#[test]
fn public_compile_seam_resolves_relative_assets_from_base_dir() {
    const FILE_ASSET_CONTENTS_TYPE_KEY: u16 = 106;

    let spec = fixture_spec("embedded_assets.json");
    let missing_base = compile_scene(&spec, None, 0)
        .expect_err("relative asset sources should require a base directory");
    assert_eq!(missing_base.code(), "invalid-scene");

    let base_dir = fixture_dir();
    let bytes = compile_scene(&spec, Some(&base_dir), 0)
        .expect("relative asset sources should resolve from the supplied base directory");
    let parsed =
        parse_riv(&bytes, &InspectFilter::default()).expect("compiled asset scene should parse");
    let content_lengths = parsed
        .objects
        .iter()
        .filter(|object| object.type_key == FILE_ASSET_CONTENTS_TYPE_KEY)
        .flat_map(|object| &object.properties)
        .filter_map(|property| match &property.value {
            PropertyValueRead::Bytes { length } => Some(*length),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(content_lengths.len(), 2);
    assert!(content_lengths.iter().all(|length| *length > 0));
}

#[test]
fn public_compile_seam_returns_typed_build_error() {
    let invalid: SceneSpec = serde_json::from_str(
        r#"{"scene_format_version":1,"artboard":{"name":"Invalid","width":-1,"height":100,"children":[]}}"#,
    )
    .expect("invalid builder input should still deserialize as SceneSpec");

    let error = compile_scene(&invalid, None, 0).expect_err("invalid SceneSpec should not compile");

    assert_eq!(error.code(), "invalid-scene");
    assert!(matches!(error, CompileError::Build(_)));
}
