use std::collections::HashMap;

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
                .saturating_add(item.multiplicity);
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
                .saturating_add(generated_cells);
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
