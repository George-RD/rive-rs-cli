#!/usr/bin/env node
/** materialize-state.mjs — Replay war-log.jsonl into structured war-state JSON. */
import { createReadStream } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { createInterface } from 'node:readline';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { validateWarEvent, validationWarning } from './lib/war-state-schema.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, '..', '..');
const args = process.argv.slice(2);

function usage() {
  return `materialize-state.mjs — Replay war-log JSONL into structured war-state JSON

Usage: node materialize-state.mjs [<input>] [--out <output>] [--dry-run] [--campaign <slug>] [--validate-stronghold <path>]

  <input>                       Path to war-log.jsonl (default: .sauron/state/campaigns/<slug>/war-log.jsonl)
  --out <path>                  Output path (default: .sauron/state/campaigns/<slug>/war-state.json)
  --dry-run                     Print JSON to stdout instead of writing file
  --campaign <slug>             Return state for one campaign
  --validate-stronghold <path>  Compare campaign phase against a stronghold file
  --help, -h                    Show this help`;
}

if (args.includes('--help') || args.includes('-h')) {
  console.log(usage());
  process.exit(0);
}

function optionValue(name) {
  const index = args.indexOf(name);
  return index === -1 ? null : args[index + 1];
}

const dryRun = args.includes('--dry-run');
const campaignFilter = optionValue('--campaign');

function defaultInputPath() {
  if (campaignFilter) return resolve(ROOT, '.sauron', 'state', 'campaigns', campaignFilter, 'war-log.jsonl');
  return resolve(ROOT, '.sauron', 'state', 'campaigns', 'default', 'war-log.jsonl');
}

function defaultOutputPath() {
  if (campaignFilter) return resolve(ROOT, '.sauron', 'state', 'campaigns', campaignFilter, 'war-state.json');
  return resolve(ROOT, '.sauron', 'state', 'campaigns', 'default', 'war-state.json');
}

const outputPath = optionValue('--out') ? resolve(optionValue('--out')) : defaultOutputPath();
const strongholdPath = optionValue('--validate-stronghold');
const optionNames = new Set(['--dry-run', '--out', '--campaign', '--validate-stronghold']);
const optionValueIndexes = new Set();
for (const option of ['--out', '--campaign', '--validate-stronghold']) {
  const index = args.indexOf(option);
  if (index !== -1) optionValueIndexes.add(index + 1);
}
const positional = args.filter((arg, index) => !optionNames.has(arg) && !optionValueIndexes.has(index));
const inputPath = resolve(positional[0] || defaultInputPath());
const warnings = [];

function createState(campaign) {
  return {
    campaign,
    status: null,
    current_phase: null,
    currentPhase: null,
    created: null,
    updated: null,
    phases: [],
    blockers: [],
    decisions: [],
    discoveries: [],
    activeTasks: [],
    completedTasks: [],
    unverifiedTasks: [],
    gateStatus: {},
    watcherVerdicts: [],
    qualityGates: [],
    evaluations: [],
    templateDeviations: [],
  };
}

function internals(state) {
  if (!state._phaseMap) {
    Object.defineProperties(state, {
      _phaseMap: { value: new Map(), enumerable: false },
      _taskMap: { value: new Map(), enumerable: false },
      _blockerMap: { value: new Map(), enumerable: false },
    });
  }
  return state;
}

function getOrCreatePhase(state, id) {
  internals(state);
  const phaseId = id || '_unknown';
  if (state._phaseMap.has(phaseId)) return state._phaseMap.get(phaseId);
  const phase = { id: phaseId, name: phaseId, status: 'pending', tasks: [] };
  state._phaseMap.set(phaseId, phase);
  state.phases.push(phase);
  return phase;
}

function getOrCreateTask(state, id, phaseId) {
  internals(state);
  const taskId = id || '_unknown';
  if (state._taskMap.has(taskId)) return state._taskMap.get(taskId);
  const phase = getOrCreatePhase(state, phaseId);
  const task = { id: taskId, task_id: taskId, name: taskId, status: 'pending' };
  state._taskMap.set(taskId, task);
  phase.tasks.push(task);
  return task;
}

function setCurrentPhase(state, phase) {
  state.current_phase = phase;
  state.currentPhase = phase;
}

function touch(state, ts) {
  if (ts) state.updated = ts;
}

function setTaskStatus(task, status) {
  task.status = status;
  if (status === 'completed') {
    task.verified = false;
    delete task.verified_by;
  } else if (status === 'verified') {
    task.verified = true;
  } else {
    delete task.verified;
    delete task.verified_by;
  }
  if (status !== 'failed') {
    delete task.fail_reason;
    delete task.attempts;
  }
  if (status !== 'blocked') delete task.blocked_by;
}

function taskId(ev) {
  return ev.task || ev.task_id;
}

const handlers = {
  campaign_started(state, ev) {
    state.status = 'active';
    state.created = ev.ts;
    state.template = ev.template || 'standard';
    if (Array.isArray(ev.phases)) for (const phase of ev.phases) getOrCreatePhase(state, phase);
    touch(state, ev.ts);
  },
  template_deviation(state, ev) {
    if (!state.templateDeviations) state.templateDeviations = [];
    state.templateDeviations.push({
      phase: ev.phase,
      expected_template: ev.expected_template,
      at: ev.ts,
    });
    touch(state, ev.ts);
  },
  campaign_completed(state, ev) {
    state.status = 'completed';
    touch(state, ev.ts);
  },
  campaign_abandoned(state, ev) {
    state.status = 'abandoned';
    if (ev.reason) state.abandon_reason = ev.reason;
    touch(state, ev.ts);
  },
  phase_started(state, ev) {
    const phase = getOrCreatePhase(state, ev.phase);
    phase.status = 'active';
    if (ev.name) phase.name = ev.name;
    setCurrentPhase(state, ev.phase);
    touch(state, ev.ts);
  },
  phase_completed(state, ev) {
    getOrCreatePhase(state, ev.phase).status = 'completed';
    if (state.current_phase === ev.phase) setCurrentPhase(state, null);
    state.gateStatus[ev.phase] = 'closed';
    touch(state, ev.ts);
  },
  phase_skipped(state, ev) {
    const phase = getOrCreatePhase(state, ev.phase);
    phase.status = 'skipped';
    phase.skip_reason = ev.reason;
    if (state.current_phase === ev.phase) setCurrentPhase(state, null);
    state.gateStatus[ev.phase] = 'closed';
    touch(state, ev.ts);
  },
  gate_defined(state, ev) {
    getOrCreatePhase(state, ev.phase).gate = {
      name: ev.name || 'unnamed',
      cleared: false,
      conditions: ev.conditions || [],
      met: [],
    };
    state.gateStatus[ev.phase] = 'open';
    touch(state, ev.ts);
  },
  gate_cleared(state, ev) {
    const gate = getOrCreatePhase(state, ev.phase).gate;
    if (gate) {
      gate.cleared = true;
      if (ev.conditions_met) gate.met = ev.conditions_met;
    }
    state.gateStatus[ev.phase] = 'closed';
    touch(state, ev.ts);
  },
  task_created(state, ev) {
    const task = getOrCreateTask(state, taskId(ev), ev.phase);
    if (ev.name) task.name = ev.name;
    if (ev.depends_on) task.depends_on = ev.depends_on;
    setTaskStatus(task, 'pending');
    touch(state, ev.ts);
  },
  task_dispatched(state, ev) {
    const task = getOrCreateTask(state, taskId(ev), ev.phase || state.current_phase || '_unknown');
    setTaskStatus(task, 'dispatched');
    if (ev.force) task.force = ev.force;
    if (ev.agent) task.agent = ev.agent;
    if (ev.worktree) task.worktree = ev.worktree;
    if (ev.ts) task.since = ev.ts;
    touch(state, ev.ts);
  },
  task_completed(state, ev) {
    const task = getOrCreateTask(state, taskId(ev), ev.phase || state.current_phase || '_unknown');
    setTaskStatus(task, 'completed');
    touch(state, ev.ts);
  },
  task_verified(state, ev) {
    const task = getOrCreateTask(state, taskId(ev), ev.phase || state.current_phase || '_unknown');
    setTaskStatus(task, 'verified');
    if (ev.verified_by) task.verified_by = ev.verified_by;
    touch(state, ev.ts);
  },
  task_failed(state, ev) {
    const task = getOrCreateTask(state, taskId(ev), ev.phase || state.current_phase || '_unknown');
    setTaskStatus(task, 'failed');
    if (ev.reason) task.fail_reason = ev.reason;
    if (ev.attempts != null) task.attempts = ev.attempts;
    touch(state, ev.ts);
  },
  task_blocked(state, ev) {
    const task = getOrCreateTask(state, taskId(ev), ev.phase || state.current_phase || '_unknown');
    task.previousStatus = task.status;
    task.status = 'blocked';
    if (ev.blocked_by) task.blocked_by = ev.blocked_by;
    touch(state, ev.ts);
  },
  task_unblocked(state, ev) {
    const task = getOrCreateTask(state, taskId(ev), ev.phase || state.current_phase || '_unknown');
    task.status = task.previousStatus || 'pending';
    delete task.blocked_by;
    delete task.previousStatus;
    touch(state, ev.ts);
  },
  blocker_added(state, ev) {
    internals(state);
    const blocker = { id: ev.id, blocker: ev.blocker, blocks: ev.blocks || [], status: 'active', added: ev.ts };
    state._blockerMap.set(ev.id, blocker);
    state.blockers.push(blocker);
    touch(state, ev.ts);
  },
  blocker_resolved(state, ev) {
    internals(state);
    const blocker = state._blockerMap.get(ev.id);
    if (blocker) {
      blocker.status = 'resolved';
      blocker.resolved = ev.ts;
    }
    touch(state, ev.ts);
  },
  decision(state, ev) {
    state.decisions.push({ decision: ev.decision, reason: ev.reason || null, at: ev.ts });
    touch(state, ev.ts);
  },
  dependency_discovered(state, ev) {
    state.discoveries.push({
      task_id: ev.task_id,
      dependency: ev.dependency,
      impact: ev.impact,
      at: ev.ts,
    });
    touch(state, ev.ts);
  },
  watcher_verdict_recorded(state, ev) {
    state.watcherVerdicts.push({
      task_id: ev.task_id,
      verdict: ev.verdict,
      severity_counts: ev.severity_counts || null,
      scope: ev.scope || null,
      at: ev.ts,
    });
    touch(state, ev.ts);
  },
  quality_gate_executed(state, ev) {
    state.qualityGates.push({
      phase: ev.phase,
      gate_name: ev.gate_name,
      outcome: ev.outcome,
      details: ev.details || null,
      at: ev.ts,
    });
    touch(state, ev.ts);
  },
  evaluation_outcome(state, ev) {
    state.evaluations.push({
      task_id: ev.task_id,
      watcher_verdict: ev.watcher_verdict,
      actual_outcome: ev.actual_outcome,
      outcome_source: ev.outcome_source,
      divergence_type: ev.divergence_type || null,
      at: ev.ts,
    });
    touch(state, ev.ts);
  },
};

function finalizeState(state) {
  const tasks = [];
  for (const phase of state.phases) {
    for (const task of phase.tasks) {
      delete task.previousStatus;
      tasks.push(task);
    }
  }
  state.activeTasks = tasks
    .filter(task => task.status === 'dispatched' || task.status === 'blocked')
    .map(task => ({ task_id: task.id, force: task.force, agent: task.agent, since: task.since, status: task.status }));
  state.completedTasks = tasks
    .filter(task => task.status === 'completed' || task.status === 'verified')
    .map(task => task.id);
  state.unverifiedTasks = tasks
    .filter(task => task.status === 'completed')
    .map(task => task.id);
  if (state.current_phase && !state.gateStatus[state.current_phase]) state.gateStatus[state.current_phase] = 'open';
  state.warnings = warnings;
  return state;
}

async function recordMaterializeError(line, raw, reason) {
  const sidecar = resolve(ROOT, '.sauron', 'materialize-errors.jsonl');
  const entry = JSON.stringify({
    ts: new Date().toISOString(),
    source: inputPath,
    line,
    reason,
    raw,
  }) + '\n';
  await mkdir(dirname(sidecar), { recursive: true });
  await writeFile(sidecar, entry, { flag: 'a', encoding: 'utf8' });
}

async function validateStronghold(state) {
  if (!strongholdPath || !campaignFilter) return;
  const content = await readFile(resolve(strongholdPath), 'utf8');
  const phaseMatch = content.match(/(?:##\s+Phase|status:\s*)\s*[\r\n ]+([a-z0-9_-]+)/i);
  const strongholdPhase = phaseMatch?.[1] || null;
  if (strongholdPhase && state.current_phase && strongholdPhase !== state.current_phase) {
    state.cross_validation_failure = {
      war_log_phase: state.current_phase,
      stronghold_phase: strongholdPhase,
      divergence_type: 'phase_mismatch',
    };
  }
}

async function replay() {
  const states = new Map();
  const rl = createInterface({ input: createReadStream(inputPath, 'utf8'), crlfDelay: Infinity });
  let lineNum = 0;

  for await (const line of rl) {
    lineNum++;
    const trimmed = line.trim();
    if (!trimmed) continue;

    let event;
    try {
      event = JSON.parse(trimmed);
    } catch (error) {
      const warning = validationWarning(lineNum, 'invalid JSON');
      warnings.push(warning);
      process.stderr.write(`validation_warning: line ${lineNum}: invalid JSON\n`);
      await recordMaterializeError(lineNum, line, warning.reason);
      continue;
    }

    const validation = validateWarEvent(event);
    if (!validation.ok) {
      warnings.push(validationWarning(lineNum, validation.reason));
      process.stderr.write(`validation_warning: line ${lineNum}: ${validation.reason}\n`);
      continue;
    }

    if (campaignFilter && event.campaign !== campaignFilter) continue;

    if (!states.has(event.campaign)) states.set(event.campaign, createState(event.campaign));
    const state = states.get(event.campaign);
    const handler = handlers[event.event];
    if (handler) handler(state, event);
  }

  return states;
}

try {
  const states = await replay();
  const finalized = Object.fromEntries([...states.entries()].map(([campaign, state]) => [campaign, finalizeState(state)]));
  let output;

  if (campaignFilter) {
    output = finalized[campaignFilter] || finalizeState(createState(campaignFilter));
    await validateStronghold(output);
  } else {
    const campaigns = Object.keys(finalized);
    if (campaigns.length === 0) output = finalizeState(createState(null));
    else output = campaigns.length === 1 ? finalized[campaigns[0]] : { campaigns: finalized, warnings };
  }

  const json = JSON.stringify(output, null, 2) + '\n';
  if (dryRun) {
    process.stdout.write(json);
  } else {
    await mkdir(dirname(outputPath), { recursive: true });
    await writeFile(outputPath, json, 'utf8');
    process.stderr.write(`Materialized ${inputPath} -> ${outputPath}\n`);
  }
} catch (error) {
  process.stderr.write(`ERROR: ${error.code === 'ENOENT' ? `file not found: ${inputPath}` : error.message}\n`);
  process.exit(1);
}
