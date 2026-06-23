#!/usr/bin/env bash
# Parity harness: recreate an official .riv from a SceneSpec fixture and measure
# the semantic structural distance to the reference. Lower is better; 0 means
# semantically identical (up to accepted encoding deltas: synthetic names,
# default-omitted properties, object emission order).
set -euo pipefail
cd "$(dirname "$0")"

FIXTURE="${PARITY_FIXTURE:-tests/fixtures/comparison_quantize_test.json}"
REFERENCE="${PARITY_REFERENCE:-demo/riv/reference/quantize_test.riv}"
FILE_ID="${PARITY_FILE_ID:-11807}"

cargo build -q

python3 scripts/parity_metric.py \
  --binary target/debug/rive-cli \
  --fixture "$FIXTURE" \
  --reference "$REFERENCE" \
  --file-id "$FILE_ID"
