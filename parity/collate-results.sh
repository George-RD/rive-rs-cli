#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${RIVE_CLI:-$ROOT/target/release/rive-cli}"
RESULTS="$ROOT/parity/results.json"

if [[ ! -x "$BIN" ]]; then
  echo "missing $BIN; run cargo build --release first" >&2
  exit 1
fi

run() {
  local name="$1" frames="$2"
  shift 2
  "$BIN" compare \
    "$ROOT/parity/official/$name.riv" \
    "$ROOT/parity/reproductions/$name.riv" \
    --frames "$frames" --width 512 --height 512 --scale 2 \
    --background '#0B0E17' --max-pixel-diff 5 --json "$@"
}

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

run button 0,15,30,45 \
  --reference-state-machine "State Machine 1" \
  --candidate-state-machine "State Machine 1" > "$tmp/button.json"
run coffee_loader 0,15,30,45 \
  --reference-state-machine "State Machine 1" \
  --candidate-state-machine "State Machine 1" > "$tmp/coffee_loader.json"

python3 - "$tmp" "$RESULTS" <<'PY'
import json
import pathlib
import sys

tmp = pathlib.Path(sys.argv[1])
out = pathlib.Path(sys.argv[2])

RUNGS = [
    {
        "id": "button",
        "title": "button",
        "upstream": "rive-app/rive-flutter — example/assets/button.riv",
        "state_machine": "State Machine 1",
        "note": "An 805 kB embedded Inter variable font, a text run with two variation axes, three animations and a listener-driven state machine.",
    },
    {
        "id": "coffee_loader",
        "title": "coffee_loader",
        "upstream": "rive-app/rive-runtime — renderer/webgpu_player/rivs/coffee_loader.riv",
        "state_machine": "State Machine 1",
        "note": "250 objects, five state-machine layers including a 1D blend state, nine shared interpolators and ninety keyframes.",
    },
]

entries = []
for rung in RUNGS:
    report = json.loads((tmp / f"{rung['id']}.json").read_text())
    if report["missing_type_names"]:
        raise SystemExit(
            f"{rung['id']}: missing type names: {', '.join(report['missing_type_names'])}"
        )
    entry = dict(rung)
    entry["official"] = f"parity/official/{rung['id']}.riv"
    entry["reproduction"] = f"parity/reproductions/{rung['id']}.riv"
    entry["source"] = f"parity/reproductions/{rung['id']}.json"
    entry["reference_object_count"] = report["reference_object_count"]
    entry["candidate_object_count"] = report["candidate_object_count"]
    entry["max_pixel_difference"] = report["max_pixel_difference"]
    entry["frames"] = report["frames"]
    entry["missing_type_names"] = report["missing_type_names"]
    entry["type_deltas"] = [row for row in report["type_deltas"] if row["delta"] != 0]
    entries.append(entry)

out.write_text(json.dumps(entries, indent=2) + "\n")
print(f"wrote {out} with {len(entries)} rungs")
PY
