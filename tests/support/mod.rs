use rive_cli::builder::{SceneSpec, build_scene};

pub(crate) fn assert_builds(scene: serde_json::Value) {
    let scene: SceneSpec =
        serde_json::from_value(scene).expect("lowered SceneSpec must deserialize");
    build_scene(&scene, None).expect("lowered SceneSpec must pass the canonical builder");
}
