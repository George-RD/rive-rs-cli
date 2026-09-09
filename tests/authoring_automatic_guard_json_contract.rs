use rive_cli::authoring::lower_authoring_json;
use serde_json::{Value, json};

#[test]
fn automatic_guard_rejects_object_shaped_unit_variant_aliases() {
    let mut document: Value = serde_json::from_str(include_str!(
        "../examples/authoring/automatic-sequence.v0.json"
    ))
    .expect("automatic sequence");
    for guard in [json!({ "always": null }), json!({ "always": [] })] {
        document["behavior"]["statecharts"][0]["transitions"][0]["when"] = guard;
        let error = lower_authoring_json(&document.to_string())
            .expect_err("only the literal string may make a transition unconditional");
        assert!(
            error
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "invalid_json"),
            "{error:?}"
        );
    }
}
