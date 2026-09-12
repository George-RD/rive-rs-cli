import unittest

from schema_facts import SchemaError, normalize_type, parse_type, schema_changes, type_names


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


class SchemaCaptureShapeContract(unittest.TestCase):
    def test_single_and_zero_property_types_are_not_treated_as_missing_evidence(self):
        single = 'Animation  (typeKey 27)\nextends ArtboardProvider\n\nown properties:\n  name   String   key 55\n\n1 property.\n'
        empty = 'Backboard  (typeKey 23)\n\nno matching properties\n'
        self.assertEqual(len(normalize_type(single, single)['properties']), 1)
        self.assertEqual(normalize_type(empty, empty)['properties'], [])

    def test_shadowed_inherited_names_keep_both_property_identities(self):
        text = 'LibraryArtboard  (typeKey 549)\nextends Asset\n\nown properties:\n  name   String  key 794\n\ninherited from Asset:\n  name   String  key 203\n\n2 properties.\n'
        actual = normalize_type(text, text)
        self.assertEqual({(field['owner'], field['name'], field['key']) for field in actual['properties']},
                         {('LibraryArtboard', 'name', 794), ('Asset', 'name', 203)})

    def test_incomplete_and_cross_type_captures_fail(self):
        for invalid in [RUNTIME.replace('3 properties.', '4 properties.'),
                        RUNTIME.replace('key 781', 'key not-a-key'),
                        RUNTIME.replace('inherited from Drawable:', 'inherited via Drawable:'),
                        RUNTIME.replace('3 properties.', ''),
                        RUNTIME.replace('Shape  (typeKey 3)', 'Rectangle  (typeKey 7)')]:
            with self.subTest(invalid=invalid), self.assertRaises(SchemaError):
                normalize_type(invalid, ALL)

    def test_property_prose_is_not_republished_but_malformed_fact_markers_fail(self):
        prose = RUNTIME.replace('3 properties.', '      Original test-only property explanation.\n\n3 properties.')
        self.assertEqual(normalize_type(prose, ALL), normalize_type(RUNTIME, ALL))
        for line in ['      accepts = first, second', '      accepts: ',
                     '      bits malformed', '      bits (flags): ']:
            invalid = RUNTIME.replace('3 properties.', line + '\n\n3 properties.')
            with self.subTest(line=line), self.assertRaises(SchemaError):
                parse_type(invalid)

    def test_type_list_requires_unique_names_and_a_complete_count(self):
        self.assertEqual(type_names('Shape\nRectangle\n\n2 types.\n'), ['Rectangle', 'Shape'])
        for invalid in ['Shape\n2 types.', 'Shape\nShape\n2 types.', '../private\n1 types.', '']:
            with self.subTest(invalid=invalid), self.assertRaises(SchemaError):
                type_names(invalid)


class SchemaDriftContract(unittest.TestCase):
    def test_key_default_enum_and_removed_properties_are_reported(self):
        import copy
        before = {'Shape': normalize_type(RUNTIME, ALL)}
        after = copy.deepcopy(before)
        fields = {field['name']: field for field in after['Shape']['properties']}
        fields['blendModeValue']['key'] = 999
        fields['blendModeValue']['default_literal'] = 'screen'
        fields['blendModeValue']['enum_values'].append('multiply')
        after['Shape']['properties'] = [field for field in after['Shape']['properties'] if field['name'] != 'length']
        changes = schema_changes(before, after)
        paths = {row['path'] for row in changes}
        self.assertEqual(paths, {'Shape.properties.Drawable.blendModeValue.key',
                                'Shape.properties.Drawable.blendModeValue.default_literal',
                                'Shape.properties.Drawable.blendModeValue.enum_values',
                                'Shape.properties.Shape.length'})
        self.assertEqual(schema_changes(before, before), [])
        self.assertEqual(schema_changes({}, before)[0]['change'], 'added')
        self.assertEqual(schema_changes(before, {})[0]['change'], 'removed')


if __name__ == '__main__':
    unittest.main()
