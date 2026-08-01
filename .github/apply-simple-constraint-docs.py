from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text()
    if new in text:
        return
    if old not in text:
        raise SystemExit(f"missing replacement marker in {path}: {old[:80]!r}")
    file.write_text(text.replace(old, new, 1))


replace_once(
    "docs/authoring-spec-v0.md",
    "The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, triangles, polygons, stars, literal text, static images, groups, component instances, deterministic grid, radial, mirror, distribute, and along-path patterns, semantic font and image assets, and raw `SceneSpec` objects. Shapes and text share one solid/linear/radial paint contract; stroke width is a positive pixel expression, and strokes may include a typed trim path. Polygon and star point counts must be at least three; star inner radius is a scalar ratio from zero to one. Constraints, motion helpers, and statechart authoring remain separate roadmap items.",
    "The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, triangles, polygons, stars, literal text, static images, groups, component instances, deterministic grid, radial, mirror, distribute, and along-path patterns, group-scoped transform-anchor constraints, semantic font and image assets, and raw `SceneSpec` objects. Shapes and text share one solid/linear/radial paint contract; stroke width is a positive pixel expression, and strokes may include a typed trim path. Polygon and star point counts must be at least three; star inner radius is a scalar ratio from zero to one. Motion helpers and statechart authoring remain separate roadmap items.",
)

replace_once(
    "docs/authoring-spec-v0.md",
    "## Raw canonical escapes",
    """## Group constraints

A `group` may declare an optional `constraints` array. Constraints reference direct children by stable authored `id` and resolve their typed `x` and `y` transform anchors before ordinary node lowering:

```json
"constraints": [
  {
    "kind": "align",
    "id": "align-label",
    "subject": "label",
    "target": "icon",
    "axis": "y"
  },
  {
    "kind": "center",
    "id": "center-label",
    "subject": "label",
    "start": "left-edge",
    "end": "right-edge",
    "axis": "x"
  },
  {
    "kind": "offset",
    "id": "place-badge",
    "subject": "badge",
    "target": "label",
    "x": { "kind": "literal", "value": 16, "unit": "px" },
    "y": { "kind": "literal", "value": -8, "unit": "px" }
  },
  {
    "kind": "spacing",
    "id": "space-actions",
    "items": ["action-a", "action-b", "action-c"],
    "axis": "x",
    "gap": { "kind": "parameter", "name": "action-gap" }
  }
]
```

`align` copies one sibling anchor on one axis. `center` places an anchor at the midpoint between two sibling anchors. `offset` derives both axes from one sibling plus pixel expressions. `spacing` preserves the first item's authored anchor and places each later item one pixel gap after the previous item on the selected axis; the perpendicular authored coordinate is unchanged. Constraint expressions use the normal component parameter scope, so instance overrides remain deterministic.

Constraints are intentionally group-local and anchor-based. They do not inspect rendered bounds, infer edges, or act as a general CAD solver. Raw `SceneSpec` nodes cannot participate because they have no typed authoring transform. Unknown siblings, duplicate spacing entries, conflicting assignments, invalid units, and dependency cycles return authored-path diagnostics such as `unknown_constraint_node`, `constraint_conflict`, and `constraint_cycle`. Cycle messages include the stable authored anchor chain.

## Raw canonical escapes""",
)

replace_once(
    "meta/contracts/authoring.md",
    "- reusable components, instances, bounded deterministic grid, radial, mirror, distribute, and along-path patterns, and simple constraints;",
    "- reusable components, instances, bounded deterministic grid, radial, mirror, distribute, and along-path patterns, and group-scoped transform-anchor constraints;\n- constraints that reference direct typed siblings by stable authored ID, preserve component parameter and instance override semantics, and report conflicts or cycles at authored paths;",
)
replace_once(
    "meta/contracts/authoring.md",
    "The first version stays JSON. A custom textual DSL or general CAD constraint\nsolver requires separate evidence and an accepted decision.",
    "The first version stays JSON. Its constraints align or derive direct-child `x` and\n`y` transform anchors; they are not a rendered-bounds or general CAD solver. A\ncustom textual DSL or broader constraint system requires separate evidence and an\naccepted decision.",
)

replace_once(
    "meta/todos/todo.visual-authoring-compiler.md",
    "- Remaining work includes constraints and a complex static showcase without raw escapes.",
    "- PR #155 implements deterministic group-scoped align, center, offset, and ordered\n  spacing constraints over direct-child `x` and `y` transform anchors. Component\n  parameters and instance overrides flow through the same typed expression scope.\n- Constraint assignments share one stable dependency graph and emit authored-path\n  diagnostics for unknown or raw siblings, duplicate entries, conflicting writes,\n  invalid units, malformed spacing lists, and cycles with the authored anchor chain.\n- Remaining work is the complex static showcase without raw escapes.",
)
