use std::collections::HashMap;

use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec, ComponentSpec, VisualNode};

const MAX_COMPONENT_EXPANSION_DEPTH: usize = 64;
const MAX_GENERATED_COMPONENT_NODES: usize = 10_000;

#[derive(Clone, Copy)]
struct ComponentRef<'a> {
    index: usize,
    spec: &'a ComponentSpec,
}

struct WorkItem<'a> {
    node: &'a VisualNode,
    path: String,
    active_components: Vec<usize>,
    generated: bool,
    budget_path: String,
}

pub(crate) fn validate_component_expansion_depth(
    spec: &AuthoringSpec,
) -> Result<(), AuthoringError> {
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
        let mut generated_nodes = 0;
        validate_nodes(
            &component.visual,
            &format!("$.components[{index}].visual"),
            vec![index],
            &components,
            &mut generated_nodes,
        )?;
    }

    let mut generated_nodes = 0;
    validate_nodes(
        &spec.visual.nodes,
        "$.visual.nodes",
        Vec::new(),
        &components,
        &mut generated_nodes,
    )
}

fn validate_nodes<'a>(
    nodes: &'a [VisualNode],
    list_path: &str,
    active_components: Vec<usize>,
    components: &HashMap<&'a str, ComponentRef<'a>>,
    generated_nodes: &mut usize,
) -> Result<(), AuthoringError> {
    let mut work = Vec::new();
    push_nodes(
        &mut work,
        nodes,
        list_path,
        &active_components,
        false,
        list_path,
    );

    while let Some(item) = work.pop() {
        if item.generated {
            *generated_nodes += 1;
            if *generated_nodes > MAX_GENERATED_COMPONENT_NODES {
                return Err(AuthoringError::one(AuthoringDiagnostic::new(
                    item.budget_path,
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
                item.generated,
                &item.budget_path,
            );
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
        let expansion_path = format!("{}.component", item.path);
        push_nodes(
            &mut work,
            &component_ref.spec.visual,
            &format!("$.components[{}].visual", component_ref.index),
            &next_active,
            true,
            &expansion_path,
        );
    }

    Ok(())
}

fn push_nodes<'a>(
    work: &mut Vec<WorkItem<'a>>,
    nodes: &'a [VisualNode],
    list_path: &str,
    active_components: &[usize],
    generated: bool,
    budget_path: &str,
) {
    for (index, node) in nodes.iter().enumerate().rev() {
        work.push(WorkItem {
            node,
            path: format!("{list_path}[{index}]"),
            active_components: active_components.to_vec(),
            generated,
            budget_path: budget_path.to_string(),
        });
    }
}
