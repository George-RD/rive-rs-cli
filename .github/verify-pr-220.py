from __future__ import annotations

import json
import subprocess
from pathlib import Path

TEMPORARY_PATHS = [
    Path(".github/apply-authoring-stacking.py"),
    Path(".github/run-authoring-stacking.py"),
    Path(".github/finalize-authoring-stacking.py"),
    Path(".github/workflows/apply-authoring-stacking.yml"),
    Path(".github/workflows/finalize-authoring-stacking.yml"),
    Path(".github/workflows/finalize-authoring-stacking-v2.yml"),
    Path(".github/workflows/source-bundle.yml"),
]

EXPECTED_CHANGED_PATHS = {
    ".github/workflows/ci.yml",
    "CHANGELOG.md",
    "ROADMAP.md",
    "docs/authoring-spec-v0.md",
    "docs/authoring.schema.v0.json",
    "examples/authoring/README.md",
    "examples/authoring/stacking-card.v0.json",
    "meta/todos/todo.visual-authoring-compiler.md",
    "skills/claude-code/commands/rive-generate.md",
    "src/ai/openai.rs",
    "src/authoring/frontend.rs",
    "src/authoring/lower.rs",
    "src/authoring/lower/node.rs",
    "src/authoring/mod.rs",
    "src/authoring/spec.rs",
    "src/authoring/visual.rs",
    "tests/authoring_stacking_contract.rs",
    "tests/authoring_stacking_runtime.rs",
}

RUNTIME_STEP = '''      - name: Run AuthoringSpec stacking runtime contract
        run: |
          RIVE_CHROME="$(node --input-type=module -e 'import { chromium } from "playwright"; process.stdout.write(chromium.executablePath())')" \\
            cargo test --locked --test authoring_stacking_runtime -- --ignored
'''


def run(*args: str) -> str:
    return subprocess.check_output(args, text=True).strip()


def normalize_branch() -> None:
    for path in TEMPORARY_PATHS:
        if path.exists():
            path.unlink()

    fixture_path = Path("examples/authoring/stacking-card.v0.json")
    fixture = json.loads(fixture_path.read_text())
    nodes = fixture["visual"]["nodes"]
    for node in nodes:
        node.setdefault("transform", {})
        node["transform"]["x"] = {"kind": "literal", "value": 64.0, "unit": "px"}
        node["transform"]["y"] = {"kind": "literal", "value": 64.0, "unit": "px"}
    fixture_path.write_text(json.dumps(fixture, indent=2) + "\n")

    ci_path = Path(".github/workflows/ci.yml")
    ci = ci_path.read_text()
    if "Run AuthoringSpec stacking runtime contract" not in ci:
        seam = "      - run: npx playwright install --with-deps chromium\n      - name: Run typed behavior runtime contract\n"
        if seam not in ci:
            raise SystemExit("permanent Playwright runtime insertion seam not found")
        ci = ci.replace(
            seam,
            "      - run: npx playwright install --with-deps chromium\n"
            + RUNTIME_STEP
            + "      - name: Run typed behavior runtime contract\n",
            1,
        )
        ci_path.write_text(ci)


def assert_changed_paths() -> None:
    subprocess.run(["git", "fetch", "origin", "main"], check=True)
    changed = set(
        filter(
            None,
            run("git", "diff", "--name-only", "origin/main...HEAD").splitlines(),
        )
    )
    missing = EXPECTED_CHANGED_PATHS - changed
    unexpected = changed - EXPECTED_CHANGED_PATHS
    if missing or unexpected:
        raise SystemExit(
            "unexpected PR file set\n"
            f"missing: {sorted(missing)}\n"
            f"unexpected: {sorted(unexpected)}"
        )


if __name__ == "__main__":
    normalize_branch()
