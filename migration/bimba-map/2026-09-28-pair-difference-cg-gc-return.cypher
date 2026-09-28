// Return CG and GC to x - y after the sign restore flipped them in error.
//
// 2026-09-28-pair-difference-signs-restore.cypher read the kernel's comments,
// which label M3_PAIR_MATRIX[11] "GC" and [14] "CG". The table is indexed
// (n1 << 2) | n2 with C = 2 and G = 3, so [11] is CG (+1) and [14] is GC (-1):
// x - y under C=8/G=7. The ratified kernel's law is x - y for every pair except
// the cross-complementary class-stable pair signs, where AG carries GA's value
// and TC carries CT's. AG and TC stay as restored; CG and GC return here.

// @probe
MATCH (n:Bimba) WHERE n.coordinate IN ['M3-2-3-4','M3-2-4-3']
RETURN n.coordinate, n.p_3_sequence, n.c_3_sum_value, n.c_3_difference_value ORDER BY n.coordinate;

// @apply
MATCH (n:Bimba {coordinate:'M3-2-3-4'}) WHERE n.p_3_sequence = 'CG' AND n.c_3_sum_value = 15 AND n.c_3_difference_value = -1 SET n.c_3_difference_value = 1;
MATCH (n:Bimba {coordinate:'M3-2-4-3'}) WHERE n.p_3_sequence = 'GC' AND n.c_3_sum_value = 15 AND n.c_3_difference_value = 1 SET n.c_3_difference_value = -1;

// @report
MATCH (n:Bimba) WHERE n.coordinate STARTS WITH 'M3-2-' AND n.c_3_sum_value IS NOT NULL
RETURN n.p_3_sequence AS pair, n.c_3_sum_value AS sum, n.c_3_difference_value AS diff ORDER BY n.coordinate;
