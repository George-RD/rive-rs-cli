use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::render::image::{analyze, pixel_difference};
use crate::render::{RenderError, RenderManifest, RenderOptions, RenderedFrame, render};
use crate::validator::{InspectFilter, RivObject, parse_riv};

const COMPARE_FPS: f64 = 60.0;

pub struct CompareOptions {
    pub reference: PathBuf,
    pub candidate: PathBuf,
    pub frames: Vec<u32>,
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub background: Option<String>,
    pub reference_animation: Option<String>,
    pub candidate_animation: Option<String>,
    pub reference_state_machine: Option<String>,
    pub candidate_state_machine: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypeDelta {
    pub type_name: String,
    pub reference: usize,
    pub candidate: usize,
    pub delta: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FrameComparison {
    pub index: u32,
    pub pixel_difference: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompareReport {
    pub reference: String,
    pub candidate: String,
    pub reference_object_count: usize,
    pub candidate_object_count: usize,
    pub type_deltas: Vec<TypeDelta>,
    pub frames: Vec<FrameComparison>,
    pub max_pixel_difference: f64,
    pub missing_type_names: Vec<String>,
}

struct ScratchDir {
    path: PathBuf,
}

impl ScratchDir {
    fn new() -> Result<Self, RenderError> {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|elapsed| elapsed.as_nanos())
            .unwrap_or_default();
        let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rive-compare-{}-{nanos}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }
}

impl Drop for ScratchDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub fn compare(options: &CompareOptions) -> Result<CompareReport, RenderError> {
    if options.frames.is_empty() {
        return Err(RenderError::Message(
            "at least one frame must be requested".to_string(),
        ));
    }

    let reference_bytes = read_riv(&options.reference)?;
    let candidate_bytes = read_riv(&options.candidate)?;

    let reference_parsed =
        parse_riv(&reference_bytes, &InspectFilter::default()).map_err(|error| {
            RenderError::Message(format!(
                "could not decompile the reference file {}: {error}",
                options.reference.display()
            ))
        })?;
    let candidate_parsed =
        parse_riv(&candidate_bytes, &InspectFilter::default()).map_err(|error| {
            RenderError::Message(format!(
                "could not decompile the candidate file {}: {error}",
                options.candidate.display()
            ))
        })?;

    let reference_counts = type_counts(&reference_parsed.objects);
    let candidate_counts = type_counts(&candidate_parsed.objects);
    let type_deltas = type_deltas(&reference_counts, &candidate_counts);
    let missing_type_names = type_deltas
        .iter()
        .filter(|row| row.candidate == 0 && row.reference > 0)
        .map(|row| row.type_name.clone())
        .collect();

    let scratch = ScratchDir::new()?;
    let reference_dir = scratch.path.join("reference");
    let candidate_dir = scratch.path.join("candidate");

    let reference_manifest = render_side(
        options,
        &options.reference,
        reference_bytes,
        &reference_dir,
        options.reference_animation.clone(),
        options.reference_state_machine.clone(),
        "reference",
    )?;
    let candidate_manifest = render_side(
        options,
        &options.candidate,
        candidate_bytes,
        &candidate_dir,
        options.candidate_animation.clone(),
        options.candidate_state_machine.clone(),
        "candidate",
    )?;

    let mut frames = Vec::with_capacity(options.frames.len());
    let mut max_pixel_difference = 0.0_f64;
    for (position, &index) in options.frames.iter().enumerate() {
        let reference_frame = captured_frame(&reference_manifest, position, index, "reference")?;
        let candidate_frame = captured_frame(&candidate_manifest, position, index, "candidate")?;
        let left = analyze(&reference_dir.join(&reference_frame.filename))?;
        let right = analyze(&candidate_dir.join(&candidate_frame.filename))?;
        let difference = pixel_difference(&left, &right)?;
        max_pixel_difference = max_pixel_difference.max(difference);
        frames.push(FrameComparison {
            index,
            pixel_difference: difference,
        });
    }

    Ok(CompareReport {
        reference: options.reference.to_string_lossy().into_owned(),
        candidate: options.candidate.to_string_lossy().into_owned(),
        reference_object_count: reference_parsed.objects.len(),
        candidate_object_count: candidate_parsed.objects.len(),
        type_deltas,
        frames,
        max_pixel_difference,
        missing_type_names,
    })
}

fn read_riv(path: &Path) -> Result<Vec<u8>, RenderError> {
    fs::read(path).map_err(|error| {
        RenderError::Message(format!("could not read {}: {error}", path.display()))
    })
}

fn captured_frame<'manifest>(
    manifest: &'manifest RenderManifest,
    position: usize,
    index: u32,
    side: &str,
) -> Result<&'manifest RenderedFrame, RenderError> {
    match manifest.frames.get(position) {
        Some(frame) if frame.index == index => Ok(frame),
        Some(frame) => Err(RenderError::Message(format!(
            "the {side} render returned frame {} where frame {index} was requested",
            frame.index
        ))),
        None => Err(RenderError::Message(format!(
            "the {side} render returned {} frames but frame {index} was requested",
            manifest.frames.len()
        ))),
    }
}

#[allow(clippy::too_many_arguments)]
fn render_side(
    options: &CompareOptions,
    source_path: &Path,
    riv: Vec<u8>,
    output_dir: &Path,
    animation: Option<String>,
    state_machine: Option<String>,
    side: &str,
) -> Result<RenderManifest, RenderError> {
    let render_options = RenderOptions {
        riv,
        source_path: source_path.to_path_buf(),
        output_dir: output_dir.to_path_buf(),
        frames: options.frames.clone(),
        fps: COMPARE_FPS,
        animation,
        state_machine,
        inputs: Vec::new(),
        pointers: Vec::new(),
        artboard: None,
        width: options.width,
        height: options.height,
        scale: options.scale,
        background: options.background.clone(),
        contact_sheet: false,
        preview: false,
        browser: None,
    };
    render(&render_options).map_err(|error| {
        RenderError::Message(format!(
            "could not render the {side} file {}: {error}",
            source_path.display()
        ))
    })
}

fn type_counts(objects: &[RivObject]) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for object in objects {
        let name = object
            .type_name
            .clone()
            .unwrap_or_else(|| format!("unknown({})", object.type_key));
        *counts.entry(name).or_insert(0) += 1;
    }
    counts
}

fn type_deltas(
    reference: &BTreeMap<String, usize>,
    candidate: &BTreeMap<String, usize>,
) -> Vec<TypeDelta> {
    reference
        .keys()
        .chain(candidate.keys())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .map(|type_name| {
            let reference_count = reference.get(type_name).copied().unwrap_or(0);
            let candidate_count = candidate.get(type_name).copied().unwrap_or(0);
            TypeDelta {
                type_name: type_name.clone(),
                reference: reference_count,
                candidate: candidate_count,
                delta: candidate_count as i64 - reference_count as i64,
            }
        })
        .collect()
}

pub fn compare_report_text(report: &CompareReport) -> String {
    let mut text = String::new();
    text.push_str("Structural\n");
    text.push_str(&format!(
        "  reference  {}  {} objects\n",
        report.reference, report.reference_object_count
    ));
    text.push_str(&format!(
        "  candidate  {}  {} objects\n",
        report.candidate, report.candidate_object_count
    ));

    let rows: Vec<&TypeDelta> = report
        .type_deltas
        .iter()
        .filter(|row| row.delta != 0)
        .collect();
    if rows.is_empty() {
        text.push_str("  every type name appears the same number of times in both files\n");
    } else {
        let name_width = rows
            .iter()
            .map(|row| row.type_name.len())
            .chain(std::iter::once("type".len()))
            .max()
            .unwrap_or(4);
        text.push_str(&format!(
            "  {:<name_width$}  {:>9}  {:>9}  {:>6}\n",
            "type", "reference", "candidate", "delta"
        ));
        for row in rows {
            text.push_str(&format!(
                "  {:<name_width$}  {:>9}  {:>9}  {:>+6}\n",
                row.type_name, row.reference, row.candidate, row.delta
            ));
        }
    }

    text.push_str("\nVisual\n");
    text.push_str(&format!("  {:>5}  {:>10}\n", "frame", "diff %"));
    for frame in &report.frames {
        text.push_str(&format!(
            "  {:>5}  {:>9.4}%\n",
            frame.index, frame.pixel_difference
        ));
    }

    text.push_str("\nVerdict\n");
    text.push_str(&format!(
        "  max pixel difference   {:.4}%\n",
        report.max_pixel_difference
    ));
    text.push_str(&format!(
        "  type names missing from the candidate   {}\n",
        report.missing_type_names.len()
    ));
    if !report.missing_type_names.is_empty() {
        text.push_str(&format!("  {}\n", report.missing_type_names.join(", ")));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::image::ImageInfo;

    fn image(width: u32, height: u32, rgba: Vec<u8>) -> ImageInfo {
        ImageInfo {
            width,
            height,
            distinct_colors: 1,
            blank: false,
            rgba,
        }
    }

    #[test]
    fn identical_images_have_no_difference() {
        let left = image(2, 1, vec![10, 20, 30, 255, 40, 50, 60, 255]);
        let right = image(2, 1, vec![10, 20, 30, 255, 40, 50, 60, 255]);
        assert_eq!(pixel_difference(&left, &right).unwrap_or(-1.0), 0.0);
    }

    #[test]
    fn any_channel_delta_counts_as_different() {
        let left = image(1, 1, vec![10, 20, 30, 255]);
        let right = image(1, 1, vec![11, 20, 30, 255]);
        assert_eq!(pixel_difference(&left, &right).unwrap_or(-1.0), 100.0);
    }

    #[test]
    fn difference_is_a_percentage_of_all_pixels() {
        let left = image(2, 2, vec![0; 16]);
        let mut right_pixels = vec![0u8; 16];
        right_pixels[0] = 200;
        let right = image(2, 2, right_pixels);
        assert_eq!(pixel_difference(&left, &right).unwrap_or(-1.0), 25.0);
    }

    #[test]
    fn mismatched_dimensions_are_an_error() {
        let left = image(2, 1, vec![0; 8]);
        let right = image(1, 2, vec![0; 8]);
        let error = pixel_difference(&left, &right)
            .err()
            .map(|error| error.to_string())
            .unwrap_or_default();
        assert_eq!(error, "cannot compare 2x1 against 1x2");
    }

    #[test]
    fn type_deltas_cover_both_sides() {
        let mut reference = BTreeMap::new();
        reference.insert("Fill".to_string(), 2);
        reference.insert("Shape".to_string(), 1);
        let mut candidate = BTreeMap::new();
        candidate.insert("Shape".to_string(), 1);
        candidate.insert("Stroke".to_string(), 3);

        let rows = type_deltas(&reference, &candidate);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].type_name, "Fill");
        assert_eq!(rows[0].delta, -2);
        assert_eq!(rows[1].type_name, "Shape");
        assert_eq!(rows[1].delta, 0);
        assert_eq!(rows[2].type_name, "Stroke");
        assert_eq!(rows[2].delta, 3);
    }

    #[test]
    fn zero_delta_rows_are_omitted_from_the_text_report() {
        let report = CompareReport {
            reference: "a.riv".to_string(),
            candidate: "b.riv".to_string(),
            reference_object_count: 3,
            candidate_object_count: 4,
            type_deltas: vec![
                TypeDelta {
                    type_name: "Shape".to_string(),
                    reference: 1,
                    candidate: 1,
                    delta: 0,
                },
                TypeDelta {
                    type_name: "Stroke".to_string(),
                    reference: 0,
                    candidate: 1,
                    delta: 1,
                },
            ],
            frames: vec![FrameComparison {
                index: 0,
                pixel_difference: 1.5,
            }],
            max_pixel_difference: 1.5,
            missing_type_names: Vec::new(),
        };
        let text = compare_report_text(&report);
        assert!(!text.contains("Shape"));
        assert!(text.contains("Stroke"));
        assert!(text.contains("1.5000%"));
    }
}
