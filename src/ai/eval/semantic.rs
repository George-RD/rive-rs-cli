use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::authoring::AuthoringSourceMap;

use super::model::{
    AnimatedSemanticCheck, SemanticCheckEvidence, SemanticEvidence, SemanticExpectations,
    StaticSemanticCheck,
};

fn collect_named_types(value: &Value, out: &mut Vec<(String, String)>) {
    match value {
        Value::Array(items) => {
            for item in items {
                collect_named_types(item, out);
            }
        }
        Value::Object(object) => {
            if let (Some(name), Some(object_type)) = (
                object.get("name").and_then(Value::as_str),
                object.get("type").and_then(Value::as_str),
            ) {
                out.push((name.to_string(), object_type.to_string()));
            }
            for child in object.values() {
                collect_named_types(child, out);
            }
        }
        _ => {}
    }
}

fn static_check(
    scene: &Value,
    source_map: &AuthoringSourceMap,
    check: &StaticSemanticCheck,
) -> SemanticCheckEvidence {
    match check {
        StaticSemanticCheck::AuthoredIdPresent { authored_id } => {
            let passed = source_map
                .entries
                .iter()
                .any(|entry| entry.authored_id == *authored_id);
            SemanticCheckEvidence {
                check: format!("authored_id_present:{authored_id}"),
                passed,
                detail: if passed {
                    format!("source map contains authored id '{authored_id}'")
                } else {
                    format!("source map is missing authored id '{authored_id}'")
                },
            }
        }
        StaticSemanticCheck::AuthoredIdHasRuntimeType {
            authored_id,
            object_type,
        } => {
            let runtime_names = source_map
                .entries
                .iter()
                .find(|entry| entry.authored_id == *authored_id)
                .map(|entry| entry.runtime_names.iter().cloned().collect::<HashSet<_>>())
                .unwrap_or_default();
            let mut named_types = Vec::new();
            collect_named_types(scene, &mut named_types);
            let passed = named_types.iter().any(|(name, actual_type)| {
                runtime_names.contains(name) && actual_type == object_type
            });
            SemanticCheckEvidence {
                check: format!("authored_id_runtime_type:{authored_id}:{object_type}"),
                passed,
                detail: if passed {
                    format!("authored id '{authored_id}' maps to runtime type '{object_type}'")
                } else {
                    format!(
                        "authored id '{authored_id}' does not map to runtime type '{object_type}'"
                    )
                },
            }
        }
    }
}

fn animated_check(case_dir: &Path, check: &AnimatedSemanticCheck) -> SemanticCheckEvidence {
    match check {
        AnimatedSemanticCheck::FramesDiffer { from, to } => {
            let from_path = case_dir.join("render").join(format!("frame_{from:05}.png"));
            let to_path = case_dir.join("render").join(format!("frame_{to:05}.png"));
            let result = fs::read(&from_path)
                .and_then(|from_bytes| fs::read(&to_path).map(|to_bytes| from_bytes != to_bytes));
            match result {
                Ok(passed) => SemanticCheckEvidence {
                    check: format!("frames_differ:{from}:{to}"),
                    passed,
                    detail: if passed {
                        format!("rendered frames {from} and {to} differ")
                    } else {
                        format!("rendered frames {from} and {to} are identical")
                    },
                },
                Err(error) => SemanticCheckEvidence {
                    check: format!("frames_differ:{from}:{to}"),
                    passed: false,
                    detail: format!("could not compare rendered frames {from} and {to}: {error}"),
                },
            }
        }
    }
}

pub fn evaluate_semantics(
    case_dir: &Path,
    scene: &Value,
    source_map: &AuthoringSourceMap,
    expectations: &SemanticExpectations,
) -> SemanticEvidence {
    let static_checks = expectations
        .static_checks
        .iter()
        .map(|check| static_check(scene, source_map, check))
        .collect::<Vec<_>>();
    let animated_checks = expectations
        .animated_checks
        .iter()
        .map(|check| animated_check(case_dir, check))
        .collect::<Vec<_>>();

    let static_passed =
        (!static_checks.is_empty()).then(|| static_checks.iter().all(|evidence| evidence.passed));
    let animated_passed = (!animated_checks.is_empty())
        .then(|| animated_checks.iter().all(|evidence| evidence.passed));
    let failures = static_checks
        .iter()
        .chain(animated_checks.iter())
        .filter(|evidence| !evidence.passed)
        .map(|evidence| evidence.detail.clone())
        .collect::<Vec<_>>();

    SemanticEvidence {
        static_passed,
        animated_passed,
        static_checks,
        animated_checks,
        failure_reason: (!failures.is_empty()).then(|| failures.join("; ")),
    }
}
