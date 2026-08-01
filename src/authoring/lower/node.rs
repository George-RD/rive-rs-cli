use serde_json::{Value, json};

use super::super::constraint::resolve_group_constraints;
use super::super::expression::evaluate_transform;
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry};
use super::super::visual::VisualNode;
use super::{Lowerer, NodeContext, collect_named_paths, runtime_name, validate_sibling_ids_result};

impl<'a> Lowerer<'a> {
    pub(super) fn lower_node(
        &mut self,
        node: &VisualNode,
        context: NodeContext<'_>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        if let Some(pattern) = node.pattern() {
            return self.lower_pattern(pattern, context, component_stack);
        }
        if let Some(shape) = node.shape() {
            return self.lower_shape(shape, context);
        }
        if let Some(text) = node.text_node() {
            return self.lower_text(text, context);
        }
        if let Some(image) = node.image_node() {
            return self.lower_image(image, context);
        }

        let NodeContext {
            authored_path,
            definition_path,
            authored_id,
            runtime_segments,
            scene_path,
            scope,
        } = context;

        match node {
            VisualNode::Group {
                transform,
                constraints,
                children,
                ..
            } => {
                validate_sibling_ids_result(children, &format!("{authored_path}.children"))?;
                let children =
                    resolve_group_constraints(children, constraints, &authored_path, scope)?;
                let transform_values =
                    evaluate_transform(transform, &format!("{authored_path}.transform"), scope)?;
                let wrapper_name = runtime_name(&runtime_segments, "group");
                self.register_runtime_names(
                    std::slice::from_ref(&wrapper_name),
                    &format!("{authored_path}.id"),
                )?;
                self.source_map.entries.push(SourceMapEntry {
                    authored_id: authored_id.clone(),
                    authored_path: authored_path.clone(),
                    definition_path: definition_path.clone(),
                    runtime_names: vec![wrapper_name.clone()],
                    scene_paths: vec![scene_path.clone()],
                });

                let mut lowered_children = Vec::with_capacity(children.len());
                for (index, child) in children.iter().enumerate() {
                    let child_authored_path = format!("{authored_path}.children[{index}]");
                    let child_definition_path = definition_path
                        .as_ref()
                        .map(|path| format!("{path}.children[{index}]"));
                    let child_authored_id = format!("{authored_id}/{}", child.id());
                    let mut child_runtime_segments = runtime_segments.clone();
                    child_runtime_segments.push(child.id().to_string());
                    let child_scene_path = format!("{scene_path}/children/{index}");
                    lowered_children.push(self.lower_node(
                        child,
                        NodeContext {
                            authored_path: child_authored_path,
                            definition_path: child_definition_path,
                            authored_id: child_authored_id,
                            runtime_segments: child_runtime_segments,
                            scene_path: child_scene_path,
                            scope,
                        },
                        component_stack,
                    )?);
                }

                Ok(json!({
                    "type": "node",
                    "name": wrapper_name,
                    "x": transform_values.x,
                    "y": transform_values.y,
                    "rotation": transform_values.rotation,
                    "scale_x": transform_values.scale_x,
                    "scale_y": transform_values.scale_y,
                    "children": lowered_children
                }))
            }
            VisualNode::Instance {
                component,
                overrides,
                transform,
                ..
            } => {
                let component_ref = *self.components.get(component.as_str()).ok_or_else(|| {
                    AuthoringDiagnostic::new(
                        format!("{authored_path}.component"),
                        "unknown_component",
                        format!("component '{component}' is not defined"),
                    )
                })?;
                if component_stack.iter().any(|entry| entry == component) {
                    let mut cycle = component_stack.clone();
                    cycle.push(component.clone());
                    return Err(AuthoringDiagnostic::new(
                        format!("{authored_path}.component"),
                        "component_cycle",
                        format!("component cycle detected: {}", cycle.join(" -> ")),
                    ));
                }
                for (name, quantity) in overrides {
                    if !component_ref.spec.parameters.contains_key(name) {
                        return Err(AuthoringDiagnostic::new(
                            format!("{authored_path}.overrides.{name}"),
                            "unknown_override",
                            format!(
                                "component '{}' has no parameter named '{name}'",
                                component_ref.spec.id
                            ),
                        ));
                    }
                    if !quantity.value.is_finite() {
                        return Err(AuthoringDiagnostic::new(
                            format!("{authored_path}.overrides.{name}.value"),
                            "non_finite",
                            "numeric values must be finite",
                        ));
                    }
                }

                let transform_values =
                    evaluate_transform(transform, &format!("{authored_path}.transform"), scope)?;
                let wrapper_name = runtime_name(&runtime_segments, "instance");
                self.register_runtime_names(
                    std::slice::from_ref(&wrapper_name),
                    &format!("{authored_path}.id"),
                )?;
                self.source_map.entries.push(SourceMapEntry {
                    authored_id: authored_id.clone(),
                    authored_path: authored_path.clone(),
                    definition_path: definition_path.clone(),
                    runtime_names: vec![wrapper_name.clone()],
                    scene_paths: vec![scene_path.clone()],
                });

                let mut component_scope = self.spec.parameters.clone();
                component_scope.extend(component_ref.spec.parameters.clone());
                component_scope.extend(overrides.clone());
                component_stack.push(component.clone());
                let mut lowered_children = Vec::with_capacity(component_ref.spec.visual.len());
                for (index, child) in component_ref.spec.visual.iter().enumerate() {
                    let child_authored_path = format!("{authored_path}.expanded[{index}]");
                    let child_definition_path = Some(format!(
                        "$.components[{}].visual[{index}]",
                        component_ref.index
                    ));
                    let child_authored_id = format!("{authored_id}/{}", child.id());
                    let mut child_runtime_segments = runtime_segments.clone();
                    child_runtime_segments.push(child.id().to_string());
                    let child_scene_path = format!("{scene_path}/children/{index}");
                    match self.lower_node(
                        child,
                        NodeContext {
                            authored_path: child_authored_path,
                            definition_path: child_definition_path,
                            authored_id: child_authored_id,
                            runtime_segments: child_runtime_segments,
                            scene_path: child_scene_path,
                            scope: &component_scope,
                        },
                        component_stack,
                    ) {
                        Ok(child) => lowered_children.push(child),
                        Err(error) => {
                            component_stack.pop();
                            return Err(error);
                        }
                    }
                }
                component_stack.pop();

                Ok(json!({
                    "type": "node",
                    "name": wrapper_name,
                    "x": transform_values.x,
                    "y": transform_values.y,
                    "rotation": transform_values.rotation,
                    "scale_x": transform_values.scale_x,
                    "scale_y": transform_values.scale_y,
                    "children": lowered_children
                }))
            }
            VisualNode::RawSceneObject { object, .. } => {
                if !object.is_object() {
                    return Err(AuthoringDiagnostic::new(
                        format!("{authored_path}.object"),
                        "invalid_raw_scene_object",
                        "raw SceneSpec escape must be a JSON object",
                    ));
                }
                let mut runtime_names = Vec::new();
                let mut scene_paths = Vec::new();
                collect_named_paths(object, &scene_path, &mut runtime_names, &mut scene_paths);
                self.register_runtime_names(&runtime_names, &format!("{authored_path}.object"))?;
                self.source_map.entries.push(SourceMapEntry {
                    authored_id,
                    authored_path,
                    definition_path,
                    runtime_names,
                    scene_paths: if scene_paths.is_empty() {
                        vec![scene_path]
                    } else {
                        scene_paths
                    },
                });
                Ok(object.clone())
            }
            VisualNode::Ellipse { .. }
            | VisualNode::Rectangle { .. }
            | VisualNode::Triangle { .. }
            | VisualNode::Polygon { .. }
            | VisualNode::Star { .. }
            | VisualNode::Text { .. }
            | VisualNode::Image { .. }
            | VisualNode::Grid { .. }
            | VisualNode::Radial { .. }
            | VisualNode::Mirror { .. }
            | VisualNode::Distribute { .. }
            | VisualNode::AlongPath { .. } => {
                unreachable!("shape, text, image, and pattern nodes are handled above")
            }
        }
    }
}
