# Bimba Cypher Log — granular record of database updates

Every Cypher statement applied to the Bimba graph is logged here, dated and granular (one row per statement), with its idempotency mechanism and observed result. Scripts live alongside as dated `.cypher` files; the log is the record of what actually ran and what it did. Read `skills/bimba-cypher/SKILL.md` before writing new entries.

Connection of record: `docker exec -i epi-neo4j cypher-shell -a bolt://localhost:7687 < <script>` (auth disabled; file-based input avoids shell/Cypher quote mangling — Cypher escapes apostrophes with **backslash**, not SQL doubling).

---

## 2026-09-06 — Fold-in across the seats (M1-5 / M2-0 / M3-0 / M2-4 / L2′ family)

- **Script**: [`2026-09-06-fold-in-seats.cypher`](2026-09-06-fold-in-seats.cypher) (full idempotent re-run after an escaping fix; statements 1–3 applied once, re-applied identically on re-run — idempotency demonstrated)
- **Operator**: zcode — ql-mef geometric foundation session
- **Authority**: owner acceptance in session (M2-4 rename with refined naming); register erratum (Mineral→Salt); Map relations of record; integrated physical-pole object; [`docs/geometry/FOLD-AND-RULING-GRAMMAR.md`](../docs/geometry/FOLD-AND-RULING-GRAMMAR.md) §17.4

| # | Statement (granular) | Target | Idempotency | Result |
|---|---|---|---|---|
| 1 | M2-4 rename: `c_1_name` → `Vibrational Templateure`; `c_1_former_name` (coalesce) ← `Parashakti: Vibrational Arena of Archetypal Powers`; `c_1_primary_designation` → `Vibrational Templateure — the modal form-potential storehouse of Paraśakti`; `c_1_appellations` ← [Arena of Archetypal Powers, Cosmic Resonance Chamber…]; new `c_2_templateure` (V72 = ⊕4V18; T = I4⊗T18→16; M2-C25) | node M2-4 | SET + coalesce | ✅ applied; read-back verified |
| 2 | M1-5: new `c_5_ruling_surface` (P×L as the two rulings of the doubly-ruled quadric; torus as projective closure; Villarceau/Dupin; Clifford/S³-Hopf; K² twist) and `c_2_torus_measures` (R=16/9, r=1 → 25:7 equators; sin θ = 9/16 = (3/4)²) | node M1-5 | SET | ✅ applied |
| 3 | M2-0: new `c_2_pentad_four_faces` (hypotenuse 3²+4²=5² / partition 100/5=20° / ratio φ / container-120-in-S³) and `c_2_curvature_seat` (the open — hyperbolic; between flat hinge and spherical closure; Gauss–Bonnet spine) | node M2-0 | SET | ✅ applied |
| 4 | M3-0: new `c_2_two_descriptions_law` (8 collisions vs 9 non-closing fold-points; the aliasing law) and `c_2_epogdoon_two_way_door` (descent 8/9 / ascent 9/8; T₀/T₁; (64/36)×(9/8)=2/1) | node M3-0 | SET | ✅ applied |
| 5 | L2-5′ erratum propagation: `c_1_name` `Mineral` → `Salt`; `c_1_former_name` (coalesce) ← `Mineral` | node L2-5′ | SET + coalesce | ✅ applied; read-back verified |
| 6 | L2′ parent: new `c_2_solids_register` (element-bearing lens as Timaeus completed; Offered-grade, movement-33 gate). Sublens family L2-0′…L2-5′: new `c_2_platonic_solid` per node (dodecahedron-withheld / cube / icosahedron / octahedron / tetrahedron / Salt-no-solid) | nodes L2′, L2-0′…L2-5′ | SET (CASE) | ✅ applied (7 nodes) |
| 7a | MERGE (M1-5)-[:PROVIDES_FOUNDATION]->(M2) | relation | MERGE + ON CREATE/ON MATCH | ✅ **pre-existed** (2026-07-28, paramasiva-deep); stamped `c_4_last_verified = 2026-09-06` |
| 7b | MERGE (M3-0)-[:INHERITS_QUATERNION_FROM]->(M1-5) | relation | MERGE | ✅ pre-existed (mahamaya-deep); verified-stamped |
| 7c | MERGE (M3-0)-[:RECEIVES_VIBRATIONAL_MATRIX_FROM]->(M2) | relation | MERGE | ✅ pre-existed (mahamaya-deep); verified-stamped |
| 7d | MERGE (M2-4)-[:TRANSMITS_VIBRATIONAL_KNOWLEDGE {9:8, templateure→form-potential}]->(M3-0) | relation | MERGE | ✅ **created** (branch `fold-in-2026-09-06`) — the genuinely new edge |
| 7e | MERGE (M3-0)-[:TRANSFORMS_72_TO_64_VIA {ratio 9:8}]->(M2-5) | relation | MERGE | ✅ pre-existed (mahamaya-deep); verified-stamped |

**Incident note**: first execution failed at statement 4 on Cypher apostrophe escaping (`''` SQL-doubling is invalid; Cypher requires `\'`). Statements 1–3 had applied; the script was fixed in place and re-run in full — statements 1–3 re-applied identically (idempotency demonstrated), 4–7e then completed. Lesson recorded in the skill.

**Post-write verification** (read-back, same date): M2-4 name/former/appellations/templateure confirmed; L2-5′ = Salt with former_name Mineral; five target relations present — 4 pre-existing with `c_4_last_verified`, 1 new with fold-in provenance; also observed pre-existing `REFLECTS_FOUNDATION` (M3-0→M2-5, the 5/0 non-dual link).

**Erratum propagation status after this entry**: graph ✅, Rust registry/fixtures ✅ — remaining stragglers: `Control/user/identity/sources/natal-chart.md` (Mineral/lapis prose), `epi …/S4-1p-hen/CONTRACT.md` (sub-position list). Not touched here (human/contract ground — propose separately).

---

## 2026-09-06 — M1-branch fold-in, round 2 (exponent genesis; double cover; equation-as-frames; M1-4.5 reservation)

- **Script**: [`2026-09-06-fold-in-m1-branch.cypher`](2026-09-06-fold-in-m1-branch.cypher)
- **Operator**: zcode — ql-mef geometric foundation session, later pass (after consulting the M1 branch at depth: M1-0/M1-1/M1-3/M1-3-4/M1-4 from the wayfinder branch's Map files + the kernel contract)
- **Authority**: owner direction in session (exponent genesis → existing matrix + map, no new docs; the 0/1 re-grounding; the [4.5] reservation)
- **Infrastructure note**: executed after a Docker daemon recovery — the earlier session's `docker restart` was killed mid-cycle leaving the container stopped and the daemon API wedged; resolved by full Docker Desktop restart (quit + relaunch). Data intact (2,141 Bimba nodes verified post-recovery).

| # | Statement (granular) | Target | Idempotency | Result |
|---|---|---|---|---|
| 1 | M1-4: new `c_2_exponent_genesis` — the 1→12 compositional line (2⁴/3² = six factors = the position hexad; ±(6+6) → the 12-ring at 12:6 = 2:1; 12×6 = 72; 2⁶ = 64; 3+4 = 7 and 3×4 = 12), with the standing note (coordinates = 1, derivation = 0, retroactive grounding) | node M1-4 | SET | ✅ applied; verified |
| 2 | M1-3: new `c_2_double_cover_reading` — the equation's two operations as the two circuits (division = first traversal/alias; summation = second/recognition; T1/T2 = the two helices; pratyabhijñā = the return-switch; kernel seats n↔n′ and 720°) | node M1-3 | SET | ✅ applied; verified |
| 3 | M1-3-4: new `c_2_equation_as_frames` — the complete formulation and the Context-Frame sub-stages as one sequence (0000↔0/0 CF1 … (5/0)↔1/1 CF7) | node M1-3-4 | SET | ✅ applied; verified |
| 4 | M1-4.5 ("Harmonic Meta-Frames & Quintessential Integration"): new `c_2_reserved_genesis` — the [4.5] space reserved for the full harmonic/musical genesis (#31 edge: musical derivation v3, pre-M derivation, Jankó), empty by design, refinements streaming | node M1-4.5 | SET | ✅ applied; verified |
