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
    AuthoringSourceMap, AuthoringSpec, BehaviorBindingSpec, BehaviorModelSpec, BehaviorSection,
    BehaviorStatechartSpec, ComponentSpec, ConstraintAxis, ConstraintSpec, GradientKind,
    GradientPaintSpec, GradientStopSpec, LoweredAuthoring, MotionContinuity, MotionEasingSpec,
    MotionInterpolation, MotionLoop, MotionSection, MotionTrackSpec, MotionWaypoint, PaintSpec,
    PoseKeyframeSpec, PoseSpec,
    PoseTargetSpec, Quantity, RawSceneFragment, ScalarExpr, SourceMapEntry, StackingSpec,
    StrokeSpec, TransformSpec, Unit, VisualSection,
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

    #[test]
    fn acyclic_component_depth_is_bounded() {
        let mut input = document();
        let components = (0..65)
            .map(|index| {
                let visual = if index == 64 {
                    json!([
                        {
                            "kind": "ellipse",
                            "id": "leaf",
                            "width": { "kind": "literal", "value": 1.0, "unit": "px" },
                            "height": { "kind": "literal", "value": 1.0, "unit": "px" },
                            "fill": "#000000"
                        }
                    ])
                } else {
                    json!([
                        {
                            "kind": "instance",
                            "id": format!("next-{index}"),
                            "component": format!("component-{}", index + 1)
                        }
                    ])
                };
                json!({
                    "id": format!("component-{index}"),
                    "visual": visual
                })
            })
            .collect::<Vec<_>>();
        input["components"] = json!(components);
        input["visual"]["nodes"] = json!([
            { "kind": "instance", "id": "root", "component": "component-0" }
        ]);

        let error = lower(&input).expect_err("deep acyclic expansion must be bounded");
        assert!(has_diagnostic(
            &error,
            "component_expansion_depth_limit",
            "$.components[63].visual[0].component"
        ));
    }

    #[test]
    fn generated_component_nodes_have_a_total_budget() {
        let mut input = document();
        let component_nodes = (0..100)
            .map(|index| {
                json!({
                    "kind": "ellipse",
                    "id": format!("dot-{index}"),
                    "width": { "kind": "literal", "value": 1.0, "unit": "px" },
                    "height": { "kind": "literal", "value": 1.0, "unit": "px" },
                    "fill": "#000000"
                })
            })
            .collect::<Vec<_>>();
        input["components"] = json!([
            { "id": "field", "visual": component_nodes }
        ]);
        let instances = (0..101)
            .map(|index| {
                json!({
                    "kind": "instance",
                    "id": format!("field-{index}"),
                    "component": "field"
                })
            })
            .collect::<Vec<_>>();
        input["visual"]["nodes"] = json!(instances);

        let error = lower(&input).expect_err("generated nodes must have a total budget");
        assert!(
            error
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "component_expansion_node_limit")
        );
    }

    #[test]
    fn common_shape_primitives_lower_deterministically_and_build() {
        let mut input = document();
        input["visual"]["nodes"] = json!([
            {
                "kind": "triangle",
                "id": "triangle",
                "width": { "kind": "literal", "value": 72.0, "unit": "px" },
                "height": { "kind": "literal", "value": 64.0, "unit": "px" },
                "fill": "#EF4444",
                "transform": {
                    "x": { "kind": "literal", "value": 64.0, "unit": "px" },
                    "y": { "kind": "literal", "value": 120.0, "unit": "px" }
                }
            },
            {
                "kind": "polygon",
                "id": "hexagon",
                "width": { "kind": "literal", "value": 80.0, "unit": "px" },
                "height": { "kind": "literal", "value": 80.0, "unit": "px" },
                "points": 6,
                "corner_radius": { "kind": "literal", "value": 4.0, "unit": "px" },
                "fill": "#22C55E",
                "transform": {
                    "x": { "kind": "literal", "value": 160.0, "unit": "px" },
                    "y": { "kind": "literal", "value": 120.0, "unit": "px" }
                }
            },
            {
                "kind": "star",
                "id": "star",
                "width": { "kind": "literal", "value": 88.0, "unit": "px" },
                "height": { "kind": "literal", "value": 88.0, "unit": "px" },
                "points": 5,
                "inner_radius": { "kind": "literal", "value": 0.42, "unit": "scalar" },
                "corner_radius": { "kind": "literal", "value": 2.0, "unit": "px" },
                "fill": "#3B82F6",
                "transform": {
                    "x": { "kind": "literal", "value": 256.0, "unit": "px" },
                    "y": { "kind": "literal", "value": 120.0, "unit": "px" }
                }
            }
        ]);

        let first = lower(&input).expect("first primitive lowering");
        let second = lower(&input).expect("second primitive lowering");
        assert_eq!(first.scene, second.scene);
        assert_eq!(first.source_map, second.source_map);

        let children = first.scene["artboard"]["children"]
            .as_array()
            .expect("lowered artboard children");
        assert_eq!(children[0]["children"][0]["type"], "triangle");
        assert_eq!(children[1]["children"][0]["type"], "polygon");
        assert_eq!(children[1]["children"][0]["points"], 6);
        assert_eq!(children[1]["children"][0]["corner_radius"], 4.0);
        assert_eq!(children[2]["children"][0]["type"], "star");
        assert_eq!(children[2]["children"][0]["points"], 5);
        assert_eq!(children[2]["children"][0]["inner_radius"], 0.42);

        for authored_id in ["triangle", "hexagon", "star"] {
            let entry = first
                .source_map
                .entries
                .iter()
                .find(|entry| entry.authored_id == authored_id)
                .expect("primitive source-map entry");
            assert_eq!(entry.runtime_names.len(), 4);
            assert_eq!(entry.scene_paths.len(), 4);
        }
        assert_builds(first.scene);
    }

    #[test]
    fn polygons_and_stars_require_at_least_three_points() {
        for kind in ["polygon", "star"] {
            let mut input = document();
            let mut node = json!({
                "kind": kind,
                "id": kind,
                "width": { "kind": "literal", "value": 40.0, "unit": "px" },
                "height": { "kind": "literal", "value": 40.0, "unit": "px" },
                "points": 2,
                "fill": "#111827"
            });
            if kind == "star" {
                node["inner_radius"] = json!({ "kind": "literal", "value": 0.5, "unit": "scalar" });
            }
            input["visual"]["nodes"] = json!([node]);

            let error = lower(&input).expect_err("too few points must fail");
            assert!(has_diagnostic(
                &error,
                "invalid_points",
                "$.visual.nodes[0].points"
            ));
        }
    }

    #[test]
    fn star_inner_radius_requires_a_bounded_scalar_ratio() {
        for (value, unit, expected_code) in [
            (0.5, "px", "unit_mismatch"),
            (1.1, "scalar", "invalid_ratio"),
        ] {
            let mut input = document();
            input["visual"]["nodes"] = json!([
                {
                    "kind": "star",
                    "id": "star",
                    "width": { "kind": "literal", "value": 40.0, "unit": "px" },
                    "height": { "kind": "literal", "value": 40.0, "unit": "px" },
                    "points": 5,
                    "inner_radius": { "kind": "literal", "value": value, "unit": unit },
                    "fill": "#111827"
                }
            ]);

            let error = lower(&input).expect_err("invalid inner radius must fail");
            assert!(has_diagnostic(
                &error,
                expected_code,
                "$.visual.nodes[0].inner_radius"
            ));
        }
    }
}
