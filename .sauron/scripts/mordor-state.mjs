#!/usr/bin/env node
/**
 * mordor-state.mjs — Canonical CLI for writing validated events to per-campaign war-logs.
 *
 * Usage:
 *   mordor-state log <event-json>         Write a single validated event
 *   mordor-state log --file <path>        Write validated events from a JSONL file
 *   mordor-state validate <event-json>    Validate without writing
 *   mordor-state log --dry-run <json>     Validate and show what would be written
 */

import { appendFileSync, existsSync, mkdirSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { validateWarEvent } from './lib/war-state-schema.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));

function campaignLogPath(campaign) {
  return resolve(process.cwd(), '.sauron', 'state', 'campaigns', campaign, 'war-log.jsonl');
}

function usage() {
  return `Usage: mordor-state <command> [options]

Commands:
  log <json>       Write a validated event to the war-log
  validate <json>  Validate an event without writing
  log --file <p>   Write all valid events from a JSONL file (skips invalid)
  log --dry-run    Validate and print, do not write

Options:
  --log <path>     Path to war-log.jsonl (default: .sauron/state/campaigns/<campaign>/war-log.jsonl)
  --help           Show this help

Examples:
  mordor-state log '{"ts":"2026-05-15T12:00:00Z","event":"task_completed","campaign":"camp-a","task":"T1"}'
  echo '{...}' | mordor-state log -
`;
}

function fail(message, code = 1) {
  process.stderr.write(`mordor-state: error: ${message}\n`);
  process.exit(code);
}

export function writeEvent(event, logPath) {
  const line = JSON.stringify(event) + '\n';
  appendFileSync(logPath, line, 'utf8');
}

function parseEvent(input) {
  try {
    return JSON.parse(input);
  } catch (err) {
    return { _parseError: err.message };
  }
}

export function validateAndReport(event) {
  const result = validateWarEvent(event);
  if (!result.ok) {
    return { ok: false, reason: result.reason, event };
  }
  return { ok: true, event };
}

function readStdin() {
  return new Promise((resolve, reject) => {
    let data = '';
    process.stdin.setEncoding('utf8');
    process.stdin.on('data', chunk => { data += chunk; });
    process.stdin.on('end', () => resolve(data));
    process.stdin.on('error', reject);
  });
}

export async function main(argv = process.argv.slice(2)) {
  if (argv.includes('--help') || argv.includes('-h')) {
    process.stdout.write(usage());
    process.exit(0);
  }

  const command = argv[0];
  if (!command || !['log', 'validate'].includes(command)) {
    fail(`unknown command: ${command || '(none)'}\n${usage()}`, 2);
  }

  const logIndex = argv.indexOf('--log');
  const explicitLog = logIndex !== -1 ? argv[logIndex + 1] : null;

  const dryRun = argv.includes('--dry-run');
  const fileMode = argv.includes('--file');

  let inputs = [];

  if (fileMode) {
    const fileIndex = argv.indexOf('--file');
    const filePath = argv[fileIndex + 1];
    if (!filePath) fail('--file requires a path');
    if (!existsSync(filePath)) fail(`file not found: ${filePath}`);
    const content = readFileSync(filePath, 'utf8');
    inputs = content.split('\n').filter(line => line.trim());
  } else {
    const optionValues = new Set();
    for (let i = 0; i < argv.length; i++) {
      if (argv[i] === '--log' || argv[i] === '--file') {
        if (argv[i + 1]) optionValues.add(argv[i + 1]);
      }
    }
    const jsonArg = argv.find((arg, i) => i > 0 && !arg.startsWith('--') && arg !== '-' && !optionValues.has(arg));
    const hasStdin = argv.includes('-');

    if (hasStdin) {
      const stdinData = await readStdin();
      inputs = stdinData.split('\n').filter(line => line.trim());
    } else if (jsonArg) {
      inputs = [jsonArg];
    } else {
      fail('no event JSON provided');
    }
  }

  const results = [];
  for (const input of inputs) {
    const event = parseEvent(input);
    if (event._parseError) {
      results.push({ ok: false, reason: `invalid JSON: ${event._parseError}`, raw: input });
      continue;
    }

    const validation = validateAndReport(event);
    if (!validation.ok) {
      results.push(validation);
      continue;
    }

    const logPath = explicitLog || campaignLogPath(event.campaign);

    if (command === 'validate' || dryRun) {
      results.push({ ok: true, event, written: false });
    } else {
      try {
        mkdirSync(dirname(logPath), { recursive: true });
        writeEvent(event, logPath);
        results.push({ ok: true, event, written: true, log: logPath });
      } catch (err) {
        results.push({ ok: false, reason: `write failed: ${err.message}`, event });
      }
    }
  }

  const allOk = results.every(r => r.ok);
  const output = { ok: allOk, results };

  process.stdout.write(JSON.stringify(output, null, 2) + '\n');
  process.exit(allOk ? 0 : 1);
}

if (import.meta.url === new URL(`file://${process.argv[1]}`, 'file://').href) {
  await main();
}
