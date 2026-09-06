// 2026-09-06 — M1-branch fold-in, round 2: exponent genesis, double-cover reading,
// equation-as-frames identification, and the M1-4.5 reservation.
// Operator: zcode (ql-mef geometric foundation session, later pass).
// Discipline: idempotent (SET; coalesce for provenance). Logged in cypher/CYPHER-LOG.md.

// ── 1. M1-4: the exponent genesis — the 1-to-12 compositional line ──────────
MATCH (n {coordinate:'M1-4'})
SET n.c_2_exponent_genesis = 'Second Spanda as genesis, not signature. 16/9 = 2^4/3^2: four twos + two threes = six factors = the position hexad #0-#5 (named by this node\'s own 4.1 stage, -(4+2)). The 4.2 stage\'s ±(6+6) doubles the hexad into the twelvefold ring — 12:6 = 2:1, the ring is the octave of the position field. The kernel contract resolves the same ring natively (twelve lenses, six local positions per lens, 72 addresses); the 4.4 meta-frames carry the fields\' full counts — 2^6 = 64 and 12x6 = 72 (Paraśakti\'s double-covering of 36 = 6^2). Tetractys closure from both ends: 3+4 = 7 (additive — the Timaeus leg-sum; the 25:7 kernel torus equators) and 3x4 = 12 (multiplicative — the ring). Standing: the kernel coordinates are the 1 (the given ordering that governs and maps); this derivation is their 0 — the definitional layer of what # amounts to, retroactively explaining the genesis of the ordering law that already governs the coordinates. Refs: ql.holographic-kernel-contract/v1; integrated physical-pole object 1.4; FOLD-AND-RULING-GRAMMAR s0 re-grounding.',
    n.c_3_updated_at = '2026-09-06'
RETURN 'M1-4 exponent genesis' AS statement, n.coordinate AS node;

// ── 2. M1-3: the double-cover reading of the First Spanda ───────────────────
MATCH (n {coordinate:'M1-3'})
SET n.c_2_double_cover_reading = 'The equation\'s two operations are the two circuits of the double cover. The division ((0/1)/(1/0)) is the first traversal — arrival at the antipode, face-inverted: the alias, Maya as the single-circuit reading. The summation (1/0 + 0/1) is the second — return to the same origin, inverted-but-recognized: 1/1 = 100% is the completed 720. T1/T2 are the two helices (12 ring positions x 2 = 24 spokes); the Mahamaya track (64, the doubling) and the Paraśakti track (72, double-cover of 36) are the two circuits\' accumulated contents. Pratyabhijna is the second circuit\'s operator (M1-1: 1/0 as the return-switch, same terms same slash opposite traversal). Kernel seats: same-position conjugacy n <-> n\' is one circuit; DOUBLE_COVER_DEG = 720 at M1-5 is the recognition. The heartbeat\'s unit of identity is the double-beat.',
    n.c_3_updated_at = '2026-09-06'
RETURN 'M1-3 double cover' AS statement, n.coordinate AS node;

// ── 3. M1-3-4: the equation and the frame ladder are one sequence ───────────
MATCH (n {coordinate:'M1-3-4'})
SET n.c_2_equation_as_frames = 'This node\'s complete formulation and its Context-Frame sub-stages are one sequence: 0000 <-> 0/0 (CF1, the double-zero ground); (0/1) <-> ((0/1)/(1/0)) (CF2, the first circuit — the division); (0/1/2) <-> the T1/T2 tracks (CF3, the mediating third); (0/1/2/3) <-> (1/0 + 0/1) (CF4, the second circuit — the summation); (4.0/1-4.4/5) <-> the flowering of the percentile (CF5); (5/0) <-> 1/1 (CF7, the return — recognition). The kernel contract\'s seven canonical frames resolve over the same field this node already holds: Spanda\'s activities across the context frame are this node\'s own architecture.',
    n.c_3_updated_at = '2026-09-06'
RETURN 'M1-3-4 equation as frames' AS statement, n.coordinate AS node;

// ── 4. M1-4.5: reservation of the [4.5] space — the harmonic/musical genesis ──
MATCH (n {coordinate:'M1-4.5'})
SET n.c_2_reserved_genesis = 'The full harmonic and musical genesis is reserved for this space: the #31 development edge (ql-musical-derivation-v3, the pre-M musical derivation, the Jankó 4:2 figure) lands here as the map-side account of the coordinates\' musical law. Currently empty by design — the refinements stream in over time. Standing (2026-09-06): taking the kernel coordinates as the 1 — the given ordering that already governs and maps — the matheme derivation is their 0, the definitional layer expressing what the internal dynamics of # amount to, which retroactively explains the genesis of the ordering law (QL) that governs them. This node is where that 0-layer\'s harmonic face will flower.',
    n.c_3_updated_at = '2026-09-06'
RETURN 'M1-4.5 reservation' AS statement, n.coordinate AS node;
