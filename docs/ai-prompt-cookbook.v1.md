# AI Prompt Cookbook v1

This cookbook defines known-good deterministic templates and expected output traits for prompt-lab regression runs.

## Run Harness

```bash
cargo run -- ai lab \
  --suite evals/suites/prompt_lab.v1.json \
  --output-dir evals/runs \
  --write-baseline evals/baselines/prompt_lab.v1.json
```

Regression check:

```bash
cargo run -- ai lab \
  --suite evals/suites/prompt_lab.v1.json \
  --output-dir evals/runs \
  --baseline evals/baselines/prompt_lab.v1.json
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
