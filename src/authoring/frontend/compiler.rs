use std::collections::HashSet;

use serde_json::Value;

use super::super::spec::{
    AuthoringDiagnostic, AuthoringError, AuthoringSourceMap, AuthoringSpec, LoweredAuthoring,
    SourceMapEntry,
};
use super::{lower_target_graph, motion, rewrite_error_paths};

pub(super) struct AuthoringCompiler<'a> {
    spec: &'a AuthoringSpec,
    state: CompilerState,
}

struct CompilerState {
    scene: Value,
    source_map: AuthoringSourceMap,
    runtime_names: RuntimeNameRegistry,
}

#[derive(Default)]
struct RuntimeNameRegistry {
    names: HashSet<String>,
}

impl<'a> AuthoringCompiler<'a> {
    pub(super) fn new(spec: &'a AuthoringSpec) -> Result<Self, AuthoringError> {
        let state = CompilerState::from_lowered(lower_target_graph(spec)?)?;
        Ok(Self { spec, state })
    }

    pub(super) fn lower_motion(self) -> Result<Self, AuthoringError> {
        let Self { spec, state } = self;
        let lowered = motion::lower_motion(spec, state.into_lowered())
            .map_err(|error| rewrite_error_paths(spec, error))?;
        Ok(Self {
            spec,
            state: CompilerState::from_lowered(lowered)?,
        })
    }

    pub(super) fn finish(self) -> Result<LoweredAuthoring, AuthoringError> {
        Ok(self.state.into_lowered())
    }
}

impl CompilerState {
    fn from_lowered(lowered: LoweredAuthoring) -> Result<Self, AuthoringError> {
        let LoweredAuthoring { scene, source_map } = lowered;
        let runtime_names = RuntimeNameRegistry::from_source_map(&source_map)?;
        Ok(Self {
            scene,
            source_map,
            runtime_names,
        })
    }

    fn into_lowered(self) -> LoweredAuthoring {
        let Self {
            scene,
            source_map,
            runtime_names,
        } = self;
        debug_assert_eq!(
            runtime_names.len(),
            source_map
                .entries
                .iter()
                .map(|entry| entry.runtime_names.len())
                .sum()
        );
        LoweredAuthoring { scene, source_map }
    }
}

impl RuntimeNameRegistry {
    fn from_source_map(source_map: &AuthoringSourceMap) -> Result<Self, AuthoringError> {
        let mut registry = Self::default();
        for entry in &source_map.entries {
            registry.register(entry)?;
        }
        Ok(registry)
    }

    fn len(&self) -> usize {
        self.names.len()
    }

    fn register(&mut self, entry: &SourceMapEntry) -> Result<(), AuthoringError> {
        for runtime_name in &entry.runtime_names {
            if !self.names.insert(runtime_name.clone()) {
                return Err(AuthoringError::one(AuthoringDiagnostic::new(
                    runtime_name_collision_path(entry),
                    "runtime_name_collision",
                    format!(
                        "runtime name '{runtime_name}' is generated or declared more than once"
                    ),
                )));
            }
        }
        Ok(())
    }
}

fn runtime_name_collision_path(entry: &SourceMapEntry) -> String {
    if entry
        .authored_path
        .starts_with("$.motion.raw_animations[")
        || entry
            .authored_path
            .starts_with("$.behavior.raw_state_machines[")
    {
        format!("{}.value", entry.authored_path)
    } else {
        entry.authored_path.clone()
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
