use std::collections::{HashMap, HashSet};

use super::super::super::spec::{AuthoringDiagnostic, MotionInterpolation, MotionSection};
use super::super::validate_id;
use super::property;

const MAX_EASINGS: usize = 1_000;
const MAX_POSES: usize = 1_000;
const MAX_POSE_TARGETS: usize = 1_000;
const MAX_TRACKS: usize = 1_000;
const MAX_TRACK_KEYFRAMES: usize = 1_000;
const MAX_EXPANDED_MOTION_KEYFRAMES: u64 = 10_000;
const MAX_FPS: u64 = 240;

pub(in crate::authoring::frontend) fn validate_motion(
    motion: &MotionSection,
) -> Vec<AuthoringDiagnostic> {
    let mut diagnostics = Vec::new();
    validate_count(
        motion.easings.len(),
        0,
        MAX_EASINGS,
        "$.motion.easings",
        "motion_easing_limit",
        "motion easing count",
        &mut diagnostics,
    );
    let mut easing_ids = HashSet::new();
    for (easing_index, easing) in motion.easings.iter().enumerate() {
        let easing_path = format!("$.motion.easings[{easing_index}]");
        validate_id(easing.id(), &format!("{easing_path}.id"), &mut diagnostics);
        if !easing_ids.insert(easing.id()) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{easing_path}.id"),
                "duplicate_easing",
                format!("motion easing id '{}' is duplicated", easing.id()),
            ));
        }
    }

    validate_count(
        motion.poses.len(),
        0,
        MAX_POSES,
        "$.motion.poses",
        "motion_pose_limit",
        "motion pose count",
        &mut diagnostics,
    );

    let mut pose_ids = HashSet::new();
    let mut pose_property_counts: HashMap<&str, u64> = HashMap::new();
    for (pose_index, pose) in motion.poses.iter().enumerate() {
        let pose_path = format!("$.motion.poses[{pose_index}]");
        validate_id(&pose.id, &format!("{pose_path}.id"), &mut diagnostics);
        if !pose_ids.insert(pose.id.as_str()) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{pose_path}.id"),
                "duplicate_pose",
                format!("pose id '{}' is duplicated", pose.id),
            ));
        }
        validate_count(
            pose.targets.len(),
            1,
            MAX_POSE_TARGETS,
            &format!("{pose_path}.targets"),
            "motion_pose_target_limit",
            "pose target count",
            &mut diagnostics,
        );

        let mut target_ids = HashSet::new();
        let mut property_count = 0_u64;
        for (target_index, target) in pose.targets.iter().enumerate() {
            let target_path = format!("{pose_path}.targets[{target_index}]");
            if target.target.trim().is_empty() {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{target_path}.target"),
                    "invalid_motion_target",
                    "motion target must not be empty",
                ));
            }
            if !target_ids.insert(target.target.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{target_path}.target"),
                    "duplicate_pose_target",
                    format!("pose target '{}' is duplicated", target.target),
                ));
            }
            let target_property_count = property::count(target);
            if target_property_count == 0 {
                diagnostics.push(AuthoringDiagnostic::new(
                    target_path,
                    "empty_pose_target",
                    "pose targets must declare at least one transform, opacity, width, or height property",
                ));
            }
            property_count = property_count.saturating_add(target_property_count);
        }
        pose_property_counts
            .entry(pose.id.as_str())
            .and_modify(|existing| *existing = (*existing).max(property_count))
            .or_insert(property_count);
    }

    validate_count(
        motion.tracks.len(),
        0,
        MAX_TRACKS,
        "$.motion.tracks",
        "motion_track_limit",
        "motion track count",
        &mut diagnostics,
    );
    let mut track_ids = HashSet::new();
    let mut expanded_keyframe_count = 0_u64;
    for (track_index, track) in motion.tracks.iter().enumerate() {
        let track_path = format!("$.motion.tracks[{track_index}]");
        validate_id(&track.id, &format!("{track_path}.id"), &mut diagnostics);
        if !track_ids.insert(track.id.as_str()) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{track_path}.id"),
                "duplicate_track",
                format!("motion track id '{}' is duplicated", track.id),
            ));
        }
        if !(1..=MAX_FPS).contains(&track.fps) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{track_path}.fps"),
                "invalid_motion_fps",
                format!("motion fps must be between 1 and {MAX_FPS}"),
            ));
        }
        validate_count(
            track.keyframes.len(),
            2,
            MAX_TRACK_KEYFRAMES,
            &format!("{track_path}.keyframes"),
            "motion_keyframe_limit",
            "motion keyframe count",
            &mut diagnostics,
        );
        for (keyframe_index, keyframe) in track.keyframes.iter().enumerate() {
            let keyframe_path = format!("{track_path}.keyframes[{keyframe_index}]");
            if !pose_ids.contains(keyframe.pose.as_str()) {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{keyframe_path}.pose"),
                    "unknown_pose",
                    format!("pose '{}' is not defined", keyframe.pose),
                ));
            }
            if let Some(easing) = keyframe.easing.as_deref() {
                if !easing_ids.contains(easing) {
                    diagnostics.push(AuthoringDiagnostic::new(
                        format!("{keyframe_path}.easing"),
                        "unknown_easing",
                        format!("motion easing '{easing}' is not defined"),
                    ));
                }
                if keyframe.interpolation == MotionInterpolation::Hold {
                    diagnostics.push(AuthoringDiagnostic::new(
                        format!("{keyframe_path}.easing"),
                        "easing_with_hold",
                        "hold keyframes cannot reference a continuous easing",
                    ));
                }
            }
        }

        let property_count = track
            .keyframes
            .iter()
            .filter_map(|keyframe| pose_property_counts.get(keyframe.pose.as_str()).copied())
            .max()
            .unwrap_or(0);
        let previous_expanded_keyframe_count = expanded_keyframe_count;
        expanded_keyframe_count = expanded_keyframe_count
            .saturating_add(property_count.saturating_mul(track.keyframes.len() as u64));
        if previous_expanded_keyframe_count <= MAX_EXPANDED_MOTION_KEYFRAMES
            && expanded_keyframe_count > MAX_EXPANDED_MOTION_KEYFRAMES
        {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{track_path}.keyframes"),
                "motion_keyframe_expansion_limit",
                format!(
                    "expanded motion keyframe count must not exceed {MAX_EXPANDED_MOTION_KEYFRAMES}"
                ),
            ));
        }
    }

    diagnostics
}

fn validate_count(
    value: usize,
    minimum: usize,
    maximum: usize,
    path: &str,
    code: &str,
    label: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    if !(minimum..=maximum).contains(&value) {
        diagnostics.push(AuthoringDiagnostic::new(
            path,
            code,
            format!("{label} must be between {minimum} and {maximum}"),
        ));
    }
}
