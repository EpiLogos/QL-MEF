# Gate 0 — Nara reception + reusable infrastructure

Sources read (read-only): QL main `4669d8d`; pinned graph `C-Experiments@daa660c:Idea/Bimba/Map/datasets/{parashakti-deep,nara-deep}` (= registry `fixtures/kernel/m-tree-v1.json` files, git blobs `cad8b91…`/`b18faa8…`/`93922ed…`/`e0e6ea4…`); `fixtures/kernel/m2-correspondences-v1.json`, `m2-retained-c-v1.json`; C `vendor/epi-kernel/reference/src/m2.c`; QL-MEF #201 (27 Sep comments incl. correction 20:47Z), #135 (27 Sep), #134 (closed), draft #252 @ `6a6f546`.
Standing marks: **SRC** = source-defined (graph/C cited), **DER** = derived here (derivation shown), **UND** = undetermined (alternatives given).

---

# Part A — Nara

## A1. The eight loci (SRC: parashakti-deep `nodes-full-detail.json`; registry nodes)

Registry: all eight are `structural_status: source-declared`, lexical parent `#2-5-0/1` (Sun). Only Earth has a typed `HAS_INTERNAL_COMPONENT` from the Sun; chakras 1–7 are parented by coordinate prefix (`parent_basis: existing-source-prefix`).

| Coord | Name | contextFrame | Element (`elementalCorrespondence`) | Mantra (`mantraSignature`) | Yantra (`yantraForm`) | Own planet (`PLANETARY_RESONANCE` in) | Planet interval (`intervalFromRoot`, planet node) | `modalContribution` | `timingOptimal` | Sun virtue (`CHAKRAL_VIRTUE_RECEPTION` props on →-1) | ASCENDS_TO |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `#2-5-0/1-0` | Planet Earth | 4.0-4/5 | "Primary earth element source feeding Mūlādhāra" (`elementalAnchor`) | — | — | — (in: Sun `GENERATES_TERRESTRIAL_GROUNDING`) | — | — | — | — | out: `GROUNDS_CHAKRAL_PATHWAY`, `FEEDS_EARTH_ELEMENT` → -1 |
| `#2-5-0/1-1` | Mūlādhāra | 4.0-4/5 | Pṛthivī (Earth) | LAM | 4 petals, yellow square | Saturn `#2-5-5` | 3:2 (P5) | Mixolydian depth… | Saturn hour, Saturday… | Love/Peace ("rootVirtueGrounding") | → -2 |
| `#2-5-0/1-2` | Svādhiṣṭhāna | 4.0-4/5 | Āpas (Water) | VAM | 6 petals, silver crescent | Jupiter `#2-5-6` | 5:3 (M6) | Lydian mode… | Jupiter hour, Thursday… | Joy/Play (0R) | → -3 |
| `#2-5-0/1-3` | Maṇipūra | 4.0-4/5 | Agni (Fire) | RAM | 10 petals, red triangle | Mars `#2-5-7` | 15:8 (M7) | Phrygian intensity… | Mars hour, Tuesday… | Goodness (1R) | → -4 |
| `#2-5-0/1-4` | Anāhata | 4.0-4/5 | Vāyu (Air) | YAM | 12 petals, hexagram | Venus `#2-5-2` | 9:8 (M2) | Hypolydian harmony… | Venus hour, Friday… | Love/Peace (complete) | → -5 |
| `#2-5-0/1-5` | Viśuddha | 4.0-4/5 | Ākāśa (Space) | HAM | 16 petals, circle | Mercury `#2-5-3` | 5:4 (M3) | Adaptive communication… (no mode name) | Mercury hour, Wednesday… | Truth | → -6; also `SONIC_CONDUIT` → `#2-4` |
| `#2-5-0/1-6` | Ājñā | 4.0-4/5 | beyond elements (Cit) | OM | 2 petals, bindu | Moon `#2-5-4` (+ `LUNAR_FEMININE_DIVINE_INTEGRATION`) | 4:3 (P4) | Reflective mode… (no mode name; planet node says Hypodorian) | Moon phases, Monday… | Wisdom (4R) | → -7 |
| `#2-5-0/1-7` | Sahasrāra | **5/0** | beyond all elements (Sat) | silence / Anāhata Nāda | 1000 petals, pure light | Sun `#2-5-0/1` | 1:1/2:1 | Dorian solar mode… | Sun hour, Sunday… | Reality (5R) | out: `DISSOLVES_INTO` → `#0-0` |

Absences (SRC by absence): no chakra node carries a colour, hue or frequency property. Colour appears only as yantra-symbol colours for -1..-3. The one frequency is Earth `schumannResonance` "7.83 Hz". Sahasrāra's description key is misspelt `descriptio`, and its contextFrame is "5/0", not "4.0-4/5". So the claim that the chakras share Nara's frame holds for -0..-6 only.
C-only values (not graph): `M2_PLANET_LUT` Cousto frequencies (Sun 126 Hz, Moon 210, Mercury 141, Venus 221, Mars 145, Jupiter 184, Saturn 148; `m2.c:264-294`); `M2_CHAKRA_LUT` (`m2.c:302-310`) ids 0..7, element ids, tattva idx 35..31 for -1..-5, 0xFF for Earth/-6/-7. Rust already binds that table to the eight coordinates (`fixtures/kernel/m2-retained-c-v1.json` table `chakra`, bindings `#2-5-0/1-0…7`).

## A2. Inbound M2 routing to each centre (SRC)

Pinned relations touching the eight loci: 36 unique (registry holds each twice, `record` from low-detail + deep files → 72 rows).
Registry check: **no relation joins any `#1*` or `#3*` node to `#2-5-0/1-*`**. The only non-`#2` endpoint is `DISSOLVES_INTO → #0-0`.

| Centre | In-edges (kind ×count) | Total in |
|---|---|---|
| -0 Earth | HAS_INTERNAL_COMPONENT 1, GENERATES_TERRESTRIAL_GROUNDING 1 (both from Sun) | 2 |
| -1 | PLANETARY_RESONANCE 1 (Saturn), CHAKRAL_VIRTUE_RECEPTION 1, DIVINE_NAMES_CHAKRAL_RECEPTION 1, GENDER_INCLUSIVE_SOLAR_PRACTICE 1 (all Sun), GROUNDS_CHAKRAL_PATHWAY 1, FEEDS_EARTH_ELEMENT 1 (Earth) | 6 |
| -2 | PLANETARY_RESONANCE 1 (Jupiter), CHAKRAL_VIRTUE_RECEPTION 1, ASCENDS_TO 1 | 3 |
| -3 | PLANETARY_RESONANCE 1 (Mars), CHAKRAL_VIRTUE_RECEPTION 1, ASCENDS_TO 1 | 3 |
| -4 | PLANETARY_RESONANCE 1 (Venus), CHAKRAL_VIRTUE_RECEPTION 1, DIVINE_NAMES 1, GENDER_INCLUSIVE 1, ASCENDS_TO 1 | 5 |
| -5 | PLANETARY_RESONANCE 1 (Mercury), CHAKRAL_VIRTUE_RECEPTION 1, GENDER_INCLUSIVE 1, ASCENDS_TO 1 | 4 |
| -6 | PLANETARY_RESONANCE 1 (Moon), LUNAR_FEMININE_DIVINE_INTEGRATION 1 (Moon), CHAKRAL_VIRTUE_RECEPTION 1, DIVINE_NAMES 1, GENDER_INCLUSIVE 1, ASCENDS_TO 1 | 6 |
| -7 | PLANETARY_RESONANCE 1 (Sun), CHAKRAL_VIRTUE_RECEPTION 1, DIVINE_NAMES 1, GENDER_INCLUSIVE 1, ASCENDS_TO 1 | 5 |

Planet→chakra is a bijection over the 7 classical bodies: Saturn→1, Jupiter→2, Mars→3, Venus→4, Mercury→5, Moon→6, Sun→7. Only the first `CHAKRAL_VIRTUE_RECEPTION` edge (→-1) carries properties (13 keys naming all seven virtues); the other six edges are bare. `ASCENDS_TO` and `GROUNDS_CHAKRAL_PATHWAY` carry no properties (no magnitude, no sign).

### Paths from a dated sky to a centre (sky = `providers/sky/kerykeion_snapshot.py:27` BODIES: 10 bodies)

| Route | Path | Relations available (pinned) | Coverage |
|---|---|---|---|
| R1 own planet | body P → `#2-5-x` → `PLANETARY_RESONANCE` → chakra | 7 | 7 classical bodies via `PLANETARY_RESONANCE`. No Uranus node exists. **Correction (verified after this section was written):** Neptune `#2-5-8` and Pluto `#2-5-9` do carry `HAS_CHAKRAL_ANCHOR` → Ājñā `-6` / Sahasrāra `-7` in `low-detail/parashakti-stragglers-relations.json`; the m-tree compiler reads 0 records from that file, so the registry lacks them (see m2-sky.md D8). The live graph carries both edges. |
| R2 decan ruler | λ_P → sign ⌊λ/30⌋ → decan ⌊(λ mod 30)/10⌋ → decan node `#2-3-{e}-{s}-{d}` (e: 1 Fire Ari/Leo/Sag, 2 Earth Tau/Vir/Cap, 3 Air Gem/Lib/Aqu, 4 Water Can/Sco/Pis) → `RULED_BY` (type "Chaldean Decan Rulership") → planet → R1 | 36 decan `RULED_BY` (exactly one per decan). Sign level: 11 `RULED_BY` + 1 `TRADITIONALLY_RULED_BY` (Aquarius→Saturn). Also per decan: 36 `HARMONICALLY_RESONATES_WITH`, 36 `SPANDA_TEMPORAL_RHYTHM`, 1 `RESONATES_WITH` → planets | all 10 bodies, including outer planets through their longitude |
| R3 maqam | selected maqam `#2-4.3-g-i` → `TONIC_PLANETARY_RESONANCE` (62) / `DOMINANT_PLANETARY_RESONANCE` (65) → planet → R1; chromatic: `TONIC_CHROMATIC_BRIDGE` 12 (3 maqams), `DOMINANT_CHROMATIC_TENSION` 28 (7 maqams) | 72 maqams: 10 lack a tonic planet, 7 lack a dominant planet (= the 17 `gaps` in `m2-correspondences-v1.json`) | which maqam is current is **not** given by the sky (see U4) |
| R3′ other M2 carriers into planets | 24 spiritual maqamat `#2-4.2` (`PLANETARY_SPIRITUAL_CORRESPONDENCE` 68, `_DEVELOPMENT` 69); 72 Names `#2-4.5` (ILLUMINATED_BY 8, HARMONIZED_BY 15, …, 98 total) | these land on planets and can therefore reach chakras via R1 | selection not given by the sky |
| R4 Sun reception | Sun `#2-5-0/1` → `CHAKRAL_VIRTUE_RECEPTION` → all 7 (plus `DIVINE_NAMES_CHAKRAL_RECEPTION` → 1,4,6,7; `GENDER_INCLUSIVE_SOLAR_PRACTICE` → 1,4,5,6,7) | 7 (+4, +5) | always present; no magnitude |
| R5 Earth grounding | Earth `-0` → `GROUNDS_CHAKRAL_PATHWAY` + `FEEDS_EARTH_ELEMENT` → -1 only | 2 | constant |
| R6 inter-centre | `ASCENDS_TO` -1→-2→…→-7; -7 `DISSOLVES_INTO` `#0-0`; -5 `SONIC_CONDUIT` → `#2-4` | 6 + 2 | topology only |

Existing Rust owner of R3: `crates/ql-mef/src/m2_condition.rs:120-215` verifies maqam→planet→chakra rule paths against registry relations with provenance (127 rules = 62 tonic + 65 dominant; planet→chakra counts Saturn 47, Sun 36, Venus 22, Moon 11, Jupiter 4, Mars 4, Mercury 3). It ignores the chromatic relations (audit). Reuse it for R3; do not re-derive.

Graph↔C disagreements on these routes. These are Gate 1 items and must be recorded as ledger discrepancies:
- `M2_PLANET_LUT.elem_sig` (`m2.c:266-293`): C gives Sun→Earth(0, "not chakra-mapped"), Moon→Svādhiṣṭhāna, Jupiter→Maṇipūra, Uranus→Ājñā, Neptune→Sahasrāra, Pluto→Mūlādhāra. The graph gives Sun→-7, Moon→-6, Jupiter→-2, and nothing for the three outer planets.
- Decan rulers: C uses triplicity; the graph uses Chaldean faces (audit). Cancer decan 3: the property says Moon, the relation says Saturn (audit).
- **Draft #252 inherits the C defect.** `intake_composition.rs` (@`6a6f546`, lines 165-191) partitions natal evidence by `m2::unpack_signature(elem_sig)`. Its decision doc table (`docs/NARA-IDENTITY-COMPOSITION-DECISIONS.md:15-25`) lists Moon→Svādhiṣṭhāna, Jupiter→Maṇipūra, Uranus→Ājñā, Neptune→Sahasrāra, Pluto→Mūlādhāra, and Sun unmapped. All of these contradict R1.

## A3. What `#4` holds per person (SRC: nara-deep)

- `#4.1-1` Energy-Body Architecture: `f_queryableProperties` = `targetSystems: list`, `organNetwork: string`, **`chakraState: string`**; `f_outputs: energyBodyMap`; `anatomicalSystems` "Chakras, meridians, organ networks, marma points, extraordinary vessels". Its only relation is `#4.1 HAS_INTERNAL_COMPONENT`.
- `#4.0` Mahamaya Identity Matrix: input `birthData {date,time,location}`; output `identitySignature {quaternaryCompass, deltaSigmaSets, archetypalWeights: map}`. Sub-offices: `#4.0-1` Astrological Chart ("Map Planetary Positions to Archetypal Weights" → `astrologicalWeights: map`); `-0` deltaSigmaSets/quaternaryCompass; `-2` jungianWeights; `-3` geneKeyWeights; `-4` humanDesignDirectives; `-5` aggregate → archetypalWeights.
- `#4.1` input `stateVector, identitySignature`. `#4.1-4` Temporal Astrological Intelligence: `timingWindow {start,end,quality,affinities,planetaryHour,lunarPhase,decanTransit}`.
- `#4.3-0` `f_stateProperties` [phase6, storey12, stroke24]; `#4.4.4.4` [currentPhase12, activeLensPreference, synthesizedSignature, consentFlags].
- contextFrame "4.0-4/5" on `#4.0`, `#4.1`, `#4.2`, `#4.3`, `#4.4`, … and on chakras -0..-6 (not -7).
- **Typed `#4*`↔`#2-5*` relations: none.** In the registry, 88 `#4→#2` edges all come from `#4`, `#4.4.3*` or `#4.4.4.*` and land on `#2`, `#2-1*` or `#2-2*` (tattvas). 6 `#2→#4` edges come from `#2` and `#2-1-0-4`. None touch `#2-5`.

## A4. Verified defects in current Nara code (QL main `4669d8d`)

| # | Defect | Location |
|---|---|---|
| D-a | False claim that the source lacks chakra names | `crates/ql-mef/src/nara.rs:145-146` ("does not assign a canonical chakra name that the recovered source lacks"). Names are in the registry node `names` and C `m_tree_data.inc:739-746`. |
| D-b | Ordinals 0..6 against source 1..7 (Earth 0) | `nara.rs:147,160,236,360`; `nara/domain/embodied.rs:128,240`; `anima_expression_profile.rs:138` ("must be 0..6"); `focused_instrument.rs:525-528` locus `ql:nara:{subject}:centre:{ordinal}`, which mints 0-based locus ids; `anima_expression_profile.rs:185`. Contrast: C `CHAKRA_*` enum 0=Earth…7 (`vendor/.../include/m2.h:64-71`), Rust `m2-retained-c-v1.json` chakra table 0..7, `m2-correspondences-v1.json` `chakra_index` 1..7. #252 `intake_composition.rs:189` does `ordinal = chakra-1`, carrying the off-by-one into the new code. |
| D-c | Per-centre `{m1,m2,m3}` inputs with no source basis | `nara.rs:150-153` `world_weights: [f64;3]`; `:325-330` `ReceiverEventInput {m1,m2,m3}`; `:372` `input_basis: [WorldContribution;3]`; `:494-500` drive = Σ w·[m1,m2,m3]; `nara/domain/embodied.rs:39-47` `CentreWorldInputs`; `anima_expression_profile.rs:50-105` m1/m2/m3 fields; `focused_instrument.rs:532-534`; `continuous/personal.rs:91-99` `receive_personal`. |
| D-d | Centres keyed by label/ordinal, not coordinate | `ReceiverConstitution.label` free text (`nara.rs:148`); test/fixture labels `centre-{n}` (`nara.rs:621`, `nara/multi.rs:198`, `nara/domain_operations.rs:518`, `nara/domain/acceptance.rs:29`). No field carries `#2-5-0/1-n`. |
| D-e | Per-receiver quaternion orientation/alignment with no source | `nara.rs:152` `orientation`, `:497-499` alignment × gain. #201 correction lists "quaternion axes for centres 5–7" as open. |
| D-f | EarthBody not keyed to `#2-5-0/1-0` | `EarthBodyConstitution {source, frame_ref, orientation}` (`nara.rs:174-179`); the frame ref is "earth-fixed" in tests. |
| D-g | Superseded K² binding's centre model | #201 comment 17:34Z ("rule from the current event's native values to each centre's M1/M2/M3 value"), retracted 20:47Z |

Kept (these match source): `RECEIVER_COUNT = 7`, `CENTRE_COUNT = 7` (`nara/domain/common.rs:9`), EarthBody not an eighth peer (`anima_expression_profile.rs:6,116`), EFWA order, consent/lifecycle, stale/cross-event refusal (`nara.rs:340-365, 459-478`), same-input replay.

## A5. "Two differently constituted Naras receive the same event with differentiated centre behaviour"

### Source-defined (SRC)
1. The centres are the eight coordinates above. Both Naras share the same centre identities; only person state differs (`#4.1-1 chakraState` per person; `#4` "PCO/Pratibimba overlay maintains per-user lived experience").
2. The event reaches the centres only by M2 routes R1–R6. For one dated sky, both Naras receive the **same** event route set: for each body, its R1 centre (7 classical) and its R2 decan-ruler centre (all 10).
3. The constitution reaches the centres by the same routes applied to natal positions. This gives a natal planetary–chakral distribution (#201 correction; `#4.0-1` "Map Planetary Positions to Archetypal Weights"; seed `nara-m4-0-identity-branch-integration-map.md` §10.3 "planetary body → Parashakti planet node → chakra node", §11 output `planetary_chakral_distribution`).
4. Hence the only source-defined source of differentiation is the natal distribution over `#2-5-0/1-1..7`. Two Naras differ where their natal R1/R2 hits differ, and the same event lands on those different distributions.
5. Timing vocabulary: `#4.1-4 timingWindow` {planetaryHour, lunarPhase, decanTransit}; `PLANETARY_RESONANCE.timingOptimal` (text).

### Derived (DER)
- Per-centre event hit vector E_c = Σ over routes of the hits landing on c (integers from R1/R2; +1 each from R4 to all seven; +1 from R5 on -1). Per-centre natal vector N_c from the same computation on natal longitudes. Both are computable now from the registry with no coefficient beyond "one hit per edge".
- The EFWA axes exist for -1..-4 only: Pṛthivī/Āpas/Agni/Vāyu = Earth/Water/Fire/Air. -5 is Ākāśa; -6 and -7 are beyond elements. This is why the axes for 5–7 are open (D-e).

### Undetermined — owner choices (UND)
| ID | Question | A | B | C |
|---|---|---|---|---|
| U1 | Route magnitude per edge | A: unit hits (count). Consequence: transparent, integer, no invented number, coarse. | B: seed §10.5 hierarchy (Asc 6, Sun 5, Moon 5, chart ruler 4, personal/social 3, outer 2). Consequence: provisional seed numbers become law; needs chart ruler/asc (birth time). | C: C `keplerian` column (as #252 uses). Consequence: C-sourced, not graph; ties Nara to a table Gate 1 is correcting. |
| U2 | Natal × event combination | A: product r_c = N_c·E_c (resonance only where natal and event share a centre). Consequence: a centre with no natal hit is silent. | B: sum s_c = N_c + E_c. Consequence: event dominates for sparse charts. | C: planet-to-own-natal-planet angular relation (transit P over natal P′, both routed). Consequence: needs aspect orbs; no source gives them. |
| U3 | Uranus/Neptune/Pluto | A: R2 only (decan ruler of their longitude). Consequence: graph-only, but they have no own centre. | B: adopt C `elem_sig` (Ājñā/Sahasrāra/Mūlādhāra) as a recorded discrepancy. Consequence: contradicts graph silence. | C: drop them from Nara. Consequence: 3 of 10 sky bodies inert. |
| U4 | Current maqam (R3) | A: M2′ §9.2, the planetary-hour ruler's set + intent + lens-mode. Consequence: needs observer location/time; intent is a person input. | B: none until the person selects one. Consequence: R3 dormant by default. | C: all maqams whose tonic = hour ruler, as a set. Consequence: broad and deterministic. |
| U5 | Sun reception (R4) magnitude | A: uniform to all 7. Consequence: shifts every centre equally, so it does not differentiate. | B: ordinal grade by virtue R-number (0R…5R). Consequence: the text-derived order becomes a number. | C: gated by Sun's diurnal/hour state. Consequence: needs observer. |
| U6 | `ASCENDS_TO` dynamics | A: topology/display only. | B: directed neighbour propagation with one coefficient κ (value UND). | C: gating (n active only if n−1 active). Consequence: strongly shapes behaviour with no source number. |
| U7 | Earth `-0` | A: constant grounding input to -1. | B: frame only (geocentric observer), no magnitude. | C: also carries 7.83 Hz as its sonic property. |
| U8 | Centre orientation axes -5..-7 | A: -5 Ākāśa → scalar/Aether cap (seed P0→Aether); -6/-7 no axis (alignment term omitted). | B: inherit own planet's element. Consequence: planets carry no element in the graph; only C `elem_sig` (defective). | C: drop per-centre orientation; the quaternion stays whole-system only (#134 "composed reading"). |
| U9 | `chakraState` representation | A: numeric per coordinate + provenance. Consequence: `#4.1-1` types it `string`. | B: categorical string by thresholds. Consequence: thresholds UND. | — |
| U10 | Ordinals | A: key by coordinate, local segment 1..7, Earth 0 (matches graph, C and Rust m2). | B: keep a 0..6 array index but carry the coordinate. Consequence: keeps the off-by-one latent. | — |

Proof shape (for #135 acceptance) that needs no UND beyond U1-A/U2: two controlled natal longitude sets whose R1/R2 distributions differ on at least one centre, and one dated sky. Assert that E is identical for both, that N differs, and that per-coordinate outputs differ exactly on the centres where N differs. Assert that no `{m1,m2,m3}` input exists.

---
