use std::fs;
use std::path::{Path, PathBuf};

use rive_cli::render::image::{ImageInfo, analyze};
use rive_cli::render::{RenderOptions, render};

const STAGE_WIDTH: u32 = 960;
const STAGE_HEIGHT: u32 = 540;
const STATE_MACHINE: &str = "auth__throughput_2dconsole__console__state_machine";
const LOAD_INPUT: &str = "auth__throughput_2dconsole__console__load__input";
const ARM_POINTER: &str = "down:330,432@5";
const ARM_RELEASE: &str = "up:330,432@8";
const NEEDLE_SCAN_ROW: u32 = 328;
const STANDBY_NEEDLE_LIMIT: f64 = 200.0;
const ENGAGED_NEEDLE_FLOOR: f64 = 600.0;

fn artifact_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/authoring/interactive-console.v0.riv")
}

fn render_console(case: &str, load: u32, pointers: &[&str], frame: u32) -> ImageInfo {
    let riv_path = artifact_path();
    let riv = fs::read(&riv_path).expect("committed console artifact should be readable");
    let work_dir = std::env::temp_dir().join(format!(
        "rive-authoring-console-{}-{case}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&work_dir);
    fs::create_dir_all(&work_dir).expect("runtime work directory should be created");
    let output_dir = work_dir.join("render");

    let manifest = render(&RenderOptions {
        source_path: riv_path,
        riv,
        output_dir: output_dir.clone(),
        browser: std::env::var_os("RIVE_CHROME").map(PathBuf::from),
        width: STAGE_WIDTH,
        height: STAGE_HEIGHT,
        scale: 1,
        fps: 60.0,
        frames: vec![frame],
        artboard: None,
        animation: None,
        state_machine: Some(STATE_MACHINE.to_string()),
        inputs: vec![format!("{LOAD_INPUT}={load}")],
        pointers: pointers.iter().map(|entry| (*entry).to_string()).collect(),
        background: None,
        preview: false,
        contact_sheet: false,
    })
    .expect("official runtime should render the console");
    assert_eq!(manifest.frames.len(), 1);

    let image = analyze(&output_dir.join(format!("frame_{frame:05}.png")))
        .expect("console frame should decode");
    fs::remove_dir_all(work_dir).expect("runtime work directory should be removed");
    image
}

fn needle_centre_x(image: &ImageInfo) -> f64 {
    let mut total = 0.0;
    let mut count = 0.0;
    let row = usize::try_from(NEEDLE_SCAN_ROW * image.width).expect("row offset should fit usize");
    for column in 0..image.width {
        let offset = (row + usize::try_from(column).unwrap_or_default()) * 4;
        let pixel = &image.rgba[offset..offset + 4];
        if pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200 {
            total += f64::from(column);
            count += 1.0;
        }
    }
    assert!(count > 0.0, "the gauge needle should be visible");
    total / count
}

fn lamp_coverage(image: &ImageInfo) -> usize {
    image
        .rgba
        .chunks_exact(4)
        .filter(|pixel| pixel[0] > 150 && (100..200).contains(&pixel[1]) && pixel[2] < 80)
        .count()
}

fn token_centre_x(image: &ImageInfo) -> f64 {
    let mut total = 0.0;
    let mut count = 0.0;
    for (index, pixel) in image.rgba.chunks_exact(4).enumerate() {
        if pixel[2] > 180 && pixel[1] > 150 && pixel[0] < 120 {
            total += f64::from(u32::try_from(index).unwrap_or_default() % image.width);
            count += 1.0;
        }
    }
    assert!(count > 0.0, "the stream token should be visible");
    total / count
}

#[test]
fn pointer_arming_moves_the_gauge_onto_the_blended_load() {
    let standby = render_console("standby", 90, &[], 45);
    let engaged = render_console("engaged", 90, &[ARM_POINTER, ARM_RELEASE], 45);

    assert!(
        needle_centre_x(&standby) <= STANDBY_NEEDLE_LIMIT,
        "standby left the needle at {}",
        needle_centre_x(&standby)
    );
    assert!(
        needle_centre_x(&engaged) >= ENGAGED_NEEDLE_FLOOR,
        "arming left the needle at {}",
        needle_centre_x(&engaged)
    );
}

#[test]
fn the_alert_region_escalates_on_its_own_number_condition() {
    let calm = render_console("calm", 10, &[], 45);
    let busy = render_console("busy", 90, &[], 45);

    assert!(
        lamp_coverage(&busy) > lamp_coverage(&calm),
        "load 90 covered {} lamp pixels, load 10 covered {}",
        lamp_coverage(&busy),
        lamp_coverage(&calm)
    );
}

#[test]
fn the_stream_region_runs_while_the_primary_region_stays_in_standby() {
    let early = render_console("stream-early", 0, &[], 10);
    let later = render_console("stream-later", 0, &[], 60);

    assert!(
        token_centre_x(&later) > token_centre_x(&early) + 40.0,
        "the token moved from {} to {}",
        token_centre_x(&early),
        token_centre_x(&later)
    );
}
