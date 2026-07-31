use serde_json::{Value, json};

use super::super::deterministic_math::sin_cos;
use super::super::expression::{evaluate_expression, evaluate_transform, validate_scene_number};
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry, TransformSpec, Unit};
use super::super::visual::{
    DistributeNodeRef, GridNodeRef, MirrorAxis, MirrorNodeRef, PatternNodeRef, RadialNodeRef,
    VisualNode,
};
use super::{Lowerer, NodeContext, runtime_name};

struct PatternPlacement {
    segment: String,
    x: f64,
    y: f64,
    rotation: f64,
    scale_x: f64,
    scale_y: f64,
}

impl PatternPlacement {
    fn positioned(segment: impl Into<String>, x: f64, y: f64, rotation: f64) -> Self {
        Self {
            segment: segment.into(),
            x,
            y,
            rotation,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }

    fn reflected(segment: impl Into<String>, scale_x: f64, scale_y: f64) -> Self {
        Self {
            segment: segment.into(),
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale_x,
            scale_y,
        }
    }
}

fn mirror_placements(axis: MirrorAxis) -> [PatternPlacement; 2] {
    let (scale_x, scale_y) = match axis {
        MirrorAxis::Vertical => (-1.0, 1.0),
        MirrorAxis::Horizontal => (1.0, -1.0),
    };
    [
        PatternPlacement::positioned("original", 0.0, 0.0, 0.0),
        PatternPlacement::reflected("mirrored", scale_x, scale_y),
    ]
}

fn distribute_placements(
    copies: u64,
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
) -> Vec<PatternPlacement> {
    let capacity = usize::try_from(copies).unwrap_or_default();
    let mut placements = Vec::with_capacity(capacity);
    let last = copies.saturating_sub(1);
    for index in 0..copies {
        let (x, y) = if index == 0 {
            (start_x, start_y)
        } else if index == last {
            (end_x, end_y)
        } else {
            let progress = index as f64 / last as f64;
            (
                start_x + (end_x - start_x) * progress,
                start_y + (end_y - start_y) * progress,
            )
        };
        placements.push(PatternPlacement::positioned(format!("d{index}"), x, y, 0.0));
    }
    placements
}

impl<'a> Lowerer<'a> {
    pub(super) fn lower_pattern(
        &mut self,
        pattern: PatternNodeRef<'_>,
        context: NodeContext<'_>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        match pattern {
            PatternNodeRef::Grid(grid) => self.lower_grid(grid, context, component_stack),
            PatternNodeRef::Radial(radial) => self.lower_radial(radial, context, component_stack),
            PatternNodeRef::Mirror(mirror) => self.lower_mirror(mirror, context, component_stack),
            PatternNodeRef::Distribute(distribute) => {
                self.lower_distribute(distribute, context, component_stack)
            }
        }
    }

    fn lower_grid(
        &mut self,
        grid: GridNodeRef<'_>,
        context: NodeContext<'_>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let GridNodeRef {
            columns,
            rows,
            column_step: column_step_expression,
            row_step: row_step_expression,
            item,
            transform,
        } = grid;
        let column_step_path = format!("{}.column_step", context.authored_path);
        let column_step = evaluate_expression(
            column_step_expression,
            &column_step_path,
            context.scope,
            Unit::Px,
        )?;
        let row_step_path = format!("{}.row_step", context.authored_path);
        let row_step =
            evaluate_expression(row_step_expression, &row_step_path, context.scope, Unit::Px)?;
        validate_scene_number(
            columns.saturating_sub(1) as f64 * column_step,
            &column_step_path,
        )?;
        validate_scene_number(rows.saturating_sub(1) as f64 * row_step, &row_step_path)?;

        let capacity = usize::try_from(rows.saturating_mul(columns)).unwrap_or_default();
        let mut placements = Vec::with_capacity(capacity);
        for row in 0..rows {
            for column in 0..columns {
                placements.push(PatternPlacement::positioned(
                    format!("r{row}c{column}"),
                    column as f64 * column_step,
                    row as f64 * row_step,
                    0.0,
                ));
            }
        }

        self.lower_repeated_pattern(
            "grid",
            item,
            transform,
            context,
            placements,
            component_stack,
        )
    }

    fn lower_radial(
        &mut self,
        radial: RadialNodeRef<'_>,
        context: NodeContext<'_>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let RadialNodeRef {
            copies,
            radius: radius_expression,
            start_angle: start_angle_expression,
            angle_step: angle_step_expression,
            rotate_items,
            item,
            transform,
        } = radial;
        let radius_path = format!("{}.radius", context.authored_path);
        let radius = evaluate_expression(radius_expression, &radius_path, context.scope, Unit::Px)?;
        if radius < 0.0 {
            return Err(AuthoringDiagnostic::new(
                radius_path,
                "invalid_pattern_radius",
                "radial pattern radius must be non-negative",
            ));
        }
        let start_angle_path = format!("{}.start_angle", context.authored_path);
        let start_angle = evaluate_expression(
            start_angle_expression,
            &start_angle_path,
            context.scope,
            Unit::Radians,
        )?;
        let angle_step_path = format!("{}.angle_step", context.authored_path);
        let angle_step = evaluate_expression(
            angle_step_expression,
            &angle_step_path,
            context.scope,
            Unit::Radians,
        )?;

        let capacity = usize::try_from(copies).unwrap_or_default();
        let mut placements = Vec::with_capacity(capacity);
        for index in 0..copies {
            let angle = start_angle + index as f64 * angle_step;
            validate_scene_number(angle, &angle_step_path)?;
            let (sine, cosine) = sin_cos(angle);
            let x = radius * cosine;
            let y = radius * sine;
            validate_scene_number(x, &radius_path)?;
            validate_scene_number(y, &radius_path)?;
            placements.push(PatternPlacement::positioned(
                format!("p{index}"),
                x,
                y,
                if rotate_items { angle } else { 0.0 },
            ));
        }

        self.lower_repeated_pattern(
            "radial",
            item,
            transform,
            context,
            placements,
            component_stack,
        )
    }

    fn lower_distribute(
        &mut self,
        distribute: DistributeNodeRef<'_>,
        context: NodeContext<'_>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let DistributeNodeRef {
            copies,
            start_x: start_x_expression,
            start_y: start_y_expression,
            end_x: end_x_expression,
            end_y: end_y_expression,
            item,
            transform,
        } = distribute;
        let start_x_path = format!("{}.start_x", context.authored_path);
        let start_x =
            evaluate_expression(start_x_expression, &start_x_path, context.scope, Unit::Px)?;
        let start_y_path = format!("{}.start_y", context.authored_path);
        let start_y =
            evaluate_expression(start_y_expression, &start_y_path, context.scope, Unit::Px)?;
        let end_x_path = format!("{}.end_x", context.authored_path);
        let end_x = evaluate_expression(end_x_expression, &end_x_path, context.scope, Unit::Px)?;
        let end_y_path = format!("{}.end_y", context.authored_path);
        let end_y = evaluate_expression(end_y_expression, &end_y_path, context.scope, Unit::Px)?;

        let placements = distribute_placements(copies, start_x, start_y, end_x, end_y);
        for placement in &placements {
            validate_scene_number(placement.x, &end_x_path)?;
            validate_scene_number(placement.y, &end_y_path)?;
        }

        self.lower_repeated_pattern(
            "distribute",
            item,
            transform,
            context,
            placements,
            component_stack,
        )
    }

    fn lower_mirror(
        &mut self,
        mirror: MirrorNodeRef<'_>,
        context: NodeContext<'_>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let MirrorNodeRef {
            axis,
            item,
            transform,
        } = mirror;
        let placements = Vec::from(mirror_placements(axis));

        self.lower_repeated_pattern(
            "mirror",
            item,
            transform,
            context,
            placements,
            component_stack,
        )
    }

    fn lower_repeated_pattern(
        &mut self,
        role: &str,
        item: &VisualNode,
        transform: &TransformSpec,
        context: NodeContext<'_>,
        placements: Vec<PatternPlacement>,
        component_stack: &mut Vec<String>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let NodeContext {
            authored_path,
            definition_path,
            authored_id,
            runtime_segments,
            scene_path,
            scope,
        } = context;
        let transform_values =
            evaluate_transform(transform, &format!("{authored_path}.transform"), scope)?;

        let wrapper_name = runtime_name(&runtime_segments, role);
        let mut plans = Vec::with_capacity(placements.len());
        let mut runtime_names = Vec::with_capacity(placements.len() + 1);
        let mut scene_paths = Vec::with_capacity(placements.len() + 1);
        runtime_names.push(wrapper_name.clone());
        scene_paths.push(scene_path.clone());

        for (index, placement) in placements.into_iter().enumerate() {
            let mut cell_runtime_segments = runtime_segments.clone();
            cell_runtime_segments.push(placement.segment.clone());
            let cell_name = runtime_name(&cell_runtime_segments, "cell");
            let cell_scene_path = format!("{scene_path}/children/{index}");
            runtime_names.push(cell_name.clone());
            scene_paths.push(cell_scene_path.clone());
            plans.push((placement, cell_name, cell_scene_path));
        }

        self.register_runtime_names(&runtime_names, &format!("{authored_path}.id"))?;
        self.source_map.entries.push(SourceMapEntry {
            authored_id: authored_id.clone(),
            authored_path: authored_path.clone(),
            definition_path: definition_path.clone(),
            runtime_names,
            scene_paths,
        });

        let item_authored_path = format!("{authored_path}.item");
        let item_definition_path = definition_path.as_ref().map(|path| format!("{path}.item"));
        let mut cells = Vec::with_capacity(plans.len());
        for (placement, cell_name, cell_scene_path) in plans {
            let mut item_runtime_segments = runtime_segments.clone();
            item_runtime_segments.push(placement.segment.clone());
            item_runtime_segments.push(item.id().to_string());
            let lowered_item = self.lower_node(
                item,
                NodeContext {
                    authored_path: item_authored_path.clone(),
                    definition_path: item_definition_path.clone(),
                    authored_id: format!("{authored_id}/{}/{}", placement.segment, item.id()),
                    runtime_segments: item_runtime_segments,
                    scene_path: format!("{cell_scene_path}/children/0"),
                    scope,
                },
                component_stack,
            )?;
            cells.push(json!({
                "type": "node",
                "name": cell_name,
                "x": placement.x,
                "y": placement.y,
                "rotation": placement.rotation,
                "scale_x": placement.scale_x,
                "scale_y": placement.scale_y,
                "children": [lowered_item]
            }));
        }

        Ok(json!({
            "type": "node",
            "name": wrapper_name,
            "x": transform_values.x,
            "y": transform_values.y,
            "rotation": transform_values.rotation,
            "scale_x": transform_values.scale_x,
            "scale_y": transform_values.scale_y,
            "children": cells
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::{MirrorAxis, distribute_placements, mirror_placements};

    #[test]
    fn distribute_placements_include_both_endpoints_and_equal_intervals() {
        let placements = distribute_placements(4, 0.0, 10.0, 90.0, 55.0);
        assert_eq!(placements.len(), 4);
        assert_eq!(placements[0].segment, "d0");
        assert_eq!(placements[0].x, 0.0);
        assert_eq!(placements[0].y, 10.0);
        assert_eq!(placements[1].segment, "d1");
        assert_eq!(placements[1].x, 30.0);
        assert_eq!(placements[1].y, 25.0);
        assert_eq!(placements[2].segment, "d2");
        assert_eq!(placements[2].x, 60.0);
        assert_eq!(placements[2].y, 40.0);
        assert_eq!(placements[3].segment, "d3");
        assert_eq!(placements[3].x, 90.0);
        assert_eq!(placements[3].y, 55.0);
    }

    #[test]
    fn mirror_placements_reflect_across_the_requested_axis() {
        let vertical = mirror_placements(MirrorAxis::Vertical);
        assert_eq!(vertical[0].segment, "original");
        assert_eq!(vertical[0].x, 0.0);
        assert_eq!(vertical[0].y, 0.0);
        assert_eq!(vertical[0].rotation, 0.0);
        assert_eq!(vertical[0].scale_x, 1.0);
        assert_eq!(vertical[0].scale_y, 1.0);
        assert_eq!(vertical[1].segment, "mirrored");
        assert_eq!(vertical[1].scale_x, -1.0);
        assert_eq!(vertical[1].scale_y, 1.0);

        let horizontal = mirror_placements(MirrorAxis::Horizontal);
        assert_eq!(horizontal[0].segment, "original");
        assert_eq!(horizontal[0].scale_x, 1.0);
        assert_eq!(horizontal[0].scale_y, 1.0);
        assert_eq!(horizontal[1].segment, "mirrored");
        assert_eq!(horizontal[1].scale_x, 1.0);
        assert_eq!(horizontal[1].scale_y, -1.0);
    }
}
