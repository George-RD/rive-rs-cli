from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file_path = Path(path)
    text = file_path.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected one replacement in {path}, found {count}")
    file_path.write_text(text.replace(old, new, 1))


replace_once(
    "src/authoring/spec.rs",
    """pub struct PoseTargetSpec {
    pub target: String,
    #[serde(default)]
    pub transform: TransformSpec,
    #[serde(default)]
    pub opacity: Option<ScalarExpr>,
}""",
    """pub struct PoseTargetSpec {
    pub target: String,
    #[serde(default)]
    pub transform: TransformSpec,
    #[serde(default)]
    pub opacity: Option<ScalarExpr>,
    #[serde(default)]
    pub width: Option<ScalarExpr>,
    #[serde(default)]
    pub height: Option<ScalarExpr>,
}""",
)

Path("src/authoring/frontend/motion/property.rs").write_text(
    """use std::collections::BTreeMap;

use crate::builder::property_key_for_object;

use super::super::super::expression::evaluate_expression;
use super::super::super::lower::evaluate_ratio_expression;
use super::super::super::spec::{
    AuthoringDiagnostic, AuthoringSpec, PoseTargetSpec, ScalarExpr, Unit,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum PoseProperty {
    X,
    Y,
    Rotation,
    ScaleX,
    ScaleY,
    Opacity,
    Width,
    Height,
}

#[derive(Clone, Copy)]
pub(super) struct MotionRuntimeObject<'a> {
    pub(super) runtime_name: &'a str,
    pub(super) object_type: &'a str,
}

impl PoseProperty {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
            Self::Rotation => "rotation",
            Self::ScaleX => "scale_x",
            Self::ScaleY => "scale_y",
            Self::Opacity => "opacity",
            Self::Width => "width",
            Self::Height => "height",
        }
    }

    fn unit(self) -> Unit {
        match self {
            Self::X | Self::Y | Self::Width | Self::Height => Unit::Px,
            Self::Rotation => Unit::Radians,
            Self::ScaleX | Self::ScaleY | Self::Opacity => Unit::Scalar,
        }
    }

    fn expression(self, target: &PoseTargetSpec) -> Option<&ScalarExpr> {
        match self {
            Self::X => target.transform.x.as_ref(),
            Self::Y => target.transform.y.as_ref(),
            Self::Rotation => target.transform.rotation.as_ref(),
            Self::ScaleX => target.transform.scale_x.as_ref(),
            Self::ScaleY => target.transform.scale_y.as_ref(),
            Self::Opacity => target.opacity.as_ref(),
            Self::Width => target.width.as_ref(),
            Self::Height => target.height.as_ref(),
        }
    }

    fn authored_path(self, target_path: &str) -> String {
        match self {
            Self::Opacity | Self::Width | Self::Height => {
                format!("{target_path}.{}", self.name())
            }
            _ => format!("{target_path}.transform.{}", self.name()),
        }
    }

    fn evaluate(
        self,
        expression: &ScalarExpr,
        path: &str,
        spec: &AuthoringSpec,
    ) -> Result<f64, AuthoringDiagnostic> {
        match self {
            Self::Opacity => evaluate_ratio_expression(
                expression,
                path,
                &spec.parameters,
                "motion opacity must be between zero and one",
            ),
            Self::Width | Self::Height => {
                let value = evaluate_expression(expression, path, &spec.parameters, self.unit())?;
                if value <= 0.0 {
                    Err(AuthoringDiagnostic::new(
                        path,
                        "invalid_dimension",
                        format!("motion {} must be greater than zero", self.name()),
                    ))
                } else {
                    Ok(value)
                }
            }
            _ => evaluate_expression(expression, path, &spec.parameters, self.unit()),
        }
    }
}

const POSE_PROPERTIES: [PoseProperty; 8] = [
    PoseProperty::X,
    PoseProperty::Y,
    PoseProperty::Rotation,
    PoseProperty::ScaleX,
    PoseProperty::ScaleY,
    PoseProperty::Opacity,
    PoseProperty::Width,
    PoseProperty::Height,
];

pub(super) type PoseValues = BTreeMap<(String, PoseProperty), f64>;

pub(super) fn count(target: &PoseTargetSpec) -> u64 {
    POSE_PROPERTIES
        .into_iter()
        .filter(|property| property.expression(target).is_some())
        .count() as u64
}

pub(super) fn resolve_target_values(
    spec: &AuthoringSpec,
    target: &PoseTargetSpec,
    target_path: &str,
    runtime_objects: &[MotionRuntimeObject<'_>],
    values: &mut PoseValues,
) -> Result<(), AuthoringDiagnostic> {
    for property in POSE_PROPERTIES {
        let Some(expression) = property.expression(target) else {
            continue;
        };
        let path = property.authored_path(target_path);
        let Some(runtime_object) = target_for_property(runtime_objects, property) else {
            return Err(AuthoringDiagnostic::new(
                path,
                "unsupported_motion_property",
                format!(
                    "motion target '{}' does not resolve to an object that supports property '{}'",
                    target.target,
                    property.name()
                ),
            ));
        };
        let value = property.evaluate(expression, &path, spec)?;
        if values
            .insert((runtime_object.runtime_name.to_owned(), property), value)
            .is_some()
        {
            return Err(AuthoringDiagnostic::new(
                path,
                "duplicate_motion_property",
                format!(
                    "motion target '{}' declares property '{}' more than once",
                    target.target,
                    property.name()
                ),
            ));
        }
    }
    Ok(())
}

fn target_for_property<'a>(
    runtime_objects: &[MotionRuntimeObject<'a>],
    property: PoseProperty,
) -> Option<MotionRuntimeObject<'a>> {
    runtime_objects.iter().copied().find(|runtime_object| {
        property_key_for_object(runtime_object.object_type, property.name()).is_some()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authoring::spec::TransformSpec;

    fn literal(value: f64, unit: Unit) -> ScalarExpr {
        ScalarExpr::Literal { value, unit }
    }

    #[test]
    fn property_count_includes_transform_opacity_and_dimension_values() {
        let target = PoseTargetSpec {
            target: "card".to_string(),
            transform: TransformSpec {
                x: Some(literal(10.0, Unit::Px)),
                rotation: Some(literal(0.5, Unit::Radians)),
                ..TransformSpec::default()
            },
            opacity: Some(literal(0.75, Unit::Scalar)),
            width: Some(literal(80.0, Unit::Px)),
            height: None,
        };

        assert_eq!(count(&target), 4);
    }

    #[test]
    fn authored_paths_follow_each_property_shape() {
        let target_path = "$.motion.poses[0].targets[0]";

        assert_eq!(
            PoseProperty::X.authored_path(target_path),
            "$.motion.poses[0].targets[0].transform.x"
        );
        assert_eq!(
            PoseProperty::Opacity.authored_path(target_path),
            "$.motion.poses[0].targets[0].opacity"
        );
        assert_eq!(
            PoseProperty::Width.authored_path(target_path),
            "$.motion.poses[0].targets[0].width"
        );
    }

    #[test]
    fn properties_route_to_the_first_compatible_runtime_object() {
        let runtime_objects = [
            MotionRuntimeObject {
                runtime_name: "shape",
                object_type: "shape",
            },
            MotionRuntimeObject {
                runtime_name: "geometry",
                object_type: "rectangle",
            },
        ];

        assert_eq!(
            target_for_property(&runtime_objects, PoseProperty::X)
                .expect("shape x target")
                .runtime_name,
            "shape"
        );
        assert_eq!(
            target_for_property(&runtime_objects, PoseProperty::Width)
                .expect("geometry width target")
                .runtime_name,
            "geometry"
        );
    }
}
"""
)

replace_once(
    "src/authoring/frontend/motion.rs",
    "use property::PoseValues;",
    "use property::{MotionRuntimeObject, PoseValues};",
)
replace_once(
    "src/authoring/frontend/motion.rs",
    """type MotionTargetIndex<'a> = HashMap<&'a str, IndexedMotionTarget<'a>>;

#[derive(Clone, Copy)]
struct MotionTarget<'a> {
    runtime_name: &'a str,
    object_type: &'a str,
}

#[derive(Clone, Copy)]
enum IndexedMotionTarget<'a> {
    Unique(Option<MotionTarget<'a>>),
    Ambiguous,
}""",
    """type MotionTargetIndex<'a> = HashMap<&'a str, IndexedMotionTarget<'a>>;

enum IndexedMotionTarget<'a> {
    Unique(Vec<MotionRuntimeObject<'a>>),
    Ambiguous,
}""",
)
replace_once(
    "src/authoring/frontend/motion.rs",
    """            let resolved_target = resolve_motion_target(
                &motion_targets,
                &target.target,
                &format!("{target_path}.target"),
            )?;
            property::resolve_target_values(
                spec,
                target,
                &target_path,
                resolved_target.runtime_name,
                resolved_target.object_type,
                &mut values,
            )?;""",
    """            let resolved_targets = resolve_motion_targets(
                &motion_targets,
                &target.target,
                &format!("{target_path}.target"),
            )?;
            property::resolve_target_values(
                spec,
                target,
                &target_path,
                resolved_targets,
                &mut values,
            )?;""",
)
replace_once(
    "src/authoring/frontend/motion.rs",
    """        let target = entry
            .runtime_names
            .first()
            .zip(entry.scene_paths.first())
            .and_then(|(runtime_name, scene_path)| {
                lowered
                    .scene
                    .pointer(scene_path)
                    .and_then(|object| object.get("type"))
                    .and_then(Value::as_str)
                    .map(|object_type| MotionTarget {
                        runtime_name,
                        object_type,
                    })
            });
        let indexed = IndexedMotionTarget::Unique(target);""",
    """        let target = entry
            .runtime_names
            .iter()
            .zip(&entry.scene_paths)
            .filter_map(|(runtime_name, scene_path)| {
                lowered
                    .scene
                    .pointer(scene_path)
                    .and_then(|object| object.get("type"))
                    .and_then(Value::as_str)
                    .map(|object_type| MotionRuntimeObject {
                        runtime_name: runtime_name.as_str(),
                        object_type,
                    })
            })
            .collect::<Vec<_>>();
        let indexed = IndexedMotionTarget::Unique(target);""",
)
replace_once(
    "src/authoring/frontend/motion.rs",
    """fn resolve_motion_target<'a>(
    target_index: &MotionTargetIndex<'a>,
    target: &str,
    path: &str,
) -> Result<MotionTarget<'a>, AuthoringDiagnostic> {
    match target_index.get(target).copied() {
        None => Err(AuthoringDiagnostic::new(
            path,
            "unknown_motion_target",
            format!("visual target '{target}' is not defined"),
        )),
        Some(IndexedMotionTarget::Ambiguous) => Err(AuthoringDiagnostic::new(
            path,
            "ambiguous_motion_target",
            format!("visual target '{target}' resolves to more than one authored node"),
        )),
        Some(IndexedMotionTarget::Unique(None)) => Err(AuthoringDiagnostic::new(
            path,
            "unsupported_motion_target",
            format!("visual target '{target}' has no animatable runtime object"),
        )),
        Some(IndexedMotionTarget::Unique(Some(target))) => Ok(target),
    }
}""",
    """fn resolve_motion_targets<'index, 'scene>(
    target_index: &'index MotionTargetIndex<'scene>,
    target: &str,
    path: &str,
) -> Result<&'index [MotionRuntimeObject<'scene>], AuthoringDiagnostic> {
    match target_index.get(target) {
        None => Err(AuthoringDiagnostic::new(
            path,
            "unknown_motion_target",
            format!("visual target '{target}' is not defined"),
        )),
        Some(IndexedMotionTarget::Ambiguous) => Err(AuthoringDiagnostic::new(
            path,
            "ambiguous_motion_target",
            format!("visual target '{target}' resolves to more than one authored node"),
        )),
        Some(IndexedMotionTarget::Unique(targets)) if targets.is_empty() => {
            Err(AuthoringDiagnostic::new(
                path,
                "unsupported_motion_target",
                format!("visual target '{target}' has no animatable runtime object"),
            ))
        }
        Some(IndexedMotionTarget::Unique(targets)) => Ok(targets),
    }
}""",
)

replace_once(
    "src/authoring/frontend/motion/validation.rs",
    "pose targets must declare at least one transform or opacity property",
    "pose targets must declare at least one transform, opacity, width, or height property",
)

replace_once(
    "cairn.blueprint",
    '"./tests/authoring_motion_contract.rs", "./tests/authoring_motion_easing_contract.rs"',
    '"./tests/authoring_motion_contract.rs", "./tests/authoring_motion_dimension_contract.rs", "./tests/authoring_motion_easing_contract.rs"',
)

replace_once(
    "ROADMAP.md",
    "in progress; named poses, compact tracks, shared cubic easings, and opacity tracks implemented through PR #164",
    "in progress; named poses, compact tracks, shared cubic easings, opacity, and parametric shape-dimension tracks implemented through PR #165",
)

replace_once(
    "meta/contracts/authoring.md",
    "poses with transform and opacity properties, compact motion tracks, shared easing definitions, and named statecharts;",
    "poses with transform, opacity, and parametric shape-dimension properties, compact motion tracks, shared easing definitions, and named statecharts;",
)
replace_once(
    "meta/contracts/authoring.md",
    """The current motion subset supports named transform and opacity poses, compact pose
tracks, and shared cubic Bézier easing definitions with authored visual targets,""",
    """The current motion subset supports named transform, opacity, and positive pixel-valued
parametric shape-dimension poses, compact pose tracks, and shared cubic Bézier easing
definitions with authored visual targets,""",
)
replace_once(
    "meta/contracts/authoring.md",
    """overshoot. Authored opacity expressions resolve to scalar ratios in the inclusive
zero-to-one range. A keyframe may reference one named easing unless it uses `hold`;""",
    """overshoot. Authored opacity expressions resolve to scalar ratios in the inclusive
zero-to-one range. Width and height expressions resolve to positive pixels and select
the first compatible runtime object represented by an authored visual node, while
transform properties remain on its root transform object. A keyframe may reference
one named easing unless it uses `hold`;""",
)
replace_once(
    "meta/contracts/authoring.md",
    """targets are indexed once from the authored source map. Their resolved runtime object
type is retained, and every authored property is checked against the canonical
builder's animatable-property registry before keyframe emission.""",
    """targets are indexed once from the authored source map as ordered runtime object
candidates. Every authored property selects the first candidate accepted by the
canonical builder's animatable-property registry before keyframe emission.""",
)
replace_once(
    "meta/contracts/authoring.md",
    "color and additional non-transform property tracks, and typed statecharts remain",
    "color and further non-transform property tracks, and typed statecharts remain",
)
