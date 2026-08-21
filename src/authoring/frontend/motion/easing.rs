use serde_json::{Value, json};

use super::super::super::expression::evaluate_expression;
use super::super::super::lower;
use super::super::super::spec::{
    AuthoringDiagnostic, AuthoringSpec, MotionEasingSpec, ScalarExpr, SourceMapEntry, Unit,
};

pub(super) struct ResolvedEasing {
    authored_index: usize,
    pub(super) id: String,
    pub(super) runtime_name: String,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

pub(super) struct EasingEmission {
    authored_index: usize,
    authored_id: String,
    runtime_name: String,
    scene_paths: Vec<String>,
}

impl EasingEmission {
    pub(super) fn new(easing: &ResolvedEasing) -> Self {
        Self {
            authored_index: easing.authored_index,
            authored_id: easing.id.clone(),
            runtime_name: easing.runtime_name.clone(),
            scene_paths: Vec::new(),
        }
    }

    pub(super) fn record_declaration(&mut self, animation_index: usize, interpolator_index: usize) {
        self.scene_paths.push(format!(
            "/artboard/animations/{animation_index}/interpolators/{interpolator_index}"
        ));
    }
}

pub(super) fn resolve(spec: &AuthoringSpec) -> Result<Vec<ResolvedEasing>, AuthoringDiagnostic> {
    let mut resolved = Vec::with_capacity(spec.motion.easings.len());
    for (authored_index, easing) in spec.motion.easings.iter().enumerate() {
        let easing_path = format!("$.motion.easings[{authored_index}]");
        match easing {
            MotionEasingSpec::Cubic { id, x1, y1, x2, y2 } => {
                resolved.push(ResolvedEasing {
                    authored_index,
                    id: id.clone(),
                    runtime_name: lower::runtime_name(
                        &[spec.artboard.id.clone(), id.clone()],
                        "interpolator",
                    ),
                    x1: evaluate_time_control(x1, &format!("{easing_path}.x1"), spec)?,
                    y1: evaluate_expression(
                        y1,
                        &format!("{easing_path}.y1"),
                        &spec.parameters,
                        Unit::Scalar,
                    )?,
                    x2: evaluate_time_control(x2, &format!("{easing_path}.x2"), spec)?,
                    y2: evaluate_expression(
                        y2,
                        &format!("{easing_path}.y2"),
                        &spec.parameters,
                        Unit::Scalar,
                    )?,
                });
            }
        }
    }
    Ok(resolved)
}

pub(super) fn definition(easing: &ResolvedEasing) -> Value {
    json!({
        "name": easing.runtime_name,
        "type": "cubic",
        "x1": easing.x1,
        "y1": easing.y1,
        "x2": easing.x2,
        "y2": easing.y2
    })
}

pub(super) fn source_entries(emissions: Vec<EasingEmission>) -> Vec<SourceMapEntry> {
    emissions
        .into_iter()
        .filter(|emission| !emission.scene_paths.is_empty())
        .map(|emission| SourceMapEntry {
            authored_id: emission.authored_id,
            authored_path: format!("$.motion.easings[{}]", emission.authored_index),
            definition_path: None,
            runtime_names: vec![emission.runtime_name],
            scene_paths: emission.scene_paths,
        })
        .collect()
}

fn evaluate_time_control(
    expression: &ScalarExpr,
    path: &str,
    spec: &AuthoringSpec,
) -> Result<f64, AuthoringDiagnostic> {
    let value = evaluate_expression(expression, path, &spec.parameters, Unit::Scalar)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(AuthoringDiagnostic::new(
            path,
            "invalid_easing_control_point",
            "cubic easing x control points must be between 0 and 1",
        ));
    }
    Ok(value)
}
