use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::authoring::{AuthoringSourceMap, SourceMapEntry};

use super::model::{
    AnimatedSemanticCheck, InteractiveSemanticCheck, RuntimeEvidence, SemanticCheckEvidence,
    SemanticEvidence, SemanticExpectations, StaticSemanticCheck,
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

fn frame_difference(
    case_dir: &Path,
    from: u32,
    to: u32,
    check_prefix: &str,
) -> SemanticCheckEvidence {
    let from_path = case_dir.join("render").join(format!("frame_{from:05}.png"));
    let to_path = case_dir.join("render").join(format!("frame_{to:05}.png"));
    let result = fs::read(&from_path)
        .and_then(|from_bytes| fs::read(&to_path).map(|to_bytes| from_bytes != to_bytes));
    match result {
        Ok(passed) => SemanticCheckEvidence {
            check: format!("{check_prefix}:{from}:{to}"),
            passed,
            detail: if passed {
                format!("rendered frames {from} and {to} differ")
            } else {
                format!("rendered frames {from} and {to} are identical")
            },
        },
        Err(error) => SemanticCheckEvidence {
            check: format!("{check_prefix}:{from}:{to}"),
            passed: false,
            detail: format!("could not compare rendered frames {from} and {to}: {error}"),
        },
    }
}

fn animated_check(case_dir: &Path, check: &AnimatedSemanticCheck) -> SemanticCheckEvidence {
    match check {
        AnimatedSemanticCheck::FramesDiffer { from, to } => {
            frame_difference(case_dir, *from, *to, "frames_differ")
        }
    }
}

fn source_entry<'a>(
    source_map: &'a AuthoringSourceMap,
    authored_id: &str,
) -> Option<&'a SourceMapEntry> {
    source_map
        .entries
        .iter()
        .find(|entry| entry.authored_id == authored_id)
}

fn runtime_name<'a>(source_map: &'a AuthoringSourceMap, authored_id: &str) -> Option<&'a str> {
    source_entry(source_map, authored_id).and_then(|entry| match entry.runtime_names.as_slice() {
        [name] => Some(name.as_str()),
        _ => None,
    })
}

fn scene_value<'a>(scene: &'a Value, entry: &SourceMapEntry) -> Option<&'a Value> {
    entry
        .scene_paths
        .iter()
        .find_map(|path| scene.pointer(path))
}

fn scene_index(source_map: &AuthoringSourceMap, authored_id: &str) -> Option<u64> {
    source_entry(source_map, authored_id)?
        .scene_paths
        .first()?
        .rsplit('/')
        .next()?
        .parse()
        .ok()
}

fn interactive_input_applied(
    source_map: &AuthoringSourceMap,
    runtime: &RuntimeEvidence,
    authored_id: &str,
    value: bool,
    frame: u32,
) -> SemanticCheckEvidence {
    let runtime_name = runtime_name(source_map, authored_id);
    let passed = runtime_name.is_some_and(|name| {
        runtime.applied_inputs.iter().any(|input| {
            input.get("name").and_then(Value::as_str) == Some(name)
                && input.get("value").and_then(Value::as_bool) == Some(value)
                && input.get("frame").and_then(Value::as_u64) == Some(u64::from(frame))
        })
    });
    SemanticCheckEvidence {
        check: format!("interactive_input_applied:{authored_id}:{frame}"),
        passed,
        detail: if passed {
            format!("authored input '{authored_id}' applied value {value} at frame {frame}")
        } else {
            format!(
                "authored input '{authored_id}' did not apply value {value} at frame {frame}; observed {:?}",
                runtime.applied_inputs
            )
        },
    }
}

fn interactive_pointer_applied(
    runtime: &RuntimeEvidence,
    event: &str,
    x: f64,
    y: f64,
    frame: u32,
) -> SemanticCheckEvidence {
    let passed = runtime.applied_pointers.iter().any(|pointer| {
        pointer.get("event").and_then(Value::as_str) == Some(event)
            && pointer
                .get("x")
                .and_then(Value::as_f64)
                .is_some_and(|observed| observed == x)
            && pointer
                .get("y")
                .and_then(Value::as_f64)
                .is_some_and(|observed| observed == y)
            && pointer.get("frame").and_then(Value::as_u64) == Some(u64::from(frame))
    });
    SemanticCheckEvidence {
        check: format!("interactive_pointer_applied:{event}:{frame}"),
        passed,
        detail: if passed {
            format!("pointer {event} at {x},{y} applied at frame {frame}")
        } else {
            format!(
                "pointer {event} at {x},{y} did not apply at frame {frame}; observed {:?}",
                runtime.applied_pointers
            )
        },
    }
}

fn interactive_state_motion_binding(
    scene: &Value,
    source_map: &AuthoringSourceMap,
    statechart_id: &str,
    state_id: &str,
    motion_id: &str,
) -> SemanticCheckEvidence {
    let authored_state = format!("{statechart_id}/{state_id}");
    let expected_motion = runtime_name(source_map, motion_id);
    let observed =
        source_entry(source_map, &authored_state).and_then(|entry| scene_value(scene, entry));
    let passed = expected_motion.is_some_and(|expected| {
        observed
            .and_then(|state| state.get("animation"))
            .and_then(Value::as_str)
            == Some(expected)
    });
    SemanticCheckEvidence {
        check: format!("interactive_state_motion:{authored_state}:{motion_id}"),
        passed,
        detail: if passed {
            format!("state '{authored_state}' binds authored motion '{motion_id}'")
        } else {
            format!(
                "state '{authored_state}' does not bind authored motion '{motion_id}'; observed {observed:?}"
            )
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn interactive_transition(
    scene: &Value,
    source_map: &AuthoringSourceMap,
    statechart_id: &str,
    transition_id: &str,
    from_state: &str,
    to_state: &str,
    input_id: &str,
    equals: bool,
) -> SemanticCheckEvidence {
    let authored_transition = format!("{statechart_id}/{transition_id}");
    let authored_from = format!("{statechart_id}/{from_state}");
    let authored_to = format!("{statechart_id}/{to_state}");
    let authored_input = format!("{statechart_id}/{input_id}");
    let expected_from = scene_index(source_map, &authored_from);
    let expected_to = scene_index(source_map, &authored_to);
    let expected_input = runtime_name(source_map, &authored_input);
    let observed =
        source_entry(source_map, &authored_transition).and_then(|entry| scene_value(scene, entry));
    let passed = match (observed, expected_from, expected_to, expected_input) {
        (Some(transition), Some(from), Some(to), Some(input)) => {
            transition.get("from").and_then(Value::as_u64) == Some(from)
                && transition.get("to").and_then(Value::as_u64) == Some(to)
                && transition
                    .pointer("/conditions/0/input")
                    .and_then(Value::as_str)
                    == Some(input)
                && transition
                    .pointer("/conditions/0/value")
                    .and_then(Value::as_bool)
                    == Some(equals)
        }
        _ => false,
    };
    SemanticCheckEvidence {
        check: format!("interactive_transition:{authored_transition}"),
        passed,
        detail: if passed {
            format!(
                "transition '{authored_transition}' routes {from_state} -> {to_state} when {input_id} == {equals}"
            )
        } else {
            format!(
                "transition '{authored_transition}' does not match {from_state} -> {to_state} when {input_id} == {equals}; observed {observed:?}"
            )
        },
    }
}

fn interactive_check(
    case_dir: &Path,
    scene: &Value,
    source_map: &AuthoringSourceMap,
    runtime: &RuntimeEvidence,
    check: &InteractiveSemanticCheck,
) -> SemanticCheckEvidence {
    match check {
        InteractiveSemanticCheck::InputApplied {
            authored_id,
            value,
            frame,
        } => interactive_input_applied(source_map, runtime, authored_id, *value, *frame),
        InteractiveSemanticCheck::PointerApplied { event, x, y, frame } => {
            interactive_pointer_applied(runtime, event, *x, *y, *frame)
        }
        InteractiveSemanticCheck::StateMotionBinding {
            statechart_id,
            state_id,
            motion_id,
        } => {
            interactive_state_motion_binding(scene, source_map, statechart_id, state_id, motion_id)
        }
        InteractiveSemanticCheck::Transition {
            statechart_id,
            transition_id,
            from_state,
            to_state,
            input_id,
            equals,
        } => interactive_transition(
            scene,
            source_map,
            statechart_id,
            transition_id,
            from_state,
            to_state,
            input_id,
            *equals,
        ),
        InteractiveSemanticCheck::FramesDiffer { from, to } => {
            frame_difference(case_dir, *from, *to, "interactive_frames_differ")
        }
    }
}

pub fn evaluate_semantics(
    case_dir: &Path,
    scene: &Value,
    source_map: &AuthoringSourceMap,
    expectations: &SemanticExpectations,
    runtime: Option<&RuntimeEvidence>,
) -> SemanticEvidence {
    let static_checks = expectations
        .static_checks
        .iter()
        .map(|check| static_check(scene, source_map, check))
        .collect::<Vec<_>>();
    let runtime = runtime.filter(|evidence| evidence.passed);
    let animated_checks = if runtime.is_some() {
        expectations
            .animated_checks
            .iter()
            .map(|check| animated_check(case_dir, check))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let interactive_checks = if let Some(runtime) = runtime {
        expectations
            .interactive_checks
            .iter()
            .map(|check| interactive_check(case_dir, scene, source_map, runtime, check))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let static_passed =
        (!static_checks.is_empty()).then(|| static_checks.iter().all(|evidence| evidence.passed));
    let animated_passed = (!animated_checks.is_empty())
        .then(|| animated_checks.iter().all(|evidence| evidence.passed));
    let interactive_passed = (!interactive_checks.is_empty())
        .then(|| interactive_checks.iter().all(|evidence| evidence.passed));
    let failures = static_checks
        .iter()
        .chain(animated_checks.iter())
        .chain(interactive_checks.iter())
        .filter(|evidence| !evidence.passed)
        .map(|evidence| evidence.detail.clone())
        .collect::<Vec<_>>();

    SemanticEvidence {
        static_passed,
        animated_passed,
        interactive_passed,
        static_checks,
        animated_checks,
        interactive_checks,
        failure_reason: (!failures.is_empty()).then(|| failures.join("; ")),
    }
}
