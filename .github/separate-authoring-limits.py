from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text()
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"missing replacement marker in {path}: {old[:120]!r}")
    file.write_text(text.replace(old, new, 1))


def replace_count(path: str, old: str, new: str, expected: int) -> None:
    file = Path(path)
    text = file.read_text()
    count = text.count(old)
    if count == 0 and text.count(new) == expected:
        return
    if count != expected:
        raise SystemExit(
            f"expected {expected} replacements in {path}, found {count}: {old!r}"
        )
    file.write_text(text.replace(old, new))


replace_once(
    "src/authoring/limits.rs",
    "pub(crate) const MAX_AUTHORING_ITEM_COUNT: usize = 100;",
    "const MAX_PATTERN_AXIS_COUNT: u64 = 100;",
)
replace_count(
    "src/authoring/limits.rs",
    "MAX_AUTHORING_ITEM_COUNT as u64",
    "MAX_PATTERN_AXIS_COUNT",
    2,
)

replace_once(
    "src/authoring/constraint.rs",
    """use super::expression::{evaluate_expression, evaluate_transform, validate_scene_number};
use super::limits::MAX_AUTHORING_ITEM_COUNT;
use super::spec::{""",
    """use super::expression::{evaluate_expression, evaluate_transform, validate_scene_number};
use super::spec::{""",
)
replace_once(
    "src/authoring/constraint.rs",
    """use super::visual::VisualNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]""",
    """use super::visual::VisualNode;

const MAX_GROUP_CONSTRAINTS: usize = 100;
const MAX_SPACING_ITEMS: usize = 100;
const MAX_CONSTRAINT_RESOLUTION_DEPTH: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]""",
)
replace_count(
    "src/authoring/constraint.rs",
    "MAX_AUTHORING_ITEM_COUNT",
    "MAX_GROUP_CONSTRAINTS",
    2,
)
replace_count(
    "src/authoring/constraint.rs",
    "MAX_GROUP_CONSTRAINTS).contains(&items.len())",
    "MAX_SPACING_ITEMS).contains(&items.len())",
    1,
)
replace_count(
    "src/authoring/constraint.rs",
    "{MAX_GROUP_CONSTRAINTS} ordered sibling ids",
    "{MAX_SPACING_ITEMS} ordered sibling ids",
    1,
)
replace_count(
    "src/authoring/constraint.rs",
    "depth >= MAX_GROUP_CONSTRAINTS",
    "depth >= MAX_CONSTRAINT_RESOLUTION_DEPTH",
    1,
)
replace_count(
    "src/authoring/constraint.rs",
    "exceed {MAX_GROUP_CONSTRAINTS} assignments",
    "exceed {MAX_CONSTRAINT_RESOLUTION_DEPTH} assignments",
    1,
)

replace_once(
    "meta/todos/todo.visual-authoring-compiler.md",
    """- PR #155 review hardening bounds dependency resolution and each group's constraint
  list to 100, reuses the shared authoring item limit, avoids cloning groups that declare
  no constraints, and pins align, center, and spacing behavior on both axes.""",
    """- PR #155 review hardening independently bounds dependency resolution, spacing lists,
  and each group's constraint declarations to 100, keeps those limits separate from
  pattern and path bounds, avoids cloning unconstrained groups, and pins align, center,
  and spacing behavior on both axes.""",
)
