mod model;
mod runner;
mod runtime;
mod semantic;
mod traits;
mod validation;

pub use model::{
    AnimatedSemanticCheck, EvalBaseline, EvalCase, EvalCaseReport, EvalDiagnosticEvidence,
    EvalFailureStage, EvalGates, EvalReport, EvalSuite, InputKind, RuntimeEvidence,
    RuntimeExpectations, SemanticCheckEvidence, SemanticEvidence, SemanticExpectations,
    StaticSemanticCheck,
};
pub use runner::{run_eval_suite, run_eval_suite_configured};

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashSet};

    use super::model::{
        AnimatedSemanticCheck, EvalBaseline, EvalCase, EvalGates, EvalSuite, InputKind,
        RuntimeExpectations, SemanticExpectations, StaticSemanticCheck,
    };
    use super::runner::{test_hash_bytes, test_run_id};
    use super::runtime::evaluate_runtime_frames;
    use super::semantic::evaluate_semantics;
    use super::traits::trait_score;
    use super::validation::{
        evaluate_gates, resolve_eval_config, validate_baseline, validate_suite,
    };
    use crate::authoring::{AuthoringSourceMap, SourceMapEntry};
    use crate::render::RenderedFrame;

    fn test_case(id: &str) -> EvalCase {
        EvalCase {
            id: id.to_string(),
            input_kind: InputKind::Template,
            input: "bounce".to_string(),
            expected_traits: vec!["has_animation".to_string()],
            text_hint: None,
            image_path: None,
            runtime: None,
            semantic: None,
        }
    }

    fn test_suite(cases: Vec<EvalCase>) -> EvalSuite {
        EvalSuite {
            suite_name: "contract".to_string(),
            suite_version: 1,
            gates: EvalGates::default(),
            cases,
        }
    }

    #[test]
    fn style_score_empty_expectations() {
        let scene = serde_json::json!({"scene_format_version": 1});
        let (score, matched) = trait_score(&scene, &[]);
        assert_eq!(score, 1.0);
        assert!(matched.is_empty());
    }

    #[test]
    fn style_score_matches_animation_trait() {
        let scene = serde_json::json!({
            "scene_format_version": 1,
            "artboard": {
                "name": "A",
                "width": 100,
                "height": 100,
                "children": [],
                "animations": [{"name": "anim", "fps": 60, "duration": 10, "keyframes": []}]
            }
        });
        let expected = vec!["has_animation".to_string(), "has_state_machine".to_string()];
        let (score, matched) = trait_score(&scene, &expected);
        assert!(score > 0.4 && score < 0.6);
        assert_eq!(matched, vec!["has_animation".to_string()]);
    }

    #[test]
    fn semantic_static_checks_use_authored_identity_and_runtime_type() {
        let scene = serde_json::json!({
            "artboard": {
                "children": [{"type": "rectangle", "name": "runtime-panel"}]
            }
        });
        let source_map = AuthoringSourceMap {
            entries: vec![SourceMapEntry {
                authored_id: "panel".to_string(),
                authored_path: "$.visual.nodes[0]".to_string(),
                definition_path: None,
                runtime_names: vec!["runtime-panel".to_string()],
                scene_paths: vec!["$.artboard.children[0]".to_string()],
            }],
        };
        let expectations = SemanticExpectations {
            static_checks: vec![
                StaticSemanticCheck::AuthoredIdPresent {
                    authored_id: "panel".to_string(),
                },
                StaticSemanticCheck::AuthoredIdHasRuntimeType {
                    authored_id: "panel".to_string(),
                    object_type: "rectangle".to_string(),
                },
            ],
            animated_checks: Vec::new(),
        };
        let evidence = evaluate_semantics(
            std::path::Path::new("unused"),
            &scene,
            &source_map,
            &expectations,
        );
        assert_eq!(evidence.static_passed, Some(true));
        assert_eq!(evidence.animated_passed, None);
    }

    #[test]
    fn semantic_animated_check_compares_retained_frames() {
        let root = std::env::temp_dir().join(format!(
            "rive_semantic_frames_{}_{}",
            std::process::id(),
            test_run_id().unwrap()
        ));
        let render = root.join("render");
        std::fs::create_dir_all(&render).unwrap();
        std::fs::write(render.join("frame_00000.png"), b"frame-a").unwrap();
        std::fs::write(render.join("frame_00030.png"), b"frame-b").unwrap();
        let source_map = AuthoringSourceMap::default();
        let expectations = SemanticExpectations {
            static_checks: Vec::new(),
            animated_checks: vec![AnimatedSemanticCheck::FramesDiffer {
                from: 0,
                to: 30,
            }],
        };
        let evidence = evaluate_semantics(
            &root,
            &serde_json::json!({}),
            &source_map,
            &expectations,
        );
        assert_eq!(evidence.static_passed, None);
        assert_eq!(evidence.animated_passed, Some(true));
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn hash_bytes_is_deterministic() {
        let input = b"deterministic test input";
        assert_eq!(test_hash_bytes(input), test_hash_bytes(input));
    }

    #[test]
    fn hash_bytes_has_known_value() {
        assert_eq!(
            test_hash_bytes(b"test input"),
            "9dfe6f15d1ab73af898739394fd22fd72a03db01834582f24bb2e1c66c7aaeae"
        );
    }

    #[test]
    fn hash_bytes_changes_with_input() {
        assert_ne!(test_hash_bytes(b"alpha"), test_hash_bytes(b"beta"));
    }

    #[test]
    fn run_id_is_unique_across_rapid_generation() {
        let ids = (0..100)
            .map(|_| test_run_id().unwrap())
            .collect::<HashSet<_>>();
        assert_eq!(ids.len(), 100);
    }

    #[test]
    fn run_id_has_timestamp_and_hex_suffix() {
        let id = test_run_id().unwrap();
        let parts = id.split('_').collect::<Vec<_>>();
        assert_eq!(parts.len(), 2);
        assert!(parts[0].parse::<u128>().is_ok());
        assert_eq!(parts[1].len(), 16);
        assert!(
            parts[1]
                .chars()
                .all(|character| character.is_ascii_hexdigit())
        );
    }

    #[test]
    fn validate_suite_rejects_unsafe_case_id() {
        let error = validate_suite(&test_suite(vec![test_case("../escape")])).unwrap_err();
        assert!(error.contains("case id"));
    }

    #[test]
    fn validate_suite_rejects_duplicate_case_ids() {
        let error = validate_suite(&test_suite(vec![test_case("bounce"), test_case("bounce")]))
            .unwrap_err();
        assert!(error.contains("duplicate"));
    }

    #[test]
    fn validate_suite_rejects_unknown_traits() {
        let mut case = test_case("bounce");
        case.expected_traits = vec!["looks_professional".to_string()];
        let error = validate_suite(&test_suite(vec![case])).unwrap_err();
        assert!(error.contains("unsupported expected trait"));
    }

    #[test]
    fn semantic_checks_require_authoring_input() {
        let mut case = test_case("semantic");
        case.semantic = Some(SemanticExpectations {
            static_checks: vec![StaticSemanticCheck::AuthoredIdPresent {
                authored_id: "panel".to_string(),
            }],
            animated_checks: Vec::new(),
        });
        let error = validate_suite(&test_suite(vec![case])).unwrap_err();
        assert!(error.contains("authoring_example"));
    }

    #[test]
    fn animated_semantic_checks_must_reference_runtime_frames() {
        let mut case = test_case("semantic-motion");
        case.input_kind = InputKind::AuthoringExample;
        case.input = "examples/authoring/complex-animated-showcase.v0.json".to_string();
        case.runtime = Some(RuntimeExpectations {
            frames: vec![0, 30],
            min_non_blank_frames: 2,
            ..RuntimeExpectations::default()
        });
        case.semantic = Some(SemanticExpectations {
            static_checks: Vec::new(),
            animated_checks: vec![AnimatedSemanticCheck::FramesDiffer { from: 0, to: 60 }],
        });
        let error = validate_suite(&test_suite(vec![case])).unwrap_err();
        assert!(error.contains("must both be rendered"));
    }

    #[test]
    fn validate_baseline_requires_matching_complete_case_set() {
        let suite = test_suite(vec![test_case("bounce"), test_case("spinner")]);
        let baseline = EvalBaseline {
            suite_name: "other".to_string(),
            suite_version: 1,
            case_hashes: BTreeMap::from([("bounce".to_string(), "a".repeat(64))]),
        };
        assert!(
            validate_baseline(&suite, &baseline)
                .unwrap_err()
                .contains("suite name")
        );

        let baseline = EvalBaseline {
            suite_name: suite.suite_name.clone(),
            suite_version: suite.suite_version,
            case_hashes: BTreeMap::from([("bounce".to_string(), "a".repeat(64))]),
        };
        assert!(
            validate_baseline(&suite, &baseline)
                .unwrap_err()
                .contains("missing case")
        );
    }

    #[test]
    fn eval_gates_fail_every_breached_metric() {
        let gates = EvalGates {
            min_validity_rate: 1.0,
            min_trait_adherence_rate: 0.9,
            min_pipeline_reproducibility_rate: 1.0,
            min_runtime_pass_rate: 1.0,
            min_semantic_static_pass_rate: 1.0,
            min_semantic_animated_pass_rate: 1.0,
            max_average_retries: Some(1.0),
            max_drift_count: 0,
        };
        assert_eq!(
            evaluate_gates(&gates, 0.8, 0.7, 0.5, 0.5, 0.6, 0.4, 2.0, 1).len(),
            8
        );
    }

    #[test]
    fn prompt_suite_rejects_template_provider() {
        let mut case = test_case("prompt");
        case.input_kind = InputKind::Prompt;
        case.input = "a bouncing ball".to_string();
        let error =
            resolve_eval_config(&test_suite(vec![case]), None, Some("template".to_string()))
                .unwrap_err();
        assert!(error.contains("prompt cases require"));
    }

    fn runtime_frame(index: u32, distinct_colors: usize, blank: bool) -> RenderedFrame {
        RenderedFrame {
            index,
            seconds: f64::from(index) / 60.0,
            filename: format!("frame_{index:05}.png"),
            distinct_colors,
            blank,
            preview: None,
        }
    }

    #[test]
    fn runtime_evidence_passes_when_frame_requirements_are_met() {
        let expectations = RuntimeExpectations {
            frames: vec![0, 30],
            min_non_blank_frames: 2,
            min_distinct_colors: 3,
            ..RuntimeExpectations::default()
        };
        let frames = vec![runtime_frame(0, 5, false), runtime_frame(30, 7, false)];
        let evidence = evaluate_runtime_frames(
            &expectations,
            &frames,
            std::path::Path::new("render/manifest.json"),
        );
        assert!(evidence.passed);
        assert_eq!(evidence.rendered_frame_count, 2);
        assert_eq!(evidence.non_blank_frame_count, 2);
        assert_eq!(evidence.minimum_distinct_colors_observed, 5);
        assert!(evidence.failure_reason.is_none());
    }

    #[test]
    fn runtime_evidence_rejects_wrong_frame_indices() {
        let expectations = RuntimeExpectations {
            frames: vec![0, 30],
            min_non_blank_frames: 2,
            min_distinct_colors: 3,
            ..RuntimeExpectations::default()
        };
        let frames = vec![runtime_frame(0, 5, false), runtime_frame(31, 7, false)];
        let evidence = evaluate_runtime_frames(
            &expectations,
            &frames,
            std::path::Path::new("render/manifest.json"),
        );
        assert!(!evidence.passed);
        assert!(
            evidence
                .failure_reason
                .expect("missing frame identity failure")
                .contains("frame indices")
        );
    }

    #[test]
    fn runtime_evidence_keeps_blank_and_colour_failures_separate_from_validity() {
        let expectations = RuntimeExpectations {
            frames: vec![0, 30],
            min_non_blank_frames: 2,
            min_distinct_colors: 4,
            ..RuntimeExpectations::default()
        };
        let frames = vec![runtime_frame(0, 1, true), runtime_frame(30, 3, false)];
        let evidence = evaluate_runtime_frames(
            &expectations,
            &frames,
            std::path::Path::new("render/manifest.json"),
        );
        assert!(!evidence.passed);
        let failure = evidence.failure_reason.expect("missing runtime failure");
        assert!(failure.contains("non-blank"));
        assert!(failure.contains("distinct colors"));
    }

    #[test]
    fn validate_suite_rejects_runtime_plan_without_frames() {
        let mut case = test_case("runtime");
        case.runtime = Some(RuntimeExpectations {
            frames: Vec::new(),
            ..RuntimeExpectations::default()
        });
        let error = validate_suite(&test_suite(vec![case])).unwrap_err();
        assert!(error.contains("runtime frames"));
    }

    #[test]
    fn ci_runs_official_runtime_and_authoring_semantic_suites() {
        let ci = include_str!("../../../.github/workflows/ci.yml");
        assert!(ci.contains("evals/suites/runtime_contract.v1.json"));
        assert!(ci.contains("evals/suites/authoring_semantic.v1.json"));
        assert!(ci.contains("runtime-eval-evidence"));
    }
}
