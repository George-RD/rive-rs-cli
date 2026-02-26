const { spawnSync } = require('child_process');
const fs = require('fs');
const http = require('http');
const path = require('path');

function discoverFixtures() {
  if (!fs.existsSync(FIXTURES_DIR)) {
    return [];
  }

  return fs
    .readdirSync(FIXTURES_DIR)
    .filter((name) => name.endsWith('.json'))
    .map((name) => name.replace(/\.json$/, ''))
    .filter((name) => !name.startsWith('invalid_'))
    .sort();
}

const DEMO_DIR = path.join(__dirname);
const RIV_DIR = path.join(DEMO_DIR, 'riv');
const FIXTURES_DIR = path.join(__dirname, '..', 'tests', 'fixtures');
const OFFICIAL_FIRE_SOURCE = '/tmp/rive-runtime/renderer/webgpu_player/rivs/fire_button.riv';
const OFFICIAL_FIRE_TARGET = path.join(RIV_DIR, 'official_test.riv');
const MANIFEST_JS_TARGET = path.join(RIV_DIR, 'manifest.js');

const FIXTURE_OVERRIDES = {
  minimal: {
    expectation: 'Tiny baseline file; expected to look mostly empty.',
    scope: 'visual',
    tags: ['static']
  },
  shapes: {
    expectation: 'Basic visible geometry (shape and fill sanity check).',
    scope: 'visual',
    tags: ['static']
  },
  gradients: {
    expectation: 'Linear gradient is diagonal (red->magenta->blue). Oval uses radial yellow->green; replay re-runs orientation motion.',
    scope: 'visual',
    tags: ['animated']
  },
  stroke_styles: {
    expectation: 'Stroke thickness/style showcase around paths.',
    scope: 'visual',
    tags: ['static']
  },
  path: {
    expectation: 'Path-based drawing rendered as static art.',
    scope: 'visual',
    tags: ['static']
  },
  artboard_preset: {
    category: 'nonvisual',
    expectation: 'Preset sizing encoding fixture. No drawable objects, so canvas stays blank.',
    scope: 'nonvisual',
    tags: ['schema']
  },
  bones: {
    expectation: 'Bone/rig scaffold preview; currently minimal visual output.',
    scope: 'visual',
    tags: ['static', 'structure']
  },
  empty_artboard: {
    category: 'nonvisual',
    expectation: 'Intentionally empty structural fixture; blank canvas is expected.',
    scope: 'nonvisual',
    tags: ['schema', 'structure']
  },
  text: {
    category: 'nonvisual',
    expectation: 'Text schema fixture; renderer output may be limited in this harness.',
    scope: 'nonvisual',
    tags: ['schema']
  },
  layout: {
    category: 'nonvisual',
    expectation: 'Schema/encoding fixture: validated structurally, not a visual showcase.',
    scope: 'nonvisual',
    tags: ['schema']
  },
  data_binding: {
    category: 'nonvisual',
    expectation: 'Schema/encoding fixture: validated structurally, not a visual showcase.',
    scope: 'nonvisual',
    tags: ['schema']
  },
  animation: {
    expectation: 'Red ball bounce one-shot. Use Replay to run it again.',
    scope: 'visual',
    tags: ['animated']
  },
  color_animation: {
    expectation: 'Color transitions over time.',
    scope: 'visual',
    tags: ['animated']
  },
  cubic_easing: {
    expectation: 'Width animation with cubic easing curve.',
    scope: 'visual',
    tags: ['animated']
  },
  loop_animation: {
    expectation: 'Continuous looping motion.',
    scope: 'visual',
    tags: ['animated']
  },
  trim_path: {
    expectation: 'Magenta arc sweeps around the ring over 2s; Replay restarts the sweep.',
    scope: 'visual',
    tags: ['animated']
  },
  multi_artboard: {
    expectation: 'Two artboards with per-artboard animations. Replay checks timeline behavior; Artboard switch checks scoping.',
    scope: 'visual',
    tags: ['animated', 'interactive', 'structure']
  },
  nested_artboard: {
    category: 'interactive',
    expectation: 'Nested-artboard wiring check. No timeline animation; use Artboard switch to verify Main vs Component.',
    scope: 'visual',
    tags: ['interactive', 'structure'],
    replay: false
  },
  state_machine: {
    expectation: 'Toggle isOn to switch magenta dot into active state (moves right + turns green).',
    scope: 'visual',
    tags: ['interactive', 'animated']
  },
  button_states: {
    category: 'interactive',
    expectation: 'Button with hover/press/loading states. Toggle isHovered/isPressed/isLoading to switch visual states.',
    scope: 'visual',
    tags: ['interactive', 'animated'],
    stateMachine: 'ButtonStateMachine'
  },
  official_test: {
    category: 'interactive',
    expectation: 'Official fire_button.riv. Toggle ON to ignite fire, OFF to extinguish.',
    scope: 'visual',
    tags: ['interactive', 'animated'],
    stateMachine: 'State Machine 1'
  }
};

function readFixtureSpec(fixtureName) {
  const fixturePath = path.join(FIXTURES_DIR, `${fixtureName}.json`);
  if (!fs.existsSync(fixturePath)) {
    return null;
  }

  try {
    const content = fs.readFileSync(fixturePath, 'utf8');
    return JSON.parse(content);
  } catch (error) {
    console.warn(`Warning: failed to parse ${fixtureName}.json: ${error.message}`);
    return null;
  }
}

function inferFixtureMetadata(fixtureName, spec) {
  const artboards = spec?.artboards || (spec?.artboard ? [spec.artboard] : []);
  const artboardNames = artboards.map((artboard) => artboard.name).filter(Boolean);
  const hasAnimations = artboards.some((artboard) => Array.isArray(artboard.animations) && artboard.animations.length > 0);
  const hasStateMachines = artboards.some((artboard) => Array.isArray(artboard.state_machines) && artboard.state_machines.length > 0);
  const firstStateMachine = artboards
    .flatMap((artboard) => artboard.state_machines || [])
    .find((stateMachine) => typeof stateMachine?.name === 'string');

  let category = 'static';
  if (hasStateMachines) {
    category = 'interactive';
  } else if (hasAnimations) {
    category = 'animated';
  }

  const tags = new Set([category]);
  if (artboardNames.length > 1) {
    tags.add('structure');
  }
  if (hasStateMachines && hasAnimations) {
    tags.add('animated');
  }

  const metadata = {
    name: fixtureName,
    category,
    scope: 'visual',
    tags: [...tags],
    expectation: `${fixtureName} fixture loaded.`,
    artboards: artboardNames.length > 1 ? artboardNames : undefined,
    stateMachine: firstStateMachine?.name,
    replay: category === 'animated' || hasAnimations
  };

  const override = FIXTURE_OVERRIDES[fixtureName];
  if (override) {
    return {
      ...metadata,
      ...override,
      tags: override.tags || metadata.tags
    };
  }

  return metadata;
}

function generateManifest(generatedFixtures, includeOfficialTest) {
  const fixtures = generatedFixtures
    .map((fixtureName) => {
      const spec = readFixtureSpec(fixtureName);
      return inferFixtureMetadata(fixtureName, spec || {});
    })
    .filter(Boolean);

  if (includeOfficialTest) {
    fixtures.push({
      name: 'official_test',
      ...FIXTURE_OVERRIDES.official_test
    });
  }

  return {
    generatedAt: new Date().toISOString(),
    fixtures
  };
}

// 1. Ensure cargo build works
console.log('Building project...');
const buildResult = spawnSync('cargo', ['build', '--quiet'], { stdio: 'inherit' });
if (buildResult.status !== 0) {
  console.error('Cargo build failed');
  process.exit(1);
}

// 2. Create demo/riv/ directory
if (!fs.existsSync(RIV_DIR)) {
  fs.mkdirSync(RIV_DIR, { recursive: true });
}

// 3. Generate .riv files
console.log('Generating .riv files...');
const fixtures = discoverFixtures();
let successCount = 0;
const generatedFixtures = [];
for (const fixture of fixtures) {
  const inputPath = path.join(FIXTURES_DIR, `${fixture}.json`);
  const outputPath = path.join(RIV_DIR, `${fixture}.riv`);
  
  if (!fs.existsSync(inputPath)) {
    console.warn(`Warning: Fixture ${fixture}.json not found, skipping.`);
    continue;
  }

  process.stdout.write(`Generating ${fixture}.riv... `);
  const result = spawnSync('cargo', ['run', '--quiet', '--', 'generate', inputPath, '-o', outputPath]);
  
  if (result.status === 0) {
    console.log('OK');
    successCount++;
    generatedFixtures.push(fixture);
  } else {
    console.log('FAILED');
    console.error(result.stderr.toString());
  }
}

console.log(`Generated ${successCount}/${fixtures.length} files.`);

if (!fs.existsSync(OFFICIAL_FIRE_TARGET) && fs.existsSync(OFFICIAL_FIRE_SOURCE)) {
  fs.copyFileSync(OFFICIAL_FIRE_SOURCE, OFFICIAL_FIRE_TARGET);
  console.log('Copied official fire_button.riv to demo/riv/official_test.riv');
}

if (!fs.existsSync(OFFICIAL_FIRE_TARGET)) {
  console.warn('Warning: official_test.riv missing. Add /tmp/rive-runtime/renderer/webgpu_player/rivs/fire_button.riv or place demo/riv/official_test.riv manually.');
}

const manifest = generateManifest(generatedFixtures, fs.existsSync(OFFICIAL_FIRE_TARGET));
const manifestJs = `window.__RIVE_FIXTURE_MANIFEST = ${JSON.stringify(manifest, null, 2)};\n`;
fs.writeFileSync(MANIFEST_JS_TARGET, manifestJs, 'utf8');
console.log('Generated demo/riv/manifest.js');

// 4. Start HTTP server
const PORT = 3000;
const MIME_TYPES = {
  '.html': 'text/html',
  '.js': 'text/javascript',
  '.css': 'text/css',
  '.riv': 'application/octet-stream',
  '.json': 'application/json',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.ico': 'image/x-icon'
};

const server = http.createServer((req, res) => {
  console.log(`${req.method} ${req.url}`);
  
  // Handle root path
  let filePath = req.url === '/' ? path.join(DEMO_DIR, 'index.html') : path.join(DEMO_DIR, req.url);
  
  // Prevent directory traversal
  if (!filePath.startsWith(DEMO_DIR)) {
    res.writeHead(403);
    res.end('Forbidden');
    return;
  }

  const extname = path.extname(filePath);
  const contentType = MIME_TYPES[extname] || 'application/octet-stream';

  fs.readFile(filePath, (err, content) => {
    if (err) {
      if (err.code === 'ENOENT') {
        res.writeHead(404);
        res.end('Not Found');
      } else {
        res.writeHead(500);
        res.end(`Server Error: ${err.code}`);
      }
    } else {
      res.writeHead(200, { 'Content-Type': contentType });
      res.end(content, 'utf-8');
    }
  });
});

server.listen(PORT, () => {
  console.log(`Gallery ready at http://localhost:${PORT}`);
  
  // Open browser
  const start = (process.platform == 'darwin' ? 'open' : process.platform == 'win32' ? 'start' : 'xdg-open');
  spawnSync(start, [`http://localhost:${PORT}`]);
});
