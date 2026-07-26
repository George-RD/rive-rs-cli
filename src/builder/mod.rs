mod animations;
mod objects;
mod parsers;
pub mod scene;
pub(crate) mod spec;
mod state_machines;
mod validation;

pub use scene::{artboard_presets, build_scene};
pub use spec::SceneSpec;

pub fn scene_schema() -> serde_json::Value {
    let mut schema = serde_json::to_value(schemars::schema_for!(SceneSpec))
        .unwrap_or_else(|_| serde_json::json!({}));
    if let Some(obj) = schema.as_object_mut() {
        obj.insert(
            "$id".to_string(),
            serde_json::Value::String(
                "https://github.com/George-RD/rive-rs-cli/docs/scene.schema.v1.json".to_string(),
            ),
        );
        obj.insert(
            "title".to_string(),
            serde_json::Value::String("rive-cli SceneSpec v1".to_string()),
        );
    }
    schema
}

pub fn scene_schema_json() -> String {
    let mut s = serde_json::to_string_pretty(&scene_schema()).unwrap_or_default();
    s.push('\n');
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn scene_schema_file_is_in_sync() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("docs")
            .join("scene.schema.v1.json");
        let generated = super::scene_schema_json();
        if std::env::var_os("UPDATE_SCENE_SCHEMA").is_some() {
            std::fs::write(&path, &generated).expect("write scene schema");
            return;
        }
        let committed = std::fs::read_to_string(&path).expect("read scene schema");
        assert_eq!(
            committed, generated,
            "docs/scene.schema.v1.json is stale; regenerate with UPDATE_SCENE_SCHEMA=1 cargo test scene_schema_file_is_in_sync"
        );
    }
}
