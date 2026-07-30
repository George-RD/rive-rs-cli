use std::collections::BTreeMap;
use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const AUTHORING_FORMAT_VERSION: u32 = 0;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AuthoringSpec {
    #[schemars(range(min = 0, max = 0))]
    pub authoring_format_version: u32,
    pub artboard: AuthoringArtboard,
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StrokeSpec {
    pub color: String,
    pub width: ScalarExpr,
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VisualNode {
    Ellipse {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        fill: String,
        #[serde(default)]
        stroke: Option<StrokeSpec>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Rectangle {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        fill: String,
        #[serde(default)]
        stroke: Option<StrokeSpec>,
        #[serde(default)]
        corner_radius: Option<ScalarExpr>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Triangle {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        fill: String,
        #[serde(default)]
        stroke: Option<StrokeSpec>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Polygon {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        #[schemars(range(min = 3))]
        points: u64,
        fill: String,
        #[serde(default)]
        stroke: Option<StrokeSpec>,
        #[serde(default)]
        corner_radius: Option<ScalarExpr>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Star {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        #[schemars(range(min = 3))]
        points: u64,
        inner_radius: ScalarExpr,
        fill: String,
        #[serde(default)]
        stroke: Option<StrokeSpec>,
        #[serde(default)]
        corner_radius: Option<ScalarExpr>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Group {
        id: String,
        #[serde(default)]
        transform: TransformSpec,
        #[serde(default)]
        children: Vec<VisualNode>,
    },
    Instance {
        id: String,
        component: String,
        #[serde(default)]
        overrides: BTreeMap<String, Quantity>,
        #[serde(default)]
        transform: TransformSpec,
    },
    RawSceneObject {
        id: String,
        object: Value,
    },
}

#[derive(Clone, Copy)]
pub(crate) struct ShapeNodeRef<'a> {
    pub geometry_type: &'static str,
    pub width: &'a ScalarExpr,
    pub height: &'a ScalarExpr,
    pub points: Option<u64>,
    pub corner_radius: Option<&'a ScalarExpr>,
    pub inner_radius: Option<&'a ScalarExpr>,
    pub fill: &'a str,
    pub stroke: Option<&'a StrokeSpec>,
    pub transform: &'a TransformSpec,
}

impl VisualNode {
    pub(crate) fn id(&self) -> &str {
        match self {
            Self::Ellipse { id, .. }
            | Self::Rectangle { id, .. }
            | Self::Triangle { id, .. }
            | Self::Polygon { id, .. }
            | Self::Star { id, .. }
            | Self::Group { id, .. }
            | Self::Instance { id, .. }
            | Self::RawSceneObject { id, .. } => id,
        }
    }

    pub(crate) fn shape(&self) -> Option<ShapeNodeRef<'_>> {
        let shape = match self {
            Self::Ellipse {
                width,
                height,
                fill,
                stroke,
                transform,
                ..
            } => ShapeNodeRef {
                geometry_type: "ellipse",
                width,
                height,
                points: None,
                corner_radius: None,
                inner_radius: None,
                fill,
                stroke: stroke.as_ref(),
                transform,
            },
            Self::Rectangle {
                width,
                height,
                fill,
                stroke,
                corner_radius,
                transform,
                ..
            } => ShapeNodeRef {
                geometry_type: "rectangle",
                width,
                height,
                points: None,
                corner_radius: corner_radius.as_ref(),
                inner_radius: None,
                fill,
                stroke: stroke.as_ref(),
                transform,
            },
            Self::Triangle {
                width,
                height,
                fill,
                stroke,
                transform,
                ..
            } => ShapeNodeRef {
                geometry_type: "triangle",
                width,
                height,
                points: None,
                corner_radius: None,
                inner_radius: None,
                fill,
                stroke: stroke.as_ref(),
                transform,
            },
            Self::Polygon {
                width,
                height,
                points,
                fill,
                stroke,
                corner_radius,
                transform,
                ..
            } => ShapeNodeRef {
                geometry_type: "polygon",
                width,
                height,
                points: Some(*points),
                corner_radius: corner_radius.as_ref(),
                inner_radius: None,
                fill,
                stroke: stroke.as_ref(),
                transform,
            },
            Self::Star {
                width,
                height,
                points,
                inner_radius,
                fill,
                stroke,
                corner_radius,
                transform,
                ..
            } => ShapeNodeRef {
                geometry_type: "star",
                width,
                height,
                points: Some(*points),
                corner_radius: corner_radius.as_ref(),
                inner_radius: Some(inner_radius),
                fill,
                stroke: stroke.as_ref(),
                transform,
            },
            Self::Group { .. } | Self::Instance { .. } | Self::RawSceneObject { .. } => {
                return None;
            }
        };
        Some(shape)
    }

    pub(crate) fn children(&self) -> Option<&[VisualNode]> {
        match self {
            Self::Group { children, .. } => Some(children),
            _ => None,
        }
    }
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
