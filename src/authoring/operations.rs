use super::{AuthoringDiagnostic, AuthoringError, AuthoringSpec, LoweredAuthoring, VisualNode};

#[derive(Debug, Clone)]
pub enum AuthoringOperation {
    ReplaceVisualNode { target_id: String, node: VisualNode },
}

#[derive(Debug, Clone)]
pub struct AppliedOperation {
    pub spec: AuthoringSpec,
    pub lowered: LoweredAuthoring,
}

pub fn apply_operation(
    spec: &AuthoringSpec,
    operation: &AuthoringOperation,
) -> Result<AppliedOperation, AuthoringError> {
    let mut working = spec.clone();

    match operation {
        AuthoringOperation::ReplaceVisualNode { target_id, node } => {
            replace_visual_node(&mut working.visual.nodes, target_id, node)?;
        }
    }

    let lowered = super::lower_authoring(&working)?;
    Ok(AppliedOperation {
        spec: working,
        lowered,
    })
}

fn replace_visual_node(
    nodes: &mut [VisualNode],
    target_id: &str,
    replacement: &VisualNode,
) -> Result<(), AuthoringError> {
    match count_visual_nodes(nodes, target_id, None) {
        0 => Err(target_error(
            "unknown_authored_id",
            target_id,
            "does not identify a visual node in the root visual tree",
        )),
        1 => {
            if replace_visual_node_once(nodes, target_id, replacement, None) {
                Ok(())
            } else {
                Err(target_error(
                    "unknown_authored_id",
                    target_id,
                    "does not identify a visual node in the root visual tree",
                ))
            }
        }
        _ => Err(target_error(
            "ambiguous_authored_id",
            target_id,
            "identifies more than one visual node in the root visual tree",
        )),
    }
}

fn target_error(code: &str, target_id: &str, detail: &str) -> AuthoringError {
    AuthoringError::one(AuthoringDiagnostic::new(
        "$.visual.nodes",
        code,
        format!("authored id `{target_id}` {detail}"),
    ))
}

fn count_visual_nodes(nodes: &[VisualNode], target_id: &str, parent_id: Option<&str>) -> usize {
    nodes
        .iter()
        .map(|node| count_visual_node(node, target_id, parent_id))
        .sum()
}

fn count_visual_node(node: &VisualNode, target_id: &str, parent_id: Option<&str>) -> usize {
    let authored_id = scoped_authored_id(parent_id, node.id());
    let own_match = usize::from(authored_id == target_id);
    own_match
        + match node {
            VisualNode::Group { children, .. } => {
                count_visual_nodes(children, target_id, Some(&authored_id))
            }
            _ => 0,
        }
}

fn replace_visual_node_once(
    nodes: &mut [VisualNode],
    target_id: &str,
    replacement: &VisualNode,
    parent_id: Option<&str>,
) -> bool {
    for node in nodes {
        if replace_visual_node_in(node, target_id, replacement, parent_id) {
            return true;
        }
    }
    false
}

fn replace_visual_node_in(
    node: &mut VisualNode,
    target_id: &str,
    replacement: &VisualNode,
    parent_id: Option<&str>,
) -> bool {
    let authored_id = scoped_authored_id(parent_id, node.id());
    if authored_id == target_id {
        *node = replacement.clone();
        return true;
    }

    match node {
        VisualNode::Group { children, .. } => {
            replace_visual_node_once(children, target_id, replacement, Some(&authored_id))
        }
        _ => false,
    }
}

fn scoped_authored_id(parent_id: Option<&str>, local_id: &str) -> String {
    match parent_id {
        Some(parent_id) => format!("{parent_id}/{local_id}"),
        None => local_id.to_string(),
    }
}
