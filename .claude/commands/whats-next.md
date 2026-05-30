# /whats-next

Priority routing over open issues + war-log.

## Usage

```
/whats-next
```

## Behavior

1. Query open GitHub Issues with `priority:p0`, `priority:p1`, `priority:p2`.
2. Exclude issues labeled `blocked` or `in-progress`.
3. Return highest-priority unassigned issue with recommended template.

## Output

```
Next: #121 (priority:p2, area:objects/builder)
Template: hotfix
Rationale: checked_sub index conversions — small, scoped, high correctness value
```
