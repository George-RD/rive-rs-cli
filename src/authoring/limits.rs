use std::collections::HashMap;

use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec, ComponentSpec, VisualNode};

const MAX_COMPONENT_EXPANSION_DEPTH: usize = 64;

#[derive(Clone, Copy)]
struct ComponentRef<'a> {
    index: usize,
    spec: &'a ComponentSpec,
}

struct WorkItem<'a> {
    node: &'a VisualNode,
    path: String,
    active_components: Vec<usize>,
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
        validate_nodes(
            &component.visual,
            &format!("$.components[{index}].visual"),
            vec![index],
            &components,
        )?;
    }
    validate_nodes(
        &spec.visual.nodes,
        "$.visual.nodes",
        Vec::new(),
        &components,
    )
}

fn validate_nodes<'a>(
    nodes: &'a [VisualNode],
    list_path: &str,
    active_components: Vec<usize>,
    components: &HashMap<&'a str, ComponentRef<'a>>,
) -> Result<(), AuthoringError> {
    let mut work = Vec::new();
    push_nodes(&mut work, nodes, list_path, &active_components);

    while let Some(item) = work.pop() {
        match item.node {
            VisualNode::Group { children, .. } => push_nodes(
                &mut work,
                children,
                &format!("{}.children", item.path),
                &item.active_components,
            ),
            VisualNode::Instance { component, .. } => {
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
                push_nodes(
                    &mut work,
                    &component_ref.spec.visual,
                    &format!("$.components[{}].visual", component_ref.index),
                    &next_active,
                );
            }
            VisualNode::Ellipse { .. }
            | VisualNode::Rectangle { .. }
            | VisualNode::RawSceneObject { .. } => {}
        }
    }

    Ok(())
}

fn push_nodes<'a>(
    work: &mut Vec<WorkItem<'a>>,
    nodes: &'a [VisualNode],
    list_path: &str,
    active_components: &[usize],
) {
    for (index, node) in nodes.iter().enumerate().rev() {
        work.push(WorkItem {
            node,
            path: format!("{list_path}[{index}]"),
            active_components: active_components.to_vec(),
        });
    }
}
