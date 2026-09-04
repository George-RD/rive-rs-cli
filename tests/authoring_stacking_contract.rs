use rive_cli::{
    authoring::{AuthoringSourceMap, LoweredAuthoring, lower_authoring_json},
    builder::{SceneSpec, build_scene},
};
use serde_json::{Value, json};

fn document() -> Value {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "stage",
            "width": { "value": 128.0, "unit": "px" },
            "height": { "value": 128.0, "unit": "px" }
        },
        "visual": { "nodes": [] },
        "motion": {},
        "behavior": {}
    })
}

fn rectangle(id: &str, fill: &str, x: f64, y: f64, size: f64) -> Value {
    json!({
        "kind": "rectangle",
        "id": id,
        "width": { "kind": "literal", "value": size, "unit": "px" },
        "height": { "kind": "literal", "value": size, "unit": "px" },
        "fill": fill,
        "transform": {
            "x": { "kind": "literal", "value": x, "unit": "px" },
            "y": { "kind": "literal", "value": y, "unit": "px" }
        }
    })
}

fn lower(input: &Value) -> LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("AuthoringSpec should lower")
}

fn source_entry<'a>(
    source_map: &'a AuthoringSourceMap,
    authored_id: &str,
) -> &'a rive_cli::authoring::SourceMapEntry {
    source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == authored_id)
        .expect("source-map entry should exist")
}

fn assert_builds(scene: Value) {
    let scene: SceneSpec =
        serde_json::from_value(scene).expect("lowered SceneSpec should deserialize");
    build_scene(&scene, None).expect("lowered SceneSpec should pass the canonical builder");
}

#[test]
fn root_back_to_front_stacking_is_explicit_and_backward_compatible() {
    let mut input = document();
    input["visual"]["nodes"] = json!([
        rectangle("surface", "#C2410C", 0.0, 0.0, 128.0),
        rectangle("cue", "#22C55E", 48.0, 48.0, 32.0)
    ]);

    let legacy = lower(&input);
    let legacy_children = legacy.scene["artboard"]["children"]
        .as_array()
        .expect("artboard children");
    let legacy_surface = source_entry(&legacy.source_map, "surface");
    assert_eq!(legacy_children[0]["name"], legacy_surface.runtime_names[0]);

    input["visual"]["stacking"] = json!("back_to_front");
    let first = lower(&input);
    let second = lower(&input);
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let children = first.scene["artboard"]["children"]
        .as_array()
        .expect("artboard children");
    let cue = source_entry(&first.source_map, "cue");
    let surface = source_entry(&first.source_map, "surface");
    assert_eq!(children[0]["name"], cue.runtime_names[0]);
    assert_eq!(children[1]["name"], surface.runtime_names[0]);
    assert_eq!(cue.authored_path, "$.visual.nodes[1]");
    assert_eq!(surface.authored_path, "$.visual.nodes[0]");
    assert_eq!(cue.scene_paths[0], "/artboard/children/0");
    assert_eq!(surface.scene_paths[0], "/artboard/children/1");
    assert_builds(first.scene);
}

#[test]
fn group_back_to_front_stacking_keeps_authored_paths_while_reversing_scene_paths() {
    let mut input = document();
    input["visual"]["nodes"] = json!([
        {
            "kind": "group",
            "id": "card",
            "stacking": "back_to_front",
            "children": [
                rectangle("surface", "#C2410C", 0.0, 0.0, 128.0),
                rectangle("cue", "#22C55E", 48.0, 48.0, 32.0)
            ]
        }
    ]);

    let lowered = lower(&input);
    let children = lowered.scene["artboard"]["children"][0]["children"]
        .as_array()
        .expect("group children");
    let cue = source_entry(&lowered.source_map, "card/cue");
    let surface = source_entry(&lowered.source_map, "card/surface");
    assert_eq!(children[0]["name"], cue.runtime_names[0]);
    assert_eq!(children[1]["name"], surface.runtime_names[0]);
    assert_eq!(cue.authored_path, "$.visual.nodes[0].children[1]");
    assert_eq!(surface.authored_path, "$.visual.nodes[0].children[0]");
    assert_eq!(cue.scene_paths[0], "/artboard/children/0/children/0");
    assert_eq!(surface.scene_paths[0], "/artboard/children/0/children/1");
    assert_builds(lowered.scene);
}

#[test]
fn component_back_to_front_stacking_preserves_definition_paths() {
    let mut input = document();
    input["components"] = json!([
        {
            "id": "card",
            "stacking": "back_to_front",
            "visual": [
                rectangle("surface", "#C2410C", 0.0, 0.0, 128.0),
                rectangle("cue", "#22C55E", 48.0, 48.0, 32.0)
            ]
        }
    ]);
    input["visual"]["nodes"] = json!([
        { "kind": "instance", "id": "instance", "component": "card" }
    ]);

    let lowered = lower(&input);
    let children = lowered.scene["artboard"]["children"][0]["children"]
        .as_array()
        .expect("instance children");
    let cue = source_entry(&lowered.source_map, "instance/cue");
    let surface = source_entry(&lowered.source_map, "instance/surface");
    assert_eq!(children[0]["name"], cue.runtime_names[0]);
    assert_eq!(children[1]["name"], surface.runtime_names[0]);
    assert_eq!(cue.authored_path, "$.visual.nodes[0].expanded[1]");
    assert_eq!(surface.authored_path, "$.visual.nodes[0].expanded[0]");
    assert_eq!(
        cue.definition_path.as_deref(),
        Some("$.components[0].visual[1]")
    );
    assert_eq!(
        surface.definition_path.as_deref(),
        Some("$.components[0].visual[0]")
    );
    assert_eq!(cue.scene_paths[0], "/artboard/children/0/children/0");
    assert_eq!(surface.scene_paths[0], "/artboard/children/0/children/1");
    assert_builds(lowered.scene);
}

#[test]
fn back_to_front_diagnostics_use_authored_child_indexes() {
    let mut input = document();
    let mut cue = rectangle("cue", "#22C55E", 48.0, 48.0, 32.0);
    cue["width"]["unit"] = json!("scalar");
    input["visual"]["nodes"] = json!([
        {
            "kind": "group",
            "id": "card",
            "stacking": "back_to_front",
            "children": [
                rectangle("surface", "#C2410C", 0.0, 0.0, 128.0),
                cue
            ]
        }
    ]);

    let error = lower_authoring_json(&input.to_string())
        .expect_err("invalid foreground dimensions should fail");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.path == "$.visual.nodes[0].children[1].width"
            && diagnostic.code == "unit_mismatch"
    }));
}
