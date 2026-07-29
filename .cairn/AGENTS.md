# Working with Cairn in this repository

`cairn.blueprint` is the architecture map. `ROADMAP.md` is the ordered human view;
Cairn todos under `meta/todos/` are the task records and status source of truth.

## Orient before broad file reads

```bash
cairn context
```

Use the returned node and artefact slice before loading unrelated source files.
The repository instructions in the root `AGENTS.md` still take precedence.

## Before changing architecture

1. Read the relevant contract and accepted decisions returned by `cairn context`.
2. Update or add a todo when the work changes roadmap scope or ordering.
3. Add research and a decision before replacing a canonical representation.
4. Keep `SceneSpec` as the lowered IR unless `dec.ai-authoring-layer` is revisited.

## Gates

```bash
cairn scan
cairn lint
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked --all-features
```

New source and test files must be owned by a blueprint node. The planned
`rive-cli.intelligence.authoring` node intentionally has no source path until its
first implementation lands.
