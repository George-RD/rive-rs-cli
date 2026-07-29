mod model;
mod runner;
mod traits;
mod validation;

pub use model::{
    EvalBaseline, EvalCase, EvalCaseReport, EvalGates, EvalReport, EvalSuite, InputKind,
};
pub use runner::{run_eval_suite, run_eval_suite_configured};

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashSet};

    use super::model::{EvalBaseline, EvalCase, EvalGates, EvalSuite, InputKind};
    use super::runner::{test_hash_bytes, test_run_id};
    use super::traits::trait_score;
    use super::validation::{
        evaluate_gates, resolve_eval_config, validate_baseline, validate_suite,
    };

    fn test_case(id: &str) -> EvalCase {
        EvalCase {
            id: id.to_string(),
            input_kind: InputKind::Template,
            input: "bounce".to_string(),
            expected_traits: vec!["has_animation".to_string()],
            text_hint: None,
            image_path: None,
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
            max_average_retries: Some(1.0),
            max_drift_count: 0,
        };
        assert_eq!(evaluate_gates(&gates, 0.8, 0.7, 0.5, 2.0, 1).len(), 5);
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
}
