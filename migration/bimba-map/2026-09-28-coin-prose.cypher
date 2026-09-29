// Carry C=8/G=7 into the two map descriptions that still state C=7/G=8.
// Owner ruling on #254: the map's 7/8 is an error to correct at the map.
// The same text is corrected at its sources in C-Experiments
// (Idea/Bimba/Map/M3/M3-1/M3-1.md and the recovery cypher).

// @probe
MATCH (n:Bimba) WHERE n.coordinate IN ['M3-1','M4-2']
RETURN n.coordinate, n.q_1_book_of_changes_definite CONTAINS 'C=7, G=8', n.q_1_multi_traditional_oracle_system CONTAINS 'C=7/G=8';

// @apply
MATCH (n:Bimba {coordinate:'M3-1'}) WHERE n.q_1_book_of_changes_definite CONTAINS 'A=6, T=9, C=7, G=8'
SET n.q_1_book_of_changes_definite = replace(n.q_1_book_of_changes_definite, 'A=6, T=9, C=7, G=8', 'A=6, T=9, C=8, G=7');
MATCH (n:Bimba {coordinate:'M4-2'}) WHERE n.q_1_multi_traditional_oracle_system CONTAINS 'A=6/T=9/C=7/G=8'
SET n.q_1_multi_traditional_oracle_system = replace(n.q_1_multi_traditional_oracle_system, 'A=6/T=9/C=7/G=8', 'A=6/T=9/C=8/G=7');

// @report
MATCH (n:Bimba) UNWIND keys(n) AS k WITH n, k WHERE n[k] IS :: STRING AND n[k] CONTAINS 'C=7'
RETURN count(*) AS remaining_c7_mentions;
