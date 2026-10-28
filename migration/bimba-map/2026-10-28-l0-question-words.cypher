// L0 question words follow the owner's ruling (O-I #707): L0 reads
// 0 Why · 1 What · 2 How · 3 Who / Which / Whereby · 4 Where / When / Whither · 5 Why-for / Why-not.
// The map's L0-3, L0-4 and L0-5 still taught the shifted form (O-I #721). This corrects their
// c_1_name, the question words that open l_0_position_interpretation, and the same sentence
// where the map repeats it as the `interpretation` of each node's INTERPRETS_QL_POSITION edge
// to P3, P4 and P5; the rest of each sentence and every other property stay as written.
// Each write is guarded by the value it replaces, so a re-run writes nothing.
// Rollback: 2026-10-28-l0-question-words.rollback.cypher.
//   dry run:  python3 migration/bimba-map/run.py migration/bimba-map/2026-10-28-l0-question-words.cypher
//   apply:    python3 migration/bimba-map/run.py migration/bimba-map/2026-10-28-l0-question-words.cypher --commit
// After applying: scripts/refresh-from-bimba-map.sh --endpoint http://100.92.62.101:7474.
//
// Applied 28 Oct 2026 in two passes, each after a dump of the database on oi-omarchy under
// /mnt/hdd/vm/bimba-premigration/ (copied to the Mac at Work/Quaternal-Logic/.neo4j-backups/pre-migration/):
//   20:11 UTC  node properties, 6 guarded writes; dump bimba-2026-10-28-2010-pre-o-i-721.dump
//              (sha256 5f8f820ecc56af9f2b266ecd3256a60e8242a738cce466830363eefb58b27e6f).
//   20:27 UTC  the three edge interpretations, 3 guarded writes (the node writes re-ran as none);
//              dump bimba-2026-10-28-2025-pre-o-i-721-edges.dump
//              (sha256 55f211baa8258ab10d8e1c43ab0cfed0d8883e034ac33d434ddc125b613d9fc7).
// A re-run writes nothing; the map holds 2149 nodes and 13907 relations before and after.

// @probe before (L0-3, L0-4, L0-5: name and interpretation)
MATCH (n:Bimba) WHERE n.coordinate IN ['L0-3','L0-4','L0-5']
RETURN n.coordinate, n.c_1_name, n.l_0_position_interpretation ORDER BY n.coordinate;

// @apply L0-3 Who / Which / Whereby
MATCH (n:Bimba {coordinate:'L0-3'}) WHERE n.c_1_name = 'Whom / Which / When' SET n.c_1_name = 'Who / Which / Whereby';
MATCH (n:Bimba {coordinate:'L0-3'}) WHERE n.l_0_position_interpretation = 'Who/which/when reads P3 as pattern and determining form.' SET n.l_0_position_interpretation = 'Who/which/whereby reads P3 as pattern and determining form.';

// @apply L0-4 Where / When / Whither
MATCH (n:Bimba {coordinate:'L0-4'}) WHERE n.c_1_name = 'Where / When / Why-four' SET n.c_1_name = 'Where / When / Whither';
MATCH (n:Bimba {coordinate:'L0-4'}) WHERE n.l_0_position_interpretation = 'Where/when/why-four reads P4 as context and horizon.' SET n.l_0_position_interpretation = 'Where/when/whither reads P4 as context and horizon.';

// @apply L0-5 Why-for / Why-not
MATCH (n:Bimba {coordinate:'L0-5'}) WHERE n.c_1_name = 'Why-so / Why-not' SET n.c_1_name = 'Why-for / Why-not';
MATCH (n:Bimba {coordinate:'L0-5'}) WHERE n.l_0_position_interpretation = 'Why-so/why-not reads P5 as synthesis and return.' SET n.l_0_position_interpretation = 'Why-for/why-not reads P5 as synthesis and return.';

// @apply INTERPRETS_QL_POSITION edges L0-3→P3, L0-4→P4, L0-5→P5 (the same interpretation sentence)
MATCH (:Bimba {coordinate:'L0-3'})-[r:INTERPRETS_QL_POSITION]->(:Bimba {coordinate:'P3'}) WHERE r.interpretation = 'Who/which/when reads P3 as pattern and determining form.' SET r.interpretation = 'Who/which/whereby reads P3 as pattern and determining form.';
MATCH (:Bimba {coordinate:'L0-4'})-[r:INTERPRETS_QL_POSITION]->(:Bimba {coordinate:'P4'}) WHERE r.interpretation = 'Where/when/why-four reads P4 as context and horizon.' SET r.interpretation = 'Where/when/whither reads P4 as context and horizon.';
MATCH (:Bimba {coordinate:'L0-5'})-[r:INTERPRETS_QL_POSITION]->(:Bimba {coordinate:'P5'}) WHERE r.interpretation = 'Why-so/why-not reads P5 as synthesis and return.' SET r.interpretation = 'Why-for/why-not reads P5 as synthesis and return.';

// @report edges after (L0-0 … L0-5 → P: interpretation)
MATCH (a:Bimba)-[r:INTERPRETS_QL_POSITION]->(b:Bimba) WHERE a.coordinate IN ['L0-0','L0-1','L0-2','L0-3','L0-4','L0-5']
RETURN a.coordinate, b.coordinate, r.interpretation ORDER BY a.coordinate;

// @report after (L0-0 … L0-5: name and interpretation)
MATCH (n:Bimba) WHERE n.coordinate IN ['L0-0','L0-1','L0-2','L0-3','L0-4','L0-5']
RETURN n.coordinate, n.c_1_name, n.l_0_position_interpretation ORDER BY n.coordinate;

// @report shifted-form strings left anywhere on the map (expect 0)
MATCH (n:Bimba) UNWIND keys(n) AS k WITH n, k WHERE n[k] IS :: STRING
AND (n[k] CONTAINS 'Why-four' OR n[k] CONTAINS 'why-four' OR n[k] CONTAINS 'Why-so /' OR n[k] CONTAINS 'Why-so/why-not' OR n[k] CONTAINS 'Whom / Which / When' OR n[k] CONTAINS 'Who/which/when ')
RETURN count(*) AS remaining;

// @report shifted-form strings left on any relation (expect 0)
MATCH ()-[r]->() UNWIND keys(r) AS k WITH r, k WHERE r[k] IS :: STRING
AND (r[k] CONTAINS 'why-four' OR r[k] CONTAINS 'Why-so/why-not' OR r[k] CONTAINS 'Who/which/when ')
RETURN count(*) AS remaining;
