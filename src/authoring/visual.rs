use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::spec::{
    PaintSpec, Quantity, ScalarExpr, StrokeSpec, TextAlign, TextOverflow, TransformSpec,
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VisualNode {
    Ellipse {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        fill: PaintSpec,
        #[serde(default)]
        stroke: Option<StrokeSpec>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Rectangle {
        id: String,
        width: ScalarExpr,
        height: ScalarExpr,
        fill: PaintSpec,
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
        fill: PaintSpec,
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
        fill: PaintSpec,
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
        fill: PaintSpec,
        #[serde(default)]
        stroke: Option<StrokeSpec>,
        #[serde(default)]
        corner_radius: Option<ScalarExpr>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Text {
        id: String,
        text: String,
        #[serde(default)]
        font: Option<String>,
        font_size: ScalarExpr,
        fill: PaintSpec,
        #[serde(default)]
        width: Option<ScalarExpr>,
        #[serde(default)]
        height: Option<ScalarExpr>,
        #[serde(default)]
        line_height: Option<ScalarExpr>,
        #[serde(default)]
        letter_spacing: Option<ScalarExpr>,
        #[serde(default)]
        paragraph_spacing: Option<ScalarExpr>,
        #[serde(default)]
        origin_x: Option<ScalarExpr>,
        #[serde(default)]
        origin_y: Option<ScalarExpr>,
        #[serde(default)]
        align: TextAlign,
        #[serde(default)]
        overflow: TextOverflow,
        #[serde(default)]
        transform: TransformSpec,
    },
    Grid {
        id: String,
        #[schemars(range(min = 1, max = 100))]
        columns: u64,
        #[schemars(range(min = 1, max = 100))]
        rows: u64,
        column_step: ScalarExpr,
        row_step: ScalarExpr,
        item: Box<VisualNode>,
        #[serde(default)]
        transform: TransformSpec,
    },
    Radial {
        id: String,
        #[schemars(range(min = 1, max = 100))]
        copies: u64,
        radius: ScalarExpr,
        start_angle: ScalarExpr,
        angle_step: ScalarExpr,
        #[serde(default)]
        rotate_items: bool,
        item: Box<VisualNode>,
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
    pub fill: &'a PaintSpec,
    pub stroke: Option<&'a StrokeSpec>,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) struct TextNodeRef<'a> {
    pub content: &'a str,
    pub font: Option<&'a str>,
    pub font_size: &'a ScalarExpr,
    pub fill: &'a PaintSpec,
    pub width: Option<&'a ScalarExpr>,
    pub height: Option<&'a ScalarExpr>,
    pub line_height: Option<&'a ScalarExpr>,
    pub letter_spacing: Option<&'a ScalarExpr>,
    pub paragraph_spacing: Option<&'a ScalarExpr>,
    pub origin_x: Option<&'a ScalarExpr>,
    pub origin_y: Option<&'a ScalarExpr>,
    pub align: TextAlign,
    pub overflow: TextOverflow,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) struct GridNodeRef<'a> {
    pub columns: u64,
    pub rows: u64,
    pub column_step: &'a ScalarExpr,
    pub row_step: &'a ScalarExpr,
    pub item: &'a VisualNode,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) struct RadialNodeRef<'a> {
    pub copies: u64,
    pub radius: &'a ScalarExpr,
    pub start_angle: &'a ScalarExpr,
    pub angle_step: &'a ScalarExpr,
    pub rotate_items: bool,
    pub item: &'a VisualNode,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) enum PatternNodeRef<'a> {
    Grid(GridNodeRef<'a>),
    Radial(RadialNodeRef<'a>),
}

impl<'a> PatternNodeRef<'a> {
    pub(crate) fn item(self) -> &'a VisualNode {
        match self {
            Self::Grid(grid) => grid.item,
            Self::Radial(radial) => radial.item,
        }
    }
}

impl VisualNode {
    pub(crate) fn id(&self) -> &str {
        match self {
            Self::Ellipse { id, .. }
            | Self::Rectangle { id, .. }
            | Self::Triangle { id, .. }
            | Self::Polygon { id, .. }
            | Self::Star { id, .. }
            | Self::Text { id, .. }
            | Self::Grid { id, .. }
            | Self::Radial { id, .. }
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
            Self::Text { .. }
            | Self::Grid { .. }
            | Self::Radial { .. }
            | Self::Group { .. }
            | Self::Instance { .. }
            | Self::RawSceneObject { .. } => {
                return None;
            }
        };
        Some(shape)
    }

    pub(crate) fn text_node(&self) -> Option<TextNodeRef<'_>> {
        match self {
            Self::Text {
                text,
                font,
                font_size,
                fill,
                width,
                height,
                line_height,
                letter_spacing,
                paragraph_spacing,
                origin_x,
                origin_y,
                align,
                overflow,
                transform,
                ..
            } => Some(TextNodeRef {
                content: text,
                font: font.as_deref(),
                font_size,
                fill,
                width: width.as_ref(),
                height: height.as_ref(),
                line_height: line_height.as_ref(),
                letter_spacing: letter_spacing.as_ref(),
                paragraph_spacing: paragraph_spacing.as_ref(),
                origin_x: origin_x.as_ref(),
                origin_y: origin_y.as_ref(),
                align: *align,
                overflow: *overflow,
                transform,
            }),
            _ => None,
        }
    }

    pub(crate) fn pattern(&self) -> Option<PatternNodeRef<'_>> {
        match self {
            Self::Grid {
                columns,
                rows,
                column_step,
                row_step,
                item,
                transform,
                ..
            } => Some(PatternNodeRef::Grid(GridNodeRef {
                columns: *columns,
                rows: *rows,
                column_step,
                row_step,
                item,
                transform,
            })),
            Self::Radial {
                copies,
                radius,
                start_angle,
                angle_step,
                rotate_items,
                item,
                transform,
                ..
            } => Some(PatternNodeRef::Radial(RadialNodeRef {
                copies: *copies,
                radius,
                start_angle,
                angle_step,
                rotate_items: *rotate_items,
                item,
                transform,
            })),
            _ => None,
        }
    }

    pub(crate) fn children(&self) -> Option<&[VisualNode]> {
        match self {
            Self::Group { children, .. } => Some(children),
            _ => None,
        }
    }
}
