use std::collections::BTreeMap;

use super::expression::validate_scene_number;
use super::lower;
use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringArtboard, AuthoringDiagnostic, AuthoringError,
    AuthoringSpec, BehaviorSection, LoweredAuthoring, MotionSection, Quantity, ScalarExpr,
    TransformSpec, Unit, VisualNode, VisualSection,
};

pub fn lower_authoring_json(input: &str) -> Result<LoweredAuthoring, AuthoringError> {
    let spec = serde_json::from_str::<AuthoringSpec>(input).map_err(|error| {
        AuthoringError::one(AuthoringDiagnostic::new(
            "$",
            "invalid_json",
            format!(
                "{error} at line {}, column {}",
                error.line(),
                error.column()
            ),
        ))
    })?;
    lower_authoring(&spec)
}

pub fn lower_authoring(spec: &AuthoringSpec) -> Result<LoweredAuthoring, AuthoringError> {
    let numeric_diagnostics = validate_numeric_values(spec);
    if !numeric_diagnostics.is_empty() {
        return Err(AuthoringError::many(numeric_diagnostics));
    }

    validate_component_definitions(spec)?;
    lower::lower_authoring(spec).map_err(|error| rewrite_error_paths(spec, error))
}

fn validate_component_definitions(spec: &AuthoringSpec) -> Result<(), AuthoringError> {
    if spec.authoring_format_version != AUTHORING_FORMAT_VERSION {
        return Ok(());
    }

    for (component_index, component) in spec.components.iter().enumerate() {
        let validation_spec = AuthoringSpec {
            authoring_format_version: spec.authoring_format_version,
            artboard: AuthoringArtboard {
                id: format!("__authoring_component_validation_{component_index}"),
                width: Quantity {
                    value: 1.0,
                    unit: Unit::Px,
                },
                height: Quantity {
                    value: 1.0,
                    unit: Unit::Px,
                },
            },
            parameters: spec.parameters.clone(),
            components: spec.components.clone(),
            visual: VisualSection {
                nodes: vec![VisualNode::Instance {
                    id: "__component_root".to_string(),
                    component: component.id.clone(),
                    overrides: BTreeMap::new(),
                    transform: TransformSpec::default(),
                }],
            },
            motion: MotionSection::default(),
            behavior: BehaviorSection::default(),
        };

        if let Err(error) = lower::lower_authoring(&validation_spec) {
            let mut error = rewrite_error_paths(&validation_spec, error);
            for diagnostic in &mut error.diagnostics {
                if diagnostic.path == "$.lowered_scene" {
                    diagnostic.path = format!("$.components[{component_index}].visual");
                    diagnostic.code = "invalid_component_scene".to_string();
                    diagnostic.message = format!(
                        "component '{}' does not lower to a canonical SceneSpec graph: {}",
                        component.id, diagnostic.message
                    );
                }
            }
            return Err(error);
        }
    }

    Ok(())
}

fn validate_numeric_values(spec: &AuthoringSpec) -> Vec<AuthoringDiagnostic> {
    let mut diagnostics = Vec::new();
    validate_quantity(
        spec.artboard.width,
        "$.artboard.width",
        &mut diagnostics,
    );
    validate_quantity(
        spec.artboard.height,
        "$.artboard.height",
        &mut diagnostics,
    );
    validate_quantity_map(&spec.parameters, "$.parameters", &mut diagnostics);

    for (component_index, component) in spec.components.iter().enumerate() {
        let component_path = format!("$.components[{component_index}]");
        validate_quantity_map(
            &component.parameters,
            &format!("{component_path}.parameters"),
            &mut diagnostics,
        );
        validate_nodes(
            &component.visual,
            &format!("{component_path}.visual"),
            &mut diagnostics,
        );
    }
    validate_nodes(&spec.visual.nodes, "$.visual.nodes", &mut diagnostics);
    diagnostics
}

fn validate_quantity_map(
    quantities: &BTreeMap<String, Quantity>,
    path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (name, quantity) in quantities {
        validate_quantity(*quantity, &format!("{path}.{name}"), diagnostics);
    }
}

fn validate_quantity(
    quantity: Quantity,
    path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    if let Err(diagnostic) = validate_scene_number(quantity.value, &format!("{path}.value")) {
        diagnostics.push(diagnostic);
    }
}

fn validate_nodes(
    nodes: &[VisualNode],
    list_path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (index, node) in nodes.iter().enumerate() {
        validate_node(node, &format!("{list_path}[{index}]"), diagnostics);
    }
}

fn validate_node(
    node: &VisualNode,
    path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    match node {
        VisualNode::Ellipse {
            width,
            height,
            transform,
            ..
        } => {
            validate_expression(width, &format!("{path}.width"), diagnostics);
            validate_expression(height, &format!("{path}.height"), diagnostics);
            validate_transform(transform, &format!("{path}.transform"), diagnostics);
        }
        VisualNode::Rectangle {
            width,
            height,
            corner_radius,
            transform,
            ..
        } => {
            validate_expression(width, &format!("{path}.width"), diagnostics);
            validate_expression(height, &format!("{path}.height"), diagnostics);
            if let Some(corner_radius) = corner_radius {
                validate_expression(
                    corner_radius,
                    &format!("{path}.corner_radius"),
                    diagnostics,
                );
            }
            validate_transform(transform, &format!("{path}.transform"), diagnostics);
        }
        VisualNode::Group {
            transform,
            children,
            ..
        } => {
            validate_transform(transform, &format!("{path}.transform"), diagnostics);
            validate_nodes(children, &format!("{path}.children"), diagnostics);
        }
        VisualNode::Instance {
            overrides,
            transform,
            ..
        } => {
            validate_quantity_map(overrides, &format!("{path}.overrides"), diagnostics);
            validate_transform(transform, &format!("{path}.transform"), diagnostics);
        }
        VisualNode::RawSceneObject { .. } => {}
    }
}

fn validate_transform(
    transform: &TransformSpec,
    path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (name, expression) in [
        ("x", transform.x.as_ref()),
        ("y", transform.y.as_ref()),
        ("rotation", transform.rotation.as_ref()),
        ("scale_x", transform.scale_x.as_ref()),
        ("scale_y", transform.scale_y.as_ref()),
    ] {
        if let Some(expression) = expression {
            validate_expression(expression, &format!("{path}.{name}"), diagnostics);
        }
    }
}

fn validate_expression(
    expression: &ScalarExpr,
    path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    match expression {
        ScalarExpr::Literal { value, .. } => {
            if let Err(diagnostic) = validate_scene_number(*value, &format!("{path}.value")) {
                diagnostics.push(diagnostic);
            }
        }
        ScalarExpr::Parameter { .. } => {}
        ScalarExpr::Add { left, right } | ScalarExpr::Subtract { left, right } => {
            validate_expression(left, &format!("{path}.left"), diagnostics);
            validate_expression(right, &format!("{path}.right"), diagnostics);
        }
        ScalarExpr::Multiply { value, factor } => {
            validate_expression(value, &format!("{path}.value"), diagnostics);
            if let Err(diagnostic) = validate_scene_number(*factor, &format!("{path}.factor")) {
                diagnostics.push(diagnostic);
            }
        }
        ScalarExpr::Divide { value, divisor } => {
            validate_expression(value, &format!("{path}.value"), diagnostics);
            if let Err(diagnostic) = validate_scene_number(*divisor, &format!("{path}.divisor")) {
                diagnostics.push(diagnostic);
            }
        }
    }
}

fn rewrite_error_paths(spec: &AuthoringSpec, mut error: AuthoringError) -> AuthoringError {
    for diagnostic in &mut error.diagnostics {
        if let Some(path) = resolve_expanded_path(spec, &diagnostic.path) {
            diagnostic.path = path;
        }
    }
    error
}

fn resolve_expanded_path(spec: &AuthoringSpec, path: &str) -> Option<String> {
    let (root_index, mut remainder) = take_index(path, "$.visual.nodes[")?;
    let mut node = spec.visual.nodes.get(root_index)?;
    let mut resolved = format!("$.visual.nodes[{root_index}]");

    loop {
        if let Some((expanded_index, rest)) = take_index(remainder, ".expanded[") {
            let VisualNode::Instance { component, .. } = node else {
                return None;
            };
            let component_index = spec
                .components
                .iter()
                .position(|candidate| candidate.id == *component)?;
            node = spec
                .components
                .get(component_index)?
                .visual
                .get(expanded_index)?;
            resolved = format!("$.components[{component_index}].visual[{expanded_index}]");
            remainder = rest;
            continue;
        }

        if let Some((child_index, rest)) = take_index(remainder, ".children[") {
            let VisualNode::Group { children, .. } = node else {
                return None;
            };
            node = children.get(child_index)?;
            resolved.push_str(&format!(".children[{child_index}]"));
            remainder = rest;
            continue;
        }

        resolved.push_str(remainder);
        return Some(resolved);
    }
}

fn take_index<'a>(input: &'a str, prefix: &str) -> Option<(usize, &'a str)> {
    let input = input.strip_prefix(prefix)?;
    let close = input.find(']')?;
    let index = input[..close].parse().ok()?;
    Some((index, &input[close + 1..]))
}
