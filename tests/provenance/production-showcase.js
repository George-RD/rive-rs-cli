const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const ROOT = path.resolve(__dirname, "..", "..");
const manifestPath = path.join(ROOT, "site", "showcase.json");

function git(args) {
  return execFileSync("git", args, { cwd: ROOT, encoding: "utf8" }).trim();
}

function requiredString(entry, field) {
  const value = entry[field];
  assert.equal(typeof value, "string", `${entry.id}.${field} must be a string`);
  assert.ok(value.length > 0, `${entry.id}.${field} must not be empty`);
  return value;
}

function localBlobSha(file) {
  return git(["hash-object", file]);
}

function upstreamBlobSha(commit, file) {
  return git(["rev-parse", `${commit}:${file}`]);
}

function quotedConstant(source, name, id) {
  const match = source.match(new RegExp(`const ${name} =\\s*['\"]([^'\"]+)['\"];`));
  assert.ok(match, `${id} consumer evidence is missing ${name}`);
  return match[1];
}

function assertEvidence(entry, values) {
  const evidencePath = requiredString(entry, "evidence");
  const evidence = fs.readFileSync(path.join(ROOT, evidencePath), "utf8");
  for (const [label, value] of values) {
    assert.ok(evidence.includes(value), `${entry.id} evidence is missing ${label}: ${value}`);
  }
}

function main() {
  const entries = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
  const production = entries.filter((entry) => entry.provenance === "production");
  assert.ok(production.length > 0, "showcase manifest must retain production provenance");

  const fetched = new Set();
  for (const entry of production) {
    const originRepository = requiredString(entry, "originRepository");
    const commit = requiredString(entry, "originCommit");
    const originArtifact = requiredString(entry, "originArtifact");
    const originSource = requiredString(entry, "originSource");
    const artifact = requiredString(entry, "artifact");
    const source = requiredString(entry, "source");
    const artifactGitBlob = requiredString(entry, "artifactGitBlob");
    const sourceGitBlob = requiredString(entry, "sourceGitBlob");
    const consumerRepository = requiredString(entry, "consumerRepository");
    const consumerCommit = requiredString(entry, "consumerCommit");
    const consumerPath = requiredString(entry, "consumerPath");
    const consumerGitBlob = requiredString(entry, "consumerGitBlob");
    const consumerAttestation = requiredString(entry, "consumerAttestation");
    const consumerEvidence = requiredString(entry, "consumerEvidence");
    const consumerEvidenceGitBlob = requiredString(entry, "consumerEvidenceGitBlob");
    const animation = requiredString(entry, "animation");

    assert.match(commit, /^[0-9a-f]{40}$/, `${entry.id}.originCommit must be a full Git SHA`);
    assert.match(consumerCommit, /^[0-9a-f]{40}$/, `${entry.id}.consumerCommit must be a full Git SHA`);
    assert.match(artifactGitBlob, /^[0-9a-f]{40}$/, `${entry.id}.artifactGitBlob must be a Git blob SHA`);
    assert.match(sourceGitBlob, /^[0-9a-f]{40}$/, `${entry.id}.sourceGitBlob must be a Git blob SHA`);
    assert.match(consumerGitBlob, /^[0-9a-f]{40}$/, `${entry.id}.consumerGitBlob must be a Git blob SHA`);
    assert.match(
      consumerEvidenceGitBlob,
      /^[0-9a-f]{40}$/,
      `${entry.id}.consumerEvidenceGitBlob must be a Git blob SHA`
    );

    if (!fetched.has(commit)) {
      git(["fetch", "--quiet", "--no-tags", "--depth=1", "origin", commit]);
      fetched.add(commit);
    }

    const upstreamArtifact = upstreamBlobSha(commit, originArtifact);
    const upstreamSource = upstreamBlobSha(commit, originSource);
    assert.equal(upstreamArtifact, artifactGitBlob, `${entry.id} artifact pin diverged from origin commit`);
    assert.equal(upstreamSource, sourceGitBlob, `${entry.id} source pin diverged from origin commit`);
    assert.equal(localBlobSha(artifact), upstreamArtifact, `${entry.id} local artifact diverged from origin commit`);
    assert.equal(localBlobSha(source), upstreamSource, `${entry.id} local source diverged from origin commit`);
    assert.equal(
      localBlobSha(consumerEvidence),
      consumerEvidenceGitBlob,
      `${entry.id} retained consumer evidence diverged from its blob pin`
    );

    const expectedArtifactUrl = `https://raw.githubusercontent.com/${originRepository}/${commit}/${originArtifact}`;
    const retainedConsumer = fs.readFileSync(path.join(ROOT, consumerEvidence), "utf8");
    assert.equal(
      quotedConstant(retainedConsumer, "RIVE_FILE_URL", entry.id),
      expectedArtifactUrl,
      `${entry.id} retained consumer evidence no longer pins the production artifact`
    );
    assert.equal(
      quotedConstant(retainedConsumer, "RIVE_ANIMATION", entry.id),
      animation,
      `${entry.id} retained consumer evidence no longer requests the production animation`
    );

    const attestation = JSON.parse(fs.readFileSync(path.join(ROOT, consumerAttestation), "utf8"));
    assert.equal(attestation.repository, consumerRepository, `${entry.id} attestation repository diverged`);
    assert.equal(attestation.commit, consumerCommit, `${entry.id} attestation commit diverged`);
    assert.equal(attestation.path, consumerPath, `${entry.id} attestation path diverged`);
    assert.equal(attestation.git_blob, consumerGitBlob, `${entry.id} attestation consumer blob diverged`);
    assert.equal(
      attestation.retained_evidence?.path,
      consumerEvidence,
      `${entry.id} attestation consumer evidence path diverged`
    );
    assert.equal(
      attestation.retained_evidence?.git_blob,
      consumerEvidenceGitBlob,
      `${entry.id} attestation consumer evidence blob diverged`
    );
    assert.equal(
      attestation.observed?.rive_file_url,
      expectedArtifactUrl,
      `${entry.id} attestation no longer records the retained production artifact`
    );
    assert.equal(
      attestation.observed?.rive_animation,
      animation,
      `${entry.id} attestation no longer records the retained animation`
    );

    assertEvidence(entry, [
      ["origin repository", originRepository],
      ["origin commit", commit],
      ["origin artifact path", originArtifact],
      ["origin source path", originSource],
      ["artifact blob", upstreamArtifact],
      ["source blob", upstreamSource],
      ["consumer repository", consumerRepository],
      ["consumer commit", consumerCommit],
      ["consumer path", consumerPath],
      ["consumer blob", consumerGitBlob],
      ["consumer attestation", consumerAttestation],
      ["retained consumer evidence", consumerEvidence],
      ["retained consumer evidence blob", consumerEvidenceGitBlob],
      ["consumer artifact URL", expectedArtifactUrl],
      ["consumer animation", animation],
    ]);
  }

  process.stdout.write(
    `Verified ${production.length} production showcase provenance record(s): immutable rive-cli origin plus hashed minimal consumer evidence\n`
  );
}

try {
  main();
} catch (error) {
  console.error(error);
  process.exit(1);
}
