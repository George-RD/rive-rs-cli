use std::collections::HashMap;

use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec, ComponentSpec, PaintSpec};
use super::visual::{PatternNodeRef, VisualNode};

const MAX_COMPONENT_EXPANSION_DEPTH: usize = 64;
const MAX_GENERATED_COMPONENT_NODES: u64 = 10_000;
const MAX_PATTERN_AXIS_COUNT: u64 = 100;
const MAX_GENERATED_PATTERN_NODES: u64 = 10_000;

#[derive(Clone, Copy)]
struct ComponentRef<'a> {
    index: usize,
    spec: &'a ComponentSpec,
}

#[derive(Default)]
struct ExpansionBudget {
    generated_component_nodes: u64,
    generated_pattern_nodes: u64,
}

#[derive(Clone)]
struct ExpansionContext {
    active_components: Vec<usize>,
    multiplicity: u64,
    generated_by_component: bool,
    component_budget_path: Option<String>,
    generated_by_pattern: bool,
    pattern_budget_path: Option<String>,
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
        generated_by_pattern: false,
        pattern_budget_path: None,
    };
    push_nodes(&mut work, nodes, list_path, &root_expansion);

    while let Some(item) = work.pop() {
        let WorkItem {
            node,
            path,
            expansion,
        } = item;

        validate_repeatable_pattern_node(node, &path, &expansion)?;
        let generated_nodes = expansion
            .multiplicity
            .saturating_mul(generated_node_weight(node));
        charge_generated_nodes(budget, &expansion, &path, generated_nodes)?;

        if let Some(children) = node.children() {
            push_nodes(&mut work, children, &format!("{path}.children"), &expansion);
            continue;
        }

        if let Some(pattern) = node.pattern() {
            let (item_count, limit_path) = validate_pattern(pattern, &path)?;
            let generated_nodes = expansion.multiplicity.saturating_mul(item_count);
            work.push(WorkItem {
                node: pattern.item(),
                path: format!("{path}.item"),
                expansion: ExpansionContext {
                    multiplicity: generated_nodes,
                    generated_by_pattern: true,
                    pattern_budget_path: Some(limit_path),
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

fn validate_repeatable_pattern_node(
    node: &VisualNode,
    path: &str,
    expansion: &ExpansionContext,
) -> Result<(), AuthoringError> {
    if expansion.generated_by_pattern
        && expansion.multiplicity > 1
        && matches!(node, VisualNode::RawSceneObject { .. })
    {
        return Err(AuthoringError::one(AuthoringDiagnostic::new(
            path,
            "unsupported_repeated_raw_scene_object",
            "raw SceneSpec objects cannot be repeated because embedded names and references cannot be safely namespaced",
        )));
    }
    Ok(())
}

fn generated_node_weight(node: &VisualNode) -> u64 {
    if let Some(shape) = node.shape() {
        return 1_u64
            .saturating_add(paint_child_count(shape.fill))
            .saturating_add(
                shape
                    .stroke
                    .map_or(0, |stroke| paint_child_count(&stroke.paint)),
            );
    }

    1_u64.saturating_add(
        node.text_node()
            .map_or(0, |text| paint_child_count(text.fill)),
    )
}

fn paint_child_count(paint: &PaintSpec) -> u64 {
    match paint {
        PaintSpec::Solid(_) => 0,
        PaintSpec::Gradient(gradient) => u64::try_from(gradient.stops.len()).unwrap_or(u64::MAX),
    }
}

fn charge_generated_nodes(
    budget: &mut ExpansionBudget,
    expansion: &ExpansionContext,
    path: &str,
    generated: u64,
) -> Result<(), AuthoringError> {
    if expansion.generated_by_component {
        charge_expansion_budget(
            &mut budget.generated_component_nodes,
            generated,
            MAX_GENERATED_COMPONENT_NODES,
            expansion.component_budget_path.as_deref().unwrap_or(path),
            "component_expansion_node_limit",
            "component expansion exceeds the maximum generated-node budget",
        )?;
    }

    if expansion.generated_by_pattern {
        charge_expansion_budget(
            &mut budget.generated_pattern_nodes,
            generated,
            MAX_GENERATED_PATTERN_NODES,
            expansion.pattern_budget_path.as_deref().unwrap_or(path),
            "pattern_expansion_node_limit",
            "pattern expansion exceeds the maximum generated-node budget",
        )?;
    }

    Ok(())
}

fn charge_expansion_budget(
    used: &mut u64,
    generated: u64,
    maximum: u64,
    path: &str,
    code: &str,
    message: &str,
) -> Result<(), AuthoringError> {
    *used = used.saturating_add(generated);
    if *used <= maximum {
        return Ok(());
    }
    Err(AuthoringError::one(AuthoringDiagnostic::new(
        path,
        code,
        format!("{message} of {maximum}"),
    )))
}

fn validate_pattern(
    pattern: PatternNodeRef<'_>,
    path: &str,
) -> Result<(u64, String), AuthoringError> {
    match pattern {
        PatternNodeRef::Grid(grid) => {
            validate_pattern_count(grid.rows, 1, &format!("{path}.rows"))?;
            validate_pattern_count(grid.columns, 1, &format!("{path}.columns"))?;
            Ok((
                grid.rows.saturating_mul(grid.columns),
                format!("{path}.rows"),
            ))
        }
        PatternNodeRef::Radial(radial) => {
            validate_pattern_count(radial.copies, 1, &format!("{path}.copies"))?;
            Ok((radial.copies, format!("{path}.copies")))
        }
        PatternNodeRef::Mirror(_) => Ok((2, format!("{path}.item"))),
        PatternNodeRef::Distribute(distribute) => {
            validate_pattern_count(distribute.copies, 2, &format!("{path}.copies"))?;
            Ok((distribute.copies, format!("{path}.copies")))
        }
        PatternNodeRef::AlongPath(along_path) => {
            validate_pattern_count(along_path.copies, 2, &format!("{path}.copies"))?;
            validate_path_point_count(along_path.points.len(), &format!("{path}.points"))?;
            Ok((along_path.copies, format!("{path}.copies")))
        }
    }
}

fn validate_pattern_count(value: u64, minimum: u64, path: &str) -> Result<(), AuthoringError> {
    validate_bounded_count(
        value,
        minimum,
        MAX_PATTERN_AXIS_COUNT,
        path,
        "invalid_pattern_count",
        "pattern counts",
    )
}

fn validate_path_point_count(value: usize, path: &str) -> Result<(), AuthoringError> {
    validate_bounded_count(
        u64::try_from(value).unwrap_or(u64::MAX),
        2,
        MAX_PATTERN_AXIS_COUNT,
        path,
        "invalid_path_point_count",
        "path point counts",
    )
}

fn validate_bounded_count(
    value: u64,
    minimum: u64,
    maximum: u64,
    path: &str,
    code: &str,
    label: &str,
) -> Result<(), AuthoringError> {
    if (minimum..=maximum).contains(&value) {
        return Ok(());
    }
    Err(AuthoringError::one(AuthoringDiagnostic::new(
        path,
        code,
        format!("{label} must be between {minimum} and {maximum}"),
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::authoring::lower_authoring_json;

    #[test]
    fn repeated_gradient_stops_share_the_pattern_node_budget() {
        let stops = (0..=100)
            .map(|index| {
                json!({
                    "color": "#2563EB",
                    "position": {
                        "kind": "literal",
                        "value": index as f64 / 100.0,
                        "unit": "scalar"
                    }
                })
            })
            .collect::<Vec<_>>();
        let input = json!({
            "authoring_format_version": 0,
            "artboard": {
                "id": "stage",
                "width": { "value": 320.0, "unit": "px" },
                "height": { "value": 220.0, "unit": "px" }
            },
            "visual": {
                "nodes": [
                    {
                        "kind": "radial",
                        "id": "gradient-orbit",
                        "copies": 100,
                        "radius": { "kind": "literal", "value": 20.0, "unit": "px" },
                        "start_angle": { "kind": "literal", "value": 0.0, "unit": "degrees" },
                        "angle_step": { "kind": "literal", "value": 3.6, "unit": "degrees" },
                        "item": {
                            "kind": "rectangle",
                            "id": "gradient-tile",
                            "width": { "kind": "literal", "value": 16.0, "unit": "px" },
                            "height": { "kind": "literal", "value": 12.0, "unit": "px" },
                            "fill": {
                                "kind": "linear_gradient",
                                "start_x": { "kind": "literal", "value": 0.0, "unit": "px" },
                                "start_y": { "kind": "literal", "value": 0.0, "unit": "px" },
                                "end_x": { "kind": "literal", "value": 16.0, "unit": "px" },
                                "end_y": { "kind": "literal", "value": 12.0, "unit": "px" },
                                "stops": stops
                            }
                        }
                    }
                ]
            },
            "motion": {},
            "behavior": {}
        })
        .to_string();

        let error = lower_authoring_json(&input)
            .expect_err("repeated gradient stops must share the pattern node budget");
        assert!(error.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "pattern_expansion_node_limit"
                && diagnostic.path == "$.visual.nodes[0].copies"
        }));
    }
}
