const crypto = require('node:crypto');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { execFileSync } = require('node:child_process');
const { createPage, layouts } = require('./page');

const ROOT = path.resolve(__dirname, '../..');
const SCENES = path.join(ROOT, 'site/scenes');
const serialize = data => JSON.stringify(data, null, 2) + '\n';
const digest = bytes => crypto.createHash('sha256').update(bytes).digest('hex');

function reconcile(file, bytes, check) {
  if (check) {
    if (!fs.existsSync(file) || !fs.readFileSync(file).equals(Buffer.from(bytes))) {
      throw new Error(`generated page drift: ${path.relative(ROOT, file)}`);
    }
  } else {
    fs.writeFileSync(file, bytes);
  }
}

function runtimeName(result, id, authoredPath) {
  const entry = result.source_map.entries.find(entry => entry.authored_id === id && entry.authored_path === authoredPath);
  if (!entry || entry.runtime_names.length !== 1) throw new Error(`missing runtime binding: ${id}`);
  return entry.runtime_names[0];
}

function build({ check = false, cli = process.env.RIVE_CLI || path.join(ROOT, 'target/debug/rive-cli') } = {}) {
  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), 'rive-page-'));
  fs.mkdirSync(SCENES, { recursive: true });
  try {
    const entries = layouts.map(layout => {
      const { scene, controls } = createPage(layout);
      const source = `${layout.id}.v0.json`;
      const artifact = `${layout.id}.riv`;
      const sourceBytes = serialize(scene);
      reconcile(path.join(SCENES, source), sourceBytes, check);
      const output = path.join(temporary, artifact);
      const result = JSON.parse(execFileSync(cli, ['authoring', 'compile', path.join(SCENES, source), '-o', output, '--json'], { cwd: ROOT, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 }));
      if (!result.ok || result.warnings?.length) throw new Error(`page compile failed or warned: ${layout.id}`);
      execFileSync(cli, ['validate', output], { cwd: ROOT, stdio: 'pipe' });
      const bytes = fs.readFileSync(output);
      reconcile(path.join(SCENES, artifact), bytes, check);
      return {
        id: layout.id, minWidth: layout.minWidth, width: layout.width, height: layout.height,
        source: `scenes/${source}`, artifact: `scenes/${artifact}`,
        sourceSha256: digest(sourceBytes), artifactSha256: digest(bytes), bytes: bytes.length,
        stateMachine: runtimeName(result, 'interface', '$.behavior.statecharts[0]'),
        inputs: {
          morph: runtimeName(result, 'interface/morph', '$.behavior.statecharts[0].inputs[0]'),
          paused: runtimeName(result, 'interface/paused', '$.behavior.statecharts[0].inputs[1]'),
        },
        controls,
      };
    });
    const manifest = { format: 1, generator: 'rive-cli authoring compile', layouts: entries };
    reconcile(path.join(SCENES, 'manifest.json'), serialize(manifest), check);
    return manifest;
  } finally {
    fs.rmSync(temporary, { recursive: true, force: true });
  }
}

module.exports = { build, digest, runtimeName };
if (require.main === module) {
  try {
    const result = build({ check: process.argv.includes('--check') });
    console.log(`${process.argv.includes('--check') ? 'Verified' : 'Generated'} ${result.layouts.length} full-page Rive scenes with the public CLI.`);
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
