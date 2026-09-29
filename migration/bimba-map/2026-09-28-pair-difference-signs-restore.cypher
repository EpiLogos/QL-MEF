// Restore the recorded pair-difference signs the coin migration overrode.
//
// 2026-09-28-d0-reauthor-and-coin.cypher corrected the pair sums and
// difference magnitudes to C=8/G=7 (owner ruling, #254), but it also rewrote
// four difference signs to x - y. The map had carried AG=GA and TC=CT as
// class-stable signs, and the ratified kernel correction M3-COIN-1 keeps every
// recorded sign verbatim (M3 unresolved item 2: the class-stable semantics of
// differenceValue is open). The sign change was not ruled, so the map returns
// to the recorded signs with the ruled magnitudes, matching the kernel.

// @probe
MATCH (n:Bimba) WHERE n.coordinate IN ['M3-2-1-4','M3-2-2-3','M3-2-3-4','M3-2-4-3']
RETURN n.coordinate, n.p_3_sequence, n.c_3_sum_value, n.c_3_difference_value ORDER BY n.coordinate;

// @apply
MATCH (n:Bimba {coordinate:'M3-2-1-4'}) WHERE n.c_3_sum_value = 13 AND n.c_3_difference_value = -1 SET n.c_3_difference_value = 1;
MATCH (n:Bimba {coordinate:'M3-2-2-3'}) WHERE n.c_3_sum_value = 17 AND n.c_3_difference_value = 1 SET n.c_3_difference_value = -1;
MATCH (n:Bimba {coordinate:'M3-2-3-4'}) WHERE n.c_3_sum_value = 15 AND n.c_3_difference_value = 1 SET n.c_3_difference_value = -1;
MATCH (n:Bimba {coordinate:'M3-2-4-3'}) WHERE n.c_3_sum_value = 15 AND n.c_3_difference_value = -1 SET n.c_3_difference_value = 1;

// @report
MATCH (n:Bimba) WHERE n.coordinate STARTS WITH 'M3-2-' AND n.c_3_sum_value IS NOT NULL
RETURN n.p_3_sequence AS pair, n.c_3_sum_value AS sum, n.c_3_difference_value AS diff ORDER BY n.coordinate;
