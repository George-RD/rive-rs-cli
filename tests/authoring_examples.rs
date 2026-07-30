use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::{SceneSpec, build_scene};

const COMPONENT_BADGES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/authoring/component-badges.v0.json"
));
const RAW_PULSE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/authoring/raw-pulse.v0.json"
));
const TEXT_LABEL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/authoring/text-label.v0.json"
));

fn assert_deterministic_and_buildable(input: &str) {
    let first = lower_authoring_json(input).expect("first lowering must succeed");
    let second = lower_authoring_json(input).expect("second lowering must succeed");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let scene: SceneSpec =
        serde_json::from_value(first.scene).expect("lowered scene must deserialize");
    build_scene(&scene, None).expect("lowered scene must pass the canonical builder");
}

#[test]
fn component_badges_example_is_deterministic_and_buildable() {
    assert_deterministic_and_buildable(COMPONENT_BADGES);
}

#[test]
fn raw_pulse_example_is_deterministic_and_buildable() {
    assert_deterministic_and_buildable(RAW_PULSE);
}

#[test]
fn text_label_example_is_deterministic_and_buildable() {
    assert_deterministic_and_buildable(TEXT_LABEL);
}
