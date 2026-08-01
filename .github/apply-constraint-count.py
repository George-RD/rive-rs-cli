from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text()
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"missing replacement marker in {path}: {old[:120]!r}")
    file.write_text(text.replace(old, new, 1))


replace_once(
    "src/authoring/constraint.rs",
    """    if constraints.is_empty() {
        return Ok(Cow::Borrowed(children));
    }

    validate_constraint_ids(constraints, group_path)?;""",
    """    if constraints.is_empty() {
        return Ok(Cow::Borrowed(children));
    }
    if constraints.len() > MAX_AUTHORING_ITEM_COUNT {
        return Err(AuthoringDiagnostic::new(
            format!("{group_path}.constraints"),
            "invalid_constraint_count",
            format!(
                "groups support at most {MAX_AUTHORING_ITEM_COUNT} constraints"
            ),
        ));
    }

    validate_constraint_ids(constraints, group_path)?;""",
)

replace_once(
    "src/authoring/visual.rs",
    """        #[serde(default)]
        constraints: Vec<ConstraintSpec>,""",
    """        #[serde(default)]
        #[schemars(length(max = 100))]
        constraints: Vec<ConstraintSpec>,""",
)

replace_once(
    "docs/authoring-spec-v0.md",
    """Constraints are intentionally group-local and anchor-based. They do not inspect rendered bounds, infer edges, or act as a general CAD solver. Raw `SceneSpec` nodes cannot participate because they have no typed authoring transform. Each constraint `id` must be non-empty after trimming, must not contain `/`, and must be unique within its group. Dependency chains are bounded to 100 assignments. Unknown siblings, invalid or duplicate constraint IDs, duplicate spacing entries, conflicting assignments, invalid units, excessive dependency depth, and dependency cycles return authored-path diagnostics such as `unknown_constraint_node`, `invalid_constraint_id`, `duplicate_constraint_id`, `constraint_conflict`, `constraint_resolution_depth_limit`, and `constraint_cycle`. Cycle messages include the stable authored anchor chain.""",
    """Constraints are intentionally group-local and anchor-based. They do not inspect rendered bounds, infer edges, or act as a general CAD solver. Raw `SceneSpec` nodes cannot participate because they have no typed authoring transform. A group may declare at most 100 constraints. Each constraint `id` must be non-empty after trimming, must not contain `/`, and must be unique within its group. Dependency chains are bounded to 100 assignments. Unknown siblings, oversized constraint lists, invalid or duplicate constraint IDs, duplicate spacing entries, conflicting assignments, invalid units, excessive dependency depth, and dependency cycles return authored-path diagnostics such as `unknown_constraint_node`, `invalid_constraint_count`, `invalid_constraint_id`, `duplicate_constraint_id`, `constraint_conflict`, `constraint_resolution_depth_limit`, and `constraint_cycle`. Cycle messages include the stable authored anchor chain.""",
)

replace_once(
    "meta/contracts/authoring.md",
    """- constraints that reference direct typed siblings by stable authored ID, preserve component parameter and instance override semantics, and report invalid IDs, conflicts, bounded dependency depth, or cycles at authored paths;""",
    """- constraints that reference direct typed siblings by stable authored ID, preserve component parameter and instance override semantics, bound each group to 100 declarations, and report invalid IDs, conflicts, bounded dependency depth, or cycles at authored paths;""",
)

replace_once(
    "meta/todos/todo.visual-authoring-compiler.md",
    """- PR #155 review hardening bounds dependency resolution to 100 assignments, reuses
  the shared authoring item limit, and avoids cloning groups that declare no constraints.
- Remaining work is the complex static showcase without raw escapes.""",
    """- PR #155 review hardening bounds dependency resolution and each group's constraint
  list to 100, reuses the shared authoring item limit, avoids cloning groups that declare
  no constraints, and pins align, center, and spacing behavior on both axes.
- Remaining work is the complex static showcase without raw escapes.""",
)
