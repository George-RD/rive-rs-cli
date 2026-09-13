import argparse
import re
import tempfile
from pathlib import Path

import reference
from schema_facts import type_names


MAX_SCHEMA_TYPES = 1000


def capture(binary: Path, archive: Path, output: Path, lock_path: Path | None = None) -> None:
    lock_path = lock_path or reference.HERE / "lock.json"
    lock = reference.read_json(lock_path)
    installation = reference.verify_installation(binary, archive, lock["official"])
    if output.exists():
        raise reference.ReferenceError("schema capture never overwrites an existing directory")
    output.mkdir(parents=True)
    recorder = reference.Recorder(output)
    result = {"schema_version": 1, "status": "failed", "official": installation,
              "lock_sha256": reference.digest(lock_path),
              "commands": recorder.commands,
              "scope": "Public schema facts only; not runtime or parity evidence.",
              "source": "https://rive.app/docs/cli/reference/commands",
              "attribution": "Rive, Inc. Public CLI schema; no executable, docs, samples or assets retained."}
    try:
        result["source_head"] = recorder.run("head", ["git", "rev-parse", "HEAD"], reference.ROOT).stdout.decode().strip()
        if not re.fullmatch(r"[0-9a-f]{40}", result["source_head"]):
            raise reference.ReferenceError("invalid source head")
        if recorder.run("worktree", ["git", "status", "--porcelain"], reference.ROOT).stdout:
            raise reference.ReferenceError("schema capture requires a clean checkout")
        with tempfile.TemporaryDirectory(prefix="rive-schema-home-") as home:
            env = reference.isolated_environment(Path(home))
            version = recorder.run("version", reference.OFFLINE_PREFIX + [str(binary), "--version"], reference.ROOT, env)
            reference.verify_version(version.stdout.decode(), lock["official"]["version"])
            result["official"]["version_output"] = version.stdout.decode().strip()
            inventory = recorder.run("types", reference.OFFLINE_PREFIX + [str(binary), "schema", "--list"], reference.ROOT, env)
            names = type_names(inventory.stdout.decode())
            if not 0 < len(names) <= MAX_SCHEMA_TYPES:
                raise reference.ReferenceError("schema inventory exceeds the capture budget")
            for name in names:
                for suffix, flags in [("runtime", []), ("all", ["--all"])]:
                    recorder.run(name + "-" + suffix,
                                 reference.OFFLINE_PREFIX + [str(binary), "schema", name] + flags,
                                 reference.ROOT, env)
        result["status"] = "passed"
    finally:
        result["artifacts"] = {str(path.relative_to(output)): reference.digest(path)
                               for path in sorted(output.rglob("*")) if path.is_file()}
        reference.write_json(output / "schema-capture.json", result)


def main() -> None:
    parser = argparse.ArgumentParser(description="Capture the pinned public schema in an isolated developer run.")
    parser.add_argument("--official-cli", type=Path, required=True)
    parser.add_argument("--archive", type=Path, required=True)
    parser.add_argument("--lock", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    try:
        capture(args.official_cli.resolve(), args.archive.resolve(), args.output.resolve(), args.lock)
    except (reference.ReferenceError, OSError, ValueError, KeyError, TypeError) as error:
        parser.exit(1, f"schema capture: {error}\n")


if __name__ == "__main__":
    main()
