from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected one match in {path}, found {count}")
    file.write_text(text.replace(old, new))


spec_path = Path("src/authoring/spec.rs")
spec = spec_path.read_text()
if "pub enum VisualNode" not in spec:
    raise SystemExit(0)

visual_start_marker = '''#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VisualNode {
'''
visual_end_marker = '''#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MotionSection {
'''
start = spec.index(visual_start_marker)
end = spec.index(visual_end_marker, start)
visual_block = spec[start:end]

spec = spec[:start] + spec[end:]
spec = spec.replace(
    "use serde_json::Value;\n\npub const AUTHORING_FORMAT_VERSION",
    "use serde_json::Value;\n\nuse super::visual::VisualNode;\n\npub const AUTHORING_FORMAT_VERSION",
    1,
)
spec_path.write_text(spec)

visual_header = '''use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::spec::{
    PaintSpec, Quantity, ScalarExpr, StrokeSpec, TextAlign, TextOverflow, TransformSpec,
};

'''
Path("src/authoring/visual.rs").write_text(visual_header + visual_block)

replace_once(
    "src/authoring/mod.rs",
    "mod validation;\n",
    "mod validation;\nmod visual;\n",
)
replace_once(
    "src/authoring/mod.rs",
    '''    RawSceneFragment, ScalarExpr, SourceMapEntry, StrokeSpec, TransformSpec, Unit, VisualNode,
    VisualSection,
};
''',
    '''    RawSceneFragment, ScalarExpr, SourceMapEntry, StrokeSpec, TransformSpec, Unit, VisualSection,
};
pub use visual::VisualNode;
''',
)

replace_once(
    "src/authoring/lower.rs",
    '''use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringDiagnostic, AuthoringError, AuthoringSourceMap,
    AuthoringSpec, ComponentSpec, GradientKind, LoweredAuthoring, PaintSpec, Quantity, ScalarExpr,
    ShapeNodeRef, SourceMapEntry, TextAlign, TextNodeRef, TextOverflow, TrimPathMode, TrimPathSpec,
    Unit, VisualNode,
};
''',
    '''use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringDiagnostic, AuthoringError, AuthoringSourceMap,
    AuthoringSpec, ComponentSpec, GradientKind, LoweredAuthoring, PaintSpec, Quantity, ScalarExpr,
    SourceMapEntry, TextAlign, TextOverflow, TrimPathMode, TrimPathSpec, Unit,
};
use super::visual::{ShapeNodeRef, TextNodeRef, VisualNode};
''',
)

replace_once(
    "src/authoring/validation.rs",
    '''use super::spec::{
    AuthoringDiagnostic, AuthoringSpec, PaintSpec, Quantity, ScalarExpr, TransformSpec, VisualNode,
};
''',
    '''use super::spec::{
    AuthoringDiagnostic, AuthoringSpec, PaintSpec, Quantity, ScalarExpr, TransformSpec,
};
use super::visual::VisualNode;
''',
)

replace_once(
    "src/authoring/frontend.rs",
    '''use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringArtboard, AuthoringDiagnostic, AuthoringError,
    AuthoringSpec, BehaviorSection, LoweredAuthoring, MotionSection, Quantity, RawSceneFragment,
    TransformSpec, Unit, VisualNode, VisualSection,
};
''',
    '''use super::spec::{
    AUTHORING_FORMAT_VERSION, AuthoringArtboard, AuthoringDiagnostic, AuthoringError,
    AuthoringSpec, BehaviorSection, LoweredAuthoring, MotionSection, Quantity, RawSceneFragment,
    TransformSpec, Unit, VisualSection,
};
use super::visual::VisualNode;
''',
)

replace_once(
    "src/authoring/limits.rs",
    "use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec, ComponentSpec, VisualNode};\n",
    '''use super::spec::{AuthoringDiagnostic, AuthoringError, AuthoringSpec, ComponentSpec};
use super::visual::VisualNode;
''',
)
