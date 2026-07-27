#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST="$ROOT/parity/official/manifest.json"
OFFICIAL_DIR="$ROOT/parity/official"
tmp=""

cleanup() {
  if [[ -n "$tmp" && -f "$tmp" ]]; then
    rm -f "$tmp"
  fi
}
trap cleanup EXIT

verify_file() {
  local name="$1" file="$2" expected_sha="$3" expected_bytes="$4"
  local actual_sha actual_bytes

  if [[ ! -f "$file" ]]; then
    echo "$name: missing file $file" >&2
    exit 1
  fi

  actual_sha="$(shasum -a 256 "$file" | awk '{print $1}')"
  actual_bytes="$(wc -c < "$file" | tr -d '[:space:]')"

  if [[ "$actual_sha" != "$expected_sha" ]]; then
    echo "$name: SHA-256 mismatch: expected $expected_sha, got $actual_sha" >&2
    exit 1
  fi
  if [[ "$actual_bytes" != "$expected_bytes" ]]; then
    echo "$name: byte-size mismatch: expected $expected_bytes, got $actual_bytes (SHA-256 expected $expected_sha, got $actual_sha)" >&2
    exit 1
  fi
}

while IFS=$'\x1f' read -r name source upstream_repo upstream_path commit_sha expected_sha expected_bytes; do
  file="$OFFICIAL_DIR/$name"

  if [[ "$source" == "in-repo" ]]; then
    verify_file "$name" "$file" "$expected_sha" "$expected_bytes"
  else
    url="https://raw.githubusercontent.com/$upstream_repo/$commit_sha/$upstream_path"
    tmp="$(mktemp "$OFFICIAL_DIR/.${name}.XXXXXX")"
    if ! curl -sL --fail "$url" -o "$tmp"; then
      echo "$name: failed to download $url" >&2
      exit 1
    fi
    verify_file "$name" "$tmp" "$expected_sha" "$expected_bytes"
    mv "$tmp" "$file"
    tmp=""
  fi

  echo "$name: verified"
done < <(python3 - "$MANIFEST" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as manifest_file:
    for entry in json.load(manifest_file):
        print("\x1f".join(str(entry[field]) if entry[field] is not None else "" for field in (
            "name",
            "source",
            "upstream_repo",
            "upstream_path",
            "commit_sha",
            "sha256",
            "bytes",
        )))
PY
)
