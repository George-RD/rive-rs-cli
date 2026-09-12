import unittest

from schema_facts import normalize_type


RUNTIME = '''Shape  (typeKey 3)
extends Drawable > Node

own properties:
  length                         BD  double       key 781     = 0

inherited from Drawable:
  blendModeValue                     uint8        key 23      = srcOver
      accepts: srcOver, screen, overlay

inherited from Node:
  name                               String       key 4

3 properties.  A = animatable, B = bindable, D = derived (computed, not authored -- bind to it, do not set it).
Editor-only properties are hidden; --all shows them.
'''
ALL = RUNTIME.replace('3 properties.', '''  childOrder                         FractionalIndex key 6

4 properties.''')


class SchemaFactsContract(unittest.TestCase):
    def test_public_schema_normalization_preserves_unknowns_and_field_facts(self):
        actual = normalize_type(RUNTIME, ALL)
        self.assertEqual(actual['name'], 'Shape')
        self.assertEqual(actual['type_key'], 3)
        self.assertEqual(actual['inherits'], ['Drawable', 'Node'])
        fields = {row['name']: row for row in actual['properties']}
        self.assertEqual(fields['blendModeValue']['enum_values'], ['srcOver', 'screen', 'overlay'])
        self.assertEqual(fields['blendModeValue']['default_literal'], 'srcOver')
        self.assertEqual(fields['blendModeValue']['owner'], 'Drawable')
        self.assertTrue(fields['length']['derived'])
        self.assertTrue(fields['length']['bindable'])
        self.assertFalse(fields['length']['animatable'])
        self.assertIsNone(fields['name']['default_literal'])
        self.assertIsNone(fields['name']['enum_values'])
        self.assertTrue(fields['childOrder']['hidden_without_all'])
        self.assertFalse(fields['name']['hidden_without_all'])


if __name__ == '__main__':
    unittest.main()
