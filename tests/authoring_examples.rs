use std::{collections::BTreeSet, fs, path::Path};

use rive_cli::authoring::{LoweredAuthoring, lower_authoring_json};
use rive_cli::builder::{SceneSpec, build_scene};
use rive_cli::encoder::encode_riv;
use rive_cli::validator::validate_riv;
use serde_json::Value;

const ANIMATED_SHOWCASE_DURATION_FRAMES: u64 = 120;
const ANIMATED_SHOWCASE_FPS: u64 = 60;
const ANIMATED_SHOWCASE_MIN_OBJECT_COUNT: usize = 40;
const ANIMATED_SHOWCASE_MIN_RIV_BYTES: usize = 1_000;
const ANIMATED_SHOWCASE_SETTLE_FRAME: f64 = 78.0;
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

fn lower_deterministically(input: &str) -> LoweredAuthoring {
    let first = lower_authoring_json(input).expect("first lowering must succeed");
    let second = lower_authoring_json(input).expect("second lowering must succeed");
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);
    first
}

fn assert_deterministic_and_buildable(input: &str) {
    let lowered = lower_deterministically(input);
    let scene: SceneSpec =
        serde_json::from_value(lowered.scene).expect("lowered scene must deserialize");
    build_scene(&scene, None).expect("lowered scene must pass the canonical builder");
}

fn collect_string_field(value: &Value, field: &str, values: &mut BTreeSet<String>) {
    match value {
        Value::Array(items) => {
            for item in items {
                collect_string_field(item, field, values);
            }
        }
        Value::Object(object) => {
            if let Some(value) = object.get(field).and_then(Value::as_str) {
                values.insert(value.to_string());
            }
            for value in object.values() {
                collect_string_field(value, field, values);
            }
        }
        _ => {}
    }
}

fn complex_static_showcase_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("authoring")
        .join("complex-static-showcase.v0.json")
}

fn complex_animated_showcase_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("authoring")
        .join("complex-animated-showcase.v0.json")
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

#[test]
fn complex_static_showcase_compiles_without_raw_escapes() {
    let input = fs::read_to_string(complex_static_showcase_path())
        .expect("complex static AuthoringSpec showcase must exist");
    let authored: Value = serde_json::from_str(&input).expect("showcase must be valid JSON");

    assert!(
        authored
            .get("motion")
            .and_then(Value::as_object)
            .is_some_and(serde_json::Map::is_empty)
    );
    assert!(
        authored
            .get("behavior")
            .and_then(Value::as_object)
            .is_some_and(serde_json::Map::is_empty)
    );

    let mut kinds = BTreeSet::new();
    collect_string_field(&authored, "kind", &mut kinds);
    assert!(!kinds.contains("raw_scene_object"));
    for expected in [
        "along_path",
        "distribute",
        "ellipse",
        "grid",
        "group",
        "instance",
        "linear_gradient",
        "mirror",
        "radial",
        "radial_gradient",
        "rectangle",
        "star",
        "text",
    ] {
        assert!(kinds.contains(expected), "showcase is missing {expected}");
    }

    let mut ids = BTreeSet::new();
    collect_string_field(&authored, "id", &mut ids);
    for expected in [
        "footer",
        "hero-panel",
        "orbit",
        "proof-card",
        "proof-grid",
        "showcase-stage",
    ] {
        assert!(ids.contains(expected), "showcase is missing id {expected}");
    }

    let lowered = lower_deterministically(&input);
    assert!(
        lowered
            .source_map
            .entries
            .iter()
            .all(|entry| entry.runtime_names.len() == entry.scene_paths.len())
    );
    assert!(lowered.source_map.entries.len() >= 30);
    assert!(
        lowered
            .source_map
            .entries
            .iter()
            .filter(|entry| entry.authored_id.contains('/'))
            .count()
            >= 12
    );
    assert!(
        lowered
            .source_map
            .entries
            .iter()
            .map(|entry| entry.runtime_names.len())
            .sum::<usize>()
            >= 100
    );

    let scene: SceneSpec =
        serde_json::from_value(lowered.scene).expect("showcase SceneSpec must deserialize");
    let objects = build_scene(&scene, None).expect("showcase must pass the canonical builder");
    let object_refs = objects
        .iter()
        .map(|object| object.as_ref())
        .collect::<Vec<_>>();
    let bytes = encode_riv(&object_refs, 0);
    let report = validate_riv(&bytes).expect("encoded showcase must parse");

    assert!(report.valid, "encoded showcase errors: {:?}", report.errors);
    assert!(report.object_count >= 100);
    assert!(bytes.len() >= 1_000);
}

#[test]
fn complex_animated_showcase_compiles_without_raw_escapes() {
    let showcase_path = complex_animated_showcase_path();
    let input = fs::read_to_string(&showcase_path)
        .expect("complex animated AuthoringSpec showcase must exist");
    let authored: Value = serde_json::from_str(&input).expect("showcase must be valid JSON");

    assert!(
        authored
            .get("behavior")
            .and_then(Value::as_object)
            .is_some_and(serde_json::Map::is_empty)
    );
    assert!(authored["motion"].get("raw_animations").is_none());

    let mut kinds = BTreeSet::new();
    collect_string_field(&authored, "kind", &mut kinds);
    assert!(!kinds.contains("raw_scene_object"));
    for expected in ["ellipse", "group", "rectangle", "text"] {
        assert!(kinds.contains(expected), "showcase is missing {expected}");
    }

    let poses = authored["motion"]["poses"]
        .as_array()
        .expect("showcase motion poses");
    let pose_ids = poses
        .iter()
        .filter_map(|pose| pose["id"].as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        pose_ids,
        BTreeSet::from(["action-ready", "connected", "overloaded", "scattered"])
    );

    let easings = authored["motion"]["easings"]
        .as_array()
        .expect("showcase motion easings");
    assert_eq!(easings.len(), 1);
    assert_eq!(easings[0]["id"], "settle-out");
    assert_eq!(easings[0]["x1"]["value"].as_f64(), Some(0.16));
    assert_eq!(easings[0]["y1"]["value"].as_f64(), Some(1.0));
    assert_eq!(easings[0]["x2"]["value"].as_f64(), Some(0.3));
    assert_eq!(easings[0]["y2"]["value"].as_f64(), Some(1.0));

    let tracks = authored["motion"]["tracks"]
        .as_array()
        .expect("showcase motion tracks");
    assert_eq!(tracks.len(), 1);
    assert_eq!(tracks[0]["id"], "decision-flow");
    assert_eq!(tracks[0]["fps"], ANIMATED_SHOWCASE_FPS);
    let authored_settle_keyframe = tracks[0]["keyframes"]
        .as_array()
        .expect("showcase authored keyframes")
        .iter()
        .find(|keyframe| {
            keyframe["frame"]["value"].as_f64() == Some(ANIMATED_SHOWCASE_SETTLE_FRAME)
        })
        .expect("showcase frame 78 keyframe");
    assert_eq!(authored_settle_keyframe["easing"], "settle-out");

    let lowered = lower_deterministically(&input);
    for expected_id in [
        "decision-flow",
        "decision-hub",
        "focus-ring",
        "input-chat",
        "input-mail",
        "input-ops",
        "input-sheet",
        "next-action",
    ] {
        let source = lowered
            .source_map
            .entries
            .iter()
            .find(|entry| entry.authored_id == expected_id)
            .unwrap_or_else(|| panic!("showcase source map is missing {expected_id}"));
        assert!(
            !source.runtime_names.is_empty(),
            "showcase source map has no runtime names for {expected_id}"
        );
        assert_eq!(source.runtime_names.len(), source.scene_paths.len());
    }

    let animations = lowered.scene["artboard"]["animations"]
        .as_array()
        .expect("showcase animations");
    assert_eq!(animations.len(), 1);
    let animation = &animations[0];
    assert_eq!(animation["fps"], ANIMATED_SHOWCASE_FPS);
    assert_eq!(animation["duration"], ANIMATED_SHOWCASE_DURATION_FRAMES);
    assert_eq!(animation["loop_type"], "oneshot");
    let interpolators = animation["interpolators"]
        .as_array()
        .expect("shared cubic easing declaration");
    assert_eq!(interpolators.len(), 1);
    let interpolator = &interpolators[0];
    assert_eq!(interpolator["type"], "cubic");
    assert_eq!(interpolator["x1"].as_f64(), Some(0.16));
    assert_eq!(interpolator["y1"].as_f64(), Some(1.0));
    assert_eq!(interpolator["x2"].as_f64(), Some(0.3));
    assert_eq!(interpolator["y2"].as_f64(), Some(1.0));
    let interpolator_name = interpolator["name"]
        .as_str()
        .expect("shared cubic easing runtime name");

    let keyframe_groups = animation["keyframes"]
        .as_array()
        .expect("showcase keyframe groups");
    let keyed_properties = keyframe_groups
        .iter()
        .filter_map(|group| group["property"].as_str())
        .collect::<BTreeSet<_>>();
    for expected in ["height", "opacity", "width", "x", "y"] {
        assert!(
            keyed_properties.contains(expected),
            "showcase animation is missing {expected} motion"
        );
    }
    let settled_frames = keyframe_groups
        .iter()
        .flat_map(|group| {
            group["frames"]
                .as_array()
                .expect("showcase property frames")
                .iter()
        })
        .filter(|frame| frame["frame"].as_f64() == Some(ANIMATED_SHOWCASE_SETTLE_FRAME))
        .collect::<Vec<_>>();
    assert!(
        !settled_frames.is_empty(),
        "showcase must lower frame 78 property keyframes"
    );
    assert!(settled_frames.iter().all(|frame| {
        frame["interpolation"] == "cubic" && frame["interpolator"] == interpolator_name
    }));

    let scene: SceneSpec =
        serde_json::from_value(lowered.scene).expect("showcase SceneSpec must deserialize");
    let source_dir = showcase_path
        .parent()
        .expect("showcase fixture must have a source directory");
    let objects =
        build_scene(&scene, Some(source_dir)).expect("showcase must pass the canonical builder");
    let object_refs = objects
        .iter()
        .map(|object| object.as_ref())
        .collect::<Vec<_>>();
    let bytes = encode_riv(&object_refs, 0);
    let report = validate_riv(&bytes).expect("encoded showcase must parse");

    assert!(report.valid, "encoded showcase errors: {:?}", report.errors);
    assert!(report.object_count >= ANIMATED_SHOWCASE_MIN_OBJECT_COUNT);
    assert!(bytes.len() >= ANIMATED_SHOWCASE_MIN_RIV_BYTES);
}
