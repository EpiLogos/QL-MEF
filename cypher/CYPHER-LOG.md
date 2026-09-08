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

---

## 2026-09-06 — M1-4.5 musical genesis fill, round 3 (the reserved space filled from the current ql-mef canon)

- **Script**: [`2026-09-06-m1-4.5-musical-genesis.cypher`](2026-09-06-m1-4.5-musical-genesis.cypher)
- **Operator**: zcode — ql-mef musical genesis session (task N2)
- **Authority**: owner direction in session (task N2: fill the round-2 reservation from the current canon); authority order = current code + current docs over older map names. Canon surveyed: `crates/ql-mef/src/music.rs` (MUSICAL_HARMONIC_VERSION 1.0.0), `music_completion.rs`, tests `musical_harmonic_system.rs` / `musical_completion.rs` / `musical_traversal_classification.rs`, `docs/music/PRE-M-MUSICAL-DERIVATION-v1.md`, `docs/music/JANKO-QL-INSTRUMENT-FIGURE.md`, `docs/sources/ql-musical-derivation-v3.md` (blob 6414c56…), `fixtures/music/*.tsv`.
- **Pre-write reads**: M1-4.5 / M1-4 / M1-3 / M1-3-4 full property maps (lineage composition); M1-4.5 has **no children** (subtree = 1 node) — account landed as properties, no tree invented.
- **Post-write verification** (read-back, same date): all 8 new `c_2_musical_*` properties + `c_3_musical_canon_refs` present with expected sizes; `c_3_updated_at = 2026-09-06`; both corrections hold with former readings preserved under coalesce; apostrophe escaping verified in-text; `c_2_reserved_genesis` untouched; Bimba node count unchanged at 2,141 (writes were property-only).

| # | Statement (granular) | Target | Idempotency | Result |
|---|---|---|---|---|
| 1 | M1-4.5: new `c_2_musical_genesis_chain` — the reservation fulfilled: full stage-by-stage plumbing (# → source equation → both Spanda equations → (4:2)/(3:3) → 16/9 → ratio field → two bases/Z12 → A/B/C+D grammar → 12 lens anchors → 8+4 & 3×3 squares → CF cut & 7 modes → 84 landscape → Jankó 4:2 figure), with the standing note and cross-refs to M1-4/M1-3/M1-3-4 lineage properties | node M1-4.5 | SET | ✅ applied; verified |
| 2 | M1-4.5: new `c_2_musical_spanda_equations` — First Spanda 3:3 (T0‖T1 as the two co-arising faces; code-face vocabulary Direct/Conjugate, Name/Power) + Second Spanda 4:2 (100% = 2⁶+6²; 64/36 = 16/9 = (4/3)² = 2⁴/3² as ratio-face of 1/1) + cross-comparison (4:2)/(3:3) = (4/3, 2/3) + Galperin 3-to-π grounding + rhythm→pitch ~20 Hz threshold | node M1-4.5 | SET | ✅ applied; verified |
| 3 | M1-4.5: new `c_2_musical_ratio_field` — the canonical eight ratios with roles as code holds them (CANONICAL_RATIOS + harmonic-ratios-v1.tsv), three convergent derivations of 9/8 (incl. 72/64 cross-refs to M3-0/M2-5), the two load-bearing identities (16/9×9/8 = 2/1; 4/3×9/8×4/3 = 2/1) | node M1-4.5 | SET | ✅ applied; verified |
| 4 | M1-4.5: new `c_2_musical_two_bases` — chromatic (9/8, +2 st, axis 1 st) vs fifths (3/2, +7 st, axis 6 st/tritone); pitch laws (2n mod 12 / 7n mod 12); L0 mappings both bases; both unions = one Z12; (4+2)+(4′+2′) = 6+6 = 12 | node M1-4.5 | SET | ✅ applied; verified |
| 5 | M1-4.5: new `c_2_musical_pairing_grammar` — canon family law: A adjacent {(0,1),(2,3),(4,5)}, B offset {(1,2),(3,4),(5,0)}, C mirror {(0,5),(1,4),(2,3)}; (2,3) double membership A-and-C; D1→D3 as completion degrees (not a fourth family); semantic cross operators separate; basis-specific interval realisations | node M1-4.5 | SET | ✅ applied; verified |
| 6 | M1-4.5: new `c_2_musical_lens_anchors` — 12 tonic anchors derived from kernel coordinates (both bases' anchor pitch lists); FamilySamePosition address-field sharing; 8+4 explicate/implicate partition; 3×3 squares, 9 per basis / 8 unique tetrads (A-sq2 = C-sq3 midpoint self-mirror) | node M1-4.5 | SET | ✅ applied; verified |
| 7 | M1-4.5: new `c_2_musical_cf_modes_84` — canonical seven selections #0,#1,#2,#2′,#3′,#4′,#5′ (cross-ref M1-3-4 equation_as_frames); L0 reference diatonic C-D-E-F-G-A-B with Name-Name-Name-Power-Power-Power-Power; the 7 modes as CF1–CF7 groundings with authored form patterns; major/minor at degrees 3/6/7; 12×7 = 84 mode-tonic landscape | node M1-4.5 | SET | ✅ applied; verified |
| 8 | M1-4.5: new `c_2_musical_janko_figure` — the 1882 six-row surface as instrument Figure: 4 white : 2 black ↔ Second Spanda 4:2, 3:3 ↔ First Spanda; two interleaved whole-tone families ↔ the direct/prime helices; diagonal semitone ↔ conjugate axis; Figure-not-proof provenance boundary | node M1-4.5 | SET | ✅ applied; verified |
| 9 | M1-4.5: new `c_3_musical_canon_refs` (code/doc/fixture refs + authority order note) and `c_3_updated_at = '2026-09-06'` | node M1-4.5 | SET | ✅ applied; verified |
| 10 | M1-4.5: `q_1_harmonic_families_abc` **corrected to canon** (family letters B and C were swapped: B is offset-transition {(1,2),(3,4),(5,0)}, C is converse-mirror {(0,5),(1,4),(2,3)}; (2,3) double membership is A-and-C, not A-and-B); former reading preserved in new `q_1_former_reading` via coalesce (coalesce assignment ordered before the overwrite) | node M1-4.5 | SET + coalesce | ✅ applied; verified — corrected text and preserved former both read back |
| 11 | M1-4.5: `q_0_foundational_ratios_inherited` **corrected to canon** (slash-inversion reciprocals `3/4, 3/2`, previously written `3/4, 2/3`; canon: 4/3↔3/4, 2/3↔3/2); former reading preserved in new `q_0_former_reading` via coalesce | node M1-4.5 | SET + coalesce | ✅ applied; verified |

**Scope note**: only node M1-4.5 was written. Two further canon-vs-map deltas were found on OTHER nodes and are recorded as proposals, not executed: (a) M1-3 `c_2_double_cover_reading` says "T1/T2" for the two tracks — the authored sources (v3 §II-5.1, PRE-M contract) name them **T0/T1**; (b) the vendored v3 source still names the L2′ sixth element "Mineral" where the graph (round-1 erratum) and registry now hold "Salt" — doc-lags-graph, vendored blob not editable. The node's older `c_1_complete_formulation` ("4²→16-Fold→64(Mahamaya)") grounds 64 via the 16-fold reading where canon grounds it in 2⁶ (the doubling, as M1-4's `c_2_exponent_genesis` already holds) — tension noted, not rewritten.

**Incident note (post-verification)**: after the round-3 read-back verification completed successfully, the Docker daemon wedged (same failure class as round 2: `docker ps` hangs, bolt port closed, socket serving the "Docker Desktop is unable to start" stub). All round-3 writes were already verified before the stall. Recovery: GUI quit left the privileged `com.docker.backend` (pid 74701) holding the stub socket; it ignored SIGTERM and was SIGKILLed, Docker Desktop relaunched clean — engine up ~160 s later, `epi-neo4j` restarted (it had exited 255), and the graph re-verified post-recovery: 2,141 Bimba nodes intact, all 11 round-3 properties present, both corrections holding, `c_2_toroidal_element` confirmed absent on M1-3-0 (the N3 scoping proposal was executed nowhere). A parallel scoping session had already reported the wedge onset during its read-only pass.
