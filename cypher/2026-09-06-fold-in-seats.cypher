// 2026-09-06 — Fold-in across the seats: M1-5, M2-0, M3-0, M2-4, L2' family
// Operator: zcode (ql-mef geometric foundation session)
// Discipline: idempotent throughout — SET for properties (naturally idempotent),
// coalesce() for provenance fields, MERGE ... ON CREATE SET / ON MATCH SET for relationships.
// Every statement here is logged granularly in cypher/CYPHER-LOG.md with its result.

// ── 1. M2-4 rename: Vibrational Arena → Vibrational Templateure ─────────────
// Owner acceptance 2026-09-06: drop the "Parashakti:" prefix (implied by M2 origin)
// and "of Archetypal Powers" (kept as appellation props).
MATCH (n {coordinate:'M2-4'})
SET n.c_1_name = 'Vibrational Templateure',
    n.c_1_former_name = coalesce(n.c_1_former_name, 'Parashakti: Vibrational Arena of Archetypal Powers'),
    n.c_1_primary_designation = 'Vibrational Templateure — the modal form-potential storehouse of Paraśakti',
    n.c_1_appellations = ['Arena of Archetypal Powers', 'Cosmic Resonance Chamber of Vibrational-Harmonic Manifestation'],
    n.c_2_templateure = 'V72 = V18(Earth) ⊕ V18(Fire) ⊕ V18(Water) ⊕ V18(Air); T72→64 = I4 ⊗ T18→16 — the archetypal/modal storehouse of lawful potential forms, indexed by the resonant disposition of the same physical body, that M3 resolves into determinate rūpa via the 9:8 epogdoon (M2-C25; integrated physical-pole object §3)',
    n.c_3_updated_at = '2026-09-06'
RETURN 'M2-4 rename' AS statement, n.coordinate AS node;

// ── 2. M1-5: ruling surface + torus measures ────────────────────────────────
MATCH (n {coordinate:'M1-5'})
SET n.c_5_ruling_surface = 'P (position) and L (lens) as the two ruling families of the doubly-ruled quadric: every address is one P-line crossing one L-line. The torus is its projective closure — each ruling family closing into one of the two generating circles (RP1 × RP1 ≅ T2). Villarceau sections are the ruling shadows; torus, cylinder and hyperboloid are one orbit under sphere inversion (Dupin cyclides); the natural stage is S3 — the unit quaternions — where the Clifford torus stereographically projects the two generating families (the Hopf setting this node already carries). K2 is the twist-identification of the same square: the retrospective face of the same rulings.',
    n.c_2_torus_measures = 'Kernel constants R = 16/9, r = 1: outer/inner equators (R+r):(R−r) = 25:7 — the per-element pentad 25 = 9+16 = 5² over the Timaeus leg-sum 7 = 3+4. Villarceau slicing angle sin θ = r/R = 9/16 = (3/4)² — the fourth, squared. The torus recites the arithmetic the map attests.',
    n.c_3_updated_at = '2026-09-06'
RETURN 'M1-5 rulings' AS statement, n.coordinate AS node;

// ── 3. M2-0: pentad four faces + curvature seat ─────────────────────────────
MATCH (n {coordinate:'M2-0'})
SET n.c_2_pentad_four_faces = 'Hypotenuse: per elemental carrier 9+16 = 25 — 3²+4² = 5², 100 = 4×25 — the unique consecutive-integer right triangle, the only rational closure of the two generator-legs. Partition: 100/5 = 20 = 360°/18, the pentadic quotient of the M1 totality as the angular quantum of an elemental fibre. Ratio: φ = 2cos36°, worst-approximable real, field-disjoint from Q(√2,√3) — the never-closing face; the golden flow fills the torus most uniformly and is the last invariant torus to break (KAM). Container: outside the four-element array; closes finitely at 120 in S3 (icosians). Five closes the count and opens the ratio.',
    n.c_2_curvature_seat = 'The open — hyperbolic (χ<0): φ-tilings of the Poincaré disk, infinite expansions in finite space (M1-5 c_5_hyperbolic_integration); pentad-potentiation between the flat hinge (M1-5 torus, χ=0) and the closed (M3 spherical forms, χ>0). Gauss–Bonnet: the manifestational spine as curvature contraction; the zero is the curvatural pivot between the never-closing and the closed.',
    n.c_3_updated_at = '2026-09-06'
RETURN 'M2-0 pentad+curvature' AS statement, n.coordinate AS node;

// ── 4. M3-0: two-descriptions law (aliasing) + epogdoon two-way door ────────
MATCH (n {coordinate:'M3-0'})
SET n.c_2_two_descriptions_law = 'The 72→64 many-to-one map reads as 8 codomain collisions from the address side and 9 non-closing source fold-points from the flow side — two descriptions of one map, never one gap count. The aliasing law beneath both: sample a dense (irrational) flow at a rational rate and the closure you see is the sampling\'s, not the flow\'s. The DET floor(i·8/9) is a sampling of the 72-fold flow at the epogdoon rate.',
    n.c_2_epogdoon_two_way_door = 'Descent: 72 × 8/9 = 64 — manifestation, nāma→rūpa, the DET. Ascent: 64 →(9/8)→ 72 — recognition, 137 = 1+64+72 as the conjugate relational account (Third Spanda). T0/T1 holds both arrows as one state\'s simultaneous descriptions. (64/36)×(9/8) = 2/1 — the essay movement-26 identity: octave return through, not by eliminating, its remainder.',
    n.c_3_updated_at = '2026-09-06'
RETURN 'M3-0 aliasing+door' AS statement, n.coordinate AS node;

// ── 5. L2-5' erratum: Mineral → Salt (register erratum; Rust registry canonical) ──
MATCH (n {coordinate:'L2-5\''})
SET n.c_1_name = 'Salt',
    n.c_1_former_name = coalesce(n.c_1_former_name, 'Mineral'),
    n.c_3_updated_at = '2026-09-06'
RETURN 'L2-5 prime erratum' AS statement, n.coordinate AS node;

// ── 6. L2' parent + sublens family: solids register (Offered-grade) ─────────
MATCH (n {coordinate:'L2\''})
SET n.c_2_solids_register = 'THE ELEMENT-BEARING LENS as the Timaeus completed: 0 Aether/dodecahedron (withheld — the container; φ, Q(√5); closes at 120 in S3), 1 Earth/cube (half-square, √2), 2 Water/icosahedron (half-equilateral, √3), 3 Air/octahedron (√3), 4 Fire/tetrahedron (√3), 5 Salt — the container grown, the achieved second body, Möbius complement of Aether within L2\'. Offered-grade annotation (essay movement 33 gate) 2026-09-06.',
    n.c_3_updated_at = '2026-09-06'
RETURN 'L2 prime solids register' AS statement, n.coordinate AS node;

MATCH (n) WHERE n.coordinate IN ['L2-0\'','L2-1\'','L2-2\'','L2-3\'','L2-4\'','L2-5\'']
SET n.c_2_platonic_solid = CASE n.coordinate
      WHEN 'L2-0\'' THEN 'dodecahedron — withheld: the container (φ, Q(√5)); outside the four-element array; Offered-grade per movement 33 gate'
      WHEN 'L2-1\'' THEN 'cube — built from the half-square (√2); earth'
      WHEN 'L2-2\'' THEN 'icosahedron — built from the half-equilateral (√3); water'
      WHEN 'L2-3\'' THEN 'octahedron — built from the half-equilateral (√3); air'
      WHEN 'L2-4\'' THEN 'tetrahedron — built from the half-equilateral (√3); fire'
      WHEN 'L2-5\'' THEN 'no solid — Salt: the container grown, the achieved second body; Möbius complement of Aether within L2\'' END,
    n.c_3_updated_at = '2026-09-06'
RETURN 'L2 prime sublens solids' AS statement, collect(n.coordinate) AS nodes;

// ── 7. Relational fold-in (MERGE — idempotent, convention-conformant) ───────

// 7a. M1-5 → M2 : PROVIDES_FOUNDATION (Map M1-5 relation of record)
MATCH (a {coordinate:'M1-5'}), (b {coordinate:'M2'})
MERGE (a)-[r:PROVIDES_FOUNDATION]->(b)
ON CREATE SET r.c_0_source_coordinate = 'M1-5',
              r.c_1_relation_description = 'Recognition reopens as vibration: the 720° double-cover establishes the 36×2 = 72 structure that becomes Paraśakti\'s vibrational framework; the torus fold completes and hands itself forward (5→0).',
              r.c_2_relation_type = 'foundation',
              r.c_3_created_at = '2026-09-06',
              r.c_3_dataset_branch = 'fold-in-2026-09-06'
ON MATCH SET r.c_4_last_verified = '2026-09-06'
RETURN 'rel M1-5->M2' AS statement, count(r) AS rels;

// 7b. M3-0 → M1-5 : INHERITS_QUATERNION_FROM (Map M3-0 relation of record)
MATCH (a {coordinate:'M3-0'}), (b {coordinate:'M1-5'})
MERGE (a)-[r:INHERITS_QUATERNION_FROM]->(b)
ON CREATE SET r.c_0_source_coordinate = 'M3-0',
              r.c_1_relation_description = 'SU(2) rotational mathematics and the 720° double-covering received as the quaternionic ground of reception.',
              r.c_2_relation_type = 'inheritance',
              r.c_3_created_at = '2026-09-06',
              r.c_3_dataset_branch = 'fold-in-2026-09-06'
ON MATCH SET r.c_4_last_verified = '2026-09-06'
RETURN 'rel M3-0->M1-5' AS statement, count(r) AS rels;

// 7c. M3-0 → M2 : RECEIVES_VIBRATIONAL_MATRIX_FROM (Map M3-0 relation of record)
MATCH (a {coordinate:'M3-0'}), (b {coordinate:'M2'})
MERGE (a)-[r:RECEIVES_VIBRATIONAL_MATRIX_FROM]->(b)
ON CREATE SET r.c_0_source_coordinate = 'M3-0',
              r.c_1_relation_description = 'The complete 72-fold vibrational template enters reception; M2-4\'s templateure (V72 = ⊕4 V18) is the form-potential face of what is received.',
              r.c_2_relation_type = 'reception',
              r.c_3_created_at = '2026-09-06',
              r.c_3_dataset_branch = 'fold-in-2026-09-06'
ON MATCH SET r.c_4_last_verified = '2026-09-06'
RETURN 'rel M3-0->M2' AS statement, count(r) AS rels;

// 7d. M2-4 → M3-0 : TRANSMITS_VIBRATIONAL_KNOWLEDGE (existing type; templateure edge)
MATCH (a {coordinate:'M2-4'}), (b {coordinate:'M3-0'})
MERGE (a)-[r:TRANSMITS_VIBRATIONAL_KNOWLEDGE]->(b)
ON CREATE SET r.c_0_source_coordinate = 'M2-4',
              r.c_1_relation_description = 'Vibrational templateure → form potential → determinate rūpa: the modal storehouse hands its lawful potentials to reception, which resolves an address.',
              r.c_2_relation_ratio = '9:8 epogdoon',
              r.c_2_relation_type = 'transduction',
              r.c_3_created_at = '2026-09-06',
              r.c_3_dataset_branch = 'fold-in-2026-09-06'
ON MATCH SET r.c_4_last_verified = '2026-09-06'
RETURN 'rel M2-4->M3-0' AS statement, count(r) AS rels;

// 7e. M3-0 → M2-5 : TRANSFORMS_72_TO_64_VIA (Map M3-0 relation of record)
MATCH (a {coordinate:'M3-0'}), (b {coordinate:'M2-5'})
MERGE (a)-[r:TRANSFORMS_72_TO_64_VIA]->(b)
ON CREATE SET r.c_0_source_coordinate = 'M3-0',
              r.c_1_relation_description = 'The 72→64 transduction\'s receiving end names its gate: the 9:8 epogdoon enacted at the planetary-chakral bridge.',
              r.c_2_relation_ratio = '9:8',
              r.c_3_created_at = '2026-09-06',
              r.c_3_dataset_branch = 'fold-in-2026-09-06'
ON MATCH SET r.c_4_last_verified = '2026-09-06'
RETURN 'rel M3-0->M2-5' AS statement, count(r) AS rels;
