const fs = require("node:fs");
const http = require("node:http");
const path = require("node:path");

const { stage, ROOT } = require("./stage");

const STAGED = path.join(ROOT, "target", "site");
const PORT = Number(process.env.SITE_PORT || 8770);

const MIME = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".js": "application/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".txt": "text/plain; charset=utf-8",
  ".wasm": "application/wasm",
  ".riv": "application/octet-stream",
  ".png": "image/png",
};

stage(STAGED);

const server = http.createServer((request, response) => {
  const clean = decodeURIComponent(request.url.split("?")[0]);
  const rel = clean === "/" ? "index.html" : clean.replace(/^\/+/, "");
  const file = path.join(STAGED, rel);
  if (!file.startsWith(STAGED) || !fs.existsSync(file) || !fs.statSync(file).isFile()) {
    response.writeHead(404, { "content-type": "text/plain" });
    response.end(`not found: ${request.url}`);
    return;
  }
  response.writeHead(200, {
    "content-type": MIME[path.extname(file)] || "application/octet-stream",
  });
  response.end(fs.readFileSync(file));
});

server.listen(PORT, "127.0.0.1", () => {
  process.stdout.write(`site: http://127.0.0.1:${PORT}/\n`);
});
