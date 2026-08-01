from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text()
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"missing replacement marker in {path}: {old[:80]!r}")
    file.write_text(text.replace(old, new, 1))


replace_once(
    "src/authoring/spec.rs",
    "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]\n#[serde(rename_all = \"snake_case\")]\npub enum GradientKind",
    """#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = \"snake_case\")]
pub enum ConstraintAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = \"kind\", rename_all = \"snake_case\", deny_unknown_fields)]
pub enum ConstraintSpec {
    Align {
        id: String,
        subject: String,
        target: String,
        axis: ConstraintAxis,
    },
    Center {
        id: String,
        subject: String,
        start: String,
        end: String,
        axis: ConstraintAxis,
    },
    Offset {
        id: String,
        subject: String,
        target: String,
        x: ScalarExpr,
        y: ScalarExpr,
    },
    Spacing {
        id: String,
        #[schemars(length(min = 2, max = 100))]
        items: Vec<String>,
        axis: ConstraintAxis,
        gap: ScalarExpr,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = \"snake_case\")]
pub enum GradientKind""",
)

replace_once(
    "src/authoring/visual.rs",
    """use super::spec::{
    PaintSpec, Quantity, ScalarExpr, StrokeSpec, TextAlign, TextOverflow, TransformSpec,
};""",
    """use super::spec::{
    ConstraintSpec, PaintSpec, Quantity, ScalarExpr, StrokeSpec, TextAlign, TextOverflow,
    TransformSpec,
};""",
)
replace_once(
    "src/authoring/visual.rs",
    """    Group {
        id: String,
        #[serde(default)]
        transform: TransformSpec,
        #[serde(default)]
        children: Vec<VisualNode>,
    },""",
    """    Group {
        id: String,
        #[serde(default)]
        transform: TransformSpec,
        #[serde(default)]
        constraints: Vec<ConstraintSpec>,
        #[serde(default)]
        children: Vec<VisualNode>,
    },""",
)

replace_once(
    "src/authoring/mod.rs",
    "mod deterministic_math;",
    "mod constraint;\nmod deterministic_math;",
)
replace_once(
    "src/authoring/mod.rs",
    """    AuthoringSourceMap, AuthoringSpec, BehaviorSection, ComponentSpec, GradientKind,
    GradientPaintSpec, GradientStopSpec, LoweredAuthoring, MotionSection, PaintSpec, Quantity,""",
    """    AuthoringSourceMap, AuthoringSpec, BehaviorSection, ComponentSpec, ConstraintAxis,
    ConstraintSpec, GradientKind, GradientPaintSpec, GradientStopSpec, LoweredAuthoring,
    MotionSection, PaintSpec, Quantity,""",
)

replace_once(
    "src/authoring/lower/node.rs",
    "use super::super::expression::evaluate_transform;",
    "use super::super::constraint::resolve_group_constraints;\nuse super::super::expression::evaluate_transform;",
)
replace_once(
    "src/authoring/lower/node.rs",
    """            VisualNode::Group {
                transform,
                children,
                ..
            } => {
                validate_sibling_ids_result(children, &format!(\"{authored_path}.children\"))?;
                let transform_values =""",
    """            VisualNode::Group {
                transform,
                constraints,
                children,
                ..
            } => {
                validate_sibling_ids_result(children, &format!(\"{authored_path}.children\"))?;
                let children =
                    resolve_group_constraints(children, constraints, &authored_path, scope)?;
                let transform_values =""",
)

replace_once(
    "src/authoring/validation.rs",
    """use super::spec::{
    AuthoringDiagnostic, AuthoringSpec, PaintSpec, Quantity, ScalarExpr, TransformSpec,
};""",
    """use super::spec::{
    AuthoringDiagnostic, AuthoringSpec, ConstraintSpec, PaintSpec, Quantity, ScalarExpr,
    TransformSpec,
};""",
)
replace_once(
    "src/authoring/validation.rs",
    """        VisualNode::Group {
            transform,
            children,
            ..
        } => {
            validate_transform(transform, &format!(\"{path}.transform\"), diagnostics);
            validate_nodes(children, &format!(\"{path}.children\"), diagnostics);
        }""",
    """        VisualNode::Group {
            transform,
            constraints,
            children,
            ..
        } => {
            validate_transform(transform, &format!(\"{path}.transform\"), diagnostics);
            for (index, constraint) in constraints.iter().enumerate() {
                let constraint_path = format!(\"{path}.constraints[{index}]\");
                match constraint {
                    ConstraintSpec::Offset { x, y, .. } => {
                        validate_expression(x, &format!(\"{constraint_path}.x\"), diagnostics);
                        validate_expression(y, &format!(\"{constraint_path}.y\"), diagnostics);
                    }
                    ConstraintSpec::Spacing { gap, .. } => {
                        validate_expression(
                            gap,
                            &format!(\"{constraint_path}.gap\"),
                            diagnostics,
                        );
                    }
                    ConstraintSpec::Align { .. } | ConstraintSpec::Center { .. } => {}
                }
            }
            validate_nodes(children, &format!(\"{path}.children\"), diagnostics);
        }""",
)
