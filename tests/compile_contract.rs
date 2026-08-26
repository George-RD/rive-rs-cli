use rive_cli::{builder::SceneSpec, compile::compile_scene};

#[test]
fn public_compile_seam_encodes_valid_scene_spec() {
    let spec: SceneSpec = serde_json::from_str(include_str!("fixtures/minimal.json"))
        .expect("minimal fixture should deserialize");

    let bytes = compile_scene(&spec, None, 0).expect("valid SceneSpec should compile");

    assert_eq!(&bytes[..4], b"RIVE");
}
