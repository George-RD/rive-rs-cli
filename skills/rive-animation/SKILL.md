---
name: rive-animation
description: Expert SceneSpec escape-hatch guidance for rive-cli. Use for bounded low-level Rive diagnostics or unsupported AuthoringSpec concepts; prefer AuthoringSpec for complex AI-authored work.
---

# Rive SceneSpec expert workflow

Complex AI generation should use the high-level `AuthoringSpec` path described in
`skills/README.md`. Use this skill only when you deliberately need the lower-level
`SceneSpec` IR: bounded diagnostics, known-good templates, parity work, or an
AuthoringSpec raw escape for a concept the typed schema does not yet represent.

## Do not copy schema facts from memory

`rive-cli` is authoritative for the current SceneSpec contract:

```bash
rive-cli schema
rive-cli types
rive-cli describe ellipse
rive-cli describe shape
```

Those commands are generated from the same Rust types and property resolver used
by compilation. Prefer them over copied object tables or historical skill text.

A SceneSpec document uses `"scene_format_version": 1`. Start from a checked-in or
CLI-generated working scene rather than inventing object structure:

```bash
rive-cli new --list
rive-cli new animated -o scene.json
rive-cli generate scene.json -o output.riv
```

## Runtime-sensitive rules

Treat runtime facts as measured claims, not schema assumptions. In particular:

- rotation values are radians;
- colors use `#RRGGBB` or `#RRGGBBAA` (alpha last);
- keyframe property names are type-specific; query `rive-cli describe <type>`;
- state-machine transitions address indices in that layer's declared state list;
- generated state-machine layers must satisfy the runtime requirements enforced by
  the current builder; do not hard-code historical system-state ordering rules;
- structural validation can pass while the official runtime rejects or visually
  misbehaves, so rendering is a separate gate.

Known runtime limitations and measured workarounds belong in `docs/parity.md` and
`CHANGELOG.md`, not duplicated into this skill.

## Visual structure

For direct SceneSpec work, remember that transform ownership matters. Geometry such
as an ellipse or rectangle is normally nested under a `shape`; position, rotation,
scale, and opacity are typically animated on the owning transform object while
geometry-specific dimensions belong to the geometry. Query `describe` for the
exact current fields and animatable properties before authoring.

Rive sibling draw order can be non-intuitive. Use rendered evidence rather than
assuming HTML/SVG stacking semantics. If a full-artboard opaque plate obscures the
composition, prefer `render --background` for verification instead of baking a
verification backdrop into the scene.

## Verification loop

```bash
rive-cli generate scene.json -o output.riv
rive-cli validate output.riv
rive-cli render output.riv --frames 0,15,30,45 --preview -o frames/
```

For interactive work, select the state machine and drive representative inputs or
pointer events with `render`. Inspect the retained frames/manifests before claiming
success.

Keep these evidence dimensions separate:

1. SceneSpec parses and compiles.
2. The `.riv` binary passes structural validation.
3. The official runtime loads and renders it.
4. Static, animated, or interactive output matches the intended concept.

A pass in one dimension is not proof of another.

## AuthoringSpec boundary

Before reaching for raw SceneSpec on a complex prompt, inspect the typed frontend:

```bash
rive-cli authoring schema
```

If the requested concept is represented there, author stable IDs and intent through
AuthoringSpec and compile with `rive-cli authoring compile`. If it is not represented,
use the documented raw escape or this direct SceneSpec workflow and state that
boundary explicitly. Do not invent typed fields or make the model manage generated
runtime indices merely for convenience.
