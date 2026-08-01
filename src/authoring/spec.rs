use std::collections::BTreeMap;
use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::visual::VisualNode;

pub const AUTHORING_FORMAT_VERSION: u32 = 0;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AuthoringSpec {
    #[schemars(range(min = 0, max = 0))]
    pub authoring_format_version: u32,
    pub artboard: AuthoringArtboard,
    #[serde(default)]
    pub font_assets: BTreeMap<String, String>,
    #[serde(default)]
    pub image_assets: BTreeMap<String, String>,
    #[serde(default)]
    pub parameters: BTreeMap<String, Quantity>,
    #[serde(default)]
    pub components: Vec<ComponentSpec>,
    pub visual: VisualSection,
    pub motion: MotionSection,
    pub behavior: BehaviorSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AuthoringArtboard {
    pub id: String,
    pub width: Quantity,
    pub height: Quantity,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Quantity {
    pub value: f64,
    pub unit: Unit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    Px,
    Scalar,
    Degrees,
    Radians,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ScalarExpr {
    Literal {
        value: f64,
        unit: Unit,
    },
    Parameter {
        name: String,
    },
    Add {
        left: Box<ScalarExpr>,
        right: Box<ScalarExpr>,
    },
    Subtract {
        left: Box<ScalarExpr>,
        right: Box<ScalarExpr>,
    },
    Multiply {
        value: Box<ScalarExpr>,
        factor: f64,
    },
    Divide {
        value: Box<ScalarExpr>,
        divisor: f64,
    },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TransformSpec {
    #[serde(default)]
    pub x: Option<ScalarExpr>,
    #[serde(default)]
    pub y: Option<ScalarExpr>,
    #[serde(default)]
    pub rotation: Option<ScalarExpr>,
    #[serde(default)]
    pub scale_x: Option<ScalarExpr>,
    #[serde(default)]
    pub scale_y: Option<ScalarExpr>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConstraintSpec {
    Align {
        id: String,
        subject: String,
        target: String,
        axis: ConstraintAxis,
    },
    Center {
        id: String,
        subject: String,
        start: String,
        end: String,
        axis: ConstraintAxis,
    },
    Offset {
        id: String,
        subject: String,
        target: String,
        x: ScalarExpr,
        y: ScalarExpr,
    },
    Spacing {
        id: String,
        #[schemars(length(min = 2, max = 100))]
        items: Vec<String>,
        axis: ConstraintAxis,
        gap: ScalarExpr,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GradientKind {
    LinearGradient,
    RadialGradient,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GradientStopSpec {
    pub color: String,
    pub position: ScalarExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GradientPaintSpec {
    pub kind: GradientKind,
    pub start_x: ScalarExpr,
    pub start_y: ScalarExpr,
    pub end_x: ScalarExpr,
    pub end_y: ScalarExpr,
    pub stops: Vec<GradientStopSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum PaintSpec {
    Solid(String),
    Gradient(GradientPaintSpec),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TrimPathMode {
    Sequential,
    Synchronized,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TrimPathSpec {
    pub start: ScalarExpr,
    pub end: ScalarExpr,
    #[serde(default)]
    pub offset: Option<ScalarExpr>,
    pub mode: TrimPathMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StrokeSpec {
    #[serde(alias = "color")]
    pub paint: PaintSpec,
    pub width: ScalarExpr,
    #[serde(default)]
    pub trim: Option<TrimPathSpec>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    #[default]
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TextOverflow {
    #[default]
    Visible,
    Hidden,
    Clipped,
    Ellipsis,
    Fit,
    FitFontSize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ComponentSpec {
    pub id: String,
    #[serde(default)]
    pub parameters: BTreeMap<String, Quantity>,
    pub visual: Vec<VisualNode>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VisualSection {
    #[serde(default)]
    pub nodes: Vec<VisualNode>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MotionSection {
    #[serde(default)]
    pub raw_animations: Vec<RawSceneFragment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BehaviorSection {
    #[serde(default)]
    pub raw_state_machines: Vec<RawSceneFragment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RawSceneFragment {
    pub id: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LoweredAuthoring {
    pub scene: Value,
    pub source_map: AuthoringSourceMap,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct AuthoringSourceMap {
    pub entries: Vec<SourceMapEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceMapEntry {
    pub authored_id: String,
    pub authored_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_path: Option<String>,
    pub runtime_names: Vec<String>,
    pub scene_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoringDiagnostic {
    pub path: String,
    pub code: String,
    pub message: String,
}

impl AuthoringDiagnostic {
    pub(crate) fn new(
        path: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            code: code.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoringError {
    pub diagnostics: Vec<AuthoringDiagnostic>,
}

impl AuthoringError {
    pub(crate) fn one(diagnostic: AuthoringDiagnostic) -> Self {
        Self {
            diagnostics: vec![diagnostic],
        }
    }

    pub(crate) fn many(diagnostics: Vec<AuthoringDiagnostic>) -> Self {
        Self { diagnostics }
    }
}

impl fmt::Display for AuthoringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(first) = self.diagnostics.first() {
            write!(
                formatter,
                "AuthoringSpec failed at {} [{}]: {}",
                first.path, first.code, first.message
            )
        } else {
            formatter.write_str("AuthoringSpec failed without a diagnostic")
        }
    }
}

impl std::error::Error for AuthoringError {}
