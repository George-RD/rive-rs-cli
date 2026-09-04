use std::fs;
use std::path::{Path, PathBuf};

use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::render::image::analyze;
use rive_cli::render::{RenderOptions, render};

const ARTBOARD_EDGE: u32 = 128;
const SURFACE_RGBA: [u8; 4] = [194, 65, 12, 255];
const CUE_RGBA: [u8; 4] = [34, 197, 94, 255];

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/authoring/stacking-card.v0.json")
}

fn work_dir(case: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "rive-authoring-stacking-runtime-{}-{case}",
        std::process::id()
    ))
}

fn assert_pixel_near(rgba: &[u8], width: u32, x: u32, y: u32, expected: [u8; 4]) {
    let offset = usize::try_from((y * width + x) * 4).expect("pixel offset should fit usize");
    let actual = &rgba[offset..offset + 4];
    for (channel, expected_channel) in actual.iter().zip(expected) {
        assert!(
            channel.abs_diff(expected_channel) <= 2,
            "pixel ({x}, {y}) was {actual:?}, expected {expected:?}"
        );
    }
}

fn render_first_frame(case: &str, document: &str) -> (Vec<u8>, u32) {
    let lowered = lower_authoring_json(document).expect("stacking document should lower");
    let scene: SceneSpec =
        serde_json::from_value(lowered.scene).expect("lowered SceneSpec should deserialize");
    let bytes = compile_scene(&scene, fixture_path().parent(), 0).expect("scene should compile");

    let work_dir = work_dir(case);
    let _ = fs::remove_dir_all(&work_dir);
    fs::create_dir_all(&work_dir).expect("runtime work directory should be created");
    let riv_path = work_dir.join("stacking-card.riv");
    fs::write(&riv_path, &bytes).expect("compiled Rive file should be written");
    let output_dir = work_dir.join("render");

    let manifest = render(&RenderOptions {
        source_path: riv_path,
        riv: bytes,
        output_dir: output_dir.clone(),
        browser: std::env::var_os("RIVE_CHROME").map(PathBuf::from),
        width: ARTBOARD_EDGE,
        height: ARTBOARD_EDGE,
        scale: 1,
        fps: 60.0,
        frames: vec![0],
        artboard: None,
        animation: None,
        state_machine: None,
        inputs: Vec::new(),
        pointers: Vec::new(),
        background: Some("#000000".to_string()),
        preview: false,
        contact_sheet: false,
    })
    .expect("official runtime should render the stacking document");
    assert_eq!(manifest.frames.len(), 1);

    let image =
        analyze(&output_dir.join("frame_00000.png")).expect("rendered frame should decode");
    assert_eq!((image.width, image.height), (ARTBOARD_EDGE, ARTBOARD_EDGE));
    fs::remove_dir_all(work_dir).expect("runtime work directory should be removed");
    (image.rgba, image.width)
}

#[test]
fn back_to_front_stacking_keeps_foreground_visible_in_the_official_runtime() {
    let document = fs::read_to_string(fixture_path()).expect("fixture should be readable");
    let (rgba, width) = render_first_frame("back-to-front", &document);

    assert_pixel_near(&rgba, width, ARTBOARD_EDGE / 2, ARTBOARD_EDGE / 2, CUE_RGBA);
    assert_pixel_near(&rgba, width, 8, 8, SURFACE_RGBA);
}

#[test]
fn runtime_stacking_still_paints_the_first_sibling_on_top() {
    let document = fs::read_to_string(fixture_path()).expect("fixture should be readable");
    let runtime_order = document.replace("\"stacking\": \"back_to_front\"", "\"stacking\": \"runtime\"");
    assert!(!runtime_order.contains("back_to_front"));
    let (rgba, width) = render_first_frame("runtime", &runtime_order);

    assert_pixel_near(
        &rgba,
        width,
        ARTBOARD_EDGE / 2,
        ARTBOARD_EDGE / 2,
        SURFACE_RGBA,
    );
}
