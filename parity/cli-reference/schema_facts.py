import re


class SchemaError(ValueError):
    pass


def type_names(text: str) -> list[str]:
    lines = text.strip().splitlines()
    summary = re.fullmatch(r'(\d+) types\.', lines[-1]) if lines else None
    names = [line for line in lines[:-1] if line]
    if (summary is None or int(summary[1]) != len(names) or len(names) != len(set(names))
            or not all(re.fullmatch(r'[A-Za-z][A-Za-z0-9]*', name) for name in names)):
        raise SchemaError('unrecognized or incomplete type inventory')
    return sorted(names)


def parse_type(text: str) -> dict:
    lines = text.strip().splitlines()
    header = re.fullmatch(r'(\w+)\s+\(typeKey (\d+)\)', lines[0]) if lines else None
    if header is None:
        raise SchemaError('unrecognized schema header')
    result = {'name': header[1], 'type_key': int(header[2]), 'inherits': [], 'properties': []}
    owner = result['name']
    count = None
    for line in lines[1:]:
        if line.startswith('extends '):
            result['inherits'] = line.removeprefix('extends ').split(' > ')
        elif line == 'own properties:':
            owner = result['name']
        elif line.startswith('inherited from ') and line.endswith(':'):
            owner = line[len('inherited from '):-1]
        elif match := re.fullmatch(r'  (\w+)\s+([ABD]*)\s+(.+?)\s+key (\d+)(?:\s+= (.*))?', line):
            name, flags, value_type, key, default = match.groups()
            result['properties'].append({'name': name, 'key': int(key), 'owner': owner,
                                         'value_type': value_type, 'default_literal': default,
                                         'enum_values': None, 'bits': None,
                                         'animatable': 'A' in flags, 'bindable': 'B' in flags,
                                         'derived': 'D' in flags})
        elif line.startswith('      accepts: ') and result['properties']:
            result['properties'][-1]['enum_values'] = line[len('      accepts: '):].split(', ')
        elif line.startswith('      bits ') and result['properties']:
            result['properties'][-1]['bits'] = line.split('): ', 1)[-1].split()
        elif match := re.match(r'(\d+) properties\.', line):
            count = int(match[1])
    fields = result['properties']
    if count != len(fields) or len({field['name'] for field in fields}) != len(fields):
        raise SchemaError(f"incomplete or duplicate property inventory: {result['name']}")
    result['properties'] = sorted(fields, key=lambda field: field['name'])
    return result


def normalize_type(runtime_text: str, all_text: str) -> dict:
    runtime, complete = parse_type(runtime_text), parse_type(all_text)
    if any(runtime[field] != complete[field] for field in ['name', 'type_key', 'inherits']):
        raise SchemaError('runtime/all schema identity mismatch')
    visible = {field['name']: field for field in runtime['properties']}
    all_fields = {field['name']: field for field in complete['properties']}
    if any(field != all_fields.get(name) for name, field in visible.items()):
        raise SchemaError('runtime/all property facts disagree')
    for field in complete['properties']:
        field['hidden_without_all'] = field['name'] not in visible
    return complete
