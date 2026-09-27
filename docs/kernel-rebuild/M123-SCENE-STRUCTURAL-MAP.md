# M1–M2–M3 live Expression scene: Gate 0 structural map

**Standing:** agent-derived, 27 September 2026, for owner review before any engine
code (QL-MEF #135 / O:I #335, Gate 0). This map is not authority: where it
differs from the owner or the graph, they win. It extends
`M123-GROUNDING-AUDIT-2026-09-27.md` and corrects that audit in §9 below. The
full per-field derivations, with every property literal and file:line, are in
`m123-scene-map/` (`m3-clock.md`, `m2-sky.md`, `m1-body.md`, `nara.md`,
`infra-split.md`). This file is their summary, the joined dataflow, and the
decisions only the owner can make.

## 0. Sources, and which graph

- **Coordinates and relations:** the C-compiled registry
  `fixtures/kernel/m-tree-v1.json` (`c/src/m_tree.c`, `m_tree_data.inc`;
  registry `259a2f49…`), plus the K8 promotions in the build registry
  `c/build/live/m-tree-v2.json` (the aperture identities `#2-0*`).
- **Property values:** the pinned source that the registry hashes,
  Epi-Logos-C-Experiments@`daa660c` `Idea/Bimba/Map/datasets/`. All 57
  pinned files re-hashed equal to `m-tree-v1.json.files[].sha256`. They are
  read the way `scripts/m2-correspondences.py` reads them (literal claims
  per coordinate and property). No new index was built.
- **Code bindings and known disagreements:** `fixtures/kernel/m-ledger-v1.json`
  (1,915 rows, 588 discrepancies).
- **The live Neo4j graph** (Omarchy `bimba-neo4j`, read-only snapshot
  27 Sep: 2,141 :Bimba nodes, 13,844 relations) was checked only where the
  scene depends on it:
  - It agrees with the pin on `PLANETARY_RESONANCE` (7 edges), on Cancer
    decan 3 `RULED_BY → Saturn`, and on the M3-5 degree nodes (same content,
    renamed keys, no codon or hexagram).
  - It carries the Neptune/Pluto `HAS_CHAKRAL_ANCHOR` edges that the
    registry compiler drops.
  - It has **lost** the decan `planetaryRuler` and `degreesRange`
    properties.

  So the map cites the pin. How the registry should follow the live
  authority that C-Experiments declared on 13 July is a separate open task,
  not settled here. That session measured the pin against the live graph
  (branch `feat/bimba-live-divergence`, `41ad072`,
  `docs/kernel-rebuild/BIMBA-LIVE-DIVERGENCE.md`) and ledgered the
  differences:
  - Live has no counterpart for decan `planetaryRuler` (114 values) or
    `degreesRange` (36).
  - Other values this scene reads are also absent live:
    `intervalStructure` ×81, `elementalCorrespondence` ×65 and
    `planetaryMode` ×7.
  - All 9 straggler edges exist live, and the QL compiler reads none of
    them (`bimba-live-relation-stragglers-uncompiled`).

  Until that task is ruled, the scene's owners must keep reading these
  values from the pin.

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

**M3 clock**

- **D1. Per-degree inscription.** The graph fixes the family per quadrant
  (G, A, C, T from 0°) and nothing finer.
  - A: charge-weighted spans (pp/4). These miss the governor quadrants, and the result depends on the C↔G coin values.
  - B: a uniform 5.625° per codon. This aligns with the governors but gives non-integer spans.
  - **C (R): 6° per non-dual 7-state codon and 5° per dual 8-state codon.** Every family has exactly 10 + 6 (checked against `native_profiles`), so each family closes at 90° on the governor arcs, and 40·6 + 24·5 = 360. The order within a family still needs a choice: prototype XXX first at the arc origin (R), or address order.
  - D: the 384-line inscription (`m3.h:941`), which needs a choice of which 24 lines.
  - E: placement by trigram direction, which contradicts the governor prototypes.
- **D2. Codon ↔ hexagram pairing.**
  - **A (R):** the graph pairing (`associatedCodons`) for the inscription, with the C bit address kept as a separate address. `m3-domain-v1` already keeps them apart.
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

- **D9. Decan ruler.** The graph is Chaldean on every reading; the C table is triplicity, and the spec does not assert it.
  - **Graph (R).** Regenerate the C table from `.planetaryRuler` (Gate 1 fix).
  - Cancer decan 3: **Moon (R)**, from five source readings, against `RULED_BY → Saturn`. Correct the graph edge.
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
  - A planet's pitch: **`intervalFromRoot` above the Sun tonic (R)**, or the `modalSignature` letter. The letter reading differs for Mercury and Mars.
  - Base frequency: this needs your value. Cousto's Sun 126 Hz is C-only; the alternative is a declared base.
- **D14. Colour.**
  - **A (R):** the Shem arc under each planet → its `planetaryColor` literal, through a declared `RenderPalette` for RGB. 18 arcs carry two literals, so pick the first by edge id, or blend.
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
ledger's own JSON shape are in each appendix.

| Id | Disagreement | Count | Who rules |
|---|---|---|---|
| g0-m2:decan-ruler | C triplicity vs graph Chaldean | 27/36 | fix (D9) |
| g0-m2:cancer-3-ruler | property/4 relations Moon vs `RULED_BY` Saturn vs C Jupiter | 1 | graph correction (D9) |
| g0-m2:planet-chakra | `elem_sig` vs `PLANETARY_RESONANCE` | Sun, Moon, Jupiter, Neptune, Pluto | owner, since the spec calls C canon (D10) |
| g0-m2:shem-links | synthetic `planet_link`/`decan_link`/`element_id` vs arcs | 63/72, 67/72, 56/72 | owner (D11) |
| g0-m2:shem-expresses | 26 `EXPRESSES_THROUGH`, 7 `MANIFESTS_THROUGH` | 33 | owner (D11) |
| g0-m2:maqam-ruler | family constant vs mode tonic/dominant | 43/72 | owner (D12) |
| g0-m2:straggler-relations | 9 Neptune/Pluto edges not compiled; **already ledgered** as `bimba-live-relation-stragglers-uncompiled` on `feat/bimba-live-divergence`, so do not duplicate | 9 | fix the compiler |
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
7. Neptune and Pluto do have relations (stragglers, uncompiled).
8. Three of the "C defects" (planet→chakra, the Shem layout, the maqam family ruler) are also in the owner's M2′ spec. They are rulings, not plain fixes.
9. `ql-sky` is the packaged provider built by the install script on #251, not a crate.
