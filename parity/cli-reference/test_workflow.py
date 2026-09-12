import re
import unittest

import reference


class CaptureWorkflow(unittest.TestCase):
    def test_capture_output_is_run_specific_and_outside_rust_cache(self):
        workflow = (reference.ROOT / ".github/workflows/cli-reference.yml").read_text()
        output = re.search(r"^      REFERENCE_OUTPUT: (.+)$", workflow, re.MULTILINE)
        self.assertIsNotNone(output, "capture requires an output outside the cached target directory")
        self.assertEqual(output.group(1), "${{ runner.temp }}/official-reference-${{ github.run_id }}-${{ github.run_attempt }}")
        self.assertIn('--output "$REFERENCE_OUTPUT"', workflow)
        self.assertIn('path: ${{ env.REFERENCE_OUTPUT }}', workflow)
        self.assertNotIn('target/official-reference', workflow)
