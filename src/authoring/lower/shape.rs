use serde_json::{Value, json};

use super::super::expression::{evaluate_expression, evaluate_transform};
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry, Unit};
use super::super::visual::ShapeNodeRef;
use super::{
    LoweredObject, Lowerer, NodeContext, PaintTarget, evaluate_ratio_expression, runtime_name,
};

impl<'a> Lowerer<'a> {
    pub(super) fn lower_shape(
        &mut self,
        shape: ShapeNodeRef<'_>,
        context: NodeContext<'_>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let ShapeNodeRef {
            geometry_type,
            width: width_expression,
            height: height_expression,
            points,
            corner_radius: corner_radius_expression,
            inner_radius: inner_radius_expression,
            fill,
            stroke,
            transform,
        } = shape;
        let NodeContext {
            authored_path,
            definition_path,
            authored_id,
            runtime_segments,
            scene_path,
            scope,
        } = context;

        let width = evaluate_expression(
            width_expression,
            &format!("{authored_path}.width"),
            scope,
            Unit::Px,
        )?;
        let height = evaluate_expression(
            height_expression,
            &format!("{authored_path}.height"),
            scope,
            Unit::Px,
        )?;
        if width <= 0.0 {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.width"),
                "invalid_dimension",
                "shape width must be greater than zero",
            ));
        }
        if height <= 0.0 {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.height"),
                "invalid_dimension",
                "shape height must be greater than zero",
            ));
        }
        if points.is_some_and(|points| points < 3) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.points"),
                "invalid_points",
                "polygon and star point counts must be at least three",
            ));
        }
        let corner_radius = corner_radius_expression
            .map(|expression| {
                evaluate_expression(
                    expression,
                    &format!("{authored_path}.corner_radius"),
                    scope,
                    Unit::Px,
                )
            })
            .transpose()?;
        if corner_radius.is_some_and(|radius| radius < 0.0) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.corner_radius"),
                "invalid_dimension",
                "corner radius must not be negative",
            ));
        }
        let inner_radius = inner_radius_expression
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
        let stroke_thickness = stroke
            .map(|stroke| {
                evaluate_expression(
                    &stroke.width,
                    &format!("{authored_path}.stroke.width"),
                    scope,
                    Unit::Px,
                )
            })
            .transpose()?;
        if stroke_thickness.is_some_and(|thickness| thickness <= 0.0) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.stroke.width"),
                "invalid_dimension",
                "stroke width must be greater than zero",
            ));
        }
        let transform_values =
            evaluate_transform(transform, &format!("{authored_path}.transform"), scope)?;

        let shape_name = runtime_name(&runtime_segments, "shape");
        let geometry_name = runtime_name(&runtime_segments, "geometry");
        let fill_name = runtime_name(&runtime_segments, "fill");
        let LoweredObject {
            object: fill_paint,
            runtime_names: fill_runtime_names,
            scene_paths: fill_scene_paths,
        } = self.lower_paint(
            fill,
            &format!("{authored_path}.fill"),
            &runtime_segments,
            &format!("{scene_path}/children/1/children/0"),
            scope,
            PaintTarget::Fill,
        )?;
        let mut runtime_names = vec![shape_name.clone(), geometry_name.clone(), fill_name.clone()];
        runtime_names.extend(fill_runtime_names);
        let mut scene_paths = vec![
            scene_path.clone(),
            format!("{scene_path}/children/0"),
            format!("{scene_path}/children/1"),
        ];
        scene_paths.extend(fill_scene_paths);

        let mut geometry = json!({
            "type": geometry_type,
            "name": geometry_name,
            "width": width,
            "height": height,
            "origin_x": 0.5,
            "origin_y": 0.5
        });
        if let Some(object) = geometry.as_object_mut() {
            if matches!(geometry_type, "rectangle" | "polygon" | "star") {
                object.insert(
                    "corner_radius".to_string(),
                    corner_radius.map_or(Value::Null, Value::from),
                );
            }
            if let Some(points) = points {
                object.insert("points".to_string(), Value::from(points));
            }
            if let Some(inner_radius) = inner_radius {
                object.insert("inner_radius".to_string(), Value::from(inner_radius));
            }
        }

        let mut children = vec![
            geometry,
            json!({
                "type": "fill",
                "name": fill_name,
                "children": [fill_paint]
            }),
        ];
        if let (Some(stroke), Some(thickness)) = (stroke, stroke_thickness) {
            let stroke_name = runtime_name(&runtime_segments, "stroke");
            let LoweredObject {
                object: stroke_paint,
                runtime_names: stroke_runtime_names,
                scene_paths: stroke_scene_paths,
            } = self.lower_paint(
                &stroke.paint,
                &format!("{authored_path}.stroke.paint"),
                &runtime_segments,
                &format!("{scene_path}/children/2/children/0"),
                scope,
                PaintTarget::Stroke,
            )?;
            runtime_names.push(stroke_name.clone());
            runtime_names.extend(stroke_runtime_names);
            scene_paths.push(format!("{scene_path}/children/2"));
            scene_paths.extend(stroke_scene_paths);

            let mut stroke_children = vec![stroke_paint];
            if let Some(trim) = &stroke.trim {
                let LoweredObject {
                    object,
                    runtime_names: trim_runtime_names,
                    scene_paths: trim_scene_paths,
                } = self.lower_trim_path(
                    trim,
                    &format!("{authored_path}.stroke.trim"),
                    &runtime_segments,
                    &format!("{scene_path}/children/2/children/1"),
                    scope,
                )?;
                runtime_names.extend(trim_runtime_names);
                scene_paths.extend(trim_scene_paths);
                stroke_children.push(object);
            }

            children.push(json!({
                "type": "stroke",
                "name": stroke_name,
                "thickness": thickness,
                "children": stroke_children
            }));
        }

        self.register_runtime_names(&runtime_names, &format!("{authored_path}.id"))?;
        self.source_map.entries.push(SourceMapEntry {
            authored_id,
            authored_path,
            definition_path,
            runtime_names,
            scene_paths,
        });

        Ok(json!({
            "type": "shape",
            "name": shape_name,
            "x": transform_values.x,
            "y": transform_values.y,
            "rotation": transform_values.rotation,
            "scale_x": transform_values.scale_x,
            "scale_y": transform_values.scale_y,
            "children": children
        }))
    }
}
