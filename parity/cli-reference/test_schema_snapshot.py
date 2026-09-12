import copy
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import reference
from compiler_metadata import read_metadata
from schema_facts import SchemaError
from schema_snapshot import from_capture, read_snapshot, write_snapshot
from test_schema_facts import RUNTIME, ALL


HEAD = '1' * 40


def synthetic_capture(root):
    lock = reference.read_json(reference.HERE / 'lock.json')
    prefix = reference.OFFLINE_PREFIX + ['/synthetic/rive']
    specs = [('head', ['git', 'rev-parse', 'HEAD'], HEAD + '\n'),
             ('worktree', ['git', 'status', '--porcelain'], ''),
             ('version', prefix + ['--version'], 'rive ' + lock['official']['version'] + '\n'),
             ('types', prefix + ['schema', '--list'], 'Shape\n\n1 types.\n'),
             ('Shape-runtime', prefix + ['schema', 'Shape'], RUNTIME),
             ('Shape-all', prefix + ['schema', 'Shape', '--all'], ALL)]
    (root / 'commands').mkdir()
    commands, artifacts = [], {}
    for name, argv, output in specs:
        row = {'name': name, 'argv': argv, 'exit_code': 0}
        for channel, text in [('stdout', output), ('stderr', '')]:
            file = f'commands/{name}.{channel}'
            (root / file).write_text(text)
            digest = reference.digest(root / file)
            artifacts[file] = digest
            row[channel + '_file'] = file
            row[channel + '_sha256'] = digest
        commands.append(row)
    recording = {'schema_version': 1, 'status': 'passed', 'source_head': HEAD,
                 'lock_sha256': reference.digest(reference.HERE / 'lock.json'),
                 'official': {'source': lock['official']['source'], 'archive_sha256': lock['official']['archive_sha256'],
                              'binary_sha256': '2' * 64, 'version_output': 'rive ' + lock['official']['version']},
                 'commands': commands, 'artifacts': artifacts}
    reference.write_json(root / 'schema-capture.json', recording)
    return recording


class SchemaSnapshotContract(unittest.TestCase):
    def test_capture_normalization_is_offline_and_candidate_writes_do_not_overwrite(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            synthetic_capture(root)
            with patch('subprocess.run', side_effect=AssertionError('offline normalization executed a tool')):
                result = from_capture(root, HEAD)
                result['compiler_metadata'] = read_metadata(reference.ROOT)
            self.assertEqual(result['types']['Shape']['type_key'], 3)
            output = root / 'candidate.json.xz'
            write_snapshot(output, result)
            self.assertEqual(read_snapshot(output), result)
            with self.assertRaises(FileExistsError):
                write_snapshot(output, result)

    def test_candidate_requires_complete_typed_property_facts(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            synthetic_capture(root)
            original = from_capture(root, HEAD)
            original['compiler_metadata'] = read_metadata(reference.ROOT)
            candidate = root / 'candidate.json'
            mutations = [lambda value: value.update(schema_version=True),
                         lambda value: value['types']['Shape']['properties'][0].pop('default_literal'),
                         lambda value: value['types']['Shape']['properties'][0].update(enum_values='srcOver'),
                         lambda value: value['types']['Shape']['properties'][0].update(default_literal=1),
                         lambda value: value['types']['Shape']['properties'][0].update(key=True)]
            for index, mutate in enumerate(mutations):
                value = copy.deepcopy(original)
                mutate(value)
                candidate.write_text(json.dumps(value))
                with self.subTest(index=index), self.assertRaises(SchemaError):
                    read_snapshot(candidate)

    def test_snapshot_rejects_unrecognized_top_level_members(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            synthetic_capture(root)
            original = from_capture(root, HEAD)
            original['compiler_metadata'] = read_metadata(reference.ROOT)
            for suffix in ['.json', '.json.xz']:
                valid = root / ('valid' + suffix)
                write_snapshot(valid, original)
                self.assertEqual(read_snapshot(valid), original)
                for name, value in [('runtime_tested', True), ('extra', None)]:
                    candidate = root / (name + suffix)
                    write_snapshot(candidate, dict(original, **{name: value}))
                    with self.subTest(name=name, suffix=suffix):
                        with self.assertRaisesRegex(SchemaError, 'unrecognized snapshot members'):
                            read_snapshot(candidate)

    def test_snapshot_requires_complete_capture_provenance(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            synthetic_capture(root)
            original = from_capture(root, HEAD)
            original['compiler_metadata'] = read_metadata(reference.ROOT)
            candidate = root / 'candidate.json'
            mutations = [lambda row: row['provenance'].pop('source_head'),
                         lambda row: row['provenance'].update(source_head='not-a-head'),
                         lambda row: row['provenance'].pop('raw_output_inventory_sha256'),
                         lambda row: row['provenance'].update(recording_sha256='not-a-digest'),
                         lambda row: row['provenance'].update(official={}),
                         lambda row: row['provenance']['official'].update(platform='Linux')]
            for index, mutate in enumerate(mutations):
                value = copy.deepcopy(original)
                mutate(value)
                candidate.write_text(json.dumps(value))
                with self.subTest(index=index), self.assertRaises(SchemaError):
                    read_snapshot(candidate)

    def test_failed_duplicate_misidentified_or_nonisolated_commands_cannot_become_evidence(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            original = synthetic_capture(root)
            mutations = [lambda row: row.update(status='failed'),
                         lambda row: row.update(source_head='3' * 40),
                         lambda row: row['commands'][0].update(argv=['echo', HEAD]),
                         lambda row: row['commands'][1].update(argv=['true']),
                         lambda row: row['commands'][2].update(argv=['/synthetic/rive', '--version']),
                         lambda row: row['commands'][-1].update(argv=['/synthetic/rive', 'schema', 'Shape']),
                         lambda row: row['commands'][-1].update(exit_code=1),
                         lambda row: row['commands'].append(row['commands'][-1]),
                         lambda row: row['commands'].pop()]
            for index, mutate in enumerate(mutations):
                recording = copy.deepcopy(original)
                mutate(recording)
                reference.write_json(root / 'schema-capture.json', recording)
                with self.subTest(index=index), self.assertRaises((SchemaError, reference.ReferenceError)):
                    from_capture(root, HEAD)

    def test_changed_raw_output_and_unknown_capture_files_fail(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            synthetic_capture(root)
            (root / 'commands/Shape-all.stdout').write_text(ALL.replace('key 781', 'key 900'))
            with self.assertRaises(SchemaError):
                from_capture(root, HEAD)
            (root / 'commands/Shape-all.stdout').write_text(ALL)
            (root / 'extra.txt').write_text('not indexed')
            with self.assertRaises(SchemaError):
                from_capture(root, HEAD)


if __name__ == '__main__':
    unittest.main()
