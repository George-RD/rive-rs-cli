#!/usr/bin/env bash
# Usage: ./data/log_attempt.sh <run_id> <target_name> <model> <category> <skills> <complexity> <json_path>
#
# Runs generate → validate → inspect on a scene JSON file and logs results to the ledger.
# Called by the orchestration workflow after each model produces output.

set -euo pipefail

RUN_ID="$1"
TARGET_NAME="$2"
MODEL="$3"
CATEGORY="$4"
SKILLS="$5"            # JSON array string, e.g. '["rive-scene-schema","rive-anti-patterns"]'
COMPLEXITY="$6"        # static | animated | interactive
JSON_PATH="$7"
ATTEMPT_NUM="${8:-0}"

# Validate inputs to prevent path traversal and SQL injection
if [[ ! "$TARGET_NAME" =~ ^[a-zA-Z0-9_-]+$ ]]; then
    echo "ERROR: target_name contains invalid characters" >&2; exit 1
fi
if [[ ! "$RUN_ID" =~ ^[a-zA-Z0-9_-]+$ ]]; then
    echo "ERROR: run_id contains invalid characters" >&2; exit 1
fi
if [[ ! "$ATTEMPT_NUM" =~ ^[0-9]+$ ]]; then
    echo "ERROR: attempt_num must be a non-negative integer" >&2; exit 1
fi

DB="data/rive_ledger.db"
RESULTS_DIR="docs/race-results/${TARGET_NAME}/${RUN_ID}"
mkdir -p "$RESULTS_DIR"
MODEL_SAFE="${MODEL//\//_}"
RIV_PATH="${RESULTS_DIR}/${ATTEMPT_NUM}-${MODEL_SAFE}.riv"

# SQL escape helper
sql_escape() { printf "%s" "$1" | sed "s/'/''/g"; }

START_MS=$(python3 -c 'import time; print(int(time.time()*1000))')

# Step 1: Generate
GENERATE_OK=0
GENERATE_ERR=""
GEN_ERR_FILE="${RESULTS_DIR}/.gen-err-${MODEL_SAFE}.txt"
if cargo run --quiet -- generate "$JSON_PATH" -o "$RIV_PATH" 2>"$GEN_ERR_FILE"; then
    GENERATE_OK=1
    rm -f "$GEN_ERR_FILE"
else
    GENERATE_ERR=$(head -5 "$GEN_ERR_FILE" | tr '\n' ' ')
fi

# Step 2: Validate (only if generate succeeded)
VALIDATE_OK=0
if [ "$GENERATE_OK" -eq 1 ]; then
    if cargo run --quiet -- validate "$RIV_PATH" 2>/dev/null; then
        VALIDATE_OK=1
    fi
fi

# Step 3: Inspect (count objects)
OBJECT_COUNT=0
if [ "$VALIDATE_OK" -eq 1 ]; then
    OBJECT_COUNT=$(cargo run --quiet -- inspect "$RIV_PATH" --json 2>/dev/null | python3 -c "import sys,json; d=json.load(sys.stdin); print(len(d.get('objects',[])))" 2>/dev/null || echo 0)
fi

END_MS=$(python3 -c 'import time; print(int(time.time()*1000))')
DURATION=$((END_MS - START_MS))

# Determine error stage
ERROR_STAGE="NULL"
ERROR_MSG="NULL"
if [ "$GENERATE_OK" -eq 0 ]; then
    ERROR_STAGE="'schema'"
    ERROR_MSG="'$(echo "$GENERATE_ERR" | sed "s/'/''/g")'"
elif [ "$VALIDATE_OK" -eq 0 ]; then
    ERROR_STAGE="'validation'"
    ERROR_MSG="'validation failed'"
fi

SUCCESS=$((GENERATE_OK && VALIDATE_OK))

# Log to SQLite
sqlite3 "$DB" "INSERT INTO attempts (run_id, target_name, model, category, skills_loaded, attempt_num, output_json_path, output_riv_path, generate_ok, validate_ok, inspect_object_count, error_stage, error_message, action_type, complexity_tier, success, duration_ms) VALUES ('$(sql_escape "$RUN_ID")', '$(sql_escape "$TARGET_NAME")', '$(sql_escape "$MODEL")', '$(sql_escape "$CATEGORY")', '$(sql_escape "$SKILLS")', $ATTEMPT_NUM, '$(sql_escape "$JSON_PATH")', '$(sql_escape "$RIV_PATH")', $GENERATE_OK, $VALIDATE_OK, $OBJECT_COUNT, $ERROR_STAGE, $ERROR_MSG, 'create', '$(sql_escape "$COMPLEXITY")', $SUCCESS, $DURATION);"

# Output summary
if [ "$SUCCESS" -eq 1 ]; then
    echo "OK  model=$MODEL objects=$OBJECT_COUNT duration=${DURATION}ms"
else
    echo "FAIL model=$MODEL stage=$(echo $ERROR_STAGE | tr -d "'") duration=${DURATION}ms"
    [ "$ERROR_MSG" != "NULL" ] && echo "  error: $(echo $ERROR_MSG | tr -d "'")"
fi
