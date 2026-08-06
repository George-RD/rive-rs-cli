from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    content = file.read_text()
    if content.count(old) != 1:
        raise SystemExit(f"expected one anchor in {path!r}, found {content.count(old)}")
    file.write_text(content.replace(old, new, 1))


replace_once(
    "src/authoring/mod.rs",
    "    MotionInterpolation, MotionLoop, MotionSection, MotionTrackSpec, PaintSpec, PoseKeyframeSpec,\n",
    "    MotionEasingSpec, MotionInterpolation, MotionLoop, MotionSection, MotionTrackSpec, PaintSpec,\n    PoseKeyframeSpec,\n",
)

replace_once(
    "Cargo.toml",
    'edition = "2024"\n',
    'edition = "2024"\nrust-version = "1.88"\n',
)

replace_once(
    "AGENTS.md",
    "- **Edition 2024** (requires Rust 1.84+)",
    "- **Edition 2024** with a declared Rust 1.88 minimum",
)

motion_path = Path("src/authoring/frontend/motion.rs")
motion = motion_path.read_text()
motion = motion.replace("mod easing;\nmod validation;\n", "mod easing;\nmod timing;\nmod validation;\n", 1)
motion = motion.replace(
    "    MotionInterpolation, MotionLoop, Quantity, RawSceneFragment, ScalarExpr, TransformSpec, Unit,\n",
    "    MotionInterpolation, MotionLoop, RawSceneFragment, TransformSpec, Unit,\n",
    1,
)
motion = motion.replace(
    "use easing::{EasingEmission, ResolvedEasing};\n\npub(super) use validation::validate_motion;\n",
    "use easing::{EasingEmission, ResolvedEasing};\nuse timing::evaluate_frame_value;\n\npub(super) use validation::validate_motion;\n",
    1,
)
motion = motion.replace(
    "const FRAME_ROUNDING_ULPS: f64 = 8.0;\nconst HALF_FRAME: f64 = 0.5;\nconst MAX_FRAME_ROUNDING_WINDOW: f64 = 1e-9;\nconst WHOLE_FRAME: f64 = 1.0;\n\n",
    "",
    1,
)
start = motion.index("\nfn frame_rounding_tolerance(")
end = motion.index("\nfn rewrite_motion_error_paths", start)
motion = motion[:start] + motion[end:]
motion_path.write_text(motion)

Path("src/authoring/frontend/motion/timing.rs").write_text(
    '''use std::collections::BTreeMap;

use super::super::super::expression::evaluate_expression;
use super::super::super::spec::{AuthoringDiagnostic, Quantity, ScalarExpr, Unit};

const FRAME_ROUNDING_ULPS: f64 = 8.0;
const HALF_FRAME: f64 = 0.5;
const MAX_FRAME_ROUNDING_WINDOW: f64 = 1e-9;
const WHOLE_FRAME: f64 = 1.0;

pub(super) fn evaluate_frame_value(
    expression: &ScalarExpr,
    path: &str,
    scope: &BTreeMap<String, Quantity>,
    code: &str,
    message: &str,
) -> Result<u64, AuthoringDiagnostic> {
    let value = evaluate_expression(expression, path, scope, Unit::Scalar)?;
    let rounded = value.round();
    if !value.is_finite() || rounded < 0.0 || rounded >= u64::MAX as f64 {
        return Err(AuthoringDiagnostic::new(path, code, message));
    }
    let Some(rounding_tolerance) = frame_rounding_tolerance(value) else {
        return Err(AuthoringDiagnostic::new(path, code, message));
    };
    if (value - rounded).abs() > rounding_tolerance {
        return Err(AuthoringDiagnostic::new(path, code, message));
    }
    Ok(rounded as u64)
}

fn frame_rounding_tolerance(value: f64) -> Option<f64> {
    let magnitude = value.abs().max(1.0);
    let one_ulp = f64::from_bits(magnitude.to_bits() + 1) - magnitude;
    if one_ulp >= WHOLE_FRAME {
        return None;
    }
    if one_ulp >= HALF_FRAME {
        return Some(0.0);
    }
    Some(
        (one_ulp * FRAME_ROUNDING_ULPS)
            .min(MAX_FRAME_ROUNDING_WINDOW)
            .max(one_ulp),
    )
}
'''
)

replace_once(
    "cairn.blueprint",
    '"./tests/authoring_motion_easing_contract.rs", "./tests/authoring_motion_expansion_contract.rs",',
    '"./tests/authoring_motion_easing_contract.rs", "./tests/authoring_motion_expansion_contract.rs", "./tests/authoring_motion_typed_api_contract.rs",',
)

replace_once(
    "meta/todos/todo.motion-authoring-compiler.md",
    "- implementation reuses the canonical SceneSpec interpolator path rather than adding encoder logic or exposing runtime indices.\n",
    """- implementation reuses the canonical SceneSpec interpolator path rather than adding encoder logic or exposing runtime indices.

Review hardening for PR #163 remains within this P2 slice:

- exact-head RED `ed94807` in CI run `31092625558` proves the JSON contract was exposed while Rust callers could not name `MotionEasingSpec` through the public typed API;
- the Rust 1.85 compatibility probe at `8ce6f23` in workflow run `31093299020` instead stopped at the locked dependency graph because `darling` 0.23 already requires Rust 1.88, so the repository now declares and continuously checks its actual Rust 1.88 minimum;
- implementation `FIX_SHA_PENDING` publicly re-exports `MotionEasingSpec`, retains the typed construction regression under Cairn ownership, and extracts frame timing into `motion/timing.rs` so this slice no longer pushes `motion.rs` beyond the module-size guideline;
- preflight workflow run `REVIEW_FIX_RUN_ID` passed the typed API regression, Rust 1.88 library check, formatting, Clippy, Cairn scan, and Cairn lint before durable evidence was committed;
- the existing oversized warnings for `constraint.rs`, `visual.rs`, and `lower.rs` predate this slice and remain separate architecture cleanup rather than expanding the easing PR.
""",
)
