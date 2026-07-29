use std::path::Path;

use crate::render::{self, RenderOptions, RenderedFrame};

use super::model::{RuntimeEvidence, RuntimeExpectations};

pub fn evaluate_runtime_frames(
    expectations: &RuntimeExpectations,
    frames: &[RenderedFrame],
    manifest_path: &Path,
) -> RuntimeEvidence {
    let rendered_frame_count = frames.len();
    let rendered_frame_indices = frames.iter().map(|frame| frame.index).collect::<Vec<_>>();
    let non_blank_frame_count = frames.iter().filter(|frame| !frame.blank).count();
    let minimum_distinct_colors_observed = frames
        .iter()
        .map(|frame| frame.distinct_colors)
        .min()
        .unwrap_or_default();

    let mut failures = Vec::new();
    if rendered_frame_count != expectations.frames.len() {
        failures.push(format!(
            "rendered {rendered_frame_count} frame(s), expected {}",
            expectations.frames.len()
        ));
    }
    if rendered_frame_indices != expectations.frames {
        failures.push(format!(
            "rendered frame indices {rendered_frame_indices:?}, expected {:?}",
            expectations.frames
        ));
    }
    if non_blank_frame_count < expectations.min_non_blank_frames {
        failures.push(format!(
            "{non_blank_frame_count} non-blank frame(s), expected at least {}",
            expectations.min_non_blank_frames
        ));
    }
    if minimum_distinct_colors_observed < expectations.min_distinct_colors {
        failures.push(format!(
            "minimum distinct colors {minimum_distinct_colors_observed}, expected at least {}",
            expectations.min_distinct_colors
        ));
    }

    RuntimeEvidence {
        passed: failures.is_empty(),
        rendered_frame_count,
        non_blank_frame_count,
        minimum_distinct_colors_observed,
        manifest_path: Some(manifest_path.display().to_string()),
        failure_reason: (!failures.is_empty()).then(|| failures.join("; ")),
    }
}

pub fn failed_runtime_evidence(reason: impl Into<String>) -> RuntimeEvidence {
    RuntimeEvidence {
        passed: false,
        rendered_frame_count: 0,
        non_blank_frame_count: 0,
        minimum_distinct_colors_observed: 0,
        manifest_path: None,
        failure_reason: Some(reason.into()),
    }
}

pub fn render_runtime_evidence(
    case_dir: &Path,
    riv: &[u8],
    expectations: &RuntimeExpectations,
) -> RuntimeEvidence {
    let output_dir = case_dir.join("render");
    let manifest_path = output_dir.join("manifest.json");
    let options = RenderOptions {
        riv: riv.to_vec(),
        source_path: case_dir.join("output.riv"),
        output_dir,
        frames: expectations.frames.clone(),
        fps: expectations.fps,
        animation: expectations.animation.clone(),
        state_machine: expectations.state_machine.clone(),
        inputs: Vec::new(),
        pointers: Vec::new(),
        artboard: expectations.artboard.clone(),
        width: expectations.width,
        height: expectations.height,
        scale: expectations.scale,
        background: expectations.background.clone(),
        contact_sheet: false,
        preview: false,
        browser: None,
    };

    match render::render(&options) {
        Ok(manifest) => evaluate_runtime_frames(expectations, &manifest.frames, &manifest_path),
        Err(error) => failed_runtime_evidence(format!("runtime render failed: {error}")),
    }
}
