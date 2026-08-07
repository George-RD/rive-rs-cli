mod support;

use rive_cli::authoring::{LoweredAuthoring, SourceMapEntry, lower_authoring_json};
use serde_json::{Value, json};
use support::assert_builds;

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn mixed_document() -> Value {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "compiler-stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 200.0, "unit": "px" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "rectangle",
                    "id": "card",
                    "width": literal(80.0, "px"),
                    "height": literal(48.0, "px"),
                    "fill": "#172554"
                }
            ]
        },
        "motion": {
            "poses": [
                {
                    "id": "rest",
                    "targets": [
                        {
                            "target": "card",
                            "transform": { "x": literal(16.0, "px") }
                        }
                    ]
                },
                {
                    "id": "moved",
                    "targets": [
                        {
                            "target": "card",
                            "transform": { "x": literal(160.0, "px") }
                        }
                    ]
                }
            ],
            "tracks": [
                {
                    "id": "entrance",
                    "fps": 60,
                    "duration_frames": literal(30.0, "scalar"),
                    "keyframes": [
                        { "frame": literal(0.0, "scalar"), "pose": "rest" },
                        { "frame": literal(30.0, "scalar"), "pose": "moved" }
                    ]
                },
                {
                    "id": "settle",
                    "fps": 60,
                    "duration_frames": literal(30.0, "scalar"),
                    "keyframes": [
                        { "frame": literal(0.0, "scalar"), "pose": "moved" },
                        { "frame": literal(30.0, "scalar"), "pose": "rest" }
                    ]
                }
            ],
            "raw_animations": [
                {
                    "id": "raw-first",
                    "value": {
                        "name": "raw_first",
                        "fps": 60,
                        "duration": 1,
                        "keyframes": []
                    }
                },
                {
                    "id": "raw-second",
                    "value": {
                        "name": "raw_second",
                        "fps": 60,
                        "duration": 1,
                        "keyframes": []
                    }
                }
            ]
        },
        "behavior": {
            "raw_state_machines": [
                {
                    "id": "mixed-machine",
                    "value": {
                        "name": "mixed_machine",
                        "layers": [
                            {
                                "states": [
                                    { "type": "entry" },
                                    { "type": "exit" },
                                    {
                                        "type": "animation",
                                        "animation": "auth__compiler_2dstage__settle__animation"
                                    }
                                ]
                            }
                        ]
                    }
                }
            ]
        }
    })
}

fn lower(input: &Value) -> LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("mixed authoring document must lower")
}

fn source_entry<'a>(lowered: &'a LoweredAuthoring, authored_id: &str) -> &'a SourceMapEntry {
    lowered
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == authored_id)
        .expect("expected source-map entry")
}

fn assert_source_entry(
    lowered: &LoweredAuthoring,
    authored_id: &str,
    authored_path: &str,
    runtime_names: &[&str],
    scene_paths: &[&str],
) {
    let entry = source_entry(lowered, authored_id);
    assert_eq!(entry.authored_path, authored_path);
    assert_eq!(entry.definition_path, None);
    assert_eq!(
        entry.runtime_names,
        runtime_names
            .iter()
            .map(|name| (*name).to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        entry.scene_paths,
        scene_paths
            .iter()
            .map(|path| (*path).to_string())
            .collect::<Vec<_>>()
    );
}

#[test]
fn mixed_typed_raw_motion_preserves_order_references_and_source_identity() {
    let input = mixed_document();
    let first = lower(&input);
    let second = lower(&input);

    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let animation_names = first.scene["artboard"]["animations"]
        .as_array()
        .expect("mixed animation list")
        .iter()
        .map(|animation| animation["name"].as_str().expect("animation name"))
        .collect::<Vec<_>>();
    assert_eq!(
        animation_names,
        vec![
            "auth__compiler_2dstage__entrance__animation",
            "auth__compiler_2dstage__settle__animation",
            "raw_first",
            "raw_second"
        ]
    );

    assert_eq!(
        first.scene["artboard"]["state_machines"][0]["layers"][0]["states"][2]["animation"],
        "auth__compiler_2dstage__settle__animation"
    );

    let compiler_entries = first
        .source_map
        .entries
        .iter()
        .filter(|entry| {
            entry.authored_path.starts_with("$.motion.")
                || entry.authored_path.starts_with("$.behavior.")
        })
        .map(|entry| entry.authored_id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        compiler_entries,
        vec![
            "entrance",
            "settle",
            "raw-first",
            "raw-second",
            "mixed-machine"
        ]
    );

    assert_source_entry(
        &first,
        "entrance",
        "$.motion.tracks[0]",
        &["auth__compiler_2dstage__entrance__animation"],
        &["/artboard/animations/0"],
    );
    assert_source_entry(
        &first,
        "settle",
        "$.motion.tracks[1]",
        &["auth__compiler_2dstage__settle__animation"],
        &["/artboard/animations/1"],
    );
    assert_source_entry(
        &first,
        "raw-first",
        "$.motion.raw_animations[0]",
        &["raw_first"],
        &["/artboard/animations/2"],
    );
    assert_source_entry(
        &first,
        "raw-second",
        "$.motion.raw_animations[1]",
        &["raw_second"],
        &["/artboard/animations/3"],
    );
    assert_source_entry(
        &first,
        "mixed-machine",
        "$.behavior.raw_state_machines[0]",
        &["mixed_machine"],
        &["/artboard/state_machines/0"],
    );

    assert_builds(first.scene);
}

#[test]
fn raw_animation_diagnostic_keeps_authored_index_after_typed_prefix() {
    let mut input = mixed_document();
    input["motion"]["raw_animations"][0]["value"] = json!(7);

    let error = lower_authoring_json(&input.to_string())
        .expect_err("invalid raw animation must fail after typed tracks are generated");
    assert_eq!(error.diagnostics.len(), 1);
    let diagnostic = &error.diagnostics[0];
    assert_eq!(diagnostic.code, "invalid_raw_scene_fragment");
    assert_eq!(
        diagnostic.path,
        "$.motion.raw_animations[0].value",
        "the generated two-track prefix must not leak into authored diagnostics"
    );
    assert_eq!(
        diagnostic.message,
        "raw SceneSpec escape must be a JSON object"
    );
}
