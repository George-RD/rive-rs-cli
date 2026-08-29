use super::spec::{
    AuthoringDiagnostic, AuthoringError, AuthoringSpec, BehaviorBindingSpec, BehaviorModelSpec,
    BehaviorStatechartSpec, ComponentSpec, LoweredAuthoring, MotionEasingSpec, MotionTrackSpec,
    PoseSpec, RawSceneFragment,
};
use super::visual::VisualNode;

#[derive(Debug, Clone)]
pub enum AuthoringEntity {
    VisualNode(Box<VisualNode>),
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

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum AuthoringOperation {
    ReplaceVisualNode {
        target_id: String,
        node: VisualNode,
    },
    Insert {
        entity: AuthoringEntity,
        placement: AuthoringPlacement,
    },
    Move {
        target: AuthoringTarget,
        placement: AuthoringPlacement,
    },
    Remove {
        target: AuthoringTarget,
    },
}

#[derive(Debug, Clone)]
pub struct AppliedOperation {
    pub spec: AuthoringSpec,
    pub lowered: LoweredAuthoring,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntityKind {
    Visual,
    Component,
    MotionEasing,
    MotionPose,
    MotionTrack,
    MotionRawAnimation,
    BehaviorModel,
    BehaviorBinding,
    BehaviorStatechart,
    BehaviorRawStateMachine,
}

enum ListPlacement<'a> {
    Append,
    Before(&'a str),
    After(&'a str),
}

trait AuthoredId {
    fn authored_id(&self) -> &str;
}

macro_rules! field_authored_id {
    ($type:ty) => {
        impl AuthoredId for $type {
            fn authored_id(&self) -> &str {
                self.id.as_str()
            }
        }
    };
}

field_authored_id!(ComponentSpec);
field_authored_id!(PoseSpec);
field_authored_id!(MotionTrackSpec);
field_authored_id!(RawSceneFragment);
field_authored_id!(BehaviorModelSpec);
field_authored_id!(BehaviorBindingSpec);
field_authored_id!(BehaviorStatechartSpec);

impl AuthoredId for MotionEasingSpec {
    fn authored_id(&self) -> &str {
        self.id()
    }
}

impl AuthoringEntity {
    fn kind(&self) -> EntityKind {
        match self {
            Self::VisualNode(_) => EntityKind::Visual,
            Self::Component(_) => EntityKind::Component,
            Self::MotionEasing(_) => EntityKind::MotionEasing,
            Self::MotionPose(_) => EntityKind::MotionPose,
            Self::MotionTrack(_) => EntityKind::MotionTrack,
            Self::MotionRawAnimation(_) => EntityKind::MotionRawAnimation,
            Self::BehaviorModel(_) => EntityKind::BehaviorModel,
            Self::BehaviorBinding(_) => EntityKind::BehaviorBinding,
            Self::BehaviorStatechart(_) => EntityKind::BehaviorStatechart,
            Self::BehaviorRawStateMachine(_) => EntityKind::BehaviorRawStateMachine,
        }
    }
}

impl AuthoringTarget {
    fn kind(&self) -> EntityKind {
        match self {
            Self::VisualNode { .. } => EntityKind::Visual,
            Self::Component { .. } => EntityKind::Component,
            Self::MotionEasing { .. } => EntityKind::MotionEasing,
            Self::MotionPose { .. } => EntityKind::MotionPose,
            Self::MotionTrack { .. } => EntityKind::MotionTrack,
            Self::MotionRawAnimation { .. } => EntityKind::MotionRawAnimation,
            Self::BehaviorModel { .. } => EntityKind::BehaviorModel,
            Self::BehaviorBinding { .. } => EntityKind::BehaviorBinding,
            Self::BehaviorStatechart { .. } => EntityKind::BehaviorStatechart,
            Self::BehaviorRawStateMachine { .. } => EntityKind::BehaviorRawStateMachine,
        }
    }

    fn target_id(&self) -> &str {
        match self {
            Self::VisualNode { target_id }
            | Self::Component { target_id }
            | Self::MotionEasing { target_id }
            | Self::MotionPose { target_id }
            | Self::MotionTrack { target_id }
            | Self::MotionRawAnimation { target_id }
            | Self::BehaviorModel { target_id }
            | Self::BehaviorBinding { target_id }
            | Self::BehaviorStatechart { target_id }
            | Self::BehaviorRawStateMachine { target_id } => target_id,
        }
    }
}

impl AuthoringContainer {
    fn kind(&self) -> EntityKind {
        match self {
            Self::VisualRoot | Self::VisualGroup { .. } => EntityKind::Visual,
            Self::Components => EntityKind::Component,
            Self::MotionEasings => EntityKind::MotionEasing,
            Self::MotionPoses => EntityKind::MotionPose,
            Self::MotionTracks => EntityKind::MotionTrack,
            Self::MotionRawAnimations => EntityKind::MotionRawAnimation,
            Self::BehaviorModels => EntityKind::BehaviorModel,
            Self::BehaviorBindings => EntityKind::BehaviorBinding,
            Self::BehaviorStatecharts => EntityKind::BehaviorStatechart,
            Self::BehaviorRawStateMachines => EntityKind::BehaviorRawStateMachine,
        }
    }
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
    let kind = entity.kind();
    match entity {
        AuthoringEntity::VisualNode(node) => insert_visual(spec, *node, placement),
        AuthoringEntity::Component(item) => {
            insert_list(&mut spec.components, item, placement, kind, "$.components")
        }
        AuthoringEntity::MotionEasing(item) => insert_list(
            &mut spec.motion.easings,
            item,
            placement,
            kind,
            "$.motion.easings",
        ),
        AuthoringEntity::MotionPose(item) => insert_list(
            &mut spec.motion.poses,
            item,
            placement,
            kind,
            "$.motion.poses",
        ),
        AuthoringEntity::MotionTrack(item) => insert_list(
            &mut spec.motion.tracks,
            item,
            placement,
            kind,
            "$.motion.tracks",
        ),
        AuthoringEntity::MotionRawAnimation(item) => insert_list(
            &mut spec.motion.raw_animations,
            item,
            placement,
            kind,
            "$.motion.raw_animations",
        ),
        AuthoringEntity::BehaviorModel(item) => insert_list(
            &mut spec.behavior.models,
            item,
            placement,
            kind,
            "$.behavior.models",
        ),
        AuthoringEntity::BehaviorBinding(item) => insert_list(
            &mut spec.behavior.bindings,
            item,
            placement,
            kind,
            "$.behavior.bindings",
        ),
        AuthoringEntity::BehaviorStatechart(item) => insert_list(
            &mut spec.behavior.statecharts,
            item,
            placement,
            kind,
            "$.behavior.statecharts",
        ),
        AuthoringEntity::BehaviorRawStateMachine(item) => insert_list(
            &mut spec.behavior.raw_state_machines,
            item,
            placement,
            kind,
            "$.behavior.raw_state_machines",
        ),
    }
}

fn insert_list<T: AuthoredId>(
    items: &mut Vec<T>,
    item: T,
    placement: &AuthoringPlacement,
    kind: EntityKind,
    path: &str,
) -> Result<(), AuthoringError> {
    match list_placement(placement, kind, path)? {
        ListPlacement::Append => items.push(item),
        ListPlacement::Before(target_id) => {
            let index = unique_index(items, target_id, path)?;
            items.insert(index, item);
        }
        ListPlacement::After(target_id) => {
            let index = unique_index(items, target_id, path)?;
            items.insert(index + 1, item);
        }
    }
    Ok(())
}

fn list_placement<'a>(
    placement: &'a AuthoringPlacement,
    kind: EntityKind,
    path: &str,
) -> Result<ListPlacement<'a>, AuthoringError> {
    match placement {
        AuthoringPlacement::Into { container } if container.kind() == kind => {
            Ok(ListPlacement::Append)
        }
        AuthoringPlacement::Before { anchor } if anchor.kind() == kind => {
            Ok(ListPlacement::Before(anchor.target_id()))
        }
        AuthoringPlacement::After { anchor } if anchor.kind() == kind => {
            Ok(ListPlacement::After(anchor.target_id()))
        }
        _ => Err(invalid_placement(path)),
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
        AuthoringPlacement::Before { anchor } if anchor.kind() == EntityKind::Visual => {
            insert_visual_relative(&mut spec.visual.nodes, anchor.target_id(), node, false)
        }
        AuthoringPlacement::After { anchor } if anchor.kind() == EntityKind::Visual => {
            insert_visual_relative(&mut spec.visual.nodes, anchor.target_id(), node, true)
        }
        _ => Err(invalid_placement("$.visual.nodes")),
    }
}

fn remove_entity(
    spec: &mut AuthoringSpec,
    target: &AuthoringTarget,
) -> Result<AuthoringEntity, AuthoringError> {
    match target {
        AuthoringTarget::VisualNode { target_id } => {
            remove_visual_node(&mut spec.visual.nodes, target_id)
                .map(Box::new)
                .map(AuthoringEntity::VisualNode)
        }
        AuthoringTarget::Component { target_id } => {
            remove_unique(&mut spec.components, target_id, "$.components")
                .map(AuthoringEntity::Component)
        }
        AuthoringTarget::MotionEasing { target_id } => {
            remove_unique(&mut spec.motion.easings, target_id, "$.motion.easings")
                .map(AuthoringEntity::MotionEasing)
        }
        AuthoringTarget::MotionPose { target_id } => {
            remove_unique(&mut spec.motion.poses, target_id, "$.motion.poses")
                .map(AuthoringEntity::MotionPose)
        }
        AuthoringTarget::MotionTrack { target_id } => {
            remove_unique(&mut spec.motion.tracks, target_id, "$.motion.tracks")
                .map(AuthoringEntity::MotionTrack)
        }
        AuthoringTarget::MotionRawAnimation { target_id } => remove_unique(
            &mut spec.motion.raw_animations,
            target_id,
            "$.motion.raw_animations",
        )
        .map(AuthoringEntity::MotionRawAnimation),
        AuthoringTarget::BehaviorModel { target_id } => {
            remove_unique(&mut spec.behavior.models, target_id, "$.behavior.models")
                .map(AuthoringEntity::BehaviorModel)
        }
        AuthoringTarget::BehaviorBinding { target_id } => remove_unique(
            &mut spec.behavior.bindings,
            target_id,
            "$.behavior.bindings",
        )
        .map(AuthoringEntity::BehaviorBinding),
        AuthoringTarget::BehaviorStatechart { target_id } => remove_unique(
            &mut spec.behavior.statecharts,
            target_id,
            "$.behavior.statecharts",
        )
        .map(AuthoringEntity::BehaviorStatechart),
        AuthoringTarget::BehaviorRawStateMachine { target_id } => remove_unique(
            &mut spec.behavior.raw_state_machines,
            target_id,
            "$.behavior.raw_state_machines",
        )
        .map(AuthoringEntity::BehaviorRawStateMachine),
    }
}

fn remove_unique<T: AuthoredId>(
    items: &mut Vec<T>,
    target_id: &str,
    path: &str,
) -> Result<T, AuthoringError> {
    let index = unique_index(items, target_id, path)?;
    Ok(items.remove(index))
}

fn unique_index<T: AuthoredId>(
    items: &[T],
    target_id: &str,
    path: &str,
) -> Result<usize, AuthoringError> {
    let matches = items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| (item.authored_id() == target_id).then_some(index))
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

fn replace_visual_node(
    nodes: &mut [VisualNode],
    target_id: &str,
    replacement: &VisualNode,
) -> Result<(), AuthoringError> {
    match count_visual_nodes(nodes, target_id, None) {
        0 => Err(visual_target_error(
            target_id,
            "does not identify a visual node",
        )),
        1 => {
            if replace_visual_node_once(nodes, target_id, replacement, None) {
                Ok(())
            } else {
                Err(visual_target_error(
                    target_id,
                    "does not identify a visual node",
                ))
            }
        }
        _ => Err(visual_ambiguous_error(target_id, "visual node")),
    }
}

fn append_visual_to_group(
    nodes: &mut [VisualNode],
    target_id: &str,
    node: VisualNode,
) -> Result<(), AuthoringError> {
    match count_visual_groups(nodes, target_id, None) {
        0 => Err(visual_target_error(target_id, "does not identify a group")),
        1 => {
            let mut node = Some(node);
            if append_visual_to_group_once(nodes, target_id, &mut node, None) {
                Ok(())
            } else {
                Err(visual_target_error(target_id, "does not identify a group"))
            }
        }
        _ => Err(visual_ambiguous_error(target_id, "group")),
    }
}

fn insert_visual_relative(
    nodes: &mut Vec<VisualNode>,
    target_id: &str,
    node: VisualNode,
    after: bool,
) -> Result<(), AuthoringError> {
    match count_visual_nodes(nodes, target_id, None) {
        0 => Err(visual_target_error(
            target_id,
            "does not identify a visual node",
        )),
        1 => {
            let mut node = Some(node);
            if insert_visual_relative_once(nodes, target_id, &mut node, after, None) {
                Ok(())
            } else {
                Err(visual_target_error(
                    target_id,
                    "does not identify a visual node",
                ))
            }
        }
        _ => Err(visual_ambiguous_error(target_id, "visual node")),
    }
}

fn remove_visual_node(
    nodes: &mut Vec<VisualNode>,
    target_id: &str,
) -> Result<VisualNode, AuthoringError> {
    match count_visual_nodes(nodes, target_id, None) {
        0 => Err(visual_target_error(
            target_id,
            "does not identify a visual node",
        )),
        1 => remove_visual_node_once(nodes, target_id, None)
            .ok_or_else(|| visual_target_error(target_id, "does not identify a visual node")),
        _ => Err(visual_ambiguous_error(target_id, "visual node")),
    }
}

fn invalid_placement(path: &str) -> AuthoringError {
    AuthoringError::one(AuthoringDiagnostic::new(
        path,
        "invalid_operation_placement",
        "placement and authored entity types must match",
    ))
}

fn visual_target_error(target_id: &str, detail: &str) -> AuthoringError {
    target_error("$.visual.nodes", "unknown_authored_id", target_id, detail)
}

fn visual_ambiguous_error(target_id: &str, entity: &str) -> AuthoringError {
    target_error(
        "$.visual.nodes",
        "ambiguous_authored_id",
        target_id,
        &format!("identifies more than one {entity}"),
    )
}

fn target_error(path: &str, code: &str, target_id: &str, detail: &str) -> AuthoringError {
    AuthoringError::one(AuthoringDiagnostic::new(
        path,
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
