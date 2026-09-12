import copy
import json
import shutil
import tempfile
import subprocess
import sys
import unittest
from pathlib import Path


from compiler_metadata import read_metadata
from schema_snapshot import read_snapshot


HERE = Path(__file__).resolve().parent


class CapabilityCommandContract(unittest.TestCase):
    def test_report_is_deterministic_and_does_not_promote_declared_types(self):
        command = [sys.executable, str(HERE / 'capabilities.py'), 'report', '--json']
        first = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(first.returncode, 0, first.stderr)
        second = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(first.stdout, second.stdout)
        report = json.loads(first.stdout)
        self.assertEqual(report['official']['version'], '1.0.2')
        self.assertEqual(report['summary']['official_types'], 351)
        self.assertIn('3d', report['families'])
        self.assertIn('shaders', report['families'])
        shape = report['types']['Shape']['official_schema']
        blend = next(field for field in shape['properties'] if field['name'] == 'blendModeValue')
        self.assertEqual(blend['key'], 23)
        self.assertEqual(blend['default_literal'], 'srcOver')
        self.assertIn('screen', blend['enum_values'])
        shader = report['types']['ShaderAsset']
        self.assertTrue(shader['stages']['declared'])
        for stage in ['parsed', 'lowered', 'encoded', 'runtime_tested', 'semantic_tested']:
            self.assertIsNone(shader['stages'][stage], stage)
        self.assertEqual(report['evidence']['pipeline'], 'official-cli-reference')
        self.assertTrue(report['evidence']['cases']['static']['stages']['runtime_tested'])
        self.assertIsNone(report['evidence']['cases']['static']['stages']['lowered'])
        self.assertFalse(report['evidence']['claims_our_rml_support'])
        self.assertEqual({row['issue'] for row in report['known_work']},
                         {123, 124, 125, 126, 127, 128, 175, 252, 254, 255, 267})


class CapabilityMutationContract(unittest.TestCase):
    def fixture_checkout(self, directory):
        root = Path(directory)
        destination = root / 'parity/cli-reference'
        shutil.copytree(HERE, destination, ignore=shutil.ignore_patterns('__pycache__'))
        (root / 'docs').mkdir()
        shutil.copyfile(HERE.parents[1] / 'docs/scene.schema.v1.json', root / 'docs/scene.schema.v1.json')
        (root / 'src/objects').mkdir(parents=True)
        sources = read_metadata(HERE.parents[1])['provenance']
        registry = next(name for name in sources if name.endswith('generated_registry.rs'))
        shutil.copyfile(HERE.parents[1] / registry, root / 'src/objects/generated_registry.rs')
        return root, destination

    def run_check(self, here, *arguments):
        return subprocess.run([sys.executable, str(here / 'capabilities.py'), 'check', '--json', *arguments],
                              capture_output=True, text=True)

    def test_property_key_default_enum_mutations_fail_the_public_check_without_rewriting_baseline(self):
        baseline = HERE / 'schema-baseline/facts.json.xz'
        original = baseline.read_bytes()
        snapshot = read_snapshot(baseline)
        with tempfile.TemporaryDirectory() as temporary:
            candidate = Path(temporary) / 'candidate.json'
            for field_name, value in [('key', 65500), ('default_literal', 'screen'), ('enum_values', ['new_enum'])]:
                changed = copy.deepcopy(snapshot)
                prop = next(field for field in changed['types']['Shape']['properties'] if field['name'] == 'blendModeValue')
                prop[field_name] = value
                candidate.write_text(json.dumps(changed))
                result = self.run_check(HERE, '--candidate', str(candidate))
                self.assertEqual(result.returncode, 1, result.stderr)
                changes = json.loads(result.stdout)['schema_changes']
                self.assertEqual([row['path'] for row in changes], [f'Shape.properties.Drawable.blendModeValue.{field_name}'])
        self.assertEqual(baseline.read_bytes(), original)

    def test_candidate_requires_complete_compiler_metadata(self):
        snapshot = read_snapshot(HERE / 'schema-baseline/facts.json.xz')
        original = (HERE / 'schema-baseline/facts.json.xz').read_bytes()
        mutations = [lambda row: row.pop('compiler_metadata'),
                     lambda row: row.update(compiler_metadata=None),
                     lambda row: row.update(compiler_metadata={}),
                     lambda row: row['compiler_metadata'].update(registered_types=[]),
                     lambda row: row['compiler_metadata']['registered_properties'].update({'23': 23}),
                     lambda row: row['compiler_metadata']['canonical'].pop('objects'),
                     lambda row: row['compiler_metadata']['canonical']['objects']['shape'].update(properties={}),
                     lambda row: row['compiler_metadata'].update(provenance={}),
                     lambda row: row['compiler_metadata']['provenance'].update({'docs/scene.schema.v1.json': 'not-a-digest'})]
        with tempfile.TemporaryDirectory() as temporary:
            candidate = Path(temporary) / 'candidate.json'
            for index, mutate in enumerate(mutations):
                changed = copy.deepcopy(snapshot)
                mutate(changed)
                candidate.write_text(json.dumps(changed))
                with self.subTest(index=index):
                    result = self.run_check(HERE, '--candidate', str(candidate))
                    self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                    self.assertIn('compiler metadata', result.stderr)
                    self.assertNotIn('Traceback', result.stderr)
        self.assertEqual((HERE / 'schema-baseline/facts.json.xz').read_bytes(), original)

    def test_candidate_compiler_drift_is_compared_even_when_current_checkout_is_unchanged(self):
        snapshot = read_snapshot(HERE / 'schema-baseline/facts.json.xz')
        snapshot['compiler_metadata']['registered_types']['7'] = 'CandidateRectangle'
        with tempfile.TemporaryDirectory() as temporary:
            candidate = Path(temporary) / 'candidate.json'
            candidate.write_text(json.dumps(snapshot))
            result = self.run_check(HERE, '--candidate', str(candidate))
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        comparison = json.loads(result.stdout)
        self.assertEqual(comparison['compiler_changes'], [])
        self.assertEqual([row['path'] for row in comparison['candidate_compiler_changes']],
                         ['compiler.registered_types.7'])

    def test_changed_source_fixture_invalidates_retained_runtime_evidence(self):
        with tempfile.TemporaryDirectory() as temporary:
            _, here = self.fixture_checkout(temporary)
            source = here / 'fixtures/static/scene.rml'
            source.write_text(source.read_text().replace('FF2E8BC0', 'FF000000'))
            result = self.run_check(here)
            self.assertEqual(result.returncode, 2)
            self.assertIn('reference input differs', result.stderr)

    def test_current_registry_key_changes_are_detected_and_crate_moves_are_not_semantic_drift(self):
        with tempfile.TemporaryDirectory() as temporary:
            root, here = self.fixture_checkout(temporary)
            registry = root / 'src/objects/generated_registry.rs'
            moved = root / 'crates/compiler/src/objects/generated_registry.rs'
            moved.parent.mkdir(parents=True)
            registry.rename(moved)
            unchanged = self.run_check(here)
            self.assertEqual(unchanged.returncode, 0, unchanged.stderr)
            moved.write_text(moved.read_text().replace('7 => Some("Rectangle")', '65500 => Some("Rectangle")'))
            changed = self.run_check(here)
            self.assertEqual(changed.returncode, 1, changed.stderr)
            paths = {row['path'] for row in json.loads(changed.stdout)['compiler_changes']}
            self.assertEqual(paths, {'compiler.registered_types.7', 'compiler.registered_types.65500'})

    def test_tampered_snapshot_is_not_accepted_as_new_baseline(self):
        with tempfile.TemporaryDirectory() as temporary:
            _, here = self.fixture_checkout(temporary)
            archive = here / 'schema-baseline/facts.json.xz'
            archive.write_bytes(archive.read_bytes() + b'x')
            result = self.run_check(here)
            self.assertEqual(result.returncode, 2)
            self.assertIn('snapshot digest mismatch', result.stderr)


if __name__ == '__main__':
    unittest.main()
