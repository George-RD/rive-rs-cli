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
    variants = definitions.pop('ObjectSpec')['oneOf']
    objects = {}
    for row in variants:
        name = row['properties']['type']['const']
        if not isinstance(name, str) or name in objects:
            raise SchemaError('invalid or duplicate canonical object declaration')
        objects[name] = row
    text = registry.read_text()
    return {'registered_types': registry_table(text, 'type_name'),
            'registered_properties': registry_table(text, 'property_name'),
            'canonical': {'objects': objects, 'definitions': definitions},
            'provenance': {str(path.relative_to(root)): reference.digest(path)
                           for path in [registry, schema_path]}}


def metadata_facts(metadata: dict) -> dict:
    return {key: value for key, value in metadata.items() if key != 'provenance'}
