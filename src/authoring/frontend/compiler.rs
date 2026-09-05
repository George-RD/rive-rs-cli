mod behavior;
mod target_index;

use std::collections::{HashMap, HashSet};

use serde_json::Value;

use super::super::lower;
use super::super::spec::{
    AuthoringDiagnostic, AuthoringError, AuthoringSourceMap, AuthoringSpec, LoweredAuthoring,
    SourceMapEntry,
};
use super::{motion, rewrite_error_paths};

pub(super) struct AuthoringCompiler<'a> {
    spec: &'a AuthoringSpec,
    state: CompilerState<'a>,
}

enum CompilerState<'a> {
    Draft {
        draft: lower::PartialLowering<'a>,
        runtime_names: RuntimeNameRegistry,
        motion_targets: Result<MotionTargetIndex, AuthoringDiagnostic>,
    },
    PendingMotionValidation {
        lowered: LoweredAuthoring,
        runtime_names: RuntimeNameRegistry,
        motion_targets: Result<MotionTargetIndex, AuthoringDiagnostic>,
    },
    Lowered {
        lowered: LoweredAuthoring,
        runtime_names: RuntimeNameRegistry,
    },
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
        let draft = lower::lower_visual(spec).map_err(|error| rewrite_error_paths(spec, error))?;
        if spec.motion.tracks.is_empty() {
            let lowered = draft
                .finish(Vec::new(), Vec::new(), Vec::new())
                .map_err(|error| rewrite_error_paths(spec, error))?;
            let runtime_names = RuntimeNameRegistry::from_source_map(&lowered.source_map);
            let motion_targets =
                MotionTargetIndex::from_output(&lowered.scene, &lowered.source_map);
            return Ok(Self {
                spec,
                state: CompilerState::PendingMotionValidation {
                    lowered,
                    runtime_names,
                    motion_targets,
                },
            });
        }

        draft
            .validate_provisional_scene()
            .map_err(|error| rewrite_error_paths(spec, error))?;
        let runtime_names = RuntimeNameRegistry::from_source_map(draft.source_map());
        let provisional_scene = draft.provisional_scene();
        let motion_targets = MotionTargetIndex::from_output(&provisional_scene, draft.source_map());
        Ok(Self {
            spec,
            state: CompilerState::Draft {
                draft,
                runtime_names,
                motion_targets,
            },
        })
    }

    pub(super) fn lower_motion(self) -> Result<Self, AuthoringError> {
        let Self { spec, state } = self;
        match state {
            CompilerState::Draft {
                draft,
                mut runtime_names,
                motion_targets,
            } => {
                let existing_source_entries = draft.source_map().entries.len();
                let motion::MotionLoweringOutput {
                    animations,
                    source_entries,
                    easing_source_entries,
                    warnings,
                } = motion::lower_motion(spec, motion_targets)
                    .map_err(|error| rewrite_error_paths(spec, error))?;
                let mut draft = draft;
                draft.record_warnings(warnings);
                let lowered = draft
                    .finish(animations, source_entries, easing_source_entries)
                    .map_err(|error| rewrite_error_paths(spec, error))?;
                for entry in lowered
                    .source_map
                    .entries
                    .iter()
                    .skip(existing_source_entries)
                {
                    runtime_names.register(entry);
                }
                let (lowered, runtime_names) = Self::lower_behavior(spec, lowered, runtime_names)?;
                Ok(Self {
                    spec,
                    state: CompilerState::Lowered {
                        lowered,
                        runtime_names,
                    },
                })
            }
            CompilerState::PendingMotionValidation {
                lowered,
                runtime_names,
                motion_targets,
            } => {
                let motion::MotionLoweringOutput {
                    animations,
                    source_entries,
                    easing_source_entries,
                    warnings,
                } = motion::lower_motion(spec, motion_targets)
                    .map_err(|error| rewrite_error_paths(spec, error))?;
                debug_assert!(animations.is_empty());
                debug_assert!(source_entries.is_empty());
                debug_assert!(easing_source_entries.is_empty());
                debug_assert!(warnings.is_empty());
                let (lowered, runtime_names) = Self::lower_behavior(spec, lowered, runtime_names)?;
                Ok(Self {
                    spec,
                    state: CompilerState::Lowered {
                        lowered,
                        runtime_names,
                    },
                })
            }
            CompilerState::Lowered { .. } => {
                unreachable!("authoring compiler motion lowering can only run once")
            }
        }
    }

    fn lower_behavior(
        spec: &AuthoringSpec,
        mut lowered: LoweredAuthoring,
        mut runtime_names: RuntimeNameRegistry,
    ) -> Result<(LoweredAuthoring, RuntimeNameRegistry), AuthoringError> {
        let child_index_base = lowered.scene["artboard"]["children"]
            .as_array()
            .map_or(0, Vec::len);
        let listener_targets = MotionTargetIndex::from_output(&lowered.scene, &lowered.source_map)
            .map_err(AuthoringError::one)
            .map_err(|error| rewrite_error_paths(spec, error))?;
        let behavior::BehaviorLoweringOutput {
            artboard_children,
            state_machines,
            source_entries,
        } = behavior::lower_behavior(spec, child_index_base, 0, listener_targets)
            .map_err(|error| rewrite_error_paths(spec, error))?;
        validate_behavior_source_identities(&source_entries)
            .map_err(|error| rewrite_error_paths(spec, error))?;

        if artboard_children.is_empty() && state_machines.is_empty() {
            debug_assert!(source_entries.is_empty());
            return Ok((lowered, runtime_names));
        }

        let typed_state_machine_count = state_machines.len();
        if typed_state_machine_count > 0 {
            for entry in &mut lowered.source_map.entries {
                if !entry
                    .authored_path
                    .starts_with("$.behavior.raw_state_machines[")
                {
                    continue;
                }
                for scene_path in &mut entry.scene_paths {
                    if let Some(index) = scene_path
                        .strip_prefix("/artboard/state_machines/")
                        .and_then(|index| index.parse::<usize>().ok())
                    {
                        *scene_path = format!(
                            "/artboard/state_machines/{}",
                            index + typed_state_machine_count
                        );
                    }
                }
            }
        }

        let artboard = lowered.scene["artboard"]
            .as_object_mut()
            .expect("canonical AuthoringSpec lowering always produces an artboard object");
        if !artboard_children.is_empty() {
            let children = artboard
                .entry("children")
                .or_insert_with(|| Value::Array(Vec::new()))
                .as_array_mut()
                .expect("canonical children value must remain an array");
            children.extend(artboard_children);
        }

        if !state_machines.is_empty() {
            let existing_raw = artboard
                .remove("state_machines")
                .and_then(|value| value.as_array().cloned())
                .unwrap_or_default();
            let mut ordered = state_machines;
            ordered.extend(existing_raw);
            artboard.insert("state_machines".to_string(), Value::Array(ordered));
        }

        for entry in &source_entries {
            runtime_names.register(entry);
        }
        lowered.source_map.entries.extend(source_entries);
        behavior::validate_lowered_scene(&lowered.scene)
            .map_err(|error| rewrite_error_paths(spec, error))?;

        Ok((lowered, runtime_names))
    }

    pub(super) fn finish(self) -> Result<LoweredAuthoring, AuthoringError> {
        let CompilerState::Lowered {
            lowered,
            runtime_names,
        } = self.state
        else {
            unreachable!("authoring compiler must lower motion before finalization");
        };
        debug_assert_eq!(
            runtime_names.binding_count(),
            lowered
                .source_map
                .entries
                .iter()
                .map(|entry| entry.runtime_names.len())
                .sum::<usize>()
        );
        runtime_names.validate()?;
        Ok(lowered)
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

fn validate_behavior_source_identities(entries: &[SourceMapEntry]) -> Result<(), AuthoringError> {
    let mut first_paths: HashMap<&str, &str> = HashMap::new();
    let mut diagnostics = Vec::new();
    for entry in entries {
        if let Some(first_path) = first_paths.get(entry.authored_id.as_str()) {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{}.id", entry.authored_path),
                "behavior_source_id_collision",
                format!(
                    "behavior source identity '{}' is also claimed by '{}'",
                    entry.authored_id, first_path
                ),
            ));
        } else {
            first_paths.insert(entry.authored_id.as_str(), entry.authored_path.as_str());
        }
    }
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(AuthoringError::many(diagnostics))
    }
}
