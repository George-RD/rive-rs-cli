const { spawnSync } = require('child_process');
const fs = require('fs');
const http = require('http');
const path = require('path');

const FIXTURES = [
  'minimal', 'shapes', 'animation', 'state_machine', 'path', 'cubic_easing', 'trim_path',
  'multi_artboard', 'nested_artboard', 'artboard_preset', 'gradients', 'color_animation',
  'loop_animation', 'stroke_styles', 'bones', 'constraints', 'text', 'layout', 'data_binding'
];

const DEMO_DIR = path.join(__dirname);
const RIV_DIR = path.join(DEMO_DIR, 'riv');
const FIXTURES_DIR = path.join(__dirname, '..', 'tests', 'fixtures');

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
let successCount = 0;
for (const fixture of FIXTURES) {
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
  } else {
    console.log('FAILED');
    console.error(result.stderr.toString());
  }
}

console.log(`Generated ${successCount}/${FIXTURES.length} files.`);

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
