import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


class SchemaWorkflowContract(unittest.TestCase):
    def test_capture_is_read_only_opt_in_and_retains_only_evidence(self):
        workflow = (ROOT / '.github/workflows/schema-capabilities.yml').read_text()
        condition = workflow.split('  capture:\n    if: >-\n', 1)[1].split('    runs-on:', 1)[0]
        for guard in ["github.event.action == 'edited' &&", 'github.event.changes.body != null &&',
                      'github.event.pull_request.head.repo.full_name == github.repository &&',
                      "github.event_name == 'workflow_dispatch' ||"]:
            self.assertIn(guard, condition)
        self.assertIn('!contains(github.event.changes.body.from', condition)
        self.assertIn('contents: read', workflow)
        self.assertNotIn('contents: write', workflow)
        self.assertNotIn('git push', workflow)
        self.assertNotIn('git archive', workflow)
        self.assertIn('path: ${{ env.SCHEMA_OUTPUT }}', workflow)
        self.assertIn('persist-credentials: false', workflow)

    def test_normal_check_uses_retained_evidence_not_official_cli(self):
        workflow = (ROOT / '.github/workflows/schema-capabilities.yml').read_text()
        normal = workflow.split('  capture:', 1)[0]
        self.assertIn('capabilities.py check --json', normal)
        for external in ['curl ', 'schema_capture.py', 'sudo ', 'run-schema-reference']:
            self.assertNotIn(external, normal)
        self.assertFalse((ROOT / '.github/workflows/schema-retain.yml').exists())


if __name__ == '__main__':
    unittest.main()
