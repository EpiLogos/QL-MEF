// L0 question words follow the owner's ruling (O-I #707): L0 reads
// 0 Why · 1 What · 2 How · 3 Who / Which / Whereby · 4 Where / When / Whither · 5 Why-for / Why-not.
// The map's L0-3, L0-4 and L0-5 still taught the shifted form (O-I #721). This corrects their
// c_1_name and the question words that open l_0_position_interpretation; the rest of each
// sentence and every other property stay as written. Each write is guarded by the value it
// replaces, so a re-run writes nothing. Rollback: 2026-10-28-l0-question-words.rollback.cypher.
//   dry run:  python3 migration/bimba-map/run.py migration/bimba-map/2026-10-28-l0-question-words.cypher
//   apply:    python3 migration/bimba-map/run.py migration/bimba-map/2026-10-28-l0-question-words.cypher --commit
// After applying: scripts/refresh-from-bimba-map.sh --endpoint http://100.92.62.101:7474.
//
// Applied 28 Oct 2026, 20:11 UTC: 6 guarded writes (properties_set 6); a re-run writes none;
// the map holds 2149 nodes and 13907 relations before and after. Pre-write dump:
// oi-omarchy /mnt/hdd/vm/bimba-premigration/bimba-2026-10-28-2010-pre-o-i-721.dump
// (sha256 5f8f820ecc56af9f2b266ecd3256a60e8242a738cce466830363eefb58b27e6f), copied to the Mac at
// Work/Quaternal-Logic/.neo4j-backups/pre-migration/.

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

// @report after (L0-0 … L0-5: name and interpretation)
MATCH (n:Bimba) WHERE n.coordinate IN ['L0-0','L0-1','L0-2','L0-3','L0-4','L0-5']
RETURN n.coordinate, n.c_1_name, n.l_0_position_interpretation ORDER BY n.coordinate;

// @report shifted-form strings left anywhere on the map (expect 0)
MATCH (n:Bimba) UNWIND keys(n) AS k WITH n, k WHERE n[k] IS :: STRING
AND (n[k] CONTAINS 'Why-four' OR n[k] CONTAINS 'why-four' OR n[k] CONTAINS 'Why-so /' OR n[k] CONTAINS 'Why-so/why-not' OR n[k] CONTAINS 'Whom / Which / When' OR n[k] CONTAINS 'Who/which/when ')
RETURN count(*) AS remaining;
