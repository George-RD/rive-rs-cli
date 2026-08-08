mod compiler;
mod motion;

use std::collections::{BTreeMap, HashSet};

use super::lower;
use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringArtboard, AuthoringDiagnostic, AuthoringError,
    AuthoringSpec, BehaviorSection, LoweredAuthoring, MotionSection, Quantity, RawSceneFragment,
    TransformSpec, Unit, VisualSection,
};
use super::validation::validate_numeric_values;
use super::visual::VisualNode;
use compiler::AuthoringCompiler;

pub fn lower_authoring(spec: &AuthoringSpec) -> Result<LoweredAuthoring, AuthoringError> {
    validate_authoring(spec)?;
    AuthoringCompiler::new(spec)?.lower_motion()?.finish()
}

fn lower_target_graph(spec: &AuthoringSpec) -> Result<LoweredAuthoring, AuthoringError> {
    if spec.motion.tracks.is_empty() {
        return lower::lower_authoring(spec).map_err(|error| rewrite_error_paths(spec, error));
    }
