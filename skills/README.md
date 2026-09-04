# AI Agent Skills for rive-cli

Use the high-level `AuthoringSpec` contract for complex AI-authored Rive files.
Discover the current schema with `rive-cli authoring schema`, compile with
`rive-cli authoring compile`, then validate and render representative frames or
interactions before claiming the result works.

The repository keeps [`rive-animation/SKILL.md`](rive-animation/SKILL.md) as the
low-level SceneSpec expert reference. It remains useful for bounded diagnostics,
known-good SceneSpec templates, parity work, and explicit raw escape-hatch cases;
it is not the default interface for complex AI generation.

## Authoring boundary

`AuthoringSpec` expresses visual, motion, and behavior intent through stable
written IDs. The compiler deterministically lowers that intent to canonical
`SceneSpec` and retains source-map evidence. Agents should not manage generated
runtime names, containment objects, state-array indices, or similar lower-level
bookkeeping when a typed AuthoringSpec concept exists.

Current typed coverage is defined by the generated schema and demonstrated by the
checked-in examples:

```bash
rive-cli authoring schema
rive-cli authoring compile examples/authoring/complex-static-showcase.v0.json -o static.riv
rive-cli authoring compile examples/authoring/complex-animated-showcase.v0.json -o animated.riv
rive-cli authoring compile examples/authoring/complex-interactive-showcase.v0.json -o interactive.riv
rive-cli authoring compile examples/authoring/interactive-console.v0.json -o console.riv
rive-cli authoring compile examples/authoring/signal-weave.v0.json -o weave.riv
```

Those showcases prove the supported static, motion, and interaction subsets; the
console adds `stacking`, motion `continuity`, a 1D blend state driven by a number
input, and parallel statechart regions. None of them imply that every Rive
concept is typed, so read the current field names and enum values from
`rive-cli authoring schema` rather than from this file. Where a requested concept
is not in the current schema, use an explicit AuthoringSpec raw escape or the
direct SceneSpec workflow and identify that boundary rather than inventing typed
fields.

## Verification loop

Keep structural, runtime, and semantic evidence separate:

```bash
rive-cli authoring compile input.authoring.json -o output.riv
rive-cli validate output.riv
rive-cli render output.riv --frames 0,15,30,45 --preview -o frames/
```

For state-machine work, drive the authored interaction through the appropriate
`render --state-machine`, `--input`, or `--pointer` controls and retain the
representative output. A structural `validate` pass alone does not prove runtime
loading or intended behavior.

## Install and use

Copy or symlink the low-level expert skill into the skills directory used by your
agent when SceneSpec expertise is required:

```bash
cp skills/rive-animation/SKILL.md ~/.config/opencode/skills/rive-animation.md
ln -s /path/to/rive-rs-cli/skills/rive-animation/SKILL.md ~/.config/opencode/skills/rive-animation.md
```

For Claude Code, the slash-command files provide aliases for Authoring-first
generation plus inspect and validate:

```bash
mkdir -p ~/.claude/commands
cp skills/claude-code/commands/*.md ~/.claude/commands/
```

The optional MCP configuration template is `claude-code/mcp-config.json`; enable it
with a build that includes the `mcp` feature. MCP exposes the high-level
`schema://authoring/v0` resource and `generate_authoring` tool; the SceneSpec
resource/tool remains the lower-level expert path.

## SceneSpec expert reference

For bounded low-level work, the CLI remains authoritative for SceneSpec:

```bash
rive-cli new --list
rive-cli types
rive-cli describe ellipse
rive-cli schema
rive-cli generate scene.json -o output.riv
```

SceneSpec requires `"scene_format_version": 1`. Do not multiply specialized
complex-generation skills around this lower-level representation.

## See also

- Main project: [`../AGENTS.md`](../AGENTS.md)
- Authoring examples: [`../examples/authoring/README.md`](../examples/authoring/README.md)
- Authoring contract: [`../docs/authoring-spec-v0.md`](../docs/authoring-spec-v0.md)
- SceneSpec schema: [`../docs/scene.schema.v1.json`](../docs/scene.schema.v1.json)
- Low-level expert skill: [`rive-animation/SKILL.md`](rive-animation/SKILL.md)
- CLI usage: `rive-cli --help`
