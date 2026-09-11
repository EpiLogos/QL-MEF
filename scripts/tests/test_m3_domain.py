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
 def test_exact_source_projection(self):
  root=os.environ.get('M3_SOURCE_ROOT')
  self.assertTrue(root,'M3_SOURCE_ROOT required, no skipped source proof')
  self.assertEqual(m.build(Path(root)),self.data)
 def test_truncated_source_is_rejected(self):
  d=copy.deepcopy(self.data);d['relations'].pop()
  with self.assertRaises(ValueError):m.verify(self.resigned(d),self.registry)
 def test_wrong_node_identity_is_rejected(self):
  d=copy.deepcopy(self.data);d['nodes'][0]['ref']='#2'
  with self.assertRaises(ValueError):m.verify(self.resigned(d),self.registry)
 def test_wrong_relation_endpoint_is_rejected(self):
  d=copy.deepcopy(self.data);d['relations'][0]['to_id']='1234567890abcdef'
  with self.assertRaises(ValueError):m.verify(self.resigned(d),self.registry)
 def test_backbone_prototype_is_not_current_form_or_boolean(self):
  self.assertEqual(len(self.data['backbones']),24)
  self.assertEqual(len({b['id'] for b in self.data['backbones']}),24)
  self.assertTrue(any(b['codon_address']!=b['hexagram_address'] for b in self.data['backbones']))
 def test_all_qualified_matrix_links_remain_real_source_edges(self):
  ids={e['id'] for e in self.data['relations']}
  for c in self.data['matrix_cells']:
   self.assertTrue(set(c['pair_relations']+c['codon_relations']+[c['resolves_relation']])<=ids)
if __name__=='__main__':unittest.main()
