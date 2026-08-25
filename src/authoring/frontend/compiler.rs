mod target_index;

use std::collections::{HashMap, HashSet};

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
    motion_targets: Result<MotionTargetIndex, AuthoringDiagnostic>,
}

pub(super) struct MotionLoweringInput {
    pub(super) lowered: LoweredAuthoring,
    pub(super) motion_targets: Result<MotionTargetIndex, AuthoringDiagnostic>,
}

pub(super) struct MotionLoweringOutput {
    pub(super) lowered: LoweredAuthoring,
    pub(super) typed_animation_count: usize,
    pub(super) source_entries: Vec<SourceMapEntry>,
}

pub(super) struct MotionTargetIndex {
    targets: HashMap<String, IndexedMotionTarget>,
}

enum IndexedMotionTarget {
    Unique(Vec<MotionTargetBinding>),
    Ambiguous,
}

pub(super) struct MotionTargetBinding {
    pub(super) runtime_name: String,
    pub(super) object_type: String,
    pub(super) is_primary: bool,
}

#[derive(Default)]
struct RuntimeNameRegistry {
    names: HashSet<String>,
    first_collision: Option<AuthoringDiagnostic>,
    binding_count: usize,
}

impl<'a> AuthoringCompiler<'a> {
    pub(super) fn new(spec: &'a AuthoringSpec) -> Result<Self, AuthoringError> {
        let state = CompilerState::from_lowered(lower_target_graph(spec)?);
        Ok(Self { spec, state })
    }

    pub(super) fn lower_motion(self) -> Result<Self, AuthoringError> {
        let Self { spec, state } = self;
        let MotionLoweringOutput {
            lowered,
            typed_animation_count,
            source_entries,
        } = motion::lower_motion(spec, state.into_motion_input())
            .map_err(|error| rewrite_error_paths(spec, error))?;
        Ok(Self {
            spec,
            state: CompilerState::from_lowered(lowered)
                .apply_motion_source_map(typed_animation_count, source_entries),
        })
    }

    pub(super) fn finish(self) -> Result<LoweredAuthoring, AuthoringError> {
        self.state.finish()
    }
}

impl CompilerState {
    fn from_lowered(lowered: LoweredAuthoring) -> Self {
        let LoweredAuthoring { scene, source_map } = lowered;
        let runtime_names = RuntimeNameRegistry::from_source_map(&source_map);
        let motion_targets = MotionTargetIndex::from_output(&scene, &source_map);
        Self {
            scene,
            source_map,
            runtime_names,
            motion_targets,
        }
    }

    fn apply_motion_source_map(
        mut self,
        typed_animation_count: usize,
        source_entries: Vec<SourceMapEntry>,
    ) -> Self {
        rewrite_motion_source_paths(&mut self.source_map, typed_animation_count);
        for entry in source_entries {
            self.runtime_names.register(&entry);
            self.source_map.entries.push(entry);
        }
        self
    }

    fn into_motion_input(self) -> MotionLoweringInput {
        let (scene, source_map, _, motion_targets) = self.into_parts();
        MotionLoweringInput {
            lowered: LoweredAuthoring { scene, source_map },
            motion_targets,
        }
    }

    fn finish(self) -> Result<LoweredAuthoring, AuthoringError> {
        let (scene, source_map, runtime_names, _) = self.into_parts();
        runtime_names.validate()?;
        Ok(LoweredAuthoring { scene, source_map })
    }

    fn into_parts(
        self,
    ) -> (
        Value,
        AuthoringSourceMap,
        RuntimeNameRegistry,
        Result<MotionTargetIndex, AuthoringDiagnostic>,
    ) {
        let Self {
            scene,
            source_map,
            runtime_names,
            motion_targets,
        } = self;
        debug_assert_eq!(
            runtime_names.binding_count(),
            source_map
                .entries
                .iter()
                .map(|entry| entry.runtime_names.len())
                .sum::<usize>()
        );
        (scene, source_map, runtime_names, motion_targets)
    }
}

impl MotionTargetIndex {
    pub(super) fn resolve(
        &self,
        target: &str,
        path: &str,
    ) -> Result<&[MotionTargetBinding], AuthoringDiagnostic> {
        match self.targets.get(target) {
            None => Err(AuthoringDiagnostic::new(
                path,
                "unknown_motion_target",
                format!("visual target '{target}' is not defined"),
            )),
            Some(IndexedMotionTarget::Ambiguous) => Err(AuthoringDiagnostic::new(
                path,
                "ambiguous_motion_target",
                format!("visual target '{target}' resolves to more than one authored node"),
            )),
            Some(IndexedMotionTarget::Unique(targets)) if targets.is_empty() => {
                Err(AuthoringDiagnostic::new(
                    path,
                    "unsupported_motion_target",
                    format!("visual target '{target}' has no animatable runtime object"),
                ))
            }
            Some(IndexedMotionTarget::Unique(targets)) => Ok(targets),
        }
    }
}

impl RuntimeNameRegistry {
    fn from_source_map(source_map: &AuthoringSourceMap) -> Self {
        let mut registry = Self::default();
        for entry in &source_map.entries {
            registry.register(entry);
        }
        registry
    }

    fn binding_count(&self) -> usize {
        self.binding_count
    }

    fn register(&mut self, entry: &SourceMapEntry) {
        for runtime_name in &entry.runtime_names {
            self.binding_count += 1;
            if !self.names.insert(runtime_name.clone()) && self.first_collision.is_none() {
                self.first_collision = Some(AuthoringDiagnostic::new(
                    runtime_name_collision_path(entry),
                    "runtime_name_collision",
                    format!(
                        "runtime name '{runtime_name}' is generated or declared more than once"
                    ),
                ));
            }
        }
    }

    fn validate(self) -> Result<(), AuthoringError> {
        match self.first_collision {
            Some(diagnostic) => Err(AuthoringError::one(diagnostic)),
            None => Ok(()),
        }
    }
}

fn rewrite_motion_source_paths(source_map: &mut AuthoringSourceMap, typed_animation_count: usize) {
    for entry in &mut source_map.entries {
        if let Some(path) = rewritten_motion_path(&entry.authored_path, typed_animation_count) {
            entry.authored_path = path;
        }
    }
}

pub(super) fn rewritten_motion_path(path: &str, typed_animation_count: usize) -> Option<String> {
    let remainder = path.strip_prefix("$.motion.raw_animations[")?;
    let close = remainder.find(']')?;
    let index = remainder[..close].parse::<usize>().ok()?;
    let suffix = &remainder[close + 1..];
    if index < typed_animation_count {
        let suffix = suffix.strip_prefix(".value").unwrap_or(suffix);
        Some(format!("$.motion.tracks[{index}]{suffix}"))
    } else {
        Some(format!(
            "$.motion.raw_animations[{}]{suffix}",
            index - typed_animation_count
        ))
    }
}

fn runtime_name_collision_path(entry: &SourceMapEntry) -> String {
    if entry.authored_path.starts_with("$.motion.raw_animations[")
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

    use super::super::super::spec::{AuthoringSourceMap, SourceMapEntry};
    use super::*;

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

    fn motion_lowered(runtime_names: Vec<&str>, scene_paths: Vec<&str>) -> LoweredAuthoring {
        LoweredAuthoring {
            scene: json!({
                "scene_format_version": 1,
                "artboard": {
                    "children": [
                        { "type": "shape" },
                        { "type": "rectangle" }
                    ]
                }
            }),
            source_map: AuthoringSourceMap {
                entries: vec![SourceMapEntry {
                    authored_id: "card".to_string(),
                    authored_path: "$.visual.nodes[0]".to_string(),
                    definition_path: None,
                    runtime_names: runtime_names.into_iter().map(str::to_string).collect(),
                    scene_paths: scene_paths.into_iter().map(str::to_string).collect(),
                }],
            },
        }
    }

    #[test]
    fn compiler_state_round_trips_scene_and_source_map() {
        let lowered = lowered(vec![source_entry("$.visual.nodes[0]", "card")]);
        let expected = lowered.clone();

        let actual = CompilerState::from_lowered(lowered)
            .finish()
            .expect("valid compiler state must finish");

        assert_eq!(actual, expected);
    }

    #[test]
    fn compiler_state_reports_duplicate_raw_runtime_name_at_finish() {
        let result = CompilerState::from_lowered(lowered(vec![
            source_entry("$.visual.nodes[0]", "shared"),
            source_entry("$.motion.raw_animations[0]", "shared"),
        ]))
        .finish();
        let Err(error) = result else {
            panic!("duplicate runtime names must fail compiler-state finalization");
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

    #[test]
    fn compiler_state_owns_checked_motion_target_index() {
        let input = CompilerState::from_lowered(motion_lowered(
            vec!["card", "card_geometry"],
            vec!["/artboard/children/0", "/artboard/children/1"],
        ))
        .into_motion_input();
        let Ok(targets) = input.motion_targets else {
            panic!("valid compiler bindings must produce a motion-target index");
        };
        let Ok(bindings) = targets.resolve("card", "$.motion.poses[0].targets[0].target") else {
            panic!("authored visual target must resolve from compiler state");
        };

        assert_eq!(bindings.len(), 2);
        assert_eq!(bindings[0].runtime_name, "card");
        assert_eq!(bindings[0].object_type, "shape");
        assert!(bindings[0].is_primary);
        assert_eq!(bindings[1].runtime_name, "card_geometry");
        assert_eq!(bindings[1].object_type, "rectangle");
        assert!(!bindings[1].is_primary);
    }

    #[test]
    fn compiler_state_rejects_unpaired_motion_bindings() {
        let input = CompilerState::from_lowered(motion_lowered(
            vec!["card", "card_geometry"],
            vec!["/artboard/children/0"],
        ))
        .into_motion_input();
        let Err(diagnostic) = input.motion_targets else {
            panic!("unpaired compiler bindings must fail indexing");
        };

        assert_eq!(diagnostic.code, "invalid_source_map_binding");
        assert_eq!(diagnostic.path, "$.visual.nodes[0]");
    }

    #[test]
    fn compiler_state_owns_motion_source_map_mutation() {
        let state = CompilerState::from_lowered(lowered(vec![
            source_entry("$.visual.nodes[0]", "card"),
            source_entry("$.motion.raw_animations[0]", "typed_motion"),
            source_entry("$.motion.raw_animations[1]", "raw_motion"),
        ]))
        .apply_motion_source_map(
            1,
            vec![source_entry("$.motion.easings[0]", "shared_easing")],
        );

        let actual = state
            .finish()
            .expect("compiler-owned source-map mutation must preserve valid state");
        let paths = actual
            .source_map
            .entries
            .iter()
            .map(|entry| entry.authored_path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            paths,
            vec![
                "$.visual.nodes[0]",
                "$.motion.tracks[0]",
                "$.motion.raw_animations[0]",
                "$.motion.easings[0]",
            ]
        );
    }

    #[test]
    fn compiler_state_normalizes_raw_collision_path_after_typed_prefix() {
        let result = CompilerState::from_lowered(lowered(vec![
            source_entry("$.visual.nodes[0]", "shared"),
            source_entry("$.motion.raw_animations[0]", "typed_motion"),
            source_entry("$.motion.raw_animations[1]", "shared"),
        ]))
        .apply_motion_source_map(1, Vec::new())
        .finish();
        let Err(error) = result else {
            panic!("duplicate raw runtime name must fail compiler-state finalization");
        };

        assert_eq!(error.diagnostics.len(), 1);
        assert_eq!(
            error.diagnostics[0].path,
            "$.motion.raw_animations[0].value"
        );
        assert_eq!(error.diagnostics[0].code, "runtime_name_collision");
    }

    #[test]
    fn compiler_state_registers_appended_motion_source_names() {
        let result =
            CompilerState::from_lowered(lowered(vec![source_entry("$.visual.nodes[0]", "shared")]))
                .apply_motion_source_map(0, vec![source_entry("$.motion.easings[0]", "shared")])
                .finish();
        let Err(error) = result else {
            panic!("appended motion source names must participate in collision checks");
        };

        assert_eq!(error.diagnostics.len(), 1);
        assert_eq!(error.diagnostics[0].path, "$.motion.easings[0]");
        assert_eq!(error.diagnostics[0].code, "runtime_name_collision");
    }
}
