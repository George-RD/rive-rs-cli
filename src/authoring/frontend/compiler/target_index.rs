use std::collections::hash_map::Entry;

use serde_json::Value;

use super::{IndexedMotionTarget, MotionTargetBinding, MotionTargetIndex};
use crate::authoring::spec::{AuthoringDiagnostic, AuthoringSourceMap, SourceMapEntry};

#[derive(Clone, Copy, Debug)]
struct RuntimeBinding<'a> {
    runtime_name: &'a str,
    scene_path: &'a str,
}

impl MotionTargetIndex {
    pub(super) fn from_output(
        scene: &Value,
        source_map: &AuthoringSourceMap,
    ) -> Result<Self, AuthoringDiagnostic> {
        let mut targets = std::collections::HashMap::new();
        for entry in source_map
            .entries
            .iter()
            .filter(|entry| entry.authored_path.starts_with("$.visual.nodes["))
        {
            let mut bindings = Vec::new();
            for (binding_index, binding) in
                checked_runtime_bindings(entry)?.into_iter().enumerate()
            {
                let object_type = scene
                    .pointer(binding.scene_path)
                    .and_then(|object| object.get("type"))
                    .and_then(Value::as_str)
                    .ok_or_else(|| invalid_runtime_binding(entry, binding))?;
                bindings.push(MotionTargetBinding {
                    runtime_name: binding.runtime_name.to_string(),
                    object_type: object_type.to_string(),
                    is_primary: binding_index == 0,
                });
            }
            let indexed = IndexedMotionTarget::Unique(bindings);
            match targets.entry(entry.authored_id.clone()) {
                Entry::Vacant(slot) => {
                    slot.insert(indexed);
                }
                Entry::Occupied(mut slot) => {
                    slot.insert(IndexedMotionTarget::Ambiguous);
                }
            }
        }
        Ok(Self { targets })
    }
}

fn checked_runtime_bindings(
    entry: &SourceMapEntry,
) -> Result<Vec<RuntimeBinding<'_>>, AuthoringDiagnostic> {
    if entry.runtime_names.is_empty() {
        if entry.scene_paths.len() <= 1 {
            return Ok(Vec::new());
        }
        return Err(invalid_binding_cardinality(entry));
    }
    if entry.runtime_names.len() != entry.scene_paths.len() {
        return Err(invalid_binding_cardinality(entry));
    }
    Ok(entry
        .runtime_names
        .iter()
        .zip(&entry.scene_paths)
        .map(|(runtime_name, scene_path)| RuntimeBinding {
            runtime_name,
            scene_path,
        })
        .collect())
}

fn invalid_binding_cardinality(entry: &SourceMapEntry) -> AuthoringDiagnostic {
    AuthoringDiagnostic::new(
        &entry.authored_path,
        "invalid_source_map_binding",
        format!(
            "source-map entry '{}' has {} runtime names and {} scene paths; named runtime objects must be paired one-to-one",
            entry.authored_id,
            entry.runtime_names.len(),
            entry.scene_paths.len()
        ),
    )
}

fn invalid_runtime_binding(
    entry: &SourceMapEntry,
    binding: RuntimeBinding<'_>,
) -> AuthoringDiagnostic {
    AuthoringDiagnostic::new(
        &entry.authored_path,
        "invalid_source_map_binding",
        format!(
            "source-map runtime object '{}' for authored id '{}' does not resolve to a typed scene object at '{}'",
            binding.runtime_name, entry.authored_id, binding.scene_path
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_entry(runtime_names: Vec<&str>, scene_paths: Vec<&str>) -> SourceMapEntry {
        SourceMapEntry {
            authored_id: "card".to_string(),
            authored_path: "$.visual.nodes[0]".to_string(),
            definition_path: None,
            runtime_names: runtime_names.into_iter().map(str::to_string).collect(),
            scene_paths: scene_paths.into_iter().map(str::to_string).collect(),
        }
    }

    #[test]
    fn unnamed_raw_entry_may_keep_its_root_scene_path() {
        let entry = source_entry(Vec::new(), vec!["/artboard/children/0"]);

        assert!(
            checked_runtime_bindings(&entry)
                .expect("root-only raw entry remains valid")
                .is_empty()
        );
    }
}
