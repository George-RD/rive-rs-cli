use rive_cli::authoring::{authoring_schema, lower_authoring_json};
use rive_cli::builder::{SceneSpec, build_scene};
use rive_cli::objects::core::type_keys;
use serde_json::{Value, json};

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn parameter(name: &str) -> Value {
    json!({ "kind": "parameter", "name": name })
}

fn document() -> Value {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "easing-stage",
            "width": { "value": 240.0, "unit": "px" },
            "height": { "value": 160.0, "unit": "px" }
        },
        "parameters": {
            "ease_x1": { "value": 0.16, "unit": "scalar" }
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
                    "id": "left",
                    "targets": [
                        {
                            "target": "card",
                            "transform": {
                                "x": literal(32.0, "px"),
                                "y": literal(56.0, "px")
                            }
                        }
                    ]
                },
                {
                    "id": "right",
                    "targets": [
                        {
                            "target": "card",
                            "transform": {
                                "x": literal(128.0, "px"),
                                "y": literal(56.0, "px")
                            }
                        }
                    ]
                }
            ],
            "tracks": [
                {
                    "id": "forward",
                    "fps": 60,
                    "duration_frames": literal(30.0, "scalar"),
                    "keyframes": [
                        {
                            "frame": literal(0.0, "scalar"),
                            "pose": "left"
                        },
                        {
                            "frame": literal(30.0, "scalar"),
                            "pose": "right"
                        }
                    ]
                }
            ]
        },
        "behavior": {}
    })
}

fn cubic_easing(id: &str) -> Value {
    json!({
        "kind": "cubic",
        "id": id,
        "x1": parameter("ease_x1"),
        "y1": literal(1.0, "scalar"),
        "x2": literal(0.3, "scalar"),
        "y2": literal(1.0, "scalar")
    })
}

fn lower(input: &Value) -> rive_cli::authoring::LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("shared easing authoring must lower")
}

fn cubic_interpolator_count(scene: &Value) -> usize {
    let scene: SceneSpec =
        serde_json::from_value(scene.clone()).expect("lowered SceneSpec must deserialize");
    build_scene(&scene, None)
        .expect("lowered SceneSpec must pass the canonical builder")
        .iter()
        .filter(|object| object.type_key() == type_keys::CUBIC_EASE_INTERPOLATOR)
        .count()
}

fn assert_diagnostic(input: &Value, code: &str, path: &str) {
    let error = lower_authoring_json(&input.to_string())
        .expect_err("invalid shared easing authoring must fail");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code && diagnostic.path == path),
        "missing {code} at {path}; diagnostics: {:#?}",
        error.diagnostics
    );
}

fn keyed_x(animation: &Value) -> &Value {
    animation["keyframes"]
        .as_array()
        .expect("animation keyframes")
        .iter()
        .find(|group| group["property"] == "x")
        .expect("x keyframe group")
}

#[test]
fn shared_cubic_easing_lowers_once_and_reuses_deterministically() {
    let mut input = document();
    input["motion"]["easings"] = json!([cubic_easing("soft-out")]);
    input["motion"]["tracks"][0]["keyframes"][0]["easing"] = json!("soft-out");

    let mut return_track = input["motion"]["tracks"][0].clone();
    return_track["id"] = json!("return");
    return_track["keyframes"][0]["pose"] = json!("right");
    return_track["keyframes"][1]["pose"] = json!("left");
    return_track["keyframes"][0]["easing"] = json!("soft-out");
    input["motion"]["tracks"]
        .as_array_mut()
        .expect("motion tracks")
        .push(return_track);

    let first = lower(&input);
    let second = lower(&input);
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let animations = first.scene["artboard"]["animations"]
        .as_array()
        .expect("typed animations");
    assert_eq!(animations.len(), 2);

    for animation in animations {
        let interpolators = animation["interpolators"]
            .as_array()
            .expect("referencing animation declares its interpolator");
        assert_eq!(interpolators.len(), 1);
        assert_eq!(
            interpolators[0]["name"],
            "auth__easing_2dstage__soft_2dout__interpolator"
        );
        assert_eq!(interpolators[0]["type"], "cubic");
        assert_eq!(interpolators[0]["x1"], 0.16);
        assert_eq!(interpolators[0]["y1"], 1.0);
        assert_eq!(interpolators[0]["x2"], 0.3);
        assert_eq!(interpolators[0]["y2"], 1.0);

        let first_frame = &keyed_x(animation)["frames"][0];
        assert_eq!(first_frame["interpolation"], "cubic");
        assert_eq!(
            first_frame["interpolator"],
            "auth__easing_2dstage__soft_2dout__interpolator"
        );
    }

    let easing_source = first
        .source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == "soft-out")
        .expect("shared easing source-map entry");
    assert_eq!(easing_source.authored_path, "$.motion.easings[0]");
    assert_eq!(
        easing_source.runtime_names,
        vec!["auth__easing_2dstage__soft_2dout__interpolator"]
    );
    assert_eq!(
        easing_source.scene_paths,
        vec![
            "/artboard/animations/0/interpolators/0",
            "/artboard/animations/1/interpolators/0",
        ]
    );
    assert_eq!(
        cubic_interpolator_count(&first.scene),
        1,
        "canonical builder must deduplicate identical local declarations"
    );

    let schema = serde_json::to_string(&authoring_schema()).expect("serialize schema");
    for required in ["easings", "cubic", "easing"] {
        assert!(schema.contains(required), "schema is missing {required}");
    }

}

#[test]
fn easing_references_are_validated_at_authored_paths() {
    let mut duplicate = document();
    duplicate["motion"]["easings"] = json!([
        cubic_easing("soft-out"),
        cubic_easing("soft-out")
    ]);
    assert_diagnostic(
        &duplicate,
        "duplicate_easing",
        "$.motion.easings[1].id",
    );

    let mut unknown = document();
    unknown["motion"]["tracks"][0]["keyframes"][0]["easing"] = json!("missing");
    assert_diagnostic(
        &unknown,
        "unknown_easing",
        "$.motion.tracks[0].keyframes[0].easing",
    );

    let mut held = document();
    held["motion"]["easings"] = json!([cubic_easing("soft-out")]);
    held["motion"]["tracks"][0]["keyframes"][0]["interpolation"] = json!("hold");
    held["motion"]["tracks"][0]["keyframes"][0]["easing"] = json!("soft-out");
    assert_diagnostic(
        &held,
        "easing_with_hold",
        "$.motion.tracks[0].keyframes[0].easing",
    );
}

#[test]
fn cubic_easing_control_points_use_scalar_expression_validation() {
    let mut outside_time_range = document();
    let mut invalid = cubic_easing("invalid");
    invalid["x1"] = literal(1.2, "scalar");
    outside_time_range["motion"]["easings"] = json!([invalid]);
    outside_time_range["motion"]["tracks"] = json!([]);
    assert_diagnostic(
        &outside_time_range,
        "invalid_easing_control_point",
        "$.motion.easings[0].x1",
    );

    let mut incompatible_unit = document();
    let mut invalid = cubic_easing("invalid");
    invalid["x1"] = literal(0.2, "px");
    incompatible_unit["motion"]["easings"] = json!([invalid]);
    incompatible_unit["motion"]["tracks"] = json!([]);
    assert_diagnostic(
        &incompatible_unit,
        "unit_mismatch",
        "$.motion.easings[0].x1",
    );
}

