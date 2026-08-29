const fs = require("node:fs");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..");
const SITE = __dirname;

const PAGE_FILES = [
  "index.html",
  "lab.html",
  "showcase.html",
  "showcase.json",
  "styles.css",
  "playback.css",
  "landing.js",
  "main.js",
  "playback.js",
  "showcase.js",
  "verify.txt",
];
const VENDORED = [
  ["assets/rive.js", "assets/rive.js"],
  ["assets/rive.wasm", "assets/rive.wasm"],
];
const SHOWCASE_GUIDES = [["docs/authoring-cli.md", "docs/authoring-cli.md"]];

function parityFiles() {
  const entries = [["parity/results.json", "parity/results.json"]];
  for (const [directory, extensions] of [
    ["parity/official", [".riv"]],
    ["parity/reproductions", [".riv", ".json"]],
  ]) {
    for (const name of fs.readdirSync(path.join(ROOT, directory))) {
      if (extensions.some((extension) => name.endsWith(extension))) {
        entries.push([`${directory}/${name}`, `${directory}/${name}`]);
      }
    }
  }
  return entries;
}

function referencedScenes() {
  const results = JSON.parse(fs.readFileSync(path.join(ROOT, "parity", "results.json"), "utf8"));
  const files = new Set();
  for (const rung of results) {
    files.add(rung.official);
    files.add(rung.reproduction);
    files.add(rung.source);
  }
  for (const name of ["main.js", "landing.js"]) {
    const page = fs.readFileSync(path.join(SITE, name), "utf8");
    for (const match of page.matchAll(/"(parity\/[a-z0-9_/.]+\.riv)"/g)) {
      files.add(match[1]);
    }
  }
  return [...files];
}

function assertReferencedScenesExist() {
  const missing = referencedScenes().filter(
    (file) => !fs.existsSync(path.join(ROOT, file))
  );
  if (missing.length > 0) {
    throw new Error(`the site references files that do not exist: ${missing.join(", ")}`);
  }
  return referencedScenes().length;
}

function showcaseEntries() {
  const manifest = path.join(SITE, "showcase.json");
  const entries = JSON.parse(fs.readFileSync(manifest, "utf8"));
  if (!Array.isArray(entries)) {
    throw new Error("site/showcase.json must contain an array");
  }
  return entries;
}

function showcaseFiles(entries = showcaseEntries()) {
  const files = new Map(SHOWCASE_GUIDES);
  for (const entry of entries) {
    for (const field of ["artifact", "source"]) {
      const value = entry[field];
      if (typeof value === "string" && value.length > 0) files.set(value, value);
    }
  }
  return [...files.entries()];
}

function assertShowcaseEntriesExist(entries = showcaseEntries()) {
  const missing = [];
  for (const entry of entries) {
    for (const field of ["artifact", "source"]) {
      const value = entry[field];
      if (typeof value !== "string" || value.length === 0) {
        missing.push(`${entry.id || "<unnamed>"}.${field} is missing`);
      } else if (!fs.existsSync(path.join(ROOT, value))) {
        missing.push(`${entry.id || "<unnamed>"}.${field}: ${value}`);
      }
    }
  }
  for (const [from] of SHOWCASE_GUIDES) {
    if (!fs.existsSync(path.join(ROOT, from))) missing.push(`guide: ${from}`);
  }
  if (missing.length > 0) {
    throw new Error(`showcase manifest references missing files: ${missing.join(", ")}`);
  }
  return entries.length;
}

function plan() {
  assertReferencedScenesExist();
  const entries = showcaseEntries();
  assertShowcaseEntriesExist(entries);
  return [
    ...PAGE_FILES.map((name) => [`site/${name}`, name]),
    ...VENDORED,
    ...parityFiles(),
    ...showcaseFiles(entries),
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

module.exports = {
  plan,
  stage,
  referencedScenes,
  assertReferencedScenesExist,
  showcaseEntries,
  showcaseFiles,
  assertShowcaseEntriesExist,
  ROOT,
  SITE,
};

if (require.main === module) {
  const destination = path.resolve(process.argv[2] || path.join(ROOT, "target", "site"));
  const count = stage(destination);
  process.stdout.write(`staged ${count} files into ${destination}\n`);
}
