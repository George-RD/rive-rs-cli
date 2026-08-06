use std::collections::BTreeMap;

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
