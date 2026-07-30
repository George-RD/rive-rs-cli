use std::collections::{BTreeMap, HashMap, HashSet};

use serde_json::{Value, json};

use crate::builder::{SceneSpec, build_scene};

use super::expression::{evaluate_expression, evaluate_quantity, evaluate_transform};
use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringDiagnostic, AuthoringError, AuthoringSourceMap,
    AuthoringSpec, ComponentSpec, GradientKind, LoweredAuthoring, PaintSpec, Quantity, ScalarExpr,
    ShapeNodeRef, SourceMapEntry, TrimPathMode, TrimPathSpec, Unit, VisualNode,
};

#[derive(Clone, Copy)]
struct ComponentRef<'a> {
    index: usize,
    spec: &'a ComponentSpec,
}

struct Lowerer<'a> {
    spec: &'a AuthoringSpec,
    components: HashMap<&'a str, ComponentRef<'a>>,
    source_map: AuthoringSourceMap,
    runtime_names: HashSet<String>,
}

struct NodeContext<'a> {
    authored_path: String,
    definition_path: Option<String>,
    authored_id: String,
    runtime_segments: Vec<String>,
    scene_path: String,
    scope: &'a BTreeMap<String, Quantity>,
}

struct LoweredObject {
    object: Value,
    runtime_names: Vec<String>,
    scene_paths: Vec<String>,
}

#[derive(Clone, Copy)]
enum PaintTarget {
    Fill,
    Stroke,
}

impl PaintTarget {
    fn runtime_name(self, segments: &[String], role: &str) -> String {
        match self {
            Self::Fill => runtime_name(segments, role),
            Self::Stroke => runtime_name(segments, &format!("stroke_{role}")),
        }
    }
}

pub fn lower_authoring(spec: &AuthoringSpec) -> Result<LoweredAuthoring, AuthoringError> {
    let mut diagnostics = Vec::new();
    if spec.authoring_format_version != AUTHORING_FORMAT_VERSION {
        diagnostics.push(AuthoringDiagnostic::new(
            "$.authoring_format_version",
            "unsupported_version",
            format!(
                "AuthoringSpec version {} is unsupported; expected {AUTHORING_FORMAT_VERSION}",
                spec.authoring_format_version
            ),
        ));
    }
    validate_id(&spec.artboard.id, "$.artboard.id", &mut diagnostics);
    validate_parameters(&spec.parameters, "$.parameters", &mut diagnostics);

    let mut components = HashMap::new();
    for (index, component) in spec.components.iter().enumerate() {
        let component_path = format!("$.components[{index}]");
        validate_id(
            &component.id,
            &format!("{component_path}.id"),
            &mut diagnostics,
        );
        validate_parameters(
            &component.parameters,
            &format!("{component_path}.parameters"),
            &mut diagnostics,
        );
        validate_sibling_ids(
            &component.visual,
            &format!("{component_path}.visual"),
            &mut diagnostics,
        );
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
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{component_path}.id"),
                "duplicate_component",
                format!("component id '{}' is duplicated", component.id),
            ));
        }
    }
    validate_sibling_ids(&spec.visual.nodes, "$.visual.nodes", &mut diagnostics);
    validate_fragment_ids(
        &spec.motion.raw_animations,
        "$.motion.raw_animations",
        &mut diagnostics,
    );
    validate_fragment_ids(
        &spec.behavior.raw_state_machines,
        "$.behavior.raw_state_machines",
        &mut diagnostics,
    );

    if !diagnostics.is_empty() {
        return Err(AuthoringError::many(diagnostics));
    }

    Lowerer {
        spec,
        components,
        source_map: AuthoringSourceMap::default(),
        runtime_names: HashSet::new(),
    }
    .lower()
}

impl<'a> Lowerer<'a> {
    fn lower(mut self) -> Result<LoweredAuthoring, AuthoringError> {
        let width = evaluate_quantity(self.spec.artboard.width, "$.artboard.width", Unit::Px)
            .map_err(AuthoringError::one)?;
        let height = evaluate_quantity(self.spec.artboard.height, "$.artboard.height", Unit::Px)
            .map_err(AuthoringError::one)?;
        if width <= 0.0 {
            return Err(AuthoringError::one(AuthoringDiagnostic::new(
                "$.artboard.width",
                "invalid_dimension",
                "artboard width must be greater than zero",
            )));
        }
        if height <= 0.0 {
            return Err(AuthoringError::one(AuthoringDiagnostic::new(
                "$.artboard.height",
                "invalid_dimension",
                "artboard height must be greater than zero",
            )));
        }

        let artboard_name = runtime_name(&[self.spec.artboard.id.clone()], "artboard");
        self.register_runtime_names(std::slice::from_ref(&artboard_name), "$.artboard.id")
            .map_err(AuthoringError::one)?;
        self.source_map.entries.push(SourceMapEntry {
            authored_id: self.spec.artboard.id.clone(),
            authored_path: "$.artboard".to_string(),
            definition_path: None,
            runtime_names: vec![artboard_name.clone()],
            scene_paths: vec!["/artboard".to_string()],
        });

        let mut children = Vec::with_capacity(self.spec.visual.nodes.len());
        let mut component_stack = Vec::new();
        for (index, node) in self.spec.visual.nodes.iter().enumerate() {
            let authored_path = format!("$.visual.nodes[{index}]");
            let authored_id = node.id().to_string();
            let runtime_segments = vec![self.spec.artboard.id.clone(), authored_id.clone()];
            let scene_path = format!("/artboard/children/{index}");
            let child = self
                .lower_node(
                    node,
                    NodeContext {
                        authored_path,
                        definition_path: None,
                        authored_id,
                        runtime_segments,
                        scene_path,
                        scope: &self.spec.parameters,
                    },
                    &mut component_stack,
                )
                .map_err(AuthoringError::one)?;
            children.push(child);
        }

        let animations = self
            .lower_raw_fragments(
                &self.spec.motion.raw_animations,
                "$.motion.raw_animations",
                "/artboard/animations",
            )
            .map_err(AuthoringError::one)?;
        let state_machines = self
            .lower_raw_fragments(
                &self.spec.behavior.raw_state_machines,
                "$.behavior.raw_state_machines",
                "/artboard/state_machines",
            )
            .map_err(AuthoringError::one)?;

        let mut artboard = json!({
            "name": artboard_name,
            "width": width,
            "height": height,
            "children": children
        });
        if let Some(object) = artboard.as_object_mut() {
            if !animations.is_empty() {
                object.insert("animations".to_string(), Value::Array(animations));
            }
            if !state_machines.is_empty() {
                object.insert("state_machines".to_string(), Value::Array(state_machines));
            }
        }
        let scene = json!({
            "scene_format_version": 1,
            "artboard": artboard
        });

        let scene_spec = serde_json::from_value::<SceneSpec>(scene.clone()).map_err(|error| {
            AuthoringError::one(AuthoringDiagnostic::new(
                "$.lowered_scene",
                "invalid_lowered_scene",
                error.to_string(),
            ))
        })?;
        build_scene(&scene_spec, None).map_err(|error| {
            AuthoringError::one(AuthoringDiagnostic::new(
                "$.lowered_scene",
                "builder_rejected_scene",
                error.to_string(),
            ))
        })?;

        Ok(LoweredAuthoring {
            scene,
            source_map: self.source_map,
        })
    }

    fn lower_node(
        &mut self,
        node: &VisualNode,
        context: NodeContext<'_>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        if let Some(shape) = node.shape() {
            return self.lower_shape(shape, context);
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
                children,
                ..
            } => {
                validate_sibling_ids_result(children, &format!("{authored_path}.children"))?;
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
            | VisualNode::Star { .. } => unreachable!("shape nodes are handled above"),
        }
    }

    fn lower_shape(
        &mut self,
        shape: ShapeNodeRef<'_>,
        context: NodeContext<'_>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let ShapeNodeRef {
            geometry_type,
            width: width_expression,
            height: height_expression,
            points,
            corner_radius: corner_radius_expression,
            inner_radius: inner_radius_expression,
            fill,
            stroke,
            transform,
        } = shape;
        let NodeContext {
            authored_path,
            definition_path,
            authored_id,
            runtime_segments,
            scene_path,
            scope,
        } = context;

        let width = evaluate_expression(
            width_expression,
            &format!("{authored_path}.width"),
            scope,
            Unit::Px,
        )?;
        let height = evaluate_expression(
            height_expression,
            &format!("{authored_path}.height"),
            scope,
            Unit::Px,
        )?;
        if width <= 0.0 {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.width"),
                "invalid_dimension",
                "shape width must be greater than zero",
            ));
        }
        if height <= 0.0 {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.height"),
                "invalid_dimension",
                "shape height must be greater than zero",
            ));
        }
        if points.is_some_and(|points| points < 3) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.points"),
                "invalid_points",
                "polygon and star point counts must be at least three",
            ));
        }
        let corner_radius = corner_radius_expression
            .map(|expression| {
                evaluate_expression(
                    expression,
                    &format!("{authored_path}.corner_radius"),
                    scope,
                    Unit::Px,
                )
            })
            .transpose()?;
        if corner_radius.is_some_and(|radius| radius < 0.0) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.corner_radius"),
                "invalid_dimension",
                "corner radius must not be negative",
            ));
        }
        let inner_radius = inner_radius_expression
            .map(|expression| {
                let path = format!("{authored_path}.inner_radius");
                evaluate_ratio_expression(
                    expression,
                    &path,
                    scope,
                    "star inner radius must be between zero and one",
                )
            })
            .transpose()?;
        let stroke_thickness = stroke
            .map(|stroke| {
                evaluate_expression(
                    &stroke.width,
                    &format!("{authored_path}.stroke.width"),
                    scope,
                    Unit::Px,
                )
            })
            .transpose()?;
        if stroke_thickness.is_some_and(|thickness| thickness <= 0.0) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.stroke.width"),
                "invalid_dimension",
                "stroke width must be greater than zero",
            ));
        }
        let transform_values =
            evaluate_transform(transform, &format!("{authored_path}.transform"), scope)?;

        let shape_name = runtime_name(&runtime_segments, "shape");
        let geometry_name = runtime_name(&runtime_segments, "geometry");
        let fill_name = runtime_name(&runtime_segments, "fill");
        let LoweredObject {
            object: fill_paint,
            runtime_names: fill_runtime_names,
            scene_paths: fill_scene_paths,
        } = self.lower_paint(
            fill,
            &format!("{authored_path}.fill"),
            &runtime_segments,
            &format!("{scene_path}/children/1/children/0"),
            scope,
            PaintTarget::Fill,
        )?;
        let mut runtime_names = vec![shape_name.clone(), geometry_name.clone(), fill_name.clone()];
        runtime_names.extend(fill_runtime_names);
        let mut scene_paths = vec![
            scene_path.clone(),
            format!("{scene_path}/children/0"),
            format!("{scene_path}/children/1"),
        ];
        scene_paths.extend(fill_scene_paths);

        let mut geometry = json!({
            "type": geometry_type,
            "name": geometry_name,
            "width": width,
            "height": height,
            "origin_x": 0.5,
            "origin_y": 0.5
        });
        if let Some(object) = geometry.as_object_mut() {
            if matches!(geometry_type, "rectangle" | "polygon" | "star") {
                object.insert(
                    "corner_radius".to_string(),
                    corner_radius.map_or(Value::Null, Value::from),
                );
            }
            if let Some(points) = points {
                object.insert("points".to_string(), Value::from(points));
            }
            if let Some(inner_radius) = inner_radius {
                object.insert("inner_radius".to_string(), Value::from(inner_radius));
            }
        }

        let mut children = vec![
            geometry,
            json!({
                "type": "fill",
                "name": fill_name,
                "children": [fill_paint]
            }),
        ];
        if let (Some(stroke), Some(thickness)) = (stroke, stroke_thickness) {
            let stroke_name = runtime_name(&runtime_segments, "stroke");
            let LoweredObject {
                object: stroke_paint,
                runtime_names: stroke_runtime_names,
                scene_paths: stroke_scene_paths,
            } = self.lower_paint(
                &stroke.paint,
                &format!("{authored_path}.stroke.paint"),
                &runtime_segments,
                &format!("{scene_path}/children/2/children/0"),
                scope,
                PaintTarget::Stroke,
            )?;
            runtime_names.push(stroke_name.clone());
            runtime_names.extend(stroke_runtime_names);
            scene_paths.push(format!("{scene_path}/children/2"));
            scene_paths.extend(stroke_scene_paths);

            let mut stroke_children = vec![stroke_paint];
            if let Some(trim) = &stroke.trim {
                let LoweredObject {
                    object,
                    runtime_names: trim_runtime_names,
                    scene_paths: trim_scene_paths,
                } = self.lower_trim_path(
                    trim,
                    &format!("{authored_path}.stroke.trim"),
                    &runtime_segments,
                    &format!("{scene_path}/children/2/children/1"),
                    scope,
                )?;
                runtime_names.extend(trim_runtime_names);
                scene_paths.extend(trim_scene_paths);
                stroke_children.push(object);
            }

            children.push(json!({
                "type": "stroke",
                "name": stroke_name,
                "thickness": thickness,
                "children": stroke_children
            }));
        }

        self.register_runtime_names(&runtime_names, &format!("{authored_path}.id"))?;
        self.source_map.entries.push(SourceMapEntry {
            authored_id,
            authored_path,
            definition_path,
            runtime_names,
            scene_paths,
        });

        Ok(json!({
            "type": "shape",
            "name": shape_name,
            "x": transform_values.x,
            "y": transform_values.y,
            "rotation": transform_values.rotation,
            "scale_x": transform_values.scale_x,
            "scale_y": transform_values.scale_y,
            "children": children
        }))
    }

    fn lower_trim_path(
        &self,
        trim: &TrimPathSpec,
        authored_path: &str,
        runtime_segments: &[String],
        scene_path: &str,
        scope: &BTreeMap<String, Quantity>,
    ) -> Result<LoweredObject, AuthoringDiagnostic> {
        let start_path = format!("{authored_path}.start");
        let start = evaluate_ratio_expression(
            &trim.start,
            &start_path,
            scope,
            "trim start must be between zero and one",
        )?;
        let end_path = format!("{authored_path}.end");
        let end = evaluate_ratio_expression(
            &trim.end,
            &end_path,
            scope,
            "trim end must be between zero and one",
        )?;

        let offset = trim
            .offset
            .as_ref()
            .map(|expression| {
                evaluate_expression(
                    expression,
                    &format!("{authored_path}.offset"),
                    scope,
                    Unit::Scalar,
                )
            })
            .transpose()?
            .unwrap_or(0.0);
        let mode = match trim.mode {
            TrimPathMode::Sequential => "sequential",
            TrimPathMode::Synchronized => "synchronized",
        };
        let runtime_name = runtime_name(runtime_segments, "stroke_trim");

        Ok(LoweredObject {
            object: json!({
                "type": "trim_path",
                "name": runtime_name.clone(),
                "start": start,
                "end": end,
                "offset": offset,
                "mode": mode
            }),
            runtime_names: vec![runtime_name],
            scene_paths: vec![scene_path.to_string()],
        })
    }

    fn lower_paint(
        &self,
        paint: &PaintSpec,
        authored_path: &str,
        runtime_segments: &[String],
        scene_path: &str,
        scope: &BTreeMap<String, Quantity>,
        target: PaintTarget,
    ) -> Result<LoweredObject, AuthoringDiagnostic> {
        match paint {
            PaintSpec::Solid(color) => {
                let color_name = target.runtime_name(runtime_segments, "color");
                Ok(LoweredObject {
                    object: json!({
                        "type": "solid_color",
                        "name": color_name.clone(),
                        "color": color
                    }),
                    runtime_names: vec![color_name],
                    scene_paths: vec![scene_path.to_string()],
                })
            }
            PaintSpec::Gradient(gradient) => {
                if gradient.stops.len() < 2 {
                    return Err(AuthoringDiagnostic::new(
                        format!("{authored_path}.stops"),
                        "invalid_gradient_stops",
                        "gradient paints require at least two stops",
                    ));
                }

                let start_x = evaluate_expression(
                    &gradient.start_x,
                    &format!("{authored_path}.start_x"),
                    scope,
                    Unit::Px,
                )?;
                let start_y = evaluate_expression(
                    &gradient.start_y,
                    &format!("{authored_path}.start_y"),
                    scope,
                    Unit::Px,
                )?;
                let end_x = evaluate_expression(
                    &gradient.end_x,
                    &format!("{authored_path}.end_x"),
                    scope,
                    Unit::Px,
                )?;
                let end_y = evaluate_expression(
                    &gradient.end_y,
                    &format!("{authored_path}.end_y"),
                    scope,
                    Unit::Px,
                )?;

                let gradient_name = target.runtime_name(runtime_segments, "gradient");
                let mut runtime_names = vec![gradient_name.clone()];
                let mut scene_paths = vec![scene_path.to_string()];
                let mut children = Vec::with_capacity(gradient.stops.len());
                let mut previous_position = None;
                for (index, stop) in gradient.stops.iter().enumerate() {
                    let stop_path = format!("{authored_path}.stops[{index}].position");
                    let position = evaluate_ratio_expression(
                        &stop.position,
                        &stop_path,
                        scope,
                        "gradient stop positions must be between zero and one",
                    )?;
                    if previous_position.is_some_and(|previous| position < previous) {
                        return Err(AuthoringDiagnostic::new(
                            stop_path,
                            "invalid_gradient_stop_order",
                            "gradient stop positions must be in non-decreasing order",
                        ));
                    }
                    previous_position = Some(position);

                    let stop_name =
                        target.runtime_name(runtime_segments, &format!("gradient_stop_{index}"));
                    runtime_names.push(stop_name.clone());
                    scene_paths.push(format!("{scene_path}/children/{index}"));
                    children.push(json!({
                        "type": "gradient_stop",
                        "name": stop_name,
                        "color": stop.color.as_str(),
                        "position": position
                    }));
                }

                let gradient_type = match gradient.kind {
                    GradientKind::LinearGradient => "linear_gradient",
                    GradientKind::RadialGradient => "radial_gradient",
                };
                Ok(LoweredObject {
                    object: json!({
                        "type": gradient_type,
                        "name": gradient_name,
                        "start_x": start_x,
                        "start_y": start_y,
                        "end_x": end_x,
                        "end_y": end_y,
                        "children": children
                    }),
                    runtime_names,
                    scene_paths,
                })
            }
        }
    }

    fn lower_raw_fragments(
        &mut self,
        fragments: &[super::spec::RawSceneFragment],
        authored_list_path: &str,
        scene_list_path: &str,
    ) -> Result<Vec<Value>, AuthoringDiagnostic> {
        let mut lowered = Vec::with_capacity(fragments.len());
        for (index, fragment) in fragments.iter().enumerate() {
            let authored_path = format!("{authored_list_path}[{index}]");
            if !fragment.value.is_object() {
                return Err(AuthoringDiagnostic::new(
                    format!("{authored_path}.value"),
                    "invalid_raw_scene_fragment",
                    "raw SceneSpec escape must be a JSON object",
                ));
            }
            let scene_path = format!("{scene_list_path}/{index}");
            let mut runtime_names = Vec::new();
            let mut scene_paths = Vec::new();
            collect_named_paths(
                &fragment.value,
                &scene_path,
                &mut runtime_names,
                &mut scene_paths,
            );
            self.source_map.entries.push(SourceMapEntry {
                authored_id: fragment.id.clone(),
                authored_path,
                definition_path: None,
                runtime_names,
                scene_paths: if scene_paths.is_empty() {
                    vec![scene_path]
                } else {
                    scene_paths
                },
            });
            lowered.push(fragment.value.clone());
        }
        Ok(lowered)
    }

    fn register_runtime_names(
        &mut self,
        names: &[String],
        path: &str,
    ) -> Result<(), AuthoringDiagnostic> {
        for name in names {
            if !self.runtime_names.insert(name.clone()) {
                return Err(AuthoringDiagnostic::new(
                    path,
                    "runtime_name_collision",
                    format!("runtime name '{name}' is generated or declared more than once"),
                ));
            }
        }
        Ok(())
    }
}

fn evaluate_ratio_expression(
    expression: &ScalarExpr,
    path: &str,
    scope: &BTreeMap<String, Quantity>,
    message: &str,
) -> Result<f64, AuthoringDiagnostic> {
    let value = evaluate_expression(expression, path, scope, Unit::Scalar)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(AuthoringDiagnostic::new(path, "invalid_ratio", message));
    }
    Ok(value)
}

fn validate_id(id: &str, path: &str, diagnostics: &mut Vec<AuthoringDiagnostic>) {
    if id.trim().is_empty() {
        diagnostics.push(AuthoringDiagnostic::new(
            path,
            "invalid_id",
            "authored ids must not be empty",
        ));
    }
}

fn validate_parameters(
    parameters: &BTreeMap<String, Quantity>,
    path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (name, quantity) in parameters {
        if name.trim().is_empty() {
            diagnostics.push(AuthoringDiagnostic::new(
                path,
                "invalid_parameter",
                "parameter names must not be empty",
            ));
        }
        if !quantity.value.is_finite() {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{path}.{name}.value"),
                "non_finite",
                "numeric values must be finite",
            ));
        }
    }
}

fn validate_sibling_ids(
    nodes: &[VisualNode],
    list_path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    let mut ids = HashSet::new();
    for (index, node) in nodes.iter().enumerate() {
        let id_path = format!("{list_path}[{index}].id");
        validate_id(node.id(), &id_path, diagnostics);
        if !ids.insert(node.id()) {
            diagnostics.push(AuthoringDiagnostic::new(
                id_path,
                "duplicate_id",
                format!("authored id '{}' is duplicated among siblings", node.id()),
            ));
        }
    }
}

fn validate_sibling_ids_result(
    nodes: &[VisualNode],
    list_path: &str,
) -> Result<(), AuthoringDiagnostic> {
    let mut diagnostics = Vec::new();
    validate_sibling_ids(nodes, list_path, &mut diagnostics);
    if let Some(first) = diagnostics.into_iter().next() {
        Err(first)
    } else {
        Ok(())
    }
}

fn validate_fragment_ids(
    fragments: &[super::spec::RawSceneFragment],
    list_path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    let mut ids = HashSet::new();
    for (index, fragment) in fragments.iter().enumerate() {
        let id_path = format!("{list_path}[{index}].id");
        validate_id(&fragment.id, &id_path, diagnostics);
        if !ids.insert(fragment.id.as_str()) {
            diagnostics.push(AuthoringDiagnostic::new(
                id_path,
                "duplicate_id",
                format!("raw fragment id '{}' is duplicated", fragment.id),
            ));
        }
    }
}

fn runtime_name(segments: &[String], role: &str) -> String {
    let mut name = String::from("auth");
    for segment in segments {
        name.push_str("__");
        name.push_str(&encode_name_part(segment));
    }
    name.push_str("__");
    name.push_str(role);
    name
}

fn encode_name_part(input: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::new();
    for byte in input.bytes() {
        if byte.is_ascii_alphanumeric() {
            encoded.push(char::from(byte));
        } else {
            encoded.push('_');
            encoded.push(char::from(HEX[usize::from(byte >> 4)]));
            encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
    if encoded.is_empty() {
        "empty".to_string()
    } else {
        encoded
    }
}

fn collect_named_paths(
    value: &Value,
    scene_path: &str,
    names: &mut Vec<String>,
    paths: &mut Vec<String>,
) {
    match value {
        Value::Object(object) => {
            if let Some(name) = object.get("name").and_then(Value::as_str) {
                names.push(name.to_string());
                paths.push(scene_path.to_string());
            }
            if let Some(Value::Array(children)) = object.get("children") {
                for (index, child) in children.iter().enumerate() {
                    collect_named_paths(
                        child,
                        &format!("{scene_path}/children/{index}"),
                        names,
                        paths,
                    );
                }
            }
        }
        Value::Array(values) => {
            for (index, child) in values.iter().enumerate() {
                collect_named_paths(child, &format!("{scene_path}/{index}"), names, paths);
            }
        }
        _ => {}
    }
}
