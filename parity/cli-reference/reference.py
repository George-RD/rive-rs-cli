import argparse
import hashlib
import json
import math
import os
import platform
import re
import shutil
import struct
import subprocess
import sys
import tarfile
import tempfile
from datetime import datetime, timezone
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
FRAMES = [0, 15, 30]
OFFLINE_PREFIX = ["unshare", "--user", "--map-root-user", "--net", "--"]
MAX_BINARY_BYTES = 512 * 1024 * 1024


class ReferenceError(Exception):
    pass


def digest(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def read_json(path: Path) -> dict:
    value = json.loads(path.read_text())
    if not isinstance(value, dict):
        raise ReferenceError(f"expected a JSON object: {path.name}")
    return value


def write_json(path: Path, value: dict) -> None:
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + "\n")


def verify_installation(binary: Path, archive: Path, lock: dict) -> dict:
    if not binary.is_file():
        raise ReferenceError("official binary is missing; supply --official-cli explicitly")
    if not os.access(binary, os.X_OK):
        raise ReferenceError("official binary is not executable")
    if [platform.system(), platform.machine()] != lock["platform"]:
        raise ReferenceError("unsupported platform; this reference is pinned to Linux x86_64")
    if not archive.is_file():
        raise ReferenceError("pinned archive is missing; reference runs never download it")
    if digest(archive) != lock["archive_sha256"]:
        raise ReferenceError("official archive digest mismatch")
    with tarfile.open(archive, "r:gz") as package:
        members = [member for member in package.getmembers() if Path(member.name).name == "rive"]
        if len(members) != 1 or not members[0].isfile():
            raise ReferenceError("archive must contain exactly one regular rive executable")
        member = members[0]
        if not 0 < member.size <= MAX_BINARY_BYTES:
            raise ReferenceError("official executable exceeds the reference size budget")
        stream = package.extractfile(member)
        if stream is None:
            raise ReferenceError("archive executable cannot be read")
        with stream:
            expected = hashlib.file_digest(stream, "sha256").hexdigest()
    actual = digest(binary)
    if actual != expected:
        raise ReferenceError("official binary does not match the pinned archive")
    return {"binary_sha256": actual, "archive_sha256": lock["archive_sha256"],
            "source": lock["source"], "archive_member": member.name}


def verify_version(output: str, expected: str) -> None:
    versions = re.findall(r"(?<![\w.])v?(\d+\.\d+\.\d+(?:[-+][\w.-]+)?)(?![\w.])", output)
    if versions != [expected]:
        raise ReferenceError(f"wrong official CLI version; require {expected}, received {output.strip()!r}")


def isolated_environment(home: Path) -> dict:
    environment = {"PATH": os.defpath, "HOME": str(home), "LANG": "C.UTF-8",
                   "LC_ALL": "C.UTF-8", "TZ": "UTC", "RIVE_NO_TUI": "1", "RIVE_ANALYTICS": "off"}
    for key in ["XDG_CONFIG_HOME", "XDG_CACHE_HOME", "XDG_DATA_HOME"]:
        directory = home / key.lower()
        directory.mkdir()
        environment[key] = str(directory)
    return environment


def checked_relative(root: Path, name: str) -> Path:
    path = Path(name)
    if path.is_absolute() or ".." in path.parts:
        raise ReferenceError("evidence path must be relative and contained")
    result = (root / path).resolve()
    if not result.is_relative_to(root.resolve()) or not result.is_file():
        raise ReferenceError(f"missing or escaping evidence file: {name}")
    return result


def check_comparison(report: dict, changed: bool) -> None:
    if report.get("ok") is not True:
        raise ReferenceError("comparison did not complete successfully")
    rows = report.get("frames", [])
    if [row.get("index") for row in rows] != FRAMES:
        raise ReferenceError("comparison has incomplete frame coverage")
    values = [row.get("pixel_difference") for row in rows]
    values.append(report.get("max_pixel_difference"))
    if any(type(value) not in (int, float) or not math.isfinite(value)
           or not 0 <= value <= 100 for value in values):
        raise ReferenceError("comparison contains invalid pixel measurements")
    if values[-1] != max(values[:-1]):
        raise ReferenceError("comparison maximum disagrees with frame measurements")
    if changed and values[-1] <= 0.1:
        raise ReferenceError("perturbed candidate produced no detected visible difference")
    if not changed and values[-1] != 0:
        raise ReferenceError("identical reference control produced a visible difference")
    if not isinstance(report.get("type_deltas"), list) or not report["type_deltas"]:
        raise ReferenceError("comparison has no structural measurements")
    if (report.get("reference_object_count", 0) <= 0
            or report.get("reference_object_count") != report.get("candidate_object_count")
            or any(row.get("delta") != 0 for row in report.get("type_deltas", []))):
        raise ReferenceError("comparison produced a structural difference")


def check_render(report: dict, directory: Path, animated: bool) -> list[str]:
    if report.get("ok", True) is not True or [row.get("index") for row in report.get("frames", [])] != FRAMES:
        raise ReferenceError("runtime evidence has incomplete frame coverage")
    if [report.get("width"), report.get("height"), report.get("scale"), report.get("fps")] != [128, 96, 1, 60]:
        raise ReferenceError("runtime evidence has the wrong viewport")
    if animated and report.get("animation") != "QuarterTurn":
        raise ReferenceError("runtime did not select the requested animation")
    hashes = []
    for frame in report["frames"]:
        if frame.get("blank") is not False:
            raise ReferenceError("official runtime rendered a blank or unsupported frame")
        image = checked_relative(directory, frame["filename"])
        data = image.read_bytes()
        if (len(data) < 45 or data[:8] != b"\x89PNG\r\n\x1a\n"
                or data[12:16] != b"IHDR" or struct.unpack(">II", data[16:24]) != (128, 96)
                or data[-12:] != b"\x00\x00\x00\x00IEND\xaeB`\x82"):
            raise ReferenceError("runtime frame is not complete 128x96 PNG evidence")
        hashes.append(digest(image))
    if animated and len(set(hashes)) != len(FRAMES):
        raise ReferenceError("animation did not produce three distinct captured frames")
    if not animated and len(set(hashes)) != 1:
        raise ReferenceError("static control changed across captured frames")
    return hashes


class Recorder:
    def __init__(self, directory: Path):
        self.directory = directory
        self.commands = []
        (directory / "commands").mkdir()

    def run(self, name: str, argv: list[str], cwd: Path, env=None, required=True,
            retain_output=True, timeout=120) -> subprocess.CompletedProcess:
        try:
            result = subprocess.run(argv, cwd=cwd, env=env, capture_output=True, timeout=timeout)
        except (OSError, subprocess.TimeoutExpired) as error:
            self.commands.append({"name": name, "argv": argv, "cwd": str(cwd),
                                  "exit_code": None, "execution_error": str(error)})
            raise ReferenceError(f"{name} could not execute: {error}") from error
        record = {"name": name, "argv": argv, "cwd": str(cwd), "exit_code": result.returncode}
        for label, data in [("stdout", result.stdout), ("stderr", result.stderr)]:
            record[label + "_sha256"] = hashlib.sha256(data).hexdigest()
            if retain_output:
                path = self.directory / "commands" / f"{name}.{label}"
                path.write_bytes(data)
                record[label + "_file"] = str(path.relative_to(self.directory))
        self.commands.append(record)
        if required and result.returncode != 0:
            raise ReferenceError(f"{name} failed with exit {result.returncode}; inspect command evidence")
        return result


def own_json(result: subprocess.CompletedProcess) -> dict:
    value = json.loads(result.stdout)
    if not isinstance(value, dict) or value.get("ok", True) is not True:
        raise ReferenceError("rive-cli did not return successful JSON evidence")
    return value


def runtime_identity(lock: dict) -> dict:
    assets = {}
    for name, expected in lock["git_blobs"].items():
        path = ROOT / name
        data = path.read_bytes()
        git_hash = hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()
        if git_hash != expected:
            raise ReferenceError(f"pinned runtime/harness changed: {name}")
        assets[name] = {"git_blob_sha1": git_hash, "sha256": digest(path)}
    return {"package": lock["package"], "version": lock["version"], "assets": assets}


def check_official_build(report: dict, command: str) -> None:
    if (not isinstance(report, dict) or report.get("success") is not True
            or report.get("command") != command or report.get("errors")
            or not isinstance(report.get("data"), dict)
            or any(problem.get("severity") == "error" for problem in report["data"].get("problems", []))):
        raise ReferenceError("official compilation reported errors or a different mode despite its exit code")


def compile_fragment(recorder: Recorder, binary: Path, project: Path, env: dict, name: str) -> Path:
    verified = recorder.run(name + "-verify", OFFLINE_PREFIX + [str(binary), ".", "--verify", "--format=json"], project, env)
    check_official_build(json.loads(verified.stdout), "verify")
    result = recorder.run(name + "-compile", OFFLINE_PREFIX + [str(binary), ".", "--once", "--format=json"], project, env)
    report = json.loads(result.stdout)
    check_official_build(report, "build")
    value = report.get("data", {}).get("riv")
    if not isinstance(value, str):
        raise ReferenceError("official compilation did not identify an output")
    path = (project / value).resolve()
    if not path.is_relative_to((project / "build").resolve()) or path.suffix != ".riv":
        raise ReferenceError("official output escaped the project build directory")
    if not path.is_file() or path.read_bytes()[:4] != b"RIVE":
        raise ReferenceError("official compilation did not produce RIVE bytes")
    result = recorder.run(name + "-inspect", OFFLINE_PREFIX + [str(binary), "inspect", ".", "--json"], project, env)
    inspection = json.loads(result.stdout)
    if not isinstance(inspection, dict) or not inspection.get("artboards"):
        raise ReferenceError("official inspection did not contain an artboard")
    if any(problem.get("severity") == "error" for problem in inspection.get("problems", [])):
        raise ReferenceError("official inspection reported an error")
    return path


def verify_recording(directory: Path, expected_head: str) -> dict:
    recording = read_json(directory / "recording.json")
    if recording.get("schema_version") != 1 or recording.get("status") != "passed":
        raise ReferenceError("recording is absent, failed, or incomplete; no runtime pass can be inferred")
    if not re.fullmatch(r"[0-9a-f]{40}", expected_head) or recording.get("source_head") != expected_head:
        raise ReferenceError("recording does not match the expected source head")
    lock = read_json(HERE / "lock.json")
    if recording.get("lock_sha256") != digest(HERE / "lock.json"):
        raise ReferenceError("recording used a different reference lock")
    if recording.get("rml_version") != lock["rml_version"] or recording.get("planning_base") != lock["planning_base"]:
        raise ReferenceError("recording has a different RML version or planning base")
    official = recording.get("official", {})
    if official.get("archive_sha256") != lock["official"]["archive_sha256"] or official.get("source") != lock["official"]["source"]:
        raise ReferenceError("recording used a different official archive")
    verify_version(official.get("version_output", ""), lock["official"]["version"])
    for value, length in [(recording.get("source_head"), 40), (official.get("binary_sha256"), 64),
                          (recording.get("browser_sha256"), 64), (recording.get("comparison_binary_sha256"), 64),
                          (recording.get("runner_sha256"), 64), (recording.get("cargo_lock_sha256"), 64)]:
        if not isinstance(value, str) or not re.fullmatch("[0-9a-f]{" + str(length) + "}", value):
            raise ReferenceError("recording has incomplete tool or source identities")
    runtime = recording.get("runtime", {})
    if runtime.get("package") != lock["runtime"]["package"] or runtime.get("version") != lock["runtime"]["version"]:
        raise ReferenceError("recording used a different runtime")
    for name, expected in lock["runtime"]["git_blobs"].items():
        identity = runtime.get("assets", {}).get(name, {})
        if identity.get("git_blob_sha1") != expected or not re.fullmatch("[0-9a-f]{64}", identity.get("sha256", "")):
            raise ReferenceError("recording has incomplete runtime asset identities")
    artifacts = recording.get("artifacts", {})
    if not isinstance(artifacts, dict) or not artifacts:
        raise ReferenceError("recording has no retained artifacts")
    actual_files = {str(path.relative_to(directory)) for path in directory.rglob("*")
                    if path.is_file() and path != directory / "recording.json"}
    if set(artifacts) != actual_files:
        raise ReferenceError("recording has unindexed or missing evidence files")
    for name, expected in artifacts.items():
        if digest(checked_relative(directory, name)) != expected:
            raise ReferenceError(f"retained evidence digest mismatch: {name}")
    commands = {}
    for command in recording.get("commands", []):
        name = command["name"]
        if name in commands or type(command.get("exit_code")) is not int:
            raise ReferenceError("duplicate or incomplete command evidence")
        commands[name] = command
        for label in ["stdout", "stderr"]:
            path = command.get(label + "_file")
            if (not name.startswith("audit-") and path is None) or (path is not None and
                    (path != f"commands/{name}.{label}" or artifacts.get(path) != command[label + "_sha256"])):
                raise ReferenceError("command evidence is not bound to retained output")
    required = ["head", "worktree", "browser-version", "rust-toolchain", "build-comparison-tool", "official-version"]
    for name in ["static", "animated"]:
        required += [name + "-" + operation for operation in ["render", "control", "negative"]]
        for suffix in ["", "-repeat", "-perturbed"]:
            required += [name + suffix + "-" + operation for operation in ["verify", "compile", "inspect"]]
    for name in required:
        if name not in commands or commands[name]["exit_code"] != 0:
            raise ReferenceError(f"missing successful command evidence: {name}")
        if any(f"commands/{name}.{label}" not in artifacts for label in ["stdout", "stderr"]):
            raise ReferenceError(f"missing required command output: {name}")
    for index, argv in enumerate(lock["audit_commands"]):
        command = commands.get(f"audit-{index:02d}", {})
        if command.get("argv", [])[len(OFFLINE_PREFIX) + 1:] != argv:
            raise ReferenceError("missing public command audit evidence")
    if (directory / "commands/head.stdout").read_text().strip() != recording["source_head"]:
        raise ReferenceError("recorded source head differs from command evidence")
    if (directory / "commands/worktree.stdout").read_text().strip():
        raise ReferenceError("recording was not captured from a clean worktree")
    if (directory / "commands/official-version.stdout").read_text().strip() != official["version_output"]:
        raise ReferenceError("recorded version differs from command evidence")
    for name, command in commands.items():
        if (name.startswith(("audit-", "official-", "unauthenticated-"))
                or name.endswith(("-verify", "-compile", "-inspect"))):
            if command.get("argv", [])[:len(OFFLINE_PREFIX)] != OFFLINE_PREFIX:
                raise ReferenceError("official command evidence lacks network isolation")
    for label in ["publish", "rev"]:
        if commands.get("unauthenticated-" + label, {}).get("exit_code") not in [3, 7]:
            raise ReferenceError("authentication/network restriction is inconclusive")
    for name in ["static", "animated"]:
        fixture = recording.get("fixtures", {}).get(name, {})
        for filename in ["scene.rml", "rive.yaml"]:
            path = f"projects/{name}/{filename}"
            if artifacts.get(path) != digest(HERE / "fixtures" / name / filename):
                raise ReferenceError("reference input differs from the committed original fixture")
        for key in ["riv", "negative_riv"]:
            path = fixture.get(key, "")
            if path not in artifacts or checked_relative(directory, path).read_bytes()[:4] != b"RIVE":
                raise ReferenceError("missing compiled RIVE artifact")
        if fixture.get("repeat_sha256") != artifacts[fixture["riv"]]:
            raise ReferenceError("repeated compilation evidence differs")
        for suffix in ["", "-repeat", "-perturbed"]:
            for operation, mode in [("verify", "verify"), ("compile", "build")]:
                check_official_build(read_json(directory / f"commands/{name}{suffix}-{operation}.stdout"), mode)
        for operation in ["render", "control", "negative"]:
            report = read_json(directory / f"commands/{name}-{operation}.stdout")
            if operation == "render":
                check_render(report, directory / "renders" / name, name == "animated")
            else:
                check_comparison(report, operation == "negative")
    return recording


def capture(binary: Path, archive: Path, browser: Path, output: Path) -> None:
    lock = read_json(HERE / "lock.json")
    official = verify_installation(binary, archive, lock["official"])
    runtime = runtime_identity(lock["runtime"])
    if not browser.is_file() or not os.access(browser, os.X_OK):
        raise ReferenceError("browser binary is missing or not executable")
    if output.exists():
        raise ReferenceError("output already exists; reference runs never overwrite a baseline")
    output.mkdir(parents=True)
    recorder = Recorder(output)
    recording = {"schema_version": 1, "status": "failed", "lock_sha256": digest(HERE / "lock.json"),
                 "started_at": datetime.now(timezone.utc).isoformat(), "official": official,
                 "runtime": runtime, "platform": platform.platform(), "commands": recorder.commands,
                 "rml_version": lock["rml_version"], "runner_sha256": digest(Path(__file__)), "fixtures": {},
                 "limitations": ["Only original shape/paint and transform animation fixtures are tested.",
                                 "Scripts, shaders, signed distribution and account-bound publishing remain untested.",
                                 "Runtime uses bundled local assets but is not network-isolated."]}
    try:
        head = recorder.run("head", ["git", "rev-parse", "HEAD"], ROOT).stdout.decode().strip()
        dirty = recorder.run("worktree", ["git", "status", "--porcelain", "--untracked-files=normal"], ROOT).stdout
        if dirty:
            raise ReferenceError("reference capture requires a clean committed worktree")
        recording["source_head"] = head
        recording["planning_base"] = lock["planning_base"]
        recording["browser_sha256"] = digest(browser)
        recorder.run("browser-version", [str(browser), "--version"], ROOT)
        recorder.run("rust-toolchain", ["rustc", "--version", "--verbose"], ROOT)
        build_env = dict(os.environ, CARGO_TARGET_DIR=str(ROOT / "target"))
        recorder.run("build-comparison-tool", ["cargo", "build", "--locked", "--bin", "rive-cli"], ROOT, build_env, timeout=1200)
        ours = ROOT / "target" / "debug" / "rive-cli"
        recording["comparison_binary_sha256"] = digest(ours)
        recording["cargo_lock_sha256"] = digest(ROOT / "Cargo.lock")
        render_env = dict(os.environ, RIVE_HARNESS=str(ROOT / "assets" / "render-harness.html"),
                          RIVE_CHROME=str(browser))
        with tempfile.TemporaryDirectory(prefix="rive-reference-home-") as home:
            env = isolated_environment(Path(home))
            version = recorder.run("official-version", OFFLINE_PREFIX + [str(binary), "--version"], ROOT, env)
            verify_version(version.stdout.decode(), lock["official"]["version"])
            official["version_output"] = version.stdout.decode().strip()
            recording["official_environment"] = {"credentials": "new empty HOME and XDG directories; environment allowlist",
                                                   "network": "unshare --user --map-root-user --net for every official command"}
            for index, command in enumerate(lock["audit_commands"]):
                recorder.run(f"audit-{index:02d}", OFFLINE_PREFIX + [str(binary)] + command,
                             ROOT, env, required=False, retain_output=False)
            for name in ["static", "animated"]:
                project = output / "projects" / name
                shutil.copytree(HERE / "fixtures" / name, project)
                riv = compile_fragment(recorder, binary, project, env, name)
                first_hash = digest(riv)
                repeated = compile_fragment(recorder, binary, project, env, name + "-repeat")
                if digest(repeated) != first_hash:
                    raise ReferenceError("repeated unsigned compilation changed the reference bytes")
                negative_project = output / "projects" / (name + "-negative")
                shutil.copytree(HERE / "fixtures" / name, negative_project)
                source = negative_project / "scene.rml"
                text = source.read_text()
                if text.count("FF2E8BC0") != 1:
                    raise ReferenceError("negative control must perturb exactly one paint")
                source.write_text(text.replace("FF2E8BC0", "FFF15A24"))
                negative = compile_fragment(recorder, binary, negative_project, env, name + "-perturbed")
                recording["fixtures"][name] = {"riv": str(riv.relative_to(output)),
                                               "negative_riv": str(negative.relative_to(output)),
                                               "repeat_sha256": digest(repeated)}
                render_dir = output / "renders" / name
                animation = ["--animation", "QuarterTurn"] if name == "animated" else []
                result = recorder.run(name + "-render", [str(ours), "render", str(riv), "--frames", "0,15,30",
                                      "--width", "128", "--height", "96", "--scale", "1", "--json", "-o", str(render_dir)] + animation,
                                      ROOT, render_env)
                check_render(own_json(result), render_dir, name == "animated")
                selected = ["--reference-animation", "QuarterTurn", "--candidate-animation", "QuarterTurn"] if animation else []
                for label, candidate in [("control", riv), ("negative", negative)]:
                    result = recorder.run(name + "-" + label, [str(ours), "compare", str(riv), str(candidate),
                                          "--frames", "0,15,30", "--width", "128", "--height", "96", "--scale", "1", "--json"] + selected,
                                          ROOT, render_env)
                    check_comparison(own_json(result), label == "negative")
            project = output / "probes"
            shutil.copytree(HERE / "fixtures" / "static", project)
            for label, flag in [("publish", "--publish"), ("rev", "--rev=build/probe.rev")]:
                result = recorder.run("unauthenticated-" + label, OFFLINE_PREFIX + [str(binary), ".", flag],
                                      project, env, required=False)
                if result.returncode not in [3, 7]:
                    raise ReferenceError(f"{label} restriction was not established; observed exit {result.returncode}")
        recording["status"] = "passed"
    except (ReferenceError, OSError, ValueError, KeyError, TypeError) as error:
        recording["failure"] = str(error)
        raise
    finally:
        recording["artifacts"] = {str(path.relative_to(output)): digest(path)
                                  for path in sorted(output.rglob("*")) if path.is_file()}
        write_json(output / "recording.json", recording)
    try:
        verify_recording(output, head)
    except (ReferenceError, OSError, ValueError, KeyError, TypeError) as error:
        recording["status"] = "failed"
        recording["failure"] = str(error)
        write_json(output / "recording.json", recording)
        raise


def main() -> int:
    parser = argparse.ArgumentParser(description="Explicit, pinned official CLI reference capture; never downloads tools")
    subcommands = parser.add_subparsers(dest="command", required=True)
    run = subcommands.add_parser("capture")
    run.add_argument("--official-cli", type=Path, required=True)
    run.add_argument("--archive", type=Path, required=True)
    run.add_argument("--browser", type=Path, required=True)
    run.add_argument("--output", type=Path, required=True)
    verify = subcommands.add_parser("verify", help="check retained evidence offline; does not rerun the official CLI")
    verify.add_argument("directory", type=Path)
    verify.add_argument("--expected-head", required=True)
    args = parser.parse_args()
    try:
        if args.command == "capture":
            capture(args.official_cli.resolve(), args.archive.resolve(), args.browser.resolve(), args.output.resolve())
            print("Reference capture and retained-evidence verification passed.")
        else:
            verify_recording(args.directory.resolve(), args.expected_head)
            print("Retained evidence verified offline. No official CLI or runtime was executed.")
        return 0
    except (ReferenceError, OSError, ValueError, KeyError, TypeError, tarfile.TarError) as error:
        print(f"reference failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
