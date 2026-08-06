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
        }
    }

    fn unit(self) -> Unit {
        match self {
            Self::X | Self::Y => Unit::Px,
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
        }
    }

    fn authored_path(self, target_path: &str) -> String {
        match self {
            Self::Opacity => format!("{target_path}.opacity"),
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
            _ => evaluate_expression(expression, path, &spec.parameters, self.unit()),
        }
    }
}

const POSE_PROPERTIES: [PoseProperty; 6] = [
    PoseProperty::X,
    PoseProperty::Y,
    PoseProperty::Rotation,
    PoseProperty::ScaleX,
    PoseProperty::ScaleY,
    PoseProperty::Opacity,
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
    runtime_name: &str,
    object_type: &str,
    values: &mut PoseValues,
) -> Result<(), AuthoringDiagnostic> {
    for property in POSE_PROPERTIES {
        let Some(expression) = property.expression(target) else {
            continue;
        };
        let path = property.authored_path(target_path);
        if property_key_for_object(object_type, property.name()).is_none() {
            return Err(AuthoringDiagnostic::new(
                path,
                "unsupported_motion_property",
                format!(
                    "motion target '{}' resolves to a {object_type}, which does not support property '{}'",
                    target.target,
                    property.name()
                ),
            ));
        }
        let value = property.evaluate(expression, &path, spec)?;
        if values
            .insert((runtime_name.to_owned(), property), value)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authoring::spec::TransformSpec;

    fn literal(value: f64, unit: Unit) -> ScalarExpr {
        ScalarExpr::Literal { value, unit }
    }

    #[test]
    fn property_count_includes_transform_and_opacity_values() {
        let target = PoseTargetSpec {
            target: "card".to_string(),
            transform: TransformSpec {
                x: Some(literal(10.0, Unit::Px)),
                rotation: Some(literal(0.5, Unit::Radians)),
                ..TransformSpec::default()
            },
            opacity: Some(literal(0.75, Unit::Scalar)),
        };

        assert_eq!(count(&target), 3);
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
    }
}
