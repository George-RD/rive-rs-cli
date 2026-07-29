from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file_path = Path(path)
    text = file_path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f"expected one match in {path}, found {count}")
    file_path.write_text(text.replace(old, new, 1), encoding="utf-8")


replace_once(
    "src/authoring/spec.rs",
    '''    Rectangle {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        fill: String,
        #[serde(default)]
        corner_radius: Option<ScalarExpr>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Group {
''',
    '''    Rectangle {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        fill: String,
        #[serde(default)]
        corner_radius: Option<ScalarExpr>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Triangle {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        fill: String,
        #[serde(default)]
        transform: TransformSpec,
    },
    Polygon {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        #[schemars(range(min = 3))]
        points: u64,
        fill: String,
        #[serde(default)]
        corner_radius: Option<ScalarExpr>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Star {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        #[schemars(range(min = 3))]
        points: u64,
        inner_radius: ScalarExpr,
        fill: String,
        #[serde(default)]
        corner_radius: Option<ScalarExpr>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Group {
''',
)

replace_once(
    "src/authoring/spec.rs",
    '''            Self::Ellipse { id, .. }
            | Self::Rectangle { id, .. }
            | Self::Group { id, .. }
''',
    '''            Self::Ellipse { id, .. }
            | Self::Rectangle { id, .. }
            | Self::Triangle { id, .. }
            | Self::Polygon { id, .. }
            | Self::Star { id, .. }
            | Self::Group { id, .. }
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''            VisualNode::Ellipse {
                width,
                height,
                fill,
                transform,
                ..
            } => self.lower_shape(
                "ellipse",
                width,
                height,
                None,
                fill,
                transform,
                authored_path,
                definition_path,
                authored_id,
                runtime_segments,
                scene_path,
                scope,
            ),
            VisualNode::Rectangle {
                width,
                height,
                fill,
                corner_radius,
                transform,
                ..
            } => self.lower_shape(
                "rectangle",
                width,
                height,
                corner_radius.as_ref(),
                fill,
                transform,
                authored_path,
                definition_path,
                authored_id,
                runtime_segments,
                scene_path,
                scope,
            ),
            VisualNode::Group {
''',
    '''            VisualNode::Ellipse {
                width,
                height,
                fill,
                transform,
                ..
            } => self.lower_shape(
                "ellipse",
                width,
                height,
                None,
                None,
                None,
                fill,
                transform,
                authored_path,
                definition_path,
                authored_id,
                runtime_segments,
                scene_path,
                scope,
            ),
            VisualNode::Rectangle {
                width,
                height,
                fill,
                corner_radius,
                transform,
                ..
            } => self.lower_shape(
                "rectangle",
                width,
                height,
                None,
                corner_radius.as_ref(),
                None,
                fill,
                transform,
                authored_path,
                definition_path,
                authored_id,
                runtime_segments,
                scene_path,
                scope,
            ),
            VisualNode::Triangle {
                width,
                height,
                fill,
                transform,
                ..
            } => self.lower_shape(
                "triangle",
                width,
                height,
                None,
                None,
                None,
                fill,
                transform,
                authored_path,
                definition_path,
                authored_id,
                runtime_segments,
                scene_path,
                scope,
            ),
            VisualNode::Polygon {
                width,
                height,
                points,
                fill,
                corner_radius,
                transform,
                ..
            } => self.lower_shape(
                "polygon",
                width,
                height,
                Some(*points),
                corner_radius.as_ref(),
                None,
                fill,
                transform,
                authored_path,
                definition_path,
                authored_id,
                runtime_segments,
                scene_path,
                scope,
            ),
            VisualNode::Star {
                width,
                height,
                points,
                inner_radius,
                fill,
                corner_radius,
                transform,
                ..
            } => self.lower_shape(
                "star",
                width,
                height,
                Some(*points),
                corner_radius.as_ref(),
                Some(inner_radius),
                fill,
                transform,
                authored_path,
                definition_path,
                authored_id,
                runtime_segments,
                scene_path,
                scope,
            ),
            VisualNode::Group {
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''        geometry_type: &str,
        width_expression: &super::spec::ScalarExpr,
        height_expression: &super::spec::ScalarExpr,
        corner_radius_expression: Option<&super::spec::ScalarExpr>,
        fill: &str,
''',
    '''        geometry_type: &str,
        width_expression: &super::spec::ScalarExpr,
        height_expression: &super::spec::ScalarExpr,
        points: Option<u64>,
        corner_radius_expression: Option<&super::spec::ScalarExpr>,
        inner_radius_expression: Option<&super::spec::ScalarExpr>,
        fill: &str,
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''        if height <= 0.0 {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.height"),
                "invalid_dimension",
                "shape height must be greater than zero",
            ));
        }
        let corner_radius = corner_radius_expression
''',
    '''        if height <= 0.0 {
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
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''        if corner_radius.is_some_and(|radius| radius < 0.0) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.corner_radius"),
                "invalid_dimension",
                "corner radius must not be negative",
            ));
        }
        let transform_values =
''',
    '''        if corner_radius.is_some_and(|radius| radius < 0.0) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.corner_radius"),
                "invalid_dimension",
                "corner radius must not be negative",
            ));
        }
        let inner_radius = inner_radius_expression
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
        let transform_values =
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''        if geometry_type == "rectangle"
            && let Some(object) = geometry.as_object_mut()
        {
            object.insert(
                "corner_radius".to_string(),
                corner_radius.map_or(Value::Null, Value::from),
            );
        }
''',
    '''        if let Some(object) = geometry.as_object_mut() {
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
''',
)

replace_once(
    "src/authoring/frontend.rs",
    '''            VisualNode::Ellipse { .. }
            | VisualNode::Rectangle { .. }
            | VisualNode::RawSceneObject { .. } => {}
''',
    '''            VisualNode::Ellipse { .. }
            | VisualNode::Rectangle { .. }
            | VisualNode::Triangle { .. }
            | VisualNode::Polygon { .. }
            | VisualNode::Star { .. }
            | VisualNode::RawSceneObject { .. } => {}
''',
)

replace_once(
    "src/authoring/frontend.rs",
    '''        VisualNode::Ellipse {
            width,
            height,
            transform,
            ..
        } => {
            validate_expression(width, &format!("{path}.width"), diagnostics);
            validate_expression(height, &format!("{path}.height"), diagnostics);
            validate_transform(transform, &format!("{path}.transform"), diagnostics);
        }
        VisualNode::Rectangle {
            width,
            height,
            corner_radius,
            transform,
            ..
        } => {
            validate_expression(width, &format!("{path}.width"), diagnostics);
            validate_expression(height, &format!("{path}.height"), diagnostics);
            if let Some(corner_radius) = corner_radius {
                validate_expression(corner_radius, &format!("{path}.corner_radius"), diagnostics);
            }
            validate_transform(transform, &format!("{path}.transform"), diagnostics);
        }
        VisualNode::Group {
''',
    '''        VisualNode::Ellipse {
            width,
            height,
            transform,
            ..
        }
        | VisualNode::Triangle {
            width,
            height,
            transform,
            ..
        } => {
            validate_expression(width, &format!("{path}.width"), diagnostics);
            validate_expression(height, &format!("{path}.height"), diagnostics);
            validate_transform(transform, &format!("{path}.transform"), diagnostics);
        }
        VisualNode::Rectangle {
            width,
            height,
            corner_radius,
            transform,
            ..
        }
        | VisualNode::Polygon {
            width,
            height,
            corner_radius,
            transform,
            ..
        } => {
            validate_expression(width, &format!("{path}.width"), diagnostics);
            validate_expression(height, &format!("{path}.height"), diagnostics);
            if let Some(corner_radius) = corner_radius {
                validate_expression(corner_radius, &format!("{path}.corner_radius"), diagnostics);
            }
            validate_transform(transform, &format!("{path}.transform"), diagnostics);
        }
        VisualNode::Star {
            width,
            height,
            inner_radius,
            corner_radius,
            transform,
            ..
        } => {
            validate_expression(width, &format!("{path}.width"), diagnostics);
            validate_expression(height, &format!("{path}.height"), diagnostics);
            validate_expression(
                inner_radius,
                &format!("{path}.inner_radius"),
                diagnostics,
            );
            if let Some(corner_radius) = corner_radius {
                validate_expression(corner_radius, &format!("{path}.corner_radius"), diagnostics);
            }
            validate_transform(transform, &format!("{path}.transform"), diagnostics);
        }
        VisualNode::Group {
''',
)

replace_once(
    "src/authoring/limits.rs",
    '''            VisualNode::Ellipse { .. }
            | VisualNode::Rectangle { .. }
            | VisualNode::RawSceneObject { .. } => {}
''',
    '''            VisualNode::Ellipse { .. }
            | VisualNode::Rectangle { .. }
            | VisualNode::Triangle { .. }
            | VisualNode::Polygon { .. }
            | VisualNode::Star { .. }
            | VisualNode::RawSceneObject { .. } => {}
''',
)

replace_once(
    "docs/authoring-spec-v0.md",
    "The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, groups, component instances, and raw `SceneSpec` objects. Broader primitives, bounded patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.",
    "The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, triangles, polygons, stars, groups, component instances, and raw `SceneSpec` objects. Polygon and star point counts must be at least three; star inner radius is a scalar ratio from zero to one. Bounded patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.",
)
