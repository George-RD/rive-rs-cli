# AI Prompt Cookbook v1

This cookbook defines known-good deterministic templates and expected output traits for prompt-lab regression runs.

The prompt contract itself is `docs/ai/scene-prompt-schema.json`, the curated schema subset embedded in the `ai generate` system prompt. It is a deliberate subset of the full `docs/scene.schema.v1.json`; `ai generate` may only emit the object types it declares.

## Run Harness

Baselines are not committed. Produce one first with `--write-baseline`, pointing at any path you choose:

```bash
cargo run -- ai lab \
  --suite evals/suites/prompt_lab.v1.json \
  --output-dir evals/runs \
  --write-baseline /tmp/prompt_lab.v1.baseline.json
```

Then check later runs against it with `--baseline`:

```bash
cargo run -- ai lab \
  --suite evals/suites/prompt_lab.v1.json \
  --output-dir evals/runs \
  --baseline /tmp/prompt_lab.v1.baseline.json
```

## Templates and Expected Traits

1. `bounce` -> `has_animation`
2. `spinner` -> `has_animation`
3. `pulse` -> `has_animation`
4. `fade` -> `has_animation`
5. `state_machine` -> `has_state_machine`
6. `text` -> `has_text`
7. `layout` -> `has_layout`
8. `data_binding` -> `has_data_binding`
9. `bones` -> `has_bones`
10. `constraints` -> `has_constraints`

## Artifact Layout

- `evals/runs/<run_id>/suite.json`
- `evals/runs/<run_id>/report.json`
- `evals/runs/<run_id>/samples/<case_id>/input.txt`
- `evals/runs/<run_id>/samples/<case_id>/scene.json`
- `evals/runs/<run_id>/samples/<case_id>/output.riv`
- `evals/runs/<run_id>/samples/<case_id>/validate.json`
- `evals/runs/<run_id>/samples/<case_id>/inspect.json`

## Multimodal Hooks

Suite cases support optional fields:

- `text_hint`
- `image_path`

These fields are persisted into run reports for future image/text-informed prompt variants.
