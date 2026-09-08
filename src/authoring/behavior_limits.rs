use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec};

const MAX_BEHAVIOR_COLLECTION_ITEMS: usize = 1000;

pub(crate) fn validate_behavior_limits(spec: &AuthoringSpec) -> Result<(), AuthoringError> {
    let count = spec.behavior.models.len();
    if count > MAX_BEHAVIOR_COLLECTION_ITEMS {
        return Err(AuthoringError::one(AuthoringDiagnostic::new(
            "$.behavior.models",
            "invalid_behavior_collection_count",
            format!(
                "behavior collection has {count} items; expected between 0 and {MAX_BEHAVIOR_COLLECTION_ITEMS}"
            ),
        )));
    }
    Ok(())
}
