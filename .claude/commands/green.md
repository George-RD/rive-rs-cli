# /green

Build-until-CI-green loop. Run the local quality gate repeatedly until passing.

## Usage

```
/green [--campaign <slug>]
```

## Gate

```bash
cargo build
cargo clippy -- -D warnings
cargo fmt --check
cargo test
cargo test --test e2e
```

## Behavior

- Run all commands in sequence.
- If any fail, dispatch fix nazgûl with failure log as Retry Context.
- Re-run gate. Bounded to ≤5 attempts; else emit `task_blocked`.
- On pass, emit `quality_gate_executed` with outcome `pass`.
