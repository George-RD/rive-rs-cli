import argparse
import json
import re
import tarfile
import tempfile
from pathlib import Path

import reference
from compiler_metadata import metadata_facts, read_metadata
from schema_facts import SchemaError, schema_changes, value_changes
from schema_snapshot import from_capture, read_snapshot, write_snapshot


STAGES = ('declared', 'parsed', 'lowered', 'encoded', 'runtime_tested', 'semantic_tested')
FAMILIES = ('geometry-paint', 'paths', 'text', 'layout', 'assets', 'animation',
            'rigging-constraints', 'state-machines-blends', 'view-models-converters',
            'nested-composition', 'scripting', 'shaders', '3d', 'other')
KNOWN_WORK = [
    {'issue': 123, 'gap': 'builder-encoder', 'scope': 'Property keys must be audited with type/owner identity, not bare names.'},
    {'issue': 124, 'gap': 'builder-encoder', 'scope': 'sourcePathIds byte backing and encoding require their own proof.'},
    {'issue': 125, 'gap': 'builder-encoder', 'scope': 'Decompile round-trip coverage is not established by schema declarations.'},
    {'issue': 126, 'gap': 'builder-encoder', 'scope': 'Runtime defaults and suppression need object-specific tests.'},
    {'issue': 127, 'gap': 'canonical-representation', 'scope': 'Human-readable enums need parser and rejection evidence.'},
    {'issue': 128, 'gap': 'canonical-representation', 'scope': 'Schema inheritance does not establish valid scene parent/child rules.'},
    {'issue': 175, 'gap': 'canonical-representation', 'scope': 'The parent Authoring behavior programme remains incomplete; this report adds no behavior vocabulary.'},
    {'issue': 252, 'gap': 'builder-encoder', 'scope': 'Remaining view-model property name/parent emission defects are not fixed by registry presence.'},
    {'issue': 254, 'gap': 'builder-encoder', 'scope': 'Malformed bound-boolean conditions need the dedicated canonical guard.'},
    {'issue': 255, 'gap': 'builder-encoder', 'scope': 'Direct blend source validation and enum ownership remain separate work.'},
    {'issue': 267, 'gap': 'rml-parser', 'scope': 'The first independent static RML compile/evidence slice is separate work.'},
]
MAX_REFERENCE_BYTES = 64 * 1024 * 1024


def family_for(name: str) -> str:
    rules = [
        ('3d', r'3[Dd]|^Perspective|^Camera'), ('shaders', r'Shader'), ('scripting', r'Script|^Code'),
        ('nested-composition', r'^Nested|^Library'),
        ('view-models-converters', r'ViewModel|^Data|^Bindable|^Formula|^CustomProperty'),
        ('assets', r'Asset|^PaintImage'), ('text', r'^Text'),
        ('layout', r'Layout|^Grid|^Axis|^NSlic|Scroll'),
        ('rigging-constraints', r'Constraint|Bone|^Skin$|^Tendon$|Weight'),
        ('state-machines-blends', r'State|^Blend|Transition|Listener|^Focus|Input|^Event$|Event$|^Semantic'),
        ('animation', r'Animation|^KeyFrame|^Keyed|Interpolator'),
        ('paths', r'Path|Vertex|^Contour'),
        ('geometry-paint', r'^(Artboard|Node|Shape|Ellipse|Rectangle|Triangle|Polygon|Star|SolidColor|Fill|Stroke|GradientStop|LinearGradient|RadialGradient|Solo|Image|Mesh|Feather|Dash|DrawRules|DrawTarget)$'),
    ]
    return next((family for family, pattern in rules if re.search(pattern, name)), 'other')


def verified_seed() -> dict:
    baseline = reference.HERE / 'baseline'
    pin = reference.read_json(baseline / 'capture.json')
    archive = reference.checked_relative(baseline, pin['archive'])
    if reference.digest(archive) != pin['archive_sha256']:
        raise SchemaError('reference fixture archive changed; evidence is stale')
    with tempfile.TemporaryDirectory(prefix='rive-capability-evidence-') as temporary:
        root = Path(temporary)
        with tarfile.open(archive, 'r:xz') as package:
            members = package.getmembers()
            if sum(member.size for member in members) > MAX_REFERENCE_BYTES:
                raise SchemaError('reference fixture archive exceeds the size budget')
            seen = set()
            for member in members:
                path = Path(member.name)
                if (not member.isfile() or path.is_absolute() or '..' in path.parts
                        or member.name in seen):
                    raise SchemaError('unsafe or duplicate reference fixture member')
                seen.add(member.name)
                target = root / path
                target.parent.mkdir(parents=True, exist_ok=True)
                with package.extractfile(member) as source:
                    target.write_bytes(source.read())
        recording = reference.verify_recording(root, pin['source_head'])
    return {'pipeline': 'official-cli-reference', 'source_head': pin['source_head'],
            'archive_sha256': pin['archive_sha256'], 'runtime': recording['runtime'],
            'claims_our_rml_support': False,
            'cases': {name: {'stages': {'declared': True, 'parsed': True, 'lowered': None,
                                      'encoded': True, 'runtime_tested': True, 'semantic_tested': None},
                             'scope': 'Original fixture only; repeat bytes, three captured frames and paint sensitivity. No full intent evaluation.'}
                      for name in ['static', 'animated']}}


def load_baseline() -> dict:
    directory = reference.HERE / 'schema-baseline'
    pin = reference.read_json(directory / 'snapshot.json')
    path = reference.checked_relative(directory, pin['archive'])
    snapshot = read_snapshot(path, pin['archive_sha256'])
    lock = reference.read_json(reference.HERE / 'lock.json')
    if (snapshot['provenance'].get('lock_sha256') != reference.digest(reference.HERE / 'lock.json')
            or snapshot['provenance'].get('official', {}).get('archive_sha256') != lock['official']['archive_sha256']):
        raise SchemaError('schema baseline belongs to a different reference pin')
    return snapshot


def report(snapshot: dict, metadata: dict, evidence: dict) -> dict:
    registered = {name: int(key) for key, name in metadata['registered_types'].items()}
    properties = metadata['registered_properties']
    objects = metadata['canonical']['objects']
    candidates = {}
    for name in objects:
        candidates.setdefault(name.replace('_', '').lower(), []).append(name)
    families = {family: {'types': [], 'whole_family_runtime_tested': False} for family in FAMILIES}
    families['3d']['observation'] = 'No separately named 3D family identified in this captured type list; capability remains unassessed.'
    rows = {}
    for name, official in snapshot['types'].items():
        family = family_for(name)
        families[family]['types'].append(name)
        matches = candidates.get(name.lower(), [])
        issues = []
        for field in official['properties']:
            ours = properties.get(str(field['key']))
            if ours != field['name'] and not field['hidden_without_all']:
                issues.append({'property': f"{field['owner']}.{field['name']}", 'official_key': field['key'],
                               'registered_name': ours, 'classification': 'builder-encoder',
                               'interpretation': 'Declaration mismatch only; the emitter was not exercised.'})
        gaps = ['rml-parser', 'runtime-version']
        if not matches:
            gaps.append('canonical-representation')
        if registered.get(name) != official['type_key'] or issues:
            gaps.append('builder-encoder')
        if any(field['hidden_without_all'] for field in official['properties']):
            gaps.append('editor-only')
        if family in ('scripting', 'shaders'):
            gaps.append('external-compiler-signing-tool')
        rows[name] = {'family': family, 'official_type_key': official['type_key'],
                      'registered_type_key': registered.get(name),
                      'canonical_name_candidates': matches,
                      'candidate_warning': 'Name similarity is not a confirmed lowering or field mapping.',
                      'stages': {stage: True if stage == 'declared' else None for stage in STAGES},
                      'gap_classes': gaps, 'property_declaration_differences': issues}
    compiler_changes = value_changes(metadata_facts(snapshot['compiler_metadata']), metadata_facts(metadata), 'compiler')
    return {'schema_version': 1, 'official': snapshot['provenance']['official'],
            'summary': {'official_types': len(rows), 'registered_types': len(registered),
                        'canonical_object_variants': len(objects),
                        'fixture_cases': len(evidence['cases'])},
            'stage_scope': 'Per-type stages concern independent RML support; declared means official schema only. Null means unassessed, not unsupported. Fixture evidence has its own pipeline and scope.',
            'gap_scope': 'Classes identify work or unassessed boundaries, not independently proven defects. Schema-filtered fields are grouped as editor-only; emitted flags can still require separate investigation.',
            'compiler_sources': metadata['provenance'], 'compiler_changes_since_snapshot': compiler_changes,
            'families': families, 'types': rows, 'evidence': evidence, 'known_work': KNOWN_WORK,
            'canonical_metadata': metadata['canonical']}


def render_text(result: dict) -> str:
    summary = result['summary']
    lines = [f"Official CLI {result['official']['version']}: {summary['official_types']} declared types",
             f"Current metadata: {summary['registered_types']} registry types; {summary['canonical_object_variants']} canonical object variants",
             'Counts are inventories, not parity percentages.', '', 'Family                       Declared   Family runtime proof']
    for name, family in result['families'].items():
        lines.append(f"{name:28} {len(family['types']):8}   not established")
    lines += ['', 'Evidence: official CLI static and animated seed only; retained capture verified offline.',
              'This is not an independent RML compile, a fresh runtime run or semantic parity.',
              'Stage columns: declared / parsed / lowered / encoded / runtime-tested / semantic-tested.',
              'Known work: ' + ', '.join('#' + str(row['issue']) for row in result['known_work']),
              f"Compiler metadata changes since snapshot: {len(result['compiler_changes_since_snapshot'])}",
              'Use --json for per-type gaps, property differences, canonical fields and scoped evidence.']
    return '\n'.join(lines)


def main() -> None:
    parser = argparse.ArgumentParser(description='Inspect declarations and evidence without claiming registry parity.')
    commands = parser.add_subparsers(dest='command', required=True)
    for name in ['report', 'check']:
        child = commands.add_parser(name)
        child.add_argument('--json', action='store_true')
        if name == 'check':
            child.add_argument('--candidate', type=Path)
    refresh = commands.add_parser('refresh', help='Normalize a capture to a new candidate; never replace the baseline.')
    refresh.add_argument('capture', type=Path)
    refresh.add_argument('--expected-head', required=True)
    refresh.add_argument('--lock', type=Path)
    refresh.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == 'refresh':
            snapshot = from_capture(args.capture.resolve(), args.expected_head, args.lock)
            snapshot['compiler_metadata'] = read_metadata(reference.ROOT)
            write_snapshot(args.output, snapshot)
            print(f'Candidate written to {args.output}; compare with check --candidate and review before retention.')
            return
        baseline = load_baseline()
        metadata = read_metadata(reference.ROOT)
        evidence = verified_seed()
        result = report(baseline, metadata, evidence)
        if args.command == 'check':
            candidate = read_snapshot(args.candidate) if args.candidate else baseline
            changes = schema_changes(baseline['types'], candidate['types'])
            provenance_changes = value_changes(baseline['provenance']['official'], candidate['provenance']['official'], 'official')
            result = {'schema_changes': changes, 'official_changes': provenance_changes,
                      'compiler_changes': result['compiler_changes_since_snapshot'],
                      'stale_fixture_evidence': False}
            changed = bool(changes or provenance_changes or result['compiler_changes'])
            result['ok'] = not changed
            print(json.dumps(result, indent=2, sort_keys=True) if args.json else
                  f"Schema changes: {len(changes)}; official pin changes: {len(provenance_changes)}; compiler changes: {len(result['compiler_changes'])}. Evidence is current for its retained source.")
            raise SystemExit(1 if changed else 0)
        print(json.dumps(result, indent=2, sort_keys=True) if args.json else render_text(result))
    except (SchemaError, reference.ReferenceError, OSError, ValueError, KeyError, TypeError, tarfile.TarError) as error:
        parser.exit(2, f'capabilities: {error}\n')


if __name__ == '__main__':
    main()
