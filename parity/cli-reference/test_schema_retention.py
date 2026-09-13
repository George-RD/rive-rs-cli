import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

import reference
from schema_snapshot import read_snapshot, write_snapshot


HERE = Path(__file__).resolve().parent


class RetentionCommandContract(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.base = self.root / 'base'
        self.candidate = self.root / 'candidate'
        for root in [self.base, self.candidate]:
            target = root / 'parity/cli-reference/schema-baseline'
            target.mkdir(parents=True)
            for name in ['snapshot.json', 'facts.json.xz']:
                shutil.copyfile(HERE / 'schema-baseline' / name, target / name)
        self.initial_digest = self.pin(self.base)['archive_sha256']

    def directory(self, root):
        return root / 'parity/cli-reference/schema-baseline'

    def pin(self, root):
        return json.loads((self.directory(root) / 'snapshot.json').read_text())

    def check(self, *extra):
        return subprocess.run([sys.executable, str(HERE / 'schema_retention.py'),
                               '--base-root', str(self.base), '--candidate-root', str(self.candidate), *extra],
                              capture_output=True, text=True)

    def replace_candidate(self):
        directory = self.directory(self.candidate)
        archive = directory / 'facts.json.xz'
        value = read_snapshot(archive)
        value['types']['Shape']['type_key'] = 65500
        archive.unlink()
        write_snapshot(archive, value)
        pin = self.pin(self.candidate)
        pin['archive_sha256'] = reference.digest(archive)
        reference.write_json(directory / 'snapshot.json', pin)
        return pin['archive_sha256']

    def test_rehashing_changed_baseline_cannot_bypass_the_base_comparison(self):
        self.replace_candidate()
        result = self.check('--reviewed-change', 'absent:' + self.initial_digest)
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        report = json.loads(result.stdout)
        self.assertFalse(report['ok'])
        self.assertFalse(report['reviewed_change'])
        self.assertEqual([row['path'] for row in report['schema_changes']], ['Shape.type_key'])
        self.assertEqual(read_snapshot(self.directory(self.base) / 'facts.json.xz')['types']['Shape']['type_key'], 3)

    def test_identical_retention_needs_no_exception(self):
        result = self.check()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertFalse(json.loads(result.stdout)['changed'])

    def test_reviewed_change_is_bound_to_both_digests_and_still_reports_changes(self):
        digest = self.replace_candidate()
        wrong = self.check('--reviewed-change', 'f' * 64 + ':' + digest)
        self.assertEqual(wrong.returncode, 1, wrong.stdout + wrong.stderr)
        result = self.check('--reviewed-change', self.initial_digest + ':' + digest)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        report = json.loads(result.stdout)
        self.assertTrue(report['changed'])
        self.assertTrue(report['reviewed_change'])
        self.assertEqual([row['path'] for row in report['schema_changes']], ['Shape.type_key'])

    def test_initial_retention_requires_the_explicit_candidate_digest(self):
        shutil.rmtree(self.directory(self.base))
        for arguments in [[], ['--reviewed-change', 'absent:' + 'f' * 64]]:
            with self.subTest(arguments=arguments):
                result = self.check(*arguments)
                self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        result = self.check('--reviewed-change', 'absent:' + self.initial_digest)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertTrue(json.loads(result.stdout)['initial_retention'])
        self.replace_candidate()
        result = self.check('--reviewed-change', 'absent:' + self.initial_digest)
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)

    def test_manifest_changes_are_not_silently_accepted(self):
        pin = self.pin(self.candidate)
        pin['workflow_run'] += 1
        reference.write_json(self.directory(self.candidate) / 'snapshot.json', pin)
        result = self.check()
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertEqual([row['path'] for row in json.loads(result.stdout)['manifest_changes']],
                         ['manifest.workflow_run'])

    def test_incomplete_base_is_not_treated_as_an_initial_retention(self):
        (self.directory(self.base) / 'snapshot.json').unlink()
        result = self.check('--reviewed-change', 'absent:' + self.initial_digest)
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn('incomplete retained snapshot', result.stderr)

    def test_manifest_source_and_duplicate_members_are_validated(self):
        path = self.directory(self.candidate) / 'snapshot.json'
        original = path.read_text()
        for changed in [original.replace('"source_head": "', '"source_head": "extra'),
                        original.replace('"schema_version": 1', '"schema_version": 999, "schema_version": 1')]:
            path.write_text(changed)
            with self.subTest(changed=changed):
                result = self.check()
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                self.assertNotIn('Traceback', result.stderr)


if __name__ == '__main__':
    unittest.main()
