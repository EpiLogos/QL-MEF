# Gate 0 — M3-5 clock structural map (QL-MEF #135 / O:I #335)

Read-only. Repo `Quaternal-Logic` main @ `4669d8d`. Registry `fixtures/kernel/m-tree-v1.json` (rev `259a2f49…`, source C-Experiments `daa660c`). Property values from `mahamaya-deep/nodes-full-detail.json` (996 records) and `mahamaya-deep/relations.json` (4891 records), cited as `MD/nodes :: ref .prop = value` and `MD/rel`. All counts below come from ad-hoc Python run over those files, the parsed `CLOCK_DEGREE_LUT` (parsed with `scripts/generate-m3.py:clock_rows`) and generated `c/build/m3_data.inc` (gitignored build output of `scripts/generate-m3.py`).

**Relation-count note:** m-tree carries 3238 relations touching `#3-5*`. That is exactly 2 × 1619 distinct source relations, because both `low-detail/relations_mahamaya.json` and `mahamaya-deep/relations.json` are compiled. Every per-kind "720" in the registry therefore means 360 distinct assertions. The low-detail node file adds nothing for the clock: degree records have `coreNature/formulation/essence/structure = null`.

---

## 1. Clock construction dataflow

| Element | Coordinates | Relation kinds (distinct source count) | Properties = literal values | C / Rust owner | Standing |
|---|---|---|---|---|---|
| **Wheel** | `#3-5` (names: "The 360° Mythic Synthesis Wheel") | `HAS_INTERNAL_COMPONENT` →`#3-5-1..4`, `#3-5-5/0` (5); `#3-0 ANCHORS_SYNTHESIS_WHEEL` {coveringAngle:720, divisionLenses:16, transformationChambers:9} (1); `#3-4 DEVELOPS_INTO`, `#3-4-5/0 CREATES_PATHWAYS_THROUGH`, `RETURNS_TO #3-0`, `INTEGRATES_WITH #4.0` {type:"Quintessence to Context"} (only cross-M edge) | `.lensSystem = "16 archetypal divisions from 1°×360 to 360°×1…"`; `.degreeArchitecture = "360 positions … 16-lens viewing capability"`; `.chamberStructure = "9 chambers × 40° each = 360°…"`; `.doubleCoveringDynamics = "720° quaternionic rotation where every position exists in light/shadow superposition"`; `.centralUnityPoint = "…(#3-5-360)…"` (historical alias, see APERTURES §4); `.internalStructure` says "Total 365 nodes" (the actual count is 390) | ledger row `deep-M3:M3-C24` (c/rust bound) | SOURCE-DEFINED (prose); node count in prose is stale |
| **64 codons + charges** | `#3-2-a-b-c` (64) | — (no codon→`#3-5` relation) | `.inner_charge_pp/nn/np/pn` on all 64, e.g. `#3-2-1-1-1 .sequence="AAA" .inner_charge_pp=18 .inner_charge_nn=-6`. The source values imply **A=6, T=9, C=7, G=8** (e.g. `AAC .pp=19`, `AAG .pp=20`) | `vendor/epi-kernel/reference/include/m3.h:48` `NUCLEOTIDE_ICHING_VALUE={6,9,8,7}` (A,T,C,G), ratified owner correction 2026-09-07 (`vendor/epi-kernel/corrections/M3-COIN-1.patch`); `c/build/m3_data.inc:23 native_values={6,9,8,7}`; `c/src/m3.c:26 ql_m3_codon` computes pp=x+y+z, mm=x−y−z, mp=x−y+z, pm=x+y−z | Charges: DERIVED in C from ratified values. Graph and C disagree on 56/64 codons, all through C↔G swapped (existing ledger, §6) |
| **Integral → 360** | the whole of `#3-2-*` | — | Σpp over 64 = 16·a + 240 per outer family. **Ratified C:** A 336, T 384, C 368, G 352 → Σ=1440, /4 = **360**; per suit /4 = 84/96/92/88. **Graph values:** A 336, T 384, C 352, G 368 → Σ=1440, /4 = 360; per suit 84/96/**88/92** | `m3.h:596-604` (`M3_INTEGRAL_INVARIANT 360`, suits 84/96/92/88 static-asserted); `vendor/.../src/m3.c:439-470 m3_verify_integral_invariant` | Total 360: DERIVED and invariant under the C↔G swap. Per-suit split: C-ratified, contradicted by the pinned graph |
| **720 double cover** | no separate pratibimba degree nodes (only 360) | `#3-0 ANCHORS_SYNTHESIS_WHEEL .coveringAngle=720` | `#3-5-5/0 .rotationalCenter="Axis_of_720_degree_double_covering_spiral"`; wheel `.doubleCoveringDynamics` (above) | `c/src/m3.c:146 ql_m3_clock`: degree720=steps%720, layer=degree720/360, polar720; LUT col `shadow_degree = d+360` (all 360 rows); `c/src/coupled_clock.c:19-22 ql_phase_double_cover_half_degrees` (turn parity); `crates/ql-core/src/m3_clock.rs:56-81` | DERIVED: layer 1 (pratibimba) reuses degree node d with layer bit. Graph gives no second-layer coordinates |
| **4 cardinals** | `#3-5-1` N, `#3-5-2` S, `#3-5-3` E, `#3-5-4` W | `MANIFESTS_AT_DEGREE` →0/360, 180, 90, 270 (4); `POLAR_OPPOSITION` N↔S, E↔W {degreeSpan:180} (2); `QUARTERLY_PROGRESSION` N→E→S→W→N {degreeSpan:90} (4); `INITIATES_SEASON` (4, see governors); `EMBODIES_EARLIER_HEAVEN`/`EXPRESSES_LATER_HEAVEN` → trigrams (4+4); `QUINTESSENTIAL_UNITY` from `#3-5-5/0` (4); `BRIDGES_CARDINAL_DIRECTIONS` from `#3-1-0,1,3,6` (8); `ANCHORS_TO_COMPASS` from `#3-3-5-1-0/1` (2) | `#3-5-1 .direction="North" .degreeAlignment=0 .seasonAnchor="Winter_Solstice" .clockFunction="Zero_degree_origin_point_and_360_return_convergence"`; `#3-5-3 .degreeAlignment=90 "Spring_Equinox"`; `#3-5-2 180 "Summer_Solstice"`; `#3-5-4 270 "Autumn_Equinox"` (18 keys each) | Rust `m3_engine` binds as coordinates; census rows `census:#3-5-1..4` c=unimplemented | SOURCE-DEFINED. Orientation is **clockwise N(0)→E(90)→S(180)→W(270)** |
| **24 governors (backbone)** | `#3-5-q-p`, q∈1..4, p∈0..5 | `GOVERNS_DEGREE_ARC` (360) / inverse `ANCHORED_BY` (360); `EMANATES_FROM` & `TRANSCENDENT_SPOKE` {spokeType:"perfectPalindrome"} → `#3-5-5/0` (24+24); `FLOWS_TO` {flowType:"seasonal", step:"halfMonth"} ring of 24 (24); `TRANSITIONS_TO` at p=5 (4); `EARLIER_HEAVEN_LINK` (4) / `LATER_HEAVEN_LINK` (10) | 11 keys each: `position` 0..5, `halfMonthNumber` 1..24, `quadrant` q, `season`, `codonSequence`, `hexagramNumber`, `hexagramName`, `element`. **Only 4 distinct prototypes, one per quadrant:** q1 `GGG / H30 "Li over Li"`, "Fire within Water", Winter; q2 `AAA / H2 Kun`, Spring; q3 `CCC / H29 Kan`, Summer; q4 `TTT / H1 Qian`, Autumn | `c/build/m3_data.inc native_group_10[24]`, `native_anchors[360]` (built and validated in `scripts/generate-m3.py:107-121`: 15 degrees each, reciprocal, clockwise and polar checked); `crates/ql-mef/src/m3_engine.rs:167-202` (refuses unless 24×15); `fixtures/kernel/m3-domain-v1.json backbones[]` codon_address 63/0/42/21, hexagram_address 45/0/18/63, standing `source-recorded-backbone-prototype-not-current-form` | SOURCE-DEFINED (arcs, prototypes). Arc of `#3-5-q-*` = [90(q−1)+15p, +14] |
| (governor ↔ cardinal) | — | `INITIATES_SEASON`: N→`#3-5-1-0` {codonFamily:"G"}, **E(`#3-5-3`)→`#3-5-2-0` {"A"}**, **S(`#3-5-2`)→`#3-5-3-0` {"C"}**, W→`#3-5-4-0` {"T"} | — | — | SOURCE-DEFINED by typed relation. The lexical parent (`#3-5-2 ⊃ #3-5-2-*`) is **not** the initiating cardinal for q=2 and q=3. Use `INITIATES_SEASON` / `GOVERNS_DEGREE_ARC`, never containment |
| **Axis Mundi** | `#3-5-5/0` | `EMANATES_PRIMARY`→0/360; `EMANATES_TRIGRAM`→`#3-1-0..7` (8); `QUINTESSENTIAL_UNITY` (4); `MAJOR_SPOKE_TO_CENTER` from degrees 15,30,…,345 {spoke_number 1..23} (23; degree 0 has none); `POWERS_CENTRAL_UNITY` from `#3-3-5-2` | 36 keys, e.g. `.dualNature="QuintessentialUnity_AND_ZeroDegree"`, `.transformationalProtocol="Enables_instantaneous_translation_between_any_two_positions"` | `c/src/coupled_clock.c:100 ql_clock_centre()` = `#3-5-5/0`; live promotion `#3-0 REALISED_AT #3-5-5/0` (`c/build/live/m-tree-v2.json`, K8) | SOURCE-DEFINED |
| **Alpha-Omega** | `#3-5-5/0-0/360` | `RETURNS_TO #3-5-5/0`; `ALIGNS_WITH_COMPASS #3-5-1`; `FLOWS_CLOCKWISE 359→0/360 {completing_cycle:true}` | `.degree=0 .isAlphaOmega=true .transcendentNature="Portal between unity and multiplicity" .elementalAffinity="Void-transitioning-to-Water" .yinYangBalance="Ultimate-Yin-turning-Yang" .quadrant=1 .rotationalPhase=0.0` | C degree 0 = `native_group_9[0]` | SOURCE-DEFINED |
| **Degree nodes** | `#3-5-5/0-1 … -359` + Alpha-Omega = **360** | `FLOWS_CLOCKWISE` {sequential:true, step:1} d→d+1 (360, all verified); `POLAR_OPPOSITE` {angular_distance:180} d→d+180 (360, verified); `ANCHORED_BY` (360); `ALIGNS_WITH_COMPASS` at 0,90,180,270 (4); `CROSS_QUARTER` {aspect:"square"} 45→135→225→315→45 (4); `MAJOR_SPOKE_TO_CENTER` (23) | **Keys on the 360 degree nodes:** `bimbaCoordinate` 360, `degree` 360, `description` 360, `elementalAffinity` 360, `name` 360, `quadrant` 360, `rotationalPhase` 360, `subsystem` 360 (=3), `yinYangBalance` 360, `compassAlignment` 4 (0 North, 90 East, 180 South, 270 West), `isAlphaOmega` 1, `transcendentNature` 1. **No codon, hexagram, line, zodiac or lens key.** `rotationalPhase = degree/360` exactly (360/360, e.g. `#3-5-5/0-17 .rotationalPhase=0.04722…`). `quadrant = ceil(d/90)` (1..359; 91/90/90/89 counts). `elementalAffinity` = 16 labels × 22.5°, index = ceil(d/22.5)−1 exact for d=1..359, plus "Void-transitioning-to-Water" at 0/360, i.e. **16+1**. `yinYangBalance`: 90° sawtooth `Yin-(90−p)-Yang-p` …; POLAR_OPPOSITE degrees swap the yin/yang values (d=1 `Yin-89-Yang-1` ↔ d=181 `Yang-89-Yin-1`); equilibrium labels at 90/180/270/360 | `native_group_9[360]`; `ql_m3_clock` degree_node / clockwise_node / polar_node; `m3_engine.rs:173-195` refuses on any FLOWS_CLOCKWISE/POLAR_OPPOSITE vs arithmetic mismatch | rotationalPhase, quadrant, elementalAffinity, yinYang: all DERIVED from the degree (formulas shown). Edges SOURCE-DEFINED and equal to arithmetic |
| **Relations (all kinds touching `#3-5*`, distinct)** | — | GOVERNS_DEGREE_ARC 360, FLOWS_CLOCKWISE 360, POLAR_OPPOSITE 360, ANCHORED_BY 360, EMANATES_FROM 24, FLOWS_TO 24, TRANSCENDENT_SPOKE 24, MAJOR_SPOKE_TO_CENTER 23, LATER_HEAVEN_LINK 10, BRIDGES_CARDINAL_DIRECTIONS 8, EMANATES_TRIGRAM 8, HAS_INTERNAL_COMPONENT 6, EMBODIES_EARLIER_HEAVEN 4, EXPRESSES_LATER_HEAVEN 4, QUARTERLY_PROGRESSION 4, INITIATES_SEASON 4, MANIFESTS_AT_DEGREE 4, EARLIER_HEAVEN_LINK 4, TRANSITIONS_TO 4, QUINTESSENTIAL_UNITY 4, ALIGNS_WITH_COMPASS 4, CROSS_QUARTER 4, ANCHORS_TO_COMPASS 2, RETURNS_TO 2, POLAR_OPPOSITION 2, and 1 each: ANCHORS_SYNTHESIS_WHEEL, POWERS_CENTRAL_UNITY, DEVELOPS_INTO, CREATES_PATHWAYS_THROUGH, INTEGRATES_WITH, EMANATES_PRIMARY = **1619** | — | — | Live-only promotions (K8, `c/build/live/m-tree-v2.json`, lineage `K8-APERTURES-CENTRE-2026-09-11`): `#3-5 RECEIVES_APERTURE_ARCHITECTURE #2-0`, `#3-0 REALISED_AT #3-5-5/0`, `#2-5 PLANETARY_CONDITION_RECEIVED_IN_FIELD #3-0` (their `class` reads `"bimba-source"`; only the `k8:` relation_ref marks them as promotions) |

Graph-internal coherence verified: governor prototypes relate to their compass positions exactly by trigram. `EARLIER_HEAVEN_LINK`: Li→E, Kun→N, Kan→W, Qian→S. `LATER_HEAVEN_LINK`: Li→S, Kan→N, Kun→S+W (SW), Qian→N+W (NW). All match `#3-1-x .directionEarlierHeaven / .directionLaterHeaven`.

One graph-internal defect: at d = 90, 180 and 270, the degree node's `quadrant` (ceil rule) disagrees with the `quadrant` of its `ANCHORED_BY` governor (floor rule).

---

## 2. Degree → codon/hexagram inscription

**What the graph carries.** Degree nodes have no codon, hexagram or line property, and no relation to `#3-1*` or `#3-2*`. The only per-degree inscription is derived by one route: degree →`ANCHORED_BY`→ governor → `.codonSequence/.hexagramNumber`. It is constant across each 90° quadrant:

| degrees | governor family | codon | hexagram | C codon addr | C hex addr (trigram bits) |
|---|---|---|---|---|---|
| 0–89 | `#3-5-1-*` | GGG | H30 Li/Li | 63 | 45 |
| 90–179 | `#3-5-2-*` | AAA | H2 Kun | 0 | 0 |
| 180–269 | `#3-5-3-*` | CCC | H29 Kan | 42 | 18 |
| 270–359 | `#3-5-4-*` | TTT | H1 Qian | 21 | 63 |

The codon↔hexagram pairing is also source-given by `#3-1-x-y .associatedCodons`, e.g. `#3-1-5-5 (H30 Li) .associatedCodons=['GGG:1','GGG:1','GGG:2',…]` and `#3-1-0-0 (H1) = TTT×6`. That pairing is **not** C's bit identity (C: codon address = hexagram address, so GGG = 63 = Qian).

**C `CLOCK_DEGREE_LUT` columns** (`vendor/epi-kernel/reference/src/m3_clock_lut.c`, header comment line 1: "Using computed hexagram approximation (no Neo4j)"). Every column was reverse-engineered over all 360 rows:

- `hexagram_id = floor(d·64/360)` (360/360). This gives 40 indices spanning 6° and 24 spanning 5°.
- `hexagram_line_active = floor(d·384/360) mod 6` (360/360).
- `codon_lower_pair = hex & 3` (360/360); `codon_upper_pair = (hex>>3) & 3` (d//45 mod 4). These are not a coherent codon decomposition.
- `is_non_dual`/`codon_class` do not follow codon class of the hex index (179/360).
- `tarot`, `ananda` and `archetype` are all 0.
- Arithmetic-only columns (exact): sign = d//30, zodiac_deg = d%30, decan = d//10, backbone flag = (d%15==0), tick12 = d//30, shadow = d+360, polar = (d+180)%360, chamber = d//40.

**Agreement: graph (quadrant prototype) vs C `hexagram_id`, per degree:**

| reading of C `hexagram_id` | agree | disagree | agreeing degrees |
|---|---|---|---|
| outer nucleotide `hex>>4` vs graph family (C order A,T,C,G; graph G,A,C,T) | 90 | 270 | 180–269 |
| exact codon address | 5 | 355 | 237–241 |
| hexagram binary address | 5 | 355 | 355–359 |
| King Wen − 1 | 0 | 360 | — |

The disagreement is not local; it spans the whole wheel. The C estimate starts family A at 0°; the graph starts family G (Winter/North) at 0°.

**What `ql-core m3_clock.rs` refuses.** `RecordedClockProjection::reconciled_symbolic_fields() = 0` (m3_clock.rs:102). `uniform_hexagram_estimate` is labelled "Arithmetic sector estimate …; not King Wen id" (:75-78), and the LUT is kept as values "not recognized symbolic assignments" (:91-93). C mirrors this: `ql_m3_clock_reconciled_symbolic_fields() { return 0; }` (c/src/m3.c:165), with field names `*_ESTIMATE`/`*_PLACEHOLDER` (c/include/ql/m3.h:25-35). The refusal is correct: the columns are a floor partition with no graph basis.

**Deriving a per-degree inscription.** The graph fixes three things: family per quadrant (G,A,C,T via `INITIATES_SEASON .codonFamily`), the quadrant prototype (XXX palindrome) and the 15° governors. It fixes **neither** the span per codon **nor** the order within a family. Concrete alternatives:

- **A. Charge-weighted span, span_c = pp_c/4.**
  - Spans are 4.5°…6.75°; only 16/64 are integers.
  - With the graph's family order G,A,C,T, the boundaries fall at 88/172/264 (ratified values) or 92/176/264 (graph values). Both miss the governor quadrants at 90/180/270.
  - With the C address order A,T,C,G, the halves close exactly: A+T = 180 = C+G. But that order contradicts `INITIATES_SEASON`.
  - Consequence: families cross governor arcs, and the choice depends on the unresolved C/G coin values.
- **B. Uniform 5.625° per codon (90/16) inside each graph quadrant.**
  - Aligns with governors, but spans are non-integer.
  - Order within the family is still a choice.
- **C. Class-based integer span: non-dual 7-state codon → 6°, dual 8-state → 5°.**
  - From `m3_data.inc native_profiles` (R7 = 40, R8/R8P = 24), every family has exactly 10 R7 + 6 R8 → 10·6 + 6·5 = **90°**.
  - This is the only integer {5,6} assignment that closes each governor quadrant, and 40·6 + 24·5 = 360.
  - Consequence: consistent with `GOVERNS_DEGREE_ARC` and the 40/24 class law (§8). Order within the family is still a choice, e.g. prototype XXX first at the quadrant start (a governor prototype sits at its arc origin) versus address order.
  - The C estimate's 6° slots coincide with 7-state codons only 22/40 times, so C does not implement C.
- **D. 384-line inscription.** The `m3.h:941` static assert says 360 degrees + 24 backbone = 64×6 lines. Each clock node carries one hexagram line, and the 24 governors carry 24 lines. This needs a choice of which 24 lines (the source says "palindromic", but there are only 16 XyX palindromes and 4 perfect ones), plus an ordering.
- **E. Place by trigram direction** (upper-trigram Later Heaven sector × lower-trigram sub-sector, 5.625°). Source properties exist (`.directionLaterHeaven`). But it contradicts the governor prototypes in all four quadrants: Li LH=South (180), while `#3-5-1-*` puts Li/Li at 0–89.

**Standing:** per-degree codon/hexagram is UNDETERMINED below 90°. C is the structurally best-fitting derivation, but it still needs an owner ruling on intra-family order.

---

## 3. Lenses

| Aperture | Definition sites | What it does to the degree field | Graph carrier on `#3-5` |
|---|---|---|---|
| 16 static, native index p: division w_p ∈ {1,2,4,8,9,10,12,15,24,30,36,40,45,90,180,360}°, segments 360/w; reciprocal p↔15−p | `docs/origami work/M3/M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md` §11 (lines ~814-847); **owner-ratified ownership** `docs/kernel-rebuild/APERTURES-AND-CLOCK-CENTRE.md` §2.1: `#2-0-2-k-{0,1}` = native p=k / 15−k; `c/registry/promotions/k8-apertures-v1.json` pairs; `c/build/live/m2_aperture_data.inc` (division_half_degrees, segments, orientation = 45p half-degrees); `c/include/ql/m2_aperture.h` (`QL_M2_APERTURE_COUNT 18`); `crates/ql-core/src/pole/aperture.rs:60-135` (`ApertureIndex`, `reciprocal()=15−p`, `orientation()=22.5°·p`) | A partition of the same 360 degree nodes: segment s = floor((d − origin)/w). It adds no nodes, changes no inscription and changes no degree identity. The reciprocal lens swaps segment width and count | p=0 (1°×360) = the degree nodes; **p=7 (15°×24) = `GOVERNS_DEGREE_ARC` exactly (origin 0)**; p=13 (90°×4) = quadrants/codon families; p=11 (40°×9) = `#3-5 .chamberStructure` (C `chamber = d//40`); p=12 (45°×8) = `CROSS_QUARTER` points and trigram compass octants; p=14 (180°×2) = `POLAR_OPPOSITE`; p=5 (10°×36) = C decan column; p=9 (30°×12) = C sign/tick12 columns |
| **Lens 9 = 30° × 12** | same | 12 arcs of 30°. It is one partition; the zodiac is its naming | none in the M3 graph; C columns `zodiac_sign`, `tick12` |
| Void/fold ring 16 × 22.5° (`#2-0-0`, "16+1") | §11 "16+1 fold-aperture" (lines ~859-881); APERTURES §2 (renamed; "16+1/M3-only wording" superseded, §7); `m2_aperture_data.inc` row 16 (`QL_M2_VOID_APERTURE`, 45 half-deg × 16); `aperture.rs AnuttaraVoidRing`; `m2_aperture.c ql_m2_void_antipode (p+8)%16` | An orientation ring for the 16 static lenses (lens p at 22.5°·p). It is also a 16-arc partition | **`elementalAffinity` on degree nodes = 16 arcs of 22.5° (right-closed) + Void at 0/360** |
| Fibonacci/Pisano 60 × 6° (`#2-0-1`) | §11 "pre-lensic ground"; `mahamaya-deep/fibonacci-60-pisano-integration.md` (status "Proposed", §6 leaves "Lens 16 vs walk" open; superseded by APERTURES §2.1); `m2_aperture_data.inc` row 17; `coupled_clock.c:84` quantum 12 half-deg | Base phase grid; position = d//6 | Rust already reads it: `FoldState::from_codon(…, clock.degree360()/6)` (`crates/ql-mef/src/m3_state.rs:179-183, 207, 262, 291`) |
| Grid closures | APERTURES §2.3; `k8-apertures-v1.json closures_half_degrees [120,180,360]`; `coupled_clock.c:86 ql_m2_grid_closure` | 6°∧20° = 60°, 6°∧22.5° = 90°, 20°∧22.5° = 180° (6/4/2 per turn); 22.5/20 = 9/8 | — |

**Lens selection.** The lens *identities* are SOURCE-DEFINED: owner-ratified K8 promotion, present only in the live registry `c/build/live/m-tree-v2.json` (28 `#2-0*` nodes, 78 promoted relations), not in pinned m-tree-v1. The *selected* lens is a **user/agent control**. It is `M3Request.aperture` with the operations `SetAperture` and `ReciprocalAperture` (`m3_state.rs:61-64, 259-268`). APERTURES §3 says: "Changing [inscription] and changing the aperture are different operations; a declared coupled trajectory can change both"; §6 says applying one is "a domain Action with its own receipt". No source selects a lens from state.

The **lens partition origin** is not carried anywhere. `QL_CoupledClock.grid_origin_half_degrees[3]` covers only the Fibonacci, elemental and void grids. Alternatives:

- O-A: origin 0 = Alpha-Omega (matches `GOVERNS_DEGREE_ARC` for p=7 and p=13).
- O-B: the right-closed convention the graph uses on `quadrant` and `elementalAffinity` (segments (w(k−1), wk], so d=90 belongs to Q1).
- O-C: a per-lens phase offset held as state (APERTURES §2.3: "Keep current relative phase offsets as state").

O-A and O-B differ at every boundary degree, e.g. the 3 quadrant-boundary degrees above.

---

## 4. Motion / tick

**Source-defined:**

- Direction is clockwise N→E→S→W: `FLOWS_CLOCKWISE` step 1, `QUARTERLY_PROGRESSION` degreeSpan 90, `TRANSITIONS_TO` at p=5.
- The unit step is 1°: `FLOWS_CLOCKWISE .step=1`, cycle closed by `{completing_cycle:true}`.
- Governors form a 24-step ring: `FLOWS_TO .step="halfMonth"`, `.halfMonthNumber` 1..24.
- No M3 node or relation states a rate. A search of all `#3*` properties for tick/rate/speed found nothing clock-related.

§11 "Tick / pulse / diachronic relation — open" (lines ~883-901) leaves the pulse/walk ontology open.

**The graph's own labels are seasonal/half-month.** Examples: `#3-5-1 .seasonAnchor="Winter_Solstice"`, `.halfMonthNumber`. These are inscription properties. No relation binds the clock to a date, ephemeris or observer. This is reported as data, not a ruling against the owner's geometric reading.

**Kernel.**

- `ql_m3_clock` (c/src/m3.c:146) is integer steps: 1 step = 1°, 720-step cover, `tick12 = degree360/30` (12 per 360-degree layer; `m3_clock.rs:65-68` comment "12 ticks PER 360-degree layer, not per 720").
- `coupled_clock.h` defines two independent `QL_PhaseLift` axes (inscription, lensing). Units are half-degrees; winding is kept in `turns`, and the 720 cover is the parity of `turns`.
- Rates are rational `rate_numerators[2]/rate_denominator` applied to an external driver in half-degrees (`ql_clock_advance`, coupled_clock.c:62-79).
- Every setter is generation-checked and transactional.
- The driver itself (what advances the clock) is not defined.

**The M1↔M3 conflict, exactly.** `vendor/.../include/m1.h:537 DEGREE_PER_TICK 30` ("30° per tick12 step on clock") versus `m1.h:533 TRIG_STEP_DEG 60` and `RING_QUATERNION_LUT[12]` (m1.h:551-563; `crates/ql-core/src/pole/quaternion.rs:89` "ascending by 60° per tick"). One M1 ring cycle is 12 ticks and covers the SU(2) double cover, with −1 at tick 11. Choices:

- **T-A — 1 M1 tick = 30° of inscription.** 12 M1 ticks = one 360° layer; the M3 720 cover = 2 M1 ring cycles. M1's −1 (tick 11) lands at 330°, not at the 360 layer boundary. This is consistent with C/Rust `tick12`.
- **T-B — 1 M1 tick = 60°.** 12 ticks = 720 = one M3 double cover, so the M1 and M3 covers coincide (M1 −1 at 660°). But then M3 `tick12` is a half-tick, which contradicts `m3_clock.rs:65`.
- **T-C — decoupled.** M1 drives K² only; inscription advances by its own declared rational trajectory (`ql_clock_set_trajectory`). Any M1–M3 lock becomes an explicit, receipted trajectory. There is no phase lock by default.
- **Lensing-axis rate:** L-0 = 0 (lens changes only by Action; APERTURES §3 says "aperture may be implicit in ordinary viewing") or L-r = a nonzero coupled rate (a declared trajectory).

The audit's "1°/s" is none of these and has no source basis.

---

## 5. K² × T²_Mahāmāyā

**Source.** §11 (lines ~903-941): T²_Mahāmāyā = inscription-circle × lens-circle, "two independent cyclic degrees of freedom"; the full clock is K² × T², with K² = "M1/M2 harmonic-audio-genesis double-cover". Further sources:

- APERTURES §5: S¹_inscription × S¹_lensing; "Static aperture identity, cyclic lensing phase, inscription phase and double-cover layer remain separately represented. The inherited `ApertureIndex` and `orientation()` convention does not by itself supply a continuous lift."
- `docs/geometry/FOLD-AND-RULING-GRAMMAR.md:158`: K² is the twist-identification of the same square.
- C realises the T² factor as `QL_CoupledClock{inscription, lensing}` (coupled_clock.h:17-25).

Undetermined:

**(i) Lens-circle phase → lens index.**
- LC-A: floor(φ_L/22.5°) on the void-ring orientation (the `native_orientation_half_degrees = 45p` convention).
- LC-B: the lens index changes only by discrete Action; φ_L is transition/display phase.
- LC-C: continuous fold across the reciprocity mirror axis at 168.75° (`aperture.rs:110-120`).

LC-A makes lens reciprocity p↔15−p a mirror, and void antipodality p↔p+8 a half-turn, on the same circle. LC-B keeps them as separate laws, as APERTURES §2.2 requires.

**(ii) Embedding with M1's torus** (`#1-5-1`, R/r = 16/9; longitude = chromatic circle; meridian = fifths/tick circle).
- EM-A: inscription ↔ M1 meridian (the tick circle, via T-A/T-B).
- EM-B: inscription ↔ M1 longitude.
- EM-C: no identification. K² (M1/M2) is the base and T² is a product factor. It is rendered as a separate torus or fibre, coupled only by declared trajectories.

EM-A/EM-B each make one M1 circle carry M3 meaning, which no typed relation asserts. The only M1 link is live-only `#3-0 RECEIVES_ROTATIONAL_TOPOLOGICAL_CAPACITY #1`. EM-C is the only choice that asserts nothing beyond source.

---

## 6. Ledger discrepancies

**Existing, touching `#3-5*` / clock inputs** (`fixtures/kernel/m-ledger-v1.json`; all `open`):

- `k7-source-legacy-clock-placeholder-574a427a879d2384`: subjects M3-C24/C26; bimba→c; relation axis. Unwarranted: computed hexagram approximation, tarot/ananda/archetype = 0, degree flag as backbone identity.
- `k7-m3:difference-aperture-source`: bimba→c, source axis. "Accepted Rust 18-lens canon is promoted into native C … Older imported 16+1 prose remains a source discrepancy".
- 112 × `k7-source-codon-charge-*`: bimba→c, authority `vendor/epi-kernel/corrections/M3-COIN-1.patch`. 56 are `#3-2-*` codons (matches the 56/64 computed here) and 56 are `#3-4.0-*`. Plus 12 × `pair-descriptor` under the same patch.
- `k7-m3:difference-inverse`, `k7-m3:difference-continuous`, `k6-m2:difference-m1-m3` (context).

**Stale assessment (not a discrepancy):** `census:#3-5-5/0-*` (360) and `census:#3-5-q-p` (24) rows say c=unimplemented and rust=unimplemented. But `native_group_9/10` and `native_anchors` (C) and `M3NodeKind::Degree/Backbone` (Rust, m3_engine.rs:149-202) bind them.

**Proposed NEW entries** (ledger shape; not written to the file):

```json
[
 {"id":"gate0-m3-clock-inscription-family-order","subjects":["#3-5","#3-5-1-0","#3-5-2-0","#3-5-3-0","#3-5-4-0"],"axis":"relation","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"{\"code\":\"clock-inscription-family\",\"source\":\"INITIATES_SEASON codonFamily + governor codonSequence: 0-89 G(GGG/H30), 90-179 A(AAA/H2), 180-269 C(CCC/H29), 270-359 T(TTT/H1)\",\"legacy\":\"CLOCK_DEGREE_LUT hexagram_id=floor(d*64/360): outer nucleotide A,T,C,G from 0\",\"family_agree\":90,\"family_disagree\":270,\"exact_codon_agree_degrees\":\"237-241\",\"source_granularity\":\"quadrant only; no per-degree codon/hexagram property or relation\"}",
  "current_authority":{"peer":"bimba","reference":"fixtures/kernel/m-tree-v1.json","reason":"Typed relations fix family per governor quadrant; sub-quadrant inscription requires an owner ruling (derivation C: 7-state 6°, 8-state 5°, 90° per family)."},
  "proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"#3-5"}]},
 {"id":"gate0-m3-codon-hexagram-pairing","subjects":["#3-1-5-5","#3-1-4-4","#3-1-0-0","#3-2-4-4-4","#3-2-3-3-3","#3-2-2-2-2"],"axis":"coordinate","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"{\"code\":\"codon-hexagram-pairing\",\"source\":\"#3-1-x-y .associatedCodons and #3-5 governors: GGG<->Li/Li(45), CCC<->Kan/Kan(18), TTT<->Qian(63), AAA<->Kun(0)\",\"native\":\"ql_m3_codon/ql_m3_hexagram share one 6-bit address: GGG=63=Qian, CCC=42, TTT=21\",\"agree\":\"AAA only\"}",
  "current_authority":{"peer":"c","reference":"c/src/m3.c","reason":"Retain both; m3-domain-v1 backbones already keep codon_address and hexagram_address separate."},
  "proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"#3-1-5-5"}]},
 {"id":"gate0-m3-lens9-phase-origin","subjects":["#3-5-5/0-0/360","#3-5-1","#3-5-3"],"axis":"operational","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"{\"code\":\"lens-phase-origin\",\"legacy\":\"CLOCK_DEGREE_LUT sign 0 at d=0 with Chaldean decan rulers Mars,Sun,Venus,... (Aries face order)\",\"source\":\"#3-5-1 .degreeAlignment=0 .seasonAnchor=Winter_Solstice; #3-5-3 .degreeAlignment=90 .seasonAnchor=Spring_Equinox\",\"native_state\":\"QL_CoupledClock has no static-lens origin\",\"alternatives\":[\"origin 0, lens-internal indices only\",\"Aries-0 at degree 90\",\"retain legacy Aries-0 at degree 0\"]}",
  "current_authority":{"peer":"bimba","reference":"docs/kernel-rebuild/APERTURES-AND-CLOCK-CENTRE.md","reason":"Relative phase offsets are state; ecliptic-longitude -> clock-degree placement of ephemeris planets depends on this ruling."},
  "proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"#3-5"}]},
 {"id":"gate0-m3-trigram-degree-anchor","subjects":["#3-1-0","#3-1-1","#3-1-2","#3-1-3","#3-1-4","#3-1-5","#3-1-6","#3-1-7"],"axis":"coordinate","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"{\"code\":\"trigram-degree-anchor\",\"native\":\"M3_TRIGRAM_LUT degree_anchor Qian0 Kun180 Zhen90 Xun315 Kan270 Li135 Gen45 Dui225 (vendor m3.c:71-87; ql-core pole/iching.rs)\",\"source\":\"directionEarlierHeaven/LaterHeaven x #3-5 cardinal degreeAlignment (N0 E90 S180 W270; intercardinals by midpoint)\",\"agree_earlier_heaven\":1,\"agree_later_heaven\":2}",
  "current_authority":{"peer":"bimba","reference":"fixtures/kernel/m-tree-v1.json","reason":"Graph EARLIER/LATER_HEAVEN_LINK relations place trigrams consistently; the C anchor matches neither arrangement."},
  "proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"#3-1"}]},
 {"id":"gate0-m3-degree-quadrant-boundary","subjects":["#3-5-5/0-90","#3-5-5/0-180","#3-5-5/0-270"],"axis":"relation","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"{\"code\":\"quadrant-boundary\",\"property\":\"degree .quadrant=ceil(d/90) (90->1,180->2,270->3)\",\"relation\":\"ANCHORED_BY -> #3-5-2-0/#3-5-3-0/#3-5-4-0 (.quadrant 2/3/4)\",\"native\":\"native_anchors follows ANCHORED_BY\"}",
  "current_authority":{"peer":"bimba","reference":"fixtures/kernel/m-tree-v1.json","reason":"Graph-internal property/relation disagreement at three degrees; typed relation is the executable binding."},
  "proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"#3-5-5/0"}]},
 {"id":"gate0-m3-pisano-lens-numbering","subjects":["#2-0-1","#2-0-2-2","#2-0-2-6"],"axis":"source","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"{\"code\":\"lens-numbering\",\"source\":\"mahamaya-deep/fibonacci-60-pisano-integration.md §2a: 'Lens 2 (4°×90°, quaternary cross)', 'Lens 6 (12°×30°, zodiacal)' (count-first reading)\",\"native\":\"k8-apertures-v1 / m2_aperture_data.inc: index 2 = 4° width x 90, index 6 = 12° x 30, index 9 = 30° x 12 (zodiacal)\"}",
  "current_authority":{"peer":"bimba","reference":"docs/kernel-rebuild/APERTURES-AND-CLOCK-CENTRE.md","reason":"Owner-ratified width-first native index governs; retained proposal prose is not lens authority."},
  "proposal":null,"decision":null,"promotion":null,"history":[{"state":"open","reference":"#2-0-1"}]}
]
```

---

## Where the audit doc (`M123-GROUNDING-AUDIT-2026-09-27.md` §M3) is contradicted or needs tightening

1. "24 governors with prototype codon/hexagram": true, but there are only **4 distinct** prototypes (one per 90° quadrant).
2. "359 degree nodes with `rotationalPhase`": all **360** carry it, including Alpha-Omega (0.0). The only other degree keys are those listed in §1.
3. "suits 84/96/92/88" is the **C-ratified** split (M3-COIN-1, 2026-09-07). The pinned graph's charges give 84/96/**88/92**; only the total 360 is common. There are 112 open ledger items.
4. It lists only 4 relation kinds; there are **31** (1619 distinct).
5. "sixteen static lenses … plus Fibonacci … plus the 16+1 fold aperture … Source §11": ownership and naming are superseded by APERTURES §1–2/§7. **M2-0 owns 18 apertures** (`#2-0-0` void 16×22.5°, `#2-0-1` Fibonacci, `#2-0-2` sixteen). These exist only as a live promotion, not in m-tree-v1.
6. "The degree → codon/hexagram inscription has not been reconciled from the M3-5 graph into the kernel": the graph **has no per-degree inscription** to reconcile. It is quadrant-level only, so what's missing is an owner ruling (§2 A–E), not a transcription.
7. It omits one graph-carried lens: degree `elementalAffinity` **is** the 16+1 (16×22.5° + Void) partition.
