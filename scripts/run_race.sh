#!/usr/bin/env bash
# Usage: ./scripts/run_race.sh <target_name> <prompt_description> [complexity]
#
# Runs a multi-model race for a Rive fixture target.
# Each configured model generates scene JSON from the prompt, then the output
# is validated, logged to the SQLite ledger, and persisted to docs/race-results/.
#
# Prerequisites:
#   - At least one model API key set (see Model Configuration below)
#   - jq, curl, sqlite3, python3 installed
#   - cargo build passes
#
# Model Configuration (via environment variables):
#   OPENAI_API_KEY     — enables Codex model (gpt-4.1)
#   GEMINI_API_KEY     — enables Gemini model (gemini-2.5-pro)
#   ANTHROPIC_API_KEY  — enables Opus model (claude-opus-4-6)
#
# Models without API keys are skipped with a warning.
#
# Examples:
#   ./scripts/run_race.sh loader "spinning loading indicator with smooth rotation"
#   ./scripts/run_race.sh icon_set "3 simple icons: home, settings, profile" static
#   OPENAI_API_KEY=sk-... ./scripts/run_race.sh game_hud "health bar and score counter" animated

set -euo pipefail

# Check required tools
for cmd in jq curl sqlite3 python3; do
    if ! command -v "$cmd" &>/dev/null; then
        echo "ERROR: Required tool '$cmd' not found. Please install it."
        exit 1
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
RUN_ID="race_$(date +%s)_${TARGET_NAME}"
RESULTS_DIR="$PROJECT_ROOT/docs/race-results/$TARGET_NAME/$RUN_ID"

# Ensure ledger exists
if [ ! -f "$DB" ]; then
    echo "Initializing ledger..."
    sqlite3 "$DB" < "$PROJECT_ROOT/data/init_ledger.sql"
fi

mkdir -p "$RESULTS_DIR"

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

# Model definitions: name|api_key_env|endpoint|model_id|category
declare -a MODELS=(
    "codex|OPENAI_API_KEY|https://api.openai.com/v1/chat/completions|gpt-4.1|deep"
    "gemini|GEMINI_API_KEY|https://generativelanguage.googleapis.com/v1beta/openai/chat/completions|gemini-2.5-pro|artistry"
    "opus|ANTHROPIC_API_KEY|https://api.anthropic.com/v1/messages|claude-opus-4-6|unspecified-high"
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
    IFS='|' read -r MODEL_NAME API_KEY_ENV ENDPOINT MODEL_ID CATEGORY <<< "$model_spec"
    API_KEY="${!API_KEY_ENV:-}"

    if [ -z "$API_KEY" ]; then
        echo "SKIP $MODEL_NAME — $API_KEY_ENV not set"
        continue
    fi

    JSON_OUT="$RESULTS_DIR/${MODEL_NAME}.json"

    echo "--- $MODEL_NAME ($MODEL_ID) ---"

    # Call the model API
    CALL_OK=0
    if [ "$MODEL_NAME" = "opus" ]; then
        # Anthropic uses a different API format
        RESPONSE=$(curl -s --connect-timeout 30 --max-time 300 -w "\n%{http_code}" "$ENDPOINT" \
            -H "Content-Type: application/json" \
            -H "x-api-key: $API_KEY" \
            -H "anthropic-version: 2023-06-01" \
            -d "$(jq -n \
                --arg model "$MODEL_ID" \
                --arg system "$SYSTEM_PROMPT" \
                --arg user "$USER_PROMPT" \
                '{model: $model, max_tokens: 8192, system: $system, messages: [{role: "user", content: $user}]}')" \
            2>/dev/null) || true

        HTTP_CODE=$(echo "$RESPONSE" | tail -1)
        BODY=$(echo "$RESPONSE" | sed '$d')

        if [ "$HTTP_CODE" = "200" ]; then
            # Extract text from Anthropic response
            echo "$BODY" | jq -r '.content[0].text' > "$JSON_OUT" 2>/dev/null && CALL_OK=1
        else
            echo "  API error (HTTP $HTTP_CODE)"
            echo "$BODY" | head -3
        fi
    else
        # OpenAI-compatible API (Codex, Gemini)
        RESPONSE=$(curl -s --connect-timeout 30 --max-time 300 -w "\n%{http_code}" "$ENDPOINT" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $API_KEY" \
            -d "$(jq -n \
                --arg model "$MODEL_ID" \
                --arg system "$SYSTEM_PROMPT" \
                --arg user "$USER_PROMPT" \
                '{model: $model, max_tokens: 8192, messages: [{role: "system", content: $system}, {role: "user", content: $user}]}')" \
            2>/dev/null) || true

        HTTP_CODE=$(echo "$RESPONSE" | tail -1)
        BODY=$(echo "$RESPONSE" | sed '$d')

        if [ "$HTTP_CODE" = "200" ]; then
            echo "$BODY" | jq -r '.choices[0].message.content' > "$JSON_OUT" 2>/dev/null && CALL_OK=1
        else
            echo "  API error (HTTP $HTTP_CODE)"
            echo "$BODY" | head -3
        fi
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

    ATTEMPTED_MODELS=$((ATTEMPTED_MODELS + 1))
    if [ -n "$ATTEMPTED_MODEL_NAMES" ]; then
        ATTEMPTED_MODEL_NAMES="$ATTEMPTED_MODEL_NAMES,$MODEL_NAME"
    else
        ATTEMPTED_MODEL_NAMES="$MODEL_NAME"
    fi

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
    echo "ERROR: No models produced valid output. Check API keys and responses."
    echo "Set at least one of: OPENAI_API_KEY, GEMINI_API_KEY, ANTHROPIC_API_KEY"
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
    echo "No successful outputs. Check API responses and skill content."
fi
echo ""
echo "Ledger: sqlite3 $DB \"SELECT * FROM attempts WHERE run_id='$RUN_ID';\""
echo "Results: $RESULTS_DIR/"
