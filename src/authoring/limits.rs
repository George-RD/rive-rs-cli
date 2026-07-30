use std::collections::HashMap;

use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec, ComponentSpec};
use super::visual::{PatternNodeRef, VisualNode};

const MAX_COMPONENT_EXPANSION_DEPTH: usize = 64;
const MAX_GENERATED_COMPONENT_NODES: u64 = 10_000;
const MAX_PATTERN_AXIS_COUNT: u64 = 100;
const MAX_GENERATED_PATTERN_ITEMS: u64 = 10_000;

#[derive(Clone, Copy)]
struct ComponentRef<'a> {
    index: usize,
    spec: &'a ComponentSpec,
}

#[derive(Default)]
struct ExpansionBudget {
    generated_component_nodes: u64,
    generated_pattern_items: u64,
}

#[derive(Clone)]
struct ExpansionContext {
    active_components: Vec<usize>,
    multiplicity: u64,
    generated_by_component: bool,
    component_budget_path: Option<String>,
}

struct WorkItem<'a> {
    node: &'a VisualNode,
    path: String,
    expansion: ExpansionContext,
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
    let root_expansion = ExpansionContext {
        active_components,
        multiplicity: 1,
        generated_by_component: false,
        component_budget_path: None,
    };
    push_nodes(&mut work, nodes, list_path, &root_expansion);

    while let Some(item) = work.pop() {
        let WorkItem {
            node,
            path,
            expansion,
        } = item;

        if expansion.generated_by_component {
            budget.generated_component_nodes = budget
                .generated_component_nodes
                .saturating_add(expansion.multiplicity);
            if budget.generated_component_nodes > MAX_GENERATED_COMPONENT_NODES {
                return Err(AuthoringError::one(AuthoringDiagnostic::new(
                    expansion
                        .component_budget_path
                        .clone()
                        .unwrap_or_else(|| path.clone()),
                    "component_expansion_node_limit",
                    format!(
                        "component expansion exceeds the maximum generated-node budget of {MAX_GENERATED_COMPONENT_NODES}"
                    ),
                )));
            }
        }

        if let Some(children) = node.children() {
            push_nodes(&mut work, children, &format!("{path}.children"), &expansion);
            continue;
        }

        if let Some(pattern) = node.pattern() {
            let (item_count, limit_path) = validate_pattern(pattern, &path)?;
            let generated_items = expansion.multiplicity.saturating_mul(item_count);
            budget.generated_pattern_items = budget
                .generated_pattern_items
                .saturating_add(generated_items);
            if budget.generated_pattern_items > MAX_GENERATED_PATTERN_ITEMS {
                return Err(AuthoringError::one(AuthoringDiagnostic::new(
                    limit_path,
                    "pattern_expansion_node_limit",
                    format!(
                        "pattern expansion exceeds the maximum generated-item budget of {MAX_GENERATED_PATTERN_ITEMS}"
                    ),
                )));
            }
            work.push(WorkItem {
                node: pattern.item(),
                path: format!("{path}.item"),
                expansion: ExpansionContext {
                    multiplicity: generated_items,
                    ..expansion
                },
            });
            continue;
        }

        let VisualNode::Instance { component, .. } = node else {
            continue;
        };
        let Some(component_ref) = components.get(component.as_str()).copied() else {
            continue;
        };
        if expansion.active_components.contains(&component_ref.index) {
            continue;
        }
        if expansion.active_components.len() >= MAX_COMPONENT_EXPANSION_DEPTH {
            return Err(AuthoringError::one(AuthoringDiagnostic::new(
                format!("{path}.component"),
                "component_expansion_depth_limit",
                format!(
                    "component expansion exceeds the maximum depth of {MAX_COMPONENT_EXPANSION_DEPTH}"
                ),
            )));
        }

        let mut next_expansion = expansion;
        next_expansion.active_components.push(component_ref.index);
        next_expansion.generated_by_component = true;
        if next_expansion.component_budget_path.is_none() {
            next_expansion.component_budget_path = Some(format!("{path}.component"));
        }
        push_nodes(
            &mut work,
            &component_ref.spec.visual,
            &format!("$.components[{}].visual", component_ref.index),
            &next_expansion,
        );
    }

    Ok(())
}

fn validate_pattern(
    pattern: PatternNodeRef<'_>,
    path: &str,
) -> Result<(u64, String), AuthoringError> {
    match pattern {
        PatternNodeRef::Grid(grid) => {
            validate_pattern_count(grid.rows, &format!("{path}.rows"))?;
            validate_pattern_count(grid.columns, &format!("{path}.columns"))?;
            Ok((
                grid.rows.saturating_mul(grid.columns),
                format!("{path}.rows"),
            ))
        }
        PatternNodeRef::Radial(radial) => {
            validate_pattern_count(radial.copies, &format!("{path}.copies"))?;
            Ok((radial.copies, format!("{path}.copies")))
        }
    }
}

fn validate_pattern_count(value: u64, path: &str) -> Result<(), AuthoringError> {
    if (1..=MAX_PATTERN_AXIS_COUNT).contains(&value) {
        return Ok(());
    }
    Err(AuthoringError::one(AuthoringDiagnostic::new(
        path,
        "invalid_pattern_count",
        format!("pattern counts must be between 1 and {MAX_PATTERN_AXIS_COUNT}"),
    )))
}

fn push_nodes<'a>(
    work: &mut Vec<WorkItem<'a>>,
    nodes: &'a [VisualNode],
    list_path: &str,
    expansion: &ExpansionContext,
) {
    for (index, node) in nodes.iter().enumerate().rev() {
        work.push(WorkItem {
            node,
            path: format!("{list_path}[{index}]"),
            expansion: expansion.clone(),
        });
    }
}
