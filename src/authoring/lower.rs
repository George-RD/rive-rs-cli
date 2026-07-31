use std::collections::{BTreeMap, HashMap, HashSet};

use serde_json::{Value, json};

use crate::builder::{SceneSpec, build_scene};

use super::expression::{evaluate_expression, evaluate_quantity};
use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringDiagnostic, AuthoringError, AuthoringSourceMap,
    AuthoringSpec, ComponentSpec, LoweredAuthoring, Quantity, ScalarExpr, SourceMapEntry, Unit,
};
use super::visual::VisualNode;

mod node;
mod paint;
mod pattern;
mod shape;
mod text;

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
    Text,
}

impl PaintTarget {
    fn runtime_name(self, segments: &[String], role: &str) -> String {
        match self {
            Self::Fill => runtime_name(segments, role),
            Self::Stroke => runtime_name(segments, &format!("stroke_{role}")),
            Self::Text => runtime_name(segments, &format!("text_{role}")),
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

        let mut children =
            Vec::with_capacity(self.spec.font_assets.len() + self.spec.visual.nodes.len());
        for (index, (id, source)) in self.spec.font_assets.iter().enumerate() {
            let runtime_name = font_asset_runtime_name(&self.spec.artboard.id, id);
            let authored_path = format!("$.font_assets.{id}");
            self.register_runtime_names(std::slice::from_ref(&runtime_name), &authored_path)
                .map_err(AuthoringError::one)?;
            self.source_map.entries.push(SourceMapEntry {
                authored_id: id.clone(),
                authored_path,
                definition_path: None,
                runtime_names: vec![runtime_name.clone()],
                scene_paths: vec![format!("/artboard/children/{index}")],
            });
            children.push(json!({
                "type": "font_asset",
                "name": runtime_name,
                "source": source
            }));
        }
        let visual_offset = children.len();
        let mut component_stack = Vec::new();
        for (index, node) in self.spec.visual.nodes.iter().enumerate() {
            let authored_path = format!("$.visual.nodes[{index}]");
            let authored_id = node.id().to_string();
            let runtime_segments = vec![self.spec.artboard.id.clone(), authored_id.clone()];
            let scene_path = format!("/artboard/children/{}", visual_offset + index);
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

        let validation_scene = without_asset_sources(&scene);
        let scene_spec =
            serde_json::from_value::<SceneSpec>(validation_scene).map_err(|error| {
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

fn font_asset_runtime_name(artboard_id: &str, asset_id: &str) -> String {
    runtime_name(
        &[artboard_id.to_string(), asset_id.to_string()],
        "font_asset",
    )
}

fn without_asset_sources(scene: &Value) -> Value {
    let mut validation_scene = scene.clone();
    let Some(children) = validation_scene
        .pointer_mut("/artboard/children")
        .and_then(Value::as_array_mut)
    else {
        return validation_scene;
    };
    for child in children {
        let is_file_asset = matches!(
            child.get("type").and_then(Value::as_str),
            Some("font_asset" | "image_asset")
        );
        if is_file_asset && let Some(object) = child.as_object_mut() {
            object.remove("source");
        }
    }
    validation_scene
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
