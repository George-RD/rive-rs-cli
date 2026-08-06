use rive_cli::authoring::{MotionEasingSpec, MotionSection, ScalarExpr, Unit};

fn scalar(value: f64) -> ScalarExpr {
    ScalarExpr::Literal {
        value,
        unit: Unit::Scalar,
    }
}

#[test]
fn typed_authoring_api_exposes_shared_easings() {
    let motion = MotionSection {
        easings: vec![MotionEasingSpec::Cubic {
            id: "soft-out".to_string(),
            x1: scalar(0.16),
            y1: scalar(1.0),
            x2: scalar(0.3),
            y2: scalar(1.0),
        }],
        ..MotionSection::default()
    };

    assert_eq!(motion.easings.len(), 1);
}
