# Cyclical Engineering Workflow DAG

## Outer Loop

```
/whats-next → select issue → /orchestration <issue#> → run campaign → merge → war-log campaign_completed → close issue → /whats-next
```

## Campaign DAG (full template)

```
recon → design → baseline → implement → review → fix → cleanse → ready → forging → siege → conquered
```

### Gates

- Advances **only on `task_verified`** (Law 8).
- Never on `task_completed` alone.

### Nested Loops

1. **/green** (inside `implement`):
   ```
   cargo build → clippy → fmt --check → test → e2e → playwright
   ```
   Not green → fix nazgûl → re-run (≤5).

2. **/review** (`review` + `fix`):
   ```
   4a parallel review → 4b fix → 4c debate → 4d re-review
   ```
   Loop until PASS or 3-strike escalation.

3. **/siege** (`siege`):
   ```
   PR open → green CI + reviewers → fixes → push → re-review
   ```
   Loop until merged.

## Template Heuristics

| Template | Phases | Meta-Review |
|---|---|---|
| `full` | 11 | Mandatory |
| `standard` | 8 | Optional |
| `hotfix` | 5 | Skipped |
| `goblin` | 3 | Skipped |

## Law 8: No Victory Unwatched

Every implementer phase MUST have a downstream Watcher/review phase. `dag-validate` enforces this.
