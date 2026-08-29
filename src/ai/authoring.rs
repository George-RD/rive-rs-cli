use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::authoring::{
    AuthoringContainer, AuthoringDiagnostic, AuthoringEntity, AuthoringOperation, AuthoringPlacement,
    AuthoringSourceMap, AuthoringSpec, AuthoringTarget, BehaviorBindingSpec, BehaviorModelSpec,
    BehaviorStatechartSpec, ComponentSpec, LoweredAuthoring, MotionEasingSpec, MotionTrackSpec,
    PoseSpec, RawSceneFragment, VisualNode, apply_operation, lower_authoring,
};

use super::provider::AiProvider;
use super::{AiConfig, AiError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationTarget {
    Authoring,
    Scene,
}

impl GenerationTarget {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoring => "authoring",
            Self::Scene => "scene",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum AiAuthoringEntity {
    VisualNode(Box<VisualNode>),
    Component(ComponentSpec),
    MotionEasing(MotionEasingSpec),
    MotionPose(PoseSpec),
    MotionTrack(MotionTrackSpec),
    MotionRawAnimation(RawSceneFragment),
    BehaviorModel(BehaviorModelSpec),
    BehaviorBinding(BehaviorBindingSpec),
    BehaviorStatechart(BehaviorStatechartSpec),
    BehaviorRawStateMachine(RawSceneFragment),
}

impl From<AiAuthoringEntity> for AuthoringEntity {
    fn from(value: AiAuthoringEntity) -> Self {
        match value {
            AiAuthoringEntity::VisualNode(node) => Self::VisualNode(node),
            AiAuthoringEntity::Component(component) => Self::Component(component),
            AiAuthoringEntity::MotionEasing(easing) => Self::MotionEasing(easing),
            AiAuthoringEntity::MotionPose(pose) => Self::MotionPose(pose),
            AiAuthoringEntity::MotionTrack(track) => Self::MotionTrack(track),
            AiAuthoringEntity::MotionRawAnimation(fragment) => Self::MotionRawAnimation(fragment),
            AiAuthoringEntity::BehaviorModel(model) => Self::BehaviorModel(model),
            AiAuthoringEntity::BehaviorBinding(binding) => Self::BehaviorBinding(binding),
            AiAuthoringEntity::BehaviorStatechart(statechart) => {
                Self::BehaviorStatechart(statechart)
            }
            AiAuthoringEntity::BehaviorRawStateMachine(fragment) => {
                Self::BehaviorRawStateMachine(fragment)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AiAuthoringTarget {
    VisualNode { target_id: String },
    Component { target_id: String },
    MotionEasing { target_id: String },
    MotionPose { target_id: String },
    MotionTrack { target_id: String },
    MotionRawAnimation { target_id: String },
    BehaviorModel { target_id: String },
    BehaviorBinding { target_id: String },
    BehaviorStatechart { target_id: String },
    BehaviorRawStateMachine { target_id: String },
}

impl From<AiAuthoringTarget> for AuthoringTarget {
    fn from(value: AiAuthoringTarget) -> Self {
        match value {
            AiAuthoringTarget::VisualNode { target_id } => Self::VisualNode { target_id },
            AiAuthoringTarget::Component { target_id } => Self::Component { target_id },
            AiAuthoringTarget::MotionEasing { target_id } => Self::MotionEasing { target_id },
            AiAuthoringTarget::MotionPose { target_id } => Self::MotionPose { target_id },
            AiAuthoringTarget::MotionTrack { target_id } => Self::MotionTrack { target_id },
            AiAuthoringTarget::MotionRawAnimation { target_id } => {
                Self::MotionRawAnimation { target_id }
            }
            AiAuthoringTarget::BehaviorModel { target_id } => Self::BehaviorModel { target_id },
            AiAuthoringTarget::BehaviorBinding { target_id } => Self::BehaviorBinding { target_id },
            AiAuthoringTarget::BehaviorStatechart { target_id } => {
                Self::BehaviorStatechart { target_id }
            }
            AiAuthoringTarget::BehaviorRawStateMachine { target_id } => {
                Self::BehaviorRawStateMachine { target_id }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AiAuthoringContainer {
    VisualRoot,
    VisualGroup { target_id: String },
    Components,
    MotionEasings,
    MotionPoses,
    MotionTracks,
    MotionRawAnimations,
    BehaviorModels,
    BehaviorBindings,
    BehaviorStatecharts,
    BehaviorRawStateMachines,
}

impl From<AiAuthoringContainer> for AuthoringContainer {
    fn from(value: AiAuthoringContainer) -> Self {
        match value {
            AiAuthoringContainer::VisualRoot => Self::VisualRoot,
            AiAuthoringContainer::VisualGroup { target_id } => Self::VisualGroup { target_id },
            AiAuthoringContainer::Components => Self::Components,
            AiAuthoringContainer::MotionEasings => Self::MotionEasings,
            AiAuthoringContainer::MotionPoses => Self::MotionPoses,
            AiAuthoringContainer::MotionTracks => Self::MotionTracks,
            AiAuthoringContainer::MotionRawAnimations => Self::MotionRawAnimations,
            AiAuthoringContainer::BehaviorModels => Self::BehaviorModels,
            AiAuthoringContainer::BehaviorBindings => Self::BehaviorBindings,
            AiAuthoringContainer::BehaviorStatecharts => Self::BehaviorStatecharts,
            AiAuthoringContainer::BehaviorRawStateMachines => Self::BehaviorRawStateMachines,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AiAuthoringPlacement {
    Into { container: AiAuthoringContainer },
    Before { anchor: AiAuthoringTarget },
    After { anchor: AiAuthoringTarget },
}

impl From<AiAuthoringPlacement> for AuthoringPlacement {
    fn from(value: AiAuthoringPlacement) -> Self {
        match value {
            AiAuthoringPlacement::Into { container } => Self::Into {
                container: container.into(),
            },
            AiAuthoringPlacement::Before { anchor } => Self::Before {
                anchor: anchor.into(),
            },
            AiAuthoringPlacement::After { anchor } => Self::After {
                anchor: anchor.into(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum AiAuthoringOperation {
    ReplaceVisualNode {
        target_id: String,
        node: VisualNode,
    },
    Insert {
        entity: AiAuthoringEntity,
        placement: AiAuthoringPlacement,
    },
    Move {
        target: AiAuthoringTarget,
        placement: AiAuthoringPlacement,
    },
    Remove {
        target: AiAuthoringTarget,
    },
}

impl From<AiAuthoringOperation> for AuthoringOperation {
    fn from(value: AiAuthoringOperation) -> Self {
        match value {
            AiAuthoringOperation::ReplaceVisualNode { target_id, node } => {
                Self::ReplaceVisualNode { target_id, node }
            }
            AiAuthoringOperation::Insert { entity, placement } => Self::Insert {
                entity: entity.into(),
                placement: placement.into(),
            },
            AiAuthoringOperation::Move { target, placement } => Self::Move {
                target: target.into(),
                placement: placement.into(),
            },
            AiAuthoringOperation::Remove { target } => Self::Remove {
                target: target.into(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AuthoringRepairEnvelope {
    pub operation: AiAuthoringOperation,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthoringRepairRequest {
    pub authoring_spec: Value,
    pub diagnostics: Vec<AuthoringDiagnostic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_map: Option<AuthoringSourceMap>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthoringRepairAttempt {
    pub attempt: u8,
    pub diagnostics: Vec<AuthoringDiagnostic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<AiAuthoringOperation>,
    pub succeeded: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthoringRepairResult {
    pub authoring_json: Value,
    #[serde(skip)]
    pub lowered: LoweredAuthoring,
    pub attempts: Vec<AuthoringRepairAttempt>,
    pub total_retries: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthoringRepairFailure {
    pub message: String,
    pub diagnostics: Vec<AuthoringDiagnostic>,
    pub attempts: Vec<AuthoringRepairAttempt>,
}

impl std::fmt::Display for AuthoringRepairFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AuthoringRepairFailure {}

pub fn authoring_repair_schema() -> Value {
    serde_json::to_value(schema_for!(AuthoringRepairEnvelope))
        .unwrap_or_else(|_| Value::Object(serde_json::Map::new()))
}

pub fn repair_authoring_spec(
    provider: &dyn AiProvider,
    config: &AiConfig,
    initial_json: Value,
    max_retries: u8,
) -> Result<AuthoringRepairResult, AuthoringRepairFailure> {
    let mut spec = serde_json::from_value::<AuthoringSpec>(initial_json).map_err(|error| {
        let diagnostic = AuthoringDiagnostic {
            path: "$".to_string(),
            code: "invalid_json".to_string(),
            message: error.to_string(),
        };
        AuthoringRepairFailure {
            message: format!("generated AuthoringSpec is invalid: {error}"),
            diagnostics: vec![diagnostic],
            attempts: Vec::new(),
        }
    })?;

    let mut attempts = Vec::new();
    let mut diagnostics = match lower_authoring(&spec) {
        Ok(lowered) => {
            attempts.push(AuthoringRepairAttempt {
                attempt: 0,
                diagnostics: Vec::new(),
                operation: None,
                succeeded: true,
            });
            return Ok(AuthoringRepairResult {
                authoring_json: serde_json::to_value(&spec).map_err(|error| {
                    serialization_failure(error.to_string(), attempts.clone())
                })?,
                lowered,
                attempts,
                total_retries: 0,
            });
        }
        Err(error) => error.diagnostics,
    };

    attempts.push(AuthoringRepairAttempt {
        attempt: 0,
        diagnostics: diagnostics.clone(),
        operation: None,
        succeeded: false,
    });

    for attempt in 1..=max_retries {
        let authoring_spec = serde_json::to_value(&spec)
            .map_err(|error| serialization_failure(error.to_string(), attempts.clone()))?;
        let request = AuthoringRepairRequest {
            authoring_spec,
            diagnostics: diagnostics.clone(),
            source_map: None,
        };
        let repair_json = provider
            .repair_authoring(&request, config)
            .map_err(|error| provider_failure(error, attempts.clone(), diagnostics.clone()))?;
        let envelope = match serde_json::from_value::<AuthoringRepairEnvelope>(repair_json) {
            Ok(envelope) => envelope,
            Err(error) => {
                diagnostics = vec![AuthoringDiagnostic {
                    path: "$.operation".to_string(),
                    code: "invalid_repair_operation".to_string(),
                    message: error.to_string(),
                }];
                attempts.push(AuthoringRepairAttempt {
                    attempt,
                    diagnostics: diagnostics.clone(),
                    operation: None,
                    succeeded: false,
                });
                continue;
            }
        };
        let ai_operation = envelope.operation;
        let operation = ai_operation.clone().into();
        match apply_operation(&spec, &operation) {
            Ok(applied) => {
                spec = applied.spec;
                attempts.push(AuthoringRepairAttempt {
                    attempt,
                    diagnostics: Vec::new(),
                    operation: Some(ai_operation),
                    succeeded: true,
                });
                let authoring_json = serde_json::to_value(&spec)
                    .map_err(|error| serialization_failure(error.to_string(), attempts.clone()))?;
                return Ok(AuthoringRepairResult {
                    authoring_json,
                    lowered: applied.lowered,
                    attempts,
                    total_retries: attempt,
                });
            }
            Err(error) => {
                diagnostics = error.diagnostics;
                attempts.push(AuthoringRepairAttempt {
                    attempt,
                    diagnostics: diagnostics.clone(),
                    operation: Some(ai_operation),
                    succeeded: false,
                });
            }
        }
    }

    let message = diagnostics
        .first()
        .map(|diagnostic| {
            format!(
                "AuthoringSpec repair failed at {} [{}]: {}",
                diagnostic.path, diagnostic.code, diagnostic.message
            )
        })
        .unwrap_or_else(|| "AuthoringSpec repair failed without a diagnostic".to_string());
    Err(AuthoringRepairFailure {
        message,
        diagnostics,
        attempts,
    })
}

pub fn format_authoring_repair_summary(attempts: &[AuthoringRepairAttempt]) -> String {
    let mut output = String::new();
    for attempt in attempts {
        let status = if attempt.succeeded { "ok" } else { "failed" };
        output.push_str(&format!("attempt {}: {status}\n", attempt.attempt));
        for diagnostic in &attempt.diagnostics {
            output.push_str(&format!(
                "  {} [{}]: {}\n",
                diagnostic.path, diagnostic.code, diagnostic.message
            ));
        }
        if let Some(operation) = &attempt.operation {
            let name = match operation {
                AiAuthoringOperation::ReplaceVisualNode { .. } => "replace_visual_node",
                AiAuthoringOperation::Insert { .. } => "insert",
                AiAuthoringOperation::Move { .. } => "move",
                AiAuthoringOperation::Remove { .. } => "remove",
            };
            output.push_str(&format!("  operation: {name}\n"));
        }
    }
    output
}

fn serialization_failure(message: String, attempts: Vec<AuthoringRepairAttempt>) -> AuthoringRepairFailure {
    AuthoringRepairFailure {
        message: format!("failed to serialize AuthoringSpec: {message}"),
        diagnostics: Vec::new(),
        attempts,
    }
}

fn provider_failure(
    error: AiError,
    attempts: Vec<AuthoringRepairAttempt>,
    diagnostics: Vec<AuthoringDiagnostic>,
) -> AuthoringRepairFailure {
    AuthoringRepairFailure {
        message: format!("AuthoringSpec repair provider failed: {error}"),
        diagnostics,
        attempts,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;
    use crate::ai::provider::AiProvider;

    struct RepairProvider {
        repair: Value,
    }

    impl AiProvider for RepairProvider {
        fn generate(
            &self,
            _input: &str,
            _config: &AiConfig,
            _target: GenerationTarget,
        ) -> Result<Value, AiError> {
            unreachable!("repair test does not generate")
        }

        fn repair_authoring(
            &self,
            _request: &AuthoringRepairRequest,
            _config: &AiConfig,
        ) -> Result<Value, AiError> {
            Ok(self.repair.clone())
        }
    }

    fn invalid_parameter_document() -> Value {
        json!({
            "authoring_format_version": 0,
            "artboard": {
                "id": "stage",
                "width": { "value": 320.0, "unit": "px" },
                "height": { "value": 240.0, "unit": "px" }
            },
            "visual": {
                "nodes": [{
                    "kind": "rectangle",
                    "id": "panel",
                    "width": { "kind": "parameter", "name": "missing" },
                    "height": { "kind": "literal", "value": 80.0, "unit": "px" },
                    "fill": "#3366FF"
                }]
            },
            "motion": {},
            "behavior": {}
        })
    }

    #[test]
    fn repairs_failed_authoring_with_one_stable_id_operation() {
        let provider = RepairProvider {
            repair: json!({
                "operation": {
                    "op": "replace_visual_node",
                    "target_id": "panel",
                    "node": {
                        "kind": "rectangle",
                        "id": "panel",
                        "width": { "kind": "literal", "value": 120.0, "unit": "px" },
                        "height": { "kind": "literal", "value": 80.0, "unit": "px" },
                        "fill": "#3366FF"
                    }
                }
            }),
        };
        let config = AiConfig::default();

        let repaired = repair_authoring_spec(&provider, &config, invalid_parameter_document(), 1)
            .expect("stable-ID operation should repair the authored concept");

        assert_eq!(repaired.total_retries, 1);
        assert_eq!(repaired.authoring_json["visual"]["nodes"][0]["id"], "panel");
        assert!(repaired.attempts[0].diagnostics.iter().any(|diagnostic| {
            diagnostic.path.contains("visual.nodes[0].width")
                && diagnostic.code == "unknown_parameter"
        }));
        assert!(repaired.attempts[1].succeeded);
    }

    #[test]
    fn repair_schema_requires_one_incremental_operation() {
        let schema = authoring_repair_schema();
        let text = serde_json::to_string(&schema).expect("serialize repair schema");
        assert!(text.contains("replace_visual_node"));
        assert!(text.contains("target_id"));
        assert!(text.contains("operation"));
    }
}
