use std::collections::BTreeMap;

use serde_json::json;

use super::super::expression::evaluate_expression;
use super::super::spec::{
    AuthoringDiagnostic, GradientKind, PaintSpec, Quantity, TrimPathMode, TrimPathSpec, Unit,
};
use super::{LoweredObject, Lowerer, PaintTarget, evaluate_ratio_expression, runtime_name};

impl<'a> Lowerer<'a> {
    pub(super) fn lower_trim_path(
        &self,
        trim: &TrimPathSpec,
        authored_path: &str,
        runtime_segments: &[String],
        scene_path: &str,
        scope: &BTreeMap<String, Quantity>,
    ) -> Result<LoweredObject, AuthoringDiagnostic> {
        let start_path = format!("{authored_path}.start");
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

        let offset = trim
            .offset
            .as_ref()
            .map(|expression| {
                evaluate_expression(
                    expression,
                    &format!("{authored_path}.offset"),
                    scope,
                    Unit::Scalar,
                )
            })
            .transpose()?
            .unwrap_or(0.0);
        let mode = match trim.mode {
            TrimPathMode::Sequential => "sequential",
            TrimPathMode::Synchronized => "synchronized",
        };
        let runtime_name = runtime_name(runtime_segments, "stroke_trim");

        Ok(LoweredObject {
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
    }

    pub(super) fn lower_paint(
        &self,
        paint: &PaintSpec,
        authored_path: &str,
        runtime_segments: &[String],
        scene_path: &str,
        scope: &BTreeMap<String, Quantity>,
        target: PaintTarget,
    ) -> Result<LoweredObject, AuthoringDiagnostic> {
        match paint {
            PaintSpec::Solid(color) => {
                let color_name = target.runtime_name(runtime_segments, "color");
                Ok(LoweredObject {
                    object: json!({
                        "type": "solid_color",
                        "name": color_name.clone(),
                        "color": color
                    }),
                    runtime_names: vec![color_name],
                    scene_paths: vec![scene_path.to_string()],
                })
            }
            PaintSpec::Gradient(gradient) => {
                if gradient.stops.len() < 2 {
                    return Err(AuthoringDiagnostic::new(
                        format!("{authored_path}.stops"),
                        "invalid_gradient_stops",
                        "gradient paints require at least two stops",
                    ));
                }

                let start_x = evaluate_expression(
                    &gradient.start_x,
                    &format!("{authored_path}.start_x"),
                    scope,
                    Unit::Px,
                )?;
                let start_y = evaluate_expression(
                    &gradient.start_y,
                    &format!("{authored_path}.start_y"),
                    scope,
                    Unit::Px,
                )?;
                let end_x = evaluate_expression(
                    &gradient.end_x,
                    &format!("{authored_path}.end_x"),
                    scope,
                    Unit::Px,
                )?;
                let end_y = evaluate_expression(
                    &gradient.end_y,
                    &format!("{authored_path}.end_y"),
                    scope,
                    Unit::Px,
                )?;

                let gradient_name = target.runtime_name(runtime_segments, "gradient");
                let mut runtime_names = vec![gradient_name.clone()];
                let mut scene_paths = vec![scene_path.to_string()];
                let mut children = Vec::with_capacity(gradient.stops.len());
                let mut previous_position = None;
                for (index, stop) in gradient.stops.iter().enumerate() {
                    let stop_path = format!("{authored_path}.stops[{index}].position");
                    let position = evaluate_ratio_expression(
                        &stop.position,
                        &stop_path,
                        scope,
                        "gradient stop positions must be between zero and one",
                    )?;
                    if previous_position.is_some_and(|previous| position < previous) {
                        return Err(AuthoringDiagnostic::new(
                            stop_path,
                            "invalid_gradient_stop_order",
                            "gradient stop positions must be in non-decreasing order",
                        ));
                    }
                    previous_position = Some(position);

                    let stop_name =
                        target.runtime_name(runtime_segments, &format!("gradient_stop_{index}"));
                    runtime_names.push(stop_name.clone());
                    scene_paths.push(format!("{scene_path}/children/{index}"));
                    children.push(json!({
                        "type": "gradient_stop",
                        "name": stop_name,
                        "color": stop.color.as_str(),
                        "position": position
                    }));
                }

                let gradient_type = match gradient.kind {
                    GradientKind::LinearGradient => "linear_gradient",
                    GradientKind::RadialGradient => "radial_gradient",
                };
                Ok(LoweredObject {
                    object: json!({
                        "type": gradient_type,
                        "name": gradient_name,
                        "start_x": start_x,
                        "start_y": start_y,
                        "end_x": end_x,
                        "end_y": end_y,
                        "children": children
                    }),
                    runtime_names,
                    scene_paths,
                })
            }
        }
    }
}
