use serde_json::{Value, json};

use super::super::expression::{evaluate_expression, evaluate_transform, validate_scene_number};
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry, TransformSpec, Unit};
use super::super::visual::{GridNodeRef, PatternNodeRef, RadialNodeRef, VisualNode};
use super::{Lowerer, NodeContext, runtime_name};

struct PatternPlacement {
    segment: String,
    x: f64,
    y: f64,
    rotation: f64,
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
                placements.push(PatternPlacement {
                    segment: format!("r{row}c{column}"),
                    x: column as f64 * column_step,
                    y: row as f64 * row_step,
                    rotation: 0.0,
                });
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
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            validate_scene_number(x, &radius_path)?;
            validate_scene_number(y, &radius_path)?;
            placements.push(PatternPlacement {
                segment: format!("p{index}"),
                x,
                y,
                rotation: if rotate_items { angle } else { 0.0 },
            });
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
                "scale_x": 1.0,
                "scale_y": 1.0,
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
