use std::collections::BTreeMap;

use super::super::super::expression::evaluate_expression;
use super::super::super::spec::{AuthoringDiagnostic, Quantity, ScalarExpr, Unit};

const FRAME_ROUNDING_ULPS: f64 = 8.0;
const HALF_FRAME: f64 = 0.5;
const MAX_FRAME_ROUNDING_WINDOW: f64 = 1e-9;
const WHOLE_FRAME: f64 = 1.0;

pub(super) fn evaluate_frame_value(
    expression: &ScalarExpr,
    path: &str,
    scope: &BTreeMap<String, Quantity>,
    code: &str,
    message: &str,
) -> Result<u64, AuthoringDiagnostic> {
    let value = evaluate_expression(expression, path, scope, Unit::Scalar)?;
    let rounded = value.round();
    if !value.is_finite() || rounded < 0.0 || rounded >= u64::MAX as f64 {
        return Err(AuthoringDiagnostic::new(path, code, message));
    }
    let Some(rounding_tolerance) = frame_rounding_tolerance(value) else {
        return Err(AuthoringDiagnostic::new(path, code, message));
    };
    if (value - rounded).abs() > rounding_tolerance {
        return Err(AuthoringDiagnostic::new(path, code, message));
    }
    Ok(rounded as u64)
}

fn frame_rounding_tolerance(value: f64) -> Option<f64> {
    let magnitude = value.abs().max(1.0);
    let one_ulp = f64::from_bits(magnitude.to_bits() + 1) - magnitude;
    if one_ulp >= WHOLE_FRAME {
        return None;
    }
    if one_ulp >= HALF_FRAME {
        return Some(0.0);
    }
    Some(
        (one_ulp * FRAME_ROUNDING_ULPS)
            .min(MAX_FRAME_ROUNDING_WINDOW)
            .max(one_ulp),
    )
}
