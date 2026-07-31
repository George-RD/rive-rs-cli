#!/usr/bin/env python3
from __future__ import annotations

import os
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def replace_exact(relative_path: str, old: str, new: str, expected: int = 1) -> None:
    path = ROOT / relative_path
    text = path.read_text(encoding="utf-8")
    actual = text.count(old)
    if actual != expected:
        raise SystemExit(
            f"{relative_path}: expected {expected} occurrence(s), found {actual}: {old[:100]!r}"
        )
    path.write_text(text.replace(old, new, expected), encoding="utf-8")


def write_new(relative_path: str, content: str) -> None:
    path = ROOT / relative_path
    if path.exists():
        raise SystemExit(f"{relative_path}: refusing to overwrite existing file")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def run(*command: str, env: dict[str, str] | None = None) -> None:
    print("+", " ".join(command), flush=True)
    subprocess.run(command, cwd=ROOT, check=True, env=env)


replace_exact(
    "src/authoring/spec.rs",
    """    #[serde(default)]
    pub font_assets: BTreeMap<String, String>,
    #[serde(default)]
    pub parameters: BTreeMap<String, Quantity>,
""",
    """    #[serde(default)]
    pub font_assets: BTreeMap<String, String>,
    #[serde(default)]
    pub image_assets: BTreeMap<String, String>,
    #[serde(default)]
    pub parameters: BTreeMap<String, Quantity>,
""",
)

replace_exact(
    "src/authoring/visual.rs",
    """        #[serde(default)]
        overflow: TextOverflow,
        #[serde(default)]
        transform: TransformSpec,
    },
    Grid {
""",
    """        #[serde(default)]
        overflow: TextOverflow,
        #[serde(default)]
        transform: TransformSpec,
    },
    Image {
        id: String,
        asset: String,
        #[serde(default)]
        transform: TransformSpec,
    },
    Grid {
""",
)
replace_exact(
    "src/authoring/visual.rs",
    """#[derive(Clone, Copy)]
pub(crate) struct GridNodeRef<'a> {
""",
    """#[derive(Clone, Copy)]
pub(crate) struct ImageNodeRef<'a> {
    pub asset: &'a str,
    pub transform: &'a TransformSpec,
}

#[derive(Clone, Copy)]
pub(crate) struct GridNodeRef<'a> {
""",
)
replace_exact(
    "src/authoring/visual.rs",
    """            | Self::Text { id, .. }
            | Self::Grid { id, .. }
""",
    """            | Self::Text { id, .. }
            | Self::Image { id, .. }
            | Self::Grid { id, .. }
""",
)
replace_exact(
    "src/authoring/visual.rs",
    """            Self::Text { .. }
            | Self::Grid { .. }
""",
    """            Self::Text { .. }
            | Self::Image { .. }
            | Self::Grid { .. }
""",
)
replace_exact(
    "src/authoring/visual.rs",
    """    pub(crate) fn pattern(&self) -> Option<PatternNodeRef<'_>> {
""",
    """    pub(crate) fn image_node(&self) -> Option<ImageNodeRef<'_>> {
        match self {
            Self::Image {
                asset, transform, ..
            } => Some(ImageNodeRef { asset, transform }),
            _ => None,
        }
    }

    pub(crate) fn pattern(&self) -> Option<PatternNodeRef<'_>> {
""",
)

replace_exact(
    "src/authoring/frontend.rs",
    """    validate_font_assets(&spec.font_assets, &mut diagnostics);
    validate_parameter_names(&spec.parameters, "$.parameters", &mut diagnostics);
""",
    """    validate_file_assets(
        &spec.font_assets,
        "$.font_assets",
        "font",
        &mut diagnostics,
    );
    validate_file_assets(
        &spec.image_assets,
        "$.image_assets",
        "image",
        &mut diagnostics,
    );
    validate_parameter_names(&spec.parameters, "$.parameters", &mut diagnostics);
""",
)
replace_exact(
    "src/authoring/frontend.rs",
    """fn validate_font_assets(
    font_assets: &BTreeMap<String, String>,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (id, source) in font_assets {
        if !is_authored_map_key(id) {
            diagnostics.push(AuthoringDiagnostic::new(
                "$.font_assets",
                "invalid_asset_id",
                format!("font asset id '{id}' must contain only ASCII letters, digits, '_' or '-'"),
            ));
        }
        if source.trim().is_empty() {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("$.font_assets.{id}"),
                "invalid_asset_source",
                "font asset source must not be empty",
            ));
        }
    }
}
""",
    """fn validate_file_assets(
    assets: &BTreeMap<String, String>,
    list_path: &str,
    kind: &str,
    diagnostics: &mut Vec<AuthoringDiagnostic>,
) {
    for (id, source) in assets {
        if !is_authored_map_key(id) {
            diagnostics.push(AuthoringDiagnostic::new(
                list_path,
                "invalid_asset_id",
                format!(
                    "{kind} asset id '{id}' must contain only ASCII letters, digits, '_' or '-'"
                ),
            ));
        }
        if source.trim().is_empty() {
            diagnostics.push(AuthoringDiagnostic::new(
                format!("{list_path}.{id}"),
                "invalid_asset_source",
                format!("{kind} asset source must not be empty"),
            ));
        }
    }
}
""",
)
replace_exact(
    "src/authoring/frontend.rs",
    """            font_assets: spec.font_assets.clone(),
            parameters: BTreeMap::new(),
""",
    """            font_assets: spec.font_assets.clone(),
            image_assets: spec.image_assets.clone(),
            parameters: BTreeMap::new(),
""",
)

replace_exact(
    "src/authoring/lower.rs",
    """mod node;
mod paint;
""",
    """mod image;
mod node;
mod paint;
""",
)
replace_exact(
    "src/authoring/lower.rs",
    """        let mut children =
            Vec::with_capacity(self.spec.font_assets.len() + self.spec.visual.nodes.len());
        for (index, (id, source)) in self.spec.font_assets.iter().enumerate() {
            let runtime_name = font_asset_runtime_name(&self.spec.artboard.id, id);
            let authored_path = format!("$.font_assets.{id}");
            self.register_runtime_names(std::slice::from_ref(&runtime_name), &authored_path)
                .map_err(AuthoringError::one)?;
            self.source_map.entries.push(SourceMapEntry {
                authored_id: id.clone(),
                authored_path,
                definition_path: None,
                runtime_names: vec![runtime_name.clone()],
                scene_paths: vec![format!("/artboard/children/{index}")],
            });
            children.push(json!({
                "type": "font_asset",
                "name": runtime_name,
                "source": source
            }));
        }
        let visual_offset = children.len();
""",
    """        let spec = self.spec;
        let mut children = Vec::with_capacity(
            spec.font_assets.len() + spec.image_assets.len() + spec.visual.nodes.len(),
        );
        for (list_path, role, assets) in [
            ("$.font_assets", "font_asset", &spec.font_assets),
            ("$.image_assets", "image_asset", &spec.image_assets),
        ] {
            for (id, source) in assets {
                let index = children.len();
                let runtime_name = file_asset_runtime_name(&spec.artboard.id, id, role);
                let authored_path = format!("{list_path}.{id}");
                self.register_runtime_names(std::slice::from_ref(&runtime_name), &authored_path)
                    .map_err(AuthoringError::one)?;
                self.source_map.entries.push(SourceMapEntry {
                    authored_id: id.clone(),
                    authored_path,
                    definition_path: None,
                    runtime_names: vec![runtime_name.clone()],
                    scene_paths: vec![format!("/artboard/children/{index}")],
                });
                children.push(json!({
                    "type": role,
                    "name": runtime_name,
                    "source": source
                }));
            }
        }
        let visual_offset = children.len();
""",
)
replace_exact(
    "src/authoring/lower.rs",
    """fn font_asset_runtime_name(artboard_id: &str, asset_id: &str) -> String {
    runtime_name(
        &[artboard_id.to_string(), asset_id.to_string()],
        "font_asset",
    )
}
""",
    """fn file_asset_runtime_name(artboard_id: &str, asset_id: &str, role: &str) -> String {
    runtime_name(&[artboard_id.to_string(), asset_id.to_string()], role)
}

fn font_asset_runtime_name(artboard_id: &str, asset_id: &str) -> String {
    file_asset_runtime_name(artboard_id, asset_id, "font_asset")
}

fn image_asset_runtime_name(artboard_id: &str, asset_id: &str) -> String {
    file_asset_runtime_name(artboard_id, asset_id, "image_asset")
}
""",
)

write_new(
    "src/authoring/lower/image.rs",
    """use serde_json::{Value, json};

use super::super::expression::evaluate_transform;
use super::super::spec::{AuthoringDiagnostic, SourceMapEntry};
use super::super::visual::ImageNodeRef;
use super::{Lowerer, NodeContext, image_asset_runtime_name, runtime_name};

impl<'a> Lowerer<'a> {
    pub(super) fn lower_image(
        &mut self,
        image: ImageNodeRef<'_>,
        context: NodeContext<'_>,
    ) -> Result<Value, AuthoringDiagnostic> {
        let ImageNodeRef { asset, transform } = image;
        let NodeContext {
            authored_path,
            definition_path,
            authored_id,
            runtime_segments,
            scene_path,
            scope,
        } = context;

        if !self.spec.image_assets.contains_key(asset) {
            return Err(AuthoringDiagnostic::new(
                format!("{authored_path}.asset"),
                "unknown_image_asset",
                format!("image asset '{asset}' is not declared"),
            ));
        }

        let transform_values =
            evaluate_transform(transform, &format!("{authored_path}.transform"), scope)?;
        let anchor_name = runtime_name(&runtime_segments, "image_anchor");
        let image_name = runtime_name(&runtime_segments, "image");
        let image_scene_path = format!("{scene_path}/children/0");
        let runtime_names = vec![anchor_name.clone(), image_name.clone()];
        let scene_paths = vec![scene_path.clone(), image_scene_path];
        self.register_runtime_names(&runtime_names, &format!("{authored_path}.id"))?;
        self.source_map.entries.push(SourceMapEntry {
            authored_id,
            authored_path,
            definition_path,
            runtime_names,
            scene_paths,
        });

        Ok(json!({
            "type": "node",
            "name": anchor_name,
            "x": transform_values.x,
            "y": transform_values.y,
            "rotation": transform_values.rotation,
            "scale_x": transform_values.scale_x,
            "scale_y": transform_values.scale_y,
            "children": [{
                "type": "image",
                "name": image_name,
                "asset": image_asset_runtime_name(&self.spec.artboard.id, asset)
            }]
        }))
    }
}
""",
)

replace_exact(
    "src/authoring/lower/node.rs",
    """        if let Some(text) = node.text_node() {
            return self.lower_text(text, context);
        }

        let NodeContext {
""",
    """        if let Some(text) = node.text_node() {
            return self.lower_text(text, context);
        }
        if let Some(image) = node.image_node() {
            return self.lower_image(image, context);
        }

        let NodeContext {
""",
)
replace_exact(
    "src/authoring/lower/node.rs",
    """            | VisualNode::Text { .. }
            | VisualNode::Grid { .. }
            | VisualNode::Radial { .. } => {
                unreachable!("shape, text, and pattern nodes are handled above")
""",
    """            | VisualNode::Text { .. }
            | VisualNode::Image { .. }
            | VisualNode::Grid { .. }
            | VisualNode::Radial { .. } => {
                unreachable!("shape, text, image, and pattern nodes are handled above")
""",
)

replace_exact(
    "src/authoring/validation.rs",
    """    if let Some(pattern) = node.pattern() {
""",
    """    if let Some(image) = node.image_node() {
        validate_transform(image.transform, &format!("{path}.transform"), diagnostics);
        return;
    }

    if let Some(pattern) = node.pattern() {
""",
)
replace_exact(
    "src/authoring/validation.rs",
    """        | VisualNode::Text { .. }
        | VisualNode::Grid { .. }
        | VisualNode::Radial { .. } => {
            unreachable!("shape, text, and pattern nodes are handled above")
""",
    """        | VisualNode::Text { .. }
        | VisualNode::Image { .. }
        | VisualNode::Grid { .. }
        | VisualNode::Radial { .. } => {
            unreachable!("shape, text, image, and pattern nodes are handled above")
""",
)

replace_exact(
    "docs/authoring-spec-v0.md",
    """- `font_assets`: semantic font IDs mapped to file sources.
- `components`: reusable authored visual definitions with typed parameter defaults.
""",
    """- `font_assets`: semantic font IDs mapped to file sources.
- `image_assets`: semantic image IDs mapped to file sources.
- `components`: reusable authored visual definitions with typed parameter defaults.
""",
)
replace_exact(
    "docs/authoring-spec-v0.md",
    """The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, triangles, polygons, stars, literal text, groups, component instances, deterministic grid and radial patterns, semantic font assets, and raw `SceneSpec` objects. Shapes and text share one solid/linear/radial paint contract; stroke width is a positive pixel expression, and strokes may include a typed trim path. Polygon and star point counts must be at least three; star inner radius is a scalar ratio from zero to one. Image assets, mirror/distribute/along-path patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.
""",
    """The visual compiler slice is intentionally narrow. It supports ellipses, rectangles, triangles, polygons, stars, literal text, static images, groups, component instances, deterministic grid and radial patterns, semantic font and image assets, and raw `SceneSpec` objects. Shapes and text share one solid/linear/radial paint contract; stroke width is a positive pixel expression, and strokes may include a typed trim path. Polygon and star point counts must be at least three; star inner radius is a scalar ratio from zero to one. Mirror/distribute/along-path patterns, constraints, motion helpers, and statechart authoring remain separate roadmap items.
""",
)
replace_exact(
    "docs/authoring-spec-v0.md",
    """Parameter names and font asset IDs must contain only ASCII letters, digits, `_`, or `-`. This keeps semantic references and diagnostic paths unambiguous.
""",
    """Parameter names and font or image asset IDs must contain only ASCII letters, digits, `_`, or `-`. This keeps semantic references and diagnostic paths unambiguous.
""",
)
replace_exact(
    "docs/authoring-spec-v0.md",
    """Font assets lower in sorted ID order before visual nodes. Each asset receives a deterministic runtime name and its own source-map entry. Text may reference the semantic ID through `font`; unknown IDs fail at the authored text path. Lowering preserves the source in returned `SceneSpec` while keeping compiler validation independent of the filesystem. The canonical builder embeds the file bytes when its caller supplies an explicit base directory.

## Text
""",
    """Font assets lower in sorted ID order before visual nodes. Each asset receives a deterministic runtime name and its own source-map entry. Text may reference the semantic ID through `font`; unknown IDs fail at the authored text path. Lowering preserves the source in returned `SceneSpec` while keeping compiler validation independent of the filesystem. The canonical builder embeds the file bytes when its caller supplies an explicit base directory.

## Image assets

A document declares images by semantic ID and references them from transformable static image nodes:

```json
"image_assets": {
  "aurora": "assets/textures/aurora.png"
}
```

```json
{
  "kind": "image",
  "id": "backdrop",
  "asset": "aurora",
  "transform": {
    "x": { "kind": "literal", "value": 160, "unit": "px" },
    "y": { "kind": "literal", "value": 120, "unit": "px" }
  }
}
```

Font assets lower first, followed by image assets, each registry sorted by authored ID. Image nodes reference the generated asset name rather than a runtime ordinal, and unknown IDs fail at the authored `asset` path. The returned `SceneSpec` keeps the source; the canonical builder resolves the global image ordinal and embeds bytes when given an explicit base directory.

## Text
""",
)

replace_exact(
    "meta/contracts/authoring.md",
    """- semantic font asset IDs that text can reference without runtime indices;
- deterministic file-scope asset ordering and collision-checked runtime names;
""",
    """- semantic font asset IDs that text can reference without runtime indices;
- semantic image asset IDs that static image nodes can reference without runtime indices;
- deterministic file-scope asset ordering and collision-checked runtime names;
""",
)

replace_exact(
    "meta/todos/todo.visual-authoring-compiler.md",
    """- PR #148 also proves actual vendored TTF byte embedding through the canonical
  builder with an explicit base directory. Pathless compiler validation preserves
  the returned asset source while avoiding filesystem-dependent lowering.
- Font and parameter map keys share one strict validator instead of maintaining
  duplicate identifier rules. The published AuthoringSpec schema now has a tested
  regeneration path.
""",
    """- PR #148 also proves actual vendored TTF byte embedding through the canonical
  builder with an explicit base directory. Pathless compiler validation preserves
  the returned asset source while avoiding filesystem-dependent lowering.
- Semantic image assets and transformable static image nodes are implemented in PR
  #149. Fonts lower before images, both registries are sorted by authored ID, and
  named references flow through the canonical global asset-ordinal resolver.
- PR #149 proves actual vendored PNG byte embedding and complete source maps for
  root and component-expanded image nodes without exposing runtime indices.
- Font, image, and parameter map keys reuse one strict authored-key rule; file-asset
  declarations share one validator and one deterministic lowering loop. The
  published AuthoringSpec schema has a tested regeneration path.
""",
)
replace_exact(
    "meta/todos/todo.visual-authoring-compiler.md",
    """- Remaining work includes image assets, mirror/distribute/along-path patterns,
  constraints, and a complex static showcase without raw escapes.
""",
    """- Remaining work includes mirror/distribute/along-path patterns, constraints,
  and a complex static showcase without raw escapes.
""",
)

run("cargo", "fmt")
run("cargo", "test", "--test", "authoring_image_asset_contract")
run(
    "cargo",
    "test",
    "--test",
    "authoring_contract",
    "regenerate_published_authoring_schema",
    "--",
    "--ignored",
    "--exact",
)
run("cargo", "fmt", "--check")
run("cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings")
run("cargo", "test", "--locked", "--all-features")

run(
    "bash",
    "-lc",
    "curl --proto '=https' --tlsv1.2 -LsSf https://github.com/cairn-framework/cairn/releases/download/v0.9.0/cairn-installer.sh | sh",
)
cairn_env = os.environ.copy()
cairn_env["PATH"] = f"{Path.home() / '.local/bin'}:{Path.home() / '.cargo/bin'}:{cairn_env['PATH']}"
run("cairn", "scan", "--json", env=cairn_env)
run("cairn", "lint", "--json", env=cairn_env)

for generated in ["cairn-scan.json", "cairn-lint.json"]:
    path = ROOT / generated
    if path.exists():
        path.unlink()

(ROOT / ".github/workflows/stage-image-asset.yml").unlink()
Path(__file__).unlink()

run("git", "diff", "--check")
run("git", "status", "--short")
run("git", "config", "user.name", "github-actions[bot]")
run("git", "config", "user.email", "41898282+github-actions[bot]@users.noreply.github.com")
run("git", "add", "-A")
run("git", "commit", "-m", "Add semantic image asset lowering")
run("git", "push", "origin", "HEAD:agent/image-asset-v0")
