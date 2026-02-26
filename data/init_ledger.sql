-- Rive Creation Ledger Schema
-- Tracks all fixture generation attempts across models/skills for pattern mining.

CREATE TABLE IF NOT EXISTS attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id TEXT NOT NULL,
    target_name TEXT NOT NULL,
    model TEXT NOT NULL,
    category TEXT NOT NULL,
    skills_loaded TEXT NOT NULL,
    attempt_num INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),

    prompt_text TEXT,
    prompt_hash TEXT,
    output_json_path TEXT,
    output_riv_path TEXT,

    generate_ok INTEGER,
    validate_ok INTEGER,
    inspect_object_count INTEGER,

    error_stage TEXT CHECK(error_stage IN ('schema', 'build', 'validation', 'encoding')),
    error_message TEXT,
    fix_applied TEXT,

    action_type TEXT NOT NULL CHECK(action_type IN ('create', 'repair', 'retry')),
    complexity_tier TEXT NOT NULL CHECK(complexity_tier IN ('static', 'animated', 'interactive')),
    object_types_used TEXT,

    success INTEGER NOT NULL DEFAULT 0,
    duration_ms INTEGER
);

CREATE TABLE IF NOT EXISTS skill_updates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_name TEXT NOT NULL,
    update_type TEXT NOT NULL CHECK(update_type IN ('add_pattern', 'add_anti_pattern', 'fix_example', 'restructure', 'consolidate')),
    trigger_attempt_id INTEGER REFERENCES attempts(id),
    description TEXT NOT NULL,
    diff_summary TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS run_summaries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id TEXT NOT NULL UNIQUE,
    target_name TEXT NOT NULL,
    models_used TEXT NOT NULL,
    best_model TEXT,
    total_attempts INTEGER NOT NULL DEFAULT 0,
    successful_attempts INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- Pattern mining views

CREATE VIEW IF NOT EXISTS error_frequency AS
SELECT
    error_stage,
    error_message,
    model,
    COUNT(*) as occurrences,
    SUM(CASE WHEN fix_applied IS NOT NULL THEN 1 ELSE 0 END) as auto_fixed,
    GROUP_CONCAT(DISTINCT target_name) as affected_targets
FROM attempts
WHERE success = 0 AND error_message IS NOT NULL
GROUP BY error_stage, error_message, model
ORDER BY occurrences DESC;

CREATE VIEW IF NOT EXISTS model_scorecard AS
SELECT
    model,
    COUNT(*) as total_attempts,
    SUM(success) as successes,
    ROUND(100.0 * SUM(success) / COUNT(*), 1) as success_pct,
    ROUND(AVG(attempt_num), 2) as avg_retries,
    ROUND(AVG(duration_ms), 0) as avg_duration_ms,
    GROUP_CONCAT(DISTINCT complexity_tier) as tiers_attempted
FROM attempts
GROUP BY model
ORDER BY success_pct DESC;

CREATE VIEW IF NOT EXISTS target_difficulty AS
SELECT
    target_name,
    complexity_tier,
    COUNT(*) as total_attempts,
    SUM(success) as successes,
    ROUND(100.0 * SUM(success) / COUNT(*), 1) as success_pct,
    GROUP_CONCAT(DISTINCT error_stage) as error_stages,
    GROUP_CONCAT(DISTINCT model) as models_tried
FROM attempts
GROUP BY target_name
ORDER BY success_pct ASC;

CREATE VIEW IF NOT EXISTS skill_impact AS
SELECT
    su.skill_name,
    su.update_type,
    su.description,
    a.error_stage as triggered_by_error_stage,
    a.error_message as triggered_by_error,
    a.model as triggered_by_model,
    su.created_at
FROM skill_updates su
LEFT JOIN attempts a ON su.trigger_attempt_id = a.id
ORDER BY su.created_at DESC;
