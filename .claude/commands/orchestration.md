# /orchestration

Campaign DAG dispatcher. Instantiate and run a campaign for a given issue.

## Usage

```
/orchestration <issue#> [--template full|standard|hotfix|goblin] [--dry-run]
```

## Steps

1. Read the issue from GitHub.
2. Select template (default: infer from scope).
3. Run `dag-validate` on the campaign JSON.
4. If `--dry-run`: print wave plan, exit.
5. Write `campaign_started` to war-log.
6. Dispatch forces wave by wave.
7. Gate advances only on `task_verified` (Law 8).

## Templates

| Template | When |
|---|---|
| `full` | New feature, large refactor, multi-epic |
| `standard` | Spec exists, medium scope |
| `hotfix` | Urgent bug, no design needed |
| `goblin` | ≤5 lines, 1 file |

## Examples

```
/orchestration #121 --template hotfix
/orchestration #72 --template standard
/orchestration #37 --dry-run
```
