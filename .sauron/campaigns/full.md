# Full Campaign — 11 Phase March

> For new features, significant refactors, or multi-epic work.

| Field | Value |
|---|---|
| Name | `full` |
| Phases | 11 |
| Meta-Review | Mandatory |

## Phase Sequence

| Order | Phase ID | Force | Required Events |
|---|---|---|---|
| 1 | `recon` | uruk-hai-scout | phase_started, phase_completed |
| 2 | `design` | architect | phase_started, phase_completed |
| 3 | `baseline` | uruk-hai-scout | phase_started, phase_completed, baseline_verified OR baseline_recorded OR baseline_skipped |
| 4 | `implement` | nazgul | phase_started, phase_completed |
| 5 | `review` | watcher | phase_started, phase_completed, watcher_verdict_recorded |
| 6 | `fix` | nazgul | phase_started, phase_completed |
| 7 | `cleanse` | cleansing-watcher | phase_started, phase_completed |
| 8 | `ready` | — | phase_started, phase_completed |
| 9 | `forging` | siege-commander | phase_started, phase_completed |
| 10 | `siege` | siege-commander | phase_started, phase_completed, siege_tick |
| 11 | `conquered` | — | phase_started, phase_completed, campaign_completed |

## Flags

| Flag | Effect |
|---|---|
| `--no-cleanse` | Skips cleanse phase |
| `--no-meta-review` | Not permitted for full campaigns |

## War-Log Event Summary

- One `campaign_started`
- One `phase_started` + `phase_completed` per phase (or `phase_skipped`)
- One `baseline_verified` or `baseline_recorded` or `baseline_skipped`
- One `campaign_completed`
