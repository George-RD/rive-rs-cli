use std::collections::{BTreeMap, BTreeSet};

use super::expression::{evaluate_expression, evaluate_transform, validate_scene_number};
use super::spec::{
    AuthoringDiagnostic, ConstraintAxis, ConstraintSpec, Quantity, ScalarExpr, TransformSpec, Unit,
};
use super::visual::VisualNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Anchor {
    node: usize,
    axis: ConstraintAxis,
}

#[derive(Debug, Clone)]
struct Assignment {
    constraint_id: String,
    path: String,
    formula: Formula,
}

#[derive(Debug, Clone, Copy)]
enum Formula {
    Alias(Anchor),
    Midpoint(Anchor, Anchor),
    Offset(Anchor, f64),
}

pub(crate) fn resolve_group_constraints(
    children: &[VisualNode],
    constraints: &[ConstraintSpec],
    group_path: &str,
    scope: &BTreeMap<String, Quantity>,
) -> Result<Vec<VisualNode>, AuthoringDiagnostic> {
    if constraints.is_empty() {
        return Ok(children.to_vec());
    }

    validate_constraint_ids(constraints, group_path)?;

    let node_indices = children
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id(), index))
        .collect::<BTreeMap<_, _>>();
    let mut base_values = BTreeMap::new();
    for (index, node) in children.iter().enumerate() {
        let Some(transform) = node_transform(node) else {
            continue;
        };
        let values = evaluate_transform(
            transform,
            &format!("{group_path}.children[{index}].transform"),
            scope,
        )?;
        base_values.insert(
            Anchor {
                node: index,
                axis: ConstraintAxis::X,
            },
            values.x,
        );
        base_values.insert(
            Anchor {
                node: index,
                axis: ConstraintAxis::Y,
            },
            values.y,
        );
    }

    let mut assignments = BTreeMap::new();
    for (index, constraint) in constraints.iter().enumerate() {
        let path = format!("{group_path}.constraints[{index}]");
        match constraint {
            ConstraintSpec::Align {
                id,
                subject,
                target,
                axis,
            } => {
                let subject = resolve_reference(
                    subject,
                    *axis,
                    &format!("{path}.subject"),
                    children,
                    &node_indices,
                )?;
                let target = resolve_reference(
                    target,
                    *axis,
                    &format!("{path}.target"),
                    children,
                    &node_indices,
                )?;
                insert_assignment(
                    &mut assignments,
                    subject,
                    Assignment {
                        constraint_id: id.clone(),
                        path: format!("{path}.subject"),
                        formula: Formula::Alias(target),
                    },
                    children,
                )?;
            }
            ConstraintSpec::Center {
                id,
                subject,
                start,
                end,
                axis,
            } => {
                let subject = resolve_reference(
                    subject,
                    *axis,
                    &format!("{path}.subject"),
                    children,
                    &node_indices,
                )?;
                let start = resolve_reference(
                    start,
                    *axis,
                    &format!("{path}.start"),
                    children,
                    &node_indices,
                )?;
                let end = resolve_reference(
                    end,
                    *axis,
                    &format!("{path}.end"),
                    children,
                    &node_indices,
                )?;
                insert_assignment(
                    &mut assignments,
                    subject,
                    Assignment {
                        constraint_id: id.clone(),
                        path: format!("{path}.subject"),
                        formula: Formula::Midpoint(start, end),
                    },
                    children,
                )?;
            }
            ConstraintSpec::Offset {
                id,
                subject,
                target,
                x,
                y,
            } => {
                let x_offset = evaluate_expression(x, &format!("{path}.x"), scope, Unit::Px)?;
                let y_offset = evaluate_expression(y, &format!("{path}.y"), scope, Unit::Px)?;
                for (axis, offset) in [
                    (ConstraintAxis::X, x_offset),
                    (ConstraintAxis::Y, y_offset),
                ] {
                    let subject = resolve_reference(
                        subject,
                        axis,
                        &format!("{path}.subject"),
                        children,
                        &node_indices,
                    )?;
                    let target = resolve_reference(
                        target,
                        axis,
                        &format!("{path}.target"),
                        children,
                        &node_indices,
                    )?;
                    insert_assignment(
                        &mut assignments,
                        subject,
                        Assignment {
                            constraint_id: id.clone(),
                            path: format!("{path}.subject"),
                            formula: Formula::Offset(target, offset),
                        },
                        children,
                    )?;
                }
            }
            ConstraintSpec::Spacing {
                id,
                items,
                axis,
                gap,
            } => {
                if !(2..=100).contains(&items.len()) {
                    return Err(AuthoringDiagnostic::new(
                        format!("{path}.items"),
                        "invalid_constraint_items",
                        "spacing constraints require between 2 and 100 ordered sibling ids",
                    ));
                }
                let mut seen = BTreeSet::new();
                let mut anchors = Vec::with_capacity(items.len());
                for (item_index, item) in items.iter().enumerate() {
                    if !seen.insert(item.as_str()) {
                        return Err(AuthoringDiagnostic::new(
                            format!("{path}.items[{item_index}]"),
                            "duplicate_constraint_node",
                            format!("spacing constraint '{id}' names sibling '{item}' more than once"),
                        ));
                    }
                    anchors.push(resolve_reference(
                        item,
                        *axis,
                        &format!("{path}.items[{item_index}]"),
                        children,
                        &node_indices,
                    )?);
                }
                let gap = evaluate_expression(gap, &format!("{path}.gap"), scope, Unit::Px)?;
                for item_index in 1..anchors.len() {
                    insert_assignment(
                        &mut assignments,
                        anchors[item_index],
                        Assignment {
                            constraint_id: id.clone(),
                            path: format!("{path}.items[{item_index}]"),
                            formula: Formula::Offset(anchors[item_index - 1], gap),
                        },
                        children,
                    )?;
                }
            }
        }
    }

    let mut memo = BTreeMap::new();
    let mut stack = Vec::new();
    for anchor in assignments.keys().copied().collect::<Vec<_>>() {
        resolve_anchor(
            anchor,
            &assignments,
            &base_values,
            &mut memo,
            &mut stack,
            children,
        )?;
    }

    let mut resolved = children.to_vec();
    for (anchor, value) in memo {
        if !assignments.contains_key(&anchor) {
            continue;
        }
        let transform = node_transform_mut(&mut resolved[anchor.node]).ok_or_else(|| {
            AuthoringDiagnostic::new(
                group_path,
                "unsupported_constraint_node",
                "raw scene objects cannot receive typed authoring constraints",
            )
        })?;
        let value = Some(ScalarExpr::Literal {
            value,
            unit: Unit::Px,
        });
        match anchor.axis {
            ConstraintAxis::X => transform.x = value,
            ConstraintAxis::Y => transform.y = value,
        }
    }

    Ok(resolved)
}

fn validate_constraint_ids(
    constraints: &[ConstraintSpec],
    group_path: &str,
) -> Result<(), AuthoringDiagnostic> {
    let mut ids = BTreeSet::new();
    for (index, constraint) in constraints.iter().enumerate() {
        let id = constraint_id(constraint);
        let path = format!("{group_path}.constraints[{index}].id");
        if id.trim().is_empty() || id.contains('/') {
            return Err(AuthoringDiagnostic::new(
                path,
                "invalid_constraint_id",
                "constraint ids must not be empty or contain the reserved '/' separator",
            ));
        }
        if !ids.insert(id) {
            return Err(AuthoringDiagnostic::new(
                path,
                "duplicate_constraint_id",
                format!("constraint id '{id}' is declared more than once in this group"),
            ));
        }
    }
    Ok(())
}

fn constraint_id(constraint: &ConstraintSpec) -> &str {
    match constraint {
        ConstraintSpec::Align { id, .. }
        | ConstraintSpec::Center { id, .. }
        | ConstraintSpec::Offset { id, .. }
        | ConstraintSpec::Spacing { id, .. } => id,
    }
}

fn resolve_reference(
    id: &str,
    axis: ConstraintAxis,
    path: &str,
    children: &[VisualNode],
    node_indices: &BTreeMap<&str, usize>,
) -> Result<Anchor, AuthoringDiagnostic> {
    let Some(index) = node_indices.get(id).copied() else {
        return Err(AuthoringDiagnostic::new(
            path,
            "unknown_constraint_node",
            format!("constraint references unknown direct sibling '{id}'"),
        ));
    };
    if node_transform(&children[index]).is_none() {
        return Err(AuthoringDiagnostic::new(
            path,
            "unsupported_constraint_node",
            format!("direct sibling '{id}' is a raw scene object without a typed transform"),
        ));
    }
    Ok(Anchor { node: index, axis })
}

fn insert_assignment(
    assignments: &mut BTreeMap<Anchor, Assignment>,
    anchor: Anchor,
    assignment: Assignment,
    children: &[VisualNode],
) -> Result<(), AuthoringDiagnostic> {
    if let Some(existing) = assignments.get(&anchor) {
        return Err(AuthoringDiagnostic::new(
            assignment.path,
            "constraint_conflict",
            format!(
                "constraints '{}' and '{}' both assign {}",
                existing.constraint_id,
                assignment.constraint_id,
                anchor_label(anchor, children)
            ),
        ));
    }
    assignments.insert(anchor, assignment);
    Ok(())
}

fn resolve_anchor(
    anchor: Anchor,
    assignments: &BTreeMap<Anchor, Assignment>,
    base_values: &BTreeMap<Anchor, f64>,
    memo: &mut BTreeMap<Anchor, f64>,
    stack: &mut Vec<Anchor>,
    children: &[VisualNode],
) -> Result<f64, AuthoringDiagnostic> {
    if let Some(value) = memo.get(&anchor) {
        return Ok(*value);
    }
    if let Some(cycle_start) = stack.iter().position(|entry| *entry == anchor) {
        let mut cycle = stack[cycle_start..].to_vec();
        cycle.push(anchor);
        let path = assignments
            .get(&anchor)
            .map(|assignment| assignment.path.as_str())
            .unwrap_or("$");
        return Err(AuthoringDiagnostic::new(
            path,
            "constraint_cycle",
            format!(
                "constraint cycle detected: {}",
                cycle
                    .into_iter()
                    .map(|entry| anchor_label(entry, children))
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ),
        ));
    }

    let Some(assignment) = assignments.get(&anchor) else {
        return base_values.get(&anchor).copied().ok_or_else(|| {
            AuthoringDiagnostic::new(
                "$",
                "unsupported_constraint_node",
                format!("{} has no typed transform anchor", anchor_label(anchor, children)),
            )
        });
    };

    stack.push(anchor);
    let result = match assignment.formula {
        Formula::Alias(source) => resolve_anchor(
            source,
            assignments,
            base_values,
            memo,
            stack,
            children,
        ),
        Formula::Midpoint(start, end) => {
            let start = resolve_anchor(
                start,
                assignments,
                base_values,
                memo,
                stack,
                children,
            )?;
            let end = resolve_anchor(
                end,
                assignments,
                base_values,
                memo,
                stack,
                children,
            )?;
            Ok((start + end) / 2.0)
        }
        Formula::Offset(source, amount) => resolve_anchor(
            source,
            assignments,
            base_values,
            memo,
            stack,
            children,
        )
        .map(|value| value + amount),
    };
    stack.pop();
    let value = result?;
    validate_scene_number(value, &assignment.path)?;
    memo.insert(anchor, value);
    Ok(value)
}

fn anchor_label(anchor: Anchor, children: &[VisualNode]) -> String {
    format!(
        "{}.{}",
        children[anchor.node].id(),
        match anchor.axis {
            ConstraintAxis::X => "x",
            ConstraintAxis::Y => "y",
        }
    )
}

fn node_transform(node: &VisualNode) -> Option<&TransformSpec> {
    match node {
        VisualNode::Ellipse { transform, .. }
        | VisualNode::Rectangle { transform, .. }
        | VisualNode::Triangle { transform, .. }
        | VisualNode::Polygon { transform, .. }
        | VisualNode::Star { transform, .. }
        | VisualNode::Text { transform, .. }
        | VisualNode::Image { transform, .. }
        | VisualNode::Grid { transform, .. }
        | VisualNode::Radial { transform, .. }
        | VisualNode::Mirror { transform, .. }
        | VisualNode::Distribute { transform, .. }
        | VisualNode::AlongPath { transform, .. }
        | VisualNode::Group { transform, .. }
        | VisualNode::Instance { transform, .. } => Some(transform),
        VisualNode::RawSceneObject { .. } => None,
    }
}

fn node_transform_mut(node: &mut VisualNode) -> Option<&mut TransformSpec> {
    match node {
        VisualNode::Ellipse { transform, .. }
        | VisualNode::Rectangle { transform, .. }
        | VisualNode::Triangle { transform, .. }
        | VisualNode::Polygon { transform, .. }
        | VisualNode::Star { transform, .. }
        | VisualNode::Text { transform, .. }
        | VisualNode::Image { transform, .. }
        | VisualNode::Grid { transform, .. }
        | VisualNode::Radial { transform, .. }
        | VisualNode::Mirror { transform, .. }
        | VisualNode::Distribute { transform, .. }
        | VisualNode::AlongPath { transform, .. }
        | VisualNode::Group { transform, .. }
        | VisualNode::Instance { transform, .. } => Some(transform),
        VisualNode::RawSceneObject { .. } => None,
    }
}
