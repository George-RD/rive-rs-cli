#!/usr/bin/env python3
"""Apply the bounded deterministic grid authoring implementation."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text()


def write(path: str, content: str) -> None:
    target = ROOT / path
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(content)


def replace_once(text: str, old: str, new: str, path: str) -> str:
    if text.count(old) != 1:
        raise SystemExit(f"expected one marker in {path}: {old!r}; found {text.count(old)}")
    return text.replace(old, new, 1)


visual_path = "src/authoring/visual.rs"
visual = read(visual_path)
visual = replace_once(
    visual,
    """    Group {
""",
    """    Grid {
        id: String,
        #[schemars(range(min = 1, max = 100))]
        columns: u64,
        #[schemars(range(min = 1, max = 100))]
        rows: u64,
        column_step: ScalarExpr,
        row_step: ScalarExpr,
        item: Box<VisualNode>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Group {
""",
    visual_path,
)
visual = replace_once(
    visual,
    """
impl VisualNode {
""",
    """
#[derive(Clone, Copy)]
pub(crate) struct GridNodeRef<'a> {
    pub columns: u64,
    pub rows: u64,
    pub column_step: &'a ScalarExpr,
    pub row_step: &'a ScalarExpr,
    pub item: &'a VisualNode,
    pub transform: &'a TransformSpec,
}

impl VisualNode {
""",
    visual_path,
)
visual = replace_once(
    visual,
    """            | Self::Text { id, .. }
            | Self::Group { id, .. }
""",
    """            | Self::Text { id, .. }
            | Self::Grid { id, .. }
            | Self::Group { id, .. }
""",
    visual_path,
)
visual = replace_once(
    visual,
    """            Self::Text { .. }
            | Self::Group { .. }
""",
    """            Self::Text { .. }
            | Self::Grid { .. }
            | Self::Group { .. }
""",
    visual_path,
)
visual = replace_once(
    visual,
    """    pub(crate) fn children(&self) -> Option<&[VisualNode]> {
""",
    """    pub(crate) fn grid(&self) -> Option<GridNodeRef<'_>> {
        match self {
            Self::Grid {
                columns,
                rows,
                column_step,
                row_step,
                item,
                transform,
                ..
            } => Some(GridNodeRef {
                columns: *columns,
                rows: *rows,
                column_step,
                row_step,
                item,
                transform,
            }),
            _ => None,
        }
    }

    pub(crate) fn children(&self) -> Option<&[VisualNode]> {
""",
    visual_path,
)
write(visual_path, visual)

limits_path = "src/authoring/limits.rs"
write(
    limits_path,
    """use std::collections::HashMap;

use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec, ComponentSpec};
use super::visual::VisualNode;

const MAX_COMPONENT_EXPANSION_DEPTH: usize = 64;
const MAX_GENERATED_COMPONENT_NODES: u64 = 10_000;
const MAX_PATTERN_AXIS_COUNT: u64 = 100;
const MAX_GENERATED_PATTERN_CELLS: u64 = 10_000;

#[derive(Clone, Copy)]
struct ComponentRef<'a> {
    index: usize,
    spec: &'a ComponentSpec,
}

#[derive(Default)]
struct ExpansionBudget {
    generated_component_nodes: u64,
    generated_pattern_cells: u64,
}

struct WorkItem<'a> {
    node: &'a VisualNode,
    path: String,
    active_components: Vec<usize>,
    multiplicity: u64,
    generated_by_component: bool,
    component_budget_path: Option<String>,
}

pub(crate) fn validate_expansion_limits(spec: &AuthoringSpec) -> Result<(), AuthoringError> {
    let mut components = HashMap::new();
    for (index, component) in spec.components.iter().enumerate() {
        if components
            .insert(
                component.id.as_str(),
                ComponentRef {
                    index,
                    spec: component,
                },
            )
            .is_some()
        {
            return Ok(());
        }
    }

    for (index, component) in spec.components.iter().enumerate() {
        validate_nodes(
            &component.visual,
            &format!("$.components[{index}].visual"),
            vec![index],
            &components,
            &mut ExpansionBudget::default(),
        )?;
    }

    validate_nodes(
        &spec.visual.nodes,
        "$.visual.nodes",
        Vec::new(),
        &components,
        &mut ExpansionBudget::default(),
    )
}

fn validate_nodes<'a>(
    nodes: &'a [VisualNode],
    list_path: &str,
    active_components: Vec<usize>,
    components: &HashMap<&'a str, ComponentRef<'a>>,
    budget: &mut ExpansionBudget,
) -> Result<(), AuthoringError> {
    let mut work = Vec::new();
    push_nodes(
        &mut work,
        nodes,
        list_path,
        &active_components,
        1,
        false,
        None,
    );

    while let Some(item) = work.pop() {
        if item.generated_by_component {
            budget.generated_component_nodes = budget
                .generated_component_nodes
                .checked_add(item.multiplicity)
                .unwrap_or(u64::MAX);
            if budget.generated_component_nodes > MAX_GENERATED_COMPONENT_NODES {
                return Err(AuthoringError::one(AuthoringDiagnostic::new(
                    item.component_budget_path.unwrap_or(item.path),
                    "component_expansion_node_limit",
                    format!(
                        "component expansion exceeds the maximum generated-node budget of {MAX_GENERATED_COMPONENT_NODES}"
                    ),
                )));
            }
        }

        if let Some(children) = item.node.children() {
            push_nodes(
                &mut work,
                children,
                &format!("{}.children", item.path),
                &item.active_components,
                item.multiplicity,
                item.generated_by_component,
                item.component_budget_path.as_deref(),
            );
            continue;
        }

        if let Some(grid) = item.node.grid() {
            validate_pattern_count(grid.rows, &format!("{}.rows", item.path))?;
            validate_pattern_count(grid.columns, &format!("{}.columns", item.path))?;
            let generated_cells = item
                .multiplicity
                .checked_mul(grid.rows)
                .and_then(|count| count.checked_mul(grid.columns))
                .unwrap_or(u64::MAX);
            budget.generated_pattern_cells = budget
                .generated_pattern_cells
                .checked_add(generated_cells)
                .unwrap_or(u64::MAX);
            if budget.generated_pattern_cells > MAX_GENERATED_PATTERN_CELLS {
                return Err(AuthoringError::one(AuthoringDiagnostic::new(
                    format!("{}.rows", item.path),
                    "pattern_expansion_node_limit",
                    format!(
                        "pattern expansion exceeds the maximum generated-cell budget of {MAX_GENERATED_PATTERN_CELLS}"
                    ),
                )));
            }
            work.push(WorkItem {
                node: grid.item,
                path: format!("{}.item", item.path),
                active_components: item.active_components,
                multiplicity: generated_cells,
                generated_by_component: item.generated_by_component,
                component_budget_path: item.component_budget_path,
            });
            continue;
        }

        let VisualNode::Instance { component, .. } = item.node else {
            continue;
        };
        let Some(component_ref) = components.get(component.as_str()).copied() else {
            continue;
        };
        if item.active_components.contains(&component_ref.index) {
            continue;
        }
        if item.active_components.len() >= MAX_COMPONENT_EXPANSION_DEPTH {
            return Err(AuthoringError::one(AuthoringDiagnostic::new(
                format!("{}.component", item.path),
                "component_expansion_depth_limit",
                format!(
                    "component expansion exceeds the maximum depth of {MAX_COMPONENT_EXPANSION_DEPTH}"
                ),
            )));
        }

        let mut next_active = item.active_components;
        next_active.push(component_ref.index);
        let expansion_path = item
            .component_budget_path
            .unwrap_or_else(|| format!("{}.component", item.path));
        push_nodes(
            &mut work,
            &component_ref.spec.visual,
            &format!("$.components[{}].visual", component_ref.index),
            &next_active,
            item.multiplicity,
            true,
            Some(&expansion_path),
        );
    }

    Ok(())
}

fn validate_pattern_count(value: u64, path: &str) -> Result<(), AuthoringError> {
    if (1..=MAX_PATTERN_AXIS_COUNT).contains(&value) {
        return Ok(());
    }
    Err(AuthoringError::one(AuthoringDiagnostic::new(
        path,
        "invalid_pattern_count",
        format!("grid counts must be between 1 and {MAX_PATTERN_AXIS_COUNT}"),
    )))
}

#[allow(clippy::too_many_arguments)]
fn push_nodes<'a>(
    work: &mut Vec<WorkItem<'a>>,
    nodes: &'a [VisualNode],
    list_path: &str,
    active_components: &[usize],
    multiplicity: u64,
    generated_by_component: bool,
    component_budget_path: Option<&str>,
) {
    for (index, node) in nodes.iter().enumerate().rev() {
        work.push(WorkItem {
            node,
            path: format!("{list_path}[{index}]"),
            active_components: active_components.to_vec(),
            multiplicity,
            generated_by_component,
            component_budget_path: component_budget_path.map(str::to_string),
        });
    }
}
""",
)

mod_path = "src/authoring/mod.rs"
mod = read(mod_path)
mod = replace_once(
    mod,
    "limits::validate_component_expansion_depth(spec)?;",
    "limits::validate_expansion_limits(spec)?;",
    mod_path,
)
write(mod_path, mod)

validation_path = "src/authoring/validation.rs"
validation = read(validation_path)
validation = replace_once(
    validation,
    """    match node {
""",
    """    if let Some(grid) = node.grid() {
        validate_expression(
            grid.column_step,
            &format!("{path}.column_step"),
            diagnostics,
        );
        validate_expression(grid.row_step, &format!("{path}.row_step"), diagnostics);
        validate_transform(grid.transform, &format!("{path}.transform"), diagnostics);
        validate_node(grid.item, &format!("{path}.item"), diagnostics);
        return;
    }

    match node {
""",
    validation_path,
)
validation = replace_once(
    validation,
    """        | VisualNode::Star { .. }
        | VisualNode::Text { .. } => unreachable!("shape and text nodes are handled above"),
""",
    """        | VisualNode::Star { .. }
        | VisualNode::Text { .. }
        | VisualNode::Grid { .. } => {
            unreachable!("shape, text, and grid nodes are handled above")
        }
""",
    validation_path,
)
write(validation_path, validation)

frontend_path = "src/authoring/frontend.rs"
frontend = read(frontend_path)
frontend = replace_once(
    frontend,
    """fn validate_node_names(
    nodes: &[VisualNode],
    list_path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (index, node) in nodes.iter().enumerate() {
        let path = format!("{list_path}[{index}]");
        validate_id(node.id(), &format!("{path}.id"), diagnostics);
        if let Some(children) = node.children() {
            validate_node_names(children, &format!("{path}.children"), diagnostics);
        }
        if let VisualNode::Instance { overrides, .. } = node {
            validate_parameter_names(overrides, &format!("{path}.overrides"), diagnostics);
        }
    }
}
""",
    """fn validate_node_names(
    nodes: &[VisualNode],
    list_path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (index, node) in nodes.iter().enumerate() {
        validate_node_name(node, &format!("{list_path}[{index}]"), diagnostics);
    }
}

fn validate_node_name(
    node: &VisualNode,
    path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    validate_id(node.id(), &format!("{path}.id"), diagnostics);
    if let Some(children) = node.children() {
        validate_node_names(children, &format!("{path}.children"), diagnostics);
    }
    if let Some(grid) = node.grid() {
        validate_node_name(grid.item, &format!("{path}.item"), diagnostics);
    }
    if let VisualNode::Instance { overrides, .. } = node {
        validate_parameter_names(overrides, &format!("{path}.overrides"), diagnostics);
    }
}
""",
    frontend_path,
)
write(frontend_path, frontend)

lower_path = "src/authoring/lower.rs"
lower = read(lower_path)
lower = replace_once(
    lower,
    """mod node;
mod paint;
""",
    """mod node;
mod paint;
mod pattern;
""",
    lower_path,
)
write(lower_path, lower)

node_path = "src/authoring/lower/node.rs"
node = read(node_path)
node = replace_once(
    node,
    """        if let Some(shape) = node.shape() {
""",
    """        if let Some(grid) = node.grid() {
            return self.lower_grid(grid, context, component_stack);
        }
        if let Some(shape) = node.shape() {
""",
    node_path,
)
node = replace_once(
    node,
    """            | VisualNode::Star { .. }
            | VisualNode::Text { .. } => unreachable!("shape and text nodes are handled above"),
""",
    """            | VisualNode::Star { .. }
            | VisualNode::Text { .. }
            | VisualNode::Grid { .. } => {
                unreachable!("shape, text, and grid nodes are handled above")
            }
""",
    node_path,
)
write(node_path, node)

write(
    "src/authoring/lower/pattern.rs",
    """use serde_json::{Value, json};

use super::super::expression::{evaluate_expression, evaluate_transform};
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry, Unit};
use super::super::visual::GridNodeRef;
use super::{Lowerer, NodeContext, runtime_name};

impl<'a> Lowerer<'a> {
    pub(super) fn lower_grid(
        &mut self,
        grid: GridNodeRef<'_>,
        context: NodeContext<'_>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let GridNodeRef {
            columns,
            rows,
            column_step: column_step_expression,
            row_step: row_step_expression,
            item,
            transform,
        } = grid;
        let NodeContext {
            authored_path,
            definition_path,
            authored_id,
            runtime_segments,
            scene_path,
            scope,
        } = context;

        let column_step = evaluate_expression(
            column_step_expression,
            &format!("{authored_path}.column_step"),
            scope,
            Unit::Px,
        )?;
        let row_step = evaluate_expression(
            row_step_expression,
            &format!("{authored_path}.row_step"),
            scope,
            Unit::Px,
        )?;
        let transform_values =
            evaluate_transform(transform, &format!("{authored_path}.transform"), scope)?;

        let wrapper_name = runtime_name(&runtime_segments, "grid");
        let capacity = usize::try_from(rows.saturating_mul(columns)).unwrap_or_default();
        let mut plans = Vec::with_capacity(capacity);
        let mut runtime_names = Vec::with_capacity(capacity + 1);
        let mut scene_paths = Vec::with_capacity(capacity + 1);
        runtime_names.push(wrapper_name.clone());
        scene_paths.push(scene_path.clone());

        for row in 0..rows {
            for column in 0..columns {
                let index = plans.len();
                let cell_segment = format!("r{row}c{column}");
                let mut cell_runtime_segments = runtime_segments.clone();
                cell_runtime_segments.push(cell_segment.clone());
                let cell_name = runtime_name(&cell_runtime_segments, "cell");
                let cell_scene_path = format!("{scene_path}/children/{index}");
                runtime_names.push(cell_name.clone());
                scene_paths.push(cell_scene_path.clone());
                plans.push((row, column, cell_segment, cell_name, cell_scene_path));
            }
        }

        self.register_runtime_names(&runtime_names, &format!("{authored_path}.id"))?;
        self.source_map.entries.push(SourceMapEntry {
            authored_id: authored_id.clone(),
            authored_path: authored_path.clone(),
            definition_path: definition_path.clone(),
            runtime_names,
            scene_paths,
        });

        let item_authored_path = format!("{authored_path}.item");
        let item_definition_path = definition_path
            .as_ref()
            .map(|path| format!("{path}.item"));
        let mut cells = Vec::with_capacity(capacity);
        for (row, column, cell_segment, cell_name, cell_scene_path) in plans {
            let mut item_runtime_segments = runtime_segments.clone();
            item_runtime_segments.push(cell_segment.clone());
            item_runtime_segments.push(item.id().to_string());
            let lowered_item = self.lower_node(
                item,
                NodeContext {
                    authored_path: item_authored_path.clone(),
                    definition_path: item_definition_path.clone(),
                    authored_id: format!("{authored_id}/{cell_segment}/{}", item.id()),
                    runtime_segments: item_runtime_segments,
                    scene_path: format!("{cell_scene_path}/children/0"),
                    scope,
                },
                component_stack,
            )?;
            cells.push(json!({
                "type": "node",
                "name": cell_name,
                "x": column as f64 * column_step,
                "y": row as f64 * row_step,
                "rotation": 0.0,
                "scale_x": 1.0,
                "scale_y": 1.0,
                "children": [lowered_item]
            }));
        }

        Ok(json!({
            "type": "node",
            "name": wrapper_name,
            "x": transform_values.x,
            "y": transform_values.y,
            "rotation": transform_values.rotation,
            "scale_x": transform_values.scale_x,
            "scale_y": transform_values.scale_y,
            "children": cells
        }))
    }
}
""",
)

blueprint_path = "cairn.blueprint"
blueprint = read(blueprint_path)
blueprint = replace_once(
    blueprint,
    '"./tests/authoring_gradient_contract.rs", "./tests/authoring_stroke_contract.rs"',
    '"./tests/authoring_gradient_contract.rs", "./tests/authoring_grid_contract.rs", "./tests/authoring_stroke_contract.rs"',
    blueprint_path,
)
write(blueprint_path, blueprint)

todo_path = "meta/todos/todo.visual-authoring-compiler.md"
todo = read(todo_path)
todo = replace_once(
    todo,
    """- Remaining work includes font and image assets, bounded patterns, constraints, and
""",
    """- Deterministic bounded grid patterns are implemented in PR #146. Row-major expansion,
  component parameter overrides, stable generated IDs, complete source maps, and a global
  nested-pattern cell budget are pinned by the authoring grid contract suite.
- Remaining work includes font and image assets, radial/mirror/distribute/along-path patterns,
  constraints, and
""",
    todo_path,
)
write(todo_path, todo)
