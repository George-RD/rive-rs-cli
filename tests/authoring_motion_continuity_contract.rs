use rive_cli::authoring::{LoweredAuthoring, lower_authoring_json};
use rive_cli::builder::{SceneSpec, build_scene};
use serde_json::{Value, json};

fn literal(value: f64, unit: &str) -> Value {
    json!({ "kind": "literal", "value": value, "unit": unit })
}

fn pose(id: &str, x: f64) -> Value {
    json!({
        "id": id,
        "targets": [
            {
                "target": "token",
                "transform": { "x": literal(x, "px"), "y": literal(80.0, "px") }
            }
        ]
    })
}

fn keyframe(frame: f64, pose: &str, easing: Option<&str>) -> Value {
    let mut value = json!({ "frame": literal(frame, "scalar"), "pose": pose });
    if let (Some(easing), Some(object)) = (easing, value.as_object_mut()) {
        object.insert("easing".to_string(), json!(easing));
    }
    value
}

fn document() -> Value {
    json!({
        "authoring_format_version": 0,
        "artboard": {
            "id": "transit-stage",
            "width": { "value": 320.0, "unit": "px" },
            "height": { "value": 160.0, "unit": "px" }
        },
        "visual": {
            "nodes": [
                {
                    "kind": "rectangle",
                    "id": "token",
                    "width": literal(24.0, "px"),
                    "height": literal(24.0, "px"),
                    "fill": "#22C55E"
                }
            ]
        },
        "motion": {
            "easings": [
                {
                    "kind": "cubic",
                    "id": "settle",
                    "x1": literal(0.23, "scalar"),
                    "y1": literal(1.0, "scalar"),
                    "x2": literal(0.32, "scalar"),
                    "y2": literal(1.0, "scalar")
                }
            ],
            "poses": [pose("start", 40.0), pose("mid", 160.0), pose("arrive", 280.0)],
            "tracks": [
                {
                    "id": "transit",
                    "fps": 60,
                    "duration_frames": literal(60.0, "scalar"),
                    "keyframes": [
                        keyframe(0.0, "start", Some("settle")),
                        keyframe(30.0, "mid", Some("settle")),
                        keyframe(60.0, "arrive", Some("settle"))
                    ]
                }
            ]
        },
        "behavior": {}
    })
}

fn lower(input: &Value) -> LoweredAuthoring {
    lower_authoring_json(&input.to_string()).expect("waypoint motion should lower")
}

fn assert_builds(scene: &Value) {
    let scene: SceneSpec =
        serde_json::from_value(scene.clone()).expect("lowered SceneSpec should deserialize");
    build_scene(&scene, None).expect("lowered SceneSpec should pass the canonical builder");
}

fn animation(scene: &Value) -> &Value {
    &scene["artboard"]["animations"][0]
}

fn x_frames(scene: &Value) -> &Vec<Value> {
    animation(scene)["keyframes"]
        .as_array()
        .expect("animation keyframe groups")
        .iter()
        .find(|group| group["property"] == "x")
        .expect("x keyframe group")["frames"]
        .as_array()
        .expect("x keyframes")
}

fn set_track_continuity(input: &mut Value, continuity: &str) {
    input["motion"]["tracks"][0]["continuity"] = json!(continuity);
}

fn set_waypoint(input: &mut Value, keyframe_index: usize, waypoint: &str) {
    input["motion"]["tracks"][0]["keyframes"][keyframe_index]["waypoint"] = json!(waypoint);
}

#[test]
fn through_continuity_lowers_interior_waypoints_without_stopping() {
    let mut input = document();
    set_track_continuity(&mut input, "through");

    let first = lower(&input);
    let second = lower(&input);
    assert_eq!(first.scene, second.scene);
    assert_eq!(first.source_map, second.source_map);

    let frames = x_frames(&first.scene);
    assert_eq!(frames[0]["interpolation"], "linear");
    assert_eq!(frames[0].get("interpolator"), None);
    assert_eq!(frames[1]["interpolation"], "cubic");
    assert!(frames[1].get("interpolator").is_some());
    assert!(first.warnings.is_empty());
    assert_builds(&first.scene);
}

#[test]
fn settle_waypoints_keep_authored_easing_inside_a_through_track() {
    let mut input = document();
    set_track_continuity(&mut input, "through");
    set_waypoint(&mut input, 1, "settle");

    let lowered = lower(&input);
    let frames = x_frames(&lowered.scene);
    assert_eq!(frames[0]["interpolation"], "cubic");
    assert!(frames[0].get("interpolator").is_some());
    assert_builds(&lowered.scene);
}

#[test]
fn transit_waypoints_preserve_velocity_inside_a_per_keyframe_track() {
    let mut input = document();
    set_waypoint(&mut input, 1, "transit");

    let lowered = lower(&input);
    let frames = x_frames(&lowered.scene);
    assert_eq!(frames[0]["interpolation"], "linear");
    assert_eq!(frames[0].get("interpolator"), None);
    assert_eq!(frames[1]["interpolation"], "cubic");
    assert!(lowered.warnings.is_empty());
}

#[test]
fn shared_stop_start_easing_reports_a_waypoint_warning() {
    let lowered = lower(&document());

    let warning = lowered
        .warnings
        .iter()
        .find(|warning| warning.code == "waypoint_stop_start")
        .expect("stop-start waypoint should be reported");
    assert_eq!(warning.path, "$.motion.tracks[0].keyframes[1]");
    assert!(warning.message.contains("settle"));
    assert_eq!(lowered.warnings.len(), 1);
    assert_builds(&lowered.scene);
}

#[test]
fn through_continuity_clears_the_stop_start_warning() {
    let mut input = document();
    set_track_continuity(&mut input, "through");

    assert!(lower(&input).warnings.is_empty());
}

#[test]
fn explicit_settle_waypoints_are_not_reported_as_stop_start() {
    let mut input = document();
    set_waypoint(&mut input, 1, "settle");

    assert!(lower(&input).warnings.is_empty());
}

#[test]
fn hold_keyframes_survive_continuity_rewriting() {
    let mut input = document();
    set_track_continuity(&mut input, "through");
    input["motion"]["tracks"][0]["keyframes"][0] = keyframe(0.0, "start", None);
    input["motion"]["tracks"][0]["keyframes"][0]["interpolation"] = json!("hold");

    let lowered = lower(&input);
    let frames = x_frames(&lowered.scene);
    assert_eq!(frames[0]["interpolation"], "hold");
    assert_eq!(frames[0].get("interpolator"), None);
}

#[test]
fn continuity_drops_interpolators_that_no_longer_have_a_segment() {
    let mut input = document();
    set_track_continuity(&mut input, "through");
    input["motion"]["tracks"][0]["keyframes"][1] = keyframe(30.0, "mid", None);
    input["motion"]["tracks"][0]["keyframes"][2] = keyframe(60.0, "arrive", None);

    let lowered = lower(&input);
    assert_eq!(animation(&lowered.scene).get("interpolators"), None);
    assert_eq!(x_frames(&lowered.scene)[0]["interpolation"], "linear");
    assert_builds(&lowered.scene);
}

#[test]
fn waypoint_markers_outside_the_interior_report_the_authored_path() {
    let mut input = document();
    set_waypoint(&mut input, 0, "transit");

    let error = lower_authoring_json(&input.to_string())
        .expect_err("a non-interior waypoint marker must fail");
    assert!(
        error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "waypoint_not_interior"
                && diagnostic.path == "$.motion.tracks[0].keyframes[0].waypoint"
        }),
        "diagnostics: {:#?}",
        error.diagnostics
    );
}
