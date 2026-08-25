# Domain docs

This is a single-context repo. Engineering skills should use one domain glossary while respecting Cairn as the existing architecture and status authority.

## Read before exploring

1. `CONTEXT.md` for canonical domain vocabulary.
2. `AGENTS.md` and the nearest scoped `AGENTS.md` for coding and repository rules.
3. Relevant accepted decisions under `meta/decisions/` before proposing architecture changes.
4. Relevant contracts under `meta/contracts/` before changing externally meaningful behavior.
5. Relevant todos under `meta/todos/` and `ROADMAP.md` before selecting or sequencing work.

If one of these is absent, proceed with the remaining sources.

## Vocabulary

Use terms as defined in `CONTEXT.md` in issue titles, specs, tests, architecture reviews, and code-facing explanations. If a new project-specific concept needs a name, use the `domain-modeling` skill to sharpen it and update the glossary.

`CONTEXT.md` is a glossary only. Do not put implementation decisions, roadmap status, or ticket plans in it.

## Durable decisions

This repo already uses Cairn decisions under `meta/decisions/`. Treat those as the equivalent of architecture decision records for this project.

Do **not** create a parallel `docs/adr/` authority unless the maintainer explicitly changes repository governance. When the generic `domain-modeling` skill would normally offer an ADR, record the accepted decision in the existing Cairn decision format instead.

If a proposal contradicts an accepted Cairn decision, surface the conflict explicitly and use its revisit triggers rather than silently overriding it.

## Status and execution

Cairn todos hold durable milestone status and acceptance criteria. GitHub issues are the execution surface. `ROADMAP.md` identifies the current implementation frontier and reconciles ticket blockers with Cairn milestones.
