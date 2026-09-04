mod support;

use std::fs;
use std::path::{Path, PathBuf};

use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::render::image::analyze;
use rive_cli::render::{RenderOptions, render};
use serde_json::Value;
use support::WorkDir;

const STAGE_WIDTH: u32 = 320;
const STAGE_HEIGHT: u32 = 160;
const WAYPOINT_FRAME: u32 = 30;
const APPROACH_FRAME: u32 = 26;
const CONSTANT_SPEED_TRAVEL: f64 = 12.0;
const STOPPED_TRAVEL: f64 = 2.0;

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/authoring/waypoint-transit.v0.json")
}

fn without_continuity() -> String {
    let document = fs::read_to_string(fixture_path()).expect("fixture should be readable");
    let mut document: Value =
        serde_json::from_str(&document).expect("fixture should parse as JSON");
    let tracks = document["motion"]["tracks"]
        .as_array_mut()
        .expect("fixture should declare motion tracks");
    let mut removed = 0;
    for track in tracks {
        let track = track
            .as_object_mut()
            .expect("each track should be an object");
        if track.remove("continuity").is_some() {
            removed += 1;
        }
    }
    assert!(removed > 0, "fixture should declare continuity to remove");
    document.to_string()
}

fn token_centre_x(rgba: &[u8], width: u32) -> f64 {
    let mut total = 0.0;
    let mut count = 0.0;
    for (index, pixel) in rgba.as_chunks::<4>().0.iter().enumerate() {
        if pixel[1] > pixel[0].saturating_add(48) && pixel[1] > pixel[2].saturating_add(48) {
            total += f64::from(u32::try_from(index).unwrap_or_default() % width);
            count += 1.0;
        }
    }
    assert!(count > 0.0, "the token should be visible in every frame");
    total / count
}

fn travel_into_the_waypoint(case: &str, document: &str) -> f64 {
    let lowered = lower_authoring_json(document).expect("waypoint document should lower");
    let scene: SceneSpec =
        serde_json::from_value(lowered.scene).expect("lowered SceneSpec should deserialize");
    let bytes = compile_scene(&scene, fixture_path().parent(), 0).expect("scene should compile");

    let work_dir = WorkDir::new(&format!("rive-authoring-continuity-{case}"));
    let riv_path = work_dir.join("waypoint-transit.riv");
    fs::write(&riv_path, &bytes).expect("compiled Rive file should be written");
    let output_dir = work_dir.join("render");

    let manifest = render(&RenderOptions {
        source_path: riv_path,
        riv: bytes,
        output_dir: output_dir.clone(),
        browser: std::env::var_os("RIVE_CHROME").map(PathBuf::from),
        width: STAGE_WIDTH,
        height: STAGE_HEIGHT,
        scale: 1,
        fps: 60.0,
        frames: vec![APPROACH_FRAME, WAYPOINT_FRAME],
        artboard: None,
        animation: None,
        state_machine: None,
        inputs: Vec::new(),
        pointers: Vec::new(),
        background: Some("#000000".to_string()),
        preview: false,
        contact_sheet: false,
    })
    .expect("official runtime should render the waypoint document");
    assert_eq!(manifest.frames.len(), 2);

    let approach = analyze(&output_dir.join(format!("frame_{APPROACH_FRAME:05}.png")))
        .expect("approach frame should decode");
    let waypoint = analyze(&output_dir.join(format!("frame_{WAYPOINT_FRAME:05}.png")))
        .expect("waypoint frame should decode");
    token_centre_x(&waypoint.rgba, waypoint.width) - token_centre_x(&approach.rgba, approach.width)
}

#[test]
fn through_continuity_keeps_the_token_moving_into_the_waypoint() {
    let document = fs::read_to_string(fixture_path()).expect("fixture should be readable");
    let travel = travel_into_the_waypoint("through", &document);

    assert!(
        travel >= CONSTANT_SPEED_TRAVEL,
        "through continuity moved {travel} px into the waypoint; expected at least {CONSTANT_SPEED_TRAVEL}"
    );
}

#[test]
fn per_keyframe_easing_stops_the_token_at_the_waypoint() {
    let travel = travel_into_the_waypoint("per-keyframe", &without_continuity());

    assert!(
        travel <= STOPPED_TRAVEL,
        "shared ease-out moved {travel} px into the waypoint; expected at most {STOPPED_TRAVEL}"
    );
}
