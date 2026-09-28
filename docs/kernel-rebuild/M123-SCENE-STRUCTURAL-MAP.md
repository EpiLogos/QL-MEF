# M1–M2–M3 live Expression scene: Gate 0 structural map

**Standing:** agent-derived, 27 September 2026, re-grounded on the live map 28 September, for owner review before any engine
code (QL-MEF #135 / O:I #335, Gate 0). This map is not authority: where it
differs from the owner or the graph, they win. It extends
`M123-GROUNDING-AUDIT-2026-09-27.md` and corrects that audit in §9 below. The
full per-field derivations, with every property literal and file:line, are in
`m123-scene-map/` (`m3-clock.md`, `m2-sky.md`, `m1-body.md`, `nara.md`,
`infra-split.md`). This file is their summary, the joined dataflow, and the
decisions only the owner can make.

## Owner corrections, 28 September 2026 (supersede the sections below where they differ)

The owner reviewed this map and corrected it on five points. Each is now
traced to the design sources in Epi-Logos-C-Experiments `origin/main`
`Idea/Bimba/Seeds/`, which this map had not read:

- `M/INTEGRATED-1-2-3-COSMIC-ENGINE-ARCHITECTURE.md`
- `M/M1'/M1'-SPEC.md`
- `M/M3'/M3'-SPEC.md` §8.6, §8.15
- `M/M4'/Legacy/plans/CLOCK-AND-NARA-SPECS/HOPF-INTEGRATION-READ.md` §XI–XII, §XV–XVII
- `S/S4/S4'/Legacy/superpowers/specs/2026-04-02-cosmic-clock-unified-scene-design.md` §2

1. **M1 is the sound generator; M2 is the surface and the modulator.**
   - M1 generates: Prakāśa, the played K² torus as oscillator, and the
     M1-3 Spanda dual oscillator (`φ̇ = Δω − a·sin φ − 2b·sin 2φ`,
     a = 1, b = 9/16, beat ≈ 2.5 Hz; M1'-SPEC §14.1).
   - M2-1′ Vimarśa reads that cloud through the MEF lenses and writes
     `audio_octet[8]` / `nodal_quartet[4]`. M2 is also the cymatic skin
     on the torus.
   - The carrier reading is: torus = oscillator, lens system = modulator,
     clock = temporal carrier, profile = patch (INTEGRATED §7.3).
   - So this map's "M2's own sounding" is wrong. The planetary octave and
     the maqam are M2 **modulations** of M1's generated sound, not a
     second generator.
2. **Earth is at the centre; the scene is geocentric in 3D.**
   - Earth (`#2-5-0/1-0`) is the observer at the Axis Mundi, identity
     quaternion `(1,0,0,0)`.
   - The ten planets sit around it at their live positions.
   - The clocks are orbital structure around Earth: the solar system as
     a computational object, rooted in Earth through the planet →
     chakra relations (HOPF §XVI–XVII).
3. **The clock is the dual orthogonal codon clock, and the torus is the
   clock.**
   - θ, the major circle, is the ecliptic degree: **Clock A**, degree →
     decan (lens 5) → codon.
   - φ, the minor circle, is the Spanda fibre tick: **Clock B**,
     hexagram → codon.
   - They are two orthogonal projections of one quaternionic state, and
     they meet at the codon on the torus surface (scene design §2;
     HOPF §XI).
   - Planets sit on the equator at their ecliptic θ, so θ = λ in these
     designs. The M3-5 graph's cardinal season anchors (0° =
     Winter_Solstice) read the ring differently; that is the one
     registration left to rule, D6.
4. **The inscription is dynamic.** It is not a fixed per-degree table
   (D1 is replaced):
   - **The 384 line-change graph** is 64 × 6 = 360 dynamic degree states
     + 24 backbone (M3'-SPEC §8.6). It equals 64 base codons × 3 matrix
     paths × 2 polarities through `YIELDS_CODON` (§8.15). The live map
     holds 322 of those 384 edges; the spec says to regenerate the
     missing ones from the matrix law, with provenance.
   - **The seed placement is already in the live map** and needs only
     placing:
     - 36 pip-card codon reflections sit on the 36 decans, joined by
       tarot card: decan `m_2_3_tarot_card` = `#3-4.0` `m_3_4_tarot_card`,
       then `REFLECTS_DNA_FORM` → codon. All 36/36 match, e.g. 0–10°
       Two of Wands → TTA.
     - The 4 aces are the XXX palindromes, the four quadrant
       prototypes: AAA Cups, TTT Wands, CCC Pentacles, GGG Swords.
     - 24 court-card codons pair with the 24 backbone positions.
   - The 7-state/8-state split is 40/24: pips 27/9, courts 8/16,
     aces 4/0.
   - The 24 backbone nodes are the fixed points. The 336 others (168
     complementary pairs) are where the state evolves: symmetry is made
     and broken as the three matrices `#3-3-2-0/1/2` act, over 472
     rotational states. That behaviour is what the instrument lets the
     user discover (HOPF §XI–XII).
5. **What drives the clock.** Codons advance on the real SU(2)
   rotational state, via `spanda_codon_advance` and the 9:8 compression
   from 72 to 64, **never on the bare `tick12`** (M1'-SPEC §14.1). The
   2.5 Hz beat, the 1 Hz profile heartbeat and the 12 Hz display rate
   stay distinct. This answers D5 and D18.

**Second owner review, 28 September 2026.** These replace the items
above where they differ.

- **Degree.** The degree ring follows the live map. The cardinals are
  canonical I-Ching (`#3-5-1` North 0°, `#3-5-3` East 90°, `#3-5-2`
  South 180°, `#3-5-4` West 270°). The ecliptic, and the planets on it,
  superposes as its own reading and does not re-origin the ring. D6 is
  closed.
- **Court cards.** The HOPF read's placement of court cards on sign cusps
  or on the backbone is withdrawn. Courts keep their given role in the
  64 → 56 Minor Arcana compression: 48 single-codon cards plus 8
  dual-codon court cards × 2 codons (M3 matrix §10; M3'-SPEC §8.7).
- **The 384.**
  - The 24 backbone governors `#3-5-q-p` are nodes **in addition to**
    the 360 degree nodes, not a subset of them. 360 + 24 = 384 = 64 × 6.
  - Each governor carries its season's perfect palindrome, per the live
    map's `c_1_description` on all 24: GGG ×6 (Winter), AAA ×6 (Spring),
    CCC ×6 (Summer), TTT ×6 (Autumn). These are the four ace codons.
  - All 360 degree nodes are dynamic.
  - HOPF §XI's "336 = 360 − 24" is wrong and is not used.
- **The visual is a native Expression.**
  - The scene is an `oi.expression/v1` Document in O:I's Expressions
    format. It is not a separate renderer.
  - The contract lives in O:I `desktop/cradle/kernel/src/expression.rs`,
    `expression_scene.rs` and `expression_carrier.rs`, and in
    `docs/contracts/EXPRESSION-APPLICATION-V1.md`.
  - Structure:
    - one Scene with a 3D view;
    - bodies (Earth, the planets, the clock torus and its rings) as
      entities bound to Bimba coordinates through `subject_bind`, with
      the trace read on the verso;
    - `scene_material_set` carrying `oi.journey-scene/v1`;
    - the Scene body carrier `engine_composition`;
    - the live field attached through `native-field/` (controller,
      channel, domain, projection).
  - The kernel caps a Scene at 32 entities, so the 360 degrees, 36
    decans and 64 codons are carried as ring bodies and fields, not as
    one entity each.

**Staleness found while tracing these.** QL's frozen C reference
(`vendor/epi-kernel/reference`, locked at C-Experiments `daa660c`) lacks
37 later `Body/S/S0/epi-lib` commits that are on `origin/main`. They
include:
- the T2.11 Spanda dual oscillator (`spanda_codon_advance`, the HKB
  field);
- the canonical 12×12 raw Vortex Modulae Ananda core (10.T10.10);
- `pisano_digit_lut[60]` and a backbone degree-defect fix (04.T4.15);
- the three-matrix `m3_quat_active_state` fold;
- the 4:5:6 energy law;
- the epogdoon bridge and the planetary-elemental feed.

So the owner's ruling that "QL is stale" covers the kernel reference as
well as the registry. The port (§8, step 0) has to take the kernel too.

## 0. Sources, and which graph

**The live Bimba map is the authority.** The owner ruled this on
28 September 2026, in the closing comment of #255. QL's registry
(`fixtures/kernel/m-tree-v1.json`) and its M2/M3 generators were compiled
from the July pre-migration dataset exports (Epi-Logos-C-Experiments
`daa660c`: camelCase keys, `#` coordinates), so they are stale, not the
map. The owner names the real precursor work: port QL's registry compiler
and M2/M3 generators to read the map as it is (current coordinates,
`c_N_*` keys, relations), fed from a graph export or the daily backup.

Sources used here:

- **Live map:** Omarchy `bimba-neo4j`. It was read with read-only Cypher,
  and with one read-only snapshot taken 27 Sep 21:02Z (2,141 :Bimba nodes,
  13,844 relations). §0.1 records what it holds for this scene.
- **`daa660c` export:** the appendices in `m123-scene-map/` were derived
  first from this export. It is the same material in pre-migration form,
  cited as `#`-coordinates and camelCase keys. Where §0.1 shows a
  different current form, the map's form governs.
- **C/Rust/C++ code and the ledger:** `fixtures/kernel/m-ledger-v1.json`
  (1,915 rows, 588 discrepancies) and the file:line citations are
  unaffected.

Coordinates are written here in the `#` form. The map's form is
`M`-prefixed with parenthesised fractions: `#2-5-0/1-3` is
`M2-5-(0/1)-3`.

### 0.1 This scene's facts in the live map

Relations in the map carry structural keys; few carry a payload beyond
`c_2_relation_kind`.

**Held, under renamed keys or unchanged relations:**

- M3-5:
  - 360 degree nodes with `m_3_5_degree`, `_rotational_phase`,
    `_quadrant`, `_elemental_affinity`, `_yin_yang_balance`;
  - `FLOWS_CLOCKWISE`, `POLAR_OPPOSITE`, `GOVERNS_DEGREE_ARC`,
    `ANCHORED_BY` (360 each), `INITIATES_SEASON` (4),
    `MANIFESTS_AT_DEGREE` (4), `EARLIER_/LATER_HEAVEN_LINK` (4/10);
  - cardinal `t_3_season_anchor` and `c_3_direction`;
  - `#3-5` `c_3_lens_system`, `c_1_chamber_structure`,
    `c_3_double_covering_dynamics`.
- M3-2: codon charges `c_3_inner_charge_*` and `p_3_sequence`.
- M2-5:
  - planet `c_0_modal_signature` (the D…E letters) and `c_2_harmonic_role`;
  - `#2-5` `c_2_chaldean_order_verified` and `c_2_musical_cosmology`
    ("Sun(1/8) … Venus(2) … Mars(7)", i.e. digit = scale degree);
  - `c_5_harmonic_series_resonance` (2:1, 3:2, 4:3, 5:4, 6:5);
  - `PLANETARY_RESONANCE` (7, same pairs), `HAS_CHAKRAL_ANCHOR`
    (Neptune → -6, Pluto → -7), `CHAKRAL_VIRTUE_RECEPTION` (7),
    `GROUNDS_CHAKRAL_PATHWAY`, `FEEDS_EARTH_ELEMENT`, `ASCENDS_TO`
    (-1 → … → -7);
  - chakra `c_1_yantra_form` and `l_2_mantra_signature` (each names its
    element, e.g. "LAM - earth element activation"); Earth
    `c_5_schumann_resonance`.
- M2-3, decans: `RULED_BY` of kind "Chaldean Decan Rulership" (36, one
  per decan), `HARMONICALLY_RESONATES_WITH` "Modal-Harmonic Resonance",
  `SPANDA_TEMPORAL_RHYTHM`, `QUANTUM_FIELD_OPERATOR` (36 each), and
  `HAS_ASPECT` (72).
- M2-4.5, Shem: `m_2_4_zodiacal_influence` (the 5° arcs), 72 names each
  with one planet relation, and `EXPRESSES_THROUGH` (72).
- M2-4.3, maqamat: `c_2_tonic_note`, `c_2_dominant_note` and `c_2_ajnas`
  (the jins and their notes, 72/72); `TONIC_/DOMINANT_PLANETARY_RESONANCE`
  (62/65) and the chromatic relations (6/14).
- M1: `#1-5 c_1_winding`, `#1-5-1 c_1_operational_symbolics`,
  `#1-4.0 c_1_torus_generation_begins`, `#1-3-5 c_2_percentile_identity`,
  `MANIFESTS_GEOMETRY`, `INHERITS_QUATERNION_FROM`.
- Nara: `#4.1-1 s_4_queryable_properties`, `#4.0-1`, and `#4.1-4
  t_4_temporal_factors`.

**Held in another form:**

- The decan ruler is the `RULED_BY` relation. The sign node's
  `c_1_decanic_structure` names the three faces with their rulers and
  degrees ("Mars-ruled … (0°-10°), Sun-ruled … (10°-20°) …"). A decan's
  degree span follows from its coordinate. The decan node itself carries
  `c_2_date_range`.
- Sign nodes also carry `c_2_sign_ruler` and
  `c_2_triplicity_ruler_day/night`. So the C decan table's triplicity
  reading is a **second reading held in the map** at sign level, not a
  reading with no source (see D9).
- The governor prototype is in the governor's `c_1_description`
  ("Perfect palindromic GGG - Light within darkness"; "… AAA - Pure
  receptivity"), with `t_3_season` and `p_3_position`.

**Not held in the live map under any key or relation** (full-text search
of every node and relation property, 27–28 Sep). The scene map used each
of these, from the export:

- the per-planet just ratios and scale functions: `scalarDegree`,
  `intervalFromRoot` ("Major Seventh (15:8)"), `scaleFunction`,
  `planetaryMode`, and the decan `harmonicInterval`;
- Shem `planetaryColor`, and the other relation payloads: `decanAspect`,
  `modalContribution`, `timingOptimal`, `codonFamily`, virtue texts;
- governor `hexagramNumber`/`hexagramName`, and hexagram
  `associatedCodons` (the codon ↔ hexagram pairing);
- chakra `elementalCorrespondence`, now only inside the mantra text;
- maqam `intervalStructure`, now expressed through `c_2_ajnas`.

These are put to the owner as **D0**.

## 1. The scene as one dataflow

```text
dated sky (providers/sky/kerykeion_snapshot.py: 10 bodies, λ β r, speeds; ecliptic-of-date)
  │  body → coordinate: Sun #2-5-0/1, Venus -2, Mercury -3, Moon -4, Saturn -5,
  │  Jupiter -6, Mars -7, Neptune -8, Pluto -9, Uranus: no node; Earth #2-5-0/1-0 = centre
  ▼
M2-5 state (M2 IS the sky)
  ├─ λ → decan #2-3-e-s-d  (degreesRange; m2.rs:537 situated_decan)  → ruler (RULED_BY /
  │        .planetaryRuler, Chaldean)  → music (HARMONICALLY_RESONATES_WITH .harmonicInterval)
  ├─ λ → Shem arc #2-4.5-c-p (5°, .zodiacalInfluence) → planet + .planetaryColor
  ├─ planet → chakra #2-5-0/1-n (PLANETARY_RESONANCE .modalContribution .timingOptimal)
  ├─ Sun → all 7 (CHAKRAL_VIRTUE_RECEPTION); Earth → -1 (GROUNDS_CHAKRAL_PATHWAY)
  ├─ planet → pitch: .scalarDegree/.intervalFromRoot (just octave 1:1 … 15:8, 2:1)
  └─ hour ruler (chaldeanOrderVerified cycle) → maqam set (TONIC_/DOMINANT_PLANETARY_RESONANCE,
           chromatic bridges) → maqam intervals (M2′ §9.2 F_routing)
  ▼                                            ▼
Nara: 7 centres = #2-5-0/1-1…7, Earth -0;      M2 sounding: planetary just octave + maqam
person state #4.1-1 .chakraState; natal         (no producer today; Vimarśā octet is 12-TET
distribution by the same routes                 from M1's mode)

M3-5 clock (independent of the sky; the sky is placed AROUND it)
  360 degree nodes #3-5-5/0-d (FLOWS_CLOCKWISE, POLAR_OPPOSITE, ANCHORED_BY → 24 governors
  #3-5-q-p, 15° arcs); 4 cardinals; Axis Mundi #3-5-5/0; 720 = layer bit (ql_m3_clock)
  inscription: family per quadrant from INITIATES_SEASON (G A C T); within-quadrant: §3 D1
  integral: Σpp(64 codons)=1440, /4 = 360 (m3.h M3_INTEGRAL_INVARIANT)
  lenses: 18 apertures owned by M2-0 (#2-0-0 void 16×22.5°, #2-0-1 Pisano 60×6°,
          #2-0-2-k sixteen static), partitions of the same degrees; lens 9 = 30°×12
  T²_Mahāmāyā = QL_CoupledClock{inscription, lensing} (coupled_clock.h)

M1-5 body: torus #1-5-1 (standard embedding; R/r = 16/9 derived via #1-3-5 → #1-4.0 → #1-5)
  carrier/rotor/shadow laws (m1_state.c ql_m1_carrier, ql_m1_rotor) = graph literals;
  12-fold lattice (tick12 ↔ chromatic/fifths pitch class, music.rs:152); Ananda 12×12
  (#1-2 MANIFESTS_GEOMETRY #1-5-1; m1_source_data.inc); 4π cover (#1-5-2), shadow −e^{iθ} (#1-5-3)
  ▼
K8 composition → C++ continuous field (field_worker.cpp, continuous_field.hpp) → PCM + GPU
  → O:I Live-instrument shell (trace inspectable per quantity)
```

## 2. Standing of each element

Marks: **S** = source-defined (graph or C, cited in the appendix);
**D** = derived here (derivation in the appendix); **U** = undetermined (an
owner decision in §4); **G1** = graph↔C disagreement to fix or rule in Gate 1
(§5).

### M3-5 clock (`m123-scene-map/m3-clock.md`)

| Element | Mark | Where it lives |
|---|---|---|
| 360 degree nodes, clockwise N0→E90→S180→W270, 1° step, polar pairs | S | `#3-5-5/0-*`, `FLOWS_CLOCKWISE .step=1`, `POLAR_OPPOSITE`; `ql_m3_clock` (`c/src/m3.c:146`); `m3_engine.rs:173-195` refuses any mismatch |
| degree properties `rotationalPhase`=d/360, `quadrant`, `elementalAffinity` (16×22.5° + Void = the 16+1), `yinYangBalance` | S, and D as formulas | all 360 nodes (the audit said 359) |
| 24 governors, 15° arcs, half-month ring | S | `GOVERNS_DEGREE_ARC`/`ANCHORED_BY`, `FLOWS_TO`; `native_group_10`, `native_anchors` |
| governor prototypes: **4, one per quadrant**: 0–89 GGG/H30 Li, 90–179 AAA/H2 Kun, 180–269 CCC/H29 Kan, 270–359 TTT/H1 Qian | S | `.codonSequence/.hexagramNumber`; `INITIATES_SEASON .codonFamily` (not coordinate containment) |
| 720 double cover | D | layer bit on the same 360 nodes (`ql_m3_clock` degree720; `coupled_clock.c:19`); no pratibimba degree nodes in the graph |
| 64-codon charges, Σpp = 1440 → 360 | S (total) | `m3.h:596-604`; per suit 84/96/92/88 is the owner-ratified M3-COIN-1 value set, and the pinned graph gives 84/96/88/92 (C↔G; 112 ledger items already open) |
| per-degree codon/hexagram inscription | **U** | the graph has none, at any granularity below 90°. The C `CLOCK_DEGREE_LUT` columns are a uniform floor estimate that starts family A at 0° against the graph's G, and agrees on 5/360 codons. `m3_clock.rs` rightly refuses them. See D1. |
| codon ↔ hexagram pairing | G1 | graph `#3-1-x-y .associatedCodons` (GGG↔Li/Li) vs C bit identity (GGG = 63 = Qian) |
| 18 apertures as partitions of the degree field | S | `APERTURES-AND-CLOCK-CENTRE.md` §2; `c/build/live/m2_aperture_data.inc`; `ql-core pole/aperture.rs`. p=7 (15°×24) = the governors; p=13 = quadrants; p=11 = chambers; p=9 = 30°×12 |
| which lens is selected | S (as a control) | `SetAperture`/`ReciprocalAperture` Actions (`m3_state.rs:61-64`); no source selects a lens from state |
| lens partition origin | **U** | D4 |
| clock rate / what drives the degree | **U** | no M3 node states a rate; `ql_clock_advance` takes an external driver (D5) |
| T²_Mahāmāyā | S | `QL_CoupledClock{inscription, lensing}`; §11 of the M3 matrix |

### M2 sky (`m123-scene-map/m2-sky.md`)

| Element | Mark | Where it lives |
|---|---|---|
| body → coordinate | S | table in §1; `m2-retained-c-v1.json` bindings; Uranus has no node (`k6-m2:difference-planet`) |
| λ → decan | S | `.degreesRange`; `situated_decan` is correct, but `m2_engine.rs:501-509` then reports the **C ruler** |
| decan ruler (Chaldean faces) | S / G1 | `.planetaryRuler` 36/36 and three relation kinds 36/36; C disagrees 27/36 (D1 in the appendix) |
| λ → Shem arc → planet + colour | S / G1 | 72 arcs, 90 `planetaryColor` literals; C `planet_link` agrees 9/72 |
| planet → chakra | S / G1 | `PLANETARY_RESONANCE`: Saturn→1, Jupiter→2, Mars→3, Venus→4, Mercury→5, Moon→6, Sun→7. C `elem_sig` differs for Sun, Moon, Jupiter and is written into the owner's M2′ spec §8.5/§9.5 |
| Neptune → Ājñā, Pluto → Sahasrāra | S (uncompiled) | `HAS_CHAKRAL_ANCHOR` in the stragglers file; the registry compiler reads 0 records from it |
| planetary just octave | S | `.scalarDegree`/`.intervalFromRoot`: Sun 1:1, Venus 9:8, Mercury 5:4, Moon 4:3, Saturn 3:2, Jupiter 5:3, Mars 15:8, Sun 2:1; repeated on 36/36 decan `harmonicInterval`s |
| hour ruler cycle, weekday rulers | D | `#2-5 .chaldeanOrderVerified` digit order (+1 per hour, +3 per day) = `PLANETARY_RESONANCE .timingOptimal` |
| maqam → planet | S | 127 tonic/dominant edges, all by `modalSignature` letter; 40 chromatic edges unread by `m2-correspondences-v1.json` (10 of its 17 "gaps" have a chromatic path) |
| maqam family ruler | G1 (conceptual) | C `planet_ruler` is a per-family constant from M2′ §8.1 with no graph relation; it matches neither the tonic nor the dominant in 43/72 modes |
| hour basis (needs an observer and sunrise) | **U** | no owner computes the planetary hour; a shared geocentric snapshot refuses an observer (`kerykeion_snapshot.py:113`) |
| M2's own sounding | **U** (producer absent) | the Vimarśā octet is 12-TET from M1's mode (`m2_vimarsha.rs`); the just octave has no producer; M2′ §9.8 leaves synthesis to the audio profile |
| colour | S | Shem `planetaryColor` covers every 5° of the sky; chakra yantra colours cover 3/7; no RGB mapping exists in any owner (`RenderPalette`) |
| latitude, speed, retrograde | dropped | `attach_m2` (`kerykeion_snapshot.py:338`) passes only λ |

### M1 body (`m123-scene-map/m1-body.md`)

| Element | Mark | Where it lives |
|---|---|---|
| standard torus embedding | S | `#1-5-1 .operationalSymbolics`; `ql_m1_torus` (`c/src/m1.c:141`) |
| R/r = 16/9 | D | not on `#1-5-1`; derived via `#1-3-5` → `#1-4.0 .torusGenerationBegins` → `#1-4 DEVELOPS_INTO #1-5`, and Vortex CSV line 12. Two seeds at the pin say 9/8, from an illustrative aside |
| carrier, rotor `e^{ia}e^{jb}`, shadow `−e^{iθ}`, 72 carriers, 4g+2g | S | graph literals = `ql_m1_carrier`, `ql_m1_rotor` (`m1_state.c:46,56`) exactly |
| 4π cover, two sheets | S (law) | `#1-5-2`; but the clock carries **two** different sheet bits: face `tick/6` and fibre `cycle&1` (G1) |
| 12-fold lattice | D | tick12 ↦ one pitch class on both the chromatic and the fifths circle (bijections); 30° step = lens 9 |
| longitude = chromatic, meridian = fifths | seed only | not in the graph ("fifths" appears nowhere in the pinned datasets). The graph puts QL positions 1–2 on the meridian and 3–4 on the longitude (`#1-5 .winding`) |
| Ananda 12×12 on the torus | S (that), **U** (how) | `#1-2 MANIFESTS_GEOMETRY #1-5-1`; `#1-5-1 .practicalApplications "Texture Mapping … (θ₁, θ₂) as UV"`; placement D7 |
| K² orientation over time | **U** | no slerp exists in `c/`, `crates/` or `cpp/`; the legacy ring LUT stalls and retraces (D8) |
| C++ body motion | G1 | `continuous_field.hpp:193-210` applies only a Z rotation, which drops the SU(2) sign |
| tick rate | **U** | no owner value; seeds offer 12/s, 1 Hz or the audible band (D10) |

### Nara (`m123-scene-map/nara.md`)

| Element | Mark | Where it lives |
|---|---|---|
| 7 centres `#2-5-0/1-1…7`, Earth `-0` | S | names, element, mantra and yantra on the nodes; `M2_CHAKRA_LUT` binds the same eight |
| routes into centres | S | R1 own planet; R2 decan ruler (all 10 bodies); R3 maqam → planet; R4 Sun → all 7; R5 Earth → -1; R6 `ASCENDS_TO` (no magnitudes) |
| no `#1*`/`#3*` → chakra and no `#4*` → `#2-5*` relation | S (absence) | so per-centre `{m1,m2,m3}` inputs have no basis |
| per-person state | S (shape) | `#4.1-1 .chakraState: string`; `#4.0-1 astrologicalWeights`; `#4.1-4 timingWindow` |
| differentiation between two Naras | D | only through each person's natal distribution over the same routes |
| magnitudes | **U** | no source gives a weight, gain or combination law (D14–D16) |
| code defects | G1 | `nara.rs:145-146` false "no names" claim; ordinals 0..6 vs 1..7 in five files; `{m1,m2,m3}` inputs; draft #252 routes natal planets through the defective C `elem_sig` |

## 3. What the scene shows, per determinant (the expected structural effects)

These are the verification cases (§7), computed from source:

- **A planet crosses a decan boundary** (Mars λ 9.9° → 10.1° Aries):
  - decan `#2-3-1-0-0` → `#2-3-1-0-1`; ruler Mars → Sun (Chaldean);
  - Shem arc Jeliel `#2-4.5-0-2` (5–10°) → Sitael `#2-4.5-0-3` (10–15°); the planet stays Mars (the Aries domicile), and the `planetaryColor` literal changes;
  - the R2 centre hit moves Maṇipūra `-3` → Sahasrāra `-7`; the R1 hit (Mars → `-3`) is unchanged;
  - the clock placement (under D6-A) moves 99.9° → 100.1° inside the governor arc `#3-5-2-0` (90–104°), so the inscription family A is unchanged.
- **The clock degree crosses 90°**: governor `#3-5-1-5` → `#3-5-2-0`; family G → A; prototype H30 Li → H2 Kun; `elementalAffinity` arc changes; the sky placement is unchanged.
- **The lens changes** (p=9 → p=7): the partition changes from 12×30° to 24×15° (the governor arcs). The inscription, the sky and the sound must not change. This is an invariance test, and it is what makes the lens a reading rather than a clock.
- **M1 ticks once**: tick12 +1 → one pitch-class step on the selected basis, the face/fibre bit per D9, the Ananda cell per D7, and the orientation per D8. The M3 degree moves only if D5 couples it.

## 4. Owner decisions

Each decision gives the options with their consequence. **R** is my
recommendation, where the source leans one way. Defaults you accept
together unblock Gate 1 and the build.

**D0 — Facts the scene used that the live map does not hold** (§0.1).
For each, the owner chooses: (a) retired, so the scene does not use it;
(b) re-authored into the map, which the scene then reads; or (c) used
provisionally from the `daa660c` export, marked as such, until (a) or (b).

- The planetary just ratios decide how M2 sounds. Without them the map
  gives only "digit = scale degree" plus a partial harmonic-series list,
  so the tuning becomes a choice. **R: (b).**
- Shem `planetaryColor` is the only colour that covers the whole sky.
  Without it, the map's colour is the yantra words (3/7 chakras).
  **R: (b).**
- The codon ↔ hexagram pairing (`associatedCodons`) grounds D2.
  **R: (b), or (c) until then.**
- `decanAspect`, `modalContribution`/`timingOptimal`, virtue texts and
  `codonFamily` are not needed by the scene: the family is in the
  governor description. **R: (a) for the scene.**

**M3 clock**

- **D1. Per-degree inscription.** The map fixes the family per quadrant
  (G, A, C, T from 0°, in the governors' `c_1_description` with
  `INITIATES_SEASON`) and nothing finer.
  - A: charge-weighted spans (pp/4). These miss the governor quadrants, and the result depends on the C↔G coin values.
  - B: a uniform 5.625° per codon. This aligns with the governors but gives non-integer spans.
  - **C (R): 6° per non-dual 7-state codon and 5° per dual 8-state codon.** Every family has exactly 10 + 6 (checked against `native_profiles`), so each family closes at 90° on the governor arcs, and 40·6 + 24·5 = 360. The order within a family still needs a choice: prototype XXX first at the arc origin (R), or address order.
  - D: the 384-line inscription (`m3.h:941`), which needs a choice of which 24 lines.
  - E: placement by trigram direction, which contradicts the governor prototypes.
- **D2. Codon ↔ hexagram pairing.**
  - **A (R):** the `associatedCodons` pairing, which is not currently held in the live map (D0), for the inscription, with the C bit address kept as a separate address. `m3-domain-v1` already keeps them apart.
  - B: the C bit identity.
- **D3. Hexagram numbering in the inscription.** Answered by D2 once it is ruled. This is listed only because the C LUT's `hexagram_id` is neither King Wen nor binary.
- **D4. Lens partition origin.**
  - **A (R):** origin 0 = Alpha-Omega. This already matches p=7 and p=13 in the graph.
  - B: the graph's right-closed convention on `quadrant`/`elementalAffinity`.
  - C: a per-lens phase offset held as state.
- **D5. What drives the clock degree.**
  - **A (R):** one M1 tick = 30° (degree/30). This matches `tick12` in C and Rust, and the torus's (1,1) winding `#1-5 .winding` "(2π,2π)~(0,0)".
  - B: one tick = 60°. This merges the two sheet bits, but changes two native laws, and 60° is not a lens.
  - C: decoupled, with a declared rational trajectory (`ql_clock_set_trajectory`) and any M1 lock stated with a receipt.
  - Lensing axis: **rate 0 (R)**, so the lens changes only by Action; or a declared coupled rate.
- **D6. Where the sky sits around the clock.**
  - **A (R):** degree = (λ + 90) mod 360. This is derived from the graph's own cardinal anchors: `#3-5-1` 0° `Winter_Solstice`, `#3-5-3` 90° `Spring_Equinox`, `#3-5-2` 180° `Summer_Solstice`, `#3-5-4` 270° `Autumn_Equinox`, which are λ 270/0/90/180, with `QUARTERLY_PROGRESSION` N→E→S→W. The clock stays geometric; only the planets' placement uses these properties.
  - B: no registration, with the planets on their own ecliptic ring beside the clock.
  - C: λ = degree, the C LUT's Aries-at-0. This contradicts the cardinal anchors.
- **D7. Lens-circle phase → lens index.**
  - **B (R):** the index changes only by Action, and the phase is transition/display. APERTURES §2.2 requires reciprocity and void antipodality to stay separate laws.
  - A: floor(φ/22.5°).
  - C: a continuous fold across the mirror axis.
- **D8. K² × T² in space.**
  - **EM-C with E2 (R):** no identification of an M1 circle with M3 meaning (nothing asserts one). The codon annulus sits on the torus's major equator (seed `INTEGRATED-1-2-3` §5.4), with the 16×22.5° lens ring as a separate meta-orientation ring.
  - The alternatives: the inscription rides the meridian or the longitude (E1), or co-foliated tori (E3).

**M2 sky and sound**

- **D9. Decan ruler.** The map holds two readings. The decan-level
  `RULED_BY` relation is of kind "Chaldean Decan Rulership", and the
  sign's `c_1_decanic_structure` agrees. The sign-level
  `c_2_triplicity_ruler_day/night` is the basis the C decan table resembles.
  - **`RULED_BY` for the decan ruler (R).** Regenerate the C decan table
    from it, and keep triplicity as a sign-level reading.
  - Cancer decan 3 (`M2-3-4-0-2`): `RULED_BY` → Saturn, against
    `HARMONICALLY_RESONATES_WITH`/`SPANDA`/`QFO`/`RESONATES_WITH` → Moon
    and the sign's "Moon-ruled deep nurturing (20°-30°)". **R: Moon;
    correct the `RULED_BY` edge in the map.**
- **D10. Planet → chakra.** The graph `PLANETARY_RESONANCE` against C `elem_sig`, which your M2′ spec §8.5/§9.5 reproduces and calls canon.
  - **Graph (R).** It is the typed relation, it carries `modalContribution`/`timingOptimal`, and the kernel's own `m2-correspondences-v1.json` already follows it.
  - Alternatively, rule that the spec wins and correct the graph.
  - Outer planets: **the straggler anchors (R)**, Neptune → Ājñā and Pluto → Sahasrāra, compiled into the registry. Uranus has no node, so it acts only through its decan ruler (R) until a node is authored.
- **D11. Shem joins.**
  - **Arc geometry (R)** for planet and decan: 72/72 arcs parse, and the Shem → planet relations are sign domicile 72/72. C and M2′ §8.2 use a synthetic sequence.
  - The 26 shifted `EXPRESSES_THROUGH` and 7 wrong-element `MANIFESTS_THROUGH` edges: **geometry wins (R)**, retarget the edges; or rule the relation wins.
- **D12. Maqam routing for the moment** (M2′ §9.2).
  - Hour basis:
    - A: observer-private, with sunrise and sunset added to the provider.
    - **B (R):** an owner-declared public place of the world as a named shared observer, which needs a contract change at `kerykeion_snapshot.py:113`.
    - C: weekday ruler only.
  - Ruler's set:
    - A: the §8.1 family table, which has no graph relation.
    - B: modes whose tonic is the ruler.
    - **C (R):** tonic ∪ dominant ∪ chromatic, so every classical ruler is reachable.
  - Mode within the set: **the person's intent, with the lens as tie-break (R)**, following your spec wording.
- **D13. How M2 sounds.**
  - Join with the Vimarśā octet:
    - **A (R):** a separate bus, with the planetary octave and maqam as condition bindings on distinct modes. The mechanism exists.
    - B: retune the octet to just ratios.
    - C: sound only the ruler's Cousto Hz, which is C-only with no graph source.
  - A planet's pitch: the scale degree from `#2-5 c_2_musical_cosmology`, tuned by **the just ratios once D0 restores them (R)**, or by the `c_0_modal_signature` letter, which is held in the map but differs from the export's ratios for Mercury and Mars.
  - Base frequency: this needs your value. Cousto's Sun 126 Hz is C-only; the alternative is a declared base.
- **D14. Colour.**
  - **A (R, needs D0):** the Shem arc under each planet → its `planetaryColor` literal, through a declared `RenderPalette` for RGB. In the export, 18 arcs carry two literals, so pick the first by edge id, or blend.
  - B: the chakra yantra colour, which covers 3/7.
  - C: an element palette, which is presentation, not source.

**M1 body**

- **D15. Ananda placement.**
  - **A (R):** columns → longitude, rows → meridian/tick, so the active row follows M1 time.
  - B/C: the two seed readings, which contradict each other.
  - D: static.
  - Whatever is chosen, the sample grid must be a multiple of 12 on both axes. **R: 360 × 72** (degree-native longitude, 5° decan latitude; 25,920 ≤ 32,768 samples), or 192 × 96.
- **D16. Orientation orbit.**
  - **A (R):** the generated carrier (`m1.rs:83`): monotone 30°/tick, SU(2) return after 24 ticks, about the i-axis. This needs a C++ SU(2) tumble, not the Z rotation.
  - B: the legacy LUT, which stalls and retraces.
  - C: 60°/tick.
- **D17. Which sheet carries anti-periodic (−ψ) content.**
  - **A (R under D5-A):** the Hopf fibre (`degree720 ≥ 360`).
  - B: the Klein face.
  - C: both, with four sheets.
  - Either way, odd modes are carried on two sheets, sheet₁ = −sheet₀, so no seam.
- **D18. Tick band.** 12 ticks/s, 1 Hz world clock, audible, or **declared per context with a receipt (R)**.
- **D19. The field on the body.** Texture/colour modulation (**R**; the seed says the field is a texture on K², not a mesh), or normal displacement as a declared K8 reading.
- **D20. Torus scale.** r = 1 (the C value) or R + r = 1 (the portal value). This is presentation only; the 16:9 proportion is fixed. **R: keep r = 1 in the kernel and scale in presentation.**
- **D21. Mode amplitude weight.** Divide by the maximum frequency (M2 architecture spec) or by octet[0] (the reference implementation).

**Nara**

- **D22. Route magnitude.** **Unit hit per edge (R**, integer and transparent), or the seed hierarchy (Asc 6, Sun 5 …), or the C Keplerian column.
- **D23. How natal and event combine.**
  - **A (R):** product, so resonance appears only where the natal chart and the event share a centre.
  - B: sum.
  - C: transit-over-natal angles, which need orbs no source gives.
- **D24. Sun reception.** **Uniform (R)**; or graded by virtue; or gated by the hour.
- **D25. `ASCENDS_TO`.** **Topology/display only (R)**; or propagation with a coefficient κ; or gating.
- **D26. Earth.** **Constant grounding into `-1` (R)**; or frame only; or also sounding 7.83 Hz.
- **D27. Centre axes 5–7.** Ākāśa cap for -5 with no axis for -6/-7; the planet's element; or **no per-centre orientation (R**, with the quaternion for the whole system only).
- **D28. `chakraState`.** Numeric per coordinate with provenance (R), or categorical by thresholds.
- **D29. Ordinals.** **Key centres by coordinate, local segment 1..7, Earth 0 (R).**
- **D30. Magnitudes** (damping, strike, gains, metres). None has a source. **R:** keep each as a declared policy value that carries its standing on every reading (as `MATERIAL_STANDING` does now), and give me your values or accept the current declared ones as provisional.

## 5. Gate 1: graph↔C reconciliation, to record then fix

None of these is in the ledger yet unless stated. Proposed entries in the
ledger's own JSON shape are in each appendix, and cite the export. Re-state
each against the map's current form when recording it, after QL's registry
and generators are ported to read the map (§8, step 0).

| Id | Disagreement | Count | Who rules |
|---|---|---|---|
| g0-m2:decan-ruler | C decan table (triplicity-like) vs map `RULED_BY` Chaldean | 27/36 | D9 |
| g0-m2:cancer-3-ruler | 4 relations and the sign text say Moon, `RULED_BY` says Saturn, C says Jupiter | 1 | map correction (D9) |
| g0-m2:planet-chakra | `elem_sig` vs `PLANETARY_RESONANCE` | Sun, Moon, Jupiter, Neptune, Pluto | owner, since the spec calls C canon (D10) |
| g0-m2:shem-links | synthetic `planet_link`/`decan_link`/`element_id` vs arcs | 63/72, 67/72, 56/72 | owner (D11) |
| g0-m2:shem-expresses | 26 `EXPRESSES_THROUGH`, 7 `MANIFESTS_THROUGH` | 33 | owner (D11) |
| g0-m2:maqam-ruler | family constant vs mode tonic/dominant | 43/72 | owner (D12) |
| (straggler relations) | moot: the edges are in the map and arrive once QL compiles from it | — | the port |
| g0-m2:chromatic-paths | 40 chromatic edges unread; supersedes "17 gaps" | 10 of 17 | fix the generator |
| gate0-m3-clock-inscription-family-order | LUT A,T,C,G from 0° vs graph G,A,C,T | 270/360 | fix after D1 |
| gate0-m3-codon-hexagram-pairing | GGG↔Li vs GGG=Qian | 3 of 4 prototypes | owner (D2) |
| gate0-m3-lens9-phase-origin | LUT Aries at 0° vs cardinal anchors | — | owner (D6) |
| gate0-m3-trigram-degree-anchor | `M3_TRIGRAM_LUT` anchors match neither heaven | 7/8, 6/8 | fix to graph |
| gate0-m3-degree-quadrant-boundary | `quadrant` ceil vs governor floor at 90/180/270 | 3 | graph-internal; the relation governs |
| gate0-m3-pisano-lens-numbering | count-first vs width-first lens numbers | — | APERTURES governs |
| gate0-m1:tick-degree-law | degree/30 vs 60°/tick | — | owner (D5) |
| gate0-m1:two-sheet-indices | face vs fibre bit | — | owner (D17) |
| gate0-m1:legacy-ring-orbit | LUT slerp stalls | — | owner (D16) |
| gate0-m1:cpp-z-rotation-not-su2 | C++ drops the SU(2) sign | — | fix after D16 |
| gate0-m1:torus-aspect-derivation, -normalisation, chromatic-fifths-axes-unseated, ananda-texture-placement, topology-name-anchor | record | — | see m1-body.md |
| nara naming, ordinals, `{m1,m2,m3}` | code defects in `nara.rs` and 4 other files, and in draft #252 | — | fix after D29 |

Also noted: the census rows marking `#3-5-5/0-*` and `#3-5-q-p` as
c=unimplemented are stale, because `native_group_9/10`, `native_anchors` and
`m3_engine.rs` bind them.

The method follows the M3 coin-value precedent (`docs/pole`, PRs #111–#112):
record each entry as open with both values, get the ruling, apply it at the
owner with a test, and mark it applied. Neither side is picked silently.

## 6. Infrastructure: split #251 and #545 (`m123-scene-map/infra-split.md`)

- **QL-A (infrastructure only, off main):**
  - Take whole: `213c2a2` (managed install), `9b91d53` (ql-sky digest) and `bbf0e9f` (worker I/O).
  - Take in part: the `replace-shapes` hunks of `982b19a` (C++, `FieldSession`/`CoupledFieldSession`, receipt), and the `k8_reshape` CI step.
  - Move the ledger cache into `m_ledger.rs`. As written it changes `m2_engine.rs` and breaks the M2 proof pin (`m2-finite-proof-v1.json:21`), which only the weekly `kernel-m2.yml` checks.
  - Defer the host ops, `k2-binding` and all of `k2.rs`. `9e436cd` is dropped.
  - Dispatch `kernel-k8-continuous.yml` and `kernel-m2.yml` by hand: the worker and C++ tests are not PR-gated.
- **O:I-A (after QL-A):**
  - Take `3647a825` (companions, `oi update`, `oi where`), `OI_BIN` pinning, the world-lens host hunks, the causal-trace hooks, and the backcheck provenance pinning.
  - The QL pin in `surfaces.json` must move to the QL-A merge in the same PR, because the install script does not exist at the current pin.
- **QL-B / O:I-B:** the replacement meaning built from §4's rulings, carrying compose, the host event/influence/personal operations (personal re-keyed by coordinate and route), the session adapter, and the Live-instrument shell. The O:I instrument node tests must be wired into CI there.
- **Omarchy replay scripts** existed only in a session scratchpad. They are preserved in `m123-scene-map/replay/`, and they still target the superseded branches.

## 7. Verification plan (after the rulings)

For each §3 case: hold the event, subject and seed, and vary one determinant.
Then check the source-predicted change at each stage:

- producer output: decan, ruler, arc, colour literal, centre hits, governor, family;
- K8 inputs: bindings and frequencies;
- C++ state readback: mode frequencies, targets, PCM;
- GPU readback on the mounted renderer;
- the inspectable trace in the O:I shell.

The lens case must show **no** change in inscription, sky or sound. A
disconnected consumer (the existing `K2_DISCONNECT_TARGETS`-style negative)
must fail each case. Two controlled Naras with differing natal distributions
must receive an identical event vector E, and differ exactly on the centres
where their natal vectors N differ. Then run the managed install on the Mac,
an independent replay on Omarchy, and the owner's installed listening walk.

## 8. Order of work once you've ruled

0. Port QL's registry compiler and M2/M3 generators to read the live map (owner direction, #255). Every later step measures against the map, not the export.
1. QL-A and O:I-A: land the infrastructure, which carries no meaning.
2. Gate 1: record every §5 entry in the ledger; apply the rulings at their owners with tests (C tables regenerated from graph literals, the registry compiler taught the straggler shape, the chromatic paths, the inscription table); fix the Nara defects.
3. QL-B: the scene owners. That means clock inscription and placement, the sky → M2-5 routing with colour and sound producers, the M1 body motion and field on two sheets, and Nara reception by route. Every quantity carries its coordinate trace.
4. O:I-B: the scene in the Live-instrument shell, with the trace inspectable.
5. §7 verification, the Omarchy replay, and the listening walk.

## 9. Corrections to the grounding audit

1. There are only 4 distinct governor prototypes, not 24.
2. 360 degree nodes carry `rotationalPhase`, not 359.
3. 84/96/92/88 is the owner-ratified C split (M3-COIN-1); the pinned graph gives 84/96/88/92. Only the 360 total is common to both.
4. 31 relation kinds touch `#3-5` (1,619 distinct), not 4.
5. The apertures are M2-0's 18, per APERTURES §1–2. The "16+1" wording there is superseded.
6. The graph carries **no** per-degree inscription to reconcile. What is missing is a ruling (D1), not a transcription.
7. Neptune and Pluto do have relations: they are in the map, and QL's stale compile dropped them.
8. Three of the "C defects" (planet→chakra, the Shem layout, the maqam family ruler) are also in the owner's M2′ spec. They are rulings, not plain fixes.
9. `ql-sky` is the packaged provider built by the install script on #251, not a crate.
