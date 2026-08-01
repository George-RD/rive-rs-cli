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
    "const MAX_PATTERN_AXIS_COUNT: u64 = 100;",
    "pub(crate) const MAX_AUTHORING_ITEM_COUNT: usize = 100;",
)
replace_count(
    "src/authoring/limits.rs",
    "MAX_PATTERN_AXIS_COUNT,",
    "MAX_AUTHORING_ITEM_COUNT as u64,",
    2,
)

replace_once(
    "src/authoring/constraint.rs",
    "use std::collections::{BTreeMap, BTreeSet};",
    "use std::borrow::Cow;\nuse std::collections::{BTreeMap, BTreeSet};",
)
replace_once(
    "src/authoring/constraint.rs",
    "use super::expression::{evaluate_expression, evaluate_transform, validate_scene_number};",
    "use super::expression::{evaluate_expression, evaluate_transform, validate_scene_number};\nuse super::limits::MAX_AUTHORING_ITEM_COUNT;",
)
replace_once(
    "src/authoring/constraint.rs",
    """pub(crate) fn resolve_group_constraints(
    children: &[VisualNode],
    constraints: &[ConstraintSpec],
    group_path: &str,
    scope: &BTreeMap<String, Quantity>,
) -> Result<Vec<VisualNode>, AuthoringDiagnostic> {
    if constraints.is_empty() {
        return Ok(children.to_vec());
    }""",
    """pub(crate) fn resolve_group_constraints<'a>(
    children: &'a [VisualNode],
    constraints: &[ConstraintSpec],
    group_path: &str,
    scope: &BTreeMap<String, Quantity>,
) -> Result<Cow<'a, [VisualNode]>, AuthoringDiagnostic> {
    if constraints.is_empty() {
        return Ok(Cow::Borrowed(children));
    }""",
)
replace_once(
    "src/authoring/constraint.rs",
    """                if !(2..=100).contains(&items.len()) {
                    return Err(AuthoringDiagnostic::new(
                        format!("{path}.items"),
                        "invalid_constraint_items",
                        "spacing constraints require between 2 and 100 ordered sibling ids",
                    ));
                }""",
    """                if !(2..=MAX_AUTHORING_ITEM_COUNT).contains(&items.len()) {
                    return Err(AuthoringDiagnostic::new(
                        format!("{path}.items"),
                        "invalid_constraint_items",
                        format!(
                            "spacing constraints require between 2 and {MAX_AUTHORING_ITEM_COUNT} ordered sibling ids"
                        ),
                    ));
                }""",
)
replace_once(
    "src/authoring/constraint.rs",
    """            &mut memo,
            &mut stack,
            children,
        )?;""",
    """            &mut memo,
            &mut stack,
            children,
            0,
        )?;""",
)
replace_once(
    "src/authoring/constraint.rs",
    "    Ok(resolved)\n}",
    "    Ok(Cow::Owned(resolved))\n}",
)
replace_once(
    "src/authoring/constraint.rs",
    """fn resolve_anchor(
    anchor: Anchor,
    assignments: &BTreeMap<Anchor, Assignment>,
    base_values: &BTreeMap<Anchor, f64>,
    memo: &mut BTreeMap<Anchor, f64>,
    stack: &mut Vec<Anchor>,
    children: &[VisualNode],
) -> Result<f64, AuthoringDiagnostic> {""",
    """fn resolve_anchor(
    anchor: Anchor,
    assignments: &BTreeMap<Anchor, Assignment>,
    base_values: &BTreeMap<Anchor, f64>,
    memo: &mut BTreeMap<Anchor, f64>,
    stack: &mut Vec<Anchor>,
    children: &[VisualNode],
    depth: usize,
) -> Result<f64, AuthoringDiagnostic> {""",
)
replace_once(
    "src/authoring/constraint.rs",
    """    };

    stack.push(anchor);
    let result = match assignment.formula {""",
    """    };

    if depth >= MAX_AUTHORING_ITEM_COUNT {
        return Err(AuthoringDiagnostic::new(
            &assignment.path,
            "constraint_resolution_depth_limit",
            format!(
                "constraint dependency chains must not exceed {MAX_AUTHORING_ITEM_COUNT} assignments"
            ),
        ));
    }

    stack.push(anchor);
    let result = match assignment.formula {""",
)
replace_once(
    "src/authoring/constraint.rs",
    """        Formula::Alias(source) => {
            resolve_anchor(source, assignments, base_values, memo, stack, children)
        }""",
    """        Formula::Alias(source) => resolve_anchor(
            source,
            assignments,
            base_values,
            memo,
            stack,
            children,
            depth + 1,
        ),""",
)
replace_once(
    "src/authoring/constraint.rs",
    """            let start = resolve_anchor(start, assignments, base_values, memo, stack, children)?;
            let end = resolve_anchor(end, assignments, base_values, memo, stack, children)?;""",
    """            let start = resolve_anchor(
                start,
                assignments,
                base_values,
                memo,
                stack,
                children,
                depth + 1,
            )?;
            let end = resolve_anchor(
                end,
                assignments,
                base_values,
                memo,
                stack,
                children,
                depth + 1,
            )?;""",
)
replace_once(
    "src/authoring/constraint.rs",
    """        Formula::Offset(source, amount) => {
            resolve_anchor(source, assignments, base_values, memo, stack, children)
                .map(|value| value + amount)
        }""",
    """        Formula::Offset(source, amount) => resolve_anchor(
            source,
            assignments,
            base_values,
            memo,
            stack,
            children,
            depth + 1,
        )
        .map(|value| value + amount),""",
)

replace_once(
    "docs/authoring-spec-v0.md",
    """Constraints are intentionally group-local and anchor-based. They do not inspect rendered bounds, infer edges, or act as a general CAD solver. Raw `SceneSpec` nodes cannot participate because they have no typed authoring transform. Unknown siblings, duplicate spacing entries, conflicting assignments, invalid units, and dependency cycles return authored-path diagnostics such as `unknown_constraint_node`, `constraint_conflict`, and `constraint_cycle`. Cycle messages include the stable authored anchor chain.""",
    """Constraints are intentionally group-local and anchor-based. They do not inspect rendered bounds, infer edges, or act as a general CAD solver. Raw `SceneSpec` nodes cannot participate because they have no typed authoring transform. Each constraint `id` must be non-empty after trimming, must not contain `/`, and must be unique within its group. Dependency chains are bounded to 100 assignments. Unknown siblings, invalid or duplicate constraint IDs, duplicate spacing entries, conflicting assignments, invalid units, excessive dependency depth, and dependency cycles return authored-path diagnostics such as `unknown_constraint_node`, `invalid_constraint_id`, `duplicate_constraint_id`, `constraint_conflict`, `constraint_resolution_depth_limit`, and `constraint_cycle`. Cycle messages include the stable authored anchor chain.""",
)

replace_once(
    "meta/contracts/authoring.md",
    "- constraints that reference direct typed siblings by stable authored ID, preserve component parameter and instance override semantics, and report conflicts or cycles at authored paths;",
    "- constraints that reference direct typed siblings by stable authored ID, preserve component parameter and instance override semantics, and report invalid IDs, conflicts, bounded dependency depth, or cycles at authored paths;",
)

replace_once(
    "meta/todos/todo.visual-authoring-compiler.md",
    """- Constraint assignments share one stable dependency graph and emit authored-path
  diagnostics for unknown or raw siblings, duplicate entries, conflicting writes,
  invalid units, malformed spacing lists, and cycles with the authored anchor chain.
- Remaining work is the complex static showcase without raw escapes.""",
    """- Constraint assignments share one stable dependency graph and emit authored-path
  diagnostics for unknown or raw siblings, invalid or duplicate constraint IDs,
  duplicate entries, conflicting writes, invalid units, malformed spacing lists, and
  cycles with the authored anchor chain.
- PR #155 review hardening bounds dependency resolution to 100 assignments, reuses
  the shared authoring item limit, and avoids cloning groups that declare no constraints.
- Remaining work is the complex static showcase without raw escapes.""",
)

replace_once(
    "cairn.blueprint",
    '"./tests/authoring_constraint_contract.rs", "./tests/authoring_contract.rs"',
    '"./tests/authoring_constraint_contract.rs", "./tests/authoring_constraint_safety_contract.rs", "./tests/authoring_contract.rs"',
)
