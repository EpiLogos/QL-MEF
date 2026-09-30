# Gate 0 — M1 body (K²) structural map

Scope: QL-MEF #135 / O:I #335, M1 section. Read-only. Standing: agent-derived map; graph and owner win where they differ.

## Sources and pins

| Key | What | Pin / check |
|---|---|---|
| **PD** | `C-Experiments@daa660c:Idea/Bimba/Map/datasets/paramasiva-deep/nodes-full-detail.json` | sha256 `9d895710…` = `m-tree-v1.json.files[]` entry (verified) |
| **PR** | `…/paramasiva-deep/relations.json` | `bc739b4e…` (verified) |
| **LD** | `…/low-detail/nodes_paramasiva.json` | `a03ff991…` (verified) |
| **CSV** | `…/datasets/(0_1) Vortex Modulae … Sheet1.csv` | `3e3d0684…` (verified) |
| **TREE** | QL `fixtures/kernel/m-tree-v1.json` | `source_revision daa660c…`, `registry_revision 259a2f49…` |
| **QL** | `QL-MEF` | `main @ 4669d8d` |
| **WT** | `QL-MEF #251 branch `feat/m123-engine-binding`` (superseded K²) | `@ 046c23b`, merge-base `56921a5` |
| **CX** | Epi-Logos-C-Experiments, local clone | read at the pin `daa660c` via `git show`. HEAD is `b57ddda`; `daa660c` is an ancestor. |

Standing labels used below:

- **SOURCE**: a Bimba graph property, a graph relation or the CSV.
- **SEED**: a CX@daa660c `Idea/Bimba/Seeds/**` or `portal-core` file. These are authored, but not compiled into TREE.
- **C-RET**: a retained C/vendor constant.
- **DERIVED**: shown with its derivation.
- **UNDETERMINED**: given as alternatives.

Relations: TREE carries each `#1-5*` edge twice (low-detail + deep files). That gives 116 records and **58 unique** edges.

---

## 1. Body dataflow table

### 1a. `#1-5` subtree nodes (all seven)

| Element | Coord refs | Relations (unique, TREE) | Key property = literal (PD) | C / Rust / C++ owner | Standing |
|---|---|---|---|---|---|
| Toroidal Recognition | `#1-5` | `#1 HAS_INTERNAL_COMPONENT`; `#1-4 DEVELOPS_INTO`; `EMERGES_FROM #1-4`; `#1-3-5 ENABLES_RECURSIVE_INTEGRATION`; `RETURNS_TO #1-0`; `PROVIDES_FOUNDATION #2`; `PROVIDES_SPINOR_FOUNDATION #3` (PR relProps: "SU(2) quaternionic rotations … 720° double-covering"); `#3-0 INHERITS_QUATERNION_FROM` (mahamaya-deep relProps `rotationRequirement: 720`); `#2-1-5 ROTATIONAL_RESONANCE`; `ESTABLISHES_FRACTAL_RECURSION #5`; `INTEGRATES_CONCEPT`/`HAS_INTERNAL_COMPONENT` → `-0…-5` | `.symbol = "T² with quaternionic rotation operators Q: Torus as S¹×S¹ parametrized by exponential structure"`. `.winding = "Positions spiral through 4π total: theta₁ in [0,2π] for meridian (Positions 1-2), theta₂ in [0,2π] for longitude (Positions 3-4), synthesis at Position 5 returns as (2π,2π)~(0,0)"`. `.doubleCoveringPrinciple = "720° rotation creates 36×2=72 structure for complete cycles"`. `.topologicalSignature = "Genus g=1, Euler characteristic χ=0, … 4g+2g=6 formula, Flat metric capability"`. `.harmonicSynthesis = "Unification of spanda rhythm and ananda harmony in quaternionic space"` | C `c/src/m1.c:123 ql_m1_topology` (seat `#1-5`); Rust `crates/ql-mef/src/m1.rs:224 topology`, `m1_engine.rs:191 carrier` | SOURCE (props). The topology tables are C-RET. |
| Toroidal-Quaternionic Ground | `#1-5-0` | `INTEGRATES/HAS_INTERNAL ← #1-5`; `LEADS_TO #1-5-1`; `#3 EMPLOYS_SEQUENCE` (Fibonacci) | `.architecturalFunction = "…quaternions describe two-circle products via e^(iθ₁)e^(jθ₂)"`. `.operationalSymbolics = "…Ground state q₀=(1,0,0,0) represents origin at (θ₁=0, θ₂=0)…"`. `.keyPrinciples[…] = "Generator Seed: i as meridian generator (θ₁ parameter), j as longitude generator (θ₂ parameter)"`. `.keyPrinciples[…] = "Hopf Fibration Structure: S³→S² with S¹ fiber bundle…"` | C `c/src/m1_state.c:46 ql_m1_rotor` computes `(cos a+i sin a)(cos b+j sin b)` via `ql_quat_multiply`. Rust `m1_engine.rs:158 rotor`. | SOURCE; the rotor matches the literal exactly |
| Torus in Quaternionic Space | `#1-5-1` | `#1-2 (Ananda) MANIFESTS_GEOMETRY` (PR relProps `relationship:"Toroidal Harmonic Structure"`); `#0-1 MANIFESTS_BOUNDARY_DYNAMICS`; `EMPLOYS_GEOMETRY #0-1`; `ENABLES_HYPERBOLIC_EXPANSION #1-2`; `SCALES_TO #5`; `LEADS_TO #1-5-2` | `.keyPrinciples[…] = "Standard Embedding Formulas: x=(R+r cos θ₁)cos θ₂, y=(R+r cos θ₁)sin θ₂, z=r sin θ₁"`. `.keyPrinciples[…] = "Aspect Ratio R/r: Determines torus shape from thin to fat to self-intersecting"` (no numeric R/r in PD, LD or `paramsiva-1-5-branch-updates.cypher:89`). `.coreNature = "…e^(iθ₁) (meridian—minor circle of radius r), second by e^(jθ₂) (longitude—major circle of radius R+r)…"`. `.practicalApplications[…] = "Texture Mapping: Applying 2D textures to 3D torus using (θ₁, θ₂) as UV coordinates"` | C `c/src/m1.c:141 ql_m1_torus` (`radius = 16.0/9.0 + cos(theta1)`, z = sin θ₁). Rust `m1.rs:244 torus`. Header `c/include/ql/m1.h:50` says "fixed R=16/9, r=1". | Embedding: SOURCE. R/r: DERIVED (§1b). |
| 4π = 720° Cycle | `#1-5-2` | `#0-1 MANIFESTS_AS`; `#1-3 MANIFESTS_GEOMETRY` ("Möbius Twisting Dynamics"); `#2-1 (MEF) DOUBLE_COVERING_MANIFESTATION` ("MEF 72-fold structure manifests double-covering principle"); `#2-2 EXPRESSES` ("36x2=72"); `#2 EMPLOYS_RATIO`; `#4 EMBODIES_DISTRIBUTION`/`MANIFESTS_GAUSSIAN_STRUCTURE`; `MIRRORS_PRINCIPLE #1-2`; `MIRRORS_PHYSICS #1-5-5`; `ENSURES_EPISTEMIC_EXCLUSION #1-3`; `LEADS_TO #1-5-3` | `.operationalSymbolics = "Total angular measure: ∫∫_T² dθ₁dθ₂ = (∫₀^(2π) dθ₁)(∫₀^(2π) dθ₂) = 4π. … SU(2) rotation: R_2π(ψ) = -ψ, R_4π(ψ) = +ψ…"`. `.keyPrinciples[…] = "Spinor Mathematics: ψ(θ)→ψ(θ+2π)=-ψ(θ), ψ(θ+4π)=+ψ(θ)"`. `.connectionTo_1_4 = "QL positions map directly: Pos 1-2 on meridian (0→2π), Pos 3-4 on longitude (0→2π), Pos 5 at (2π,2π)~(0,0)"`. `.practicalApplications[…] = "Rotation Interpolation (SLERP)…"` | C `m1_state.c:56 ql_m1_carrier`: `spinor = rotor(degree720·π/180 / 2, 0)`. Rust `m1.rs:83 Clock::spinor`, `m1_engine.rs:191 carrier`. | SOURCE (law). The per-tick parametrisation is C-RET (§2). |
| Shadow as Phase-Shift | `#1-5-3` | `LEADS_TO #1-5-4`; `ENABLES_SHADOW_INTEGRATION #4` | `.operationalSymbolics = "…Shadow = e^(i(θ+π)) = e^(iθ)·e^(iπ) = -e^(iθ)…"`. `.keyPrinciples[…] = "First Negation Point: θ₁=π between QL Positions 1 and 2"`, `"Second Negation Point: θ₂=π between QL Positions 3 and 4"` | C `m1_state.c:66-67` computes `quadrature=(cos,sin)(degree720)` and `opposite_quadrature = −quadrature`. Rust `m1_engine.rs:194-206`. | SOURCE; the opposite quadrature equals `−e^{iθ}` exactly |
| Torus Generating Parashakti | `#1-5-4` | `BRIDGES_TO #2-0`; `COMPLETES_CYCLE #1-0`; `LEADS_TO #1-5-5` | `.operationalSymbolics = "36 times 2 = 72 formula … Rotational to Vibrational: e^(iθ) becomes e^(i(kx-ωt))…"`. `.bridgeFunction = "Seven (7) serves as key modulator between 6-based … and 8-based … structures"` | `ql_m1_carrier.m2_carrier_count = QL_RESONANCE_COUNT (72)` (`m1_state.c:68`). Rust `crate::field_cardinality()`. | SOURCE |
| Recognition of Toroidal Necessity | `#1-5-5` | `COMPLETES #1-5`; `RECOGNIZES_SOURCE`/`EMBODIES_EULER_IDENTITY #0-0`; `UNIFIES_DOMAINS`/`UNIFIES_PSYCHOID_REALITY #4`; `ENABLES_PSYCHOID_COMPUTING #5`; `#5 EMBODIES_IDENTITY`; `#1-2 MANIFESTS_EULER_NUMBER` | `.operationalSymbolics = "…Formula: 4g+2g with g=1 yields 6. Quotient: (2π,2π) equivalent to (0,0) in T²=R²/Z²…"` | `ql_m1_carrier`: `genus=1`, `explicate_edges=4g`, `identification_slots=2g` (`m1_state.c:68-69`); `ql_m1_grammar` (`m1_state.c:43`) | SOURCE |

### 1b. Body quantities

| Element | Coordinate refs | Relations | Property = literal | Owner (file:symbol:line) | Standing / derivation |
|---|---|---|---|---|---|
| Standard embedding | `#1-5-1` | `#1-5-0 LEADS_TO #1-5-1` | `.operationalSymbolics = "Standard embedding: P(θ₁,θ₂) = ((R+r cos θ₁)cos θ₂, (R+r cos θ₁)sin θ₂, r sin θ₁)… Quaternionic form: position = R·e^(jθ₂) + r·e^(iθ₁)·e^(jθ₂)."` | `c/src/m1.c:141 ql_m1_torus`; `crates/ql-mef/src/m1.rs:244 torus` | **SOURCE**. θ₁ = meridian (tube, radius r); θ₂ = longitude (major). |
| **R/r = 16/9** | `#1-3-5 → #1-4.0 → #1-4 → #1-5 → #1-5-1` | `#1-3-5 ENABLES_RECURSIVE_INTEGRATION #1-5`; `#1-4 DEVELOPS_INTO #1-5`; `#1-5 HAS_INTERNAL_COMPONENT #1-5-1` | See the derivation list after this table. | C `m1.c:147`, vendor `vendor/epi-kernel/reference/include/m1.h:770-773 TORUS_R_MAJOR_NUM 16u / TORUS_R_MINOR_DEN 9u / TORUS_R_MAJOR_F (16.0f/9.0f) / TORUS_R_MINOR_F 1.0f`; Rust `m1.rs:248` | **DERIVED**. `#1-5-1` has no literal aspect value. |
| **Portal R+r = 1** | same chain; 100% = 1 | — | `LD #1-4 .coreNature = "…100% = 64+36 = 16/9 = 4²/3²…"` | CX@daa660c `Body/S/S0/portal-core/src/state.rs:13` `(r, big_r) = (0.36f32, 0.64f32)` in `compute_orbital_position`. The law text is CX@**b57ddda** `portal-core/tests/k2_geometry_reference.rs:15-17,117-129` ("R/r = 16/9 with R + r = 1 (R = 0.64 Mahāmāyā 2⁶, r = 0.36 Paraśakti 6²; outer equator = the unit 1/1)"; "DERIVATION-RESOLVED 2026-07-06, Architect-directed"). That file is **not at the pin**: `0b77087` is not an ancestor of `daa660c`. | **DERIVED**. 1 = 0.64 + 0.36 splits 100% into 64 + 36, so R = 16/25 and r = 9/25. QL C uses r = 1, so R + r = 25/9 (`WT k2.rs` influence range "|x|,|y| ≤ 25/9"). The proportion agrees; the normalisation differs (§5 D-M1-02). |
| `#1-5-2` 4π double cover | `#1-5-2`, `#1-5-0` | `#2-1 DOUBLE_COVERING_MANIFESTATION`, `#3-0 INHERITS_QUATERNION_FROM #1-5 {rotationRequirement:720}` | `R_2π(ψ) = −ψ, R_4π(ψ)=+ψ` (above) | `QL_M1_Clock.degree720 = tick·30 + 360·(cycle&1)`, `hopf_fiber = cycle&1` (`c/src/m1.c:30-32`; `m1.rs:61-80`). Vendor `hopf_fiber(d720) = d720≥360` (`vendor/.../m1.h:671`). Native `QL_PhaseLift{turns, half_degrees}` + `ql_phase_double_cover_half_degrees` (`c/src/coupled_clock.c:19`). | SOURCE law; C-RET step size |
| `#1-5-3` shadow as phase shift | `#1-5-3` | `#1-5-2 LEADS_TO #1-5-3` | `Shadow = −e^(iθ)` | `opposite_quadrature` (`m1_state.c:67`) | SOURCE. The shadow is also the second Hopf sheet: `R_2π ψ = −ψ` gives the same sign. The identity "shadow sheet = −primary sheet" is DERIVED (§1c). |
| Longitude = chromatic circle | `#1-5-1` θ₂; `#1-4` (lens), `#2-1` | — (no graph relation names chromatic/fifths; zero "fifths" matches anywhere in the pinned datasets extract) | SEED `M1'/M1'-SPEC.md:190-192` "S¹ chromatic-longitude = the 12-tone octave-circle traversed by epogdoon-stacking (9/8 generator) … two longitudes (bimba helix + pratibimba helix offset by semitone) = chromatic 12" | Rust `crates/ql-mef/src/music.rs:152 MusicalBasis::pitch_at`: Chromatic `(2n) % 12` + face·1. C `c/src/m2.c:157 m2_pitch(t) = t<6 ? 2t : 2(t−6)+1` (the same chromatic map, used by Vimarśā). | **SEED**; DERIVED 12-lattice (below) |
| Meridian = circle of fifths / tick circle | `#1-5-1` θ₁ | — | SEED `M1'-SPEC.md:194-196` "S¹ fifths-meridian = the circle-of-fifths traversed by 3/2 generator-leap … gcd(7, 12) = 1 forces the meridian to visit all 12 pitch-classes". SEED `M1-2-ANANDA-VORTEX-ARCHITECTURE.md` §5.2 "Texture V = fifths-meridian θ, sampled from tick12…". CX@daa660c `state.rs:11-12`: `theta = degree·τ/360` (longitude), `phi = tick12·τ/12` (**meridian = tick circle**) | `music.rs:152` Fifths `(7n)%12` + face·6 | **SEED** |
| → 12-fold lattice | tick12 ≡ (position6, face) | `#1-5-2 .connectionTo_1_4` | — | `QL_M1_Clock.position6 = tick%6, phase = tick/6` (`m1.c:26-27`) | **DERIVED**. See the derivation after this table. |
| Ananda 12×12 field as torus texture | `#1-2`, `#1-2-0…-5` (+`-0` DR) | **`#1-2 MANIFESTS_GEOMETRY #1-5-1`** (relProps `mathematical_basis:"Torus geometry revolving around central void while maintaining coherent harmonic structure"`) | CSV lines 5-17 (rows `0X…11X`; line 15 blank) × `Position 0…11`. For example line 12 col C = `"1.777r = 16/9 - sum = 25 (x4 - 64/36 - sum = 100)"`. `#1-5-1 .practicalApplications "Texture Mapping … (θ₁, θ₂) as UV"` | C `c/src/m1_state.c:11 ql_m1_source_cell` over `c/src/m1_source_data.inc:2 source_cells[864]` (6×144; generated by `scripts/generate-m1-source.py`). Typed arithmetic `c/src/m1.c:37 ql_m1_cell`. Vendor DR `ANANDA_BIMBA/PRATIBIMBA/SUM/QUINTESSENCE` (`vendor/.../src/m1.c:22,43,64,85`). Rust `m1_engine.rs:63 source_cell`, `m1.rs:114 cell`. | Field: SOURCE. **Placement: UNDETERMINED** (§1c T1). |
| K² (what it is here) | `#1-5` (single torus), `#3-5` (product) | `#1-5-2`, `#0-1 MANIFESTS_AS #1-5-2` | SEED `M1'-SPEC.md` §10: "K² = T² with bimba ↔ pratibimba conjugation as non-orientable identification" (the **Klein bottle** over the chromatic-fifths torus). SEED `alpha_quaternionic_integration_across_M_stack.md:221` "M1-5 is *single* toroidal recognition … double-torus … K² × T²_Mahāmāya … lives at **M3-5**". QL `docs/origami work/M3/M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md:913-924` "K²: M1/M2 harmonic-audio-genesis double-cover" | No QL owner for the Klein identification. The face flip is `QL_M1_Clock.phase/conjugate_phase`. | SEED |
| K² orientation (slerp) | `#1-5-0`, `#1-5-2` | — | SEED `M1-2-ANANDA-VORTEX-ARCHITECTURE.md` §6.1: "`K².orientation = quat_slerp(q_from, q_to, dt / TICK_PERIOD)` … through `RING_QUATERNION_LUT[12]`". `#1-5-2 .practicalApplications "Rotation Interpolation (SLERP)"` | `quat_slerp` exists **only** in `vendor/.../include/m1.h:493`. There is **none** in `c/`, `crates/`, `cpp/` (grep). LUT: vendor `m1.h:551`; `crates/ql-core/src/pole/quaternion.rs:93`; exposed `m1.rs:224 topology().legacy_ring_quaternion`. Generated orbit: `m1.rs:83 Clock::spinor`, `m1_state.c:56`. C++ applies only a **Z-rotation** by the inscription/lensing half-degree (`cpp/include/ql/continuous_field.hpp:197-207`), not SU(2). | Law: SEED. **Orbit: UNDETERMINED** (§1c T2). |
| Hopf double-cover sheets | `#1-5-0` (Hopf), `#1-5-2`, `#1-5-3` | — | `#1-5-0 .keyPrinciples "Hopf Fibration Structure: S³→S² with S¹ fiber bundle"`. `#1-5-1 .resonances "Villarceau Circles…", "Hopf Fibration Projection…"`. SEED vortex §5.5: "Render the second 360° as a phase-shifted second torus … concentric with the first". `vendor m1.h:607` "Hopf bundle: S³ (tick12, 720°) → S² (QL, 360°) → S¹ (phase, binary)" | `hopf_fiber` (`m1.c:31`); vendor `hopf_project/hopf_fiber/hopf_tick12` (`m1.h:664-677`). No sheet-carrying field exists in Rust or C++. | SOURCE/SEED. Anti-periodic carriage is DERIVED (§1c T3). |
| Vimarśā audio bus | `#2-1` MEF; M2-1 reading | `#2-1 DOUBLE_COVERING_MANIFESTATION #1-5-2` | SEED `INTEGRATED-1-2-3-COSMIC-ENGINE-ARCHITECTURE.md:153,778` "Vimarsha audio bus writes (`audio_octet`, `nodal_quartet`) — `portal-core/src/parashakti/vimarsha_reading.rs:17-93`" | C `c/src/m2.c:158 ql_m2_vimarsha`. Rust `crates/ql-mef/src/m2_vimarsha.rs` (`POLICY ql.m2-vimarsha/retained-portal-reading-v1`, `SOURCE …@daa660c:…/vimarsha_reading.rs`). M1 enters via `tick12` (`m2_pitch(t)/12` + `t<6?0:12` octave lift, `m2.c:169-170`), `lens`, `harmonic_ratio` (M1 source row / `ratio_basis`), and the CF→mode. K8 binds modes to `audio_octet_hz` by `frequency_bindings`. | SEED + implemented; the Vimarśā formula body is M2's section |
| Tick rate as declared band | `#1-3` (Spanda), `#1-5-2` | — | SEED `M1'/m1-prime-paramasiva-instrument.md:81-84`: "*Pitch* … audible band (≈ 20–20,000 Hz) / *Rhythm* … somatic band (≈ 0.5–10 Hz) / *Breath/circadian* … / *Planetary/lifecycle* …". SEED `physical-pole-stack-architecture.md:37-40`: "default user-facing tick-rate of 12 ticks per second … interactive (12 Hz default) … meditative … computational". SEED `M4'/m4-prime-psychoid-cymatic-field-engine.md:621` "1 Hz from world_clock" | **No rate in any QL owner.** `docs/KERNEL-M1-ENGINE-CONTRACT.md:230,305-307` says "K8 relates this logical time to its integration/audio/render clock explicitly". `M1Engine::advance` (`m1_engine.rs:432`) is explicit-only. | SEED. **Value: UNDETERMINED** (§1c T4). |
| Per-coordinate topology mode | all families | — | — | `c/src/holographic.c:30 ql_default_topology`: P4→LEMNISCATE, C0/C5→ZERO_SPHERE, else TORUS. Ledger binds it to `#1-5-4` only via "bimba-name-anchor (topology)". | C-RET. Not the body geometry. The name-anchor binding is a census artefact (§5 D-M1-09). |

Derivation of R/r = 16/9 (row "R/r = 16/9" above):

1. `LD #1-4 .formulation = "100% → 64/36 → 16/9 → 4²/3² → …"` (LD line 267).
2. `PD #1-4.0 .torusGenerationBegins = "The actual mathematical principles for torus generation BEGIN HERE. The 16/9 ratio seeds the 4-fold explicate (sides) and the implicate processual dynamics…"`.
3. `PD #1-3-5 .bridgeToFormalization = "…100% achieved here becomes the raw material for #1-4.0 to transform into the 16/9 ratio (100% = 64+36 = 16/9 = 4²/3²)…"`.
4. `CSV line 12 col C = "1.777r = 16/9 - sum = 25 (x4 - 64/36 - sum = 100)"`. QL cites this row as `m1.rs:313 source_ratio(1,7) = [16,9]`.
5. Therefore the torus generated through `#1-4 DEVELOPS_INTO #1-5` carries the 16:9 as its aspect: R:r = 64:36.

**Contradiction at the pin:** SEED `M1-2-ANANDA-VORTEX-ARCHITECTURE.md` §5.1 and `INTEGRATED-1-2-3…md` §5.2 say "Aspect `R/r = 9/8`". The origin is `physical-pole-stack-architecture.md:110`, where 9/8 appears as an illustrative aside. The post-pin CX `k2_geometry_reference.rs:11-22` rules 9/8 out as the aspect: it is the step ratio, with `2r/R = 72/64 = 9/8`.

Derivation of the 12-fold lattice (row "→ 12-fold lattice" above):

- Chromatic: direct face = {0,2,4,6,8,10}; conjugate face = +1 → {1,3,5,7,9,11}. Their union is Z₁₂.
- Fifths: direct = {0,7,2,9,4,11}; conjugate = +6 → {6,1,8,3,10,5}. Their union is Z₁₂.
- So tick12 ↦ one pitch class on each circle, a bijection in both cases.
- The lattice step is 30° in both circles (360/12). It coincides with M3 lens 9 (30°×12, M3 matrix §11 line 830) and with `#1-4.3 .symbolicContribution "…12-fold angular logic foundation…"`.

### 1c. Undetermined body choices (alternatives with consequences)

**T1 — Ananda 12×12 placement on (θ₁, θ₂).**
- **A:** columns (Position 0–11) → longitude/chromatic, rows (`nX`) → meridian/tick.
  - Consequence: one CSV row per tick. The active row follows M1 time and the column follows the lens/pitch.
- **B:** SEED vortex §5.2: `U = position6 → column`, `V = tick12/6·6 + lens-anchor row → row`.
  - Consequence: only 6 of 12 columns are addressable from the clock. The lens drives rows, and it conflicts with §6.2 of the same seed.
- **C:** SEED vortex §6.2: active cell `(t, t%6)`.
  - Consequence: the diagonal walk ignores the engine's `row12/col12`.
- **D:** keep the current engine: `(family,row12,col12)` is supplied config (`m1_engine.rs:353`) and the texture is static.
  - Consequence: the tick does not move the cell.
- All options need a sample count divisible by 12 on each axis for cell edges to be exact.

**T2 — Orientation orbit.** The computed table follows the list.
- **A (generated carrier, `m1.rs:83`):** half-angle = degree720/2, so SO(3) turns monotonically by 30°/tick, 360° per 12 ticks, and the SU(2) sign returns after 24 ticks.
  - Consequence: consistent with `degree360=tick·30`.
- **B (legacy LUT, vendor `m1.h:551`):** SO(3) angle by tick 0…11 is 0,60,120,180,240,300,**300**,240,180,120,60,0. `q6 = −q5` and `q11 = −q0`, and the slerp arc at 5→6 and 11→0 is 0°.
  - Consequence: the body tumbles out and retraces with two stalls. Slerp through the LUT does **not** give the seed's "one full 360° per 12 ticks" (§6.1) nor "30° per tick" (§6.2).
- **C (monotone half-angle 30°/tick):** 60°/tick, SO(3) 720° per 12 ticks, SU(2) −1 at tick 6.
  - Consequence: it realises "12 = SU(2) double cover" (vendor `RING_SIZE`, `m1.h:303`). It contradicts both the LUT (which puts −1 at tick 11) and `DEGREE_PER_TICK 30` (`m1.h:537`).
- **Axis:** all three rotate about the i-axis. Vendor `m1.h:447` "x: i component … meridian rotation axis" makes that a tumble ⟂ to the symmetry axis. C++'s Z-rotation cannot show it on a surface of revolution.

**T3 — Where anti-periodic content lives.**
- **Fact** (from `#1-5-2 .keyPrinciples "ψ(θ+2π)=−ψ(θ)"` plus Fourier): a field single-valued on one T² sheet admits only `e^{ikφ}, k∈ℤ`. Half-integer `e^{i(k+½)φ}` needs φ ∈ [0,4π), i.e. two sheets with sheet₁ = −sheet₀. That equals `#1-5-3` "Shadow = −e^(iθ)".
- **A:** the second sheet is the Hopf fibre along longitude (sheet index = `hopf_fiber` = `degree720≥360`).
- **B:** the second sheet is the Klein/face identification (sheet index = `phase = tick/6`, SEED `M1'-SPEC` §10 K²).
- **C:** both (4 sheets).
- **Consequence:** the current code already carries two different binary sheet indices. Under the degree/30 law (§3) `phase` ≠ `hopf_fiber`. Under the 60°/tick law they coincide, and A = B.

**T4 — Tick band.**
- **A:** 12 ticks/s (SEED physical-pole §1).
  - Consequence: 1 kernel cycle/s. Note that 12 Hz lies **above** the seed's own somatic band (0.5–10 Hz) and below audible.
- **B:** 1 Hz world clock (SEED M4′).
  - Consequence: 12 s per cycle; 24 s for the SU(2) return under generated carrier A.
- **C:** audible band (tick = pitch).
  - Consequence: the M1 cycle becomes an oscillator frequency. The ticks are then not visual events.
- **D:** declared per context (interactive / meditative / computational) with band-crossing events (SEED `m1-prime-paramasiva-instrument.md:244,364`).
  - Consequence: a band is a declared parameter with a receipt, not a constant.

---

## 2. What the M1 C/Rust engine computes today

**Owner:** `M1Engine` (`crates/ql-mef/src/m1_engine.rs:378`), contract `ql.m1.engine/v1`. C mirror in `c/src/m1.c`, `c/src/m1_state.c`. Acceptance `docs/kernel-rebuild/m1-engine-acceptance-v1.json`, checked by `scripts/check-m1-acceptance.py`. That script checks locks, rows and hashes, not behaviour. Tolerance is 3e-7 (f32) and 2e-14 (f64); experiential standing is "unassessed".

**Inputs** (`EngineConfig`, `m1_engine.rs:353`): `event_ref, subject_coordinate, selected_coordinate, revision (u64 str), cycle (u64 str), tick12 0..11, family 0..5, row12/col12 0..11, flowering_substage 0..5, lens12 0..11, context_frame 1..7, basis chromatic|fifths`.

**Timing:** discrete and logical only.
- `advance(expected_revision, ticks)` (`m1_engine.rs:432`) carries ticks into `cycle` (`advance_clock`, `:215`) and bumps `revision`.
- There is no wall-clock rate, no interpolation and no slerp.
- `KERNEL-M1-ENGINE-CONTRACT.md:230` states: "physical scheduling and realtime interpolation remain with native consumers".

| Output | Units | Computation | Coordinate-grounded? |
|---|---|---|---|
| `clock` | `tick12`; `position6=t%6`; `phase=t/6`; `conjugate_tick12=(t+6)%12`; `degree360=30t` (deg); `hopf_fiber=cycle&1`; `degree720=degree360+360·fibre`; `spanda_stage=t%6` | `m1.c:21`, `m1.rs:61` | Partly. The 12-ring is C-RET/SEED. 30°/tick is C-RET (`DEGREE_PER_TICK`). The fibre is SOURCE (`#1-5-2`), but it is indexed by cycle parity, which is C-RET. |
| `carrier` | `spinor` quat (f32); `quadrature`/`opposite_quadrature` (cos,sin of degree720 rad); `m2_carrier_count=72`; `genus=1`; `explicate_edges=4`; `identification_slots=2` | `m1_engine.rs:191`, `m1_state.c:56` | Yes for the laws: `#1-5-2` SU(2), `#1-5-3` −e^{iθ}, `#1-5-4` 72, `#1-5-5` 4g+2g. The step size is C-RET. |
| `rotor(a,b)` | quat | `e^{ia}e^{jb}` (`m1_engine.rs:158`) | Yes: `#1-5-0` literal |
| `torus(θ₁,θ₂)` | torus units, r=1 | `(16/9+cos θ₁)(cos θ₂, sin θ₂), sin θ₁` (`m1.rs:244`) | Embedding SOURCE; 16/9 DERIVED; r=1 normalisation C choice |
| `legacy_topology` | `element_count` LUT `{1,2,2,3,4,5,8,10,12,6,7,11}`, `legacy_return_stage`, `legacy_ring_quaternion` | `m1.rs:224` | The counts are SOURCE `topologicalElementCount` on `#1-3-1/-2`, `#1-3-4.0000`, `#1-3-4.0/1/2`, `#1-3-4.0/1/2/3`, `#1-4.1/.2/.3` (PD). The tick→node order and the ring quat are C-RET. |
| `cell` / `source_cell` | ints (raw/DR/decimal) + CSV literal & CSV row/col | `m1.rs:114`, `m1_engine.rs:63` | Yes: CSV (4 applied source discrepancies, §5) |
| `spanda`, `formal`, `grammar`, `reflection`, `source_traits`, `finite_field` | ints/floats | `m1.rs:169,202`; `m1_engine.rs:79,116,600,657` | Coordinate-seated (`#1-3-*`, `#1-4.*`, `#1-0/#1-1`). The tables (weave, folds) are C-RET. |
| `ratio_basis` | 8 ratios with source-row provenance | `m1.rs:335` | Yes: CSV rows (1,0), (1,3), (0,6), (1,7), (0,8) |
| `music` | pitch class, tonic, mode, 7 pitches, Name/Power | `music.rs` via `snapshot` (`m1_engine.rs:488`) | Pre-M musical authority (ledger `deep-M1:M1-C13`) |
| `pole_identity` | `(event, revision, tick12, degree720)` | `m1_engine.rs:416` | As `clock` |

**Not computed by any QL owner:**
- K² orientation over time (slerp);
- the Klein identification;
- the Hopf second sheet as a field;
- any tick rate;
- Ananda placement on the surface;
- the R+r normalisation;
- the K² × T² embedding.

**Downstream use:** `CoupledInput::compose` (`crates/ql-mef/src/continuous/coupled.rs:197-244`) takes the M1 `harmonic_ratio`, `lens12`, `tick12%6` → `SublensRef` → `Reading72` (`mef_table_index` = address72), `context_frame`, `tick12` and `degree720` into the M2 request.

---

## 3. Composition K² × T²_Mahāmāyā with M3

**What source says:**

- **QL M3 matrix §11** (`M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md:903-935`): "T²_Mahāmāyā = inscription-circle × lens-circle … two independent cyclic degrees of freedom … K² × T²_Mahāmāyā. K²: M1/M2 harmonic-audio-genesis double-cover".
- **`LIVING-INSTRUMENT-ARCHITECTURE.md:123`**: "Keep both independent phases and their declared coupling/trajectory, winding and double-cover state. A selected static aperture … alone do[es] not supply a continuous second phase."
- **SEED `alpha_quaternionic…md:221-233`**: the product lives at **M3-5**. M1-5 is the single torus plus the 720° cover. The M3-5 wheel is a "720° wheel as Hopf base S² with inscription × lens as the two co-foliating circles".
- **Graph:**
  - `#3-0 INHERITS_QUATERNION_FROM #1-5 {rotationRequirement:720, mathematicalPrinciple:"SU(2) double-covering"}`
  - `#1-5 PROVIDES_SPINOR_FOUNDATION #3` ("…enabling codon-hexagram rotations…")
  - `#3 EMPLOYS_SEQUENCE #1-5-0` (Fibonacci)
- **Native owner:** `c/include/ql/coupled_clock.h:17-25`.
  - `QL_CoupledClock{inscription, lensing: QL_PhaseLift; grid_origin_half_degrees[3]; rational rates}`.
  - `ql_m2_grid_quantum = {12,40,45}` half-degrees = 6°/20°/22.5° (`coupled_clock.c:83-85`).
  - Field `#3-0`, centre `#3-5-5/0` (`coupled_clock.c:99-100`).
  - The product itself is 4-dimensional. No source fixes its embedding in R³ (audit: "fold solver and K²×T² embedding … undetermined").

**Existing laws that relate M1 tick to M3 degree:**

| Law | Where | Relation |
|---|---|---|
| **degree/30** | `c/src/m3.c:151` `tick12=degree360/30`; `crates/ql-core/src/m3_clock.rs:66-67` ("The retained m0/hopf law: 12 ticks PER 360-degree layer, not per 720"); vendor `hopf_tick12` (`m1.h:676`); M1 `degree360 = tick·30` (`m1.c:30`) | 12 ticks / 360°, 24 / 720°, layer = `hopf_fiber` |
| **60°/tick** | vendor `RING_QUATERNION_LUT` half-angle 30°/tick (`m1.h:551`; `ql-core/src/pole/quaternion.rs:89` "ascending by 60° per tick"); vendor "Hopf bundle: S³ (tick12, 720°)" (`m1.h:607`); `RING_SIZE 12 = SU(2) double cover` (`m1.h:303`); M1 `phase = tick/6` | 12 ticks / 720°, layer = face |

**Options for the tick ↔ degree law:**

- **A — degree/30** (`degree360 = 30·tick12`, `degree720 = degree360 + 360·hopf_fiber`).
  - The M3 degree lattice at 1 tick is lens 9 (30°×12, the zodiac lens). The 12-fold chromatic/fifths lattice lands exactly on lens-9 boundaries.
  - The portal orbital point `(θ=degree, φ=tick·30°)` (CX@daa660c `state.rs:10-17`) then traces φ = ⌊θ/30°⌋·30°, a **(1,1) curve**. It winds once around each generator per 360°, so the total is 2π+2π = 4π. That is exactly `#1-5-2 .operationalSymbolics` and `#1-5 .winding` "(2π,2π)~(0,0)".
  - On a Clifford-type torus, (1,1) curves are the Hopf fibres / Villarceau circles (`#1-5-1 .resonances`). This is DERIVED (standard geometry).
  - Cost: M1 `phase` (tick/6) and `hopf_fiber` (cycle parity) remain two distinct binary sheets (T3). SU(2) return takes 24 ticks. The vendor claim "RING_SIZE 12 = double cover" is not honoured.
- **B — 60°/tick** (`degree720 = 60·tick12`, layer = `tick/6` = face).
  - The Hopf sheet and the Klein face coincide; T3-A = T3-B. The SU(2) return takes 12 ticks, matching `RING_SIZE`.
  - Cost:
    - (i) It contradicts `DEGREE_PER_TICK 30` and `_Static_assert(TRIG_STEP_DEG == DEGREE_PER_TICK*2)` (`m1.h:537,548`) and `ql_m3_clock` (`m3.c:151`). Two native laws would change.
    - (ii) 60° is **not** one of the sixteen static apertures (1,2,4,8,9,10,12,15,24,30,36,40,45,90,180,360°). It is only the 6°/20° grid closure (`LIVING §5`, M3 §11 "lcm(6°,20°) = 60°"), so one tick lands on no lens boundary except 6°-ground multiples.
    - (iii) The portal orbital point traces a **(1,2)** curve per 360° of degree, i.e. a (2,1) curve per 720°. That no longer matches `#1-5 .winding`.
- **C — decoupled.** Inscription and lensing are independent phases driven by a declared rational trajectory (`ql_clock_set_trajectory`, `coupled_clock.c:50`). The M1 tick drives only K² (orientation, face, texture row).
  - This honours "two independent phases" (LIVING:123) and keeps both native laws intact.
  - Cost: the M1→M3 coupling ratio becomes an owner-declared policy value. Examples: 9/8 (`M1 matrix §5 "9/8 epogdoon / generative interval / tick relation"`, APERTURES `22.5/20 = 9/8`) or 1/1. The M3 degree may instead come from the dated sky, i.e. the ephemeris placement the audit names.

**Embedding options for the 4-D product (UNDETERMINED):**

- **E1 — graph of the coupling.** Use CX `state.rs`: `torus(φ=tick·30°, θ=inscription degree)`. The inscription rides the K² longitude.
  - Consequence: this is a 2-D section, and lensing has no geometric seat.
- **E2 — codon annulus.** Use SEED `INTEGRATED-1-2-3…md` §5.4 (":411"): the codon annulus (64 cells × 2π/64) sits on the K² major equator, with the inscription drawn there. The lens ring is a separate 16×22.5° meta-orientation ring (M3 §11 "16+1").
  - Consequence: both phases are visible, and the M3 form stays outside the body.
- **E3 — co-foliated tori.** Use SEED alpha §3.3: two co-foliated tori, each with its own phase pair, over one Hopf base.
  - Consequence: this needs a new presentation projection with no existing owner.

---

## 4. Superseded `WT/crates/ql-mef/src/continuous/k2.rs`: invented quantities → replacement

The binding doc is `WT/docs/kernel-rebuild/K2-EXPRESSION-BINDING.md`, commit `3db025b` (7 files, +366/−7). Its superseded banner is at lines 3-10. File `k2.rs` has 874 lines.

| # | Quantity (value) | k2.rs line | Standing | Source-grounded replacement / choice |
|---|---|---|---|---|
| 1 | Sample grid **64×64** (`longitude_samples: 64, latitude_samples: 64`) | 416-423 (comment 414-415) | Invented. It is lattice-blind: 64 mod 12 = 4, so the 30° tick/chromatic lattice and the 22.5° grid never fall on samples. | Choose N_lon, N_lat as multiples of 12 (T1). **A:** 72×12k (72 = `meaning-packet.ts:257` reference ring at CX@daa660c, 5° decans). **B:** 192 = lcm(12,16,64): tick + 22.5° + codon annulus. **C:** 360: degree-native. **D:** 720: both Hopf sheets sampled. N_lat = 12k (tick circle), ×2 if T3 doubles the meridian. The budget is `MAX_SAMPLES = 262144/8 = 32768` (`:40`): 360×72 = 25920 ✓, 192×96 = 18432 ✓, 144×144 = 20736 ✓. |
| 2 | **"1°/s" rotation**: `driver_numerator: 2, driver_denominator: 1` (half-deg/s) with `rate_numerators ["9","8"]/8` | 434-456 | Invented renderer motion. Derivation: C++ angle = `half_degrees·π/360` (`cpp/.../continuous_field.hpp:197`). Inscription = 2·9/8 = 2.25 half-deg/s = **1.125°/s**; lensing = 2 half-deg/s = **1°/s**. The comment "one degree per second" (`:434`) mis-states the inscription, and "lensing keeps its native 9:8" is inverted: the 9 is on axis 0 = inscription. | The M3 degree comes from §3 law A/B/C × the T4 band, or from dated-sky placement. The 9/8 axis assignment is UNDETERMINED (§3 C). |
| 3 | `attachment: 1` (whole body rides the inscription) | 421 | Invented. It rotates the M1 body about Z by the M3 phase. | Body orientation comes from the M1 SU(2) orbit (T2), about the i-axis. The inscription sits on the longitude or annulus (E1/E2). |
| 4 | `grid_origins: [3, 9, 21]` half-deg | 449 | Invented, copied from the test fixture `crates/ql-mef/src/continuous/receipt/tests.rs:23` | Origins come from M3 anchors (`#3-5-5/0` centre, backbone) per LIVING §5 "carry … actual relative phase offsets". Values UNDETERMINED. |
| 5 | Chladni term `sin(mφ/2)sin(nθ/2)+cos(mφ/2)cos(nθ/2)` on **one sheet** | 189-195 (samples 197-205) | Partly source: SEED `M2'/M2-ARCHITECTURE.md:365-372` "L = … `2π` for torus". On one sheet, odd m (or n) is anti-periodic, which leaves **seams** at φ=0 (θ=0). This is the named anti-pattern. | Per T3. **A:** keep L = 2π and carry the term on two sheets, with sheet₁ = −sheet₀ for odd m (spinor content visible, no seam). **B:** use integer harmonics as in the source reference `meaning-packet.ts:266` `sin(θ·m+phase)·cos(θ·n)`; periodic, one sheet, but loses anti-periodic content. **C:** both. |
| 6 | Weight `wᵢ = fᵢ / max f` | 258 | Two sources disagree: SEED `M2-ARCHITECTURE.md:370` "Hz / max_hz" vs CX `meaning-packet.ts:266` `hz / audioOctet[0]` | **A:** max (M2-ARCH). **B:** octet[0] (reference implementation). UNDETERMINED. |
| 7 | Phase `(address72+i+1)·π/36` | 259 | **Source**: CX@daa660c `meaning-packet.ts:265` | Keep |
| 8 | Mode numbers `nodal_quartet[i%4].{m,n}` ∈ 1..12 | 241-247 | **Source**: M2-ARCH §5.3.1, `meaning-packet.ts:261` | Keep. Note that m,n ∈ 1..12 is itself 12-lattice-native. |
| 9 | Displacement along the outward normal (`surface`, normal `(cosθcosφ, cosθsinφ, sinθ)`) | 178-186, 275-287 | Invented reading. SEED `INTEGRATED-1-2-3…md` §5.2-5.3 makes χ a **fragment-shader texture on K²**, "not a separate mesh". | **A:** texture/colour modulation (source). **B:** normal displacement as the K8 modal physical reading (declared policy). The choice needs the owner. |
| 10 | Material `damping 0.35/s`, `strike 0.08 m`, `gain 1.0/m`, `strike_on_event true` | 427-430 | Declared policy; no source (`MATERIAL_STANDING`, `:41`) | Remains UNDETERMINED (audit "magnitudes"). Keep the declared standing on every reading. |
| 11 | Carrier weights uniform `1/18` over the fibre's 18 carriers | 331-340 | Invented | M2 routes: planet/element relations (M2 section). UNDETERMINED here. |
| 12 | `frequency_hz: 1.0` placeholder | 342 | Harmless: the composer rebinds it | — |
| 13 | Particle ↔ sample `p mod N` | 541 | Invented correspondence | The particles sample the body lattice (declared), or a 1:1 count equal to N |
| 14 | `metres_per_unit 1.0`, r = 1 normalisation (range "|x|,|y| ≤ 25/9") | 420, influence reading | Scale is presentation (CX `k2_geometry_reference.rs:23-24` "M' may scale R+r uniformly; the PROPORTIONS are law") | **A:** r = 1 (QL C). **B:** R+r = 1 (portal). UNDETERMINED normalisation; the proportion 16/9 stays fixed. |
| 15 | `sample_rate 48_000` | 439 | Device parameter (`K8-AUDIO-OUTPUT-CONTRACT.md:62` requires device = native, 8–192 kHz) | Declared; fine |
| 16 | 1e-6 coefficient rounding | 286 | Transfer optimisation | Declared; fine |
| 17 | `m1_advance` 1..1 000 000 ticks, re-strike on advance | 709-711 | Bound + policy; no tick rate anywhere | T4 band; the strike policy is declared |
| 18 | Tick cadence "PPS 12 ticks/s … M4′ 1 Hz … instrument offers both" | binding doc :112 | "PPS" is not in QL/O-I. It is the SEED `physical-pole-stack-architecture.md:37` and `m4-prime…:621`. | T4 |
| 19 | Nara centre inputs `{m1,m2,m3}` per centre | binding doc :66-68 | No source basis (audit, Nara section) | Out of M1 scope; M2-5 routing |
| 20 | Rest body `m1::torus` R=16/9, r=1 | 178 (via `crate::m1::torus`) | DERIVED; correct | Keep (see #14) |

---

## 5. Ledger discrepancies

### Existing entries for `#1*` (`fixtures/kernel/m-ledger-v1.json`, 6 entries; none on `#1-5*`)

| id | subjects | axis | state |
|---|---|---|---|
| `k4-live-spelling-composite-m1-(1, 3, 4, 4, 0)` | `#1-3-4.4.0-4.4/5` | coordinate | rejected |
| `k4-live-spelling-composite-m1-1-3-4-4-0` | same | coordinate | rejected |
| `k5-m1:difference-a-dr` | `#1-2-3-0` | source | applied |
| `k5-m1:quintessence-dr` | `#1-2-5-0` | source | applied |
| `k5-m1:quintessence-raw` | `#1-2-5` | source | applied |
| `k5-m1:sum-dr` | `#1-2-2-0` | source | applied |

Ledger rows `k5-m1:torus`, `k5-m1:topology`, `k5-m1:carrier` and `k5-m1:clock` are all `c/rust: bound, cpp: planned`. Their invariants make no body/orbit claim.

### Proposed new entries (schema `m-ledger-v1.schema.json#/$defs/discrepancy`; NOT written)

```json
[
 {"id":"gate0-m1:torus-aspect-derivation","subjects":["#1-5-1","#1-4.0","k5-m1:torus"],"axis":"source","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"#1-5-1 carries no numeric R/r ('Aspect Ratio R/r: Determines torus shape…'). C fixes R=16/9,r=1 (c/src/m1.c:147; vendor m1.h:770-773). The value is derivable only via #1-3-5→#1-4.0 (.torusGenerationBegins: 'The 16/9 ratio seeds…') →#1-4 DEVELOPS_INTO #1-5, and CSV line 12 '16/9 … 64/36 … 100'. Seeds at the pin also assert R/r=9/8 (M1-2-ANANDA-VORTEX §5.1; INTEGRATED-1-2-3 §5.2).",
  "current_authority":{"peer":"c","reference":"c/include/ql/m1.h","reason":"Executed torus; derivation not recorded as source."},
  "proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md"}]},
 {"id":"gate0-m1:torus-normalisation","subjects":["#1-5-1","k5-m1:torus"],"axis":"operational","from_peer":"c","to_peer":"rust","state":"open",
  "detail":"QL C/Rust torus uses r=1 (R+r=25/9). The C-Experiments portal uses R=0.64, r=0.36, R+r=1 (portal-core/src/state.rs:13 @daa660c). The law 'R+r=1 … outer equator = the unit 1/1' exists only post-pin (tests/k2_geometry_reference.rs @b57ddda). Same proportion, different unit.",
  "current_authority":{"peer":"c","reference":"c/src/m1.c","reason":"Only native torus."},"proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"gate0/m1-body.md"}]},
 {"id":"gate0-m1:two-sheet-indices","subjects":["#1-5-2","#1-5-3","k5-m1:clock"],"axis":"operational","from_peer":"c","to_peer":"rust","state":"open",
  "detail":"QL_M1_Clock carries two binary sheets: phase=tick/6 (face) and hopf_fiber=cycle&1 (c/src/m1.c:27,31). Vendor defines hopf_fiber from degree720≥360 and says 'Hopf bundle: S³ (tick12, 720°)' (m1.h:607,671) and RING_SIZE 12 = double cover. Under degree360=30·tick the face and the fibre differ. Which sheet carries anti-periodic (−ψ) content is unrecorded.",
  "current_authority":{"peer":"c","reference":"c/include/ql/m1.h","reason":"Executed clock."},"proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"gate0/m1-body.md"}]},
 {"id":"gate0-m1:tick-degree-law","subjects":["#1-5-2","#3-5","k5-m1:clock"],"axis":"operational","from_peer":"c","to_peer":"c","state":"open",
  "detail":"Two native laws coexist: degree/30 (c/src/m3.c:151; ql-core m3_clock.rs:67; M1 degree360=30·tick) and 60°/tick (RING_QUATERNION_LUT half-angle 30°/tick, vendor m1.h:551; ql-core pole/quaternion.rs:89). Owner ruling needed (audit 'M1↔M3 tick law').",
  "current_authority":{"peer":"c","reference":"c/src/m3.c","reason":"ql_m3_clock executed."},"proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md"}]},
 {"id":"gate0-m1:legacy-ring-orbit","subjects":["#1-5","k5-m1:topology"],"axis":"operational","from_peer":"c","to_peer":"rust","state":"open",
  "detail":"Slerp through RING_QUATERNION_LUT gives an SO(3) path 0,60,…,300,300,240,…,0 with zero-arc steps 5→6 (q6=−q5) and 11→0 (q11=−q0). That is not the seed's claim of 'one full 360° SO(3) revolution per 12 ticks' / '30° per tick' (CX Seeds M1-2-ANANDA-VORTEX §6.1-6.2). The generated carrier (m1.rs:83) is monotone 30°/tick. No slerp exists in c/, crates/, cpp/.",
  "current_authority":{"peer":"c","reference":"c/include/ql/m1.h","reason":"LUT retained as legacy, generated orbit separate."},"proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"gate0/m1-body.md"}]},
 {"id":"gate0-m1:chromatic-fifths-axes-unseated","subjects":["#1-5-1","#1-2"],"axis":"relation","from_peer":"bimba","to_peer":"rust","state":"open",
  "detail":"The graph assigns QL positions 1-2 to the meridian and 3-4 to the longitude (#1-5 .winding; #1-5-2 .connectionTo_1_4). Seeds (not compiled into m-tree) assign longitude=chromatic and meridian=fifths/tick (M1'-SPEC §10; portal state.rs φ=tick12·30°). No graph relation or property names chromatic/fifths. The music owner (music.rs:152) has both bases but no torus seat.",
  "current_authority":{"peer":"bimba","reference":"fixtures/kernel/m-tree-v1.json","reason":"Graph properties are source; seeds are not compiled."},"proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"gate0/m1-body.md"}]},
 {"id":"gate0-m1:ananda-texture-placement","subjects":["#1-2","#1-5-1","k5-m1:ananda"],"axis":"relation","from_peer":"bimba","to_peer":"rust","state":"open",
  "detail":"#1-2 MANIFESTS_GEOMETRY #1-5-1 ('Toroidal Harmonic Structure') seats Ananda on the torus. The (row,col)→(θ₁,θ₂) placement and the per-tick active cell are unspecified: the seed gives two incompatible readings (vortex §5.2 vs §6.2), and the engine takes row12/col12 as config.",
  "current_authority":{"peer":"bimba","reference":"Idea/Bimba/Map/datasets/paramasiva-deep/relations.json","reason":"Relation is source; placement absent."},"proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"gate0/m1-body.md"}]},
 {"id":"gate0-m1:cpp-z-rotation-not-su2","subjects":["#1-5-0","#1-5-2"],"axis":"operational","from_peer":"cpp","to_peer":"rust","state":"open",
  "detail":"continuous_field.hpp:193-210 write_targets applies an SO(2) Z-rotation by half_degrees·π/360. On a surface of revolution it moves only texture/modes, and the SU(2) sign is dropped (non-injective, :191-192). Source/seed orientation is an SU(2) tumble about the i-axis (vendor m1.h:447; seed vortex §6.1).",
  "current_authority":{"peer":"cpp","reference":"docs/kernel-rebuild/K8-CONTINUOUS-CONTRACT.md","reason":"'declared Z rotation' (K8-CONTINUOUS-CONTRACT.md:60)."},"proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"gate0/m1-body.md"}]},
 {"id":"gate0-m1:topology-name-anchor","subjects":["#1-5-4"],"axis":"coordinate","from_peer":"c","to_peer":"bimba","state":"open",
  "detail":"Ledger implementations c:c/src/holographic.c:ql_default_topology and c:c/src/m1.c:ql_m1_topology bind #1-5-4 only via 'bimba-name-anchor (topology)'. ql_m1_torus binds #1-5-3/#1-5-4 via the anchor 'toru'. These are census string matches, not semantic bindings.",
  "current_authority":{"peer":"c","reference":"fixtures/kernel/m-ledger-v1.json","reason":"K4 census discovery."},"proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"gate0/m1-body.md"}]}
]
```
