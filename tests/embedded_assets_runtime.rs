mod support;

use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use rive_cli::builder::SceneSpec;
use rive_cli::compile::assets::MemoryAssets;
use rive_cli::compile::{CompileOptions, compile_scene_with_assets};
use rive_cli::render::image::analyze;
use rive_cli::render::{RenderOptions, render};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use support::WorkDir;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;
const TEXT_REGION_START: u32 = 200;
const MIN_CHANGED_PIXELS: usize = 100;
const REPLACEMENT_PIXEL: [u8; 4] = [232, 32, 96, 255];
const FONT: &[u8] = include_bytes!("../assets/fonts/Inter-Bold-Subset.ttf");
const IMAGE: &[u8] = include_bytes!("../assets/textures/aurora.png");

fn document() -> Value {
    let mut document: Value = serde_json::from_str(include_str!("fixtures/embedded_assets.json"))
        .expect("embedded scene fixture");
    document["artboard"]["children"][0]["source"] = json!("memory://font");
    document["artboard"]["children"][1]["source"] = json!("memory://image");
    document
}

fn replacement_image() -> Vec<u8> {
    let reader = png::Decoder::new(Cursor::new(IMAGE))
        .read_info()
        .expect("embedded image dimensions");
    let info = reader.info();
    let pixels = REPLACEMENT_PIXEL.repeat(info.width as usize * info.height as usize);
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, info.width, info.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().expect("replacement PNG header");
        writer
            .write_image_data(&pixels)
            .expect("replacement PNG pixels");
    }
    bytes
}

fn frame(root: &Path, name: &str, document: Value, image_bytes: &[u8]) -> Vec<u8> {
    let scene: SceneSpec = serde_json::from_value(document).expect("runtime scene");
    let mut assets = MemoryAssets::new();
    assets.insert("memory://font", FONT);
    assets.insert("memory://image", image_bytes);
    let bytes = compile_scene_with_assets(&scene, CompileOptions::default(), &assets)
        .expect("memory-only compilation");
    let output_dir = root.join(name);
    fs::create_dir_all(&output_dir).expect("evidence directory");
    let source_path = output_dir.join("scene.riv");
    fs::write(&source_path, &bytes).expect("test harness writes artifact, not compiler");
    let manifest = render(&RenderOptions {
        source_path,
        riv: bytes,
        output_dir: output_dir.clone(),
        browser: std::env::var_os("RIVE_CHROME").map(PathBuf::from),
        width: WIDTH,
        height: HEIGHT,
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
    .expect("official runtime accepts the compiled file");
    assert_eq!(manifest.frames.len(), 1);
    let image = analyze(&output_dir.join("frame_00000.png")).expect("runtime PNG");
    assert_eq!((image.width, image.height), (WIDTH, HEIGHT));
    image.rgba
}

fn differences(left: &[u8], right: &[u8], start_y: u32, end_y: u32) -> usize {
    let start = (start_y * WIDTH * 4) as usize;
    let end = (end_y * WIDTH * 4) as usize;
    let (left, _) = left[start..end].as_chunks::<4>();
    let (right, _) = right[start..end].as_chunks::<4>();
    left.iter().zip(right).filter(|(a, b)| a != b).count()
}

#[test]
fn supplied_image_and_font_both_drive_visible_official_runtime_output() {
    let temporary = WorkDir::new("rive-memory-assets-runtime");
    let root = std::env::var_os("RIVE_MEMORY_ASSET_EVIDENCE")
        .map(PathBuf::from)
        .unwrap_or_else(|| temporary.path().to_path_buf());
    fs::create_dir_all(&root).expect("evidence root");
    let full = frame(&root, "full", document(), IMAGE);
    let repeat = frame(&root, "repeat", document(), IMAGE);
    assert_eq!(
        full, repeat,
        "memory compilation and rendering are repeatable"
    );

    let replacement = replacement_image();
    let replaced = frame(&root, "replacement-image-bytes", document(), &replacement);
    let replacement_pixels = differences(&full, &replaced, 0, TEXT_REGION_START);
    assert!(
        replacement_pixels > MIN_CHANGED_PIXELS,
        "changing only the supplied image buffer must change rendered pixels: {replacement_pixels}"
    );
    assert_eq!(differences(&full, &replaced, TEXT_REGION_START, HEIGHT), 0);

    let mut no_image = document();
    no_image["artboard"]["children"]
        .as_array_mut()
        .expect("children")
        .remove(2);
    let without_image = frame(&root, "without-image", no_image, IMAGE);

    let mut no_text = document();
    no_text["artboard"]["children"]
        .as_array_mut()
        .expect("children")
        .remove(3);
    let without_text = frame(&root, "without-text", no_text, IMAGE);

    let mut no_font = document();
    no_font["artboard"]["children"][0]
        .as_object_mut()
        .expect("font")
        .remove("source");
    let without_font = frame(&root, "without-font-bytes", no_font, IMAGE);

    let image_pixels = differences(&full, &without_image, 0, TEXT_REGION_START);
    let text_pixels = differences(&full, &without_text, TEXT_REGION_START, HEIGHT);
    let font_pixels = differences(&full, &without_font, TEXT_REGION_START, HEIGHT);
    assert!(
        image_pixels > MIN_CHANGED_PIXELS,
        "embedded image must be visible: {image_pixels}"
    );
    assert!(
        text_pixels > MIN_CHANGED_PIXELS,
        "glyphs must be visible: {text_pixels}"
    );
    assert_eq!(
        font_pixels, text_pixels,
        "without supplied font bytes, the glyphs must disappear"
    );
    assert_eq!(
        without_font, without_text,
        "no unrelated font fallback can reproduce the text"
    );
    assert_eq!(differences(&full, &without_font, 0, TEXT_REGION_START), 0);

    let summary = json!({
        "compiler_inputs": "SceneSpec and caller-owned memory image/font bytes; no base directory",
        "font_sha256": format!("{:x}", Sha256::digest(FONT)),
        "image_sha256": format!("{:x}", Sha256::digest(IMAGE)),
        "replacement_image_sha256": format!("{:x}", Sha256::digest(&replacement)),
        "runtime_js_sha256": format!("{:x}", Sha256::digest(include_bytes!("../assets/rive.js"))),
        "runtime_wasm_sha256": format!("{:x}", Sha256::digest(include_bytes!("../assets/rive.wasm"))),
        "image_changed_pixels": image_pixels,
        "replacement_image_changed_pixels": replacement_pixels,
        "replacement_image_text_region_changed_pixels": 0,
        "replacement_image_same_scene_and_dimensions": true,
        "text_changed_pixels": text_pixels,
        "missing_font_changed_pixels": font_pixels,
        "repeat_pixels_identical": true,
        "missing_font_matches_removed_text": true,
        "frames": ["full/frame_00000.png", "replacement-image-bytes/frame_00000.png", "without-image/frame_00000.png", "without-text/frame_00000.png", "without-font-bytes/frame_00000.png"]
    });
    fs::write(
        root.join("evidence.json"),
        serde_json::to_vec_pretty(&summary).expect("evidence JSON"),
    )
    .expect("retain runtime evidence");
}
