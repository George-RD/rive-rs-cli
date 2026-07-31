from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file_path = Path(path)
    text = file_path.read_text()
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f"expected one match in {path}, found {count}: {old[:80]!r}")
    file_path.write_text(text.replace(old, new, 1))


replace_once(
    "src/authoring/visual.rs",
    """    Mirror {
        id: String,
        axis: MirrorAxis,
        item: Box<VisualNode>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Group {
""",
    """    Mirror {
        id: String,
        axis: MirrorAxis,
        item: Box<VisualNode>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Distribute {
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
)
replace_once(
    "src/authoring/visual.rs",
    """#[derive(Clone, Copy)]
pub(crate) struct MirrorNodeRef<'a> {
    pub axis: MirrorAxis,
    pub item: &'a VisualNode,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) enum PatternNodeRef<'a> {
""",
    """#[derive(Clone, Copy)]
pub(crate) struct MirrorNodeRef<'a> {
    pub axis: MirrorAxis,
    pub item: &'a VisualNode,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) struct DistributeNodeRef<'a> {
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
)
replace_once(
    "src/authoring/visual.rs",
    """pub(crate) enum PatternNodeRef<'a> {
    Grid(GridNodeRef<'a>),
    Radial(RadialNodeRef<'a>),
    Mirror(MirrorNodeRef<'a>),
}
""",
    """pub(crate) enum PatternNodeRef<'a> {
    Grid(GridNodeRef<'a>),
    Radial(RadialNodeRef<'a>),
    Mirror(MirrorNodeRef<'a>),
    Distribute(DistributeNodeRef<'a>),
}
""",
)
replace_once(
    "src/authoring/visual.rs",
    """            Self::Grid(grid) => grid.item,
            Self::Radial(radial) => radial.item,
            Self::Mirror(mirror) => mirror.item,
""",
    """            Self::Grid(grid) => grid.item,
            Self::Radial(radial) => radial.item,
            Self::Mirror(mirror) => mirror.item,
            Self::Distribute(distribute) => distribute.item,
""",
)
replace_once(
    "src/authoring/visual.rs",
    """            | Self::Radial { id, .. }
            | Self::Mirror { id, .. }
            | Self::Group { id, .. }
""",
    """            | Self::Radial { id, .. }
            | Self::Mirror { id, .. }
            | Self::Distribute { id, .. }
            | Self::Group { id, .. }
""",
)
replace_once(
    "src/authoring/visual.rs",
    """            | Self::Radial { .. }
            | Self::Mirror { .. }
            | Self::Group { .. }
""",
    """            | Self::Radial { .. }
            | Self::Mirror { .. }
            | Self::Distribute { .. }
            | Self::Group { .. }
""",
)
replace_once(
    "src/authoring/visual.rs",
    """            Self::Mirror {
                axis,
                item,
                transform,
                ..
            } => Some(PatternNodeRef::Mirror(MirrorNodeRef {
                axis: *axis,
                item,
                transform,
            })),
            _ => None,
""",
    """            Self::Mirror {
                axis,
                item,
                transform,
                ..
            } => Some(PatternNodeRef::Mirror(MirrorNodeRef {
                axis: *axis,
                item,
                transform,
            })),
            Self::Distribute {
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
)

replace_once(
    "src/authoring/lower/pattern.rs",
    """use super::super::visual::{
    GridNodeRef, MirrorAxis, MirrorNodeRef, PatternNodeRef, RadialNodeRef, VisualNode,
};
""",
    """use super::super::visual::{
    DistributeNodeRef, GridNodeRef, MirrorAxis, MirrorNodeRef, PatternNodeRef, RadialNodeRef,
    VisualNode,
};
""",
)
replace_once(
    "src/authoring/lower/pattern.rs",
    "\nimpl<'a> Lowerer<'a> {\n",
    """
fn distribute_placements(
    copies: u64,
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
) -> Vec<PatternPlacement> {
    let capacity = usize::try_from(copies).unwrap_or_default();
    let mut placements = Vec::with_capacity(capacity);
    let last = copies.saturating_sub(1);
    for index in 0..copies {
        let (x, y) = if index == 0 {
            (start_x, start_y)
        } else if index == last {
            (end_x, end_y)
        } else {
            let progress = index as f64 / last as f64;
            (
                start_x + (end_x - start_x) * progress,
                start_y + (end_y - start_y) * progress,
            )
        };
        placements.push(PatternPlacement::positioned(
            format!("d{index}"),
            x,
            y,
            0.0,
        ));
    }
    placements
}

impl<'a> Lowerer<'a> {
""",
)
replace_once(
    "src/authoring/lower/pattern.rs",
    """            PatternNodeRef::Radial(radial) => self.lower_radial(radial, context, component_stack),
            PatternNodeRef::Mirror(mirror) => self.lower_mirror(mirror, context, component_stack),
""",
    """            PatternNodeRef::Radial(radial) => self.lower_radial(radial, context, component_stack),
            PatternNodeRef::Mirror(mirror) => self.lower_mirror(mirror, context, component_stack),
            PatternNodeRef::Distribute(distribute) => {
                self.lower_distribute(distribute, context, component_stack)
            }
""",
)
replace_once(
    "src/authoring/lower/pattern.rs",
    """    fn lower_mirror(
""",
    """    fn lower_distribute(
        &mut self,
        distribute: DistributeNodeRef<'_>,
        context: NodeContext<'_>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let DistributeNodeRef {
            copies,
            start_x: start_x_expression,
            start_y: start_y_expression,
            end_x: end_x_expression,
            end_y: end_y_expression,
            item,
            transform,
        } = distribute;
        let start_x_path = format!("{}.start_x", context.authored_path);
        let start_x =
            evaluate_expression(start_x_expression, &start_x_path, context.scope, Unit::Px)?;
        let start_y_path = format!("{}.start_y", context.authored_path);
        let start_y =
            evaluate_expression(start_y_expression, &start_y_path, context.scope, Unit::Px)?;
        let end_x_path = format!("{}.end_x", context.authored_path);
        let end_x = evaluate_expression(end_x_expression, &end_x_path, context.scope, Unit::Px)?;
        let end_y_path = format!("{}.end_y", context.authored_path);
        let end_y = evaluate_expression(end_y_expression, &end_y_path, context.scope, Unit::Px)?;

        let placements = distribute_placements(copies, start_x, start_y, end_x, end_y);
        for placement in &placements {
            validate_scene_number(placement.x, &end_x_path)?;
            validate_scene_number(placement.y, &end_y_path)?;
        }

        self.lower_repeated_pattern(
            "distribute",
            item,
            transform,
            context,
            placements,
            component_stack,
        )
    }

    fn lower_mirror(
""",
)
replace_once(
    "src/authoring/lower/pattern.rs",
    """    use super::{MirrorAxis, mirror_placements};

    #[test]
    fn mirror_placements_reflect_across_the_requested_axis() {
""",
    """    use super::{MirrorAxis, distribute_placements, mirror_placements};

    #[test]
    fn distribute_placements_include_both_endpoints_and_equal_intervals() {
        let placements = distribute_placements(4, 0.0, 10.0, 90.0, 55.0);
        assert_eq!(placements.len(), 4);
        assert_eq!(placements[0].segment, "d0");
        assert_eq!(placements[0].x, 0.0);
        assert_eq!(placements[0].y, 10.0);
        assert_eq!(placements[1].segment, "d1");
        assert_eq!(placements[1].x, 30.0);
        assert_eq!(placements[1].y, 25.0);
        assert_eq!(placements[2].segment, "d2");
        assert_eq!(placements[2].x, 60.0);
        assert_eq!(placements[2].y, 40.0);
        assert_eq!(placements[3].segment, "d3");
        assert_eq!(placements[3].x, 90.0);
        assert_eq!(placements[3].y, 55.0);
    }

    #[test]
    fn mirror_placements_reflect_across_the_requested_axis() {
""",
)

replace_once(
    "src/authoring/limits.rs",
    "validate_pattern_count(grid.rows, &format!(\"{path}.rows\"))?;",
    "validate_pattern_count(grid.rows, 1, &format!(\"{path}.rows\"))?;",
)
replace_once(
    "src/authoring/limits.rs",
    "validate_pattern_count(grid.columns, &format!(\"{path}.columns\"))?;",
    "validate_pattern_count(grid.columns, 1, &format!(\"{path}.columns\"))?;",
)
replace_once(
    "src/authoring/limits.rs",
    "validate_pattern_count(radial.copies, &format!(\"{path}.copies\"))?;",
    "validate_pattern_count(radial.copies, 1, &format!(\"{path}.copies\"))?;",
)
replace_once(
    "src/authoring/limits.rs",
    """        PatternNodeRef::Mirror(_) => Ok((2, format!("{path}.item"))),
""",
    """        PatternNodeRef::Mirror(_) => Ok((2, format!("{path}.item"))),
        PatternNodeRef::Distribute(distribute) => {
            validate_pattern_count(distribute.copies, 2, &format!("{path}.copies"))?;
            Ok((distribute.copies, format!("{path}.copies")))
        }
""",
)
replace_once(
    "src/authoring/limits.rs",
    """fn validate_pattern_count(value: u64, path: &str) -> Result<(), AuthoringError> {
    if (1..=MAX_PATTERN_AXIS_COUNT).contains(&value) {
        return Ok(());
    }
    Err(AuthoringError::one(AuthoringDiagnostic::new(
        path,
        "invalid_pattern_count",
        format!("pattern counts must be between 1 and {MAX_PATTERN_AXIS_COUNT}"),
    )))
}
""",
    """fn validate_pattern_count(
    value: u64,
    minimum: u64,
    path: &str,
) -> Result<(), AuthoringError> {
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
)

replace_once(
    "src/authoring/validation.rs",
    """            PatternNodeRef::Mirror(mirror) => {
                validate_transform(mirror.transform, &format!("{path}.transform"), diagnostics);
            }
""",
    """            PatternNodeRef::Mirror(mirror) => {
                validate_transform(mirror.transform, &format!("{path}.transform"), diagnostics);
            }
            PatternNodeRef::Distribute(distribute) => {
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
)
replace_once(
    "src/authoring/validation.rs",
    """        | VisualNode::Radial { .. }
        | VisualNode::Mirror { .. } => {
""",
    """        | VisualNode::Radial { .. }
        | VisualNode::Mirror { .. }
        | VisualNode::Distribute { .. } => {
""",
)
replace_once(
    "src/authoring/lower/node.rs",
    """            | VisualNode::Radial { .. }
            | VisualNode::Mirror { .. } => {
""",
    """            | VisualNode::Radial { .. }
            | VisualNode::Mirror { .. }
            | VisualNode::Distribute { .. } => {
""",
)

replace_once(
    "cairn.blueprint",
    '"./tests/authoring_contract.rs", "./tests/authoring_examples.rs"',
    '"./tests/authoring_contract.rs", "./tests/authoring_distribute_contract.rs", "./tests/authoring_examples.rs"',
)
replace_once(
    "meta/contracts/authoring.md",
    "bounded deterministic grid, radial, and mirror patterns, and simple constraints",
    "bounded deterministic grid, radial, mirror, and distribute patterns, and simple constraints",
)
replace_once(
    "docs/authoring-spec-v0.md",
    "deterministic grid, radial, and mirror patterns",
    "deterministic grid, radial, mirror, and distribute patterns",
)
replace_once(
    "docs/authoring-spec-v0.md",
    "Distribute/along-path patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.",
    "Along-path patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.",
)
replace_once(
    "docs/authoring-spec-v0.md",
    """## Raw canonical escapes
""",
    """## Distribute patterns

A `distribute` node places between two and 100 copies at equal intervals along a straight authored segment. Both endpoints are included. The four endpoint expressions use pixel units and may reference component parameters.

```json
{
  "kind": "distribute",
  "id": "steps",
  "copies": 4,
  "start_x": { "kind": "literal", "value": 0, "unit": "px" },
  "start_y": { "kind": "literal", "value": 0, "unit": "px" },
  "end_x": { "kind": "literal", "value": 120, "unit": "px" },
  "end_y": { "kind": "literal", "value": 60, "unit": "px" },
  "item": {
    "kind": "ellipse",
    "id": "dot",
    "width": { "kind": "literal", "value": 16, "unit": "px" },
    "height": { "kind": "literal", "value": 16, "unit": "px" },
    "fill": "#2563EB"
  }
}
```

This example emits cells at `(0, 0)`, `(40, 20)`, `(80, 40)`, and `(120, 60)`. The pattern transform wraps the complete distribution, while the item keeps its own transform inside every cell. Distribution uses the same component expansion, runtime-name registry, source maps, raw-scene repetition safety, generated-node budget, and canonical builder path as the other bounded patterns.

## Raw canonical escapes
""",
)
replace_once(
    "meta/todos/todo.visual-authoring-compiler.md",
    """- Remaining work includes distribute/along-path patterns, constraints, and a complex
  static showcase without raw escapes.
""",
    """- Endpoint-inclusive distribute patterns are implemented in PR #153. Two to 100
  copies lower at equal intervals along a typed straight segment, including both authored
  endpoints, component overrides, definition paths, source maps, runtime-name collision
  checks, canonical builder validation, and the inherited generated-node budget.
- Distribute lowering reuses the shared placement and repeated-pattern pipeline. Pattern
  count validation now accepts primitive-specific minimums instead of duplicating a
  separate bound check for the new node.
- Remaining work includes along-path patterns, constraints, and a complex static showcase
  without raw escapes.
""",
)
