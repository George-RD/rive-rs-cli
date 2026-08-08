use super::super::spec::{AuthoringError, AuthoringSpec, LoweredAuthoring};
use super::{lower_target_graph, motion, rewrite_error_paths, validate_runtime_names};

pub(super) struct AuthoringCompiler<'a> {
    spec: &'a AuthoringSpec,
    lowered: LoweredAuthoring,
}

impl<'a> AuthoringCompiler<'a> {
    pub(super) fn new(spec: &'a AuthoringSpec) -> Result<Self, AuthoringError> {
        let lowered = lower_target_graph(spec)?;
        Ok(Self { spec, lowered })
    }

    pub(super) fn lower_motion(self) -> Result<Self, AuthoringError> {
        let lowered = motion::lower_motion(self.spec, self.lowered)
            .map_err(|error| rewrite_error_paths(self.spec, error))?;
        Ok(Self {
            spec: self.spec,
            lowered,
        })
    }

    pub(super) fn finish(self) -> Result<LoweredAuthoring, AuthoringError> {
        validate_runtime_names(self.lowered)
    }
}
