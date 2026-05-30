// Vendored from mordor-forge/ide-of-sauron/scripts/lib/war-state-schema.mjs
// Adapted for rive-rs-cli per-campaign war-logs.

export const CANONICAL_EVENT_TYPES = [
  'task_dispatched',
  'task_completed',
  'task_failed',
  'task_verified',
  'task_created',
  'task_blocked',
  'task_unblocked',
  'phase_started',
  'phase_completed',
  'phase_skipped',
  'state_transition',
  'forge_partial',
  'siege_partial',
  'sub_phase_started',
  'sub_phase_completed',
  'dependency_discovered',
  'baseline_verified',
  'baseline_recorded',
  'baseline_skipped',
  'siege_tick',
  'verification',
  'compaction_marker',
  'campaign_started',
  'campaign_completed',
  'campaign_abandoned',
  'gate_defined',
  'gate_cleared',
  'blocker_added',
  'blocker_resolved',
  'decision',
  'watcher_verdict_recorded',
  'quality_gate_executed',
  'evaluation_outcome',
  'template_deviation',
];

export const CANONICAL_EVENT_TYPE_SET = new Set(CANONICAL_EVENT_TYPES);

const REQUIRED_BY_EVENT = {
  phase_skipped: ['phase', 'reason'],
  dependency_discovered: ['task_id', 'dependency', 'impact'],
  state_transition: ['to_state'],
  forge_partial: ['phase', 'partial_state', 'branch', 'pr'],
  siege_partial: ['phase', 'round', 'branch', 'pr'],
  sub_phase_started: ['parent_phase', 'sub_phase'],
  sub_phase_completed: ['parent_phase', 'sub_phase'],
  watcher_verdict_recorded: ['task_id', 'verdict'],
  quality_gate_executed: ['phase', 'gate_name', 'outcome'],
  evaluation_outcome: ['task_id', 'watcher_verdict', 'actual_outcome', 'outcome_source'],
  template_deviation: ['phase', 'expected_template'],
};

const DEPENDENCY_IMPACTS = new Set(['blocks', 'delays', 'noted']);

export function isIsoTimestamp(value) {
  return typeof value === 'string' && !Number.isNaN(Date.parse(value));
}

export function validateWarEvent(event) {
  if (!event || typeof event !== 'object' || Array.isArray(event)) {
    return { ok: false, reason: 'event is not a JSON object' };
  }

  for (const field of ['ts', 'event', 'campaign']) {
    if (event[field] == null || event[field] === '') {
      return { ok: false, reason: `missing field ${field}` };
    }
  }

  if (!isIsoTimestamp(event.ts)) {
    return { ok: false, reason: `invalid ts: ${event.ts}` };
  }

  if (!CANONICAL_EVENT_TYPE_SET.has(event.event)) {
    return { ok: false, reason: `unknown event type: ${event.event}` };
  }

  for (const field of REQUIRED_BY_EVENT[event.event] || []) {
    if (event[field] == null || event[field] === '') {
      return { ok: false, reason: `missing field ${field}` };
    }
  }

  if (event.event === 'dependency_discovered' && !DEPENDENCY_IMPACTS.has(event.impact)) {
    return { ok: false, reason: `invalid impact: ${event.impact}` };
  }

  if (event.event === 'state_transition') {
    const validVerdicts = new Set(['PASS', 'FAIL', 'CONTINUE', 'BLOCKED', 'DONE', 'DONE_WITH_CONCERNS']);
    if (event.verdict != null && !validVerdicts.has(event.verdict)) {
      return { ok: false, reason: `invalid verdict: ${event.verdict}` };
    }
    if (event.from_state != null && typeof event.from_state !== 'string') {
      return { ok: false, reason: 'invalid from_state' };
    }
    if (!event.to_state || typeof event.to_state !== 'string') {
      return { ok: false, reason: 'missing or invalid to_state' };
    }
  }

  if (event.event === 'forge_partial') {
    if (event.partial_state !== 'push_complete') {
      return { ok: false, reason: `invalid partial_state: ${event.partial_state}` };
    }
    if (event.pr !== null) {
      return { ok: false, reason: 'forge_partial pr must be null' };
    }
  }

  if (event.event === 'siege_partial') {
    if (!Number.isInteger(event.round) || event.round < 1) {
      return { ok: false, reason: `invalid round: ${event.round}` };
    }
    if (!Number.isInteger(event.pr) || event.pr < 1) {
      return { ok: false, reason: `invalid pr: ${event.pr}` };
    }
  }

  if (event.event === 'sub_phase_started' || event.event === 'sub_phase_completed') {
    const validSubPhases = new Set(['4a', '4b', '4c', '4d']);
    if (!validSubPhases.has(event.sub_phase)) {
      return { ok: false, reason: `invalid sub_phase: ${event.sub_phase}` };
    }
    if (event.parent_phase !== 'fp-watchers') {
      return { ok: false, reason: `invalid parent_phase: ${event.parent_phase}` };
    }
  }

  if (event.event === 'watcher_verdict_recorded') {
    const validVerdicts = new Set(['PASS', 'FAIL', 'CONTINUE', 'BLOCKED']);
    if (!validVerdicts.has(event.verdict)) {
      return { ok: false, reason: `invalid verdict: ${event.verdict}` };
    }
  }

  if (event.event === 'quality_gate_executed') {
    const validOutcomes = new Set(['pass', 'fail', 'skip', 'error']);
    if (!validOutcomes.has(event.outcome)) {
      return { ok: false, reason: `invalid outcome: ${event.outcome}` };
    }
  }

  if (event.event === 'evaluation_outcome') {
    if (event.divergence_type != null) {
      const validDivergences = new Set(['false_positive', 'false_negative', 'true_positive', 'true_negative']);
      if (!validDivergences.has(event.divergence_type)) {
        return { ok: false, reason: `invalid divergence_type: ${event.divergence_type}` };
      }
    }
  }

  if (event.event === 'campaign_started') {
    const validTemplates = new Set(['full', 'standard', 'hotfix', 'goblin']);
    if (event.template != null && !validTemplates.has(event.template)) {
      return { ok: false, reason: `invalid template: ${event.template}` };
    }
  }

  if (event.event === 'template_deviation') {
    const validTemplates = new Set(['full', 'standard', 'hotfix', 'goblin']);
    if (!validTemplates.has(event.expected_template)) {
      return { ok: false, reason: `invalid expected_template: ${event.expected_template}` };
    }
    if (typeof event.phase !== 'string') {
      return { ok: false, reason: 'invalid phase' };
    }
  }

  return { ok: true };
}

export function validationWarning(line, reason) {
  return { line, reason };
}
