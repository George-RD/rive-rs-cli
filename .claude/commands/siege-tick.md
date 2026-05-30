# /siege-tick

Evaluate PR state and dispatch Siege-Commander if needed.

## Usage

```
/siege-tick --campaign <slug> --pr <number>
```

## Behavior

1. Check PR CI status and review state.
2. If `CHANGES_REQUESTED` or red CI → dispatch Siege-Commander.
3. If approved and green → emit `campaign_completed`.
4. Writes `siege_tick` event each evaluation.
