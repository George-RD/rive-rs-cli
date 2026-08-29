mod constraint;
mod deterministic_math;
mod expression;
mod frontend;
mod limits;
mod lower;
mod operations;
mod spec;
mod validation;
mod visual;

use schemars::schema_for;
use serde_json::Value;

pub use operations::{
    AppliedOperation, AuthoringContainer, AuthoringEntity, AuthoringOperation, AuthoringPlacement,
    AuthoringTarget, apply_operation, apply_operations,
};
pub use spec::{
    AUTHORING_FORMAT_VERSION, AuthoringArtboard, AuthoringDiagnostic, AuthoringError,
    AuthoringSourceMap, AuthoringSpec, BehaviorBindingConditionSpec, BehaviorBindingSpec,
    BehaviorEventSpec, BehaviorInputConditionSpec, BehaviorInputSpec, BehaviorListenerActionSpec,
    BehaviorListenerSpec, BehaviorListenerType, BehaviorModelSpec, BehaviorPropertySpec,
    BehaviorSection, BehaviorStateSpec, BehaviorStatechartSpec, BehaviorTransitionConditionSpec,
    BehaviorTransitionSpec, ComponentSpec, ConstraintAxis, ConstraintSpec, GradientKind,
    GradientPaintSpec, GradientStopSpec, LoweredAuthoring, MotionEasingSpec, MotionInterpolation,
    MotionLoop, MotionSection, MotionTrackSpec, PaintSpec, PoseKeyframeSpec, PoseSpec,
    PoseTargetSpec, Quantity, RawSceneFragment, ScalarExpr, SourceMapEntry, StrokeSpec,
    TransformSpec, Unit, VisualSection,
};
pub use visual::{MirrorAxis, PathPointSpec, VisualNode};

pub fn lower_authoring_json(input: &str) -> Result<LoweredAuthoring, AuthoringError> {
    let spec = serde_json::from_str::<AuthoringSpec>(input).map_err(|error| {
        AuthoringError::one(AuthoringDiagnostic::new(
            "$",
            "invalid_json",
            format!(
                "{error} at line {}, column {}",
                error.line(),
                error.column()
            ),
        ))
    })?;
    lower_authoring(&spec)
}

pub fn lower_authoring(spec: &AuthoringSpec) -> Result<LoweredAuthoring, AuthoringError> {
    limits::validate_expansion_limits(spec)?;
    frontend::lower_authoring(spec)
}

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

    use crate::builder::{SceneSpec, build_scene};

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

    fn assert_builds(scene: Value) {
        let scene: SceneSpec =
            serde_json::from_value(scene).expect("lowered SceneSpec must deserialize");
        build_scene(&scene, None).expect("lowered SceneSpec must pass the canonical builder");
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
    fn degree_normalization_that_underflows_f32_is_rejected() {
        let mut input = document();
        input["visual"]["nodes"] = json!([
            {
                "kind": "group",
                "id": "tiny-rotation",
                "transform": {
                    "rotation": {
                        "kind": "literal",
                        "value": 1.0e-44_f64,
                        "unit": "degrees"
                    }
                },
                "children": []
            }
        ]);

        let error = lower(&input).expect_err("normalized underflow must fail");
        assert!(has_diagnostic(
            &error,
            "numeric_out_of_range",
            "$.visual.nodes[0].transform.rotation.value"
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

        let error = lower(&input).expect_err("duplicate runtime names must fail");
        assert!(has_diagnostic(
            &error,
            "runtime_name_collision",
            "$.motion.raw_animations[1].value"
        ));
    }

    #[test]
    fn duplicate_raw_fragment_ids_are_rejected_before_lowering() {
        let mut input = document();
        input["motion"] = json!({
            "raw_animations": [
                {
                    "id": "same",
                    "value": {
                        "name": "first",
                        "fps": 60,
                        "duration": 1,
                        "keyframes": []
                    }
                },
                {
                    "id": "same",
                    "value": {
                        "name": "second",
                        "fps": 60,
                        "duration": 1,
                        "keyframes": []
                    }
                }
            ]
        });

        let error = lower(&input).expect_err("duplicate fragment ids must fail");
        assert!(has_diagnostic(
            &error,
            "duplicate_id",
            "$.motion.raw_animations[1].id"
        ));
    }

    #[test]
    fn behavior_raw_fragment_runtime_names_share_the_collision_registry() {
        let mut input = document();
        input["behavior"] = json!({
            "raw_state_machines": [
                {
                    "id": "first",
                    "value": {
                        "name": "duplicate",
                        "layers": [{ "states": [{ "type": "entry" }] }]
                    }
                },
                {
                    "id": "second",
                    "value": {
                        "name": "duplicate",
                        "layers": [{ "states": [{ "type": "entry" }] }]
                    }
                }
            ]
        });

        let error = lower(&input).expect_err("duplicate runtime names must fail");
        assert!(has_diagnostic(
            &error,
            "runtime_name_collision",
            "$.behavior.raw_state_machines[1].value"
        ));
    }

    #[test]
    fn unsupported_authoring_version_is_reported_at_authored_path() {
        let mut input = document();
        input["authoring_format_version"] = json!(1);

        let error = lower(&input).expect_err("unsupported version must fail");
        assert!(has_diagnostic(
            &error,
            "unsupported_version",
            "$.authoring_format_version"
        ));
    }

    #[test]
    fn malformed_raw_scene_object_is_rejected_at_authored_path() {
        let mut input = document();
        input["visual"]["nodes"] = json!([
            {
                "kind": "raw_scene_object",
                "id": "raw",
                "object": []
            }
        ]);

        let error = lower(&input).expect_err("raw escape must be an object");
        assert!(has_diagnostic(
            &error,
            "invalid_raw_scene_object",
            "$.visual.nodes[0].object"
        ));
    }

    #[test]
    fn raw_scene_object_can_lower_when_it_is_object_shaped() {
        let mut input = document();
        input["visual"]["nodes"] = json!([
            {
                "kind": "raw_scene_object",
                "id": "raw",
                "object": {
                    "type": "node",
                    "name": "raw_node",
                    "children": []
                }
            }
        ]);

        let lowered = lower(&input).expect("object-shaped raw escape must lower");
        assert_builds(lowered.scene);
    }
}
