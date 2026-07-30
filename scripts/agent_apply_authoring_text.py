from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected one match in {path}, found {count}")
    file.write_text(text.replace(old, new))


spec_path = Path("src/authoring/spec.rs")
if "pub enum TextAlign" in spec_path.read_text():
    raise SystemExit(0)

replace_once(
    "src/authoring/spec.rs",
    '''#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ComponentSpec {
''',
    '''#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    #[default]
    Left,
    Right,
    Center,
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum TextOverflow {
    #[default]
    Visible,
    Hidden,
    Clipped,
    Ellipsis,
    Fit,
    FitFontSize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ComponentSpec {
''',
)

replace_once(
    "src/authoring/spec.rs",
    '''    Group {
        id: String,
''',
    '''    Text {
        id: String,
        text: String,
        font_size: ScalarExpr,
        fill: PaintSpec,
        #[serde(default)]
        width: Option<ScalarExpr>,
        #[serde(default)]
        height: Option<ScalarExpr>,
        #[serde(default)]
        line_height: Option<ScalarExpr>,
        #[serde(default)]
        letter_spacing: Option<ScalarExpr>,
        #[serde(default)]
        paragraph_spacing: Option<ScalarExpr>,
        #[serde(default)]
        origin_x: Option<ScalarExpr>,
        #[serde(default)]
        origin_y: Option<ScalarExpr>,
        #[serde(default)]
        align: TextAlign,
        #[serde(default)]
        overflow: TextOverflow,
        #[serde(default)]
        transform: TransformSpec,
    },
    Group {
        id: String,
''',
)

replace_once(
    "src/authoring/spec.rs",
    '''pub(crate) struct ShapeNodeRef<'a> {
    pub geometry_type: &'static str,
    pub width: &'a ScalarExpr,
    pub height: &'a ScalarExpr,
    pub points: Option<u64>,
    pub corner_radius: Option<&'a ScalarExpr>,
    pub inner_radius: Option<&'a ScalarExpr>,
    pub fill: &'a PaintSpec,
    pub stroke: Option<&'a StrokeSpec>,
    pub transform: &'a TransformSpec,
}
''',
    '''pub(crate) struct ShapeNodeRef<'a> {
    pub geometry_type: &'static str,
    pub width: &'a ScalarExpr,
    pub height: &'a ScalarExpr,
    pub points: Option<u64>,
    pub corner_radius: Option<&'a ScalarExpr>,
    pub inner_radius: Option<&'a ScalarExpr>,
    pub fill: &'a PaintSpec,
    pub stroke: Option<&'a StrokeSpec>,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) struct TextNodeRef<'a> {
    pub content: &'a str,
    pub font_size: &'a ScalarExpr,
    pub fill: &'a PaintSpec,
    pub width: Option<&'a ScalarExpr>,
    pub height: Option<&'a ScalarExpr>,
    pub line_height: Option<&'a ScalarExpr>,
    pub letter_spacing: Option<&'a ScalarExpr>,
    pub paragraph_spacing: Option<&'a ScalarExpr>,
    pub origin_x: Option<&'a ScalarExpr>,
    pub origin_y: Option<&'a ScalarExpr>,
    pub align: TextAlign,
    pub overflow: TextOverflow,
    pub transform: &'a TransformSpec,
}
''',
)

replace_once(
    "src/authoring/spec.rs",
    '''            | Self::Star { id, .. }
            | Self::Group { id, .. }
''',
    '''            | Self::Star { id, .. }
            | Self::Text { id, .. }
            | Self::Group { id, .. }
''',
)

replace_once(
    "src/authoring/spec.rs",
    '''            Self::Group { .. } | Self::Instance { .. } | Self::RawSceneObject { .. } => {
                return None;
            }
''',
    '''            Self::Text { .. }
            | Self::Group { .. }
            | Self::Instance { .. }
            | Self::RawSceneObject { .. } => {
                return None;
            }
''',
)

replace_once(
    "src/authoring/spec.rs",
    '''    pub(crate) fn children(&self) -> Option<&[VisualNode]> {
''',
    '''    pub(crate) fn text_node(&self) -> Option<TextNodeRef<'_>> {
        match self {
            Self::Text {
                text,
                font_size,
                fill,
                width,
                height,
                line_height,
                letter_spacing,
                paragraph_spacing,
                origin_x,
                origin_y,
                align,
                overflow,
                transform,
                ..
            } => Some(TextNodeRef {
                content: text,
                font_size,
                fill,
                width: width.as_ref(),
                height: height.as_ref(),
                line_height: line_height.as_ref(),
                letter_spacing: letter_spacing.as_ref(),
                paragraph_spacing: paragraph_spacing.as_ref(),
                origin_x: origin_x.as_ref(),
                origin_y: origin_y.as_ref(),
                align: *align,
                overflow: *overflow,
                transform,
            }),
            _ => None,
        }
    }

    pub(crate) fn children(&self) -> Option<&[VisualNode]> {
''',
)

replace_once(
    "src/authoring/validation.rs",
    '''        return;
    }

    match node {
''',
    '''        return;
    }

    if let Some(text) = node.text_node() {
        validate_expression(text.font_size, &format!("{path}.font_size"), diagnostics);
        validate_paint(text.fill, &format!("{path}.fill"), diagnostics);
        for (name, expression) in [
            ("width", text.width),
            ("height", text.height),
            ("line_height", text.line_height),
            ("letter_spacing", text.letter_spacing),
            ("paragraph_spacing", text.paragraph_spacing),
            ("origin_x", text.origin_x),
            ("origin_y", text.origin_y),
        ] {
            if let Some(expression) = expression {
                validate_expression(expression, &format!("{path}.{name}"), diagnostics);
            }
        }
        validate_transform(text.transform, &format!("{path}.transform"), diagnostics);
        return;
    }

    match node {
''',
)

replace_once(
    "src/authoring/validation.rs",
    '''        VisualNode::Ellipse { .. }
        | VisualNode::Rectangle { .. }
        | VisualNode::Triangle { .. }
        | VisualNode::Polygon { .. }
        | VisualNode::Star { .. } => unreachable!("shape nodes are handled above"),
''',
    '''        VisualNode::Ellipse { .. }
        | VisualNode::Rectangle { .. }
        | VisualNode::Triangle { .. }
        | VisualNode::Polygon { .. }
        | VisualNode::Star { .. }
        | VisualNode::Text { .. } => unreachable!("shape and text nodes are handled above"),
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''    AuthoringSpec, ComponentSpec, GradientKind, LoweredAuthoring, PaintSpec, Quantity, ScalarExpr,
    ShapeNodeRef, SourceMapEntry, TrimPathMode, TrimPathSpec, Unit, VisualNode,
''',
    '''    AuthoringSpec, ComponentSpec, GradientKind, LoweredAuthoring, PaintSpec, Quantity, ScalarExpr,
    ShapeNodeRef, SourceMapEntry, TextAlign, TextNodeRef, TextOverflow, TrimPathMode,
    TrimPathSpec, Unit, VisualNode,
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''enum PaintTarget {
    Fill,
    Stroke,
}
''',
    '''enum PaintTarget {
    Fill,
    Stroke,
    Text,
}
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''            Self::Fill => runtime_name(segments, role),
            Self::Stroke => runtime_name(segments, &format!("stroke_{role}")),
''',
    '''            Self::Fill => runtime_name(segments, role),
            Self::Stroke => runtime_name(segments, &format!("stroke_{role}")),
            Self::Text => runtime_name(segments, &format!("text_{role}")),
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''        if let Some(shape) = node.shape() {
            return self.lower_shape(shape, context);
        }

        let NodeContext {
''',
    '''        if let Some(shape) = node.shape() {
            return self.lower_shape(shape, context);
        }
        if let Some(text) = node.text_node() {
            return self.lower_text(text, context);
        }

        let NodeContext {
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''            VisualNode::Ellipse { .. }
            | VisualNode::Rectangle { .. }
            | VisualNode::Triangle { .. }
            | VisualNode::Polygon { .. }
            | VisualNode::Star { .. } => unreachable!("shape nodes are handled above"),
''',
    '''            VisualNode::Ellipse { .. }
            | VisualNode::Rectangle { .. }
            | VisualNode::Triangle { .. }
            | VisualNode::Polygon { .. }
            | VisualNode::Star { .. }
            | VisualNode::Text { .. } => unreachable!("shape and text nodes are handled above"),
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''    fn lower_shape(
''',
    '''    fn lower_text(
        &mut self,
        text: TextNodeRef<'_>,
        context: NodeContext<'_>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let TextNodeRef {
            content,
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
            if let Some(line_height) = line_height {
                object.insert("line_height".to_string(), Value::from(line_height));
            }
            if let Some(letter_spacing) = letter_spacing {
                object.insert(
                    "letter_spacing".to_string(),
                    Value::from(letter_spacing),
                );
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

    fn lower_shape(
''',
)

replace_once(
    "docs/authoring-spec-v0.md",
    "The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, triangles, polygons, stars, groups, component instances, and raw `SceneSpec` objects. Shape fills and strokes share one solid/linear/radial paint contract; stroke width is a positive pixel expression, and strokes may include a typed trim path. Polygon and star point counts must be at least three; star inner radius is a scalar ratio from zero to one. Bounded patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.",
    "The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, triangles, polygons, stars, literal text, groups, component instances, and raw `SceneSpec` objects. Shapes and text share one solid/linear/radial paint contract; stroke width is a positive pixel expression, and strokes may include a typed trim path. Polygon and star point counts must be at least three; star inner radius is a scalar ratio from zero to one. Font and image assets, bounded patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.",
)

replace_once(
    "docs/authoring-spec-v0.md",
    '''## Components and instances
''',
    '''## Text

A `text` visual node lowers to a deterministic Rive text hierarchy: a transform anchor, text object, one text style with a fill, and one literal value run. Numeric styling uses the same typed expressions and component parameters as shapes:

```json
{
  "kind": "text",
  "id": "headline",
  "text": "Rive from data",
  "font_size": { "kind": "parameter", "name": "headline-size" },
  "fill": "#F8FAFC",
  "width": { "kind": "literal", "value": 280, "unit": "px" },
  "line_height": { "kind": "literal", "value": 1.2, "unit": "scalar" },
  "align": "center",
  "overflow": "visible"
}
```

Font size and optional width, height, letter spacing, and paragraph spacing are pixel expressions. Line height is a positive scalar expression. Optional `origin_x` and `origin_y` are normalized scalar expressions from zero to one. Alignment is `left`, `right`, or `center`; overflow is `visible`, `hidden`, `clipped`, `ellipsis`, `fit`, or `fit_font_size`.

Sizing is derived rather than exposed as a low-level numeric switch: no dimensions produce auto-width text, width alone produces auto-height wrapping, and width plus height produces a fixed box. A height without a width is rejected. Literal content is intentionally separate from future string parameters and view-model bindings. Font asset embedding is the next asset-focused slice.

## Components and instances
''',
)

replace_once(
    "meta/todos/todo.visual-authoring-compiler.md",
    '''- Typed trim paths on strokes are implemented in PR #141. Start, end, and optional
  offset expressions flow through component parameters and overrides; generated trim
  objects, runtime names, SceneSpec paths, and diagnostics remain source-mapped.
- Remaining work includes text and assets, bounded patterns, constraints, and a
  complex static showcase without raw escapes.
''',
    '''- Typed trim paths on strokes are implemented in PR #141. Start, end, and optional
  offset expressions flow through component parameters and overrides; generated trim
  objects, runtime names, SceneSpec paths, and diagnostics remain source-mapped.
- Literal text nodes are implemented in PR #143 with parameterized numeric styling,
  semantic alignment and overflow, derived sizing, shared paints, deterministic
  runtime names, and complete source maps.
- Remaining work includes font and image assets, bounded patterns, constraints, and
  a complex static showcase without raw escapes.
''',
)

replace_once(
    "tests/authoring_examples.rs",
    '''const RAW_PULSE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/authoring/raw-pulse.v0.json"
));
''',
    '''const RAW_PULSE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/authoring/raw-pulse.v0.json"
));
const TEXT_LABEL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/authoring/text-label.v0.json"
));
''',
)

replace_once(
    "tests/authoring_examples.rs",
    '''#[test]
fn raw_pulse_example_is_deterministic_and_buildable() {
    assert_deterministic_and_buildable(RAW_PULSE);
}
''',
    '''#[test]
fn raw_pulse_example_is_deterministic_and_buildable() {
    assert_deterministic_and_buildable(RAW_PULSE);
}

#[test]
fn text_label_example_is_deterministic_and_buildable() {
    assert_deterministic_and_buildable(TEXT_LABEL);
}
''',
)

replace_once(
    "cairn.blueprint",
    '"./tests/authoring_stroke_contract.rs", "./tests/authoring_trim_path_contract.rs", "./tests/authoring_validation_contract.rs", "./tests/support"]',
    '"./tests/authoring_stroke_contract.rs", "./tests/authoring_text_contract.rs", "./tests/authoring_trim_path_contract.rs", "./tests/authoring_validation_contract.rs", "./tests/support"]',
)

Path("examples/authoring/text-label.v0.json").write_text(
    '''{
  "authoring_format_version": 0,
  "artboard": {
    "id": "text-label",
    "width": { "value": 480, "unit": "px" },
    "height": { "value": 240, "unit": "px" }
  },
  "parameters": {
    "headline-size": { "value": 42, "unit": "px" },
    "copy-width": { "value": 360, "unit": "px" }
  },
  "visual": {
    "nodes": [
      {
        "kind": "text",
        "id": "headline",
        "text": "Rive from data",
        "font_size": { "kind": "parameter", "name": "headline-size" },
        "fill": {
          "kind": "linear_gradient",
          "start_x": { "kind": "literal", "value": 0, "unit": "px" },
          "start_y": { "kind": "literal", "value": 0, "unit": "px" },
          "end_x": { "kind": "parameter", "name": "copy-width" },
          "end_y": { "kind": "literal", "value": 0, "unit": "px" },
          "stops": [
            {
              "color": "#22D3EE",
              "position": { "kind": "literal", "value": 0, "unit": "scalar" }
            },
            {
              "color": "#7C3AED",
              "position": { "kind": "literal", "value": 1, "unit": "scalar" }
            }
          ]
        },
        "width": { "kind": "parameter", "name": "copy-width" },
        "line_height": { "kind": "literal", "value": 1.2, "unit": "scalar" },
        "align": "center",
        "origin_x": { "kind": "literal", "value": 0.5, "unit": "scalar" },
        "transform": {
          "x": { "kind": "literal", "value": 240, "unit": "px" },
          "y": { "kind": "literal", "value": 96, "unit": "px" }
        }
      }
    ]
  },
  "motion": {},
  "behavior": {}
}
'''
)
