use rive_cli::compare;

pub(super) fn json_error(command: &str, code: &str, message: impl std::fmt::Display) -> ! {
    let envelope = serde_json::json!({
        "ok": false,
        "command": command,
        "code": code,
        "message": message.to_string(),
    });
    eprintln!("{}", envelope);
    std::process::exit(1);
}

pub(super) fn json_success<T: serde::Serialize>(command: &str, value: &T) {
    let mut output = serde_json::to_value(value).unwrap_or_else(|e| {
        json_error(
            command,
            "encode-failed",
            format!("JSON serialization failed: {}", e),
        );
    });
    if let Some(object) = output.as_object_mut() {
        object.insert("ok".to_owned(), serde_json::Value::Bool(true));
    }
    match serde_json::to_string_pretty(&output) {
        Ok(text) => println!("{}", text),
        Err(e) => json_error(
            command,
            "encode-failed",
            format!("JSON serialization failed: {}", e),
        ),
    }
}
pub(super) fn json_compare_threshold_failure(
    report: &compare::CompareReport,
    threshold: f64,
    message: &str,
) -> ! {
    let mut output = serde_json::to_value(report).unwrap_or_else(|error| {
        json_error(
            "compare",
            "encode-failed",
            format!("JSON serialization failed: {error}"),
        );
    });
    if let Some(object) = output.as_object_mut() {
        object.insert("ok".to_owned(), serde_json::Value::Bool(false));
        object.insert(
            "command".to_owned(),
            serde_json::Value::String("compare".to_owned()),
        );
        object.insert(
            "code".to_owned(),
            serde_json::Value::String("pixel-diff-threshold".to_owned()),
        );
        object.insert(
            "message".to_owned(),
            serde_json::Value::String(message.to_owned()),
        );
        object.insert(
            "max_pixel_diff_threshold".to_owned(),
            serde_json::json!(threshold),
        );
    }
    match serde_json::to_string_pretty(&output) {
        Ok(text) => {
            eprintln!("{text}");
            std::process::exit(1);
        }
        Err(error) => json_error(
            "compare",
            "encode-failed",
            format!("JSON serialization failed: {error}"),
        ),
    }
}
