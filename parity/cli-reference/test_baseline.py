import tarfile
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import reference


class RetainedBaseline(unittest.TestCase):
    def test_pinned_capture_is_complete_without_executing_tools(self):
        baseline = reference.HERE / "baseline"
        pin = reference.read_json(baseline / "capture.json")
        archive = reference.checked_relative(baseline, pin["archive"])
        self.assertEqual(reference.digest(archive), pin["archive_sha256"])
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with tarfile.open(archive, "r:xz") as package:
                self.assertLess(sum(item.size for item in package.getmembers()), 64 * 1024 * 1024)
                for item in package.getmembers():
                    name = Path(item.name)
                    self.assertTrue(item.isfile())
                    self.assertFalse(name.is_absolute())
                    self.assertNotIn("..", name.parts)
                    target = root / name
                    target.parent.mkdir(parents=True, exist_ok=True)
                    with package.extractfile(item) as stream:
                        target.write_bytes(stream.read())
            with patch("subprocess.run", side_effect=AssertionError("offline verification executed a tool")):
                recording = reference.verify_recording(root, pin["source_head"])
            self.assertEqual(recording["status"], "passed")
