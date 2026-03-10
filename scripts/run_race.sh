#!/usr/bin/env bash
# Usage: ./scripts/run_race.sh <target_name> <prompt_description> [complexity]
#
# Runs a multi-model race for a Rive fixture target.
# Each configured model generates scene JSON from the prompt, then the output
# is validated, logged to the SQLite ledger, and persisted to docs/race-results/.
#
# Prerequisites:
#   - At least one CLI tool installed: claude, codex
#   - jq, sqlite3, python3 installed
#   - timeout or gtimeout installed
#   - cargo build passes
#
# Models are dispatched via local CLI tools:
#   codex  — Codex model (gpt-5.4)
#   claude — Opus model (claude opus)
#
# CLIs not found on PATH are skipped with a warning.
#
# Examples:
#   ./scripts/run_race.sh loader "spinning loading indicator with smooth rotation"
#   ./scripts/run_race.sh icon_set "3 simple icons: home, settings, profile" static
#   ./scripts/run_race.sh game_hud "health bar and score counter" animated

set -euo pipefail

# Check required tools
for cmd in jq sqlite3 python3; do
    if ! command -v "$cmd" &>/dev/null; then
        echo "ERROR: Required tool '$cmd' not found. Please install it."
        exit 1
    fi
done

if command -v timeout &>/dev/null; then
    TIMEOUT_BIN="timeout"
elif command -v gtimeout &>/dev/null; then
    TIMEOUT_BIN="gtimeout"
else
    echo "ERROR: Required tool 'timeout' not found. Install GNU coreutils ('timeout' or 'gtimeout')."
    exit 1
fi

# Warn about optional CLI tools
for cmd in claude codex; do
    if ! command -v "$cmd" &>/dev/null; then
        echo "WARNING: Optional CLI tool '$cmd' not found — its model will be skipped."
    fi
done

TARGET_NAME="${1:?Usage: run_race.sh <target_name> <prompt> [complexity]}"
PROMPT="${2:?Usage: run_race.sh <target_name> <prompt> [complexity]}"
COMPLEXITY="${3:-animated}"

# Validate target name (alphanumeric, hyphens, underscores only)
if [[ ! "$TARGET_NAME" =~ ^[a-zA-Z0-9_-]+$ ]]; then
    echo "ERROR: target_name must contain only alphanumeric characters, hyphens, and underscores"
    exit 1
fi

# SQL escape helper
sql_escape() { printf "%s" "$1" | sed "s/'/''/g"; }

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
DB="$PROJECT_ROOT/data/rive_ledger.db"
SKILLS_DIR="$PROJECT_ROOT/.opencode/skills"
RUN_ID="race_$(date +%s)_${RANDOM}_${TARGET_NAME}"
RESULTS_DIR="$PROJECT_ROOT/docs/race-results/$TARGET_NAME/$RUN_ID"

# Ensure ledger exists
if [ ! -f "$DB" ]; then
    echo "Initializing ledger..."
    sqlite3 "$DB" < "$PROJECT_ROOT/data/init_ledger.sql"
fi

mkdir -p "$RESULTS_DIR"
cd "$PROJECT_ROOT"

# Build skills context from .opencode/skills/
SKILLS_CONTEXT=""
SKILLS_LIST="[]"
if [ -d "$SKILLS_DIR" ]; then
    SKILL_NAMES=()
    shopt -s nullglob
    for skill_dir in "$SKILLS_DIR"/rive-*/; do
        skill_file="$skill_dir/SKILL.md"
        if [ -f "$skill_file" ]; then
            skill_name=$(basename "$skill_dir")
            SKILL_NAMES+=("\"$skill_name\"")
            SKILLS_CONTEXT+="
--- SKILL: $skill_name ---
$(cat "$skill_file")
--- END SKILL ---
"
        fi
    done
    shopt -u nullglob
    if [ ${#SKILL_NAMES[@]} -gt 0 ]; then
        SKILLS_LIST="[$(IFS=,; echo "${SKILL_NAMES[*]}")]"
    fi
fi

# Load the scene schema for reference
SCHEMA_FILE="$PROJECT_ROOT/docs/scene.schema.v1.json"
SCHEMA_CONTEXT=""
if [ -f "$SCHEMA_FILE" ]; then
    SCHEMA_CONTEXT="
--- SCENE SCHEMA ---
$(cat "$SCHEMA_FILE")
--- END SCHEMA ---
"
fi

# Build the system prompt
SYSTEM_PROMPT="You are a Rive animation expert. Generate a valid SceneSpec JSON that creates the requested animation.

RULES:
- Output ONLY valid JSON, no markdown fences, no explanation
- Must include \"scene_format_version\": 1
- Must have at least one artboard with children
- All object types, property keys, and type keys must match the Rive runtime spec exactly
- State machines need entry(0), any(1), exit(2) states before animation states
- Parent IDs use 0-based indexing within the artboard's child list

$SKILLS_CONTEXT
$SCHEMA_CONTEXT"

USER_PROMPT="Create a Rive scene: $PROMPT

Target name: $TARGET_NAME
Complexity tier: $COMPLEXITY"

# Model definitions: name|cli_name|model_id|category
declare -a MODELS=(
    "codex|codex|gpt-5.4|deep"
    "opus|claude|opus|artistry"
)

echo "=== Race: $TARGET_NAME ==="
echo "Run ID: $RUN_ID"
echo "Complexity: $COMPLEXITY"
echo "Prompt: $PROMPT"
echo ""

ATTEMPTED_MODELS=0
SUCCESSFUL_MODELS=0
BEST_MODEL=""
BEST_OBJECTS=0
ATTEMPTED_MODEL_NAMES=""

for model_spec in "${MODELS[@]}"; do
    IFS='|' read -r MODEL_NAME CLI_NAME MODEL_ID CATEGORY <<< "$model_spec"

    if ! command -v "$CLI_NAME" &>/dev/null; then
        echo "SKIP $MODEL_NAME — '$CLI_NAME' CLI not found"
        continue
    fi

    ATTEMPTED_MODELS=$((ATTEMPTED_MODELS + 1))
    if [ -n "$ATTEMPTED_MODEL_NAMES" ]; then
        ATTEMPTED_MODEL_NAMES="$ATTEMPTED_MODEL_NAMES,$MODEL_NAME"
    else
        ATTEMPTED_MODEL_NAMES="$MODEL_NAME"
    fi

    JSON_OUT="$RESULTS_DIR/${MODEL_NAME}.json"

    echo "--- $MODEL_NAME ($MODEL_ID) ---"

    # Call the model via local CLI
    CALL_OK=0
    if [ "$CLI_NAME" = "codex" ]; then
        FULL_PROMPT="$SYSTEM_PROMPT

$USER_PROMPT"
        "$TIMEOUT_BIN" 300 codex exec \
            -m "$MODEL_ID" \
            --full-auto \
            -o "$JSON_OUT" \
            "$FULL_PROMPT" 2>/dev/null && CALL_OK=1
    elif [ "$CLI_NAME" = "claude" ]; then
        "$TIMEOUT_BIN" 300 claude -p \
            --model "$MODEL_ID" \
            --system-prompt "$SYSTEM_PROMPT" \
            --output-format text \
            --no-session-persistence \
            --allowedTools "" \
            --max-budget-usd 5 \
            "$USER_PROMPT" > "$JSON_OUT" 2>/dev/null && CALL_OK=1
    fi

    if [ "$CALL_OK" -eq 0 ]; then
        echo "  FAIL: API call failed for $MODEL_NAME"
        continue
    fi

    # Strip markdown fences if present (only first/last lines)
    if head -1 "$JSON_OUT" | grep -q '^```'; then
        TMP_STRIP="${JSON_OUT}.strip"
        tail -n +2 "$JSON_OUT" > "$TMP_STRIP" && mv "$TMP_STRIP" "$JSON_OUT"
    fi
    if tail -1 "$JSON_OUT" | grep -q '^```'; then
        TMP_STRIP="${JSON_OUT}.strip"
        head -n -1 "$JSON_OUT" > "$TMP_STRIP" && mv "$TMP_STRIP" "$JSON_OUT"
    fi

    # Validate JSON syntax
    if ! jq empty "$JSON_OUT" 2>/dev/null; then
        echo "  FAIL: Invalid JSON output"
        continue
    fi

    # Validate scene_format_version
    if ! jq -e '(.scene_format_version | type == "number") and (.scene_format_version == 1)' "$JSON_OUT" >/dev/null 2>&1; then
        echo "  FAIL: Missing or invalid scene_format_version (must be 1)"
        continue
    fi

    echo "  JSON saved to $JSON_OUT"

    # Run log_attempt.sh (handles generate, validate, inspect, ledger logging)
    cd "$PROJECT_ROOT"
    ATTEMPT_RESULT=$(bash data/log_attempt.sh "$RUN_ID" "$TARGET_NAME" "$MODEL_NAME-$CATEGORY" "$CATEGORY" "$SKILLS_LIST" "$COMPLEXITY" "$JSON_OUT" 0 2>&1) || true
    echo "  $ATTEMPT_RESULT"

    # Track best model by object count
    if echo "$ATTEMPT_RESULT" | grep -q "^OK"; then
        SUCCESSFUL_MODELS=$((SUCCESSFUL_MODELS + 1))
        OBJ_COUNT=$(echo "$ATTEMPT_RESULT" | sed -n 's/.*objects=\([0-9]*\).*/\1/p')
        OBJ_COUNT="${OBJ_COUNT:-0}"
        if [ "$OBJ_COUNT" -gt "$BEST_OBJECTS" ]; then
            BEST_OBJECTS=$OBJ_COUNT
            BEST_MODEL="$MODEL_NAME-$CATEGORY"
        fi
    fi

    echo ""
done

if [ "$ATTEMPTED_MODELS" -eq 0 ]; then
    echo "ERROR: No models were available. Install 'claude' and/or 'codex' CLI tools."
    exit 1
fi

# Log run summary with SQL escaping
if [ -n "$BEST_MODEL" ]; then
    BEST_MODEL_SQL="'$(sql_escape "$BEST_MODEL")'"
else
    BEST_MODEL_SQL="NULL"
fi

sqlite3 "$DB" "INSERT OR REPLACE INTO run_summaries (run_id, target_name, models_used, best_model, total_attempts, successful_attempts, notes) VALUES ('$(sql_escape "$RUN_ID")', '$(sql_escape "$TARGET_NAME")', '$(sql_escape "$ATTEMPTED_MODEL_NAMES")', $BEST_MODEL_SQL, $ATTEMPTED_MODELS, $SUCCESSFUL_MODELS, 'Automated race via scripts/run_race.sh');"

echo "=== Race Summary ==="
echo "Target: $TARGET_NAME"
echo "Models: $ATTEMPTED_MODELS attempted, $SUCCESSFUL_MODELS successful"
if [ -n "$BEST_MODEL" ]; then
    echo "Best: $BEST_MODEL ($BEST_OBJECTS objects)"
    echo ""
    echo "Winner JSON: $RESULTS_DIR/$(echo "$BEST_MODEL" | cut -d- -f1).json"
    echo "To commit as fixture: cp $RESULTS_DIR/$(echo "$BEST_MODEL" | cut -d- -f1).json tests/fixtures/${TARGET_NAME}.json"
else
    echo "No successful outputs. Check CLI tool output and skill content."
fi
echo ""
echo "Ledger: sqlite3 $DB \"SELECT * FROM attempts WHERE run_id='$RUN_ID';\""
echo "Results: $RESULTS_DIR/"
