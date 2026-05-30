#!/usr/bin/env node
/**
 * dag-validate.mjs — Validate campaign DAG before dispatch.
 *
 * Checks:
 *   1. Every dependency id exists (no dangling)
 *   2. No cycles
 *   3. Law-8: every implementer phase has a downstream Watcher/review phase
 *   4. Derives waves by topological sort
 *
 * Usage:
 *   dag-validate.mjs <campaign.json> [--dry-run]
 *   dag-validate.mjs --stdin [--dry-run]
 */

import { readFileSync } from 'node:fs';

function fail(message, code = 1) {
  process.stderr.write(`dag-validate: error: ${message}\n`);
  process.exit(code);
}

function usage() {
  return `Usage: dag-validate.mjs <campaign.json> [--dry-run]
       dag-validate.mjs --stdin [--dry-run]`;
}

const args = process.argv.slice(2);
if (args.includes('--help') || args.includes('-h')) {
  console.log(usage());
  process.exit(0);
}

const dryRun = args.includes('--dry-run');
const useStdin = args.includes('--stdin');

let campaign;
if (useStdin) {
  const chunks = [];
  for await (const chunk of process.stdin) chunks.push(chunk);
  campaign = JSON.parse(Buffer.concat(chunks).toString('utf8'));
} else {
  const path = args.find(a => !a.startsWith('--'));
  if (!path) fail('missing campaign.json path');
  campaign = JSON.parse(readFileSync(path, 'utf8'));
}

const phases = campaign.phases || [];
const phaseMap = new Map(phases.map(p => [p.id, p]));

const errors = [];
const warnings = [];

// 1. Dangling dependencies
for (const phase of phases) {
  for (const dep of (phase.dependencies || [])) {
    if (!phaseMap.has(dep)) {
      errors.push(`Dangling dependency: phase "${phase.id}" depends on unknown phase "${dep}"`);
    }
  }
}

// 2. Cycle detection (DFS)
const WHITE = 0, GRAY = 1, BLACK = 2;
const color = new Map(phases.map(p => [p.id, WHITE]));
const stack = [];

function dfs(nodeId) {
  color.set(nodeId, GRAY);
  stack.push(nodeId);
  const node = phaseMap.get(nodeId);
  for (const dep of (node?.dependencies || [])) {
    if (!phaseMap.has(dep)) continue;
    if (color.get(dep) === GRAY) {
      const cycle = stack.slice(stack.indexOf(dep)).concat(dep);
      errors.push(`Cycle detected: ${cycle.join(' -> ')}`);
    } else if (color.get(dep) === WHITE) {
      dfs(dep);
    }
  }
  stack.pop();
  color.set(nodeId, BLACK);
}

for (const phase of phases) {
  if (color.get(phase.id) === WHITE) dfs(phase.id);
}

// 3. Law-8: every implementer phase has a downstream Watcher/review phase
const implementerForces = new Set(['nazgul', 'architect', 'forge-master']);
const watcherForces = new Set(['watcher', 'cleansing-watcher', 'shadow-hunter', 'test-warden', 'inquisitor', 'reforger']);

function downstreamIds(phaseId) {
  const downstream = new Set();
  function collect(id) {
    for (const p of phases) {
      if ((p.dependencies || []).includes(id)) {
        if (!downstream.has(p.id)) {
          downstream.add(p.id);
          collect(p.id);
        }
      }
    }
  }
  collect(phaseId);
  return downstream;
}

for (const phase of phases) {
  const force = (phase.force || '').toLowerCase();
  if (implementerForces.has(force)) {
    const downstream = downstreamIds(phase.id);
    const hasWatcher = [...downstream].some(did => {
      const f = (phaseMap.get(did)?.force || '').toLowerCase();
      return watcherForces.has(f);
    });
    if (!hasWatcher) {
      errors.push(`Law-8 gap: implementer phase "${phase.id}" (force: ${phase.force}) has no downstream Watcher/review phase`);
    }
  }
}

// 4. Wave derivation (Kahn's algorithm)
const inDegree = new Map(phases.map(p => [p.id, 0]));
const adj = new Map(phases.map(p => [p.id, []]));
for (const phase of phases) {
  for (const dep of (phase.dependencies || [])) {
    if (!phaseMap.has(dep)) continue;
    adj.get(dep).push(phase.id);
    inDegree.set(phase.id, inDegree.get(phase.id) + 1);
  }
}

const waves = [];
const queue = [...inDegree.entries()].filter(([, d]) => d === 0).map(([id]) => id);
const visited = new Set(queue);

while (queue.length > 0) {
  const waveSize = queue.length;
  const wave = [];
  for (let i = 0; i < waveSize; i++) {
    const id = queue.shift();
    wave.push(id);
    for (const next of adj.get(id)) {
      inDegree.set(next, inDegree.get(next) - 1);
      if (inDegree.get(next) === 0 && !visited.has(next)) {
        visited.add(next);
        queue.push(next);
      }
    }
  }
  waves.push(wave);
}

const unvisited = phases.filter(p => !visited.has(p.id));
if (unvisited.length > 0 && errors.length === 0) {
  errors.push(`Unreachable phases (cycle or disconnected): ${unvisited.map(p => p.id).join(', ')}`);
}

// Report
const report = {
  campaign: campaign.id || 'unknown',
  ok: errors.length === 0,
  errors,
  warnings,
  waves,
  waveCount: waves.length,
};

if (dryRun) {
  console.log(JSON.stringify(report, null, 2));
  process.exit(errors.length > 0 ? 1 : 0);
}

console.log(JSON.stringify(report, null, 2));
process.exit(errors.length > 0 ? 1 : 0);
