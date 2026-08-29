use super::spec::{
    AuthoringDiagnostic, AuthoringError, AuthoringSpec, BehaviorBindingSpec, BehaviorModelSpec,
    BehaviorStatechartSpec, ComponentSpec, LoweredAuthoring, MotionEasingSpec, MotionTrackSpec,
    PoseSpec, RawSceneFragment,
};
use super::visual::VisualNode;

#[derive(Debug, Clone)]
pub enum AuthoringEntity {
    VisualNode(VisualNode),
    Component(ComponentSpec),
    MotionEasing(MotionEasingSpec),
    MotionPose(PoseSpec),
    MotionTrack(MotionTrackSpec),
    MotionRawAnimation(RawSceneFragment),
    BehaviorModel(BehaviorModelSpec),
    BehaviorBinding(BehaviorBindingSpec),
    BehaviorStatechart(BehaviorStatechartSpec),
    BehaviorRawStateMachine(RawSceneFragment),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthoringTarget {
    VisualNode { target_id: String },
    Component { target_id: String },
    MotionEasing { target_id: String },
    MotionPose { target_id: String },
    MotionTrack { target_id: String },
    MotionRawAnimation { target_id: String },
    BehaviorModel { target_id: String },
    BehaviorBinding { target_id: String },
    BehaviorStatechart { target_id: String },
    BehaviorRawStateMachine { target_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthoringContainer {
    VisualRoot,
    VisualGroup { target_id: String },
    Components,
    MotionEasings,
    MotionPoses,
    MotionTracks,
    MotionRawAnimations,
    BehaviorModels,
    BehaviorBindings,
    BehaviorStatecharts,
    BehaviorRawStateMachines,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthoringPlacement {
    Into { container: AuthoringContainer },
    Before { anchor: AuthoringTarget },
    After { anchor: AuthoringTarget },
}

#[derive(Debug, Clone)]
pub enum AuthoringOperation {
    ReplaceVisualNode { target_id: String, node: VisualNode },
    Insert {
        entity: AuthoringEntity,
        placement: AuthoringPlacement,
    },
    Move {
        target: AuthoringTarget,
        placement: AuthoringPlacement,
    },
    Remove { target: AuthoringTarget },
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
    apply_operations(spec, std::slice::from_ref(operation))
}

pub fn apply_operations(
    spec: &AuthoringSpec,
    operations: &[AuthoringOperation],
) -> Result<AppliedOperation, AuthoringError> {
    let mut working = spec.clone();
    let mut lowered = None;

    for operation in operations {
        mutate(&mut working, operation)?;
        lowered = Some(super::lower_authoring(&working)?);
    }

    let lowered = match lowered {
        Some(lowered) => lowered,
        None => super::lower_authoring(&working)?,
    };

    Ok(AppliedOperation {
        spec: working,
        lowered,
    })
}

fn mutate(spec: &mut AuthoringSpec, operation: &AuthoringOperation) -> Result<(), AuthoringError> {
    match operation {
        AuthoringOperation::ReplaceVisualNode { target_id, node } => {
            replace_visual_node(&mut spec.visual.nodes, target_id, node)
        }
        AuthoringOperation::Insert { entity, placement } => {
            insert_entity(spec, entity.clone(), placement)
        }
        AuthoringOperation::Move { target, placement } => {
            let entity = remove_entity(spec, target)?;
            insert_entity(spec, entity, placement)
        }
        AuthoringOperation::Remove { target } => {
            remove_entity(spec, target)?;
            Ok(())
        }
    }
}

fn insert_entity(
    spec: &mut AuthoringSpec,
    entity: AuthoringEntity,
    placement: &AuthoringPlacement,
) -> Result<(), AuthoringError> {
    match entity {
        AuthoringEntity::VisualNode(node) => insert_visual(spec, node, placement),
        AuthoringEntity::Component(component) => match placement {
            AuthoringPlacement::Into {
                container: AuthoringContainer::Components,
            } => {
                spec.components.push(component);
                Ok(())
            }
            AuthoringPlacement::Before {
                anchor: AuthoringTarget::Component { target_id },
            } => insert_relative(
                &mut spec.components,
                target_id,
                component,
                false,
                "$.components",
                |item| item.id.as_str(),
            ),
            AuthoringPlacement::After {
                anchor: AuthoringTarget::Component { target_id },
            } => insert_relative(
                &mut spec.components,
                target_id,
                component,
                true,
                "$.components",
                |item| item.id.as_str(),
            ),
            _ => Err(invalid_placement("component", "$.components")),
        },
        AuthoringEntity::MotionEasing(easing) => match placement {
            AuthoringPlacement::Into {
                container: AuthoringContainer::MotionEasings,
            } => {
                spec.motion.easings.push(easing);
                Ok(())
            }
            AuthoringPlacement::Before {
                anchor: AuthoringTarget::MotionEasing { target_id },
            } => insert_relative(
                &mut spec.motion.easings,
                target_id,
                easing,
                false,
                "$.motion.easings",
                MotionEasingSpec::id,
            ),
            AuthoringPlacement::After {
                anchor: AuthoringTarget::MotionEasing { target_id },
            } => insert_relative(
                &mut spec.motion.easings,
                target_id,
                easing,
                true,
                "$.motion.easings",
                MotionEasingSpec::id,
            ),
            _ => Err(invalid_placement("motion easing", "$.motion.easings")),
        },
        AuthoringEntity::MotionPose(pose) => match placement {
            AuthoringPlacement::Into {
                container: AuthoringContainer::MotionPoses,
            } => {
                spec.motion.poses.push(pose);
                Ok(())
            }
            AuthoringPlacement::Before {
                anchor: AuthoringTarget::MotionPose { target_id },
            } => insert_relative(
                &mut spec.motion.poses,
                target_id,
                pose,
                false,
                "$.motion.poses",
                |item| item.id.as_str(),
            ),
            AuthoringPlacement::After {
                anchor: AuthoringTarget::MotionPose { target_id },
            } => insert_relative(
                &mut spec.motion.poses,
                target_id,
                pose,
                true,
                "$.motion.poses",
                |item| item.id.as_str(),
            ),
            _ => Err(invalid_placement("motion pose", "$.motion.poses")),
        },
        AuthoringEntity::MotionTrack(track) => match placement {
            AuthoringPlacement::Into {
                container: AuthoringContainer::MotionTracks,
            } => {
                spec.motion.tracks.push(track);
                Ok(())
            }
            AuthoringPlacement::Before {
                anchor: AuthoringTarget::MotionTrack { target_id },
            } => insert_relative(
                &mut spec.motion.tracks,
                target_id,
                track,
                false,
                "$.motion.tracks",
                |item| item.id.as_str(),
            ),
            AuthoringPlacement::After {
                anchor: AuthoringTarget::MotionTrack { target_id },
            } => insert_relative(
                &mut spec.motion.tracks,
                target_id,
                track,
                true,
                "$.motion.tracks",
                |item| item.id.as_str(),
            ),
            _ => Err(invalid_placement("motion track", "$.motion.tracks")),
        },
        AuthoringEntity::MotionRawAnimation(fragment) => match placement {
            AuthoringPlacement::Into {
                container: AuthoringContainer::MotionRawAnimations,
            } => {
                spec.motion.raw_animations.push(fragment);
                Ok(())
            }
            AuthoringPlacement::Before {
                anchor: AuthoringTarget::MotionRawAnimation { target_id },
            } => insert_relative(
                &mut spec.motion.raw_animations,
                target_id,
                fragment,
                false,
                "$.motion.raw_animations",
                |item| item.id.as_str(),
            ),
            AuthoringPlacement::After {
                anchor: AuthoringTarget::MotionRawAnimation { target_id },
            } => insert_relative(
                &mut spec.motion.raw_animations,
                target_id,
                fragment,
                true,
                "$.motion.raw_animations",
                |item| item.id.as_str(),
            ),
            _ => Err(invalid_placement(
                "raw motion animation",
                "$.motion.raw_animations",
            )),
        },
        AuthoringEntity::BehaviorModel(model) => match placement {
            AuthoringPlacement::Into {
                container: AuthoringContainer::BehaviorModels,
            } => {
                spec.behavior.models.push(model);
                Ok(())
            }
            AuthoringPlacement::Before {
                anchor: AuthoringTarget::BehaviorModel { target_id },
            } => insert_relative(
                &mut spec.behavior.models,
                target_id,
                model,
                false,
                "$.behavior.models",
                |item| item.id.as_str(),
            ),
            AuthoringPlacement::After {
                anchor: AuthoringTarget::BehaviorModel { target_id },
            } => insert_relative(
                &mut spec.behavior.models,
                target_id,
                model,
                true,
                "$.behavior.models",
                |item| item.id.as_str(),
            ),
            _ => Err(invalid_placement("behavior model", "$.behavior.models")),
        },
        AuthoringEntity::BehaviorBinding(binding) => match placement {
            AuthoringPlacement::Into {
                container: AuthoringContainer::BehaviorBindings,
            } => {
                spec.behavior.bindings.push(binding);
                Ok(())
            }
            AuthoringPlacement::Before {
                anchor: AuthoringTarget::BehaviorBinding { target_id },
            } => insert_relative(
                &mut spec.behavior.bindings,
                target_id,
                binding,
                false,
                "$.behavior.bindings",
                |item| item.id.as_str(),
            ),
            AuthoringPlacement::After {
                anchor: AuthoringTarget::BehaviorBinding { target_id },
            } => insert_relative(
                &mut spec.behavior.bindings,
                target_id,
                binding,
                true,
                "$.behavior.bindings",
                |item| item.id.as_str(),
            ),
            _ => Err(invalid_placement(
                "behavior binding",
                "$.behavior.bindings",
            )),
        },
        AuthoringEntity::BehaviorStatechart(statechart) => match placement {
            AuthoringPlacement::Into {
                container: AuthoringContainer::BehaviorStatecharts,
            } => {
                spec.behavior.statecharts.push(statechart);
                Ok(())
            }
            AuthoringPlacement::Before {
                anchor: AuthoringTarget::BehaviorStatechart { target_id },
            } => insert_relative(
                &mut spec.behavior.statecharts,
                target_id,
                statechart,
                false,
                "$.behavior.statecharts",
                |item| item.id.as_str(),
            ),
            AuthoringPlacement::After {
                anchor: AuthoringTarget::BehaviorStatechart { target_id },
            } => insert_relative(
                &mut spec.behavior.statecharts,
                target_id,
                statechart,
                true,
                "$.behavior.statecharts",
                |item| item.id.as_str(),
            ),
            _ => Err(invalid_placement(
                "behavior statechart",
                "$.behavior.statecharts",
            )),
        },
        AuthoringEntity::BehaviorRawStateMachine(fragment) => match placement {
            AuthoringPlacement::Into {
                container: AuthoringContainer::BehaviorRawStateMachines,
            } => {
                spec.behavior.raw_state_machines.push(fragment);
                Ok(())
            }
            AuthoringPlacement::Before {
                anchor: AuthoringTarget::BehaviorRawStateMachine { target_id },
            } => insert_relative(
                &mut spec.behavior.raw_state_machines,
                target_id,
                fragment,
                false,
                "$.behavior.raw_state_machines",
                |item| item.id.as_str(),
            ),
            AuthoringPlacement::After {
                anchor: AuthoringTarget::BehaviorRawStateMachine { target_id },
            } => insert_relative(
                &mut spec.behavior.raw_state_machines,
                target_id,
                fragment,
                true,
                "$.behavior.raw_state_machines",
                |item| item.id.as_str(),
            ),
            _ => Err(invalid_placement(
                "raw behavior state machine",
                "$.behavior.raw_state_machines",
            )),
        },
    }
}

fn insert_visual(
    spec: &mut AuthoringSpec,
    node: VisualNode,
    placement: &AuthoringPlacement,
) -> Result<(), AuthoringError> {
    match placement {
        AuthoringPlacement::Into {
            container: AuthoringContainer::VisualRoot,
        } => {
            spec.visual.nodes.push(node);
            Ok(())
        }
        AuthoringPlacement::Into {
            container: AuthoringContainer::VisualGroup { target_id },
        } => append_visual_to_group(&mut spec.visual.nodes, target_id, node),
        AuthoringPlacement::Before {
            anchor: AuthoringTarget::VisualNode { target_id },
        } => insert_visual_relative(&mut spec.visual.nodes, target_id, node, false),
        AuthoringPlacement::After {
            anchor: AuthoringTarget::VisualNode { target_id },
        } => insert_visual_relative(&mut spec.visual.nodes, target_id, node, true),
        _ => Err(invalid_placement("visual node", "$.visual.nodes")),
    }
}

fn remove_entity(
    spec: &mut AuthoringSpec,
    target: &AuthoringTarget,
) -> Result<AuthoringEntity, AuthoringError> {
    match target {
        AuthoringTarget::VisualNode { target_id } => remove_visual_node(&mut spec.visual.nodes, target_id)
            .map(AuthoringEntity::VisualNode),
        AuthoringTarget::Component { target_id } => remove_unique(
            &mut spec.components,
            target_id,
            "$.components",
            |item| item.id.as_str(),
        )
        .map(AuthoringEntity::Component),
        AuthoringTarget::MotionEasing { target_id } => remove_unique(
            &mut spec.motion.easings,
            target_id,
            "$.motion.easings",
            MotionEasingSpec::id,
        )
        .map(AuthoringEntity::MotionEasing),
        AuthoringTarget::MotionPose { target_id } => remove_unique(
            &mut spec.motion.poses,
            target_id,
            "$.motion.poses",
            |item| item.id.as_str(),
        )
        .map(AuthoringEntity::MotionPose),
        AuthoringTarget::MotionTrack { target_id } => remove_unique(
            &mut spec.motion.tracks,
            target_id,
            "$.motion.tracks",
            |item| item.id.as_str(),
        )
        .map(AuthoringEntity::MotionTrack),
        AuthoringTarget::MotionRawAnimation { target_id } => remove_unique(
            &mut spec.motion.raw_animations,
            target_id,
            "$.motion.raw_animations",
            |item| item.id.as_str(),
        )
        .map(AuthoringEntity::MotionRawAnimation),
        AuthoringTarget::BehaviorModel { target_id } => remove_unique(
            &mut spec.behavior.models,
            target_id,
            "$.behavior.models",
            |item| item.id.as_str(),
        )
        .map(AuthoringEntity::BehaviorModel),
        AuthoringTarget::BehaviorBinding { target_id } => remove_unique(
            &mut spec.behavior.bindings,
            target_id,
            "$.behavior.bindings",
            |item| item.id.as_str(),
        )
        .map(AuthoringEntity::BehaviorBinding),
        AuthoringTarget::BehaviorStatechart { target_id } => remove_unique(
            &mut spec.behavior.statecharts,
            target_id,
            "$.behavior.statecharts",
            |item| item.id.as_str(),
        )
        .map(AuthoringEntity::BehaviorStatechart),
        AuthoringTarget::BehaviorRawStateMachine { target_id } => remove_unique(
            &mut spec.behavior.raw_state_machines,
            target_id,
            "$.behavior.raw_state_machines",
            |item| item.id.as_str(),
        )
        .map(AuthoringEntity::BehaviorRawStateMachine),
    }
}

fn replace_visual_node(
    nodes: &mut [VisualNode],
    target_id: &str,
    replacement: &VisualNode,
) -> Result<(), AuthoringError> {
    match count_visual_nodes(nodes, target_id, None) {
        0 => Err(target_error(
            "$.visual.nodes",
            "unknown_authored_id",
            target_id,
            "does not identify a visual node in the root visual tree",
        )),
        1 => {
            if replace_visual_node_once(nodes, target_id, replacement, None) {
                Ok(())
            } else {
                Err(target_error(
                    "$.visual.nodes",
                    "unknown_authored_id",
                    target_id,
                    "does not identify a visual node in the root visual tree",
                ))
            }
        }
        _ => Err(target_error(
            "$.visual.nodes",
            "ambiguous_authored_id",
            target_id,
            "identifies more than one visual node in the root visual tree",
        )),
    }
}

fn append_visual_to_group(
    nodes: &mut [VisualNode],
    target_id: &str,
    node: VisualNode,
) -> Result<(), AuthoringError> {
    match count_visual_groups(nodes, target_id, None) {
        0 => Err(target_error(
            "$.visual.nodes",
            "unknown_authored_id",
            target_id,
            "does not identify a group in the root visual tree",
        )),
        1 => {
            let mut node = Some(node);
            if append_visual_to_group_once(nodes, target_id, &mut node, None) {
                Ok(())
            } else {
                Err(target_error(
                    "$.visual.nodes",
                    "unknown_authored_id",
                    target_id,
                    "does not identify a group in the root visual tree",
                ))
            }
        }
        _ => Err(target_error(
            "$.visual.nodes",
            "ambiguous_authored_id",
            target_id,
            "identifies more than one group in the root visual tree",
        )),
    }
}

fn insert_visual_relative(
    nodes: &mut Vec<VisualNode>,
    target_id: &str,
    node: VisualNode,
    after: bool,
) -> Result<(), AuthoringError> {
    match count_visual_nodes(nodes, target_id, None) {
        0 => Err(target_error(
            "$.visual.nodes",
            "unknown_authored_id",
            target_id,
            "does not identify a visual node in the root visual tree",
        )),
        1 => {
            let mut node = Some(node);
            if insert_visual_relative_once(nodes, target_id, &mut node, after, None) {
                Ok(())
            } else {
                Err(target_error(
                    "$.visual.nodes",
                    "unknown_authored_id",
                    target_id,
                    "does not identify a visual node in the root visual tree",
                ))
            }
        }
        _ => Err(target_error(
            "$.visual.nodes",
            "ambiguous_authored_id",
            target_id,
            "identifies more than one visual node in the root visual tree",
        )),
    }
}

fn remove_visual_node(
    nodes: &mut Vec<VisualNode>,
    target_id: &str,
) -> Result<VisualNode, AuthoringError> {
    match count_visual_nodes(nodes, target_id, None) {
        0 => Err(target_error(
            "$.visual.nodes",
            "unknown_authored_id",
            target_id,
            "does not identify a visual node in the root visual tree",
        )),
        1 => remove_visual_node_once(nodes, target_id, None).ok_or_else(|| {
            target_error(
                "$.visual.nodes",
                "unknown_authored_id",
                target_id,
                "does not identify a visual node in the root visual tree",
            )
        }),
        _ => Err(target_error(
            "$.visual.nodes",
            "ambiguous_authored_id",
            target_id,
            "identifies more than one visual node in the root visual tree",
        )),
    }
}

fn insert_relative<T, F>(
    items: &mut Vec<T>,
    target_id: &str,
    item: T,
    after: bool,
    path: &str,
    id_of: F,
) -> Result<(), AuthoringError>
where
    F: Fn(&T) -> &str,
{
    let index = unique_index(items, target_id, path, &id_of)?;
    items.insert(index + usize::from(after), item);
    Ok(())
}

fn remove_unique<T, F>(
    items: &mut Vec<T>,
    target_id: &str,
    path: &str,
    id_of: F,
) -> Result<T, AuthoringError>
where
    F: Fn(&T) -> &str,
{
    let index = unique_index(items, target_id, path, &id_of)?;
    Ok(items.remove(index))
}

fn unique_index<T, F>(
    items: &[T],
    target_id: &str,
    path: &str,
    id_of: &F,
) -> Result<usize, AuthoringError>
where
    F: Fn(&T) -> &str,
{
    let matches = items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| (id_of(item) == target_id).then_some(index))
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Err(target_error(
            path,
            "unknown_authored_id",
            target_id,
            "does not identify an authored entity in this container",
        )),
        [index] => Ok(*index),
        _ => Err(target_error(
            path,
            "ambiguous_authored_id",
            target_id,
            "identifies more than one authored entity in this container",
        )),
    }
}

fn invalid_placement(entity: &str, path: &str) -> AuthoringError {
    AuthoringError::one(AuthoringDiagnostic::new(
        path,
        "invalid_operation_placement",
        format!("placement is not compatible with {entity}"),
    ))
}

fn target_error(code_path: &str, code: &str, target_id: &str, detail: &str) -> AuthoringError {
    AuthoringError::one(AuthoringDiagnostic::new(
        code_path,
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

fn count_visual_groups(nodes: &[VisualNode], target_id: &str, parent_id: Option<&str>) -> usize {
    nodes
        .iter()
        .map(|node| {
            let authored_id = scoped_authored_id(parent_id, node.id());
            match node {
                VisualNode::Group { children, .. } => {
                    usize::from(authored_id == target_id)
                        + count_visual_groups(children, target_id, Some(&authored_id))
                }
                _ => 0,
            }
        })
        .sum()
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

fn append_visual_to_group_once(
    nodes: &mut [VisualNode],
    target_id: &str,
    node: &mut Option<VisualNode>,
    parent_id: Option<&str>,
) -> bool {
    for current in nodes {
        let authored_id = scoped_authored_id(parent_id, current.id());
        if let VisualNode::Group { children, .. } = current {
            if authored_id == target_id {
                children.push(node.take().expect("node is consumed once"));
                return true;
            }
            if append_visual_to_group_once(children, target_id, node, Some(&authored_id)) {
                return true;
            }
        }
    }
    false
}

fn insert_visual_relative_once(
    nodes: &mut Vec<VisualNode>,
    target_id: &str,
    node: &mut Option<VisualNode>,
    after: bool,
    parent_id: Option<&str>,
) -> bool {
    let mut index = 0;
    while index < nodes.len() {
        let authored_id = scoped_authored_id(parent_id, nodes[index].id());
        if authored_id == target_id {
            nodes.insert(
                index + usize::from(after),
                node.take().expect("node is consumed once"),
            );
            return true;
        }
        if let VisualNode::Group { children, .. } = &mut nodes[index]
            && insert_visual_relative_once(children, target_id, node, after, Some(&authored_id))
        {
            return true;
        }
        index += 1;
    }
    false
}

fn remove_visual_node_once(
    nodes: &mut Vec<VisualNode>,
    target_id: &str,
    parent_id: Option<&str>,
) -> Option<VisualNode> {
    let mut index = 0;
    while index < nodes.len() {
        let authored_id = scoped_authored_id(parent_id, nodes[index].id());
        if authored_id == target_id {
            return Some(nodes.remove(index));
        }
        if let VisualNode::Group { children, .. } = &mut nodes[index]
            && let Some(removed) = remove_visual_node_once(children, target_id, Some(&authored_id))
        {
            return Some(removed);
        }
        index += 1;
    }
    None
}

fn scoped_authored_id(parent_id: Option<&str>, local_id: &str) -> String {
    match parent_id {
        Some(parent_id) => format!("{parent_id}/{local_id}"),
        None => local_id.to_string(),
    }
}
