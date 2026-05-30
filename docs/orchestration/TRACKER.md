# Tracker Conventions

## GitHub Issues = Source of Truth for "What"

- Each work item = one issue.
- Labels: `area:{encoder,builder,objects,cli,validator,ai,qa}`, `priority:{p0..p3}`, `bug`, `enhancement`.
- Epics group related issues.
- Campaigns reference their issue number in the slug (e.g., `camp-121`).

## War-Log = Source of Truth for "Execution Progress"

- Path: `.sauron/state/campaigns/<campaign>/war-log.jsonl`
- Append-only JSONL.
- Schema-validated by `lib/war-state-schema.mjs`.

## Canonical Event Types

| Event | When |
|---|---|
| `campaign_started` | Campaign instantiation |
| `phase_started` | Phase begins |
| `phase_completed` | Phase ends |
| `phase_skipped` | Phase skipped (flag) |
| `task_dispatched` | Force assigned |
| `task_completed` | Force finished |
| `task_verified` | Watcher verified (Law 8) |
| `task_failed` | Force failed |
| `watcher_verdict_recorded` | Review PASS/FAIL |
| `quality_gate_executed` | /green outcome |
| `siege_tick` | PR evaluation round |
| `campaign_completed` | Merged, done |
| `campaign_abandoned` | Cancelled |

## Strongholds

- Path: `docs/strongholds/<campaign>-*.md`
- Per-run findings: recon reports, review verdicts, design docs.
- The orchestrator coordinates by path, never by pasting bulk data.
