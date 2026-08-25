# Issue tracker: GitHub

Issues, specs, and implementation tickets for this repo live in GitHub Issues at `George-RD/rive-rs-cli`. Use the `gh` CLI when working from a clone; connected agents may use their GitHub integration with equivalent operations.

## Conventions

- Read an issue and its discussion before acting on it.
- Publish specs and tracer-bullet implementation work as separate issues.
- Apply `ready-for-agent` to fully specified execution tickets.
- Use GitHub native issue dependencies when available. Otherwise keep a durable `## Blocked by` section in the issue body.
- A ticket is on the implementation frontier only when every issue named under `Blocked by` is closed.
- The ordered Authoring delivery spec is #175; the repository roadmap records its current frontier.
- Pull requests are not a triage request surface by default. Explicitly named PRs may still be reviewed or continued.

## Common `gh` operations

- Create: `gh issue create --title "..." --body "..."`
- Read: `gh issue view <number> --comments`
- List: `gh issue list --state open --json number,title,body,labels,assignees`
- Comment: `gh issue comment <number> --body "..."`
- Label: `gh issue edit <number> --add-label "..."`
- Close: `gh issue close <number> --comment "..."`

GitHub shares one number space across issues and PRs. Resolve an ambiguous bare `#42` before mutating it.

## Blocking relationships

Prefer GitHub's native issue dependencies when the current client exposes them. With `gh api`, add a `blocked_by` edge using the blocker's numeric database `id`, not its issue number or node id. If native dependencies are unavailable, the `## Blocked by` section is authoritative.

Do not invent blockers merely to make a linear plan. Parallel work is allowed when dependencies are genuinely independent.

## Repository governance

GitHub issues are the execution surface. Cairn todos under `meta/todos/` retain milestone status and acceptance criteria; accepted architecture decisions under `meta/decisions/` remain authoritative. `ROADMAP.md` reconciles those durable records with the current issue frontier.
