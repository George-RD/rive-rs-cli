import hashlib
import json
import lzma
import re
from pathlib import Path

import reference
from schema_facts import SchemaError, normalize_type, type_names


MAX_CAPTURE_FILES = 5000
MAX_SNAPSHOT_BYTES = 16 * 1024 * 1024
SNAPSHOT_VERSION = 1


def canonical_bytes(value: dict) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False) + '\n').encode()


def from_capture(directory: Path, expected_head: str, lock_path: Path | None = None) -> dict:
    recording = reference.read_json(directory / 'schema-capture.json')
    lock_path = lock_path or reference.HERE / 'lock.json'
    lock = reference.read_json(lock_path)
    if (type(recording.get('schema_version')) is not int or recording.get('schema_version') != SNAPSHOT_VERSION or recording.get('status') != 'passed'
            or not re.fullmatch(r'[0-9a-f]{40}', expected_head)
            or recording.get('source_head') != expected_head):
        raise SchemaError('schema capture is incomplete or has an unexpected source head')
    if recording.get('lock_sha256') != reference.digest(lock_path):
        raise SchemaError('schema capture has a different reference lock')
    official = recording.get('official', {})
    if any(official.get(key) != lock['official'][key] for key in ['source', 'archive_sha256']):
        raise SchemaError('schema capture has a different official archive')
    if not re.fullmatch(r'[0-9a-f]{64}', official.get('binary_sha256', '')):
        raise SchemaError('schema capture lacks the executable identity')
    reference.verify_version(official.get('version_output', ''), lock['official']['version'])
    artifacts = recording.get('artifacts', {})
    actual = {str(path.relative_to(directory)) for path in directory.rglob('*')
              if path.is_file() and path != directory / 'schema-capture.json'}
    if len(actual) > MAX_CAPTURE_FILES or sum((directory / name).stat().st_size for name in actual) > MAX_SNAPSHOT_BYTES:
        raise SchemaError('schema capture exceeds the evidence budget')
    if set(artifacts) != actual:
        raise SchemaError('schema capture has missing or unindexed evidence')
    for name, digest in artifacts.items():
        if reference.digest(reference.checked_relative(directory, name)) != digest:
            raise SchemaError(f'schema evidence digest mismatch: {name}')
    commands = {}
    for command in recording.get('commands', []):
        name = command.get('name')
        if not isinstance(name, str) or name in commands or type(command.get('exit_code')) is not int or command['exit_code'] != 0:
            raise SchemaError('schema capture has duplicate or unsuccessful commands')
        commands[name] = command
        for channel in ['stdout', 'stderr']:
            filename = f'commands/{name}.{channel}'
            if (command.get(channel + '_file') != filename or filename not in artifacts
                    or command.get(channel + '_sha256') != artifacts[filename]):
                raise SchemaError('schema command is not bound to its retained output')
    for name in ['head', 'worktree', 'version', 'types']:
        if name not in commands:
            raise SchemaError(f'missing schema command: {name}')
    if commands['head'].get('argv') != ['git', 'rev-parse', 'HEAD'] or commands['worktree'].get('argv') != ['git', 'status', '--porcelain']:
        raise SchemaError('schema capture has different source/cleanliness commands')
    if (directory / 'commands/head.stdout').read_text().strip() != expected_head:
        raise SchemaError('source head disagrees with recorded command output')
    if (directory / 'commands/worktree.stdout').read_text().strip():
        raise SchemaError('schema capture was made from a dirty checkout')
    version_output = (directory / 'commands/version.stdout').read_text().strip()
    if version_output != official['version_output']:
        raise SchemaError('schema version disagrees with recorded command output')
    prefix = commands['version'].get('argv', [])[:-1]
    if (prefix[:len(reference.OFFLINE_PREFIX)] != reference.OFFLINE_PREFIX
            or len(prefix) != len(reference.OFFLINE_PREFIX) + 1
            or commands['version']['argv'][-1] != '--version'):
        raise SchemaError('schema commands lack the pinned offline execution prefix')
    if commands['types'].get('argv') != prefix + ['schema', '--list']:
        raise SchemaError('schema inventory command differs')
    names = type_names((directory / 'commands/types.stdout').read_text())
    expected_commands = {'head', 'worktree', 'version', 'types'}
    types = {}
    raw_digests = {}
    for name in names:
        texts = []
        for mode, flags in [('runtime', []), ('all', ['--all'])]:
            command_name = f'{name}-{mode}'
            expected_commands.add(command_name)
            if commands.get(command_name, {}).get('argv') != prefix + ['schema', name] + flags:
                raise SchemaError(f'missing or wrong schema probe: {command_name}')
            path = directory / f'commands/{command_name}.stdout'
            texts.append(path.read_text())
            raw_digests[command_name] = reference.digest(path)
        normalized = normalize_type(*texts)
        if normalized['name'] != name:
            raise SchemaError('schema response has a different type identity')
        types[name] = normalized
    if set(commands) != expected_commands:
        raise SchemaError('schema capture has unexpected commands')
    return {'schema_version': SNAPSHOT_VERSION, 'types': types,
            'provenance': {'official': dict(lock['official'], binary_sha256=official['binary_sha256']),
                           'source_head': expected_head,
                           'lock_sha256': recording['lock_sha256'],
                           'recording_sha256': reference.digest(directory / 'schema-capture.json'),
                           'raw_output_inventory_sha256': hashlib.sha256(canonical_bytes(raw_digests)).hexdigest(),
                           'attribution': 'Rive, Inc. Public schema facts from the pinned CLI.',
                           'retention': 'Normalized facts, not documentation, samples, executable or installation.',
                           'license_scope': 'Original tooling uses the root MIT license. No license to Rive tools or distribution rights is inferred.'}}


def write_snapshot(path: Path, value: dict) -> None:
    data = canonical_bytes(value)
    if len(data) > MAX_SNAPSHOT_BYTES:
        raise SchemaError('snapshot exceeds the size budget')
    if path.suffix == '.xz':
        data = lzma.compress(data, preset=9)
    with path.open('xb') as stream:
        stream.write(data)


def read_snapshot(path: Path, expected_digest: str | None = None) -> dict:
    if expected_digest is not None and reference.digest(path) != expected_digest:
        raise SchemaError('snapshot digest mismatch; refresh requires review')
    opener = lzma.open if path.suffix == '.xz' else open
    with opener(path, 'rb') as stream:
        data = stream.read(MAX_SNAPSHOT_BYTES + 1)
    if len(data) > MAX_SNAPSHOT_BYTES:
        raise SchemaError('snapshot exceeds the size budget')
    value = json.loads(data)
    if (not isinstance(value, dict) or value.get('schema_version') != SNAPSHOT_VERSION
            or not isinstance(value.get('types'), dict) or not value['types']
            or not isinstance(value.get('provenance'), dict)):
        raise SchemaError('invalid schema snapshot')
    for name, row in value['types'].items():
        if (not isinstance(row, dict) or row.get('name') != name or type(row.get('type_key')) is not int
                or not isinstance(row.get('inherits'), list) or not isinstance(row.get('properties'), list)):
            raise SchemaError(f'invalid snapshot type: {name}')
        identities = set()
        for field in row['properties']:
            if (not isinstance(field, dict) or not isinstance(field.get('name'), str)
                    or not isinstance(field.get('owner'), str) or type(field.get('key')) is not int
                    or any(type(field.get(flag)) is not bool for flag in ['animatable', 'bindable', 'derived', 'hidden_without_all'])):
                raise SchemaError(f'invalid snapshot property: {name}')
            identity = (field['owner'], field['name'])
            if identity in identities:
                raise SchemaError(f'duplicate snapshot property: {name}')
            identities.add(identity)
    return value
