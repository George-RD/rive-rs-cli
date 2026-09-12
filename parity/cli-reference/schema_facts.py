import re


class SchemaError(ValueError):
    pass


def field_identity(field: dict) -> str:
    return f"{field['owner']}.{field['name']}"


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
        elif line == 'no matching properties':
            count = 0
        elif match := re.match(r'(\d+) propert(?:y|ies)\.', line):
            count = int(match[1])
        elif line and not line.startswith(('      ', 'Editor-only properties are hidden;')):
            raise SchemaError('unrecognized schema layout')
    fields = result['properties']
    if count != len(fields) or len({field_identity(field) for field in fields}) != len(fields):
        raise SchemaError(f"incomplete or duplicate property inventory: {result['name']}")
    result['properties'] = sorted(fields, key=field_identity)
    return result


def normalize_type(runtime_text: str, all_text: str) -> dict:
    runtime, complete = parse_type(runtime_text), parse_type(all_text)
    if any(runtime[field] != complete[field] for field in ['name', 'type_key', 'inherits']):
        raise SchemaError('runtime/all schema identity mismatch')
    visible = {field_identity(field): field for field in runtime['properties']}
    all_fields = {field_identity(field): field for field in complete['properties']}
    if any(field != all_fields.get(name) for name, field in visible.items()):
        raise SchemaError('runtime/all property facts disagree')
    for field in complete['properties']:
        field['hidden_without_all'] = field_identity(field) not in visible
    return complete


def value_changes(before, after, prefix: str = '') -> list[dict]:
    def compare(left, right, path):
        if isinstance(left, dict) and isinstance(right, dict):
            for name in sorted(left.keys() | right.keys()):
                child = f'{path}.{name}' if path else name
                if name not in left:
                    changes.append({'path': child, 'change': 'added', 'after': right[name]})
                elif name not in right:
                    changes.append({'path': child, 'change': 'removed', 'before': left[name]})
                else:
                    compare(left[name], right[name], child)
        elif type(left) is not type(right) or left != right:
            changes.append({'path': path, 'change': 'changed', 'before': left, 'after': right})

    changes = []
    compare(before, after, prefix)
    return changes


def schema_changes(before: dict, after: dict) -> list[dict]:
    def indexed(types):
        return {name: dict(row, properties={field_identity(field): field for field in row['properties']})
                for name, row in types.items()}

    return value_changes(indexed(before), indexed(after))
