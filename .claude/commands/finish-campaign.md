# /finish-campaign

Close out a campaign and sync state.

## Usage

```
/finish-campaign --campaign <slug>
```

## Behavior

1. Emit `campaign_completed` to war-log.
2. Run `sync-issues.mjs` to close linked issue and comment PR link.
3. Archive strongholds to `docs/history/`.
