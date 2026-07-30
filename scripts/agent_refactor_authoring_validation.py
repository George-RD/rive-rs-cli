from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected one match in {path}, found {count}")
    file.write_text(text.replace(old, new))


frontend_path = Path("src/authoring/frontend.rs")
frontend = frontend_path.read_text()
if "use super::validation::validate_numeric_values;" in frontend:
    raise SystemExit("authoring validation refactor already applied")

frontend = frontend.replace("use super::expression::validate_scene_number;\n", "", 1)
frontend = frontend.replace(
    "use super::lower;\n",
    "use super::lower;\nuse super::validation::validate_numeric_values;\n",
    1,
)
frontend = frontend.replace(
    "    AuthoringSpec, BehaviorSection, LoweredAuthoring, MotionSection, PaintSpec, Quantity,\n"
    "    RawSceneFragment, ScalarExpr, TransformSpec, Unit, VisualNode, VisualSection,\n",
    "    AuthoringSpec, BehaviorSection, LoweredAuthoring, MotionSection, Quantity, RawSceneFragment,\n"
    "    TransformSpec, Unit, VisualNode, VisualSection,\n",
    1,
)

validation_start = frontend.index("fn validate_numeric_values")
validation_end = frontend.index("fn rewrite_error_paths")
validation_block = frontend[validation_start:validation_end]
validation_block = validation_block.replace(
    "fn validate_numeric_values",
    "pub(super) fn validate_numeric_values",
    1,
)
frontend = frontend[:validation_start] + frontend[validation_end:]
frontend_path.write_text(frontend)

validation_header = '''use std::collections::BTreeMap;

use super::expression::validate_scene_number;
use super::spec::{
    AuthoringDiagnostic, AuthoringSpec, PaintSpec, Quantity, ScalarExpr, TransformSpec, VisualNode,
};

'''
Path("src/authoring/validation.rs").write_text(validation_header + validation_block)

replace_once(
    "src/authoring/mod.rs",
    "mod spec;\n",
    "mod spec;\nmod validation;\n",
)

lower_path = Path("src/authoring/lower.rs")
lower = lower_path.read_text()
lower = lower.replace(
    "    AuthoringSpec, ComponentSpec, GradientKind, LoweredAuthoring, PaintSpec, Quantity,\n"
    "    ShapeNodeRef, SourceMapEntry, TrimPathMode, TrimPathSpec, Unit, VisualNode,\n",
    "    AuthoringSpec, ComponentSpec, GradientKind, LoweredAuthoring, PaintSpec, Quantity,\n"
    "    ScalarExpr, ShapeNodeRef, SourceMapEntry, TrimPathMode, TrimPathSpec, Unit, VisualNode,\n",
    1,
)
lower = lower.replace(
    '''struct LoweredPaint {
    object: Value,
    runtime_names: Vec<String>,
    scene_paths: Vec<String>,
}

struct LoweredTrimPath {
    object: Value,
    runtime_name: String,
    scene_path: String,
}
''',
    '''struct LoweredObject {
    object: Value,
    runtime_names: Vec<String>,
    scene_paths: Vec<String>,
}
''',
    1,
)
lower = lower.replace("LoweredPaint", "LoweredObject")
lower = lower.replace(
    '''                let LoweredTrimPath {
                    object,
                    runtime_name,
                    scene_path,
                } = self.lower_trim_path(
''',
    '''                let LoweredObject {
                    object,
                    runtime_names: trim_runtime_names,
                    scene_paths: trim_scene_paths,
                } = self.lower_trim_path(
''',
    1,
)
lower = lower.replace(
    '''                runtime_names.push(runtime_name);
                scene_paths.push(scene_path);
                stroke_children.push(object);
''',
    '''                runtime_names.extend(trim_runtime_names);
                scene_paths.extend(trim_scene_paths);
                stroke_children.push(object);
''',
    1,
)
lower = lower.replace(
    ") -> Result<LoweredTrimPath, AuthoringDiagnostic> {",
    ") -> Result<LoweredObject, AuthoringDiagnostic> {",
    1,
)
lower = lower.replace(
    '''        Ok(LoweredTrimPath {
            object: json!({
                "type": "trim_path",
                "name": runtime_name.clone(),
                "start": start,
                "end": end,
                "offset": offset,
                "mode": mode
            }),
            runtime_name,
            scene_path: scene_path.to_string(),
        })
''',
    '''        Ok(LoweredObject {
            object: json!({
                "type": "trim_path",
                "name": runtime_name.clone(),
                "start": start,
                "end": end,
                "offset": offset,
                "mode": mode
            }),
            runtime_names: vec![runtime_name],
            scene_paths: vec![scene_path.to_string()],
        })
''',
    1,
)
lower = lower.replace(
    '''        let inner_radius = inner_radius_expression
            .map(|expression| {
                evaluate_expression(
                    expression,
                    &format!("{authored_path}.inner_radius"),
                    scope,
                    Unit::Scalar,
                )
            })
            .transpose()?;
        if inner_radius.is_some_and(|ratio| !(0.0..=1.0).contains(&ratio)) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.inner_radius"),
                "invalid_ratio",
                "star inner radius must be between zero and one",
            ));
        }
''',
    '''        let inner_radius = inner_radius_expression
            .map(|expression| {
                let path = format!("{authored_path}.inner_radius");
                evaluate_ratio_expression(
                    expression,
                    &path,
                    scope,
                    "star inner radius must be between zero and one",
                )
            })
            .transpose()?;
''',
    1,
)
lower = lower.replace(
    '''        let start = evaluate_expression(
            &trim.start,
            &format!("{authored_path}.start"),
            scope,
            Unit::Scalar,
        )?;
        if !(0.0..=1.0).contains(&start) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.start"),
                "invalid_ratio",
                "trim start must be between zero and one",
            ));
        }

        let end = evaluate_expression(
            &trim.end,
            &format!("{authored_path}.end"),
            scope,
            Unit::Scalar,
        )?;
        if !(0.0..=1.0).contains(&end) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.end"),
                "invalid_ratio",
                "trim end must be between zero and one",
            ));
        }
''',
    '''        let start_path = format!("{authored_path}.start");
        let start = evaluate_ratio_expression(
            &trim.start,
            &start_path,
            scope,
            "trim start must be between zero and one",
        )?;
        let end_path = format!("{authored_path}.end");
        let end = evaluate_ratio_expression(
            &trim.end,
            &end_path,
            scope,
            "trim end must be between zero and one",
        )?;
''',
    1,
)
lower = lower.replace(
    '''                    let position =
                        evaluate_expression(&stop.position, &stop_path, scope, Unit::Scalar)?;
                    if !(0.0..=1.0).contains(&position) {
                        return Err(AuthoringDiagnostic::new(
                            stop_path,
                            "invalid_ratio",
                            "gradient stop positions must be between zero and one",
                        ));
                    }
''',
    '''                    let position = evaluate_ratio_expression(
                        &stop.position,
                        &stop_path,
                        scope,
                        "gradient stop positions must be between zero and one",
                    )?;
''',
    1,
)
lower = lower.replace(
    "fn validate_id(id: &str, path: &str, diagnostics: &mut Vec<AuthoringDiagnostic>) {\n",
    '''fn evaluate_ratio_expression(
    expression: &ScalarExpr,
    path: &str,
    scope: &BTreeMap<String, Quantity>,
    message: &str,
) -> Result<f64, AuthoringDiagnostic> {
    let value = evaluate_expression(expression, path, scope, Unit::Scalar)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(AuthoringDiagnostic::new(path, "invalid_ratio", message));
    }
    Ok(value)
}

fn validate_id(id: &str, path: &str, diagnostics: &mut Vec<AuthoringDiagnostic>) {
''',
    1,
)
lower_path.write_text(lower)

replace_once(
    "cairn.blueprint",
    '"./tests/authoring_stroke_contract.rs", "./tests/authoring_validation_contract.rs", "./tests/support"]',
    '"./tests/authoring_stroke_contract.rs", "./tests/authoring_trim_path_contract.rs", "./tests/authoring_validation_contract.rs", "./tests/support"]',
)
