---
name: rive-validation
description: Verification workflow for Rive files authored with rive-cli.
---

# Rive validation

Use the repository's Authoring-first guidance in `skills/README.md` and the live CLI discovery commands rather than duplicating schema or runtime rules here.

For complex AI-authored work:

```bash
rive-cli authoring schema
rive-cli authoring compile input.authoring.json -o output.riv
rive-cli validate output.riv
rive-cli render output.riv --frames 0,15,30,45 --preview -o frames/
```

For interactive work, drive representative state-machine inputs or pointer interactions with `rive-cli render` and retain the output evidence.

Keep these claims separate:

- successful AuthoringSpec lowering and `.riv` structural validation;
- official-runtime loading and rendering;
- static, animated, or interactive semantic satisfaction.

Use direct SceneSpec generation only as the explicit expert escape hatch. Query `rive-cli schema`, `rive-cli types`, and `rive-cli describe` for current lower-level syntax instead of relying on copied schema tables.
