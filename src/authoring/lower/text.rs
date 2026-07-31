use serde_json::{Value, json};

use super::super::expression::{evaluate_expression, evaluate_transform};
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry, TextAlign, TextOverflow, Unit};
use super::super::visual::TextNodeRef;
use super::{
    LoweredObject, Lowerer, NodeContext, PaintTarget, evaluate_ratio_expression,
    font_asset_runtime_name, runtime_name,
};

impl<'a> Lowerer<'a> {
    pub(super) fn lower_text(
        &mut self,
        text: TextNodeRef<'_>,
        context: NodeContext<'_>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let TextNodeRef {
            content,
            font,
            font_size: font_size_expression,
            fill,
            width: width_expression,
            height: height_expression,
            line_height: line_height_expression,
            letter_spacing: letter_spacing_expression,
            paragraph_spacing: paragraph_spacing_expression,
            origin_x: origin_x_expression,
            origin_y: origin_y_expression,
            align,
            overflow,
            transform,
        } = text;
        let NodeContext {
            authored_path,
            definition_path,
            authored_id,
            runtime_segments,
            scene_path,
            scope,
        } = context;

        let font_asset = font
            .map(|id| {
                if !self.spec.font_assets.contains_key(id) {
                    return Err(AuthoringDiagnostic::new(
                        format!("{authored_path}.font"),
                        "unknown_font_asset",
                        format!("font asset '{id}' is not declared"),
                    ));
                }
                Ok(font_asset_runtime_name(&self.spec.artboard.id, id))
            })
            .transpose()?;

        let font_size = evaluate_expression(
            font_size_expression,
            &format!("{authored_path}.font_size"),
            scope,
            Unit::Px,
        )?;
        if font_size <= 0.0 {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.font_size"),
                "invalid_dimension",
                "text font size must be greater than zero",
            ));
        }

        let width = width_expression
            .map(|expression| {
                evaluate_expression(
                    expression,
                    &format!("{authored_path}.width"),
                    scope,
                    Unit::Px,
                )
            })
            .transpose()?;
        if width.is_some_and(|value| value <= 0.0) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.width"),
                "invalid_dimension",
                "text width must be greater than zero",
            ));
        }

        let height = height_expression
            .map(|expression| {
                evaluate_expression(
                    expression,
                    &format!("{authored_path}.height"),
                    scope,
                    Unit::Px,
                )
            })
            .transpose()?;
        if height.is_some_and(|value| value <= 0.0) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.height"),
                "invalid_dimension",
                "text height must be greater than zero",
            ));
        }
        if height.is_some() && width.is_none() {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.height"),
                "invalid_text_layout",
                "text height requires a width so fixed sizing is unambiguous",
            ));
        }
        let sizing_value = match (width, height) {
            (None, None) => 0,
            (Some(_), None) => 1,
            (Some(_), Some(_)) => 2,
            (None, Some(_)) => unreachable!("height without width is rejected above"),
        };

        let line_height = line_height_expression
            .map(|expression| {
                evaluate_expression(
                    expression,
                    &format!("{authored_path}.line_height"),
                    scope,
                    Unit::Scalar,
                )
            })
            .transpose()?;
        if line_height.is_some_and(|value| value <= 0.0) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.line_height"),
                "invalid_dimension",
                "text line height must be greater than zero",
            ));
        }

        let letter_spacing = letter_spacing_expression
            .map(|expression| {
                evaluate_expression(
                    expression,
                    &format!("{authored_path}.letter_spacing"),
                    scope,
                    Unit::Px,
                )
            })
            .transpose()?;
        let paragraph_spacing = paragraph_spacing_expression
            .map(|expression| {
                evaluate_expression(
                    expression,
                    &format!("{authored_path}.paragraph_spacing"),
                    scope,
                    Unit::Px,
                )
            })
            .transpose()?;
        let origin_x = origin_x_expression
            .map(|expression| {
                let path = format!("{authored_path}.origin_x");
                evaluate_ratio_expression(
                    expression,
                    &path,
                    scope,
                    "text origin must be between zero and one",
                )
            })
            .transpose()?;
        let origin_y = origin_y_expression
            .map(|expression| {
                let path = format!("{authored_path}.origin_y");
                evaluate_ratio_expression(
                    expression,
                    &path,
                    scope,
                    "text origin must be between zero and one",
                )
            })
            .transpose()?;
        let transform_values =
            evaluate_transform(transform, &format!("{authored_path}.transform"), scope)?;

        let align_value = match align {
            TextAlign::Left => 0,
            TextAlign::Right => 1,
            TextAlign::Center => 2,
        };
        let overflow_value = match overflow {
            TextOverflow::Visible => 0,
            TextOverflow::Hidden => 1,
            TextOverflow::Clipped => 2,
            TextOverflow::Ellipsis => 3,
            TextOverflow::Fit => 4,
            TextOverflow::FitFontSize => 5,
        };

        let anchor_name = runtime_name(&runtime_segments, "text_anchor");
        let text_name = runtime_name(&runtime_segments, "text");
        let style_name = runtime_name(&runtime_segments, "text_style");
        let fill_name = runtime_name(&runtime_segments, "text_fill");
        let run_name = runtime_name(&runtime_segments, "text_run");
        let text_scene_path = format!("{scene_path}/children/0");
        let style_scene_path = format!("{text_scene_path}/children/0");
        let fill_scene_path = format!("{style_scene_path}/children/0");
        let paint_scene_path = format!("{fill_scene_path}/children/0");
        let run_scene_path = format!("{text_scene_path}/children/1");
        let LoweredObject {
            object: paint,
            runtime_names: paint_runtime_names,
            scene_paths: paint_scene_paths,
        } = self.lower_paint(
            fill,
            &format!("{authored_path}.fill"),
            &runtime_segments,
            &paint_scene_path,
            scope,
            PaintTarget::Text,
        )?;

        let mut runtime_names = vec![
            anchor_name.clone(),
            text_name.clone(),
            style_name.clone(),
            fill_name.clone(),
        ];
        runtime_names.extend(paint_runtime_names);
        runtime_names.push(run_name.clone());
        let mut scene_paths = vec![
            scene_path.clone(),
            text_scene_path,
            style_scene_path,
            fill_scene_path,
        ];
        scene_paths.extend(paint_scene_paths);
        scene_paths.push(run_scene_path);

        let mut style = json!({
            "type": "text_style",
            "name": style_name.clone(),
            "font_size": font_size,
            "children": [{
                "type": "fill",
                "name": fill_name,
                "children": [paint]
            }]
        });
        if let Some(object) = style.as_object_mut() {
            if let Some(font_asset) = font_asset {
                object.insert("font_asset".to_string(), Value::String(font_asset));
            }
            if let Some(line_height) = line_height {
                object.insert("line_height".to_string(), Value::from(line_height));
            }
            if let Some(letter_spacing) = letter_spacing {
                object.insert("letter_spacing".to_string(), Value::from(letter_spacing));
            }
        }

        let run = json!({
            "type": "text_value_run",
            "name": run_name,
            "text": content,
            "style": style_name
        });
        let mut text_object = json!({
            "type": "text",
            "name": text_name,
            "align_value": align_value,
            "sizing_value": sizing_value,
            "overflow_value": overflow_value,
            "children": [style, run]
        });
        if let Some(object) = text_object.as_object_mut() {
            if let Some(width) = width {
                object.insert("width".to_string(), Value::from(width));
            }
            if let Some(height) = height {
                object.insert("height".to_string(), Value::from(height));
            }
            if let Some(origin_x) = origin_x {
                object.insert("origin_x".to_string(), Value::from(origin_x));
            }
            if let Some(origin_y) = origin_y {
                object.insert("origin_y".to_string(), Value::from(origin_y));
            }
            if let Some(paragraph_spacing) = paragraph_spacing {
                object.insert(
                    "paragraph_spacing".to_string(),
                    Value::from(paragraph_spacing),
                );
            }
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
            "type": "node",
            "name": anchor_name,
            "x": transform_values.x,
            "y": transform_values.y,
            "rotation": transform_values.rotation,
            "scale_x": transform_values.scale_x,
            "scale_y": transform_values.scale_y,
            "children": [text_object]
        }))
    }
}
