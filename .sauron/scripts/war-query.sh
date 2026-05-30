#!/usr/bin/env bash
# war-query.sh — Instant query interface for per-campaign war-state JSONL event logs
#
# Usage:
#   war-query.sh [-f <logfile>] [--json] [<campaign>] <command> [args]
#
# Commands:
#   phase              Current active phase
#   deployed           Active force assignments (dispatched minus completed/failed)
#   blockers           Unresolved blockers
#   unverified         Completed but unverified tasks
#   gate [phase]       Gate status for phase (defaults to current phase)
#   task <id>          Latest event for a specific task
#   decisions          All decision events
#   dashboard          Full campaign overview
#   health             Counts by event type
#   log [n]            Show last n events (default 10)
#
# Options:
#   -f, --log <path>   Path to war-log.jsonl (default: .sauron/state/campaigns/<campaign>/war-log.jsonl)
#   --json             Emit JSON for machine consumers
#   --help             Show this help
#
# Dependencies: jq, bash 4+, tac (or tail -r on macOS)

set -euo pipefail

_show_help() { awk 'NR==1{next} /^#/{sub(/^# ?/,"");print;next} {exit}' "$0"; }
if command -v tac &>/dev/null; then _tac() { tac "$@"; }; else _tac() { tail -r "$@"; }; fi

LOG_FILE=""
JSON_OUTPUT=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    -f|--log) LOG_FILE="$2"; shift 2 ;;
    --json) JSON_OUTPUT=1; shift ;;
    --help|-h) _show_help; exit 0 ;;
    *) break ;;
  esac
done

KNOWN_COMMANDS=" phase deployed blockers unverified gate task decisions dashboard health log help --help -h "
CAMPAIGN=""
if [[ $# -ge 2 && "$KNOWN_COMMANDS" != *" ${1:-} "* ]]; then
  CAMPAIGN="$1"
  shift
fi

CMD="${1:-help}"; shift || true

command -v jq &>/dev/null || { echo "error: jq is required but not found" &>2; exit 1; }

if [[ -z "$LOG_FILE" && -n "$CAMPAIGN" ]]; then
  LOG_FILE="$(git rev-parse --show-toplevel 2>/dev/null || echo .)/.sauron/state/campaigns/$CAMPAIGN/war-log.jsonl"
fi

_require_log() {
  [[ -f "$LOG_FILE" ]] || { echo "No war-log found at $LOG_FILE" &>2; exit 1; }
  [[ -s "$LOG_FILE" ]] || { echo "War-log is empty at $LOG_FILE" &>2; exit 1; }
}

_campaign_filter() {
  if [[ -n "$CAMPAIGN" ]]; then
    jq -r -c --arg campaign "$CAMPAIGN" 'select(.campaign == $campaign)'
  else
    cat
  fi
}
_events() { jq -Rrc 'fromjson? // empty' "$LOG_FILE" | _campaign_filter; }
_tac_events() { _tac "$LOG_FILE" | jq -Rrc 'fromjson? // empty' | _campaign_filter; }

_json_string() { jq -Rn --arg value "$1" '$value'; }
_json_array_lines() { jq -R -s 'split("\n") | map(select(length > 0))'; }

_current_phase() {
  local last_started phase_id last_completed started_ts completed_ts
  last_started=$(_tac_events | jq -r -c 'select(.event=="phase_started")' | head -1)
  [[ -z "$last_started" ]] && return
  phase_id=$(echo "$last_started" | jq -r '.phase')
  last_completed=$(_tac_events | jq -r -c "select((.event==\"phase_completed\" or .event==\"phase_skipped\") and .phase==\"$phase_id\")" | head -1)
  started_ts=$(echo "$last_started" | jq -r '.ts')
  if [[ -n "$last_completed" ]]; then
    completed_ts=$(echo "$last_completed" | jq -r '.ts')
    [[ "$completed_ts" > "$started_ts" ]] && return
  fi
  echo "$phase_id"
}

_current_phase_name() {
  local l; l=$(_tac_events | jq -r -c 'select(.event=="phase_started")' | head -1)
  [[ -n "$l" ]] && echo "$l" | jq -r '.name // .phase'
}

_campaign_name() { _events | jq -r -c 'select(.event=="campaign_started")' | tail -1 | jq -r '.campaign // "unknown"'; }

_gate_eval() {
  local phase="$1" verbose="${2:-}" gate_def gate_cleared
  gate_def=$(_events | jq -r -c "select(.event==\"gate_defined\" and .phase==\"$phase\")" | tail -1)
  [[ -z "$gate_def" ]] && { echo "No gate defined for phase: $phase"; return; }
  gate_cleared=$(_events | jq -r -c "select(.event==\"gate_cleared\" and .phase==\"$phase\")" | tail -1)
  if [[ -n "$gate_cleared" ]]; then
    local gate_name; gate_name=$(echo "$gate_def" | jq -r '.name // "unnamed"')
    echo "Gate '$gate_name' — CLEARED"; return
  fi
  local gate_name total_conds task_count verified_count blocker_count met=0
  gate_name=$(echo "$gate_def" | jq -r '.name // "unnamed"')
  total_conds=$(echo "$gate_def" | jq -r '.conditions | length')
  task_count=$(_events | jq -r -s "[.[] | select(.event==\"task_created\" and .phase==\"$phase\")] | length")
  verified_count=$(_events | jq -r -s --arg phase "$phase" '
    ([.[] | select(.event=="task_created" and .phase==$phase)] | map(.task)) as $phase_tasks |
    [.[] | select(.task and (.task | IN($phase_tasks[])) and (.event | test("^task_(created|dispatched|completed|verified|failed|blocked|unblocked)$")))]
    | group_by(.task)
    | map(sort_by(.ts) | last | select(.event=="task_verified"))
    | length
  ')
  blocker_count=$(_events | jq -r -s '[.[] | select(.event | test("^blocker_(added|resolved)$"))] | group_by(.id) | map(sort_by(.ts) | last | select(.event=="blocker_added")) | length')
  if [[ "$verbose" == "verbose" ]]; then
    echo "Gate '$gate_name' for phase '$phase':"
    echo "$gate_def" | jq -r '.conditions[]' | while read -r cond; do echo "  [ ] $cond"; done
  fi
  local -a conds=()
  while IFS= read -r cond; do conds+=("$cond"); done <<<$(echo "$gate_def" | jq -r '.conditions[]')
  for cond in "${conds[@]}"; do
    case "$cond" in
      *"all tasks verified"*)
        if [[ "$verified_count" -ge "$task_count" && "$task_count" -gt 0 ]]; then
          met=$((met + 1))
          [[ "$verbose" == "verbose" ]] && echo "  -> MET: $cond ($verified_count/$task_count verified)" || true
        else
          [[ "$verbose" == "verbose" ]] && echo "  -> NOT MET: $cond ($verified_count/$task_count verified)" || true
        fi ;;
      *"no blockers"*)
        if [[ "$blocker_count" -eq 0 ]]; then
          met=$((met + 1))
          [[ "$verbose" == "verbose" ]] && echo "  -> MET: $cond" || true
        else
          [[ "$verbose" == "verbose" ]] && echo "  -> NOT MET: $cond ($blocker_count active)" || true
        fi ;;
      *)
        [[ "$verbose" == "verbose" ]] && echo "  -> UNKNOWN: $cond" || true ;;
    esac
  done
  if [[ "$verbose" != "verbose" ]]; then
    if [[ "$met" -eq "$total_conds" && "$total_conds" -gt 0 ]]; then
      echo "Gate: ${met}/${total_conds} conditions met — ALL MET — awaiting gate_cleared"
    else
      echo "Gate: ${met}/${total_conds} conditions met — NOT CLEAR"
    fi
  fi
}

cmd_phase() {
  _require_log
  local p; p=$(_current_phase)
  if [[ "$JSON_OUTPUT" -eq 1 ]]; then
    jq -n --arg phase "$p" '{phase: ($phase | select(length > 0) // null)}'
  else
    echo "${p:-No active phase}"
  fi
}

cmd_deployed() {
  _require_log
  local output
  output=$(_events | jq -r -s '
    [.[] | select(.event | test("^task_(dispatched|completed|failed|blocked|unblocked|verified)$"))] |
    group_by(.task) | map(sort_by(.ts) | last | select(.event=="task_dispatched")) |
    .[] | "\(.task):\(.agent // .force // "unknown")"
  ')
  if [[ "$JSON_OUTPUT" -eq 1 ]]; then
    echo "$output" | _json_array_lines | jq '{deployed: .}'
  else
    echo "$output" | tr '\n' ' '
    echo
  fi
}

cmd_blockers() {
  _require_log
  _events | jq -r -s '
    [.[] | select(.event | test("^blocker_(added|resolved)$"))] |
    group_by(.id) | map(sort_by(.ts) | last | select(.event=="blocker_added")) |
    if length == 0 then "none"
    else .[] | "\(.id): \(.blocker) [blocks: \(.blocks | join(", "))]" end
  '
}

cmd_unverified() {
  _require_log
  if [[ "$JSON_OUTPUT" -eq 1 ]]; then
    _events | jq -r -s '
      [.[] | select(.event | test("^task_(completed|verified|failed)$"))] |
      group_by(.task) | map(sort_by(.ts) | last | select(.event=="task_completed")) |
      map(.task)
    ' | jq '{unverified: .}'
    return
  fi
  _events | jq -r -s '
    [.[] | select(.event | test("^task_(completed|verified|failed)$"))] |
    group_by(.task) | map(sort_by(.ts) | last | select(.event=="task_completed")) |
    if length == 0 then "none" else .[] | "\(.task)" end
  '
}

cmd_gate() {
  _require_log
  local phase="${1:-}"
  [[ -z "$phase" ]] && phase=$(_current_phase)
  [[ -z "$phase" ]] && { echo "No active phase and no phase specified"; return 1; }
  if [[ "$JSON_OUTPUT" -eq 1 ]]; then
    local satisfied_event status
    satisfied_event=$(_events | jq -r -c --arg phase "$phase" 'select((.event=="phase_completed" or .event=="phase_skipped") and .phase==$phase)' | tail -1)
    [[ -n "$satisfied_event" ]] && status="closed" || status="open"
    jq -n --arg phase "$phase" --arg status "$status" '{phase: $phase, status: $status, satisfied: ($status == "closed")}'
    return
  fi
  _gate_eval "$phase" verbose
}

cmd_task() {
  local task_id="${1:-}"
  [[ -z "$task_id" ]] && { echo "Usage: war-query.sh task <id>" &>2; return 1; }
  _require_log
  local r; r=$(_tac_events | jq -r -c "select(.task==\"$task_id\")" | head -1)
  if [[ -z "$r" ]]; then
    if [[ "$JSON_OUTPUT" -eq 1 ]]; then jq -n --arg task "$task_id" '{task: $task, event: null}'; else echo "No events found for task: $task_id"; fi
    return
  fi
  if [[ "$JSON_OUTPUT" -eq 1 ]]; then echo "$r" | jq .; else echo "$r" | jq .; fi
}

cmd_decisions() {
  _require_log
  local c; c=$(_events | jq -r -c 'select(.event=="decision")' | wc -l | tr -d ' ')
  [[ "$c" -eq 0 ]] && { echo "No decisions recorded"; return; }
  _events | jq -r 'select(.event=="decision") | "[\(.ts)] \(.decision)"'
}

cmd_dashboard() {
  _require_log
  local campaign phase phase_name total_phases idx
  campaign=$(_campaign_name); phase=$(_current_phase); phase_name=$(_current_phase_name)
  total_phases=$(_events | jq -r -s '[.[] | select(.event=="campaign_started")] | last | .phases | length // 0')
  if [[ "$total_phases" -gt 0 && -n "$phase" ]]; then
    idx=$(_events | jq -r -s "[.[] | select(.event==\"campaign_started\")] | last | .phases | to_entries[] | select(.value==\"$phase\") | .key + 1")
  else idx="?"; fi
  echo "Campaign: $campaign | Phase: ${phase_name:-none} (${idx}/${total_phases})"

  _events | jq -r -s '
    ([.[] | select(.event=="task_created")] | map({(.task): {name:.name, phase:.phase}}) | add // {}) as $cr |
    ([.[] | select(.event=="task_dispatched")] | map({(.task): {agent:.agent, force:.force}}) | add // {}) as $dp |
    ([.[] | select(.event=="task_blocked")] | map({(.task): .blocked_by}) | add // {}) as $bl |
    [.[] | select(.event | test("^task_(created|dispatched|completed|verified|failed|blocked|unblocked)$"))] |
    group_by(.task) | map(sort_by(.ts)) |
    map({task:.[0].task, latest:(.[-1].event)}) | sort_by(.task) | .[] |
    ($cr[.task].name // "") as $n | ($dp[.task].agent // "") as $a | ($bl[.task] // "") as $b |
    (if .latest=="task_verified" then "verified"
     elif .latest=="task_completed" then "completed"
     elif .latest=="task_failed" then "failed"
     elif .latest=="task_blocked" then "blocked"
     elif .latest=="task_dispatched" then "active"
     else "pending" end) as $s |
    "  [\($s)]  \(.task) \($n)" +
    (if $a!="" then " (\($a))" else "" end) +
    (if $s=="completed" then " ** UNVERIFIED **" else "" end) +
    (if $s=="blocked" then " (blocked by \($b))" else "" end)
  '

  local blockers; blockers=$(_events | jq -r -s '
    [.[] | select(.event | test("^blocker_(added|resolved)$"))] |
    group_by(.id) | map(sort_by(.ts) | last | select(.event=="blocker_added")) |
    if length==0 then "none" else map("\(.id): \(.blocker)") | join(", ") end
  ')
  echo "Blockers: $blockers"

  local dc; dc=$(_events | jq -r -s '[.[] | select(.event=="decision")] | length')
  echo "Decisions: $dc recorded"

  [[ -n "$phase" ]] && _gate_eval "$phase" ""
  true
}

cmd_health() { _require_log; _events | jq -r '.event' | sort | uniq -c | sort -rn; }

cmd_log() { _require_log; local n="${1:-10}"; _tac_events | head -"$n" | jq -c .; }

case "$CMD" in
  phase)      cmd_phase ;;
  deployed)   cmd_deployed ;;
  blockers)   cmd_blockers ;;
  unverified) cmd_unverified ;;
  gate)       cmd_gate "$@" ;;
  task)       cmd_task "$@" ;;
  decisions)  cmd_decisions ;;
  dashboard)  cmd_dashboard ;;
  health)     cmd_health ;;
  log)        cmd_log "$@" ;;
  help|--help|-h) _show_help ;;
  *) echo "Unknown command: $CMD" &>2; echo "Run with --help for usage" &>2; exit 1 ;;
esac
