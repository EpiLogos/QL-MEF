#!/usr/bin/env python3
"""Verify retained source captures by executing the pinned original Rust registry.

Only the temporary exporter, target directory and Cargo cache are written. The
Original checkout stays read-only. This is a source-equivalence test, not a graph
service, native authority grant or runtime assertion acceptance.
"""
import argparse
import gzip
import hashlib
import json
import shutil
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / 'docs/integrations/epi-logos'

EXPORT = r'''use epi_s2_graph_schema as s;
use serde_json::json;
fn main() {
    let properties: Vec<_> = s::NODE_PROPERTY_SPECS.iter()
        .chain(s::RELATIONSHIP_PROPERTY_SPECS.iter()).map(|p| json!({
            "key":p.key,"coordinate_home":p.coordinate_home,
            "owner":format!("{:?}",p.owner),"value_type":format!("{:?}",p.value_type),
            "cardinality":format!("{:?}",p.cardinality),
            "disclosure":format!("{:?}",p.disclosure),"source_family":p.source_family,
            "indexed":p.indexed,"compatibility":p.compatibility
        })).collect();
    println!("{}", json!({"schema_version":s::SCHEMA_VERSION,"properties":properties,
        "semantic_registry":s::coordinate_semantic_registry(),
        "embedding_dimensions":s::SEMANTIC_EMBEDDING_DIMENSIONS}));
}
'''

def blob(path):
    data = path.read_bytes()
    return hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()


def check(original):
    props = json.loads(gzip.decompress((BASE / 'aw-property-source.json.gz').read_bytes()))
    participation = json.loads(gzip.decompress((BASE / 'aw-original-participation.json.gz').read_bytes()))
    for source in (props['source'], participation['source']):
        if blob(original / source['path']) != source['git_blob']:
            raise ValueError('original source does not match its captured blob: ' + source['path'])
    native = json.loads((original / participation['source']['path']).read_text())
    if native != participation['source_fields']:
        raise ValueError('captured Pleroma participation differs from its original source')
    with tempfile.TemporaryDirectory(prefix='aw0-source-export-') as temporary:
        directory = Path(temporary)
        (directory / 'src').mkdir()
        (directory / 'src/main.rs').write_text(EXPORT)
        source_path = json.dumps(str((original / 'Body/S/S2/graph-schema').resolve()))
        (directory / 'Cargo.toml').write_text(
            '[package]\nname="aw-source-schema-export"\nversion="0.0.0"\nedition="2021"\n'
            '[dependencies]\nepi-s2-graph-schema={path=' + source_path + '}\nserde_json="1"\n')
        shutil.copy(ROOT / 'scripts/aw0-source-export.lock', directory / 'Cargo.lock')
        output = subprocess.check_output(
            ['cargo', 'run', '--quiet', '--locked', '--manifest-path', str(directory / 'Cargo.toml')])
        actual = json.loads(output)
    captured = [dict(zip(props['columns'], row, strict=True)) for row in props['properties']]
    if actual['properties'] != captured:
        raise ValueError('original executable property definitions differ from captured definitions')
    if actual['schema_version'] != props['original_schema_version']:
        raise ValueError('original property schema version differs')
    if actual['semantic_registry'] != props['semantic_registry'] or actual['embedding_dimensions'] != props['embedding_dimensions']:
        raise ValueError('original coordinate semantic registry or dimensional law differs')
    return {'property_definitions': len(captured), 'source_equivalence': True,
            'runtime_completion': False, 'original_mutated': False}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('original', type=Path)
    args = parser.parse_args()
    print(json.dumps(check(args.original), sort_keys=True))
