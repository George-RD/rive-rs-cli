from pathlib import Path
import re
p = Path('src/authoring/frontend/compiler/behavior.rs')
s = p.read_text()
if 'fn evaluate_fixed_blend_weight(' not in s:
    s = s.replace('SourceMapEntry, Unit,', 'ScalarExpr, SourceMapEntry, Unit,')
    s = s.replace('const DIRECT_BLEND_SOURCE_FIXED: u64 = 1;', 'const DIRECT_BLEND_SOURCE_FIXED: u64 = 1;\nconst MAX_DIRECT_BLEND_WEIGHT: f64 = 100.0;')
    pattern = r'evaluate_expression\(\s*weight,\s*(&format!\([^\n]+\)),\s*(&spec\.parameters|parameters),\s*Unit::Scalar,?\s*\)'
    s, count = re.subn(pattern, r'evaluate_fixed_blend_weight(weight, \1, \2)', s)
    assert count == 2, count
    anchor = 'fn validate_state_motion('
    assert s.count(anchor) == 1
    helper = '''fn evaluate_fixed_blend_weight(
    weight: &ScalarExpr,
    path: &str,
    parameters: &BTreeMap<String, Quantity>,
) -> Result<f64, AuthoringDiagnostic> {
    let value = evaluate_expression(weight, path, parameters, Unit::Scalar)?;
    if !(0.0..=MAX_DIRECT_BLEND_WEIGHT).contains(&value) {
        return Err(AuthoringDiagnostic::new(
            path,
            "invalid_blend_weight",
            format!("a fixed blend weight must be between 0 and {MAX_DIRECT_BLEND_WEIGHT} percent"),
        ));
    }
    Ok(value)
}

'''
    p.write_text(s.replace(anchor, helper + anchor))
