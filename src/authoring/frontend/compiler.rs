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
                } = motion::lower_motion(spec, motion_targets)
                    .map_err(|error| rewrite_error_paths(spec, error))?;
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
                } = motion::lower_motion(spec, motion_targets)
                    .map_err(|error| rewrite_error_paths(spec, error))?;
                debug_assert!(animations.is_empty());
                debug_assert!(source_entries.is_empty());
                debug_assert!(easing_source_entries.is_empty());
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
        let state_machine_index_base = lowered.scene["artboard"]["state_machines"]
            .as_array()
            .map_or(0, Vec::len);
        let behavior::BehaviorLoweringOutput {
            state_machines,
            source_entries,
        } = behavior::lower_behavior(spec, state_machine_index_base)
            .map_err(|error| rewrite_error_paths(spec, error))?;

        if state_machines.is_empty() {
            debug_assert!(source_entries.is_empty());
            return Ok((lowered, runtime_names));
        }

        let artboard = lowered.scene["artboard"]
            .as_object_mut()
            .expect("canonical AuthoringSpec lowering always produces an artboard object");
        let state_machine_value = artboard
            .entry("state_machines")
            .or_insert_with(|| Value::Array(Vec::new()));
        let state_machine_list = state_machine_value
            .as_array_mut()
            .expect("canonical state_machines value must remain an array");
        state_machine_list.extend(state_machines);

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
