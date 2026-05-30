#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const WAR_QUERY = resolve(__dirname, 'war-query.sh');
const MATERIALIZER = resolve(__dirname, 'materialize-state.mjs');

function writeJson(value) {
  process.stdout.write(JSON.stringify(value, null, 2) + '\n');
}

function fail(message, code = 1) {
  process.stderr.write(JSON.stringify({ error: message }) + '\n');
  process.exit(code);
}

function optionValue(argv, option) {
  const index = argv.indexOf(option);
  return index === -1 ? null : argv[index + 1];
}

function usage() {
  return {
    usage: 'sauron-status <campaign> <subcommand> [options]',
    subcommands: ['phase', 'deployed', 'unverified', 'gate [phase]', 'task <id>', 'dashboard', 'recover'],
    options: ['--log <path>', '--validate-stronghold <path>', '--all', '--help'],
  };
}

function parseArgs(argv) {
  if (argv.includes('--help') || argv.includes('-h')) return { help: true };
  const all = argv.includes('--all');
  const positional = [];
  for (let index = 0; index < argv.length; index++) {
    const arg = argv[index];
    if (arg === '--all') continue;
    if (arg === '--log' || arg === '--validate-stronghold') {
      index++;
      continue;
    }
    if (!arg.startsWith('--')) positional.push(arg);
  }

  let campaign = positional[0];
  let command = positional[1];
  let restStart = 2;

  if (all) {
    if (positional[0] !== 'recover') fail('--all is only supported with recover');
    campaign = null;
    command = 'recover';
    restStart = 1;
  }

  const rest = positional.slice(restStart);

  if (!command) fail('missing campaign or subcommand');

  const log = optionValue(argv, '--log');
  const validateStronghold = optionValue(argv, '--validate-stronghold');

  return { campaign, command, passthrough: rest, log, validateStronghold, all };
}

function defaultLog(campaign) {
  if (campaign) return resolve(process.cwd(), '.sauron', 'state', 'campaigns', campaign, 'war-log.jsonl');
  return resolve(process.cwd(), '.sauron', 'state', 'campaigns', 'default', 'war-log.jsonl');
}

function runWarQuery(campaign, command, args, log) {
  const queryArgs = ['--json'];
  if (log) queryArgs.push('--log', log);
  else if (campaign) queryArgs.push('--log', defaultLog(campaign));
  queryArgs.push(campaign, command, ...args);
  const stdout = execFileSync(WAR_QUERY, queryArgs, { encoding: 'utf8' });
  return JSON.parse(stdout);
}

function runMaterializer({ campaign, log, validateStronghold, all }) {
  const args = [];
  if (log) args.push(log);
  else if (campaign) args.push(defaultLog(campaign));
  args.push('--dry-run');
  if (campaign && !all) args.push('--campaign', campaign);
  if (validateStronghold) args.push('--validate-stronghold', validateStronghold);
  const stdout = execFileSync('node', [MATERIALIZER, ...args], { encoding: 'utf8' });
  return JSON.parse(stdout);
}

function dashboard(campaign, log, validateStronghold) {
  const state = runMaterializer({ campaign, log, validateStronghold });
  return {
    campaign: state.campaign,
    currentPhase: state.currentPhase ?? state.current_phase ?? null,
    activeTasks: state.activeTasks || [],
    completedTasks: state.completedTasks || [],
    unverifiedTasks: state.unverifiedTasks || [],
    gateStatus: state.gateStatus || {},
    discoveries: state.discoveries || [],
    warnings: state.warnings || [],
    ...(state.cross_validation_failure ? { cross_validation_failure: state.cross_validation_failure } : {}),
  };
}

export function main(argv = process.argv.slice(2)) {
  const parsed = parseArgs(argv);
  if (parsed.help) {
    writeJson(usage());
    return;
  }

  const { campaign, command, passthrough, log, validateStronghold, all } = parsed;
  if (!all && !campaign) fail('campaign is required');

  try {
    switch (command) {
      case 'phase':
      case 'deployed':
      case 'unverified':
      case 'gate':
      case 'task':
        writeJson(runWarQuery(campaign, command, passthrough, log));
        break;
      case 'dashboard':
        writeJson(dashboard(campaign, log, validateStronghold));
        break;
      case 'recover':
        writeJson(runMaterializer({ campaign, log, all }));
        break;
      default:
        fail(`unknown subcommand: ${command}`);
    }
  } catch (error) {
    fail(error.stderr?.toString().trim() || error.message);
  }
}

if (import.meta.url === `file://${process.argv[1]}`) main();
