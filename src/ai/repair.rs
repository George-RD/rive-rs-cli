use serde_json::Value;

use crate::ai::AiError;
use crate::builder::{SceneSpec, build_scene};
use crate::encoder::encode_riv;
use crate::objects::core::RiveObject;
use crate::validator::validate_riv;

const DEFAULT_MAX_RETRIES: u8 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCategory {
    Schema,
    Build,
    Validation,
    Encoding,
}

impl std::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCategory::Schema => write!(f, "schema"),
            ErrorCategory::Build => write!(f, "build"),
            ErrorCategory::Validation => write!(f, "validation"),
            ErrorCategory::Encoding => write!(f, "encoding"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepairDiagnostic {
    pub category: ErrorCategory,
    pub message: String,
    pub auto_fixable: bool,
}

#[derive(Debug, Clone)]
pub struct RepairAttempt {
    pub attempt: u8,
    pub diagnostics: Vec<RepairDiagnostic>,
    pub fixes_applied: Vec<String>,
    pub succeeded: bool,
}

#[derive(Debug)]
pub struct RepairResult {
    pub scene_json: Value,
    pub riv_bytes: Vec<u8>,
    pub attempts: Vec<RepairAttempt>,
    pub total_retries: u8,
}

pub struct RepairEngine {
    pub max_retries: u8,
}

impl Default for RepairEngine {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }
}

impl RepairEngine {
    pub fn new(max_retries: u8) -> Self {
        Self { max_retries }
    }

    pub fn repair(&self, mut json: Value) -> Result<RepairResult, AiError> {
        let mut attempts: Vec<RepairAttempt> = Vec::new();

        for attempt_num in 0..=self.max_retries {
            let mut diagnostics: Vec<RepairDiagnostic> = Vec::new();
            let mut fixes_applied: Vec<String> = Vec::new();

            if attempt_num > 0 {
                let fixes = normalize(&mut json);
                fixes_applied = fixes;
            }

            let spec: SceneSpec = match serde_json::from_value(json.clone()) {
                Ok(s) => s,
                Err(e) => {
                    diagnostics.push(RepairDiagnostic {
                        category: ErrorCategory::Schema,
                        message: e.to_string(),
                        auto_fixable: can_auto_fix_schema(&e.to_string()),
                    });
                    attempts.push(RepairAttempt {
                        attempt: attempt_num,
                        diagnostics,
                        fixes_applied,
                        succeeded: false,
                    });
                    continue;
                }
            };

            let scene = match build_scene(&spec) {
                Ok(s) => s,
                Err(e) => {
                    diagnostics.push(RepairDiagnostic {
                        category: ErrorCategory::Build,
                        message: e.clone(),
                        auto_fixable: can_auto_fix_build(&e),
                    });
                    attempts.push(RepairAttempt {
                        attempt: attempt_num,
                        diagnostics,
                        fixes_applied,
                        succeeded: false,
                    });
                    continue;
                }
            };

            let refs: Vec<&dyn RiveObject> = scene.iter().map(|o| &**o).collect();
            let riv_bytes = encode_riv(&refs, 0);

            let report = match validate_riv(&riv_bytes) {
                Ok(r) => r,
                Err(e) => {
                    diagnostics.push(RepairDiagnostic {
                        category: ErrorCategory::Encoding,
                        message: e.clone(),
                        auto_fixable: false,
                    });
                    attempts.push(RepairAttempt {
                        attempt: attempt_num,
                        diagnostics,
                        fixes_applied,
                        succeeded: false,
                    });
                    continue;
                }
            };

            if !report.valid {
                for err in &report.errors {
                    diagnostics.push(RepairDiagnostic {
                        category: ErrorCategory::Validation,
                        message: err.clone(),
                        auto_fixable: false,
                    });
                }
                attempts.push(RepairAttempt {
                    attempt: attempt_num,
                    diagnostics,
                    fixes_applied,
                    succeeded: false,
                });
                continue;
            }

            attempts.push(RepairAttempt {
                attempt: attempt_num,
                diagnostics,
                fixes_applied,
                succeeded: true,
            });

            return Ok(RepairResult {
                scene_json: json,
                riv_bytes,
                attempts,
                total_retries: attempt_num,
            });
        }

        let final_error = attempts
            .last()
            .and_then(|a| a.diagnostics.last())
            .map(|d| d.message.clone())
            .unwrap_or_else(|| "unknown error".to_string());

        Err(AiError::RepairFailed {
            attempts,
            final_error,
        })
    }
}

fn normalize(json: &mut Value) -> Vec<String> {
    let mut fixes = Vec::new();

    if let Some(fix) = inject_scene_format_version(json) {
        fixes.push(fix);
    }
    if let Some(fix) = fix_negative_dimensions(json) {
        fixes.push(fix);
    }
    fixes.extend(add_default_names(json));
    fixes.extend(deduplicate_names(json));
    fixes.extend(clamp_color_values(json));
    fixes.extend(fix_empty_children(json));

    fixes
}

fn inject_scene_format_version(json: &mut Value) -> Option<String> {
    if let Some(obj) = json.as_object_mut()
        && !obj.contains_key("scene_format_version")
    {
        obj.insert(
            "scene_format_version".to_string(),
            Value::Number(serde_json::Number::from(1)),
        );
        return Some("injected scene_format_version: 1".to_string());
    }
    None
}

fn fix_negative_dimensions(json: &mut Value) -> Option<String> {
    let mut fixed = false;

    fn walk_fix_dimensions(val: &mut Value, fixed: &mut bool) {
        if let Some(obj) = val.as_object_mut() {
            for key in ["width", "height"] {
                if let Some(v) = obj.get_mut(key)
                    && let Some(n) = v.as_f64()
                    && n < 0.0
                {
                    *v = serde_json::json!(n.abs());
                    *fixed = true;
                }
            }
            for (_, child) in obj.iter_mut() {
                walk_fix_dimensions(child, fixed);
            }
        }
        if let Some(arr) = val.as_array_mut() {
            for item in arr.iter_mut() {
                walk_fix_dimensions(item, fixed);
            }
        }
    }

    walk_fix_dimensions(json, &mut fixed);
    if fixed {
        Some("fixed negative width/height values".to_string())
    } else {
        None
    }
}

fn add_default_names(json: &mut Value) -> Vec<String> {
    let mut fixes = Vec::new();
    let mut counter: u32 = 0;

    fn walk_add_names(val: &mut Value, counter: &mut u32, fixes: &mut Vec<String>) {
        if let Some(obj) = val.as_object_mut() {
            if obj.contains_key("type") && !obj.contains_key("name") {
                let type_str = obj.get("type").and_then(|t| t.as_str()).unwrap_or("object");
                let name = format!("auto_{}_{}", type_str, counter);
                obj.insert("name".to_string(), Value::String(name.clone()));
                fixes.push(format!("added default name '{}'", name));
                *counter += 1;
            }
            for key in ["children", "artboard", "artboards"] {
                if let Some(child) = obj.get_mut(key) {
                    walk_add_names(child, counter, fixes);
                }
            }
        }
        if let Some(arr) = val.as_array_mut() {
            for item in arr.iter_mut() {
                walk_add_names(item, counter, fixes);
            }
        }
    }

    walk_add_names(json, &mut counter, &mut fixes);
    fixes
}

fn deduplicate_names(json: &mut Value) -> Vec<String> {
    let mut fixes = Vec::new();
    let mut seen: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let mut renames: Vec<(String, String)> = Vec::new();
    fn walk_dedup(
        val: &mut Value,
        seen: &mut std::collections::HashMap<String, u32>,
        fixes: &mut Vec<String>,
        renames: &mut Vec<(String, String)>,
    ) {
        if let Some(obj) = val.as_object_mut() {
            if let Some(name_val) = obj.get_mut("name")
                && let Some(name) = name_val.as_str().map(|s| s.to_string())
            {
                let count = seen.entry(name.clone()).or_insert(0);
                *count += 1;
                if *count > 1 {
                    let new_name = format!("{}_{}", name, count);
                    *name_val = Value::String(new_name.clone());
                    renames.push((name.clone(), new_name.clone()));
                    fixes.push(format!("renamed duplicate '{}' to '{}'", name, new_name));
                }
            }
            for key in ["children", "artboard", "artboards"] {
                if let Some(child) = obj.get_mut(key) {
                    walk_dedup(child, seen, fixes, renames);
                }
            }
        }
        if let Some(arr) = val.as_array_mut() {
            for item in arr.iter_mut() {
                walk_dedup(item, seen, fixes, renames);
            }
        }
    }

    walk_dedup(json, &mut seen, &mut fixes, &mut renames);

    if !renames.is_empty() {
        let rename_map: std::collections::HashMap<&str, &str> = renames
            .iter()
            .map(|(old, new)| (old.as_str(), new.as_str()))
            .collect();
        update_name_references(json, &rename_map, &mut fixes);
    }

    fixes
}

fn update_name_references(
    val: &mut Value,
    rename_map: &std::collections::HashMap<&str, &str>,
    fixes: &mut Vec<String>,
) {
    const REFERENCE_FIELDS: &[&str] = &["source_artboard_name"];
    if let Some(obj) = val.as_object_mut() {
        for field in REFERENCE_FIELDS {
            if let Some(ref_val) = obj.get_mut(*field)
                && let Some(old_ref) = ref_val.as_str().map(|s| s.to_string())
                && let Some(new_ref) = rename_map.get(old_ref.as_str())
            {
                *ref_val = Value::String((*new_ref).to_string());
                fixes.push(format!(
                    "updated reference '{}' in {} to '{}'",
                    old_ref, field, new_ref
                ));
            }
        }
        for key in ["children", "artboard", "artboards", "animations"] {
            if let Some(child) = obj.get_mut(key) {
                update_name_references(child, rename_map, fixes);
            }
        }
    }
    if let Some(arr) = val.as_array_mut() {
        for item in arr.iter_mut() {
            update_name_references(item, rename_map, fixes);
        }
    }
}

fn clamp_color_values(json: &mut Value) -> Vec<String> {
    let mut fixes = Vec::new();

    fn walk_clamp_colors(val: &mut Value, fixes: &mut Vec<String>) {
        if let Some(obj) = val.as_object_mut() {
            if let Some(color_val) = obj.get_mut("color")
                && let Some(s) = color_val.as_str().map(|s| s.to_string())
            {
                let stripped = s.strip_prefix('#').unwrap_or(&s);
                if stripped.len() != 6 && stripped.len() != 8 {
                    let padded = format!("{:0>8}", stripped);
                    let clamped: String = padded
                        .chars()
                        .take(8)
                        .map(|c| if c.is_ascii_hexdigit() { c } else { 'F' })
                        .collect();
                    *color_val = Value::String(clamped.clone());
                    fixes.push(format!("clamped color '{}' to '{}'", s, clamped));
                }
            }
            for (_, child) in obj.iter_mut() {
                walk_clamp_colors(child, fixes);
            }
        }
        if let Some(arr) = val.as_array_mut() {
            for item in arr.iter_mut() {
                walk_clamp_colors(item, fixes);
            }
        }
    }

    walk_clamp_colors(json, &mut fixes);
    fixes
}

fn fix_empty_children(json: &mut Value) -> Vec<String> {
    let mut fixes = Vec::new();

    fn walk_fix_children(val: &mut Value, fixes: &mut Vec<String>) {
        if let Some(obj) = val.as_object_mut() {
            if let Some(children) = obj.get("children")
                && children.is_null()
            {
                obj.remove("children");
                fixes.push("removed null children array".to_string());
            }
            if let Some(children) = obj.get_mut("children") {
                walk_fix_children(children, fixes);
            }
        }
        if let Some(arr) = val.as_array_mut() {
            for item in arr.iter_mut() {
                walk_fix_children(item, fixes);
            }
        }
    }

    walk_fix_children(json, &mut fixes);
    fixes
}

fn can_auto_fix_schema(msg: &str) -> bool {
    msg.contains("scene_format_version")
        || msg.contains("missing field")
        || msg.contains("invalid type")
}

fn can_auto_fix_build(msg: &str) -> bool {
    msg.contains("unknown preset") || msg.contains("duplicate") || msg.contains("missing")
}

pub fn format_repair_summary(attempts: &[RepairAttempt]) -> String {
    let mut out = String::new();
    for attempt in attempts {
        out.push_str(&format!(
            "attempt {}: {}\n",
            attempt.attempt,
            if attempt.succeeded {
                "succeeded"
            } else {
                "failed"
            }
        ));
        for fix in &attempt.fixes_applied {
            out.push_str(&format!("  fix: {}\n", fix));
        }
        for diag in &attempt.diagnostics {
            out.push_str(&format!(
                "  [{}] {} (auto-fixable: {})\n",
                diag.category, diag.message, diag.auto_fixable
            ));
        }
    }
    out
}

pub fn remediation_hints(attempts: &[RepairAttempt]) -> Vec<String> {
    let mut hints = Vec::new();
    if let Some(last) = attempts.last() {
        for diag in &last.diagnostics {
            match diag.category {
                ErrorCategory::Schema => {
                    if diag.message.contains("scene_format_version") {
                        hints.push(
                            "add \"scene_format_version\": 1 to the root of your JSON".to_string(),
                        );
                    } else if diag.message.contains("missing field") {
                        hints.push(format!("required field missing: {}", diag.message));
                    } else {
                        hints.push(format!("fix JSON schema: {}", diag.message));
                    }
                }
                ErrorCategory::Build => {
                    hints.push(format!("fix scene structure: {}", diag.message));
                }
                ErrorCategory::Validation => {
                    hints.push(format!("fix binary structure: {}", diag.message));
                }
                ErrorCategory::Encoding => {
                    hints.push(format!("encoding error: {}", diag.message));
                }
            }
        }
    }
    hints
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_scene_json() -> Value {
        serde_json::json!({
            "scene_format_version": 1,
            "artboard": {
                "name": "Test",
                "width": 500,
                "height": 500,
                "children": []
            }
        })
    }

    #[test]
    fn test_valid_json_passes_with_zero_retries() {
        let engine = RepairEngine::default();
        let result = engine.repair(valid_scene_json()).unwrap();
        assert_eq!(result.total_retries, 0);
        assert_eq!(result.attempts.len(), 1);
        assert!(result.attempts[0].succeeded);
        assert!(!result.riv_bytes.is_empty());
    }

    #[test]
    fn test_repair_missing_scene_format_version() {
        let json = serde_json::json!({
            "artboard": {
                "name": "Test",
                "width": 500,
                "height": 500,
                "children": []
            }
        });
        let engine = RepairEngine::default();
        let result = engine.repair(json).unwrap();
        assert!(result.total_retries > 0);
        assert!(result.attempts.iter().any(|a| a.succeeded));
        assert!(result.attempts.iter().any(|a| {
            a.fixes_applied
                .iter()
                .any(|f| f.contains("scene_format_version"))
        }));
    }

    #[test]
    fn test_repair_negative_dimensions() {
        let json = serde_json::json!({
            "artboard": {
                "name": "Test",
                "width": -500,
                "height": -300,
                "children": []
            }
        });
        let engine = RepairEngine::default();
        let result = engine.repair(json).unwrap();
        assert!(result.total_retries > 0);
        assert!(
            result
                .attempts
                .iter()
                .any(|a| a.fixes_applied.iter().any(|f| f.contains("negative")))
        );
    }

    #[test]
    fn test_repair_missing_names() {
        let json = serde_json::json!({
            "artboard": {
                "name": "Test",
                "width": 500,
                "height": 500,
                "children": [
                    {
                        "type": "shape",
                        "children": [
                            {
                                "type": "ellipse",
                                "width": 100.0,
                                "height": 100.0
                            }
                        ]
                    }
                ]
            }
        });
        let engine = RepairEngine::default();
        let result = engine.repair(json).unwrap();
        assert!(result.total_retries > 0);
        assert!(
            result
                .attempts
                .iter()
                .any(|a| a.fixes_applied.iter().any(|f| f.contains("auto_")))
        );
    }

    #[test]
    fn test_repair_duplicate_names() {
        let json = serde_json::json!({
            "artboard": {
                "name": "Test",
                "width": 500,
                "height": 500,
                "children": [
                    {
                        "type": "shape",
                        "name": "MyShape",
                        "children": [
                            {
                                "type": "ellipse",
                                "name": "MyShape",
                                "width": 100.0,
                                "height": 100.0
                            }
                        ]
                    }
                ]
            }
        });
        let engine = RepairEngine::default();
        let result = engine.repair(json).unwrap();
        assert!(result.scene_json.to_string().contains("MyShape_2") || result.total_retries == 0);
    }

    #[test]
    fn test_max_retries_exceeded() {
        let json = serde_json::json!({
            "completely": "invalid"
        });
        let engine = RepairEngine::new(2);
        let result = engine.repair(json);
        assert!(result.is_err());
        if let Err(AiError::RepairFailed {
            attempts,
            final_error,
        }) = result
        {
            assert_eq!(attempts.len(), 3);
            assert!(attempts.iter().all(|a| !a.succeeded));
            assert!(!final_error.is_empty());
        } else {
            panic!("expected RepairFailed error");
        }
    }

    #[test]
    fn test_zero_retries_fails_on_first_error() {
        let json = serde_json::json!({
            "artboard": {
                "name": "Test",
                "width": 500,
                "height": 500,
                "children": []
            }
        });
        let engine = RepairEngine::new(0);
        let result = engine.repair(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_inject_scene_format_version() {
        let mut json = serde_json::json!({"artboard": {}});
        let fix = inject_scene_format_version(&mut json);
        assert!(fix.is_some());
        assert_eq!(json["scene_format_version"], 1);
    }

    #[test]
    fn test_inject_scene_format_version_already_present() {
        let mut json = serde_json::json!({"scene_format_version": 1});
        let fix = inject_scene_format_version(&mut json);
        assert!(fix.is_none());
    }

    #[test]
    fn test_fix_negative_dimensions_pass() {
        let mut json = serde_json::json!({"width": -100, "height": -200});
        let fix = fix_negative_dimensions(&mut json);
        assert!(fix.is_some());
        assert_eq!(json["width"], 100.0);
        assert_eq!(json["height"], 200.0);
    }

    #[test]
    fn test_fix_negative_dimensions_no_change() {
        let mut json = serde_json::json!({"width": 100, "height": 200});
        let fix = fix_negative_dimensions(&mut json);
        assert!(fix.is_none());
    }

    #[test]
    fn test_add_default_names() {
        let mut json = serde_json::json!([
            {"type": "shape"},
            {"type": "ellipse"}
        ]);
        let fixes = add_default_names(&mut json);
        assert_eq!(fixes.len(), 2);
        assert_eq!(json[0]["name"], "auto_shape_0");
        assert_eq!(json[1]["name"], "auto_ellipse_1");
    }

    #[test]
    fn test_deduplicate_names() {
        let mut json = serde_json::json!([
            {"name": "foo"},
            {"name": "foo"},
            {"name": "foo"}
        ]);
        let fixes = deduplicate_names(&mut json);
        assert_eq!(fixes.len(), 2);
        assert_eq!(json[0]["name"], "foo");
        assert_eq!(json[1]["name"], "foo_2");
        assert_eq!(json[2]["name"], "foo_3");
    }

    #[test]
    fn test_clamp_color_short() {
        let mut json = serde_json::json!({"color": "FF"});
        let fixes = clamp_color_values(&mut json);
        assert_eq!(fixes.len(), 1);
        let color = json["color"].as_str().unwrap();
        assert_eq!(color.len(), 8);
    }

    #[test]
    fn test_clamp_color_valid_unchanged() {
        let mut json = serde_json::json!({"color": "FF0000FF"});
        let fixes = clamp_color_values(&mut json);
        assert!(fixes.is_empty());
    }

    #[test]
    fn test_fix_empty_children_null() {
        let mut json = serde_json::json!({"children": null});
        let fixes = fix_empty_children(&mut json);
        assert_eq!(fixes.len(), 1);
        assert!(json.get("children").is_none());
    }

    #[test]
    fn test_format_repair_summary() {
        let attempts = vec![RepairAttempt {
            attempt: 0,
            diagnostics: vec![RepairDiagnostic {
                category: ErrorCategory::Schema,
                message: "test error".to_string(),
                auto_fixable: true,
            }],
            fixes_applied: vec![],
            succeeded: false,
        }];
        let summary = format_repair_summary(&attempts);
        assert!(summary.contains("attempt 0: failed"));
        assert!(summary.contains("test error"));
    }

    #[test]
    fn test_remediation_hints() {
        let attempts = vec![RepairAttempt {
            attempt: 0,
            diagnostics: vec![RepairDiagnostic {
                category: ErrorCategory::Schema,
                message: "missing scene_format_version".to_string(),
                auto_fixable: true,
            }],
            fixes_applied: vec![],
            succeeded: false,
        }];
        let hints = remediation_hints(&attempts);
        assert!(!hints.is_empty());
        assert!(hints[0].contains("scene_format_version"));
    }

    #[test]
    fn test_normalize_applies_all_passes() {
        let mut json = serde_json::json!({
            "artboard": {
                "width": -100,
                "height": 200,
                "children": [
                    {"type": "shape", "children": null}
                ]
            }
        });
        let fixes = normalize(&mut json);
        assert!(fixes.iter().any(|f| f.contains("scene_format_version")));
        assert!(fixes.iter().any(|f| f.contains("negative")));
        assert!(fixes.iter().any(|f| f.contains("auto_")));
    }

    #[test]
    fn test_repair_with_shapes_and_fills() {
        let json = serde_json::json!({
            "scene_format_version": 1,
            "artboard": {
                "name": "ShapeTest",
                "width": 500,
                "height": 500,
                "children": [
                    {
                        "type": "shape",
                        "name": "Circle",
                        "children": [
                            {
                                "type": "ellipse",
                                "name": "CirclePath",
                                "width": 100.0,
                                "height": 100.0
                            },
                            {
                                "type": "fill",
                                "name": "CircleFill",
                                "children": [
                                    {
                                        "type": "solid_color",
                                        "name": "CircleColor",
                                        "color": "FFFF0000"
                                    }
                                ]
                            }
                        ]
                    }
                ]
            }
        });
        let engine = RepairEngine::default();
        let result = engine.repair(json).unwrap();
        assert_eq!(result.total_retries, 0);
        assert!(result.riv_bytes.len() > 10);
    }
}
