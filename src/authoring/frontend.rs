mod compiler;
mod motion;

use std::collections::{BTreeMap, HashSet};

use compiler::AuthoringCompiler;
use super::lower;
use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringArtboard, AuthoringDiagnostic, AuthoringError,
    AuthoringSpec, BehaviorSection, LoweredAuthoring, MotionSection, Quantity, RawSceneFragment,
    TransformSpec, Unit, VisualSection,
};
use super::validation::validate_numeric_values;
use super::visual::VisualNode;

pub fn lower_authoring(spec: &AuthoringSpec) -> Result<LoweredAuthoring, AuthoringError> {
    validate_authoring(spec)?;
    AuthoringCompiler::new(spec)?.lower_motion()?.finish()
}

fn lower_target_graph(spec: &AuthoringSpec) -> Result<LoweredAuthoring, AuthoringError> {
    if spec.motion.tracks.is_empty() {
        return lower::lower_authoring(spec).map_err(|error| rewrite_error_paths(spec, error));
    }

    let mut target_spec = spec.clone();
    target_spec.motion = MotionSection::default();
    target_spec.behavior = BehaviorSection::default();
    lower::lower_authoring(&target_spec).map_err(|error| rewrite_error_paths(spec, error))
}

fn validate_authoring(spec: &AuthoringSpec) -> Result<(), AuthoringError> {
    let mut name_diagnostics = validate_authored_names(spec);
    name_diagnostics.extend(motion::validate_motion(&spec.motion));
    if !name_diagnostics.is_empty() {
        return Err(AuthoringError::many(name_diagnostics));
    }

    validate_raw_fragment_runtime_names(spec)?;

    let numeric_diagnostics = validate_numeric_values(spec);
    if !numeric_diagnostics.is_empty() {
        return Err(AuthoringError::many(numeric_diagnostics));
    }

    validate_component_definitions(spec)
}

fn validate_runtime_names(lowered: LoweredAuthoring) -> Result<LoweredAuthoring, AuthoringError> {
    let mut names = HashSet::new();
    for entry in &lowered.source_map.entries {
        for name in &entry.runtime_names {
            if !names.insert(name.as_str()) {
                let path = if entry.authored_path.starts_with("$.motion.raw_animations[")
                    || entry
                        .authored_path
                        .starts_with("$.behavior.raw_state_machines[")
                {
                    format!("{}.value", entry.authored_path)
                } else {
                    entry.authored_path.clone()
                };
                return Err(AuthoringError::one(AuthoringDiagnostic::new(
                    path,
                    "runtime_name_collision",
                    format!("runtime name '{name}' is generated or declared more than once"),
                )));
            }
        }
    }
    Ok(lowered)
}

fn validate_raw_fragment_runtime_names(spec: &AuthoringSpec) -> Result<(), AuthoringError> {
    let mut names = HashSet::new();
    for (fragments, list_path) in [
        (
            spec.motion.raw_animations.as_slice(),
            "$.motion.raw_animations",
        ),
        (
            spec.behavior.raw_state_machines.as_slice(),
            "$.behavior.raw_state_machines",
        ),
    ] {
        for (index, fragment) in fragments.iter().enumerate() {
            let mut declared_names = Vec::new();
            collect_declared_names(&fragment.value, &mut declared_names);
            for name in declared_names {
                if !names.insert(name.clone()) {
                    return Err(AuthoringError::one(AuthoringDiagnostic::new(
                        format!("{list_path}[{index}].value"),
                        "runtime_name_collision",
                        format!("runtime name '{name}' is declared more than once"),
                    )));
                }
            }
        }
    }
    Ok(())
}

fn collect_declared_names(value: &serde_json::Value, names: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(object) => {
            if let Some(name) = object.get("name").and_then(serde_json::Value::as_str) {
                names.push(name.to_string());
            }
            if let Some(serde_json::Value::Array(children)) = object.get("children") {
                for child in children {
                    collect_declared_names(child, names);
                }
            }
        }
        serde_json::Value::Array(values) => {
            for child in values {
                collect_declared_names(child, names);
            }
        }
        _ => {}
    }
}

fn validate_authored_names(spec: &AuthoringSpec) -> Vec<AuthoringDiagnostic> {
    let mut diagnostics = Vec::new();
    validate_id(&spec.artboard.id, "$.artboard.id", &mut diagnostics);
    validate_file_assets(&spec.font_assets, "$.font_assets", "font", &mut diagnostics);
    validate_file_assets(
        &spec.image_assets,
        "$.image_assets",
        "image",
        &mut diagnostics,
    );
    validate_parameter_names(&spec.parameters, "$.parameters", &mut diagnostics);

    for (component_index, component) in spec.components.iter().enumerate() {
        let component_path = format!("$.components[{component_index}]");
        validate_id(
            &component.id,
            &format!("{component_path}.id"),
            &mut diagnostics,
        );
        validate_parameter_names(
            &component.parameters,
            &format!("{component_path}.parameters"),
            &mut diagnostics,
        );
        validate_node_names(
            &component.visual,
            &format!("{component_path}.visual"),
            &mut diagnostics,
        );
    }

    validate_node_names(&spec.visual.nodes, "$.visual.nodes", &mut diagnostics);
    validate_fragment_names(
        &spec.motion.raw_animations,
        "$.motion.raw_animations",
        &mut diagnostics,
    );
    validate_fragment_names(
        &spec.behavior.raw_state_machines,
        "$.behavior.raw_state_machines",
        &mut diagnostics,
    );
    diagnostics
}

fn validate_id(id: &str, path: &str, diagnostics: &mut Vec<AuthoringDiagnostic>) {
    if id.trim().is_empty() {
        diagnostics.push(AuthoringDiagnostic::new(
            path,
            "invalid_id",
            "authored ids must not be empty",
        ));
    }
    if id.contains('/') {
        diagnostics.push(AuthoringDiagnostic::new(
            path,
            "invalid_id",
            "authored ids must not contain the reserved '/' source-map separator",
        ));
    }
}

fn validate_file_assets(
    assets: &BTreeMap<String, String>,
    list_path: &str,
    kind: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (id, source) in assets {
        if !is_authored_map_key(id) {
            diagnostics.push(AuthoringDiagnostic::new(
                list_path,
                "invalid_asset_id",
                format!(
                    "{kind} asset id '{id}' must contain only ASCII letters, digits, '_' or '-'"
                ),
            ));
        }
        if source.trim().is_empty() {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{list_path}.{id}"),
                "invalid_asset_source",
                format!("{kind} asset source must not be empty"),
            ));
        }
    }
}

fn validate_parameter_names(
    parameters: &BTreeMap<String, Quantity>,
    path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for name in parameters.keys() {
        if !is_authored_map_key(name) {
            diagnostics.push(AuthoringDiagnostic::new(
                path,
                "invalid_parameter",
                format!(
                    "parameter name '{name}' must contain only ASCII letters, digits, '_' or '-'"
                ),
            ));
        }
    }
}

fn is_authored_map_key(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

fn validate_node_names(
    nodes: &[VisualNode],
    list_path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (index, node) in nodes.iter().enumerate() {
        validate_node_name(node, &format!("{list_path}[{index}]"), diagnostics);
    }
}

fn validate_node_name(node: &VisualNode, path: &str, diagnostics: &mut Vec<AuthoringDiagnostic>) {
    validate_id(node.id(), &format!("{path}.id"), diagnostics);
    if let Some(children) = node.children() {
        validate_node_names(children, &format!("{path}.children"), diagnostics);
    }
    if let Some(pattern) = node.pattern() {
        validate_node_name(pattern.item(), &format!("{path}.item"), diagnostics);
    }
    if let VisualNode::Instance { overrides, .. } = node {
        validate_parameter_names(overrides, &format!("{path}.overrides"), diagnostics);
    }
}

fn validate_fragment_names(
    fragments: &[RawSceneFragment],
    list_path: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (index, fragment) in fragments.iter().enumerate() {
        validate_id(
            &fragment.id,
            &format!("{list_path}[{index}].id"),
            diagnostics,
        );
    }
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
            font_assets: spec.font_assets.clone(),
            image_assets: spec.image_assets.clone(),
            parameters: BTreeMap::new(),
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

        if let Some(rest) = remainder.strip_prefix(".item") {
            let pattern = node.pattern()?;
            node = pattern.item();
            resolved.push_str(".item");
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
