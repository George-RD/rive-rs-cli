use std::fs;
use std::path::{Path, PathBuf};

use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::render::image::analyze;
use rive_cli::render::{RenderOptions, render};

const STAGE_WIDTH: u32 = 240;
const STAGE_HEIGHT: u32 = 160;
const STATE_MACHINE: &str = "auth__blend_2dmeter__meter__state_machine";
const LOAD_INPUT: &str = "auth__blend_2dmeter__meter__load__input";
const CALM_CENTRE: f64 = 40.0;
const SURGE_CENTRE: f64 = 200.0;
const CENTRE_TOLERANCE: f64 = 3.0;

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/authoring/blend-meter.v0.json")
}

fn needle_centre_x(rgba: &[u8], width: u32) -> f64 {
    let mut total = 0.0;
    let mut count = 0.0;
    for (index, pixel) in rgba.chunks_exact(4).enumerate() {
        if pixel[1] > pixel[0].saturating_add(48) && pixel[1] > pixel[2].saturating_add(48) {
            total += f64::from(u32::try_from(index).unwrap_or_default() % width);
            count += 1.0;
        }
    }
    assert!(count > 0.0, "the needle should be visible in every frame");
    total / count
}

fn needle_centre_at(load: u32) -> f64 {
    let document = fs::read_to_string(fixture_path()).expect("fixture should be readable");
    let lowered = lower_authoring_json(&document).expect("blend fixture should lower");
    let scene: SceneSpec =
        serde_json::from_value(lowered.scene).expect("lowered SceneSpec should deserialize");
    let bytes = compile_scene(&scene, fixture_path().parent(), 0).expect("scene should compile");

    let work_dir = std::env::temp_dir().join(format!(
        "rive-authoring-blend-runtime-{}-{load}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&work_dir);
    fs::create_dir_all(&work_dir).expect("runtime work directory should be created");
    let riv_path = work_dir.join("blend-meter.riv");
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
        frames: vec![5],
        artboard: None,
        animation: None,
        state_machine: Some(STATE_MACHINE.to_string()),
        inputs: vec![format!("{LOAD_INPUT}={load}")],
        pointers: Vec::new(),
        background: Some("#000000".to_string()),
        preview: false,
        contact_sheet: false,
    })
    .expect("official runtime should render the blend fixture");
    assert_eq!(manifest.frames.len(), 1);

    let frame = analyze(&output_dir.join("frame_00005.png")).expect("frame should decode");
    let centre = needle_centre_x(&frame.rgba, frame.width);
    fs::remove_dir_all(work_dir).expect("runtime work directory should be removed");
    centre
}

#[test]
fn a_number_input_drives_the_blend_state_in_the_official_runtime() {
    let calm = needle_centre_at(0);
    let mixed = needle_centre_at(50);
    let surge = needle_centre_at(100);

    assert!(
        (calm - CALM_CENTRE).abs() <= CENTRE_TOLERANCE,
        "load 0 placed the needle at {calm}, expected {CALM_CENTRE}"
    );
    assert!(
        (surge - SURGE_CENTRE).abs() <= CENTRE_TOLERANCE,
        "load 100 placed the needle at {surge}, expected {SURGE_CENTRE}"
    );
    assert!(
        mixed > calm && mixed < surge,
        "load 50 placed the needle at {mixed}, expected a value between {calm} and {surge}"
    );
}
