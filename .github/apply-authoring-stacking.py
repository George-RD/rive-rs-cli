from pathlib import Path
import json


def replace(path: str, old: str, new: str) -> None:
    target = Path(path)
    text = target.read_text()
    if old not in text:
        raise SystemExit(f"replacement target not found: {path}")
    target.write_text(text.replace(old, new, 1))


replace(
    "src/authoring/spec.rs",
    '''#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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
''',
    '''#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StackingOrder {
    #[default]
    FrontToBack,
    BackToFront,
}

impl StackingOrder {
    pub(crate) fn scene_index(self, authored_index: usize, child_count: usize) -> usize {
        match self {
            Self::FrontToBack => authored_index,
            Self::BackToFront => child_count - authored_index - 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ComponentSpec {
    pub id: String,
    #[serde(default)]
    pub parameters: BTreeMap<String, Quantity>,
    #[serde(default)]
    pub stacking: StackingOrder,
    pub visual: Vec<VisualNode>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VisualSection {
    #[serde(default)]
    pub stacking: StackingOrder,
    #[serde(default)]
    pub nodes: Vec<VisualNode>,
}
''',
)

replace(
    "src/authoring/visual.rs",
    '''use super::spec::{
    ConstraintSpec, PaintSpec, Quantity, ScalarExpr, StrokeSpec, TextAlign, TextOverflow,
    TransformSpec,
};
''',
    '''use super::spec::{
    ConstraintSpec, PaintSpec, Quantity, ScalarExpr, StackingOrder, StrokeSpec, TextAlign,
    TextOverflow, TransformSpec,
};
''',
)
replace(
    "src/authoring/visual.rs",
    '''    Group {
        id: String,
        #[serde(default)]
        transform: TransformSpec,
        #[serde(default)]
        #[schemars(length(max = 100))]
        constraints: Vec<ConstraintSpec>,
        #[serde(default)]
        children: Vec<VisualNode>,
    },
''',
    '''    Group {
        id: String,
        #[serde(default)]
        transform: TransformSpec,
        #[serde(default)]
        #[schemars(length(max = 100))]
        constraints: Vec<ConstraintSpec>,
        #[serde(default)]
        stacking: StackingOrder,
        #[serde(default)]
        children: Vec<VisualNode>,
    },
''',
)

replace(
    "src/authoring/mod.rs",
    '''    PoseTargetSpec, Quantity, RawSceneFragment, ScalarExpr, SourceMapEntry, StrokeSpec,
    TransformSpec, Unit, VisualSection,
''',
    '''    PoseTargetSpec, Quantity, RawSceneFragment, ScalarExpr, SourceMapEntry, StackingOrder,
    StrokeSpec, TransformSpec, Unit, VisualSection,
''',
)

replace(
    "src/authoring/frontend.rs",
    '''            visual: VisualSection {
                nodes: vec![VisualNode::Instance {
''',
    '''            visual: VisualSection {
                stacking: Default::default(),
                nodes: vec![VisualNode::Instance {
''',
)

replace(
    "src/authoring/lower.rs",
    '''        let visual_offset = children.len();
        let mut component_stack = Vec::new();
        for (index, node) in self.spec.visual.nodes.iter().enumerate() {
            let authored_path = format!("$.visual.nodes[{index}]");
            let authored_id = node.id().to_string();
            let runtime_segments = vec![self.spec.artboard.id.clone(), authored_id.clone()];
            let scene_path = format!("/artboard/children/{}", visual_offset + index);
            let child = self
                .lower_node(
                    node,
                    NodeContext {
                        authored_path,
                        definition_path: None,
                        authored_id,
                        runtime_segments,
                        scene_path,
                        scope: &self.spec.parameters,
                    },
                    &mut component_stack,
                )
                .map_err(AuthoringError::one)?;
            children.push(child);
        }
''',
    '''        let visual_offset = children.len();
        let visual_count = self.spec.visual.nodes.len();
        let mut component_stack = Vec::new();
        let mut lowered_visual = Vec::with_capacity(visual_count);
        for (authored_index, node) in self.spec.visual.nodes.iter().enumerate() {
            let scene_index = self
                .spec
                .visual
                .stacking
                .scene_index(authored_index, visual_count);
            let authored_path = format!("$.visual.nodes[{authored_index}]");
            let authored_id = node.id().to_string();
            let runtime_segments = vec![self.spec.artboard.id.clone(), authored_id.clone()];
            let scene_path = format!("/artboard/children/{}", visual_offset + scene_index);
            let child = self
                .lower_node(
                    node,
                    NodeContext {
                        authored_path,
                        definition_path: None,
                        authored_id,
                        runtime_segments,
                        scene_path,
                        scope: &self.spec.parameters,
                    },
                    &mut component_stack,
                )
                .map_err(AuthoringError::one)?;
            lowered_visual.push((scene_index, child));
        }
        lowered_visual.sort_by_key(|(scene_index, _)| *scene_index);
        children.extend(lowered_visual.into_iter().map(|(_, child)| child));
''',
)

replace(
    "src/authoring/lower/node.rs",
    '''            VisualNode::Group {
                transform,
                constraints,
                children,
                ..
            } => {
''',
    '''            VisualNode::Group {
                transform,
                constraints,
                stacking,
                children,
                ..
            } => {
''',
)
replace(
    "src/authoring/lower/node.rs",
    '''                let mut lowered_children = Vec::with_capacity(children.len());
                for (index, child) in children.iter().enumerate() {
                    let child_authored_path = format!("{authored_path}.children[{index}]");
                    let child_definition_path = definition_path
                        .as_ref()
                        .map(|path| format!("{path}.children[{index}]"));
                    let child_authored_id = format!("{authored_id}/{}", child.id());
                    let mut child_runtime_segments = runtime_segments.clone();
                    child_runtime_segments.push(child.id().to_string());
                    let child_scene_path = format!("{scene_path}/children/{index}");
                    lowered_children.push(self.lower_node(
                        child,
                        NodeContext {
                            authored_path: child_authored_path,
                            definition_path: child_definition_path,
                            authored_id: child_authored_id,
                            runtime_segments: child_runtime_segments,
                            scene_path: child_scene_path,
                            scope,
                        },
                        component_stack,
                    )?);
                }

                Ok(json!({
''',
    '''                let child_count = children.len();
                let mut lowered_children = Vec::with_capacity(child_count);
                for (authored_index, child) in children.iter().enumerate() {
                    let scene_index = stacking.scene_index(authored_index, child_count);
                    let child_authored_path =
                        format!("{authored_path}.children[{authored_index}]");
                    let child_definition_path = definition_path
                        .as_ref()
                        .map(|path| format!("{path}.children[{authored_index}]"));
                    let child_authored_id = format!("{authored_id}/{}", child.id());
                    let mut child_runtime_segments = runtime_segments.clone();
                    child_runtime_segments.push(child.id().to_string());
                    let child_scene_path = format!("{scene_path}/children/{scene_index}");
                    let lowered_child = self.lower_node(
                        child,
                        NodeContext {
                            authored_path: child_authored_path,
                            definition_path: child_definition_path,
                            authored_id: child_authored_id,
                            runtime_segments: child_runtime_segments,
                            scene_path: child_scene_path,
                            scope,
                        },
                        component_stack,
                    )?;
                    lowered_children.push((scene_index, lowered_child));
                }
                lowered_children.sort_by_key(|(scene_index, _)| *scene_index);
                let lowered_children = lowered_children
                    .into_iter()
                    .map(|(_, child)| child)
                    .collect::<Vec<_>>();

                Ok(json!({
''',
)
replace(
    "src/authoring/lower/node.rs",
    '''                let mut lowered_children = Vec::with_capacity(component_ref.spec.visual.len());
                for (index, child) in component_ref.spec.visual.iter().enumerate() {
                    let child_authored_path = format!("{authored_path}.expanded[{index}]");
                    let child_definition_path = Some(format!(
                        "$.components[{}].visual[{index}]",
                        component_ref.index
                    ));
                    let child_authored_id = format!("{authored_id}/{}", child.id());
                    let mut child_runtime_segments = runtime_segments.clone();
                    child_runtime_segments.push(child.id().to_string());
                    let child_scene_path = format!("{scene_path}/children/{index}");
                    match self.lower_node(
                        child,
                        NodeContext {
                            authored_path: child_authored_path,
                            definition_path: child_definition_path,
                            authored_id: child_authored_id,
                            runtime_segments: child_runtime_segments,
                            scene_path: child_scene_path,
                            scope: &component_scope,
                        },
                        component_stack,
                    ) {
                        Ok(child) => lowered_children.push(child),
                        Err(error) => {
                            component_stack.pop();
                            return Err(error);
                        }
                    }
                }
                component_stack.pop();

                Ok(json!({
''',
    '''                let child_count = component_ref.spec.visual.len();
                let mut lowered_children = Vec::with_capacity(child_count);
                for (authored_index, child) in component_ref.spec.visual.iter().enumerate() {
                    let scene_index = component_ref
                        .spec
                        .stacking
                        .scene_index(authored_index, child_count);
                    let child_authored_path =
                        format!("{authored_path}.expanded[{authored_index}]");
                    let child_definition_path = Some(format!(
                        "$.components[{}].visual[{authored_index}]",
                        component_ref.index
                    ));
                    let child_authored_id = format!("{authored_id}/{}", child.id());
                    let mut child_runtime_segments = runtime_segments.clone();
                    child_runtime_segments.push(child.id().to_string());
                    let child_scene_path = format!("{scene_path}/children/{scene_index}");
                    match self.lower_node(
                        child,
                        NodeContext {
                            authored_path: child_authored_path,
                            definition_path: child_definition_path,
                            authored_id: child_authored_id,
                            runtime_segments: child_runtime_segments,
                            scene_path: child_scene_path,
                            scope: &component_scope,
                        },
                        component_stack,
                    ) {
                        Ok(child) => lowered_children.push((scene_index, child)),
                        Err(error) => {
                            component_stack.pop();
                            return Err(error);
                        }
                    }
                }
                component_stack.pop();
                lowered_children.sort_by_key(|(scene_index, _)| *scene_index);
                let lowered_children = lowered_children
                    .into_iter()
                    .map(|(_, child)| child)
                    .collect::<Vec<_>>();

                Ok(json!({
''',
)

fixture_path = Path("examples/authoring/stacking-card.v0.json")
fixture = json.loads(fixture_path.read_text())
surface, cue = fixture["visual"]["nodes"]
surface["transform"] = {
    "x": {"kind": "literal", "value": 64.0, "unit": "px"},
    "y": {"kind": "literal", "value": 64.0, "unit": "px"},
}
cue["transform"]["x"]["value"] = 64.0
cue["transform"]["y"]["value"] = 64.0
fixture_path.write_text(json.dumps(fixture, indent=2) + "\n")

replace(
    "tests/authoring_stacking_contract.rs",
    '''    let legacy = lower(&input);
    let legacy_children = legacy.scene["artboard"]["children"]
        .as_array()
        .expect("artboard children");
    let legacy_surface = source_entry(&legacy.source_map, "surface");
    assert_eq!(legacy_children[0]["name"], legacy_surface.runtime_names[0]);

    input["visual"]["stacking"] = json!("back_to_front");
''',
    '''    let legacy = lower(&input);
    let legacy_children = legacy.scene["artboard"]["children"]
        .as_array()
        .expect("artboard children");
    let legacy_surface = source_entry(&legacy.source_map, "surface");
    assert_eq!(legacy_children[0]["name"], legacy_surface.runtime_names[0]);

    input["visual"]["stacking"] = json!("front_to_back");
    let explicit_compatibility = lower(&input);
    assert_eq!(legacy.scene, explicit_compatibility.scene);
    assert_eq!(legacy.source_map, explicit_compatibility.source_map);

    input["visual"]["stacking"] = json!("back_to_front");
''',
)

replace(
    "CHANGELOG.md",
    '''### Added

''',
    '''### Added

- AuthoringSpec visual sections, groups, and component definitions now accept an explicit `stacking` mode. `back_to_front` lets authors list an opaque surface before its foreground details while the compiler preserves canonical SceneSpec paint order, authored diagnostics, and source maps; omitted `front_to_back` behavior remains backward compatible.

''',
)

replace(
    "docs/authoring-spec-v0.md",
    '''Components declare parameters and a reusable visual graph. Instances select a component,
provide overrides, and receive a transform.

## Patterns
''',
    '''Components declare parameters and a reusable visual graph. Instances select a component,
provide overrides, and receive a transform.

## Stacking order

Root `visual` sections, groups, and component definitions accept an optional
`stacking` field:

- `front_to_back` is the compatibility default. The first authored sibling is the
  visually frontmost sibling.
- `back_to_front` lets authors write the natural composition order: opaque
  background first, then foreground details. The Authoring compiler reverses only
  the emitted SceneSpec sibling order.

```json
{
  "stacking": "back_to_front",
  "children": [
    { "kind": "rectangle", "id": "surface" },
    { "kind": "text", "id": "label" }
  ]
}
```

Authored indexes remain authoritative for diagnostics and source-map definition
paths. Scene paths identify the normalized runtime indexes. Raw SceneSpec input and
nested raw object internals keep the Rive runtime's native first-sibling-on-top
ordering.

## Patterns
''',
)

replace(
    "examples/authoring/README.md",
    '''- `complex-static-showcase.v0.json`: a composition proof that combines reusable
  components, parameter overrides, expression math, gradients, trimmed strokes,
  text, grid, radial, mirror, distribute, along-path patterns, and simple constraints.
''',
    '''- `complex-static-showcase.v0.json`: a composition proof that combines reusable
  components, parameter overrides, expression math, gradients, trimmed strokes,
  text, grid, radial, mirror, distribute, along-path patterns, and simple constraints.
- `stacking-card.v0.json`: a focused stacking proof where an opaque surface is
  authored before its foreground cue with `back_to_front`, then normalized by the
  compiler for the Rive runtime.
''',
)

replace(
    "skills/claude-code/commands/rive-generate.md",
    '''- Keep AuthoringSpec as the high-level source of intent.
- Keep SceneSpec as the canonical explicit lowered IR and expert escape hatch.
- Prefer typed AuthoringSpec concepts over raw escapes.
''',
    '''- Keep AuthoringSpec as the high-level source of intent.
- Keep SceneSpec as the canonical explicit lowered IR and expert escape hatch.
- Prefer typed AuthoringSpec concepts over raw escapes.
- When a visual list is naturally written as background then foreground, set
  `stacking` to `back_to_front` on the visual section, group, or component.
  Omitted `front_to_back` keeps the first authored sibling visually frontmost.
''',
)

replace(
    "src/ai/openai.rs",
    '''- Prefer typed visual primitives, components, patterns, constraints, poses, motion tracks, and behavior before raw escapes.\n\
- Use stable descriptive ids and parameterized components for repeated motifs.\n\
- Treat source-map paths as the repair interface when compilation fails.\n\
''',
    '''- Prefer typed visual primitives, components, patterns, constraints, poses, motion tracks, and behavior before raw escapes.\n\
- Use stable descriptive ids and parameterized components for repeated motifs.\n\
- When a visual list is authored as background then foreground, set stacking to back_to_front on that visual section, group, or component.\n\
- Treat source-map paths as the repair interface when compilation fails.\n\
''',
)
replace(
    "src/ai/openai.rs",
    '''        assert!(system.contains("current AuthoringSpec"));
        assert!(system.contains("incremental operations"));
        assert!(system.contains("raw_scene_object"));
''',
    '''        assert!(system.contains("current AuthoringSpec"));
        assert!(system.contains("incremental operations"));
        assert!(system.contains("stacking to back_to_front"));
        assert!(system.contains("raw_scene_object"));
''',
)

replace(
    "ROADMAP.md",
    '''[#201](https://github.com/George-RD/rive-rs-cli/issues/201) completes the independent
public-proof track in PR #219: the landing page now leads with original AuthoringSpec
work, the primary creation path points to the Made with rive-cli showcase, Horaxon is
represented as explicitly bounded production-consumer proof with retained local
artifact/generating-source provenance, and the Verification Lab remains the separate
correctness route.

No further execution slice is currently designated after #201. Select new work from
an explicit unblocked issue or create a bounded roadmap gap only when current evidence
justifies it; do not promote the independent lower-level coverage issues by default.
''',
    '''[#201](https://github.com/George-RD/rive-rs-cli/issues/201) completes the independent
public-proof track in PR #219: the landing page now leads with original AuthoringSpec
work, the primary creation path points to the Made with rive-cli showcase, Horaxon is
represented as explicitly bounded production-consumer proof with retained local
artifact/generating-source provenance, and the Verification Lab remains the separate
correctness route.

[#193](https://github.com/George-RD/rive-rs-cli/issues/193) completes explicit
AuthoringSpec stacking hardening in PR #220. Root visual sections, groups, and
components can opt into natural back-to-front authoring while lowering preserves
native SceneSpec paint order, authored diagnostics, source maps, and raw SceneSpec
semantics. A focused official-runtime pixel contract retains the original defect gate.

No further execution slice is currently designated after #193. Select new work from
an explicit unblocked issue or create a bounded roadmap gap only when current evidence
justifies it; do not promote the independent lower-level coverage issues by default.
''',
)
replace(
    "ROADMAP.md",
    '''| P1 | [Visual/component compiler slice](meta/todos/todo.visual-authoring-compiler.md) | complete in PR #156 | components, parameters, patterns, simple constraints, complex static showcase |
''',
    '''| P1 | [Visual/component compiler slice](meta/todos/todo.visual-authoring-compiler.md) | complete in PR #156; explicit stacking hardening in PR #220 | components, parameters, patterns, simple constraints, explicit stacking, complex static showcase |
''',
)
replace(
    "ROADMAP.md",
    '''## Delivery dependencies

- #176 completed the one-pass motion compiler architecture gate in PR #192.
''',
    '''## Delivery dependencies

- #193 completes a post-roadmap visual-authoring hardening slice in PR #220. The
  optional `back_to_front` contract normalizes AuthoringSpec containers without
  changing SceneSpec's native first-sibling-on-top semantics.
- #176 completed the one-pass motion compiler architecture gate in PR #192.
''',
)

replace(
    "meta/todos/todo.visual-authoring-compiler.md",
    '''- The showcase contract proves deterministic lowering and source maps, expanded authored
  IDs, canonical SceneSpec construction, `.riv` encoding, and structural validation. Its
  test helper also removes duplicated deterministic-lowering setup from the existing
  authoring examples.
- TDD run `30691630011` passed formatting, Clippy, all 614 library tests, and every prior
''',
    '''- The showcase contract proves deterministic lowering and source maps, expanded authored
  IDs, canonical SceneSpec construction, `.riv` encoding, and structural validation. Its
  test helper also removes duplicated deterministic-lowering setup from the existing
  authoring examples.
- PR #220 adds explicit stacking semantics after the original visual slice exit gate.
  Root visual sections, groups, and components accept `back_to_front` for natural
  background-to-foreground authoring while omitted `front_to_back` remains compatible.
- Stacking normalization lowers authored siblings in authored order, so diagnostics and
  source-map definition paths retain authored indexes even when canonical SceneSpec paths
  use reversed runtime indexes.
- A focused `stacking-card.v0.json` fixture and Chromium contract retain exact foreground
  and exposed-background pixels through the official Rive runtime. Raw SceneSpec keeps
  its native sibling ordering as the expert escape hatch.
- TDD run `30691630011` passed formatting, Clippy, all 614 library tests, and every prior
''',
)
replace(
    "meta/todos/todo.visual-authoring-compiler.md",
    '''- Common shapes do not require hand-authored shape/geometry/paint scaffolding.
- Components can be instantiated with parameter overrides and stable IDs.
''',
    '''- Common shapes do not require hand-authored shape/geometry/paint scaffolding.
- Root visuals, groups, and components can express natural back-to-front stacking without
  leaking reverse runtime paint order.
- Components can be instantiated with parameter overrides and stable IDs.
''',
)
