use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec, BehaviorTransitionSpec};

const MAX_BEHAVIOR_COLLECTION_ITEMS: usize = 1000;

pub(crate) fn validate_behavior_limits(spec: &AuthoringSpec) -> Result<(), AuthoringError> {
    let behavior = &spec.behavior;
    validate_count(behavior.models.len(), 0, "$.behavior.models")?;
    validate_count(behavior.bindings.len(), 0, "$.behavior.bindings")?;
    validate_count(behavior.statecharts.len(), 0, "$.behavior.statecharts")?;

    for (index, model) in behavior.models.iter().enumerate() {
        validate_count(
            model.properties.len(),
            0,
            &format!("$.behavior.models[{index}].properties"),
        )?;
    }

    for (index, chart) in behavior.statecharts.iter().enumerate() {
        let path = format!("$.behavior.statecharts[{index}]");
        for (field, count, minimum) in [
            ("inputs", chart.inputs.len(), 0),
            ("events", chart.events.len(), 0),
            ("listeners", chart.listeners.len(), 0),
            ("states", chart.states.len(), 1),
            ("transitions", chart.transitions.len(), 0),
            ("regions", chart.regions.len(), 0),
        ] {
            validate_count(count, minimum, &format!("{path}.{field}"))?;
        }
        validate_transition_guards(&chart.transitions, &path)?;
        for (listener_index, listener) in chart.listeners.iter().enumerate() {
            validate_count(
                listener.actions.len(),
                0,
                &format!("{path}.listeners[{listener_index}].actions"),
            )?;
        }
        for (region_index, region) in chart.regions.iter().enumerate() {
            let region_path = format!("{path}.regions[{region_index}]");
            validate_count(region.states.len(), 1, &format!("{region_path}.states"))?;
            validate_count(
                region.transitions.len(),
                0,
                &format!("{region_path}.transitions"),
            )?;
            validate_transition_guards(&region.transitions, &region_path)?;
        }
    }
    Ok(())
}

fn validate_transition_guards(
    transitions: &[BehaviorTransitionSpec],
    path: &str,
) -> Result<(), AuthoringError> {
    for (index, transition) in transitions.iter().enumerate() {
        validate_count(
            transition.when.conditions().len(),
            1,
            &format!("{path}.transitions[{index}].when.all"),
        )?;
    }
    Ok(())
}

fn validate_count(count: usize, minimum: usize, path: &str) -> Result<(), AuthoringError> {
    if !(minimum..=MAX_BEHAVIOR_COLLECTION_ITEMS).contains(&count) {
        return Err(AuthoringError::one(AuthoringDiagnostic::new(
            path,
            "invalid_behavior_collection_count",
            format!(
                "behavior collection has {count} items; expected between {minimum} and {MAX_BEHAVIOR_COLLECTION_ITEMS}"
            ),
        )));
    }
    Ok(())
}
