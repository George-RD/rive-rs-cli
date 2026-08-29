# Rive Generate

Generate a Rive `.riv` file from a natural-language description through the
high-level AuthoringSpec path.

Usage: `/rive-generate <description>`

Steps:
1. Inspect the current contract with `cargo run -- authoring schema`.
2. Use the closest checked-in AuthoringSpec example under `examples/authoring/` as
   guidance for static, animated, or interactive intent.
3. Author `AuthoringSpec` with stable descriptive IDs. Prefer typed visual, motion,
   and behavior concepts; use a raw escape only when the schema does not represent
   the requested Rive concept.
4. Compile through `cargo run -- authoring compile <input.json> -o <output.riv>`.
5. Run `cargo run -- validate <output.riv>` and then render representative frames or
   interactions with the official runtime before claiming the result works.
6. If typed lowering fails, correct the smallest authored concept named by the
   authored-path diagnostic rather than regenerating unrelated content.
7. Report the output path and the verification performed.

Rules:
- `authoring_format_version` is `0` for the current AuthoringSpec contract.
- Do not author runtime indices, generated runtime names, SceneSpec containment, or
  state-array positions when a typed AuthoringSpec concept exists.
- Treat `cargo run -- authoring schema` as authoritative for current typed coverage.
- Raw `SceneSpec` via `cargo run -- generate ...` is the explicit expert escape hatch
  for bounded low-level work or unsupported typed concepts, not the default complex
  AI authoring target.
- Structural validation is not runtime or semantic proof; retain those evidence
  dimensions separately.
