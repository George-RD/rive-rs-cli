mod target_index;

use std::collections::{HashMap, HashSet};

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
        let CompilerState::Draft {
            draft,
            mut runtime_names,
            motion_targets,
        } = state
        else {
            unreachable!("authoring compiler motion lowering can only run from the draft state");
        };
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
        for entry in lowered.source_map.entries.iter().skip(existing_source_entries) {
            runtime_names.register(entry);
        }
        Ok(Self {
            spec,
            state: CompilerState::Lowered {
                lowered,
                runtime_names,
            },
        })
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
