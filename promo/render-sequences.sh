#!/usr/bin/env bash
# Renders the deterministic PNG sequences the Remotion promo composes.
# Rive never runs inside Remotion: every frame is produced by rive-cli itself,
# so the video is made of exactly the frames the test suite verifies.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CLI="${RIVE_CLI:-$ROOT/target/release/rive-cli}"
OUT="$ROOT/promo/public/seq"

if [ ! -x "$CLI" ]; then
  echo "build the CLI first: cargo build --release" >&2
  exit 1
fi

render() {
  local name="$1" frames="$2"
  shift 2
  echo "==> $name"
  rm -rf "${OUT:?}/$name"
  "$CLI" render "$ROOT/showcase/$name.riv" \
    --frames "$frames" \
    --width 480 --height 480 --scale 2 \
    --background '#0B0E17' \
    -o "$OUT/$name" "$@" >/dev/null
}

mkdir -p "$OUT"

# keep the promo's terminal text identical to the captured run the site shows
node -e '
const fs = require("node:fs");
const lines = fs.readFileSync(process.argv[1], "utf8").split("\n");
fs.writeFileSync(process.argv[2], "export const TRANSCRIPT: string[] = " + JSON.stringify(lines, null, 2) + ";\n");
' "$ROOT/site/verify.txt" "$ROOT/promo/src/transcript.ts"

render wordmark          0..150:1
render liquid_loader     0..120:1
render textured_scene    0..240:2
render control_panel     0..120:1 --state-machine Panel --input level=70 --pointer down:300,452@40
render orbital_loader    0..120:1
render pulse_button      0..90:1  --state-machine PulseButtonMachine --input isHovered=true
render radial_dashboard  0..120:1
render audio_equaliser   0..120:1
render day_night_toggle  0..90:1  --state-machine DayNightMachine --input isNight=true@45
render rocket_launch     0..120:1

echo "sequences written to $OUT"
