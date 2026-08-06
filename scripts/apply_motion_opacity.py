#!/usr/bin/env python3
"""Apply the bounded typed-motion opacity slice to the current checkout."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(relative: str) -> str:
    return (ROOT / relative).read_text(encoding="utf-8")


def write(relative: str, content: str) -> None:
    (ROOT / relative).write_text(content, encoding="utf-8")


def replace_once(relative: str, old: str, new: str) -> None:
    content = read(relative)
    count = content.count(old)
    if count != 1:
        raise RuntimeError(f"expected one match in {relative}, found {count}: {old[:80]!r}")
    write(relative, content.replace(old, new, 1))


def create(relative: str, content: str) -> None:
    path = ROOT / relative
    if path.exists():
        raise RuntimeError(f"refusing to overwrite existing file: {relative}")
    path.write_text(content, encoding="utf-8")


replace_once(
    "src/authoring/spec.rs",
    """pub struct PoseTargetSpec {
    pub target: String,
    pub transform: TransformSpec,
}
""",
    """pub struct PoseTargetSpec {
    pub target: String,
    #[serde(default)]
    pub transform: TransformSpec,
    #[serde(default)]
    pub opacity: Option<ScalarExpr>,
}
""",
)

replace_once(
    "src/authoring/frontend/motion.rs",
    "mod easing;\nmod timing;\n",
    "mod easing;\nmod property;\nmod timing;\n",
)
replace_once(
    "src/authoring/frontend/motion.rs",
    "use super::super::expression::evaluate_expression;\n",
    "",
)
replace_once(
    "src/authoring/frontend/motion.rs",
    """    AuthoringDiagnostic, AuthoringError, AuthoringSourceMap, AuthoringSpec, LoweredAuthoring,
    MotionInterpolation, MotionLoop, RawSceneFragment, ScalarExpr, TransformSpec, Unit,
};
""",
    """    AuthoringDiagnostic, AuthoringError, AuthoringSourceMap, AuthoringSpec, LoweredAuthoring,
    MotionInterpolation, MotionLoop, RawSceneFragment,
};
""",
)
replace_once(
    "src/authoring/frontend/motion.rs",
    """use easing::{EasingEmission, ResolvedEasing};
use timing::evaluate_frame_value;
""",
    """use easing::{EasingEmission, ResolvedEasing};
use property::PoseValues;
use timing::evaluate_frame_value;
""",
)
replace_once(
    "src/authoring/frontend/motion.rs",
    """#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
""",
    "",
)
replace_once(
    "src/authoring/frontend/motion.rs",
    """            for property in TRANSFORM_PROPERTIES {
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
""",
    """            property::resolve_target_values(
                spec,
                target,
                &target_path,
                &runtime_name,
                &mut values,
            )?;
""",
)
replace_once(
    "src/authoring/frontend/motion.rs",
    "all poses referenced by a motion track must declare the same targets and transform properties",
    "all poses referenced by a motion track must declare the same targets and properties",
)

create(
    "src/authoring/frontend/motion/property.rs",
    """use std::collections::BTreeMap;

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
    values: &mut PoseValues,
) -> Result<(), AuthoringDiagnostic> {
    for property in POSE_PROPERTIES {
        let Some(expression) = property.expression(target) else {
            continue;
        };
        let path = property.authored_path(target_path);
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
""",
)

replace_once(
    "src/authoring/frontend/motion/validation.rs",
    """use super::super::super::spec::{
    AuthoringDiagnostic, MotionInterpolation, MotionSection, TransformSpec,
};
use super::super::validate_id;
""",
    """use super::super::super::spec::{AuthoringDiagnostic, MotionInterpolation, MotionSection};
use super::super::validate_id;
use super::property;
""",
)
replace_once(
    "src/authoring/frontend/motion/validation.rs",
    """            if target.transform.x.is_none()
                && target.transform.y.is_none()
                && target.transform.rotation.is_none()
                && target.transform.scale_x.is_none()
                && target.transform.scale_y.is_none()
            {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{target_path}.transform"),
                    "empty_pose_target",
                    "pose targets must declare at least one transform property",
                ));
            }
            property_count =
                property_count.saturating_add(transform_property_count(&target.transform));
""",
    """            if property::count(target) == 0 {
                diagnostics.push(AuthoringDiagnostic::new(
                    format!("{target_path}.transform"),
                    "empty_pose_target",
                    "pose targets must declare at least one transform or opacity property",
                ));
            }
            property_count = property_count.saturating_add(property::count(target));
""",
)
replace_once(
    "src/authoring/frontend/motion/validation.rs",
    """fn transform_property_count(transform: &TransformSpec) -> u64 {
    [
        transform.x.is_some(),
        transform.y.is_some(),
        transform.rotation.is_some(),
        transform.scale_x.is_some(),
        transform.scale_y.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count() as u64
}

""",
    "",
)

replace_once(
    "src/authoring/lower.rs",
    "fn evaluate_ratio_expression(\n",
    "pub(super) fn evaluate_ratio_expression(\n",
)

replace_once(
    "cairn.blueprint",
    '"./tests/authoring_motion_expansion_contract.rs", "./tests/authoring_motion_typed_api_contract.rs"',
    '"./tests/authoring_motion_expansion_contract.rs", "./tests/authoring_motion_opacity_contract.rs", "./tests/authoring_motion_typed_api_contract.rs"',
)

replace_once(
    "meta/contracts/authoring.md",
    "- poses, compact motion tracks, shared easing definitions, and named statecharts;",
    "- poses with transform and opacity properties, compact motion tracks, shared easing definitions, and named statecharts;",
)
replace_once(
    "meta/contracts/authoring.md",
    "The current motion subset supports named transform poses, compact pose tracks, and\nshared cubic Bézier easing definitions with authored visual targets,",
    "The current motion subset supports named transform and opacity poses, compact pose\ntracks, and shared cubic Bézier easing definitions with authored visual targets,",
)
replace_once(
    "meta/contracts/authoring.md",
    """control points remain within zero and one, while value-axis control points may
overshoot. A keyframe may reference one named easing unless it uses `hold`;
""",
    """control points remain within zero and one, while value-axis control points may
overshoot. Authored opacity expressions resolve to scalar ratios in the inclusive
zero-to-one range. A keyframe may reference one named easing unless it uses `hold`;
""",
)
replace_once(
    "meta/contracts/authoring.md",
    "Semantic motion helpers,\nnon-transform property tracks, and typed statecharts remain separate roadmap\nslices.",
    "Semantic motion helpers, color and additional non-transform property tracks, and\ntyped statecharts remain separate roadmap slices.",
)

replace_once(
    "ROADMAP.md",
    "| P2 | [Pose and motion compiler slice](meta/todos/todo.motion-authoring-compiler.md) | in progress; named poses and compact tracks complete in PR #157 | compact tracks and poses reproduce complex motion with runtime proof |",
    "| P2 | [Pose and motion compiler slice](meta/todos/todo.motion-authoring-compiler.md) | in progress; named poses, compact tracks, shared cubic easings, and opacity tracks implemented through PR #164 | compact tracks and poses reproduce complex motion with runtime proof |",
)

replace_once(
    "meta/todos/todo.motion-authoring-compiler.md",
    """## Remaining

- Semantic entrance, exit, stagger, spring, bounce, and similar motion helpers.
- Color and other non-transform property tracks.
""",
    """The opacity continuation remains within this P2 todo:

- exact-head RED `cb9334a` in CI run `31110782636` passed formatting, Clippy, the Rust 1.88 minimum check, browser contracts, and every pre-existing Rust test; only the four new opacity contracts failed because the strict pose-target schema rejected `opacity`;
- pose targets may declare optional scalar opacity without a redundant transform object;
- opacity reuses the canonical SceneSpec `opacity` property-keyframe path, the shared ratio validator, easing resolution, deterministic naming, pose-shape checks, and expansion budget;
- transform and opacity property discovery is centralized in `motion/property.rs`, reducing duplicate counting and lowering logic while keeping `motion.rs` below the Cairn module-size guideline.

## Remaining

- Semantic entrance, exit, stagger, spring, bounce, and similar motion helpers.
- Color and additional non-transform property tracks.
""",
)
