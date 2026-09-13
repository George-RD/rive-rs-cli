import re
from pathlib import Path

import reference
from schema_facts import SchemaError


def registry_table(text: str, function: str) -> dict:
    pattern = (r"pub fn " + function + r"\(key: u16\) -> Option<&'static str> \{\s*match key \{"
               r'(?P<rows>.*?)\s*_ => None,\s*\}\s*\}')
    matches = list(re.finditer(pattern, text, re.DOTALL))
    if len(matches) != 1:
        raise SchemaError(f'cannot locate the generated {function} registry')
    result = {}
    for line in matches[0]['rows'].strip().splitlines():
        match = re.fullmatch(r'\s*(\d+) => Some\("(\w+)"\),\s*', line)
        if match is None or match[1] in result:
            raise SchemaError(f'unrecognized or duplicate entry in {function}')
        result[match[1]] = match[2]
    if not result:
        raise SchemaError(f'empty registry: {function}')
    return result


def read_metadata(root: Path) -> dict:
    registries = [path for base in ['src', 'crates'] for path in (root / base).rglob('generated_registry.rs')]
    if len(registries) != 1:
        raise SchemaError('expected one authoritative generated registry under src/ or crates/')
    registry = registries[0]
    schema_path = root / 'docs/scene.schema.v1.json'
    schema = reference.read_json(schema_path)
    definitions = dict(schema['$defs'])
    object_spec = definitions.pop('ObjectSpec')
    variants = object_spec['oneOf']
    objects = {}
    for row in variants:
        name = row['properties']['type']['const']
        if not isinstance(name, str) or name in objects:
            raise SchemaError('invalid or duplicate canonical object declaration')
        objects[name] = row
    text = registry.read_text()
    result = {'registered_types': registry_table(text, 'type_name'),
            'registered_properties': registry_table(text, 'property_name'),
            'canonical': {'objects': objects, 'definitions': definitions,
                          'root': {key: value for key, value in schema.items() if key != '$defs'},
                          'object_spec': {key: value for key, value in object_spec.items() if key != 'oneOf'}},
            'provenance': {str(path.relative_to(root)): reference.digest(path)
                           for path in [registry, schema_path]}}
    validate_metadata(result)
    return result


def metadata_facts(metadata: dict) -> dict:
    return {key: value for key, value in metadata.items() if key != 'provenance'}


def validate_metadata(metadata: dict) -> None:
    required = {'registered_types', 'registered_properties', 'canonical', 'provenance'}
    if not isinstance(metadata, dict) or set(metadata) != required:
        raise SchemaError('compiler metadata is missing or incomplete')
    for section in ['registered_types', 'registered_properties']:
        table = metadata[section]
        if (not isinstance(table, dict) or not table
                or any(not isinstance(key, str) or not re.fullmatch(r'0|[1-9][0-9]{0,4}', key)
                       or int(key) > 65535 or not isinstance(name, str)
                       or not re.fullmatch(r'[A-Za-z][A-Za-z0-9_]*', name)
                       for key, name in table.items())):
            raise SchemaError(f'compiler metadata has an invalid {section} inventory')
    canonical = metadata['canonical']
    if not isinstance(canonical, dict) or set(canonical) != {'objects', 'definitions', 'root', 'object_spec'}:
        raise SchemaError('compiler metadata lacks a complete canonical schema')
    if (not isinstance(canonical['root'], dict) or not canonical['root']
            or not isinstance(canonical['object_spec'], dict)
            or '$defs' in canonical['root'] or 'oneOf' in canonical['object_spec']):
        raise SchemaError('compiler metadata has invalid canonical schema envelopes')
    for section in ['objects', 'definitions']:
        table = canonical[section]
        if (not isinstance(table, dict) or not table
                or any(not isinstance(name, str) or not name or not isinstance(row, dict)
                       for name, row in table.items())):
            raise SchemaError(f'compiler metadata has invalid canonical {section}')
    for name, row in canonical['objects'].items():
        properties = row.get('properties')
        if (row.get('type') != 'object' or not isinstance(properties, dict)
                or not isinstance(properties.get('type'), dict)
                or properties['type'].get('const') != name):
            raise SchemaError(f'compiler metadata has an invalid canonical object: {name}')
    provenance = metadata['provenance']
    if (not isinstance(provenance, dict) or len(provenance) != 2
            or 'docs/scene.schema.v1.json' not in provenance
            or sum(isinstance(name, str) and name.endswith('/generated_registry.rs')
                   for name in provenance) != 1
            or any(not isinstance(name, str) or Path(name).is_absolute() or '..' in Path(name).parts
                   or not isinstance(digest, str) or not re.fullmatch(r'[0-9a-f]{64}', digest)
                   for name, digest in provenance.items())):
        raise SchemaError('compiler metadata has invalid source provenance')
