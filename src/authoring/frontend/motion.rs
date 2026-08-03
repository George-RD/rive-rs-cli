mod validation;

use std::collections::{BTreeMap, HashMap};

use serde_json::json;

use super::super::expression::evaluate_expression;
use super::super::lower;
use super::super::spec::{
    AuthoringDiagnostic, AuthoringError, AuthoringSourceMap, AuthoringSpec, LoweredAuthoring,
    MotionInterpolation, MotionLoop, Quantity, RawSceneFragment, ScalarExpr, TransformSpec, Unit,
};

pub(super) use validation::validate_motion;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TransformProperty {
    X,
    Y,
    Rotation,
    ScaleX,
    ScaleY,
}

impl TransformProperty {
    fn name(self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
            Self::Rotation => "rotation",
            Self::ScaleX => "scale_x",
            Self::ScaleY => "scale_y",
        }
    }

    fn unit(self) -> Unit {
        match self {
            Self::X | Self::Y => Unit::Px,
            Self::Rotation => Unit::Radians,
            Self::ScaleX | Self::ScaleY => Unit::Scalar,
        }
    }

    fn expression(self, transform: &TransformSpec) -> Option<&ScalarExpr> {
        match self {
            Self::X => transform.x.as_ref(),
            Self::Y => transform.y.as_ref(),
            Self::Rotation => transform.rotation.as_ref(),
            Self::ScaleX => transform.scale_x.as_ref(),
            Self::ScaleY => transform.scale_y.as_ref(),
        }
    }
}

const TRANSFORM_PROPERTIES: [TransformProperty; 5] = [
    TransformProperty::X,
    TransformProperty::Y,
    TransformProperty::Rotation,
    TransformProperty::ScaleX,
    TransformProperty::ScaleY,
];

type PoseValues = BTreeMap<(String, TransformProperty), f64>;

struct ResolvedFrame {
    authored_index: usize,
    frame: u64,
    pose_index: usize,
    interpolation: MotionInterpolation,
}

pub(super) fn lower_motion(
    spec: &AuthoringSpec,
    lowered: LoweredAuthoring,
) -> Result<LoweredAuthoring, AuthoringError> {
    let poses = resolve_poses(spec, &lowered.source_map).map_err(AuthoringError::one)?;
    if spec.motion.tracks.is_empty() {
        return Ok(lowered);
    }

    let fragments = lower_tracks(spec, &poses).map_err(AuthoringError::one)?;
    let typed_count = fragments.len();

    let mut expanded = spec.clone();
    expanded.motion.poses.clear();
    expanded.motion.tracks.clear();
    expanded.motion.raw_animations = fragments
        .into_iter()
        .chain(spec.motion.raw_animations.iter().cloned())
        .collect();

    let mut lowered = lower::lower_authoring(&expanded)
        .map_err(|error| rewrite_motion_error_paths(error, typed_count))?;
    rewrite_motion_source_paths(&mut lowered, typed_count);
    Ok(lowered)
}

fn resolve_poses(
    spec: &AuthoringSpec,
    source_map: &AuthoringSourceMap,
) -> Result<Vec<PoseValues>, AuthoringDiagnostic> {
    let mut poses = Vec::with_capacity(spec.motion.poses.len());
    for (pose_index, pose) in spec.motion.poses.iter().enumerate() {
        let pose_path = format!("$.motion.poses[{pose_index}]");
        let mut values = BTreeMap::new();
        for (target_index, target) in pose.targets.iter().enumerate() {
            let target_path = format!("{pose_path}.targets[{target_index}]");
            let runtime_name = resolve_target_runtime_name(
                source_map,
                &target.target,
                &format!("{target_path}.target"),
            )?;
            for property in TRANSFORM_PROPERTIES {
                let Some(expression) = property.expression(&target.transform) else {
                    continue;
                };
                let value = evaluate_expression(
                    expression,
                    &format!("{target_path}.transform.{}", property.name()),
                    &spec.parameters,
                    property.unit(),
                )?;
                if values
                    .insert((runtime_name.clone(), property), value)
                    .is_some()
                {
                    return Err(AuthoringDiagnostic::new(
                        format!("{target_path}.transform.{}", property.name()),
                        "duplicate_motion_property",
                        format!(
                            "motion target '{}' declares property '{}' more than once",
                            target.target,
                            property.name()
                        ),
                    ));
                }
            }
        }
        poses.push(values);
    }
    Ok(poses)
}

fn resolve_target_runtime_name(
    source_map: &AuthoringSourceMap,
    target: &str,
    path: &str,
) -> Result<String, AuthoringDiagnostic> {
    let mut matches = source_map.entries.iter().filter(|entry| {
        entry.authored_id == target && entry.authored_path.starts_with("$.visual.nodes[")
    });
    let Some(entry) = matches.next() else {
        return Err(AuthoringDiagnostic::new(
            path,
            "unknown_motion_target",
            format!("visual target '{target}' is not defined"),
        ));
    };
    if matches.next().is_some() {
        return Err(AuthoringDiagnostic::new(
            path,
            "ambiguous_motion_target",
            format!("visual target '{target}' resolves to more than one authored node"),
        ));
    }
    entry.runtime_names.first().cloned().ok_or_else(|| {
        AuthoringDiagnostic::new(
            path,
            "unsupported_motion_target",
            format!("visual target '{target}' has no animatable runtime object"),
        )
    })
}

fn lower_tracks(
    spec: &AuthoringSpec,
    poses: &[PoseValues],
) -> Result<Vec<RawSceneFragment>, AuthoringDiagnostic> {
    let pose_lookup = spec
        .motion
        .poses
        .iter()
        .enumerate()
        .map(|(index, pose)| (pose.id.as_str(), index))
        .collect::<HashMap<_, _>>();
    let mut fragments = Vec::with_capacity(spec.motion.tracks.len());

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
            frames.push(ResolvedFrame {
                authored_index: keyframe_index,
                frame,
                pose_index,
                interpolation: keyframe.interpolation,
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
        let expected_pose = &poses[first.pose_index];
        for frame in &frames[1..] {
            if !expected_pose.keys().eq(poses[frame.pose_index].keys()) {
                return Err(AuthoringDiagnostic::new(
                    format!("{track_path}.keyframes[{}].pose", frame.authored_index),
                    "pose_shape_mismatch",
                    "all poses referenced by a motion track must declare the same targets and transform properties",
                ));
            }
        }

        let keyframes = expected_pose
            .keys()
            .map(|(object, property)| {
                let frames = frames
                    .iter()
                    .map(|frame| {
                        let key = (object.clone(), *property);
                        let value = poses[frame.pose_index]
                            .get(&key)
                            .copied()
                            .expect("pose shapes were checked before lowering");
                        json!({
                            "frame": frame.frame,
                            "value": value,
                            "interpolation": interpolation_name(frame.interpolation)
                        })
                    })
                    .collect::<Vec<_>>();
                json!({
                    "object": object,
                    "property": property.name(),
                    "frames": frames
                })
            })
            .collect::<Vec<_>>();

        fragments.push(RawSceneFragment {
            id: track.id.clone(),
            value: json!({
                "name": lower::runtime_name(
                    &[spec.artboard.id.clone(), track.id.clone()],
                    "animation",
                ),
                "fps": track.fps,
                "duration": duration,
                "loop_type": loop_name(track.loop_type),
                "keyframes": keyframes
            }),
        });
    }

    Ok(fragments)
}

fn evaluate_frame_value(
    expression: &ScalarExpr,
    path: &str,
    scope: &BTreeMap<String, Quantity>,
    code: &str,
    message: &str,
) -> Result<u64, AuthoringDiagnostic> {
    let value = evaluate_expression(expression, path, scope, Unit::Scalar)?;
    if value < 0.0 || value.fract() != 0.0 || value > u64::MAX as f64 {
        return Err(AuthoringDiagnostic::new(path, code, message));
    }
    Ok(value as u64)
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
