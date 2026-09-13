#!/usr/bin/env python3
"""Source/native projection of the complete AW field; not a runtime registry.

The existing matrices own capability IDs. This projection expands every member,
retaining its source fields and an explicit composition gap. Code/source pins
are evidence of inspection, never evidence that the full composition performed.
"""
from __future__ import annotations
import argparse
import csv
import gzip
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BASE = 'docs/integrations/epi-logos/'
LOCK = BASE + 'TA-ONTA-FULL-FIELD-LOCK.md'
LANGUAGE = 'docs/kernel-rebuild/VAK-OIKONOMIA-KNOWLEDGE-RETURN.md'
SCHEMA = 'epi.aw-field-projection/v1'
ORGANS = ('Khora', 'Hen', 'Pleroma', 'Chronos', 'Anima', 'Aletheia')


def load(path):
    file = ROOT / path
    if file.suffix == '.gz':
        data = gzip.decompress(file.read_bytes())
    else:
        data = file.read_bytes()
    return json.loads(data)


def git_blob(path):
    data = path.read_bytes()
    return hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()


def local_source(path, selector):
    if not (ROOT / path).is_file():
        raise ValueError('missing source carrier: ' + path)
    return {'repository': 'EpiLogos/QL-MEF', 'path': path,
            'git_blob': git_blob(ROOT / path), 'selector': selector}


def tables(path):
    # These two authored locks use pipe-free cells. Fail rather than silently
    # splitting future embedded pipes and changing the meaning of a source row.
    for number, line in enumerate((ROOT / path).read_text().splitlines(), 1):
        if line.startswith('|'):
            yield number, [part.strip() for part in line.strip('|').split('|')]


def source_basis():
    source = load(BASE + 'aw0-source-basis.json')
    source['files'] = []
    for member in source['members']:
        if '/' in member or not member.startswith('aw0-sources-'):
            raise ValueError('invalid source member path')
        shard = load(BASE + member)
        if shard['schema_version'] != 'epi.aw-source-pins/v1':
            raise ValueError('unsupported source shard')
        for owner, values in shard['owners'].items():
            source['files'].extend([owner, path, blob] for path, blob in values)
    return source


def project():
    blobs = {}

    def local_source(path, selector):
        if path not in blobs:
            blobs[path] = git_blob(ROOT / path)
        return {"repository": "EpiLogos/QL-MEF", "path": path,
                "git_blob": blobs[path], "selector": selector}

    rule = load(BASE + 'aw0-native-bindings.json')
    source = source_basis()
    if rule['schema_version'] != 'epi.aw-native-binding/v1' or source['schema_version'] != 'epi.aw-source-basis/v1':
        raise ValueError('unsupported AW source/binding version')
    pins = {}
    for row in source['files']:
        owner, path, blob = row
        key = owner + ":" + path
        if key in pins or len(blob) != 40 or not re.fullmatch('[0-9a-f]{40}', blob):
            raise ValueError('duplicate/invalid source pin: ' + key)
        pins[key] = {**source['repositories'][owner], 'path': path, 'git_blob': blob}
    for refs in rule['groups'].values():
        if not refs or any(ref not in pins for ref in refs):
            raise ValueError('native group has an unpinned source')
    if len(rule['sp_groups']) != 36 or len(rule['sp_gap']) != 36:
        raise ValueError('historical S-prime binding vectors must cover every record')
    records = []

    def add(kind, identity, meaning, sources, groups, gap, coordinates=(), detail=None):
        if not identity or not meaning or not sources or gap not in rule['gaps']:
            raise ValueError('incomplete source/native disposition: ' + str(identity))
        refs = sorted({ref for group in groups for ref in rule['groups'][group]})
        records.append({
            'id': f'{kind}:{identity}', 'inventory': kind, 'source_identity': identity,
            'meaning': meaning, 'sources': sources, 'coordinates': list(coordinates),
            'native_bindings': [pins[ref] for ref in refs],
            'native_binding_groups': list(groups),
            'inputs_and_results': detail or {'contract': meaning},
            'authority': rule['authority'], 'lifecycle_return': rule['lifecycle'],
            'agent_skill_method_participation': rule['participation'],
            'tests_required': rule['tests_required'],
            'disposition': 'EPI-GAP', 'gap': {'id': gap, **rule['gaps'][gap]},
            'mechanic_standing': 'INSPECTED-NATIVE-SOURCE-NOT-WHOLE-COMPOSITION-PROOF',
        })

    # The historical source shorthand is not a new coordinate in the M registry.
    historical = [(line, row) for line, row in tables(LOCK) if re.fullmatch(r'SP[0-5][0-5]', row[0])]
    if [row[0] for _, row in historical] != [f'SP{i}{j}' for i in range(6) for j in range(6)]:
        raise ValueError('full SP00-SP55 authored inventory is missing, repeated or reordered')
    for n, (line, row) in enumerate(historical):
        if len(row) != 4:
            raise ValueError('unexpected SP table structure')
        identity, meaning, owner, result = row
        i, j = identity[2:]
        key = f"Original:Idea/Bimba/Seeds/S/S{i}/S{i}'/S{i}-{j}'-SPEC.md"
        add('historical-sp', identity, meaning,
            [pins[key], local_source(LOCK, f'line:{line}')], rule['sp_groups'][n], rule['sp_gap'][n],
            detail={'current_organ_and_native_relation': owner, 'required_result_and_conformance': result,
                    'source_schema': key, 'no_historical_coordinate_rewrite': True})

    organ_rows = [(line, row) for line, row in tables(LOCK) if re.fullmatch(r'S[0-5]′ \w+', row[0])]
    if len(organ_rows) != 6:
        raise ValueError('all six current organs are required independently of SP records')
    for i, (line, row) in enumerate(organ_rows):
        add('organ', f'S{i}′', row[1], [local_source(LOCK, f'line:{line}')],
            rule['sp_groups'][6*i], rule['sp_gap'][6*i], detail={'native_participation': row[2]})

    # Root contributions remain separate obligations. Paragraph-level references
    # preserve every word of the authored whole instead of selecting pilot nouns.
    text = (ROOT / LOCK).read_text()
    section = text.split('## 3. Root-level contributions retained beyond the table\n', 1)[1].split('\n## 4.', 1)[0]
    heading = 'root'
    for block in section.split('\n\n'):
        block = block.strip()
        if block.startswith('### '):
            heading = block[4:].strip()
        elif block.strip() and not block.startswith('|'):
            add('root-contribution', str(len([r for r in records if r['inventory'] == 'root-contribution'])),
                block.strip(), [local_source(LOCK, 'section:3/' + heading)],
                ['context', 'wiki', 'resolve', 'now', 'agents', 'thought'], 'AW2-SCOPE',
                detail={'heading': heading, 'source_paragraph': block.strip()})

    # Preserve the existing authoritative 72 capability/inhabitation identities.
    matrix = BASE + 'epi-ta-onta-m-relational-field.csv'
    with (ROOT / matrix).open(newline='') as stream:
        rows = list(csv.DictReader(stream))
    if len(rows) != 72 or len({r['id'] for r in rows}) != 72:
        raise ValueError('M capabilities and inhabitation must retain all 72 original records')
    for row in rows:
        kind = 'm-capability' if row['record_type'] == 'capability' else 'm-inhabitation'
        groups = ['registry', 'vak'] if row['row_id'] in ('M0', 'M1', 'M2', 'M3') else ['context', 'agents', 'thought']
        add(kind, row['id'], row['need'] or row['operation'], [local_source(matrix, 'id:' + row['id'])],
            groups, 'NARA' if row['row_id'] == 'M4' else 'AW2-SCOPE',
            [row['row_id']], detail=row)

    # Every registered deep source matrix is expanded, including its exact
    # relations, source readiness, unresolved and drift fields. Never upgrade a
    # September matrix just because a later finite-engine test passed.
    product = load('.oi/product.json')
    entry = next(e for e in product['capability_matrices']['matrices'] if e['id'] == 'epi-deep-coordinate-matrices')
    for member in entry['members']:
        data = load(member['data'])
        for item in data['capabilities']:
            identity = member['id'] + ':' + item['id']
            add('deep-capability', identity, item['name'], [local_source(member['data'], 'id:' + item['id'])],
                ['registry', 'clock'] if 'M4' not in member['data'] else ['context', 'agents'],
                'NARA' if 'M4' in member['data'] else 'AW2-SCOPE',
                [item['coordinate']] if 'coordinate' in item else [], detail=item)
        for key in ('unresolved', 'open', 'drift'):
            for n, item in enumerate(data.get(key, [])):
                add('deep-source-condition', f'{member["id"]}:{key}:{n}', str(item),
                    [local_source(member['data'], f'{key}:{n}')], ['registry'],
                    'NARA' if 'M4' in member['data'] else 'AW2-SCOPE', detail={'source_condition': item})

    language_source = 'data/epi-bimba-map/anuttara-language-map.md'
    entries = []
    for line, raw in enumerate((ROOT / language_source).read_text().splitlines(), 1):
        match = re.match(r'^\| `(M0[^`]*)` \|', raw)
        if match:
            entries.append(match.group(1))
            add('vak-entry', match.group(1), raw, [local_source(language_source, f'line:{line}')],
                ['registry', 'vak', 'provider'], 'AW2-SCOPE', [match.group(1)],
                {'retained_source_row': raw, 'operation': 'source-backed VakRegistry lookup and interpretation; native effect requires the bound cell/method'})
    if len(entries) != 109 or len(set(entries)) != 109:
        raise ValueError('the complete 109-entry language must remain available')

    # Finite operation/horizon profiles are read from the approved table, not
    # guessed from equal-size capability matrices or emitted as 36 services.
    op_rows = [(line, row) for line, row in tables(LANGUAGE)
               if row[0] in ('@# Potential', '- Distinguish', '+ Affirm', 'x Relate', '/ Contextualise', '= Express')]
    if len(op_rows) != 6:
        raise ValueError('all six operative profile rows are required')
    for operation, (line, row) in enumerate(op_rows):
        if len(row) != 7:
            raise ValueError('each operation must retain all six horizons')
        for horizon, purpose in enumerate(row[1:]):
            add('vak-cell', f'{operation}/{horizon}', purpose,
                [local_source(LANGUAGE, f'line:{line}/column:{horizon+1}')],
                ['vak', 'provider', 'knowledge', 'methods'], 'AW2-SCOPE',
                [f'M0-5-(0/1)-{operation}', f'M0-5-(5/0)-{horizon}'],
                {'operation': row[0], 'horizon': horizon, 'operand': 'explicit native subject/source refs in current qualified context',
                 'method_result': purpose, 'effect': 'native owner admission; express may propose or perform, never auto-promote'})

    for line, row in tables(LANGUAGE):
        label = row[0]
        if label.startswith(('C0′ /', 'C1′ /', 'C2′ /', 'C3′ /', 'C4′ /', 'C5′ /')):
            add('c-prime-office', label.split(' / ')[0], row[2], [local_source(LANGUAGE, f'line:{line}')],
                ['vak', 'provider', 'workflow'], 'AW2-CONDUCT', detail={'name': row[1], 'determination': row[2]})
        elif re.fullmatch(r'CF[1-7]', label):
            add('context-frame', label, row[3], [local_source(LANGUAGE, f'line:{line}')],
                ['vak', 'clock', 'workflow'], 'AW2-CONDUCT', detail={'expression': row[1], 'reference_note_only': row[2], 'role': row[3]})
        elif re.fullmatch(r'CFP[0-5]|Z', label):
            add('thread-form', label, row[1], [local_source(LANGUAGE, f'line:{line}')],
                ['vak', 'workflow'], 'AW2-CONDUCT', detail={'form': row[1], 'musical_relation': row[2]})
        elif re.fullmatch(r'CS[0-5]', label):
            add('context-sequence', label, row[1], [local_source(LANGUAGE, f'line:{line}')],
                ['vak', 'workflow'], 'AW2-CONDUCT', detail={'ordered_cp_pairs': row[2], 'directions': ['forward', 'returning'], 'not_civil_time': True})
        elif re.fullmatch(r'T[0-5]', label):
            for offset in (0, 2):
                add('thought', row[offset], row[offset+1], [local_source(LANGUAGE, f'line:{line}/column:{offset}')],
                    ['now', 'thought', 'knowledge'], 'AW3-CONSUME', detail={'selected_report_meaning': row[offset+1], 'no_compulsory_file': True})
        elif re.fullmatch(r'#[0-5]', label):
            add('property-office', label, row[2], [local_source(LANGUAGE, f'line:{line}')],
                ['registry', 'wiki', 'source'], 'AW1-PROPERTY', detail={'office': row[1], 'not_epistemic_rank': True})

    propfile = BASE + 'aw-property-source.json.gz'
    props = load(propfile)
    for values in props['properties']:
        item = dict(zip(props['columns'], values, strict=True))
        add('property-definition', item['owner'] + ':' + item['key'], item['key'],
            [props['source'], local_source(propfile, item['owner'] + ':' + item['key'])],
            ['registry', 'wiki'], 'AW1-PROPERTY', detail={**item, 'coordinate_home_is_not_universal_applicability': True,
                'q_prefix_is_not_a_synthesis_producer': True})
    if len(props['properties']) != props['count'] or props['count'] != 194:
        raise ValueError('all original executable property definitions are required')

    for key, pin in pins.items():
        if key.startswith('Original:') and key.endswith('/SKILL.md'):
            add('source-skill', pin['path'], pin['path'].split('/')[-2], [pin],
                ['methods', 'agents', 'resolve'], 'AW2-SCOPE',
                detail={'source_class': 'staged-copy' if '/staged/' in key else 'source-skill',
                        'native_registration': 'not implied by this source file; requires existing native Skill source/admission'})
    epii_file = BASE + 'epi-epii-operational-capacities.json'
    epii = load(epii_file)
    for item in epii['capacities']:
        add('epii-on-x', item['target'], item['developmental_law'],
            [local_source(epii_file, 'target:' + item['target']),
             {'repository': epii['source_repository'], 'revision': source['repositories']['Original']['revision'],
              'path': epii['source_directory'] + item['file'], 'git_blob': item['blob']}],
            ['agents', 'development', 'source'], 'NARA' if item['target'] == 'M4' else 'AW3-CONSUME',
            [item['target']], detail=item)

    # Retain a source claim as a source claim. The original files give the 20/40
    # formula without an enumerated 20-member current basis. No Cartesian-product
    # arithmetic invents that missing authorial enumeration.
    add('source-discrepancy', 'legacy-20-40', 'Original 20-frame / 40-directional-sequence claim requires an explicit enumerated source-to-current disposition.',
        [local_source(LANGUAGE, 'section:1.4'), pins["Original:Idea/Bimba/Seeds/M/M4'/Legacy/plans/CLOCK-AND-NARA-SPECS/07-c-prime-vak-grammar-layer.md"]],
        ['vak'], 'AW2-CONDUCT', detail={'standing': 'SOURCE-CLAIM-NOT-CARDINALITY-PROOF', 'current_frame_identities': 7,
                                     'paired_passages': 6, 'direction_is_independent': True})
    # These finite C-prime subforms are expressly defined by the current lock.
    for identity, meaning in [('dialogical', 'CPF: human-engaged dialogue; goal discovery before an undertaking'),
                              ('authorised-undertaking', 'CPF: a specific admitted undertaking; uncertainty can reopen dialogue')]:
        add('participation-regime', identity, meaning, [local_source(LANGUAGE, 'section:1')],
            ['agents', 'execution', 'vak'], 'AW2-CONDUCT')
    for n, meaning in enumerate(('relational/association/dependency/assumption', 'definition/source/vocabulary',
                                 'operation/executable/test', 'pattern/image/diagram', 'context/situation', 'integration/learning/Return')):
        add('content-type', f'CT{n}', meaning, [local_source(LANGUAGE, 'section:1.1')], ['vak', 'wiki', 'methods'], 'AW2-CONDUCT')
        add('context-position', f'4.{n}', ('ground', 'definition', 'operation', 'pattern', 'context', 'integration')[n],
            [local_source(LANGUAGE, 'section:1')], ['vak', 'workflow'], 'AW2-CONDUCT')
    add('content-type', "CT4b′", 'Contextual-artifact specialization through native source/Day/NOW, not a new time owner',
        [local_source(LANGUAGE, 'section:1.1')], ['vak', 'now', 'wiki'], 'AW2-CONDUCT')
    for name in ('Rupa', 'Ontology', 'Frame Contract', 'Temporal', 'Capability', 'Sattva'):
        add('agent-definition-office', name, name, [local_source(LANGUAGE, 'section:2.1')],
            ['agents', 'context', 'methods'], 'AW2-SCOPE')
    for identity, meaning in [('Anansi', 'Investigate gaps and the authored/implemented relation'),
                              ('Janus', 'Relate temporal occasions'), ('Moirai:Klotho', 'Evidence mode of Moirai'),
                              ('Moirai:Lachesis', 'Query mode of Moirai'), ('Moirai:Atropos', 'Reflection mode of Moirai'),
                              ('Mercurius', 'Cross-domain translation'), ('Agora', 'Plural evaluation'),
                              ('Zeithoven', 'Develop the next score')]:
        add('specialist-mode', identity, meaning, [local_source(LOCK, 'section:3/Aletheia')],
            ['agents', 'methods', 'knowledge'], 'AW2-SCOPE', detail={'not_an_extra_m_agent': True,
            'Moirai_is_one_actor_with_three_modes': True})
    for key, pin in pins.items():
        if key.startswith('Original:') and '/agents/' in key and key.endswith('.md'):
            add('agent-source', pin['path'], pin['path'].rsplit('/', 1)[-1], [pin],
                ['agents', 'methods'], 'AW2-SCOPE')
    participation = load(BASE + 'aw-original-participation.json.gz')
    for field, value in participation['source_fields'].items():
        members = list(enumerate(value)) if isinstance(value, list) else list(value.items()) if isinstance(value, dict) else [('value', value)]
        for key, item in members:
            add('original-participation', f'{field}:{key}', str(item), [participation['source']],
                ['agents', 'methods', 'workflow'], 'AW2-SCOPE', detail={'source_payload': item,
                'source_field': field, 'corrections': participation['corrections'], 'permission_grant': False})
    for identity, meaning, path, symbol in [
        ('overlay-read', 'Read canonical QV separately from live cache; preserve essence/pithy alias and all five fields.',
         'Body/S/S0/epi-cli/src/core/overlay.rs', 'overlay_entry'),
        ('overlay-write', 'Source write of the five-field QV overlay through its original owner, not an inferred q_* computation.',
         'Body/S/S0/epi-cli/src/core/overlay.rs', 'save_overlay'),
        ('knowing-read', 'Read a dossier using exact source and current-versus-cached standing.',
         'Body/S/S0/epi-cli/src/core/mod.rs', 'knowing'),
        ('bake', 'Write-gated overlay-to-C essence bake; not a universal synthesis algorithm.',
         'Body/S/S0/epi-cli/src/core/mod.rs', 'knowing_bake'),
        ('relationship-write', 'Historical endpoint/type MERGE can overwrite properties; new independent assertion identities must not.',
         'Body/S/S2/graph-services/src/relationship_manager.rs', 'RelationshipWritePlan'),
        ('graph-derived', 'Derived graph algorithms require exact projection/algorithm/version/parameters and protected-scope exclusions.',
         'Body/S/S2/graph-services/src/gds.rs', 'algorithm_descriptors')]:
        add('property-producer', identity, meaning, [pins['Original:' + path]], ['registry', 'source'],
            'AW1-PROPERTY', detail={'historical_symbol': symbol, 'no_implicit_canonical_promotion': True})

    # The historical executable registry is not the whole deep property body.
    # Resolve every original property-key occurrence through the already-owned
    # registry record/file/payload identities; do not copy a second M registry.
    manifest_path = 'fixtures/kernel/m-tree-v1.json'
    manifest = load(manifest_path)
    uses = {}
    for index, record in enumerate(manifest['records']):
        for key in record['property_keys']:
            uses.setdefault(key, []).append(index)
    for key, indices in sorted(uses.items()):
        add('property-source-key', key, key,
            [local_source(manifest_path, 'records[].property_keys:' + key)],
            ['registry', 'wiki', 'knowledge'], 'AW1-PROPERTY', detail={
                'source_record_indices': indices,
                'record_identity': 'file/record_index/payload_sha256 in the retained M manifest',
                'typing': 'SOURCE-PAYLOAD-KEY-NOT-IMPLICIT-CANONICAL-DEFINITION',
                'q_producer_not_inferred': True,
            })
    job_source = pins["Original:Idea/Bimba/Seeds/S/S4/S4'/Legacy/superpowers/plans/2026-05-22-vak-as-operational-substrate.md"]
    jobs = [
        ('address', 'S0-validated Vāk address qualifies actual dispatch and every operation', ['vak', 'provider']),
        ('persistence', 'Native session persistence retains original complete address and lineage', ['session', 'provider']),
        ('artifact', 'Produced artifacts retain the actual address, source and attribution', ['wiki', 'source']),
        ('cycle', 'Compose and perform through native lifecycle; completion opens governed review', ['workflow', 'agents']),
        ('return', 'Sophia/Moirai/Aletheia/Epii Return enters actual source-linked re-composition', ['thought', 'development']),
        ('gate', 'Native admitted transitions govern work rather than cosmetic phase labels', ['workflow', 'execution']),
        ('skill', 'Every participating Skill carries source-qualified Vāk affinities without extra authority', ['methods', 'resolve']),
        ('retrieval', 'Selected Vāk scope changes actual bounded retrieval, not only its label', ['knowledge', 'provider']),
        ('music', 'Actual operation emits source-qualified musical/structural performance', ['vak', 'clock']),
        ('anima', 'Anima instantiation composes native agent identity, definition and actual capabilities', ['agents', 'context']),
    ]
    for number, (name, purpose, groups) in enumerate(jobs, 1):
        add('original-operational-job', str(number), purpose, [job_source], groups,
            'AW2-CONDUCT', detail={'source_job': number, 'name': name,
                'historical_paths_are_not_current_owner_paths': True,
                'wrong_frame_and_time_labels_are_corrected_by': LANGUAGE})

    ids = [r['id'] for r in records]
    if len(ids) != len(set(ids)):
        raise ValueError('duplicate projected identity')
    counts = {}
    for record in records:
        counts[record['inventory']] = counts.get(record['inventory'], 0) + 1
    expected = {'historical-sp': 36, 'organ': 6, 'm-capability': 36, 'm-inhabitation': 36,
                'vak-entry': 109, 'vak-cell': 36, 'c-prime-office': 6, 'context-frame': 7,
                'thread-form': 7, 'context-sequence': 6, 'thought': 12, 'property-office': 6,
                'property-definition': 194, 'source-skill': 65, 'epii-on-x': 6, 'source-discrepancy': 1, 'property-source-key': 5686, 'original-operational-job': 10}
    if any(counts.get(k) != v for k, v in expected.items()):
        raise ValueError('required field enumeration differs: ' + str(counts))
    return {'schema_version': SCHEMA, 'scope': 'enumerated-source-field-with-explicit-implementation-and-source-gaps',
            'native_revisions': source['repositories'], 'counts': counts,
            'readiness_reconciliation': {'historical': load(BASE + 'epi-capability-readiness.json'),
                'current': 'Retain historical 21 READY-TO-COMPOSE / 15 EPI-GAP as that source snapshot. Landed M1-M3 finite/native mechanics and AIKit#302 / Factory#230 do not certify full Epi composition.',
                'preserved_engine_receipts': ['docs/KERNEL-M1-ENGINE-CONTRACT.md', 'docs/kernel-rebuild/m2-engine-v1.md', 'docs/KERNEL-M3-ENGINE-CONTRACT.md']},
            'records': records}


def verify_sources(directory):
    source = source_basis()
    for owner, path, expected in source['files']:
        actual = directory / owner / path
        if not actual.is_file() or git_blob(actual) != expected:
            raise ValueError('source pin mismatch: ' + str(actual))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--json', action='store_true')
    parser.add_argument('--id', help='exact projected record identity; no fuzzy fallback')
    parser.add_argument('--verify-sources', type=Path, help='read-only parent of exact pinned owner checkouts')
    args = parser.parse_args()
    result = project()
    if args.verify_sources:
        verify_sources(args.verify_sources)
    if args.id:
        matches = [r for r in result['records'] if r['id'] == args.id]
        if len(matches) != 1:
            raise SystemExit('unknown AW record: ' + args.id)
        result = matches[0]
    print(json.dumps(result if args.json or args.id else {'schema_version': SCHEMA, 'counts': result['counts'],
          'runtime_acceptance': False}, ensure_ascii=False, sort_keys=True, indent=2))


if __name__ == '__main__':
    main()
