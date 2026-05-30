# /review

Adversarial review gate (forge-pr Phase 4).

## Usage

```
/review --campaign <slug> [--phase <id>]
```

## Phase 4 Pipeline

1. **4a** — Parallel dispatch:
   - `reforger` (hygiene)
   - `inquisitor` + `shadow-hunter` (correctness, silent failures)
   - `test-warden` (coverage)
2. **4b** — If any FAIL, dispatch fix nazgûl with Retry Context.
3. **4c** — `/debate` on contested findings.
4. **4d** — Re-review. Loop until PASS or 3-strike escalation.

## Output

Writes `watcher_verdict_recorded` events to war-log.
