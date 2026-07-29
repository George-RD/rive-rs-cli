# AI Agent Skills for rive-cli

The primary authoring skill is [`rive-animation/SKILL.md`](rive-animation/SKILL.md). It is the authoritative, tool-driven workflow for creating, validating, and rendering Rive animations. It tells an agent to scaffold a known-good SceneSpec, discover types and animatable properties from the CLI, generate and validate the `.riv`, then render frames with `--preview`.


## Scope boundary

The current skill is a low-level expert workflow for bounded scenes. It is not the
planned abstraction for reliably generating large, reusable, behavior-heavy Rive
files. Do not multiply specialized skills around raw SceneSpec. Complex AI
authoring is blocked by `meta/todos/todo.ai-generation-skills.md` until the
AuthoringSpec compiler and runtime and semantic evaluation gates in `ROADMAP.md`
are available.

## Install and use

Copy or symlink the primary skill into the skills directory used by your agent:

```bash
cp skills/rive-animation/SKILL.md ~/.config/opencode/skills/rive-animation.md
ln -s /path/to/rive-rs-cli/skills/rive-animation/SKILL.md ~/.config/opencode/skills/rive-animation.md
```

For another agent host, place the file in that host's skill/instructions directory, or reference it directly from the repository. No generated configuration is required. The skill assumes `rive-cli` is on `PATH`; otherwise use the built binary at `target/release/rive-cli`.

The six JSON scenes in [`../showcase/`](../showcase/) are examples authored with this workflow. The CLI remains the source of truth for the schema:

```bash
rive-cli new --list
rive-cli types
rive-cli describe ellipse
rive-cli schema
```

## Legacy integrations

The former OpenCode file `opencode/rive-animation.md` was removed because its animation table contradicted the CLI's authoritative resolver. In particular, `stroke.thickness` and trim `start`/`end`/`offset` are not accepted keyframe properties.

The Claude Code slash-command files remain available for users who want command aliases for generate, inspect, and validate:

```bash
mkdir -p ~/.claude/commands
cp skills/claude-code/commands/*.md ~/.claude/commands/
```

The optional MCP configuration template is `claude-code/mcp-config.json`; enable it with a build that includes the `mcp` feature.

## Schema reference

All authoring tools use `docs/scene.schema.v1.json` as the generated SceneSpec contract. Scene specs require `"scene_format_version": 1`.

## See also

- Main project: [`../AGENTS.md`](../AGENTS.md)
- Schema: [`../docs/scene.schema.v1.json`](../docs/scene.schema.v1.json)
- Primary skill: [`rive-animation/SKILL.md`](rive-animation/SKILL.md)
- CLI usage: `rive-cli --help`
