import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

from schema_facts import SchemaError, parse_type
from schema_snapshot import read_snapshot


HERE = Path(__file__).resolve().parent


class SchemaContinuationContract(unittest.TestCase):
    def test_repeated_enum_and_bit_continuations_cannot_overwrite_facts(self):
        template = 'Shape  (typeKey 3)\n\nown properties:\n  flags   uint8  key 23 = 0\n{}\n\n1 property.\n'
        for continuation, field in [('      accepts: first, second', 'enum_values'),
                                    ('      bits (flags): first second', 'bits')]:
            valid = template.format(continuation)
            self.assertEqual(parse_type(valid)['properties'][0][field], ['first', 'second'])
            for second in [continuation, continuation.replace('first', 'third')]:
                with self.subTest(field=field, second=second):
                    with self.assertRaisesRegex(SchemaError, 'repeated schema fact continuation'):
                        parse_type(template.format(continuation + '\n' + second))


class SnapshotDuplicateContract(unittest.TestCase):
    def test_duplicate_json_members_are_invalid_at_every_snapshot_depth(self):
        import lzma
        snapshot = read_snapshot(HERE / 'schema-baseline/facts.json.xz')
        text = json.dumps(snapshot)
        mutations = [text.replace('"schema_version": 1', '"schema_version": 999, "schema_version": 1', 1),
                     text.replace('"type_key": 1', '"type_key": 999, "type_key": 1', 1)]
        with tempfile.TemporaryDirectory() as temporary:
            for index, changed in enumerate(mutations):
                self.assertNotEqual(changed, text)
                for suffix in ['.json', '.json.xz']:
                    candidate = Path(temporary) / (str(index) + suffix)
                    candidate.write_bytes(lzma.compress(changed.encode()) if suffix.endswith('.xz') else changed.encode())
                    with self.subTest(index=index, suffix=suffix):
                        result = subprocess.run([sys.executable, str(HERE / 'capabilities.py'), 'check',
                                                 '--candidate', str(candidate), '--json'],
                                                capture_output=True, text=True)
                        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                        self.assertIn('duplicate JSON member', result.stderr)
                        self.assertNotIn('Traceback', result.stderr)


if __name__ == '__main__':
    unittest.main()
