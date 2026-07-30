use std::collections::BTreeMap;

use super::expression::validate_scene_number;
use super::spec::{
    AuthoringDiagnostic, AuthoringSpec, PaintSpec, Quantity, ScalarExpr, TransformSpec,
};
use super::visual::VisualNode;

pub(super) fn validate_numeric_values(spec: &AuthoringSpec) -> Vec<AuthoringDiagnostic> {
    let mut diagnostics = Vec::new();
    validate_quantity(spec.artboard.width, "$.artboard.width", &mut diagnostics);
    validate_quantity(spec.artboard.height, "$.artboard.height", &mut diagnostics);
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

fn validate_quantity(quantity: Quantity, path: &str, diagnostics: &mut Vec<AuthoringDiagnostic>) {
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

fn validate_node(node: &VisualNode, path: &str, diagnostics: &mut Vec<AuthoringDiagnostic>) {
    if let Some(shape) = node.shape() {
        validate_expression(shape.width, &format!("{path}.width"), diagnostics);
        validate_expression(shape.height, &format!("{path}.height"), diagnostics);
        if let Some(corner_radius) = shape.corner_radius {
            validate_expression(corner_radius, &format!("{path}.corner_radius"), diagnostics);
        }
        if let Some(inner_radius) = shape.inner_radius {
            validate_expression(inner_radius, &format!("{path}.inner_radius"), diagnostics);
        }
        validate_paint(shape.fill, &format!("{path}.fill"), diagnostics);
        if let Some(stroke) = shape.stroke {
            validate_paint(&stroke.paint, &format!("{path}.stroke.paint"), diagnostics);
            validate_expression(&stroke.width, &format!("{path}.stroke.width"), diagnostics);
            if let Some(trim) = &stroke.trim {
                validate_expression(
                    &trim.start,
                    &format!("{path}.stroke.trim.start"),
                    diagnostics,
                );
                validate_expression(&trim.end, &format!("{path}.stroke.trim.end"), diagnostics);
                if let Some(offset) = &trim.offset {
                    validate_expression(offset, &format!("{path}.stroke.trim.offset"), diagnostics);
                }
            }
        }
        validate_transform(shape.transform, &format!("{path}.transform"), diagnostics);
        return;
    }

    if let Some(text) = node.text_node() {
        validate_expression(text.font_size, &format!("{path}.font_size"), diagnostics);
        validate_paint(text.fill, &format!("{path}.fill"), diagnostics);
        for (name, expression) in [
            ("width", text.width),
            ("height", text.height),
            ("line_height", text.line_height),
            ("letter_spacing", text.letter_spacing),
            ("paragraph_spacing", text.paragraph_spacing),
            ("origin_x", text.origin_x),
            ("origin_y", text.origin_y),
        ] {
            if let Some(expression) = expression {
                validate_expression(expression, &format!("{path}.{name}"), diagnostics);
            }
        }
        validate_transform(text.transform, &format!("{path}.transform"), diagnostics);
        return;
    }

    match node {
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
        VisualNode::Ellipse { .. }
        | VisualNode::Rectangle { .. }
        | VisualNode::Triangle { .. }
        | VisualNode::Polygon { .. }
        | VisualNode::Star { .. }
        | VisualNode::Text { .. } => unreachable!("shape and text nodes are handled above"),
    }
}

fn validate_paint(paint: &PaintSpec, path: &str, diagnostics: &mut Vec<AuthoringDiagnostic>) {
    let PaintSpec::Gradient(gradient) = paint else {
        return;
    };

    for (name, expression) in [
        ("start_x", &gradient.start_x),
        ("start_y", &gradient.start_y),
        ("end_x", &gradient.end_x),
        ("end_y", &gradient.end_y),
    ] {
        validate_expression(expression, &format!("{path}.{name}"), diagnostics);
    }

    if gradient.stops.len() < 2 {
        diagnostics.push(AuthoringDiagnostic::new(
            format!("{path}.stops"),
            "invalid_gradient_stops",
            "gradient paints require at least two stops",
        ));
    }
    for (index, stop) in gradient.stops.iter().enumerate() {
        validate_expression(
            &stop.position,
            &format!("{path}.stops[{index}].position"),
            diagnostics,
        );
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
