from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file_path = Path(path)
    text = file_path.read_text()
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f"expected one match in {path}, found {count}: {old[:100]!r}")
    file_path.write_text(text.replace(old, new, 1))


def create_once(path: str, content: str) -> None:
    file_path = Path(path)
    if file_path.exists():
        raise RuntimeError(f"refusing to replace existing file: {path}")
    file_path.write_text(content)


replace_once(
    "src/authoring/visual.rs",
    """pub enum MirrorAxis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
""",
    """pub enum MirrorAxis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PathPointSpec {
    pub x: ScalarExpr,
    pub y: ScalarExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
""",
)
replace_once(
    "src/authoring/visual.rs",
    """    Distribute {
        id: String,
        #[schemars(range(min = 2, max = 100))]
        copies: u64,
        start_x: ScalarExpr,
        start_y: ScalarExpr,
        end_x: ScalarExpr,
        end_y: ScalarExpr,
        item: Box<VisualNode>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Group {
""",
    """    Distribute {
        id: String,
        #[schemars(range(min = 2, max = 100))]
        copies: u64,
        start_x: ScalarExpr,
        start_y: ScalarExpr,
        end_x: ScalarExpr,
        end_y: ScalarExpr,
        item: Box<VisualNode>,
        #[serde(default)]
        transform: TransformSpec,
    },
    AlongPath {
        id: String,
        #[schemars(range(min = 2, max = 100))]
        copies: u64,
        #[schemars(length(min = 2, max = 100))]
        points: Vec<PathPointSpec>,
        #[serde(default)]
        rotate_items: bool,
        item: Box<VisualNode>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Group {
""",
)
replace_once(
    "src/authoring/visual.rs",
    """pub(crate) struct DistributeNodeRef<'a> {
    pub copies: u64,
    pub start_x: &'a ScalarExpr,
    pub start_y: &'a ScalarExpr,
    pub end_x: &'a ScalarExpr,
    pub end_y: &'a ScalarExpr,
    pub item: &'a VisualNode,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) enum PatternNodeRef<'a> {
""",
    """pub(crate) struct DistributeNodeRef<'a> {
    pub copies: u64,
    pub start_x: &'a ScalarExpr,
    pub start_y: &'a ScalarExpr,
    pub end_x: &'a ScalarExpr,
    pub end_y: &'a ScalarExpr,
    pub item: &'a VisualNode,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) struct AlongPathNodeRef<'a> {
    pub copies: u64,
    pub points: &'a [PathPointSpec],
    pub rotate_items: bool,
    pub item: &'a VisualNode,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) enum PatternNodeRef<'a> {
""",
)
replace_once(
    "src/authoring/visual.rs",
    """pub(crate) enum PatternNodeRef<'a> {
    Grid(GridNodeRef<'a>),
    Radial(RadialNodeRef<'a>),
    Mirror(MirrorNodeRef<'a>),
    Distribute(DistributeNodeRef<'a>),
}
""",
    """pub(crate) enum PatternNodeRef<'a> {
    Grid(GridNodeRef<'a>),
    Radial(RadialNodeRef<'a>),
    Mirror(MirrorNodeRef<'a>),
    Distribute(DistributeNodeRef<'a>),
    AlongPath(AlongPathNodeRef<'a>),
}
""",
)
replace_once(
    "src/authoring/visual.rs",
    """            Self::Mirror(mirror) => mirror.item,
            Self::Distribute(distribute) => distribute.item,
""",
    """            Self::Mirror(mirror) => mirror.item,
            Self::Distribute(distribute) => distribute.item,
            Self::AlongPath(along_path) => along_path.item,
""",
)
replace_once(
    "src/authoring/visual.rs",
    """            | Self::Mirror { id, .. }
            | Self::Distribute { id, .. }
            | Self::Group { id, .. }
""",
    """            | Self::Mirror { id, .. }
            | Self::Distribute { id, .. }
            | Self::AlongPath { id, .. }
            | Self::Group { id, .. }
""",
)
replace_once(
    "src/authoring/visual.rs",
    """            | Self::Mirror { .. }
            | Self::Distribute { .. }
            | Self::Group { .. }
""",
    """            | Self::Mirror { .. }
            | Self::Distribute { .. }
            | Self::AlongPath { .. }
            | Self::Group { .. }
""",
)
replace_once(
    "src/authoring/visual.rs",
    """            Self::Distribute {
                copies,
                start_x,
                start_y,
                end_x,
                end_y,
                item,
                transform,
                ..
            } => Some(PatternNodeRef::Distribute(DistributeNodeRef {
                copies: *copies,
                start_x,
                start_y,
                end_x,
                end_y,
                item,
                transform,
            })),
            _ => None,
""",
    """            Self::Distribute {
                copies,
                start_x,
                start_y,
                end_x,
                end_y,
                item,
                transform,
                ..
            } => Some(PatternNodeRef::Distribute(DistributeNodeRef {
                copies: *copies,
                start_x,
                start_y,
                end_x,
                end_y,
                item,
                transform,
            })),
            Self::AlongPath {
                copies,
                points,
                rotate_items,
                item,
                transform,
                ..
            } => Some(PatternNodeRef::AlongPath(AlongPathNodeRef {
                copies: *copies,
                points,
                rotate_items: *rotate_items,
                item,
                transform,
            })),
            _ => None,
""",
)

replace_once(
    "src/authoring/mod.rs",
    "pub use visual::{MirrorAxis, VisualNode};",
    "pub use visual::{MirrorAxis, PathPointSpec, VisualNode};",
)

replace_once(
    "src/authoring/deterministic_math.rs",
    """pub(crate) fn sin_cos(radians: f64) -> (f64, f64) {
    (libm::sin(radians), libm::cos(radians))
}
""",
    """pub(crate) fn sin_cos(radians: f64) -> (f64, f64) {
    (libm::sin(radians), libm::cos(radians))
}

pub(crate) fn hypot(x: f64, y: f64) -> f64 {
    libm::hypot(x, y)
}

pub(crate) fn atan2(y: f64, x: f64) -> f64 {
    libm::atan2(y, x)
}
""",
)
replace_once(
    "src/authoring/deterministic_math.rs",
    "use super::{radians_from_degrees, sin_cos};",
    "use super::{atan2, hypot, radians_from_degrees, sin_cos};",
)
replace_once(
    "src/authoring/deterministic_math.rs",
    """    fn trigonometry_has_pinned_bits() {
        let angle = f64::from_bits(0xbfe9_0003_ce1c_711f);
        let (sine, cosine) = sin_cos(angle);
        assert_eq!(sine.to_bits(), 0xbfe6_888d_01ba_048a);
        assert_eq!(cosine.to_bits(), 0x3fe6_b896_4cae_d975);
    }
}
""",
    """    fn trigonometry_has_pinned_bits() {
        let angle = f64::from_bits(0xbfe9_0003_ce1c_711f);
        let (sine, cosine) = sin_cos(angle);
        assert_eq!(sine.to_bits(), 0xbfe6_888d_01ba_048a);
        assert_eq!(cosine.to_bits(), 0x3fe6_b896_4cae_d975);
    }

    #[test]
    fn path_math_has_pinned_bits() {
        assert_eq!(hypot(3.0, 4.0).to_bits(), 5.0_f64.to_bits());
        assert_eq!(
            atan2(1.0, 0.0).to_bits(),
            std::f64::consts::FRAC_PI_2.to_bits()
        );
    }
}
""",
)

replace_once(
    "src/authoring/lower.rs",
    """mod paint;
mod pattern;
mod shape;
""",
    """mod paint;
mod path;
mod pattern;
mod shape;
""",
)

create_once(
    "src/authoring/lower/path.rs",
    """use super::super::deterministic_math::{atan2, hypot};

#[derive(Clone, Copy)]
pub(super) struct PathPlacement {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PathSamplingError {
    InvalidCopyCount,
    InvalidPointCount,
    ZeroLengthSegment { point_index: usize },
}

struct PathSegment {
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    delta_x: f64,
    delta_y: f64,
    length: f64,
    rotation: f64,
}

pub(super) fn along_path_placements(
    copies: u64,
    points: &[(f64, f64)],
    rotate_items: bool,
) -> Result<Vec<PathPlacement>, PathSamplingError> {
    if copies < 2 {
        return Err(PathSamplingError::InvalidCopyCount);
    }
    if points.len() < 2 {
        return Err(PathSamplingError::InvalidPointCount);
    }

    let mut segments = Vec::with_capacity(points.len() - 1);
    let mut total_length = 0.0;
    for (index, pair) in points.windows(2).enumerate() {
        let (start_x, start_y) = pair[0];
        let (end_x, end_y) = pair[1];
        let delta_x = end_x - start_x;
        let delta_y = end_y - start_y;
        let length = hypot(delta_x, delta_y);
        if length == 0.0 {
            return Err(PathSamplingError::ZeroLengthSegment {
                point_index: index + 1,
            });
        }
        total_length += length;
        segments.push(PathSegment {
            start_x,
            start_y,
            end_x,
            end_y,
            delta_x,
            delta_y,
            length,
            rotation: atan2(delta_y, delta_x),
        });
    }

    let capacity = usize::try_from(copies).unwrap_or_default();
    let mut placements = Vec::with_capacity(capacity);
    let last_copy = copies - 1;
    let mut segment_index = 0;
    let mut segment_start_distance = 0.0;

    for index in 0..copies {
        let segment = &segments[segment_index];
        let (x, y, rotation) = if index == 0 {
            (segment.start_x, segment.start_y, segment.rotation)
        } else if index == last_copy {
            let last_segment = &segments[segments.len() - 1];
            (
                last_segment.end_x,
                last_segment.end_y,
                last_segment.rotation,
            )
        } else {
            let target_distance = total_length * index as f64 / last_copy as f64;
            while segment_index + 1 < segments.len()
                && target_distance
                    >= segment_start_distance + segments[segment_index].length
            {
                segment_start_distance += segments[segment_index].length;
                segment_index += 1;
            }
            let active_segment = &segments[segment_index];
            let progress =
                (target_distance - segment_start_distance) / active_segment.length;
            (
                active_segment.start_x + active_segment.delta_x * progress,
                active_segment.start_y + active_segment.delta_y * progress,
                active_segment.rotation,
            )
        };
        placements.push(PathPlacement {
            x,
            y,
            rotation: if rotate_items { rotation } else { 0.0 },
        });
    }

    Ok(placements)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use super::{PathSamplingError, along_path_placements};

    #[test]
    fn samples_equal_arc_length_with_outgoing_vertex_tangents() {
        let placements = along_path_placements(
            5,
            &[(0.0, 0.0), (60.0, 0.0), (60.0, 60.0)],
            true,
        )
        .expect("valid path sampling");

        let expected = [
            (0.0, 0.0, 0.0),
            (30.0, 0.0, 0.0),
            (60.0, 0.0, FRAC_PI_2),
            (60.0, 30.0, FRAC_PI_2),
            (60.0, 60.0, FRAC_PI_2),
        ];
        for (placement, (x, y, rotation)) in placements.iter().zip(expected) {
            assert_eq!(placement.x, x);
            assert_eq!(placement.y, y);
            assert_eq!(placement.rotation, rotation);
        }
    }

    #[test]
    fn rejects_consecutive_duplicate_points() {
        let error = along_path_placements(
            3,
            &[(0.0, 0.0), (0.0, 0.0), (20.0, 0.0)],
            true,
        )
        .expect_err("duplicate points must fail");
        assert_eq!(
            error,
            PathSamplingError::ZeroLengthSegment { point_index: 1 }
        );
    }
}
""",
)

replace_once(
    "src/authoring/lower/pattern.rs",
    """use super::super::visual::{
    DistributeNodeRef, GridNodeRef, MirrorAxis, MirrorNodeRef, PatternNodeRef, RadialNodeRef,
    VisualNode,
};
use super::{Lowerer, NodeContext, runtime_name};
""",
    """use super::super::visual::{
    AlongPathNodeRef, DistributeNodeRef, GridNodeRef, MirrorAxis, MirrorNodeRef, PatternNodeRef,
    RadialNodeRef, VisualNode,
};
use super::path::{PathSamplingError, along_path_placements};
use super::{Lowerer, NodeContext, runtime_name};
""",
)
replace_once(
    "src/authoring/lower/pattern.rs",
    """            PatternNodeRef::Distribute(distribute) => {
                self.lower_distribute(distribute, context, component_stack)
            }
""",
    """            PatternNodeRef::Distribute(distribute) => {
                self.lower_distribute(distribute, context, component_stack)
            }
            PatternNodeRef::AlongPath(along_path) => {
                self.lower_along_path(along_path, context, component_stack)
            }
""",
)
replace_once(
    "src/authoring/lower/pattern.rs",
    """    fn lower_distribute(
""",
    """    fn lower_along_path(
        &mut self,
        along_path: AlongPathNodeRef<'_>,
        context: NodeContext<'_>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let AlongPathNodeRef {
            copies,
            points,
            rotate_items,
            item,
            transform,
        } = along_path;
        let points_path = format!("{}.points", context.authored_path);
        let mut evaluated_points = Vec::with_capacity(points.len());
        for (index, point) in points.iter().enumerate() {
            let x_path = format!("{points_path}[{index}].x");
            let x = evaluate_expression(&point.x, &x_path, context.scope, Unit::Px)?;
            let y_path = format!("{points_path}[{index}].y");
            let y = evaluate_expression(&point.y, &y_path, context.scope, Unit::Px)?;
            evaluated_points.push((x, y));
        }

        let sampled = along_path_placements(copies, &evaluated_points, rotate_items).map_err(
            |error| match error {
                PathSamplingError::InvalidCopyCount => AuthoringDiagnostic::new(
                    format!("{}.copies", context.authored_path),
                    "invalid_pattern_count",
                    "along-path copy count must be at least two",
                ),
                PathSamplingError::InvalidPointCount => AuthoringDiagnostic::new(
                    points_path.clone(),
                    "invalid_path_point_count",
                    "along-path patterns require at least two points",
                ),
                PathSamplingError::ZeroLengthSegment { point_index } => {
                    AuthoringDiagnostic::new(
                        format!("{points_path}[{point_index}]"),
                        "invalid_path_segment",
                        "along-path points must not repeat consecutively",
                    )
                }
            },
        )?;
        let mut placements = Vec::with_capacity(sampled.len());
        for (index, placement) in sampled.into_iter().enumerate() {
            validate_scene_number(placement.x, &points_path)?;
            validate_scene_number(placement.y, &points_path)?;
            validate_scene_number(placement.rotation, &points_path)?;
            placements.push(PatternPlacement::positioned(
                format!("p{index}"),
                placement.x,
                placement.y,
                placement.rotation,
            ));
        }

        self.lower_repeated_pattern(
            "along_path",
            item,
            transform,
            context,
            placements,
            component_stack,
        )
    }

    fn lower_distribute(
""",
)

replace_once(
    "src/authoring/limits.rs",
    """        PatternNodeRef::Distribute(distribute) => {
            validate_pattern_count(distribute.copies, 2, &format!("{path}.copies"))?;
            Ok((distribute.copies, format!("{path}.copies")))
        }
""",
    """        PatternNodeRef::Distribute(distribute) => {
            validate_pattern_count(distribute.copies, 2, &format!("{path}.copies"))?;
            Ok((distribute.copies, format!("{path}.copies")))
        }
        PatternNodeRef::AlongPath(along_path) => {
            validate_pattern_count(along_path.copies, 2, &format!("{path}.copies"))?;
            validate_path_point_count(along_path.points.len(), &format!("{path}.points"))?;
            Ok((along_path.copies, format!("{path}.copies")))
        }
""",
)
replace_once(
    "src/authoring/limits.rs",
    """fn validate_pattern_count(value: u64, minimum: u64, path: &str) -> Result<(), AuthoringError> {
    if (minimum..=MAX_PATTERN_AXIS_COUNT).contains(&value) {
        return Ok(());
    }
    Err(AuthoringError::one(AuthoringDiagnostic::new(
        path,
        "invalid_pattern_count",
        format!("pattern counts must be between {minimum} and {MAX_PATTERN_AXIS_COUNT}"),
    )))
}
""",
    """fn validate_pattern_count(value: u64, minimum: u64, path: &str) -> Result<(), AuthoringError> {
    validate_bounded_count(
        value,
        minimum,
        MAX_PATTERN_AXIS_COUNT,
        path,
        "invalid_pattern_count",
        "pattern counts",
    )
}

fn validate_path_point_count(value: usize, path: &str) -> Result<(), AuthoringError> {
    validate_bounded_count(
        u64::try_from(value).unwrap_or(u64::MAX),
        2,
        MAX_PATTERN_AXIS_COUNT,
        path,
        "invalid_path_point_count",
        "path point counts",
    )
}

fn validate_bounded_count(
    value: u64,
    minimum: u64,
    maximum: u64,
    path: &str,
    code: &str,
    label: &str,
) -> Result<(), AuthoringError> {
    if (minimum..=maximum).contains(&value) {
        return Ok(());
    }
    Err(AuthoringError::one(AuthoringDiagnostic::new(
        path,
        code,
        format!("{label} must be between {minimum} and {maximum}"),
    )))
}
""",
)

replace_once(
    "src/authoring/validation.rs",
    """            PatternNodeRef::Distribute(distribute) => {
                for (name, expression) in [
                    ("start_x", distribute.start_x),
                    ("start_y", distribute.start_y),
                    ("end_x", distribute.end_x),
                    ("end_y", distribute.end_y),
                ] {
                    validate_expression(expression, &format!("{path}.{name}"), diagnostics);
                }
                validate_transform(
                    distribute.transform,
                    &format!("{path}.transform"),
                    diagnostics,
                );
            }
""",
    """            PatternNodeRef::Distribute(distribute) => {
                for (name, expression) in [
                    ("start_x", distribute.start_x),
                    ("start_y", distribute.start_y),
                    ("end_x", distribute.end_x),
                    ("end_y", distribute.end_y),
                ] {
                    validate_expression(expression, &format!("{path}.{name}"), diagnostics);
                }
                validate_transform(
                    distribute.transform,
                    &format!("{path}.transform"),
                    diagnostics,
                );
            }
            PatternNodeRef::AlongPath(along_path) => {
                for (index, point) in along_path.points.iter().enumerate() {
                    validate_expression(
                        &point.x,
                        &format!("{path}.points[{index}].x"),
                        diagnostics,
                    );
                    validate_expression(
                        &point.y,
                        &format!("{path}.points[{index}].y"),
                        diagnostics,
                    );
                }
                validate_transform(
                    along_path.transform,
                    &format!("{path}.transform"),
                    diagnostics,
                );
            }
""",
)
replace_once(
    "src/authoring/validation.rs",
    """        | VisualNode::Mirror { .. }
        | VisualNode::Distribute { .. } => {
""",
    """        | VisualNode::Mirror { .. }
        | VisualNode::Distribute { .. }
        | VisualNode::AlongPath { .. } => {
""",
)
replace_once(
    "src/authoring/lower/node.rs",
    """            | VisualNode::Mirror { .. }
            | VisualNode::Distribute { .. } => {
""",
    """            | VisualNode::Mirror { .. }
            | VisualNode::Distribute { .. }
            | VisualNode::AlongPath { .. } => {
""",
)

replace_once(
    "cairn.blueprint",
    '"./tests/e2e.rs", "./tests/eval_contract.rs", "./tests/authoring_contract.rs"',
    '"./tests/e2e.rs", "./tests/eval_contract.rs", "./tests/authoring_along_path_contract.rs", "./tests/authoring_contract.rs"',
)
replace_once(
    "meta/contracts/authoring.md",
    "bounded deterministic grid, radial, mirror, and distribute patterns, and simple constraints",
    "bounded deterministic grid, radial, mirror, distribute, and along-path patterns, and simple constraints",
)
replace_once(
    "docs/authoring-spec-v0.md",
    "deterministic grid, radial, mirror, and distribute patterns",
    "deterministic grid, radial, mirror, distribute, and along-path patterns",
)
replace_once(
    "docs/authoring-spec-v0.md",
    "Along-path patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.",
    "Constraints, motion helpers, and statechart authoring remain separate roadmap items.",
)
replace_once(
    "docs/authoring-spec-v0.md",
    """## Raw canonical escapes
""",
    """## Along-path patterns

An `along_path` node places between two and 100 copies at equal distances along a polyline with between two and 100 authored points. Both path endpoints are included. Point coordinates use pixel expressions and may reference component parameters.

```json
{
  "kind": "along_path",
  "id": "route",
  "copies": 5,
  "points": [
    {
      "x": { "kind": "literal", "value": 0, "unit": "px" },
      "y": { "kind": "literal", "value": 0, "unit": "px" }
    },
    {
      "x": { "kind": "literal", "value": 80, "unit": "px" },
      "y": { "kind": "literal", "value": 0, "unit": "px" }
    },
    {
      "x": { "kind": "literal", "value": 80, "unit": "px" },
      "y": { "kind": "literal", "value": 60, "unit": "px" }
    }
  ],
  "rotate_items": true,
  "item": {
    "kind": "triangle",
    "id": "marker",
    "width": { "kind": "literal", "value": 18, "unit": "px" },
    "height": { "kind": "literal", "value": 12, "unit": "px" },
    "fill": "#2563EB"
  }
}
```

Spacing is measured across the complete polyline rather than independently per segment. When `rotate_items` is true, each cell follows the active segment tangent; an item exactly on an interior vertex uses the outgoing segment. The final item uses the last segment tangent. Consecutive duplicate points are rejected because they do not define a tangent. v0 intentionally models polylines only and does not infer or fit curves.

Along-path patterns use the same component expansion, runtime-name registry, source maps, raw-scene repetition safety, generated-node budget, and canonical builder path as the other bounded patterns.

## Raw canonical escapes
""",
)
replace_once(
    "meta/todos/todo.visual-authoring-compiler.md",
    """- Remaining work includes along-path patterns, constraints, and a complex static showcase
  without raw escapes.
""",
    """- Deterministic along-path patterns are implemented in PR #154. Two to 100 copies
  are spaced by total arc length across a typed polyline with two to 100 points, including
  both endpoints, optional tangent rotation, component overrides, stable source maps,
  canonical builder validation, and the inherited generated-node budget.
- Pure polyline sampling lives in a focused lowering helper with pinned `libm` distance
  and tangent math. Shared bounded-count validation now covers both pattern copies and
  path-point counts without duplicating range logic.
- Remaining work includes constraints and a complex static showcase without raw escapes.
""",
)
