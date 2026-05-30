# Hotfix Campaign — Urgent Bug Fix

> For urgent bugs with no design phase.

| Field | Value |
|---|---|
| Name | `hotfix` |
| Phases | 5 |
| Meta-Review | Skipped |

## Phase Sequence

| Order | Phase ID | Force | Required Events |
|---|---|---|---|
| 1 | `implement` | nazgul | phase_started, phase_completed |
| 2 | `review` | watcher | phase_started, phase_completed, watcher_verdict_recorded |
| 3 | `fix` | nazgul | phase_started, phase_completed |
| 4 | `siege` | siege-commander | phase_started, phase_completed, siege_tick |
| 5 | `conquered` | — | phase_started, phase_completed, campaign_completed |
