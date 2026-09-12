import copy
import hashlib
import io
import json
import os
import subprocess
import sys
import tarfile
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import reference


class InstallationContracts(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory()
        self.addCleanup(self.scratch.cleanup)
        self.root = Path(self.scratch.name)
        self.binary = self.root / "rive"
        self.binary.write_bytes(b"unit-test executable, not the official CLI")
        self.binary.chmod(0o700)
        self.archive = self.root / "pinned.tar.gz"
        with tarfile.open(self.archive, "w:gz") as package:
            data = self.binary.read_bytes()
            member = tarfile.TarInfo("release/rive")
            member.size = len(data)
            package.addfile(member, io.BytesIO(data))
        self.lock = {"platform": ["Linux", "x86_64"], "source": "unit-test-only",
                     "archive_sha256": hashlib.sha256(self.archive.read_bytes()).hexdigest()}

    def test_missing_official_binary_fails_before_any_execution(self):
        with self.assertRaisesRegex(reference.ReferenceError, "official binary is missing"):
            reference.verify_installation(self.root / "missing", self.archive, self.lock)

    def test_matching_archive_and_binary_are_measured_without_execution(self):
        result = reference.verify_installation(self.binary, self.archive, self.lock)
        self.assertEqual(result["archive_member"], "release/rive")
        self.assertEqual(result["binary_sha256"], hashlib.sha256(self.binary.read_bytes()).hexdigest())

    def test_archive_mutation_fails(self):
        self.archive.write_bytes(self.archive.read_bytes() + b"tampering")
        with self.assertRaisesRegex(reference.ReferenceError, "archive digest mismatch"):
            reference.verify_installation(self.binary, self.archive, self.lock)

    def test_replaced_binary_fails_even_when_archive_matches(self):
        self.binary.write_bytes(b"different executable")
        with self.assertRaisesRegex(reference.ReferenceError, "binary does not match"):
            reference.verify_installation(self.binary, self.archive, self.lock)

    def test_missing_archive_is_not_downloaded(self):
        self.archive.unlink()
        with self.assertRaisesRegex(reference.ReferenceError, "never download"):
            reference.verify_installation(self.binary, self.archive, self.lock)

    def test_non_executable_binary_fails(self):
        self.binary.chmod(0o600)
        with self.assertRaisesRegex(reference.ReferenceError, "not executable"):
            reference.verify_installation(self.binary, self.archive, self.lock)

    def test_unsupported_platform_fails(self):
        with patch("platform.machine", return_value="aarch64"):
            with self.assertRaisesRegex(reference.ReferenceError, "unsupported platform"):
                reference.verify_installation(self.binary, self.archive, self.lock)

    def test_archive_symlink_is_not_accepted_as_binary(self):
        with tarfile.open(self.archive, "w:gz") as package:
            member = tarfile.TarInfo("rive")
            member.type = tarfile.SYMTYPE
            member.linkname = "/somewhere/else"
            package.addfile(member)
        self.lock["archive_sha256"] = hashlib.sha256(self.archive.read_bytes()).hexdigest()
        with self.assertRaisesRegex(reference.ReferenceError, "regular rive executable"):
            reference.verify_installation(self.binary, self.archive, self.lock)

    def test_wrong_version_and_prerelease_are_rejected(self):
        for output in ["rive 1.0.20", "rive 1.0.1", "rive 1.0.2-dev", "rive 1.0.2+local", "no version", "1.0.2 1.0.3"]:
            with self.subTest(output=output):
                with self.assertRaisesRegex(reference.ReferenceError, "wrong official CLI version"):
                    reference.verify_version(output, "1.0.2")

    def test_exact_version_is_accepted(self):
        reference.verify_version("Rive CLI 1.0.2\n", "1.0.2")

    def test_official_environment_does_not_inherit_credentials_or_proxies(self):
        with patch.dict(os.environ, {"RIVE_TOKEN": "sentinel", "HTTPS_PROXY": "sentinel", "XDG_CONFIG_HOME": "sentinel"}):
            environment = reference.isolated_environment(self.root)
        self.assertNotIn("RIVE_TOKEN", environment)
        self.assertNotIn("HTTPS_PROXY", environment)
        self.assertNotEqual(environment["XDG_CONFIG_HOME"], "sentinel")
        self.assertEqual(list(Path(environment["XDG_CONFIG_HOME"]).iterdir()), [])


class EvidenceContracts(unittest.TestCase):
    def setUp(self):
        self.control = {"ok": True, "reference_object_count": 5, "candidate_object_count": 5,
                        "type_deltas": [{"type_name": "Shape", "reference": 1, "candidate": 1, "delta": 0}],
                        "frames": [{"index": index, "pixel_difference": 0} for index in [0, 15, 30]],
                        "max_pixel_difference": 0}

    def test_identical_comparison_is_a_control_not_a_negative_pass(self):
        reference.check_comparison(self.control, False)
        with self.assertRaisesRegex(reference.ReferenceError, "no detected visible difference"):
            reference.check_comparison(self.control, True)

    def test_visible_negative_control_is_detected(self):
        negative = copy.deepcopy(self.control)
        negative["frames"][1]["pixel_difference"] = 8.5
        negative["max_pixel_difference"] = 8.5
        reference.check_comparison(negative, True)
        with self.assertRaisesRegex(reference.ReferenceError, "identical reference control"):
            reference.check_comparison(negative, False)

    def test_comparison_failure_is_not_a_detected_difference(self):
        self.control["ok"] = False
        with self.assertRaisesRegex(reference.ReferenceError, "did not complete"):
            reference.check_comparison(self.control, True)

    def test_missing_frames_cannot_pass(self):
        self.control["frames"].pop()
        with self.assertRaisesRegex(reference.ReferenceError, "incomplete frame coverage"):
            reference.check_comparison(self.control, False)

    def test_nonfinite_metrics_cannot_pass(self):
        for value in [float("nan"), float("inf"), -1, 101, True, None]:
            with self.subTest(value=value):
                report = copy.deepcopy(self.control)
                report["max_pixel_difference"] = value
                with self.assertRaisesRegex(reference.ReferenceError, "invalid pixel measurements"):
                    reference.check_comparison(report, True)

    def test_inconsistent_summary_cannot_pass(self):
        self.control["max_pixel_difference"] = 50
        with self.assertRaisesRegex(reference.ReferenceError, "maximum disagrees"):
            reference.check_comparison(self.control, True)

    def test_structural_control_difference_cannot_pass(self):
        self.control["candidate_object_count"] = 6
        with self.assertRaisesRegex(reference.ReferenceError, "structural difference"):
            reference.check_comparison(self.control, False)

    def test_structural_measurements_are_required(self):
        del self.control["type_deltas"]
        with self.assertRaisesRegex(reference.ReferenceError, "no structural measurements"):
            reference.check_comparison(self.control, False)

    def test_incomplete_recording_is_not_offline_runtime_proof(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "recording.json").write_text(json.dumps({"schema_version": 1, "status": "failed"}))
            with self.assertRaisesRegex(reference.ReferenceError, "incomplete"):
                reference.verify_recording(root, "1" * 40)

    def test_empty_pass_claim_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "recording.json").write_text(json.dumps({"schema_version": 1, "status": "passed"}))
            with self.assertRaises(reference.ReferenceError):
                reference.verify_recording(root, "1" * 40)

    def test_evidence_cannot_escape_recording_directory(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for path in ["../outside", "/etc/passwd"]:
                with self.assertRaisesRegex(reference.ReferenceError, "contained"):
                    reference.checked_relative(root, path)
            (root / "link").symlink_to("/etc/passwd")
            with self.assertRaisesRegex(reference.ReferenceError, "escaping"):
                reference.checked_relative(root, "link")

    def test_runtime_rejection_is_not_a_blank_frame_pass(self):
        report = {"ok": True, "width": 128, "height": 96, "scale": 1, "fps": 60,
                  "frames": [{"index": index, "blank": True} for index in [0, 15, 30]]}
        with self.assertRaisesRegex(reference.ReferenceError, "blank or unsupported"):
            reference.check_render(report, Path("."), False)

    def test_runtime_must_select_the_requested_animation(self):
        report = {"ok": True, "width": 128, "height": 96, "scale": 1, "fps": 60,
                  "animation": None, "frames": [{"index": index} for index in [0, 15, 30]]}
        with self.assertRaisesRegex(reference.ReferenceError, "requested animation"):
            reference.check_render(report, Path("."), True)

    def test_zero_exit_official_build_errors_cannot_pass(self):
        for report in [{"success": False, "command": "build", "errors": []},
                       {"success": True, "command": "build", "errors": ["bad"]},
                       {"success": True, "command": "build", "errors": [], "data": {"problems": [{"severity": "error"}]}},
                       {"success": True, "command": "verify", "errors": [], "data": {}}]:
            with self.subTest(report=report):
                with self.assertRaises(reference.ReferenceError):
                    reference.check_official_build(report, "build")

    def test_command_launch_failure_is_retained(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            recorder = reference.Recorder(root)
            with self.assertRaises(reference.ReferenceError):
                recorder.run("missing", [str(root / "missing")], root)
            self.assertEqual(len(recorder.commands), 1)
            self.assertIsNone(recorder.commands[0]["exit_code"])
            self.assertIn("execution_error", recorder.commands[0])

    def test_capture_requires_an_explicit_binary(self):
        result = subprocess.run([sys.executable, str(reference.HERE / "reference.py"), "capture"], capture_output=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"--official-cli", result.stderr)


if __name__ == "__main__":
    unittest.main()
