#!/usr/bin/env python3
"""Generate native M3 data from retained C and the one accepted M registry.

Build-time only; stdlib only. Never changes the vendor, registry or source authority.
The emitted include belongs in the build directory, not a second source hierarchy.
"""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = 'fixtures/kernel/m-tree-v1.json'
C_SOURCE = 'vendor/epi-kernel/reference/src/m3.c'
C_HEADER = 'vendor/epi-kernel/reference/include/m3.h'
CLOCK = 'vendor/epi-kernel/reference/src/m3_clock_lut.c'


def code(text: str) -> str:
    return re.sub(r'/\*.*?\*/|//[^\n]*', '', text, flags=re.S)


def declaration(source: str, name: str) -> str:
    match = re.search(r'(?:static\s+)?const\s+\w+\s+' + re.escape(name) + r'\b[^;]*;', source, re.S)
    if match is None:
        raise ValueError('missing retained declaration: ' + name)
    return match.group()


def clock_rows(text: str) -> list[list[int]]:
    body = code(declaration(text, 'CLOCK_DEGREE_LUT'))
    rows = []
    for values in re.findall(r'\{([^{}]+)\}', body):
        row = []
        for token in values.split(','):
            token = token.strip().rstrip('UuFf')
            if not token:
                continue
            value = float(token)
            if not value.is_integer() or not 0 <= value <= 65535:
                raise ValueError('non-integral/out-of-range clock field: ' + token)
            row.append(int(value))
        if len(row) != 26:
            raise ValueError('clock record width changed')
        rows.append(row)
    if len(rows) != 360 or any(row[0] != d for d, row in enumerate(rows)):
        raise ValueError('clock degree domain changed')
    return rows


def source_groups(registry: dict) -> tuple[list[list[str]], list[str]]:
    nodes = {n['source_ref']: n for n in registry['nodes']}
    letters = 'ATCG'
    def require(ref: str, name: str | None = None) -> str:
        node = nodes[ref]
        if name is not None and name not in node['names']:
            raise ValueError(f'source identity mismatch: {ref}: {name}')
        return ref
    nucleotide = [require(f'#3-2-{i+1}', name) for i, name in enumerate(['Adenine','Thymine','Cytosine','Guanine'])]
    pair = [require(f'#3-2-{a+1}-{b+1}', f'{letters[a]}{letters[b]}_Pair') for a in range(4) for b in range(4)]
    codon = [require(f'#3-2-{a+1}-{b+1}-{c+1}', f'Codon_{letters[a]}{letters[b]}{letters[c]}') for a in range(4) for b in range(4) for c in range(4)]
    # The source trigram ids are NOT their three-bit line patterns.
    trigram = [require(f'#3-1-{i}', name) for i, name in enumerate(['Qian','Kun','Zhen','Xun','Kan','Li','Gen','Dui'])]
    binary_to_source = [1,2,4,7,6,5,3,0]
    hexagram = [require(f'#3-1-{binary_to_source[h >> 3]}-{binary_to_source[h & 7]}') for h in range(64)]
    matrix = [require(f'#3-3-2-{i}', f'Matrix {i+1}') for i in range(3)]
    suit_paths = [2,1,4,3]  # names, not a cast between two differently ordered sixfolds
    suits = ['Cups','Wands','Pentacles','Swords']
    minor = []
    for i, path in enumerate(suit_paths):
        require(f'#3-4-{path}', suits[i])
        minor.extend(require(f'#3-4-{path}-{rank}') for rank in range(14))
    # Major coordinates keep their native 0..21 source numbering.
    major = [require(f'#3-4-5/0-{i}') for i in range(22)]
    degrees = [require('#3-5-5/0-0/360' if d == 0 else f'#3-5-5/0-{d}') for d in range(360)]
    backbone = [require(f'#3-5-{q}-{p}') for q in range(1,5) for p in range(6)]
    groups = [[require('#3')], nucleotide, pair, codon, trigram, hexagram, matrix, minor, major, degrees, backbone, [require('#3-4.0')]]
    # Source relations, including both copies of an exported assertion, stay in K2.
    # Deduplicate targets only for this functional accessor, after checking uniqueness.
    by_kind = {}
    for relation in registry['relations']:
        key = (relation['source_kind'], relation['from_ref'])
        by_kind.setdefault(key, set()).add(relation['to_ref'])
    anchors = []
    for d, ref in enumerate(degrees):
        targets = by_kind.get(('ANCHORED_BY', ref), set())
        if len(targets) != 1 or not targets <= set(backbone):
            raise ValueError('missing/ambiguous backbone binding: ' + ref)
        anchor = next(iter(targets))
        if ref not in by_kind.get(('GOVERNS_DEGREE_ARC', anchor), set()):
            raise ValueError('missing reciprocal backbone assertion: ' + ref)
        if by_kind.get(('FLOWS_CLOCKWISE', ref)) != {degrees[(d+1) % 360]}:
            raise ValueError('source/arithmetic clockwise disagreement: ' + ref)
        if by_kind.get(('POLAR_OPPOSITE', ref)) != {degrees[(d+180) % 360]}:
            raise ValueError('source/arithmetic opposition disagreement: ' + ref)
        anchors.append(anchor)
    if any(anchors.count(b) != 15 for b in backbone):
        raise ValueError('backbone must govern 15 distinct source degrees each')
    return groups, anchors


def generate(root: Path = ROOT) -> str:
    raw = {path: (root / path).read_bytes() for path in [REGISTRY,C_SOURCE,C_HEADER,CLOCK]}
    registry = json.loads(raw[REGISTRY])
    source, header = raw[C_SOURCE].decode(), raw[C_HEADER].decode()
    nodes = {n['source_ref']: n for n in registry['nodes']}
    groups, anchors = source_groups(registry)
    out = ['/* Generated by scripts/generate-m3.py. Do not hand-edit. */']
    out.append('static const char native_registry_revision[] = ' + json.dumps(registry['registry_revision']) + ';')
    digest = hashlib.sha256(b''.join(hashlib.sha256(raw[p]).digest() for p in sorted(raw))).hexdigest()
    out.append('static const char native_source_digest[] = ' + json.dumps(digest) + ';')
    def array(original: str, native: str, ctype: str, suffix: str = '', body: str = source) -> None:
        item = declaration(body, original)
        item = re.sub(r'^(?:static\s+)?const\s+\w+\s+' + original + r'([^=]+)=', lambda m: f'static const {ctype} {native}{m[1].strip()}{suffix} =', item)
        out.append(item)
    # Constants used by the retained declarative initializers, not an oracle header.
    for name, value in [('M3_NUC_A',0),('M3_NUC_T',1),('M3_NUC_C',2),('M3_NUC_G',3),('M3_MATRIX_COUNT',3),('M3_MATRIX_COMPLEMENTARY',0),('M3_MATRIX_MOVING_RESTING',1),('M3_MATRIX_SAME_QUALITY',2),('M3_MAJOR_ARCANA_COUNT',22),('M3_TAROT_SINGLE_CODON',255),('M3_TAROT_PIP_ACE',0),('M3_TAROT_PIP_PRINCESS',10),('M3_TAROT_PIP_PRINCE',11),('M3_TAROT_PIP_QUEEN',12),('M3_TAROT_PIP_KING',13),('M3_ROTATIONAL_NON_DUAL_INITIATED',0),('M3_ROTATIONAL_FULL_ROTATIONAL',1),('M3_ROTATIONAL_NO_PAIR',255),('M3_ROTATIONAL_NO_PAIRING',255)]:
        out.append(f'#define {name} {value}')
    array('NUCLEOTIDE_ICHING_VALUE','native_values','uint8_t',body=header)
    array('M3_PAIR_MATRIX','native_pairs','int8_t','[2]')
    array('M3_MATRIX_PAIR','native_matrix_pairs','uint8_t')
    array('M3_TRIGRAM_LUT','native_trigrams','uint16_t','[7]')
    array('M3_HEXAGRAM_LUT','native_hexagrams','uint8_t','[5]')
    for name, native in [('M3_COMP_MATRIX','native_comp'),('M3_MOVE_MATRIX','native_move'),('M3_RES_MATRIX','native_res'),('M3_CODON_TO_AA','native_aa')]:
        array(name,native,'uint8_t')
    array('M3_MAJOR_ARCANA','native_major','QL_M3_Major')
    tarot = source[source.index('#define COD('):source.index('#undef NONE')+len('#undef NONE')]
    tarot = tarot.replace('const M3_TarotCodonEntry M3_TAROT_CODON_MAP[4][16]', 'static const uint8_t native_minor[4][16][4]')
    out.append(tarot)
    profile = source[source.index('#define RCOD('):source.index('#undef R8P')+len('#undef R8P')]
    out.append(profile.replace('const M3_Rotational_Profile M3_ROTATIONAL_PROFILE[64]', 'static const uint8_t native_profiles[64][5]'))
    rows = clock_rows(raw[CLOCK].decode())
    out.append('static const uint16_t native_clock[360][26] = {\n' + ',\n'.join('{' + ','.join(map(str,row)) + '}' for row in rows) + '\n};')
    for i, group in enumerate(groups):
        out.append(f'static const QL_M_NodeId native_group_{i}[{len(group)}] = {{' + ','.join('UINT64_C(0x'+nodes[r]['id']+')' for r in group) + '};')
    out.append('static const QL_M_NodeId native_anchors[360] = {' + ','.join('UINT64_C(0x'+nodes[r]['id']+')' for r in anchors) + '};')
    return '\n'.join(out) + '\n'


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out',type=Path,required=True)
    args=parser.parse_args()
    content=generate()
    args.out.parent.mkdir(parents=True,exist_ok=True)
    if not args.out.exists() or args.out.read_text() != content:
        temporary=args.out.with_suffix(args.out.suffix+'.tmp')
        temporary.write_text(content)
        temporary.replace(args.out)

if __name__ == '__main__':
    main()
