---
node: rive-cli.core.builder
---
# Canonical SceneSpec builder contract

`SceneSpec` is the complete, explicit, deterministic representation consumed by
the existing builder and encoder. It is the lowered IR, not the preferred model
input for complex AI authoring.

The builder must:

- reject invalid hierarchy, references, names, properties, and cycles before emission;
- keep reference resolution deterministic and independent of declaration accidents;
- preserve the public SceneSpec v1 contract while the Authoring frontend evolves;
- accept only explicit runtime concepts. Parametric sugar belongs in the Authoring module;
- remain usable directly as an expert/raw escape hatch.
