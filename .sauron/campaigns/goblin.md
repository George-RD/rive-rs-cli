# Goblin Campaign — Trivial Fix

> For ≤5 lines / 1 file changes.

| Field | Value |
|---|---|
| Name | `goblin` |
| Phases | 3 |
| Meta-Review | Skipped |

## Phase Sequence

| Order | Phase ID | Force | Required Events |
|---|---|---|---|
| 1 | `implement` | nazgul | phase_started, phase_completed |
| 2 | `siege` | siege-commander | phase_started, phase_completed, siege_tick |
| 3 | `conquered` | — | phase_started, phase_completed, campaign_completed |
