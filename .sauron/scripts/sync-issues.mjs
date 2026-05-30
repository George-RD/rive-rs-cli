#!/usr/bin/env node
/**
 * sync-issues.mjs — Bridge war-log events to GitHub Issues.
 *
 * Usage:
 *   sync-issues.mjs --campaign <slug> [--repo <owner/repo>]
 *
 * Actions:
 *   - campaign_completed  → close issue + comment PR link
 *   - 3× task_failed      → label issue `blocked` + comment
 */

import { readFileSync } from 'node:fs';
import { execSync } from 'node:child_process';
import { resolve } from 'node:path';

function fail(message, code = 1) {
  process.stderr.write(`sync-issues: error: ${message}\n`);
  process.exit(code);
}

const args = process.argv.slice(2);
const campaignSlug = args[args.indexOf('--campaign') + 1] || fail('missing --campaign');
const repoFlag = args[args.indexOf('--repo') + 1] || '';

function repoArg() {
  return repoFlag ? ['--repo', repoFlag] : [];
}

function getRepo() {
  if (repoFlag) return repoFlag;
  try {
    const url = execSync('git remote get-url origin', { encoding: 'utf8' }).trim();
    const m = url.match(/github\.com[:/]([^/]+)\/([^/]+?)(?:\.git)?$/);
    if (m) return `${m[1]}/${m[2]}`;
  } catch {}
  return null;
}

const repo = getRepo();
if (!repo) fail('cannot determine repo; pass --repo owner/repo');

const logPath = resolve(process.cwd(), '.sauron', 'state', 'campaigns', campaignSlug, 'war-log.jsonl');

let lines;
try {
  lines = readFileSync(logPath, 'utf8').split('\n').filter(l => l.trim());
} catch (err) {
  fail(`cannot read war-log: ${err.message}`);
}

const events = lines.map(l => JSON.parse(l)).filter(e => e.campaign === campaignSlug);

const issueNumber = campaignSlug.match(/#(\d+)/)?.[1];
if (!issueNumber) {
  console.log(JSON.stringify({ ok: true, action: 'none', reason: 'no issue number in campaign slug' }));
  process.exit(0);
}

const failures = events.filter(e => e.event === 'task_failed');
const completed = events.filter(e => e.event === 'campaign_completed');
const lastCompleted = completed[completed.length - 1];

let action = 'none';

if (lastCompleted) {
  action = 'close';
  const pr = lastCompleted.pr || lastCompleted.pr_url || '';
  const body = `Campaign **${campaignSlug}** completed. ${pr ? `Merged via ${pr}.` : ''}`;
  try {
    execSync(['gh', 'issue', 'close', issueNumber, ...repoArg(), '--comment', body].join(' '), { stdio: 'inherit' });
  } catch (err) {
    fail(`gh issue close failed: ${err.message}`);
  }
} else if (failures.length >= 3) {
  action = 'block';
  const body = `Campaign **${campaignSlug}** has recorded ${failures.length} task failures. Labeling as blocked pending resolution.`;
  try {
    execSync(['gh', 'issue', 'edit', issueNumber, ...repoArg(), '--add-label', 'blocked'].join(' '), { stdio: 'inherit' });
    execSync(['gh', 'issue', 'comment', issueNumber, ...repoArg(), '--body', body].join(' '), { stdio: 'inherit' });
  } catch (err) {
    fail(`gh issue edit/comment failed: ${err.message}`);
  }
}

console.log(JSON.stringify({ ok: true, action, repo, issue: issueNumber, failures: failures.length, completed: completed.length }));
