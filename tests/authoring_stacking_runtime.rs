use std::fs;
use std::path::{Path, PathBuf};

use rive_cli::authoring::lower_authoring_json;
use rive_cli::builder::SceneSpec;
use rive_cli::compile::compile_scene;
use rive_cli::render::image::analyze;
use rive_cli::render::{RenderOptions, render};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/authoring/stacking-card.v0.json")
}

fn work_dir() -> PathBuf {
    std::env::temp_dir().join(format!(
        "rive-authoring-stacking-runtime-{}",
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

#[test]
#[ignore = "requires Chromium and the official Rive runtime"]
fn back_to_front_stacking_keeps_foreground_visible_in_the_official_runtime() {
    let fixture = fixture_path();
    let input = fs::read_to_string(&fixture).expect("stacking fixture should be readable");
    let lowered = lower_authoring_json(&input).expect("stacking fixture should lower");
    let scene: SceneSpec =
        serde_json::from_value(lowered.scene).expect("lowered SceneSpec should deserialize");
    let bytes = compile_scene(&scene, fixture.parent(), 0).expect("scene should compile");

    let work_dir = work_dir();
    let _ = fs::remove_dir_all(&work_dir);
    fs::create_dir_all(&work_dir).expect("runtime work directory should be created");
    let riv_path = work_dir.join("stacking-card.riv");
    fs::write(&riv_path, bytes).expect("compiled Rive file should be written");
    let output_dir = work_dir.join("render");
    let browser = std::env::var_os("RIVE_CHROME").map(PathBuf::from);

    let manifest = render(&RenderOptions {
        source_path: riv_path.clone(),
        riv: riv_path,
        output_dir: output_dir.clone(),
        browser,
        width: 128,
        height: 128,
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
    .expect("official runtime should render the stacking fixture");
    assert_eq!(manifest.frames.len(), 1);

    let image = analyze(&output_dir.join("frame_00000.png"))
        .expect("rendered stacking frame should decode");
    assert_eq!((image.width, image.height), (128, 128));
    assert_pixel_near(&image.rgba, image.width, 64, 64, [34, 197, 94, 255]);
    assert_pixel_near(&image.rgba, image.width, 8, 8, [194, 65, 12, 255]);

    fs::remove_dir_all(work_dir).expect("runtime work directory should be removed");
}
