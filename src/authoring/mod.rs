mod expression;
mod frontend;
mod lower;
mod spec;

use schemars::schema_for;
use serde_json::Value;

pub use frontend::{lower_authoring, lower_authoring_json};
pub use lower::lower_authoring_json as lower_authoring_json_unchecked;
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

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::{AuthoringError, authoring_schema, lower_authoring_json};

    fn document() -> Value {
        json!({
            "authoring_format_version": 0,
            "artboard": {
                "id": "stage",
                "width": { "value": 320.0, "unit": "px" },
                "height": { "value": 240.0, "unit": "px" }
            },
            "visual": { "nodes": [] },
            "motion": {},
            "behavior": {}
        })
    }

    fn lower(input: &Value) -> Result<super::LoweredAuthoring, AuthoringError> {
        lower_authoring_json(&input.to_string())
    }

    fn has_diagnostic(error: &AuthoringError, code: &str, path: &str) -> bool {
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code && diagnostic.path == path)
    }

    #[test]
    fn schema_constrains_authoring_version_to_zero() {
        let schema = authoring_schema();
        let version = &schema["properties"]["authoring_format_version"];

        assert_eq!(version["minimum"], 0);
        assert_eq!(version["maximum"], 0);
    }

    #[test]
    fn percent_is_not_part_of_the_v0_contract() {
        let schema = authoring_schema();
        let schema_text = serde_json::to_string(&schema).expect("serialize authoring schema");
        assert!(!schema_text.contains("\"percent\""));

        let mut input = document();
        input["artboard"]["width"]["unit"] = json!("percent");
        let error = lower(&input).expect_err("unsupported unit must fail at the strict frontend");
        assert!(has_diagnostic(&error, "invalid_json", "$"));
    }

    #[test]
    fn positive_values_that_underflow_f32_are_rejected() {
        let mut input = document();
        input["artboard"]["width"]["value"] = json!(1.0e-100_f64);

        let error = lower(&input).expect_err("underflowing values must fail");
        assert!(has_diagnostic(
            &error,
            "numeric_out_of_range",
            "$.artboard.width.value"
        ));
    }

    #[test]
    fn component_bodies_only_see_declared_component_parameters() {
        let mut input = document();
        input["parameters"] = json!({
            "root_only": { "value": 80.0, "unit": "px" }
        });
        input["components"] = json!([
            {
                "id": "badge",
                "visual": [
                    {
                        "kind": "rectangle",
                        "id": "body",
                        "width": { "kind": "parameter", "name": "root_only" },
                        "height": { "kind": "literal", "value": 40.0, "unit": "px" },
                        "fill": "#111827"
                    }
                ]
            }
        ]);
        input["visual"]["nodes"] = json!([
            { "kind": "instance", "id": "root", "component": "badge" }
        ]);

        let error = lower(&input).expect_err("undeclared component parameter must fail");
        assert!(has_diagnostic(
            &error,
            "unknown_parameter",
            "$.components[0].visual[0].width.name"
        ));
    }

    #[test]
    fn raw_fragment_runtime_names_share_the_collision_registry() {
        let mut input = document();
        input["motion"] = json!({
            "raw_animations": [
                {
                    "id": "first",
                    "value": {
                        "name": "duplicate",
                        "fps": 60,
                        "duration": 1,
                        "keyframes": []
                    }
                },
                {
                    "id": "second",
                    "value": {
                        "name": "duplicate",
                        "fps": 60,
                        "duration": 1,
                        "keyframes": []
                    }
                }
            ]
        });

        let error = lower(&input).expect_err("duplicate runtime name must fail before the builder");
        assert!(has_diagnostic(
            &error,
            "runtime_name_collision",
            "$.motion.raw_animations[1].value"
        ));
    }

    #[test]
    fn authored_ids_reserve_the_source_map_path_separator() {
        let mut input = document();
        input["visual"]["nodes"] = json!([
            { "kind": "group", "id": "left/disc", "children": [] }
        ]);

        let error = lower(&input).expect_err("ambiguous authored id must fail");
        assert!(has_diagnostic(&error, "invalid_id", "$.visual.nodes[0].id"));
    }

    #[test]
    fn parameter_names_cannot_contain_diagnostic_path_metacharacters() {
        let mut input = document();
        input["parameters"] = json!({
            "layout.width": { "value": 80.0, "unit": "px" }
        });

        let error = lower(&input).expect_err("ambiguous parameter name must fail");
        assert!(has_diagnostic(&error, "invalid_parameter", "$.parameters"));
    }
}
