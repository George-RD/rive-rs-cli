from pathlib import Path


def replace_once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected one match, found {count}")
    return text.replace(old, new, 1)


repair = Path("src/ai/repair.rs")
text = repair.read_text()
text = replace_once(
    text,
    "use serde::Serialize;",
    "use std::path::Path;\n\nuse serde::Serialize;",
    "repair Path import",
)
text = replace_once(
    text,
    "    pub fn repair(&self, mut json: Value, file_id: u64) -> Result<RepairResult, AiError> {\n        let mut attempts: Vec<RepairAttempt> = Vec::new();",
    """    pub fn repair(&self, json: Value, file_id: u64) -> Result<RepairResult, AiError> {
        self.repair_with_base_dir(json, file_id, None)
    }

    pub fn repair_with_base_dir(
        &self,
        mut json: Value,
        file_id: u64,
        base_dir: Option<&Path>,
    ) -> Result<RepairResult, AiError> {
        let mut attempts: Vec<RepairAttempt> = Vec::new();""",
    "repair base-dir entry point",
)
text = replace_once(
    text,
    "let scene = match build_scene(&spec, None) {",
    "let scene = match build_scene(&spec, base_dir) {",
    "repair build base dir",
)
repair.write_text(text)

runner = Path("src/ai/eval/runner.rs")
text = runner.read_text()
text = replace_once(
    text,
    "use std::path::Path;",
    "use std::path::{Path, PathBuf};",
    "runner PathBuf import",
)
text = replace_once(
    text,
    ") -> Result<(Value, Option<AuthoringSourceMap>, bool), CaseRunError> {",
    ") -> Result<(Value, Option<AuthoringSourceMap>, bool, Option<PathBuf>), CaseRunError> {",
    "runner resolved scene tuple",
)
fixture_start = text.index("        let input = fs::read_to_string(&fixture_path)")
fixture_end = text.index('        fs::write(case_dir.join("authoring-spec.json")', fixture_start)
text = text[:fixture_start] + """        let input = fs::read_to_string(&fixture_path).map_err(|error| {
            CaseRunError::structural(format!(
                "failed to read AuthoringSpec fixture {}: {}",
                fixture_path.display(),
                error
            ))
        })?;
""" + text[fixture_end:]
return_start = text.index("        return Ok((", fixture_start)
return_end = text.index("        ));", return_start) + len("        ));")
text = text[:return_start] + """        return Ok((
            lowered.scene,
            Some(lowered.source_map),
            lowering_reproducible,
            fixture_path.parent().map(Path::to_path_buf),
        ));""" + text[return_end:]
text = replace_once(
    text,
    "    Ok((generated_scene, None, true))",
    "    Ok((generated_scene, None, true, None))",
    "provider tuple",
)
text = replace_once(
    text,
    "    let (generated_scene, source_map, lowering_reproducible) =",
    "    let (generated_scene, source_map, lowering_reproducible, source_base_dir) =",
    "runner tuple destructure",
)
text = replace_once(
    text,
    ".repair(generated_scene.clone(), file_id)",
    ".repair_with_base_dir(generated_scene.clone(), file_id, source_base_dir.as_deref())",
    "primary repair base dir",
)
text = replace_once(
    text,
    "let repeat = engine.repair(generated_scene, file_id).map_err(|error| {",
    """let repeat = engine
        .repair_with_base_dir(generated_scene, file_id, source_base_dir.as_deref())
        .map_err(|error| {""",
    "repeat repair base dir",
)
text = replace_once(
    text,
    "            evaluate_semantics(case_dir, &repaired.scene_json, source_map, expectations)",
    """            evaluate_semantics(
                case_dir,
                &repaired.scene_json,
                source_map,
                expectations,
                runtime.as_ref().is_some_and(|evidence| evidence.passed),
            )""",
    "semantic runtime gating",
)
runner.write_text(text)

semantic = Path("src/ai/eval/semantic.rs")
text = semantic.read_text()
text = replace_once(
    text,
    "    expectations: &SemanticExpectations,\n) -> SemanticEvidence {",
    "    expectations: &SemanticExpectations,\n    animated_runtime_available: bool,\n) -> SemanticEvidence {",
    "semantic runtime parameter",
)
animated_start = text.index("    let animated_checks = expectations")
animated_end = text.index(";", animated_start) + 1
text = text[:animated_start] + """    let animated_checks = if animated_runtime_available {
        expectations
            .animated_checks
            .iter()
            .map(|check| animated_check(case_dir, check))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };""" + text[animated_end:]
semantic.write_text(text)

tests = Path("src/ai/eval/mod.rs")
text = tests.read_text()
static_start = text.index("    fn semantic_static_checks_use_authored_identity_and_runtime_type()")
static_call = text.index("        let evidence = evaluate_semantics(", static_start)
static_close = text.index("        );", static_call)
text = text[:static_close] + "            false,\n" + text[static_close:]
text = replace_once(
    text,
    "            evaluate_semantics(&root, &serde_json::json!({}), &source_map, &expectations);",
    """            evaluate_semantics(
                &root,
                &serde_json::json!({}),
                &source_map,
                &expectations,
                true,
            );""",
    "animated semantic test argument",
)
insert_at = text.index("    #[test]\n    fn hash_bytes_is_deterministic()")
regression = """    #[test]
    fn semantic_animated_checks_skip_without_runtime_evidence() {
        let source_map = AuthoringSourceMap::default();
        let expectations = SemanticExpectations {
            static_checks: Vec::new(),
            animated_checks: vec![AnimatedSemanticCheck::FramesDiffer { from: 0, to: 30 }],
        };
        let evidence = evaluate_semantics(
            std::path::Path::new("unused"),
            &serde_json::json!({}),
            &source_map,
            &expectations,
            false,
        );
        assert_eq!(evidence.animated_passed, None);
        assert!(evidence.animated_checks.is_empty());
        assert_eq!(evidence.failure_reason, None);
    }

"""
text = text[:insert_at] + regression + text[insert_at:]
tests.write_text(text)
