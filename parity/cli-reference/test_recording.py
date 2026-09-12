import hashlib
import json
import struct
import tempfile
import unittest
import zlib
from pathlib import Path

import reference


class RetainedRecordingContracts(unittest.TestCase):
    def setUp(self):
        scratch = tempfile.TemporaryDirectory()
        self.addCleanup(scratch.cleanup)
        self.root = Path(scratch.name)
        self.lock = reference.read_json(reference.HERE / "lock.json")
        self.recording = self.synthetic_recording()
        self.save()

    def put(self, path, data):
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
        return path

    def command(self, name, argv, output, code=0, retain=True):
        stdout = output.encode()
        command = {"name": name, "argv": argv, "cwd": "/synthetic/source", "exit_code": code,
                   "stdout_sha256": hashlib.sha256(stdout).hexdigest(),
                   "stderr_sha256": hashlib.sha256(b"").hexdigest()}
        if retain:
            command["stdout_file"] = self.put(f"commands/{name}.stdout", stdout)
            command["stderr_file"] = self.put(f"commands/{name}.stderr", b"")
        return command

    def png(self, value):
        def chunk(kind, data):
            return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data))
        pixels = (b"\x00" + bytes([value, 80, 120, 255]) * 128) * 96
        return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", 128, 96, 8, 6, 0, 0, 0))
                + chunk(b"IDAT", zlib.compress(pixels)) + chunk(b"IEND", b""))

    def synthetic_recording(self):
        official = reference.OFFLINE_PREFIX + ["/synthetic/tools/rive"]
        commands = [self.command("head", ["git", "rev-parse", "HEAD"], "1" * 40 + "\n"),
                    self.command("worktree", ["git", "status", "--porcelain", "--untracked-files=normal"], ""),
                    self.command("browser-version", ["/synthetic/browser", "--version"], "synthetic Chromium\n"),
                    self.command("rust-toolchain", ["rustc", "--version", "--verbose"], "synthetic Rust\n"),
                    self.command("build-comparison-tool", ["cargo", "build", "--locked", "--bin", "rive-cli"], ""),
                    self.command("official-version", official + ["--version"], "Rive CLI 1.0.2\n")]
        for index, argv in enumerate(self.lock["audit_commands"]):
            commands.append(self.command(f"audit-{index:02d}", official + argv, "synthetic audit", retain=False))
        for name, flag in [("publish", "--publish"), ("rev", "--rev=build/probe.rev")]:
            commands.append(self.command("unauthenticated-" + name, official + [".", flag], "", code=3))
        fixtures = {}
        for name in ["static", "animated"]:
            for filename in ["scene.rml", "rive.yaml"]:
                data = (reference.HERE / "fixtures" / name / filename).read_bytes()
                self.put(f"projects/{name}/{filename}", data)
            riv = self.put(f"projects/{name}/build/reference.riv", b"RIVEsynthetic-unit-test-only")
            negative = self.put(f"projects/{name}-negative/build/reference.riv", b"RIVEsynthetic-negative-only")
            fixtures[name] = {"riv": riv, "negative_riv": negative,
                              "repeat_sha256": reference.digest(self.root / riv)}
            for suffix in ["", "-repeat", "-perturbed"]:
                for operation, mode in [("verify", "verify"), ("compile", "build")]:
                    flag = "--verify" if operation == "verify" else "--once"
                    data = {"riv": None if operation == "verify" else "./build/reference.riv", "problems": []}
                    report = {"success": True, "command": mode, "errors": [], "data": data}
                    commands.append(self.command(name + suffix + "-" + operation,
                                                 official + [".", flag, "--format=json"], json.dumps(report)))
                commands.append(self.command(name + suffix + "-inspect", official + ["inspect", ".", "--json"],
                                             json.dumps({"artboards": [{"name": "synthetic"}], "problems": []})))
            frames = []
            for index in [0, 15, 30]:
                filename = f"frame-{index}.png"
                self.put(f"renders/{name}/{filename}", self.png(index if name == "animated" else 0))
                frames.append({"index": index, "blank": False, "filename": filename})
            render = {"ok": True, "width": 128, "height": 96, "scale": 1, "fps": 60,
                      "animation": "QuarterTurn" if name == "animated" else None, "frames": frames}
            commands.append(self.command(name + "-render", ["/synthetic/rive-cli", "render"], json.dumps(render)))
            for label in ["control", "negative"]:
                value = 0 if label == "control" else 10
                report = {"ok": True, "reference_object_count": 5, "candidate_object_count": 5,
                          "type_deltas": [{"type_name": "Shape", "reference": 1, "candidate": 1, "delta": 0}],
                          "frames": [{"index": index, "pixel_difference": value} for index in [0, 15, 30]],
                          "max_pixel_difference": value}
                commands.append(self.command(name + "-" + label, ["/synthetic/rive-cli", "compare"], json.dumps(report)))
        return {"schema_version": 1, "status": "passed", "source_head": "1" * 40,
                "lock_sha256": reference.digest(reference.HERE / "lock.json"), "planning_base": self.lock["planning_base"],
                "rml_version": "1", "official": dict(self.lock["official"], binary_sha256="2" * 64, version_output="Rive CLI 1.0.2"),
                "browser_sha256": "3" * 64, "comparison_binary_sha256": "4" * 64,
                "runner_sha256": "5" * 64, "cargo_lock_sha256": "6" * 64,
                "runtime": {"package": self.lock["runtime"]["package"], "version": self.lock["runtime"]["version"],
                            "assets": {name: {"git_blob_sha1": sha, "sha256": "7" * 64}
                                       for name, sha in self.lock["runtime"]["git_blobs"].items()}},
                "commands": commands, "fixtures": fixtures}

    def save(self):
        self.recording["artifacts"] = {str(path.relative_to(self.root)): reference.digest(path)
                                       for path in self.root.rglob("*") if path.is_file() and path.name != "recording.json"}
        reference.write_json(self.root / "recording.json", self.recording)

    def command_named(self, name):
        return next(command for command in self.recording["commands"] if command["name"] == name)

    def test_complete_synthetic_contract_shape_is_accepted_offline(self):
        reference.verify_recording(self.root)

    def test_changed_retained_frame_is_rejected(self):
        (self.root / "renders/static/frame-0.png").write_bytes(self.png(90))
        with self.assertRaisesRegex(reference.ReferenceError, "digest mismatch"):
            reference.verify_recording(self.root)

    def test_missing_retained_frame_is_rejected(self):
        (self.root / "renders/static/frame-0.png").unlink()
        with self.assertRaisesRegex(reference.ReferenceError, "unindexed or missing"):
            reference.verify_recording(self.root)

    def test_rehashed_different_original_input_is_rejected(self):
        path = self.root / "projects/static/scene.rml"
        path.write_text(path.read_text().replace("FF2E8BC0", "FFF15A24"))
        self.save()
        with self.assertRaisesRegex(reference.ReferenceError, "committed original fixture"):
            reference.verify_recording(self.root)

    def test_unindexed_extra_file_is_rejected(self):
        self.put("unexpected.bin", b"extra")
        with self.assertRaisesRegex(reference.ReferenceError, "unindexed or missing"):
            reference.verify_recording(self.root)

    def test_failed_command_cannot_be_hidden_behind_successful_output(self):
        self.command_named("static-render")["exit_code"] = 1
        self.save()
        with self.assertRaisesRegex(reference.ReferenceError, "successful command evidence"):
            reference.verify_recording(self.root)

    def test_missing_command_output_binding_is_rejected(self):
        del self.command_named("static-render")["stdout_file"]
        self.save()
        with self.assertRaises(reference.ReferenceError):
            reference.verify_recording(self.root)

    def test_offline_claim_without_network_namespace_is_rejected(self):
        self.command_named("static-compile")["argv"] = ["/synthetic/tools/rive", ".", "--once", "--format=json"]
        self.save()
        with self.assertRaises(reference.ReferenceError):
            reference.verify_recording(self.root)

    def test_source_head_disagreement_is_rejected(self):
        self.recording["source_head"] = "a" * 40
        self.save()
        with self.assertRaises(reference.ReferenceError):
            reference.verify_recording(self.root)
