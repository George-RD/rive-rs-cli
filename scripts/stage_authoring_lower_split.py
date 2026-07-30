#!/usr/bin/env python3
"""Split the AuthoringSpec lowerer into focused, behavior-preserving modules."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "src/authoring/lower.rs"
LOWER_DIR = ROOT / "src/authoring/lower"
TODO = ROOT / "meta/todos/todo.visual-authoring-compiler.md"


def require(text: str, needle: str) -> None:
    if needle not in text:
        raise SystemExit(f"required marker not found: {needle!r}")


def expose(block: str, signature: str) -> str:
    old = f"    fn {signature}("
    new = f"    pub(super) fn {signature}("
    if old not in block:
        raise SystemExit(f"method marker not found: {signature}")
    return block.replace(old, new, 1)


def module(imports: str, block: str) -> str:
    return f"{imports.rstrip()}\n\nimpl<'a> Lowerer<'a> {{\n{block.rstrip()}\n}}\n"


text = SOURCE.read_text()
if "mod node;" in text or LOWER_DIR.exists():
    raise SystemExit("authoring lowerer already appears to be split")

old_expression_import = (
    "use super::expression::{evaluate_expression, evaluate_quantity, evaluate_transform};"
)
old_spec_import = """use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringDiagnostic, AuthoringError, AuthoringSourceMap,
    AuthoringSpec, ComponentSpec, GradientKind, LoweredAuthoring, PaintSpec, Quantity, ScalarExpr,
    SourceMapEntry, TextAlign, TextOverflow, TrimPathMode, TrimPathSpec, Unit,
};"""
old_visual_import = "use super::visual::{ShapeNodeRef, TextNodeRef, VisualNode};"
for marker in (old_expression_import, old_spec_import, old_visual_import):
    require(text, marker)

node_start = text.index("    fn lower_node(")
text_start = text.index("    fn lower_text(", node_start)
shape_start = text.index("    fn lower_shape(", text_start)
trim_start = text.index("    fn lower_trim_path(", shape_start)
paint_start = text.index("    fn lower_paint(", trim_start)
raw_start = text.index("    fn lower_raw_fragments(", paint_start)

node_block = expose(text[node_start:text_start], "lower_node")
text_block = expose(text[text_start:shape_start], "lower_text")
shape_block = expose(text[shape_start:trim_start], "lower_shape")
paint_block = expose(text[trim_start:raw_start], "lower_trim_path")
paint_block = expose(paint_block, "lower_paint")

text = text[:node_start] + text[raw_start:]
text = text.replace(
    old_expression_import,
    "use super::expression::{evaluate_expression, evaluate_quantity};",
    1,
)
text = text.replace(
    old_spec_import,
    """use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringDiagnostic, AuthoringError, AuthoringSourceMap,
    AuthoringSpec, ComponentSpec, LoweredAuthoring, Quantity, ScalarExpr, SourceMapEntry, Unit,
};""",
    1,
)
text = text.replace(
    old_visual_import,
    """use super::visual::VisualNode;

mod node;
mod paint;
mod shape;
mod text;""",
    1,
)

LOWER_DIR.mkdir()
(LOWER_DIR / "node.rs").write_text(
    module(
        """use serde_json::{Value, json};

use super::{Lowerer, NodeContext, collect_named_paths, runtime_name};
use super::super::expression::evaluate_transform;
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry};
use super::super::visual::VisualNode;""",
        node_block,
    )
)
(LOWER_DIR / "text.rs").write_text(
    module(
        """use serde_json::{Value, json};

use super::{
    LoweredObject, Lowerer, NodeContext, PaintTarget, evaluate_ratio_expression, runtime_name,
};
use super::super::expression::{evaluate_expression, evaluate_transform};
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry, TextAlign, TextOverflow, Unit};
use super::super::visual::TextNodeRef;""",
        text_block,
    )
)
(LOWER_DIR / "shape.rs").write_text(
    module(
        """use serde_json::{Value, json};

use super::{
    LoweredObject, Lowerer, NodeContext, PaintTarget, evaluate_ratio_expression, runtime_name,
};
use super::super::expression::{evaluate_expression, evaluate_transform};
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry, Unit};
use super::super::visual::ShapeNodeRef;""",
        shape_block,
    )
)
(LOWER_DIR / "paint.rs").write_text(
    module(
        """use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::{LoweredObject, Lowerer, PaintTarget, evaluate_ratio_expression, runtime_name};
use super::super::expression::evaluate_expression;
use super::super::spec::{
    AuthoringDiagnostic, GradientKind, PaintSpec, Quantity, TrimPathMode, TrimPathSpec, Unit,
};""",
        paint_block,
    )
)
SOURCE.write_text(text)

progress = TODO.read_text()
marker = "- Remaining work includes font and image assets, bounded patterns, constraints, and\n"
require(progress, marker)
progress_note = (
    "- The authored lowering pipeline is split by node, text, shape, and paint responsibility.\n"
    "  Every authored lowering source file is below Cairn's module-size guideline while the\n"
    "  schema, validation, generated SceneSpec, runtime names, and source maps remain fixed.\n"
)
TODO.write_text(progress.replace(marker, progress_note + marker, 1))

for candidate in [SOURCE, *sorted(LOWER_DIR.glob("*.rs"))]:
    line_count = len(candidate.read_text().splitlines())
    if line_count > 500:
        raise SystemExit(f"{candidate.relative_to(ROOT)} remains oversized at {line_count} lines")
    print(f"{candidate.relative_to(ROOT)}: {line_count} lines")
