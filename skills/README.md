# AI Agent Skills for rive-cli

This directory contains skill definitions and configuration for AI agents to work with the rive-cli Rive animation compiler.

## Overview

| File | Purpose |
|------|---------|
| `opencode/rive-animation.md` | OpenCode skill for generating Rive animations |
| `claude-code/commands/rive-generate.md` | Claude Code slash command for generating .riv files |
| `claude-code/commands/rive-inspect.md` | Claude Code slash command for inspecting .riv files |
| `claude-code/commands/rive-validate.md` | Claude Code slash command for validating .riv files |
| `claude-code/mcp-config.json` | MCP server configuration for Claude Code/Claude Desktop |

## OpenCode Installation

Copy or symlink the skill file to your OpenCode skills directory:

```bash
# macOS/Linux
cp skills/opencode/rive-animation.md ~/.config/opencode/skills/

# Or create a symlink for easier updates
ln -s /path/to/rive-rs-cli/skills/opencode/rive-animation.md ~/.config/opencode/skills/
```

Then reference the skill in your agent configuration or prompts.

## Claude Code Installation

### Slash Commands

Copy the command files to your Claude Code commands directory:

```bash
# macOS
mkdir -p ~/.claude/commands
cp skills/claude-code/commands/*.md ~/.claude/commands/

# Or create symlinks
ln -s /path/to/rive-rs-cli/skills/claude-code/commands/rive-generate.md ~/.claude/commands/
ln -s /path/to/rive-rs-cli/skills/claude-code/commands/rive-inspect.md ~/.claude/commands/
ln -s /path/to/rive-rs-cli/skills/claude-code/commands/rive-validate.md ~/.claude/commands/
```

### MCP Server Configuration

Add the MCP server config to your Claude Code or Claude Desktop settings:

**Claude Code (project-level):**

Create or edit `.claude/mcp-config.json` in your project root:

```json
{
  "mcpServers": {
    "rive-cli": {
      "command": "cargo",
      "args": ["run", "--features", "mcp", "--", "--mcp"],
      "cwd": "/path/to/rive-rs-cli"
    }
  }
}
```

**Claude Desktop:**

Edit `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS):

```json
{
  "mcpServers": {
    "rive-cli": {
      "command": "cargo",
      "args": ["run", "--features", "mcp", "--", "--mcp"],
      "cwd": "/path/to/rive-rs-cli"
    }
  }
}
```

Update the `cwd` path to point to your rive-rs-cli installation.

## Usage

### OpenCode

Once installed, the skill enables your agent to:

- Generate SceneSpec JSON for Rive animations
- Understand the object hierarchy (artboard → shape → path/paint)
- Follow anti-patterns that prevent runtime failures
- Reference the schema at `docs/scene.schema.v1.json`

### Claude Code Slash Commands

After installation, these commands become available:

- `/rive-generate "a red circle that pulses"` — Generate a .riv file from description
- `/rive-inspect output.riv` — Inspect the object tree of a .riv file
- `/rive-validate output.riv` — Validate a .riv file for structural correctness

## File Structure

```text
skills/
├── README.md                              # This file
├── opencode/
│   └── rive-animation.md                  # OpenCode skill definition
└── claude-code/
    ├── commands/
    │   ├── rive-generate.md               # Generate command
    │   ├── rive-inspect.md                # Inspect command
    │   └── rive-validate.md               # Validate command
    └── mcp-config.json                    # MCP server config template
```

## Schema Reference

All skills reference `docs/scene.schema.v1.json` as the authoritative schema for SceneSpec JSON. The schema is versioned; always use `"scene_format_version": 1`.

## See Also

- Main project: `../AGENTS.md` — Project knowledge base
- Schema: `../docs/scene.schema.v1.json` — JSON schema reference
- CLI usage: `cargo run -- --help`
