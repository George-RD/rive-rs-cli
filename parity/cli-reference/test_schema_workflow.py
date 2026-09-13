import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


class SchemaWorkflowContract(unittest.TestCase):
    def test_capture_is_read_only_opt_in_and_retains_only_evidence(self):
        workflow = (ROOT / '.github/workflows/schema-capabilities.yml').read_text()
        condition = workflow.split('  capture:\n    if: >-\n', 1)[1].split('    runs-on:', 1)[0]
        self.assertEqual(
            ' '.join(condition.split()),
            "github.event_name == 'workflow_dispatch' || "
            "(github.event.action == 'edited' && "
            "github.event.changes.body != null && "
            "github.event.pull_request.head.repo.full_name == github.repository && "
            "contains(github.event.pull_request.body, '<!-- run-schema-reference:1.0.2 -->') && "
            "!contains(github.event.changes.body.from, '<!-- run-schema-reference:1.0.2 -->'))",
        )
        self.assertIn('contents: read', workflow)
        self.assertNotIn('contents: write', workflow)
        self.assertNotIn('git push', workflow)
        self.assertNotIn('git archive', workflow)
        self.assertIn('path: ${{ env.SCHEMA_OUTPUT }}', workflow)
        self.assertIn('persist-credentials: false', workflow)

    def test_retention_gate_uses_the_base_commit_and_explicit_initial_pin(self):
        workflow = (ROOT / '.github/workflows/schema-capabilities.yml').read_text()
        normal = workflow.split('  capture:', 1)[0]
        self.assertIn("ref: ${{ github.event.pull_request.base.sha || github.sha }}", normal)
        self.assertIn('path: .schema-base', normal)
        self.assertIn('python3 parity/cli-reference/schema_retention.py', normal)
        self.assertIn('--base-root .schema-base', normal)
        self.assertIn('--reviewed-change absent:6a879379e557eb45ecbdc4da958aa4032f7b4ca3ebef884ca3426dd79eb8b61f', normal)

    def test_normal_check_uses_retained_evidence_not_official_cli(self):
        workflow = (ROOT / '.github/workflows/schema-capabilities.yml').read_text()
        normal = workflow.split('  capture:', 1)[0]
        self.assertIn('capabilities.py check --json', normal)
        for external in ['curl ', 'schema_capture.py', 'sudo ', 'run-schema-reference']:
            self.assertNotIn(external, normal)
        self.assertFalse((ROOT / '.github/workflows/schema-retain.yml').exists())


if __name__ == '__main__':
    unittest.main()
