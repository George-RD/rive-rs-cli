mod easing;
mod property;
mod timing;
mod validation;

use std::collections::{BTreeMap, HashMap, HashSet};

use serde_json::json;

use super::super::lower;
use super::super::spec::{
    AuthoringDiagnostic, AuthoringError, AuthoringSpec, LoweredAuthoring, MotionInterpolation,
    MotionLoop, RawSceneFragment,
};
use super::compiler::{MotionLoweringInput, MotionTargetIndex};

use easing::{EasingEmission, ResolvedEasing};
use property::{MotionRuntimeObject, PoseValues};
use timing::evaluate_frame_value;

pub(super) use validation::validate_motion;

struct ResolvedFrame {
    authored_index: usize,
    frame: u64,
    pose_index: usize,
    interpolation: MotionInterpolation,
    easing_index: Option<usize>,
}

struct LoweredTracks {
    fragments: Vec<RawSceneFragment>,
    easing_emissions: Vec<EasingEmission>,
}

pub(super) fn lower_motion(
    spec: &AuthoringSpec,
    input: MotionLoweringInput,
) -> Result<LoweredAuthoring, AuthoringError> {
    let easings = easing::resolve(spec).map_err(AuthoringError::one)?;
    let MotionLoweringInput {
        lowered,
        motion_targets,
    } = input;
    let motion_targets = motion_targets.map_err(AuthoringError::one)?;
    let poses = resolve_poses(spec, &motion_targets).map_err(AuthoringError::one)?;
    if spec.motion.tracks.is_empty() {
        return Ok(lowered);
    }

    let LoweredTracks {
        fragments,
        easing_emissions,
    } = lower_tracks(spec, &poses, &easings).map_err(AuthoringError::one)?;
    let typed_count = fragments.len();

    let mut expanded = spec.clone();
    expanded.motion.easings.clear();
    expanded.motion.poses.clear();
    expanded.motion.tracks.clear();
    expanded.motion.raw_animations = fragments
        .into_iter()
        .chain(spec.motion.raw_animations.iter().cloned())
        .collect();

    let mut lowered = lower::lower_authoring(&expanded)
        .map_err(|error| rewrite_motion_error_paths(error, typed_count))?;
    rewrite_motion_source_paths(&mut lowered, typed_count);
    easing::append_source_entries(&mut lowered, easing_emissions);
    Ok(lowered)
}

fn resolve_poses(
    spec: &AuthoringSpec,
    motion_targets: &MotionTargetIndex,
) -> Result<Vec<PoseValues>, AuthoringDiagnostic> {
    let mut poses = Vec::with_capacity(spec.motion.poses.len());
    for (pose_index, pose) in spec.motion.poses.iter().enumerate() {
        let pose_path = format!("$.motion.poses[{pose_index}]");
        let mut values = BTreeMap::new();
        for (target_index, target) in pose.targets.iter().enumerate() {
            let target_path = format!("{pose_path}.targets[{target_index}]");
            let bindings = motion_targets.resolve(
                &target.target,
                &format!("{target_path}.target"),
            )?;
            let runtime_objects = bindings
                .iter()
                .map(|binding| {
                    MotionRuntimeObject::from_binding(
                        &binding.runtime_name,
                        &binding.object_type,
                        binding.is_primary,
                    )
                })
                .collect::<Vec<_>>();
            property::resolve_target_values(
                spec,
                target,
                &target_path,
                &runtime_objects,
                &mut values,
            )?;
        }
        poses.push(values);
    }
    Ok(poses)
}

fn lower_tracks(
    spec: &AuthoringSpec,
    poses: &[PoseValues],
    easings: &[ResolvedEasing],
) -> Result<LoweredTracks, AuthoringDiagnostic> {
    let pose_lookup = spec
        .motion
        .poses
        .iter()
        .enumerate()
        .map(|(index, pose)| (pose.id.as_str(), index))
        .collect::<HashMap<_, _>>();
    let easing_lookup = easings
        .iter()
        .enumerate()
        .map(|(index, easing)| (easing.id.as_str(), index))
        .collect::<HashMap<_, _>>();
    let mut fragments = Vec::with_capacity(spec.motion.tracks.len());
    let mut easing_emissions = easings.iter().map(EasingEmission::new).collect::<Vec<_>>();

    for (track_index, track) in spec.motion.tracks.iter().enumerate() {
        let track_path = format!("$.motion.tracks[{track_index}]");
        let duration = evaluate_frame_value(
            &track.duration_frames,
            &format!("{track_path}.duration_frames"),
            &spec.parameters,
            "invalid_duration_frames",
            "motion duration must be a non-negative whole frame count",
        )?;
        if duration == 0 {
            return Err(AuthoringDiagnostic::new(
                format!("{track_path}.duration_frames"),
                "invalid_duration_frames",
                "motion duration must be greater than zero",
            ));
        }

        let mut frames = Vec::with_capacity(track.keyframes.len());
        for (keyframe_index, keyframe) in track.keyframes.iter().enumerate() {
            let keyframe_path = format!("{track_path}.keyframes[{keyframe_index}]");
            let frame = evaluate_frame_value(
                &keyframe.frame,
                &format!("{keyframe_path}.frame"),
                &spec.parameters,
                "invalid_frame",
                "motion frames must be non-negative whole frame counts",
            )?;
            if frame > duration {
                return Err(AuthoringDiagnostic::new(
                    format!("{keyframe_path}.frame"),
                    "frame_out_of_range",
                    format!("motion frame {frame} exceeds duration {duration}"),
                ));
            }
            let pose_index = pose_lookup
                .get(keyframe.pose.as_str())
                .copied()
                .ok_or_else(|| {
                    AuthoringDiagnostic::new(
                        format!("{keyframe_path}.pose"),
                        "unknown_pose",
                        format!("pose '{}' is not defined", keyframe.pose),
                    )
                })?;
            let easing_index = keyframe
                .easing
                .as_deref()
                .map(|easing| {
                    easing_lookup.get(easing).copied().ok_or_else(|| {
                        AuthoringDiagnostic::new(
                            format!("{keyframe_path}.easing"),
                            "unknown_easing",
                            format!("motion easing '{easing}' is not defined"),
                        )
                    })
                })
                .transpose()?;
            if easing_index.is_some() && keyframe.interpolation == MotionInterpolation::Hold {
                return Err(AuthoringDiagnostic::new(
                    format!("{keyframe_path}.easing"),
                    "easing_with_hold",
                    "hold keyframes cannot reference a continuous easing",
                ));
            }
            frames.push(ResolvedFrame {
                authored_index: keyframe_index,
                frame,
                pose_index,
                interpolation: keyframe.interpolation,
                easing_index,
            });
        }
        frames.sort_by_key(|frame| (frame.frame, frame.authored_index));
        for pair in frames.windows(2) {
            if pair[0].frame == pair[1].frame {
                return Err(AuthoringDiagnostic::new(
                    format!("{track_path}.keyframes[{}].frame", pair[1].authored_index),
                    "duplicate_frame",
                    format!("motion frame {} is declared more than once", pair[1].frame),
                ));
            }
        }

        let first = frames.first().ok_or_else(|| {
            AuthoringDiagnostic::new(
                format!("{track_path}.keyframes"),
                "motion_keyframe_limit",
                "motion tracks require at least two keyframes",
            )
        })?;
        let expected_pose = poses
            .get(first.pose_index)
            .ok_or_else(|| pose_shape_mismatch(&track_path, first.authored_index))?;
        for frame in frames.iter().skip(1) {
            let pose = poses
                .get(frame.pose_index)
                .ok_or_else(|| pose_shape_mismatch(&track_path, frame.authored_index))?;
            if !expected_pose.keys().eq(pose.keys()) {
                return Err(pose_shape_mismatch(&track_path, frame.authored_index));
            }
        }

        let referenced_easings = frames
            .iter()
            .filter_map(|frame| frame.easing_index)
            .collect::<HashSet<_>>();
        let mut interpolators = Vec::new();
        for (easing_index, easing) in easings.iter().enumerate() {
            if referenced_easings.contains(&easing_index) {
                let interpolator_index = interpolators.len();
                interpolators.push(easing::definition(easing));
                easing_emissions[easing_index].record_declaration(track_index, interpolator_index);
            }
        }

        let mut keyframes = Vec::with_capacity(expected_pose.len());
        for key in expected_pose.keys() {
            let (object, property) = key;
            let mut property_frames = Vec::with_capacity(frames.len());
            for frame in &frames {
                let value = poses
                    .get(frame.pose_index)
                    .and_then(|pose| pose.get(key))
                    .copied()
                    .ok_or_else(|| pose_shape_mismatch(&track_path, frame.authored_index))?;
                let mut property_frame = json!({
                    "frame": frame.frame,
                    "value": value,
                    "interpolation": if frame.easing_index.is_some() {
                        "cubic"
                    } else {
                        interpolation_name(frame.interpolation)
                    }
                });
                if let Some(easing_index) = frame.easing_index {
                    let easing = easings.get(easing_index).ok_or_else(|| {
                        AuthoringDiagnostic::new(
                            format!("{track_path}.keyframes[{}].easing", frame.authored_index),
                            "unknown_easing",
                            "motion easing could not be resolved",
                        )
                    })?;
                    if let Some(object) = property_frame.as_object_mut() {
                        object.insert("interpolator".to_string(), json!(easing.runtime_name));
                    }
                }
                property_frames.push(property_frame);
            }
            keyframes.push(json!({
                "object": object,
                "property": property.name(),
                "frames": property_frames
            }));
        }

        let mut value = json!({
            "name": lower::runtime_name(
                &[spec.artboard.id.clone(), track.id.clone()],
                "animation",
            ),
            "fps": track.fps,
            "duration": duration,
            "loop_type": loop_name(track.loop_type),
            "keyframes": keyframes
        });
        if !interpolators.is_empty()
            && let Some(object) = value.as_object_mut()
        {
            object.insert("interpolators".to_string(), json!(interpolators));
        }
        fragments.push(RawSceneFragment {
            id: track.id.clone(),
            value,
        });
    }

    Ok(LoweredTracks {
        fragments,
        easing_emissions,
    })
}

fn pose_shape_mismatch(track_path: &str, authored_index: usize) -> AuthoringDiagnostic {
    AuthoringDiagnostic::new(
        format!("{track_path}.keyframes[{authored_index}].pose"),
        "pose_shape_mismatch",
        "all poses referenced by a motion track must declare the same targets and properties",
    )
}

fn rewrite_motion_error_paths(mut error: AuthoringError, typed_count: usize) -> AuthoringError {
    for diagnostic in &mut error.diagnostics {
        if let Some(path) = rewritten_motion_path(&diagnostic.path, typed_count) {
            diagnostic.path = path;
        }
    }
    error
}

fn rewrite_motion_source_paths(lowered: &mut LoweredAuthoring, typed_count: usize) {
    for entry in &mut lowered.source_map.entries {
        if let Some(path) = rewritten_motion_path(&entry.authored_path, typed_count) {
            entry.authored_path = path;
        }
    }
}

fn rewritten_motion_path(path: &str, typed_count: usize) -> Option<String> {
    let remainder = path.strip_prefix("$.motion.raw_animations[")?;
    let close = remainder.find(']')?;
    let index = remainder[..close].parse::<usize>().ok()?;
    let suffix = &remainder[close + 1..];
    if index < typed_count {
        let suffix = suffix.strip_prefix(".value").unwrap_or(suffix);
        Some(format!("$.motion.tracks[{index}]{suffix}"))
    } else {
        Some(format!(
            "$.motion.raw_animations[{}]{suffix}",
            index - typed_count
        ))
    }
}

fn interpolation_name(interpolation: MotionInterpolation) -> &'static str {
    match interpolation {
        MotionInterpolation::Hold => "hold",
        MotionInterpolation::Linear => "linear",
    }
}

fn loop_name(loop_type: MotionLoop) -> &'static str {
    match loop_type {
        MotionLoop::Oneshot => "oneshot",
        MotionLoop::Loop => "loop",
        MotionLoop::Pingpong => "pingpong",
    }
}
