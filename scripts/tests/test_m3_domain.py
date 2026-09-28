import copy, importlib.util, json, os, unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
spec=importlib.util.spec_from_file_location('domain',ROOT/'scripts/m3-domain.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
class DomainTests(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.data=json.loads((ROOT/m.PATH).read_text());cls.registry=json.loads((ROOT/m.SOURCE.REGISTRY).read_text())
 def resigned(self,d):
  d.pop('catalogue_revision',None);d['catalogue_revision']=m.SOURCE.digest(d);return d
 def test_exact_map_projection(self):
  # Recompute from a read of the registry's map when present; never skip the proof.
  read=m.SOURCE.bimba_map.CACHE
  if read.is_file() and m.SOURCE.bimba_map.load(read)['content_sha256']==self.registry['source_revision']:
   self.assertEqual(m.build(read),self.data)
  m.verify(self.data,self.registry)
 def test_truncated_source_is_rejected(self):
  d=copy.deepcopy(self.data);d['relations'].pop()
  with self.assertRaises(ValueError):m.verify(self.resigned(d),self.registry)
 def test_wrong_node_identity_is_rejected(self):
  d=copy.deepcopy(self.data);d['nodes'][0]['ref']='#2'
  with self.assertRaises(ValueError):m.verify(self.resigned(d),self.registry)
 def test_wrong_relation_endpoint_is_rejected(self):
  d=copy.deepcopy(self.data);d['relations'][0]['to_id']='1234567890abcdef'
  with self.assertRaises(ValueError):m.verify(self.resigned(d),self.registry)
 def test_backbones_take_only_map_facts(self):
  self.assertEqual(len(self.data['backbones']),24)
  self.assertEqual(len({b['id'] for b in self.data['backbones']}),24)
  nodes={n['id']:n for n in self.data['nodes']}
  for b in self.data['backbones']:
   # No map backbone -> hexagram fact; the codon only via EMBODIES_PALINDROMIC_CODON.
   self.assertIsNone(b['hexagram_id']);self.assertIsNone(b['hexagram_address'])
   if b['codon_id']:
    seq=nodes[b['codon_id']]['properties']['p_3_sequence'];self.assertEqual(seq,seq[::-1])
 def test_all_qualified_matrix_links_remain_real_source_edges(self):
  ids={e['id'] for e in self.data['relations']}
  for c in self.data['matrix_cells']:
   self.assertTrue(set(c['pair_relations']+c['codon_relations']+[c['resolves_relation']])<=ids)
if __name__=='__main__':unittest.main()
