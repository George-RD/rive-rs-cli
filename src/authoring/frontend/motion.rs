mod easing;
mod property;
mod timing;
mod validation;

use std::collections::{BTreeMap, HashMap, HashSet};

use serde_json::{Value, json};

use super::super::lower;
use super::super::spec::{
    AuthoringDiagnostic, AuthoringError, AuthoringSpec, MotionContinuity, MotionInterpolation,
    MotionLoop, MotionWaypoint, SourceMapEntry,
};
use super::compiler::MotionTargetIndex;

use easing::{EasingEmission, ResolvedEasing};
use property::{MotionRuntimeObject, PoseValues};
use timing::evaluate_frame_value;

pub(super) use validation::validate_motion;

struct ResolvedFrame {
    authored_index: usize,
    frame: u64,
    pose_index: usize,
    interpolation: MotionInterpolation,
    waypoint: MotionWaypoint,
    easing_index: Option<usize>,
}

struct ResolvedSegment {
    interpolation: MotionInterpolation,
    easing_index: Option<usize>,
}

struct LoweredTracks {
    animations: Vec<Value>,
    source_entries: Vec<SourceMapEntry>,
    easing_emissions: Vec<EasingEmission>,
    warnings: Vec<AuthoringDiagnostic>,
}

pub(super) struct MotionLoweringOutput {
    pub(super) animations: Vec<Value>,
    pub(super) source_entries: Vec<SourceMapEntry>,
    pub(super) easing_source_entries: Vec<SourceMapEntry>,
    pub(super) warnings: Vec<AuthoringDiagnostic>,
}

pub(super) fn lower_motion(
    spec: &AuthoringSpec,
    motion_targets: Result<MotionTargetIndex, AuthoringDiagnostic>,
) -> Result<MotionLoweringOutput, AuthoringError> {
    let easings = easing::resolve(spec).map_err(AuthoringError::one)?;
    let motion_targets = motion_targets.map_err(AuthoringError::one)?;
    let poses = resolve_poses(spec, &motion_targets).map_err(AuthoringError::one)?;
    if spec.motion.tracks.is_empty() {
        return Ok(MotionLoweringOutput {
            animations: Vec::new(),
            source_entries: Vec::new(),
            easing_source_entries: Vec::new(),
            warnings: Vec::new(),
        });
    }

    let LoweredTracks {
        animations,
        source_entries,
        easing_emissions,
        warnings,
    } = lower_tracks(spec, &poses, &easings).map_err(AuthoringError::one)?;
    Ok(MotionLoweringOutput {
        animations,
        source_entries,
        easing_source_entries: easing::source_entries(easing_emissions),
        warnings,
    })
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
            let bindings =
                motion_targets.resolve(&target.target, &format!("{target_path}.target"))?;
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
    let mut animations = Vec::with_capacity(spec.motion.tracks.len());
    let mut source_entries = Vec::with_capacity(spec.motion.tracks.len());
    let mut easing_emissions = easings.iter().map(EasingEmission::new).collect::<Vec<_>>();
    let mut warnings = Vec::new();

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
                waypoint: keyframe.waypoint,
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

        let transit = resolve_transit_points(&frames, track.continuity, &track_path)?;
        let segments = resolve_segments(&frames, &transit);
        warnings.extend(stop_start_warnings(
            &track_path,
            spec,
            &frames,
            &transit,
            poses,
            easings,
        ));
        let referenced_easings = segments
            .iter()
            .filter_map(|segment| segment.easing_index)
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
            for (position, frame) in frames.iter().enumerate() {
                let segment = segments
                    .get(position)
                    .ok_or_else(|| pose_shape_mismatch(&track_path, frame.authored_index))?;
                let value = poses
                    .get(frame.pose_index)
                    .and_then(|pose| pose.get(key))
                    .copied()
                    .ok_or_else(|| pose_shape_mismatch(&track_path, frame.authored_index))?;
                let mut property_frame = json!({
                    "frame": frame.frame,
                    "value": value,
                    "interpolation": if segment.easing_index.is_some() {
                        "cubic"
                    } else {
                        interpolation_name(segment.interpolation)
                    }
                });
                if let Some(easing_index) = segment.easing_index {
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

        let runtime_name =
            lower::runtime_name(&[spec.artboard.id.clone(), track.id.clone()], "animation");
        let mut value = json!({
            "name": runtime_name.clone(),
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
        source_entries.push(SourceMapEntry {
            authored_id: track.id.clone(),
            authored_path: track_path,
            definition_path: None,
            runtime_names: vec![runtime_name],
            scene_paths: vec![format!("/artboard/animations/{track_index}")],
        });
        animations.push(value);
    }

    Ok(LoweredTracks {
        animations,
        source_entries,
        easing_emissions,
        warnings,
    })
}

fn resolve_transit_points(
    frames: &[ResolvedFrame],
    continuity: MotionContinuity,
    track_path: &str,
) -> Result<Vec<bool>, AuthoringDiagnostic> {
    let last = frames.len().saturating_sub(1);
    let mut transit = Vec::with_capacity(frames.len());
    for (position, frame) in frames.iter().enumerate() {
        let interior = position > 0 && position < last;
        if !interior && frame.waypoint != MotionWaypoint::Auto {
            return Err(AuthoringDiagnostic::new(
                format!("{track_path}.keyframes[{}].waypoint", frame.authored_index),
                "waypoint_not_interior",
                "only keyframes between the first and last keyframe can be marked as a transit or settle waypoint",
            ));
        }
        transit.push(
            interior
                && match frame.waypoint {
                    MotionWaypoint::Transit => true,
                    MotionWaypoint::Settle => false,
                    MotionWaypoint::Auto => continuity == MotionContinuity::Through,
                },
        );
    }
    Ok(transit)
}

fn resolve_segments(frames: &[ResolvedFrame], transit: &[bool]) -> Vec<ResolvedSegment> {
    frames
        .iter()
        .enumerate()
        .map(|(position, frame)| {
            let arrives_in_transit = transit.get(position + 1).copied().unwrap_or(false);
            if arrives_in_transit && frame.interpolation != MotionInterpolation::Hold {
                ResolvedSegment {
                    interpolation: MotionInterpolation::Linear,
                    easing_index: None,
                }
            } else {
                ResolvedSegment {
                    interpolation: frame.interpolation,
                    easing_index: frame.easing_index,
                }
            }
        })
        .collect()
}

fn stop_start_warnings(
    track_path: &str,
    spec: &AuthoringSpec,
    frames: &[ResolvedFrame],
    transit: &[bool],
    poses: &[PoseValues],
    easings: &[ResolvedEasing],
) -> Vec<AuthoringDiagnostic> {
    let mut warnings = Vec::new();
    for (position, frame) in frames.iter().enumerate() {
        if transit.get(position).copied().unwrap_or(false)
            || frame.waypoint != MotionWaypoint::Auto
            || position == 0
            || position + 1 >= frames.len()
        {
            continue;
        }
        let Some(previous) = frames.get(position.saturating_sub(1)) else {
            continue;
        };
        let Some(easing_index) = frame.easing_index else {
            continue;
        };
        if previous.easing_index != Some(easing_index) {
            continue;
        }
        let Some(easing) = easings.get(easing_index) else {
            continue;
        };
        if !easing.settles() {
            continue;
        }
        let Some(next) = frames.get(position + 1) else {
            continue;
        };
        if !continues_in_one_direction(poses, previous, frame, next) {
            continue;
        }
        let pose_id = spec
            .motion
            .poses
            .get(frame.pose_index)
            .map(|pose| pose.id.as_str())
            .unwrap_or_default();
        warnings.push(AuthoringDiagnostic::new(
            format!("{track_path}.keyframes[{}]", frame.authored_index),
            "waypoint_stop_start",
            format!(
                "waypoint '{pose_id}' at frame {} enters and leaves on easing '{}', which stops the motion and starts it again; mark this keyframe as a transit waypoint or set the track continuity to 'through' to move through it",
                frame.frame, easing.id
            ),
        ));
    }
    warnings
}

fn continues_in_one_direction(
    poses: &[PoseValues],
    previous: &ResolvedFrame,
    current: &ResolvedFrame,
    next: &ResolvedFrame,
) -> bool {
    let (Some(before), Some(at), Some(after)) = (
        poses.get(previous.pose_index),
        poses.get(current.pose_index),
        poses.get(next.pose_index),
    ) else {
        return false;
    };
    at.iter().any(|(key, value)| {
        let (Some(before), Some(after)) = (before.get(key), after.get(key)) else {
            return false;
        };
        let arriving = value - before;
        let leaving = after - value;
        arriving != 0.0
            && leaving != 0.0
            && arriving.is_sign_positive() == leaving.is_sign_positive()
    })
}

fn pose_shape_mismatch(track_path: &str, authored_index: usize) -> AuthoringDiagnostic {
    AuthoringDiagnostic::new(
        format!("{track_path}.keyframes[{authored_index}].pose"),
        "pose_shape_mismatch",
        "all poses referenced by a motion track must declare the same targets and properties",
    )
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
