import re
import subprocess
import tempfile
import unittest
from pathlib import Path

import reference


class CaptureWorkflow(unittest.TestCase):
    def setUp(self):
        self.workflow = (reference.ROOT / ".github/workflows/cli-reference.yml").read_text()

    def test_capture_output_is_run_specific_and_outside_rust_cache(self):
        assignment = re.search(r'^          echo "REFERENCE_OUTPUT=.+$', self.workflow, re.MULTILINE)
        self.assertIsNotNone(assignment, "capture output must be selected on the runner, outside Rust's cache")
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for run_id, attempt in [("100", "1"), ("100", "2"), ("101", "1")]:
                environment_file = root / f"env-{run_id}-{attempt}"
                env = {"RUNNER_TEMP": str(root), "GITHUB_RUN_ID": run_id,
                       "GITHUB_RUN_ATTEMPT": attempt, "GITHUB_ENV": str(environment_file)}
                subprocess.run(["/bin/sh", "-eu", "-c", assignment.group(0).strip()], env=env, check=True)
                self.assertEqual(environment_file.read_text(),
                                 f"REFERENCE_OUTPUT={root}/official-reference-{run_id}-{attempt}\n")
                self.assertFalse((root / f"official-reference-{run_id}-{attempt}").exists())
        self.assertIn('--output "$REFERENCE_OUTPUT"', self.workflow)
        self.assertIn('path: ${{ env.REFERENCE_OUTPUT }}', self.workflow)
        self.assertNotIn('target/official-reference', self.workflow)

    def test_runner_context_is_not_used_before_job_steps(self):
        header = self.workflow.split('  capture:\n', 1)[1].split('    steps:\n', 1)[0]
        self.assertNotIn('runner.', header)
