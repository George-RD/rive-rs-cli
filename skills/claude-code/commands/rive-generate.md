# Rive Generate

Generate a Rive .riv animation file from a natural language description.

Usage: /rive-generate <description>

Steps:
1. Parse the user's description to understand the desired animation
2. Create a SceneSpec JSON following docs/scene.schema.v1.json
3. Save the JSON to a temporary file
4. Run: cargo run -- generate <input.json> -o <output.riv>
5. Run: cargo run -- validate <output.riv>
6. If validation fails, fix the JSON and retry
7. Report the output file path

Rules:
- Always include scene_format_version: 1
- Use descriptive object names
- Use string enums (cap="round", join="miter") not integers
- Use #RRGGBB or #RRGGBBAA color format
- Reference docs/scene.schema.v1.json for full property list
