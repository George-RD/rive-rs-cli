use serde_json::{Value, json};

use super::super::expression::{evaluate_expression, evaluate_transform};
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry, Unit};
use super::super::visual::GridNodeRef;
use super::{Lowerer, NodeContext, runtime_name};

impl<'a> Lowerer<'a> {
    pub(super) fn lower_grid(
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
        let NodeContext {
            authored_path,
            definition_path,
            authored_id,
            runtime_segments,
            scene_path,
            scope,
        } = context;

        let column_step = evaluate_expression(
            column_step_expression,
            &format!("{authored_path}.column_step"),
            scope,
            Unit::Px,
        )?;
        let row_step = evaluate_expression(
            row_step_expression,
            &format!("{authored_path}.row_step"),
            scope,
            Unit::Px,
        )?;
        let transform_values =
            evaluate_transform(transform, &format!("{authored_path}.transform"), scope)?;

        let wrapper_name = runtime_name(&runtime_segments, "grid");
        let capacity = usize::try_from(rows.saturating_mul(columns)).unwrap_or_default();
        let mut plans = Vec::with_capacity(capacity);
        let mut runtime_names = Vec::with_capacity(capacity + 1);
        let mut scene_paths = Vec::with_capacity(capacity + 1);
        runtime_names.push(wrapper_name.clone());
        scene_paths.push(scene_path.clone());

        for row in 0..rows {
            for column in 0..columns {
                let index = plans.len();
                let cell_segment = format!("r{row}c{column}");
                let mut cell_runtime_segments = runtime_segments.clone();
                cell_runtime_segments.push(cell_segment.clone());
                let cell_name = runtime_name(&cell_runtime_segments, "cell");
                let cell_scene_path = format!("{scene_path}/children/{index}");
                runtime_names.push(cell_name.clone());
                scene_paths.push(cell_scene_path.clone());
                plans.push((row, column, cell_segment, cell_name, cell_scene_path));
            }
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
        let mut cells = Vec::with_capacity(capacity);
        for (row, column, cell_segment, cell_name, cell_scene_path) in plans {
            let mut item_runtime_segments = runtime_segments.clone();
            item_runtime_segments.push(cell_segment.clone());
            item_runtime_segments.push(item.id().to_string());
            let lowered_item = self.lower_node(
                item,
                NodeContext {
                    authored_path: item_authored_path.clone(),
                    definition_path: item_definition_path.clone(),
                    authored_id: format!("{authored_id}/{cell_segment}/{}", item.id()),
                    runtime_segments: item_runtime_segments,
                    scene_path: format!("{cell_scene_path}/children/0"),
                    scope,
                },
                component_stack,
            )?;
            cells.push(json!({
                "type": "node",
                "name": cell_name,
                "x": column as f64 * column_step,
                "y": row as f64 * row_step,
                "rotation": 0.0,
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
