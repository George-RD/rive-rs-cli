import argparse
import json
import re
from pathlib import Path

import reference
from schema_facts import SchemaError, schema_changes, value_changes
from schema_snapshot import SNAPSHOT_VERSION, decode_json, read_snapshot


def read_retained(root: Path):
    directory = root / 'parity/cli-reference/schema-baseline'
    manifest = directory / 'snapshot.json'
    if not manifest.exists() and not (directory / 'facts.json.xz').exists():
        return None
    if not manifest.is_file():
        raise SchemaError('incomplete retained snapshot')
    pin = decode_json(manifest.read_bytes())
    required = {'schema_version', 'archive', 'archive_sha256', 'source_head',
                'workflow_run', 'workflow_artifact', 'artifact_zip_sha256'}
    if (not isinstance(pin, dict) or set(pin) != required
            or type(pin['schema_version']) is not int or pin['schema_version'] != SNAPSHOT_VERSION
            or not isinstance(pin['archive'], str) or not pin['archive'].strip()
            or not isinstance(pin['source_head'], str)
            or not re.fullmatch(r'[0-9a-f]{40}', pin['source_head'])
            or any(type(pin[key]) is not int or pin[key] <= 0
                   for key in ['workflow_run', 'workflow_artifact'])
            or any(not isinstance(pin[key], str) or not re.fullmatch(r'[0-9a-f]{64}', pin[key])
                   for key in ['archive_sha256', 'artifact_zip_sha256'])):
        raise SchemaError('invalid retained snapshot manifest')
    path = reference.checked_relative(directory, pin['archive'])
    snapshot = read_snapshot(path, pin['archive_sha256'])
    if pin.get('source_head') != snapshot['provenance']['source_head']:
        raise SchemaError('manifest source head differs from retained snapshot')
    return pin, snapshot


def compare_retention(base_root: Path, candidate_root: Path, reviewed_change: str | None) -> dict:
    if reviewed_change is not None and not re.fullmatch(r'(?:absent|[0-9a-f]{64}):[0-9a-f]{64}', reviewed_change):
        raise SchemaError('reviewed change requires OLD_SHA256:NEW_SHA256 or absent:NEW_SHA256')
    before = read_retained(base_root)
    after = read_retained(candidate_root)
    if after is None:
        raise SchemaError('candidate has no retained snapshot')
    pin, candidate = after
    old_digest = before[0]['archive_sha256'] if before is not None else 'absent'
    expected = old_digest + ':' + pin['archive_sha256']
    result = {'base_sha256': old_digest, 'candidate_sha256': pin['archive_sha256'],
              'initial_retention': before is None, 'reviewed_change': reviewed_change == expected,
              'schema_changes': [], 'compiler_changes': [], 'provenance_changes': [],
              'manifest_changes': [], 'inventory': {'types': len(candidate['types'])}}
    if before is not None:
        old_pin, baseline = before
        result['schema_changes'] = schema_changes(baseline['types'], candidate['types'])
        result['compiler_changes'] = value_changes(baseline['compiler_metadata'], candidate['compiler_metadata'], 'compiler')
        result['provenance_changes'] = value_changes(baseline['provenance'], candidate['provenance'], 'provenance')
        result['manifest_changes'] = value_changes(old_pin, pin, 'manifest')
    changed = before is None or any(result[key] for key in
                                   ['schema_changes', 'compiler_changes', 'provenance_changes', 'manifest_changes'])
    result['changed'] = bool(changed)
    result['ok'] = not changed or result['reviewed_change']
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description='Compare retained evidence with the pre-change repository snapshot.')
    parser.add_argument('--base-root', type=Path, required=True)
    parser.add_argument('--candidate-root', type=Path, default=reference.ROOT)
    parser.add_argument('--reviewed-change')
    args = parser.parse_args()
    try:
        result = compare_retention(args.base_root, args.candidate_root, args.reviewed_change)
        print(json.dumps(result, indent=2, sort_keys=True, allow_nan=False))
        raise SystemExit(0 if result['ok'] else 1)
    except (SchemaError, reference.ReferenceError, OSError, ValueError, KeyError, TypeError) as error:
        parser.exit(2, f'schema retention: {error}\n')


if __name__ == '__main__':
    main()
