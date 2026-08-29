---
node: rive-cli.intelligence.ai
---
# AI subsystem contract

Complex prompt generation targets `AuthoringSpec` as its primary structured output.
The AI path must use the same typed Authoring frontend, deterministic lowering,
source maps, canonical SceneSpec compilation seam, and evidence model as manually
authored AuthoringSpec documents.

`SceneSpec` remains the canonical lower-level IR and explicit expert escape hatch.
Built-in SceneSpec templates, direct `generate`, diagnostics, and unsupported typed
concepts may still use it, but complex prompt workflows must not teach models to
manage runtime indices, generated containment, or state-array bookkeeping directly.

When a generated AuthoringSpec is structurally parseable but fails typed lowering,
repair must target the smallest failed authored concept through the incremental
stable-ID operation seam. Each proposed edit is atomic and reuses whole-document
Authoring validation; repair must not replace the full document merely to correct
one authored concept. Source-map context should be supplied whenever the failure
occurs after a successful lowering and runtime evidence needs to be related back to
authored identity.

Generation and evaluation must report evidence in separate dimensions:

- Authoring schema/lowering validity and authored-path diagnostics;
- expected feature traits;
- pipeline reproducibility and baseline drift;
- official-runtime loading and render evidence;
- static, animated, and interactive semantic prompt satisfaction.

Passing one dimension must never be presented as proof of another. Skills and
examples must describe the supported typed subset accurately and identify raw
SceneSpec escapes explicitly where a requested Rive concept is not yet represented
by AuthoringSpec.
