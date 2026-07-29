use std::collections::BTreeMap;

use super::spec::{AuthoringDiagnostic, Quantity, ScalarExpr, TransformSpec, Unit};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Evaluated {
    pub value: f64,
    pub unit: Unit,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TransformValues {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
}

pub(crate) fn evaluate_quantity(
    quantity: Quantity,
    path: &str,
    expected: Unit,
) -> Result<f64, AuthoringDiagnostic> {
    validate_scene_number(quantity.value, &format!("{path}.value"))?;
    let evaluated = canonicalize(Evaluated {
        value: quantity.value,
        unit: quantity.unit,
    });
    expect_unit(evaluated, path, expected)
}

pub(crate) fn evaluate_expression(
    expression: &ScalarExpr,
    path: &str,
    scope: &BTreeMap<String, Quantity>,
    expected: Unit,
) -> Result<f64, AuthoringDiagnostic> {
    expect_unit(evaluate(expression, path, scope)?, path, expected)
}

pub(crate) fn evaluate_transform(
    transform: &TransformSpec,
    path: &str,
    scope: &BTreeMap<String, Quantity>,
) -> Result<TransformValues, AuthoringDiagnostic> {
    let x = transform.x.as_ref().map_or(Ok(0.0), |value| {
        evaluate_expression(value, &format!("{path}.x"), scope, Unit::Px)
    })?;
    let y = transform.y.as_ref().map_or(Ok(0.0), |value| {
        evaluate_expression(value, &format!("{path}.y"), scope, Unit::Px)
    })?;
    let rotation = transform.rotation.as_ref().map_or(Ok(0.0), |value| {
        evaluate_expression(value, &format!("{path}.rotation"), scope, Unit::Radians)
    })?;
    let scale_x = transform.scale_x.as_ref().map_or(Ok(1.0), |value| {
        evaluate_expression(value, &format!("{path}.scale_x"), scope, Unit::Scalar)
    })?;
    let scale_y = transform.scale_y.as_ref().map_or(Ok(1.0), |value| {
        evaluate_expression(value, &format!("{path}.scale_y"), scope, Unit::Scalar)
    })?;

    Ok(TransformValues {
        x,
        y,
        rotation,
        scale_x,
        scale_y,
    })
}

fn evaluate(
    expression: &ScalarExpr,
    path: &str,
    scope: &BTreeMap<String, Quantity>,
) -> Result<Evaluated, AuthoringDiagnostic> {
    match expression {
        ScalarExpr::Literal { value, unit } => {
            validate_scene_number(*value, &format!("{path}.value"))?;
            Ok(canonicalize(Evaluated {
                value: *value,
                unit: *unit,
            }))
        }
        ScalarExpr::Parameter { name } => {
            let quantity = scope.get(name).ok_or_else(|| {
                AuthoringDiagnostic::new(
                    format!("{path}.name"),
                    "unknown_parameter",
                    format!("parameter '{name}' is not defined in this scope"),
                )
            })?;
            validate_scene_number(quantity.value, path)?;
            Ok(canonicalize(Evaluated {
                value: quantity.value,
                unit: quantity.unit,
            }))
        }
        ScalarExpr::Add { left, right } => {
            evaluate_binary(left, right, path, scope, |left, right| left + right)
        }
        ScalarExpr::Subtract { left, right } => {
            evaluate_binary(left, right, path, scope, |left, right| left - right)
        }
        ScalarExpr::Multiply { value, factor } => {
            validate_scene_number(*factor, &format!("{path}.factor"))?;
            let evaluated = evaluate(value, &format!("{path}.value"), scope)?;
            let result = evaluated.value * factor;
            validate_scene_number(result, path)?;
            Ok(Evaluated {
                value: result,
                unit: evaluated.unit,
            })
        }
        ScalarExpr::Divide { value, divisor } => {
            validate_scene_number(*divisor, &format!("{path}.divisor"))?;
            if *divisor == 0.0 {
                return Err(AuthoringDiagnostic::new(
                    format!("{path}.divisor"),
                    "division_by_zero",
                    "expression divisor must not be zero",
                ));
            }
            let evaluated = evaluate(value, &format!("{path}.value"), scope)?;
            let result = evaluated.value / divisor;
            validate_scene_number(result, path)?;
            Ok(Evaluated {
                value: result,
                unit: evaluated.unit,
            })
        }
    }
}

fn evaluate_binary(
    left: &ScalarExpr,
    right: &ScalarExpr,
    path: &str,
    scope: &BTreeMap<String, Quantity>,
    operation: impl FnOnce(f64, f64) -> f64,
) -> Result<Evaluated, AuthoringDiagnostic> {
    let left = evaluate(left, &format!("{path}.left"), scope)?;
    let right_path = format!("{path}.right");
    let right = evaluate(right, &right_path, scope)?;
    if left.unit != right.unit {
        return Err(AuthoringDiagnostic::new(
            right_path,
            "unit_mismatch",
            format!(
                "cannot combine {:?} with {:?}; operands must have compatible units",
                left.unit, right.unit
            ),
        ));
    }
    let value = operation(left.value, right.value);
    validate_scene_number(value, path)?;
    Ok(Evaluated {
        value,
        unit: left.unit,
    })
}

fn canonicalize(evaluated: Evaluated) -> Evaluated {
    match evaluated.unit {
        Unit::Degrees => Evaluated {
            value: evaluated.value.to_radians(),
            unit: Unit::Radians,
        },
        _ => evaluated,
    }
}

fn expect_unit(
    evaluated: Evaluated,
    path: &str,
    expected: Unit,
) -> Result<f64, AuthoringDiagnostic> {
    let expected = canonicalize(Evaluated {
        value: 0.0,
        unit: expected,
    })
    .unit;
    if evaluated.unit != expected {
        return Err(AuthoringDiagnostic::new(
            path,
            "unit_mismatch",
            format!("expected {:?}, found {:?}", expected, evaluated.unit),
        ));
    }
    Ok(evaluated.value)
}

pub(crate) fn validate_scene_number(value: f64, path: &str) -> Result<(), AuthoringDiagnostic> {
    if !value.is_finite() {
        return Err(AuthoringDiagnostic::new(
            path,
            "non_finite",
            "numeric values must be finite",
        ));
    }
    if value.abs() > f64::from(f32::MAX) {
        return Err(AuthoringDiagnostic::new(
            path,
            "numeric_out_of_range",
            "numeric values must fit the canonical f32 scene representation",
        ));
    }
    Ok(())
}
