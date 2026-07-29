mod expression;
mod lower;
mod spec;

use schemars::schema_for;
use serde_json::Value;

pub use lower::{lower_authoring, lower_authoring_json};
pub use spec::{
    AUTHORING_FORMAT_VERSION, AuthoringArtboard, AuthoringDiagnostic, AuthoringError,
    AuthoringSourceMap, AuthoringSpec, BehaviorSection, ComponentSpec, LoweredAuthoring,
    MotionSection, Quantity, RawSceneFragment, ScalarExpr, SourceMapEntry, TransformSpec, Unit,
    VisualNode, VisualSection,
};

pub fn authoring_schema() -> Value {
    let mut schema = match serde_json::to_value(schema_for!(AuthoringSpec)) {
        Ok(schema) => schema,
        Err(_) => Value::Object(serde_json::Map::new()),
    };
    if let Some(object) = schema.as_object_mut() {
        object.insert(
            "$id".to_string(),
            Value::String(
                "https://github.com/George-RD/rive-rs-cli/docs/authoring.schema.v0.json"
                    .to_string(),
            ),
        );
        object.insert(
            "title".to_string(),
            Value::String("rive-cli AuthoringSpec v0".to_string()),
        );
    }
    schema
}
