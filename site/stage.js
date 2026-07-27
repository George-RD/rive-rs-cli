const fs = require("node:fs");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..");
const SITE = __dirname;

const PAGE_FILES = ["index.html", "styles.css", "main.js", "verify.txt"];
const VENDORED = [
  ["assets/rive.js", "assets/rive.js"],
  ["assets/rive.wasm", "assets/rive.wasm"],
];

function showcaseFiles() {
  return fs
    .readdirSync(path.join(ROOT, "showcase"))
    .filter((name) => name.endsWith(".riv") || name.endsWith(".json"))
    .map((name) => [`showcase/${name}`, `showcase/${name}`]);
}

function referencedScenes() {
  const source = fs.readFileSync(path.join(SITE, "main.js"), "utf8");
  const ids = new Set();
  for (const match of source.matchAll(/\bid:\s*"([a-z0-9_]+)"/g)) {
    ids.add(match[1]);
  }
  for (const match of source.matchAll(/mountScene\([^)]*id:\s*"([a-z0-9_]+)"/g)) {
    ids.add(match[1]);
  }
  return [...ids];
}

function assertReferencedScenesExist() {
  const missing = [];
  for (const id of referencedScenes()) {
    for (const extension of [".riv", ".json"]) {
      const file = path.join(ROOT, "showcase", `${id}${extension}`);
      if (!fs.existsSync(file)) {
        missing.push(`showcase/${id}${extension}`);
      }
    }
  }
  if (missing.length > 0) {
    throw new Error(`site/main.js references files that do not exist: ${missing.join(", ")}`);
  }
  return referencedScenes().length;
}

function plan() {
  assertReferencedScenesExist();
  return [
    ...PAGE_FILES.map((name) => [`site/${name}`, name]),
    ...VENDORED,
    ...showcaseFiles(),
  ];
}

function stage(destination) {
  fs.rmSync(destination, { recursive: true, force: true });
  for (const [from, to] of plan()) {
    const source = path.join(ROOT, from);
    if (!fs.existsSync(source)) {
      throw new Error(`site staging is missing ${from}`);
    }
    const target = path.join(destination, to);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.copyFileSync(source, target);
  }
  return plan().length;
}

module.exports = { plan, stage, referencedScenes, assertReferencedScenesExist, ROOT, SITE };

if (require.main === module) {
  const destination = path.resolve(process.argv[2] || path.join(ROOT, "target", "site"));
  const count = stage(destination);
  process.stdout.write(`staged ${count} files into ${destination}\n`);
}
