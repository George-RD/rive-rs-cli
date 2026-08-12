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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use super::super::super::spec::{AuthoringSourceMap, SourceMapEntry};

    fn source_entry(authored_path: &str, runtime_name: &str) -> SourceMapEntry {
        SourceMapEntry {
            authored_id: authored_path.to_string(),
            authored_path: authored_path.to_string(),
            definition_path: None,
            runtime_names: vec![runtime_name.to_string()],
            scene_paths: vec!["/artboard/children/0".to_string()],
        }
    }

    fn lowered(entries: Vec<SourceMapEntry>) -> LoweredAuthoring {
        LoweredAuthoring {
            scene: json!({ "scene_format_version": 1 }),
            source_map: AuthoringSourceMap { entries },
        }
    }

    #[test]
    fn compiler_state_round_trips_scene_and_source_map() {
        let lowered = lowered(vec![source_entry("$.visual.nodes[0]", "card")]);
        let expected = lowered.clone();

        let actual = CompilerState::from_lowered(lowered)
            .expect("unique runtime names must initialize compiler state")
            .into_lowered();

        assert_eq!(actual, expected);
    }

    #[test]
    fn compiler_state_rejects_duplicate_raw_runtime_name_at_value_path() {
        let result = CompilerState::from_lowered(lowered(vec![
            source_entry("$.visual.nodes[0]", "shared"),
            source_entry("$.motion.raw_animations[0]", "shared"),
        ]));
        let Err(error) = result else {
            panic!("duplicate runtime names must fail compiler-state initialization");
        };

        assert_eq!(error.diagnostics.len(), 1);
        let diagnostic = &error.diagnostics[0];
        assert_eq!(diagnostic.path, "$.motion.raw_animations[0].value");
        assert_eq!(diagnostic.code, "runtime_name_collision");
        assert_eq!(
            diagnostic.message,
            "runtime name 'shared' is generated or declared more than once"
        );
    }
}
