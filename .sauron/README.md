# Sauron Campaign Engine (rive-rs-cli)

Vendored from mordor-forge/ide-of-sauron, trimmed and adapted for git/rive workflows.

## Invariants

- GitHub Issues = source of truth for "what" (backlog).
- War-log JSONL = source of truth for "execution progress" (append-only).
- Every implementer has a downstream Watcher (Law 8).
- Gate advances only on `task_verified`, never `task_completed` alone.

## Layout

```
.sauron/
  scripts/          State management and validation
  campaigns/        Template definitions (full, standard, hotfix, goblin)
  forces/           Force briefing templates for task dispatch
  state/campaigns/  Runtime war-logs and materialized state
  README.md         This file
```

## Commands

- `/orchestration <issue#>` — instantiate campaign
- `/green` — local quality gate loop
- `/review` — adversarial review gate
- `/whats-next` — priority routing
- `/siege-tick` — PR evaluation loop
- `/finish-campaign` — close and sync

## Scripts

```bash
node .sauron/scripts/mordor-state.mjs log '{...}'
node .sauron/scripts/materialize-state.mjs --campaign <slug> --dry-run
node .sauron/scripts/sauron-status.mjs <campaign> dashboard
node .sauron/scripts/dag-validate.mjs <campaign.json> [--dry-run]
node .sauron/scripts/sync-issues.mjs --campaign <slug>
```

## Upstream Provenance

- Origin: `~/.claude/plugins/marketplaces/mordor-forge/ide-of-sauron/`
- Differences: per-campaign war-logs, git worktrees (no JJ), no GUI/eye, rive-adapted forces.
