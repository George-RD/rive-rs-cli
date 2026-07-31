use serde_json::{Value, json};

use super::super::expression::evaluate_transform;
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry};
use super::super::visual::ImageNodeRef;
use super::{Lowerer, NodeContext, image_asset_runtime_name, runtime_name};

impl<'a> Lowerer<'a> {
    pub(super) fn lower_image(
        &mut self,
        image: ImageNodeRef<'_>,
        context: NodeContext<'_>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let ImageNodeRef { asset, transform } = image;
        let NodeContext {
            authored_path,
            definition_path,
            authored_id,
            runtime_segments,
            scene_path,
            scope,
        } = context;

        if !self.spec.image_assets.contains_key(asset) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.asset"),
                "unknown_image_asset",
                format!("image asset '{asset}' is not declared"),
            ));
        }

        let transform_values =
            evaluate_transform(transform, &format!("{authored_path}.transform"), scope)?;
        let anchor_name = runtime_name(&runtime_segments, "image_anchor");
        let image_name = runtime_name(&runtime_segments, "image");
        let image_scene_path = format!("{scene_path}/children/0");
        let runtime_names = vec![anchor_name.clone(), image_name.clone()];
        let scene_paths = vec![scene_path.clone(), image_scene_path];
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
            "children": [{
                "type": "image",
                "name": image_name,
                "asset": image_asset_runtime_name(&self.spec.artboard.id, asset)
            }]
        }))
    }
}
