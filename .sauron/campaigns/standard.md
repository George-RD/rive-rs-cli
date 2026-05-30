# Standard Campaign — 8 Phase March

> For work where a spec already exists.

| Field | Value |
|---|---|
| Name | `standard` |
| Phases | 8 |
| Meta-Review | Optional |

## Phase Sequence

| Order | Phase ID | Force | Required Events |
|---|---|---|---|
| 1 | `recon` | uruk-hai-scout | phase_started, phase_completed |
| 2 | `implement` | nazgul | phase_started, phase_completed |
| 3 | `review` | watcher | phase_started, phase_completed, watcher_verdict_recorded |
| 4 | `fix` | nazgul | phase_started, phase_completed |
| 5 | `ready` | — | phase_started, phase_completed |
| 6 | `forging` | siege-commander | phase_started, phase_completed |
| 7 | `siege` | siege-commander | phase_started, phase_completed, siege_tick |
| 8 | `conquered` | — | phase_started, phase_completed, campaign_completed |

## Flags

| Flag | Effect |
|---|---|
| `--no-meta-review` | Skips meta-review Watcher |
