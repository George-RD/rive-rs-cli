use std::fs;
use std::path::{Path, PathBuf};

use rive_cli::render::image::{ImageInfo, analyze};
use rive_cli::render::{RenderOptions, render};

const STAGE_WIDTH: u32 = 960;
const STAGE_HEIGHT: u32 = 540;
const STATE_MACHINE: &str = "auth__signal_2dweave__weave__state_machine";
const EARLY_FRAME: u32 = 0;
const LATE_FRAME: u32 = 60;
const COURIER_TRAVEL_FLOOR: f64 = 20.0;
const HALO_ROTATION_FLOOR: f64 = 2.0;
const CORE_BREATH_FLOOR: usize = 60;

fn artifact_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/authoring/signal-weave.v0.riv")
}

fn frame(frame_index: u32) -> ImageInfo {
    let riv_path = artifact_path();
    let riv = fs::read(&riv_path).expect("committed weave artifact should be readable");
    let work_dir = std::env::temp_dir().join(format!(
        "rive-authoring-weave-{}-{frame_index}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&work_dir);
    fs::create_dir_all(&work_dir).expect("runtime work directory should be created");
    let output_dir = work_dir.join("render");

    render(&RenderOptions {
        source_path: riv_path,
        riv,
        output_dir: output_dir.clone(),
        browser: std::env::var_os("RIVE_CHROME").map(PathBuf::from),
        width: STAGE_WIDTH,
        height: STAGE_HEIGHT,
        scale: 1,
        fps: 60.0,
        frames: vec![frame_index],
        artboard: None,
        animation: None,
        state_machine: Some(STATE_MACHINE.to_string()),
        inputs: Vec::new(),
        pointers: Vec::new(),
        background: None,
        preview: false,
        contact_sheet: false,
    })
    .expect("official runtime should render the weave");

    let image = analyze(&output_dir.join(format!("frame_{frame_index:05}.png")))
        .expect("weave frame should decode");
    fs::remove_dir_all(work_dir).expect("runtime work directory should be removed");
    image
}

fn region_centroid(
    image: &ImageInfo,
    columns: std::ops::Range<u32>,
    rows: std::ops::Range<u32>,
    matches: impl Fn(&[u8]) -> bool,
) -> Option<(f64, f64)> {
    let mut total_x = 0.0;
    let mut total_y = 0.0;
    let mut count = 0.0;
    for row in rows {
        for column in columns.clone() {
            let offset = usize::try_from((row * image.width + column) * 4)
                .expect("pixel offset should fit usize");
            if matches(&image.rgba[offset..offset + 4]) {
                total_x += f64::from(column);
                total_y += f64::from(row);
                count += 1.0;
            }
        }
    }
    if count == 0.0 {
        return None;
    }
    Some((total_x / count, total_y / count))
}

fn region_count(
    image: &ImageInfo,
    columns: std::ops::Range<u32>,
    rows: std::ops::Range<u32>,
    matches: impl Fn(&[u8]) -> bool,
) -> usize {
    let mut count = 0;
    for row in rows {
        for column in columns.clone() {
            let offset = usize::try_from((row * image.width + column) * 4)
                .expect("pixel offset should fit usize");
            if matches(&image.rgba[offset..offset + 4]) {
                count += 1;
            }
        }
    }
    count
}

fn is_courier(pixel: &[u8]) -> bool {
    pixel[2] > 180 && pixel[1] > 170 && pixel[0] < 120
}

fn is_spoke(pixel: &[u8]) -> bool {
    pixel[2] > 100 && pixel[0] < 70 && pixel[1] < 100 && pixel[2] - pixel[0] > 40
}

fn is_core_highlight(pixel: &[u8]) -> bool {
    pixel[0] > 170 && pixel[1] > 200 && pixel[2] > 200
}

#[test]
fn three_regions_animate_independently_in_the_official_runtime() {
    let early = frame(EARLY_FRAME);
    let late = frame(LATE_FRAME);

    let early_courier =
        region_centroid(&early, 0..STAGE_WIDTH, 488..512, is_courier).expect("courier at frame 0");
    let late_courier = region_centroid(&late, 0..STAGE_WIDTH, 488..512, is_courier)
        .expect("courier at the later frame");
    assert!(
        late_courier.0 - early_courier.0 >= COURIER_TRAVEL_FLOOR,
        "the courier moved from {} to {}",
        early_courier.0,
        late_courier.0
    );

    let early_halo =
        region_centroid(&early, 290..410, 200..360, is_spoke).expect("halo spokes at frame 0");
    let late_halo = region_centroid(&late, 290..410, 200..360, is_spoke)
        .expect("halo spokes at the later frame");
    let halo_shift =
        ((late_halo.0 - early_halo.0).powi(2) + (late_halo.1 - early_halo.1).powi(2)).sqrt();
    assert!(
        halo_shift >= HALO_ROTATION_FLOOR,
        "the halo spokes only moved {halo_shift} px"
    );

    let early_core = region_count(&early, 420..540, 220..340, is_core_highlight);
    let late_core = region_count(&late, 420..540, 220..340, is_core_highlight);
    assert!(
        early_core.abs_diff(late_core) >= CORE_BREATH_FLOOR,
        "the core highlight covered {early_core} then {late_core} pixels"
    );
}
