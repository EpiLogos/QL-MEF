#!/usr/bin/env python3
"""Generate closed wire shapes from the actual Rust M2 types; validate fixtures.

Schema checks structure/ranges. Native validation additionally checks same-event
identity, exact registry coordinates, unique keys, chronology and fibre locality.
No handwritten second set of wire field names is authoritative.
"""
from __future__ import annotations
import argparse,json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
SOURCES=['crates/ql-mef/src/m2_engine.rs','crates/ql-mef/src/m2.rs','crates/ql-mef/src/m2_vimarsha.rs','crates/ql-mef/src/m2_condition.rs']
MAX=9007199254740991

def schemas():
    structs={}
    for path in SOURCES:
        text=(ROOT/path).read_text()
        for name,body in re.findall(r'pub struct (\w+) \{\n(.*?)^\}',text,re.S|re.M):
            fields=re.findall(r'^\s*pub (\w+): ([^\n]+),$',body,re.M)
            structs[name]=fields
    def shape(typ):
        typ=typ.strip().replace('m2::','').replace('crate::m2_vimarsha::','').replace('crate::m2_condition::','')
        if typ.startswith('Option<'):return {'anyOf':[shape(typ[7:-1]),{'type':'null'}]}
        if typ.startswith('Vec<'):return {'type':'array','items':shape(typ[4:-1])}
        if typ.startswith('BTreeMap<String, '):return {'type':'object','additionalProperties':shape(typ[17:-1])}
        if typ.startswith('['):
            inner,n=typ[1:-1].rsplit(';',1);return {'type':'array','items':shape(inner),'minItems':int(n),'maxItems':int(n)}
        if typ in ['f32','f64']:return {'type':'number'}
        if typ=='String':return {'type':'string'}
        if typ=='bool':return {'type':'boolean'}
        if typ=='MTreeId':return {'type':'string','pattern':'^[0-9a-f]{16}$'}
        if re.fullmatch('[ui](8|16|32|64|128)',typ) or typ=='usize':
            bits=int(typ[1:]) if typ!='usize' else 64
            return {'type':'integer','minimum':0 if typ.startswith('u') else -(2**(bits-1)),
                'maximum':min(MAX,2**(bits if typ.startswith('u') else bits-1)-1)}
        if typ in structs or typ in ['MaterialFibre','VimarshaHelix','CorrespondenceRole','TuningPolicy']:return {'$ref':'#/$defs/'+typ}
        raise ValueError('unhandled actual Rust wire type: '+typ)
    defs={'MaterialFibre':{'enum':['earth','fire','water','air']},'VimarshaHelix':{'enum':['bimba','pratibimba']},'CorrespondenceRole':{'enum':['tonic','dominant']},'TuningPolicy':{'enum':['retained24_tet','bimba_spelled24_tet']}}
    for name,fields in structs.items():
        if not fields:continue
        defs[name]={'type':'object','additionalProperties':False,'properties':{k:shape(t) for k,t in fields},
            'required':[k for k,t in fields if not t.startswith('Option<')]}
    def props(name):return defs[name]['properties']
    props('DescriptorReading')['fields']={'type':'object','additionalProperties':{'type':'string','pattern':'^(0|[1-9][0-9]*)$','maxLength':20}}
    props('DescriptorReading')['standing']={'const':'retained-source-reading-not-whole-subsystem-parity'}
    props('M2Request')['schema']={'const':'ql.m2-engine-request/v1'};props('M2Frame')['schema']={'const':'ql.m2-engine/v1'}
    for n in ['M2Request','M2Frame']:props(n)['registry_revision']={'type':'string','pattern':'^[0-9a-f]{64}$'}
    for n in ['M2Request','M2Frame','VimarshaSeed']:props(n)['tick12']['maximum']=11
    for n in ['M2Request','M2Frame']:props(n)['degree720']['maximum']=719
    for n in ['VimarshaInput','VimarshaSeed']:
        props(n)['lens']['maximum']=11;props(n)['musical_mode']['maximum']=6
        props(n)['harmonic_ratio']['items']['minimum']=1
    props('VimarshaInput')['pose_ordinal']['maximum']=471
    props('VimarshaSeed')['codon']['maximum']=63;props('VimarshaSeed')['rotation']['maximum']=7
    props('NodalConstraint')['ql_position']={'enum':[0,5]}
    for k in ['m','n']:props('NodalConstraint')[k].update(minimum=1,maximum=12)
    for name,field,n,bound in [('M2Request','modal_coefficients',72,1000000),('ModalState','coefficients',72,1000000),('ModalState','quadrature',72,1000000),('ModalState','form_potential',64,2000000)]:
        a=props(name)[field];a.update(minItems=n,maxItems=n);a['items']['items'].update(minimum=-bound,maximum=bound)
    for field,n in [('selections',764),('mef_conditions',72),('context_frames',7),('world_observations',10)]:props('M2Request')[field]['maxItems']=n
    props('M2Request')['mef_conditions']['items']['maximum']=71
    props('WorldObservation')['planet_id']['maximum']=9
    props('WorldObservation')['longitude_degrees'].update(minimum=0,exclusiveMaximum=360)
    props('ContinuousMode')['frequency_hz']['exclusiveMinimum']=0
    props('ContinuousMode')['damping_per_second']['minimum']=0
    props('ContinuousMode')['carrier_weights'].update(minItems=1,maxItems=18)
    props('CarrierWeight')['carrier']['maximum']=71
    props('ResonatorState')['modes'].update(minItems=1,maxItems=4096)
    props('M2Condition')['schema']={'const':'ql.m2-condition/v1'}
    props('M2ConditionInput')['maqam_index']['maximum']=71
    props('M2ConditionInput')['active_mef_condition']['maximum']=71
    props('M2ConditionInput')['tonic_hz'].update(exclusiveMinimum=0,maximum=1000000)
    props('RenderPalette')['entries']['maxItems']=32
    props('NamedPaletteEntry')['linear_rgba']['items'].update(minimum=0,maximum=1)
    def document(root,ident):
        needed=set()
        def walk(value):
            if isinstance(value,dict):
                if '$ref' in value:
                    name=value['$ref'].split('/')[-1]
                    if name not in needed:needed.add(name);walk(defs[name])
                for v in value.values():walk(v)
            elif isinstance(value,list):
                for v in value:walk(v)
        walk({'$ref':'#/$defs/'+root})
        return {'$schema':'https://json-schema.org/draft/2020-12/schema','$id':ident,
            'description':'Generated from Rust M2 wire types. Native validation also enforces semantic invariants; schema acceptance alone is not engine readiness.',
            '$ref':'#/$defs/'+root,'$defs':{k:defs[k] for k in sorted(needed)}}
    return {'m2-engine-request-v1.schema.json':document('M2Request','ql.m2-engine-request/v1'),
            'm2-engine-frame-v1.schema.json':document('M2Frame','ql.m2-engine/v1')}

def main():
    p=argparse.ArgumentParser();p.add_argument('command',choices=['refresh','check']);p.add_argument('--frame',type=Path);args=p.parse_args()
    documents=schemas()
    import jsonschema
    for name,d in documents.items():
        text=json.dumps(d,ensure_ascii=False,indent=2)+'\n';path=ROOT/'fixtures/kernel'/name
        jsonschema.Draft202012Validator.check_schema(d)
        if args.command=='refresh':path.write_text(text)
        elif path.read_text()!=text:raise ValueError('stale generated M2 shape: '+name)
    for fixture in ['m2-engine-request-v1.json','m2-condition-request-v1.json']:
        request=json.loads((ROOT/'fixtures/kernel'/fixture).read_text())
        jsonschema.validate(request,documents['m2-engine-request-v1.schema.json'])
    if args.frame:jsonschema.validate(json.loads(args.frame.read_text()),documents['m2-engine-frame-v1.schema.json'])
    print('Actual Rust M2 wire shapes and supplied fixtures verified')
if __name__=='__main__':main()
