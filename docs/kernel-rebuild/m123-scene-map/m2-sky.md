# Gate 0 — M2: the sky is M2-5 (structural map section)

Read-only research, 2026-09-27. Repo `Quaternal-Logic` main @ `4669d8d`. Registry
`fixtures/kernel/m-tree-v1.json` (registry_revision `259a2f49…`, source
`EpiLogos/Epi-Logos-C-Experiments@daa660cb`). All seven pinned parashakti source
files re-hashed against `m-tree-v1.json.files[*].sha256`: **7/7 match**
(`parashakti-deep/{nodes-full-detail,relations,parashakti-planets}.json`,
`low-detail/{nodes_parashakti,relations_parashakti,parashakti-stragglers-nodes,parashakti-stragglers-relations}.json`).
Short cite form below: `PD/nodes :: <coord> .<prop>` = parashakti-deep/nodes-full-detail.json;
`PD/rels` = parashakti-deep/relations.json; `LD/strag-*` = low-detail stragglers.
Counts are over the deep files (each deep relation is compiled **twice** in m-tree:
once from `parashakti-deep/relations.json`, once from `low-detail/relations_parashakti.json`).

**Audit contradictions found (details in place):**
1. Neptune/Pluto are *not* relation-less: `LD/strag-relations` holds 9 unique edges; the m-tree compiler dropped all of them (0 records from that file). §1, D8.
2. The C values the audit calls defects (maqam family ruler, planet→chakra `elem_sig`, sequential `decan_link`) are **also written into the owner's M2′ spec** (§8.1, §8.2, §8.5, and §9.5 "`M2_PLANET_LUT[10]` is canon"). The disagreement is graph vs (C + M2′ spec), not graph vs C alone. §4, §5.
3. New defects not in the audit: 7/72 Shem `MANIFESTS_THROUGH` land in the wrong element (D7); the sky boundary drops latitude/speed before M2 (§1); spec §8.2 "2 names per decan = light/shadow" is contradicted by the graph's 58 light / 14 shadow faces (D6).

---

## 1. Dated sky → M2-5 state

**Owner:** `providers/sky/kerykeion_snapshot.py`. **No `ql-sky` crate exists**; `grep -rl "ql-sky\|ql_sky" --include=Cargo.toml` finds nothing. Pinned deps: `kerykeion==5.7.1` and `pyswisseph==2.10.3.2` (`providers/sky/requirements.txt`).

| Output | Symbol / line |
|---|---|
| schema `ql.sky-snapshot/v1`; request `ql.sky-request/v1` | `SCHEMA`, `REQUEST` :25-26 |
| bodies (fixed order) Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune, Pluto | `BODIES` :27 |
| per body: `longitude_degrees`, `latitude_degrees`, `distance_au`, `longitude_speed_degrees_per_day`, `latitude_speed_degrees_per_day`, `radial_speed_au_per_day`, `retrograde`, `native_planet_id` (=index), `swiss_body_id`, backend, flags, data files | `produce` → `bodies.append` :211 |
| frame `ecliptic-of-date`; time `UTC-as-UT argument; UT1 correction not supplied`; `julian_day_ut_argument`; Tropical, or Sidereal with LAHIRI only | :235-236 |
| houses **not emitted** (`house_policy: not-emitted; factory auxiliary Whole-Sign houses discarded`) | :237 |
| scope: `observer-private` if observer given, otherwise `shared-geocentric` | :238 |
| digest `snapshot_ref = sha256:<canonical JSON>`, plus provider/engine/factory/adapter sha256 and the ephemeris files actually used | :241, :224-232 |
| source binding to `vendor/epi-kernel/reference/include/m2.h` `PLANET_*` ids and `m2-retained-c-v1.json` planet table | `source_bindings` :117-138 |
| hand-off to M2 carries **only** `planet_id`, `longitude_degrees`, `provider_ref`, `source_revision`, `observed_at_unix_ms`; latitude, speed and retrograde are dropped | `attach_m2` :338-341; `crates/ql-mef/src/m2_engine.rs:80-86` `WorldObservation` |

**Observer.** `perspective ∈ {Apparent Geocentric, True Geocentric, Topocentric}` (:93). A Topocentric request needs `observer{reference, longitude, latitude (±66°), altitude}` (:102-111). A geocentric request must have **no** observer (:113: `shared geocentric field must not carry a private observer`). The provider does not compute sunrise or sunset: there is no `rise_trans` and no houses. **So no owner computes the planetary hour today**, and a shared (geocentric) scene cannot carry the location that the hour requires. Choices are in §5.

**Body → coordinate** (native id from `m2.h`; bindings from `m2-retained-c-v1.json` planet table; node ids from m-tree):

| sky body | native id | coordinate | m-tree node id | graph relations |
|---|---|---|---|---|
| Sun | 0 | `#2-5-0/1` | 11a998cf5731a05a | yes |
| Moon | 1 | `#2-5-4` | 2eab4240c71cd234 | yes |
| Mercury | 2 | `#2-5-3` | e080d692dfac892d | yes |
| Venus | 3 | `#2-5-2` | e81137576ba4dacc | yes |
| Mars | 4 | `#2-5-7` | 48b8b0e65ac0ba83 | yes |
| Jupiter | 5 | `#2-5-6` | 617286c55c852728 | yes |
| Saturn | 6 | `#2-5-5` | 6e19f8d6726f28e1 | yes |
| Uranus | 7 | **none** (binding `null`; ledger `k6-m2:difference-planet`) | — | — |
| Neptune | 8 | `#2-5-8` | 53c952ba0f40d36e | 4 straggler edges; **0 compiled** |
| Pluto | 9 | `#2-5-9` | 19afdde1ee1b532c | 6 straggler edges; **0 compiled** |
| (Earth) | not a body | `#2-5-0/1-0` | 93c883441603b1e5 | geocentric origin = observer centre (M2′ §9.5) |

Neptune and Pluto edges (`LD/strag-relations`, 9 unique): `#2-5 HAS_INTERNAL_COMPONENT` → Neptune and → Pluto; Mars `HARMONICALLY_LEADS_TO` Neptune; Neptune `HARMONICALLY_LEADS_TO` Pluto; Neptune `HAS_CHAKRAL_ANCHOR` Ājñā `-6`; Pluto `CLOSES_TO_GROUND` Earth `-0`; Pluto `HAS_CHAKRAL_ANCHOR` Sahasrāra `-7`; Pluto `SPIRALS_THROUGH_VORTEX` Mercury; Jupiter `SPIRALS_THROUGH_VORTEX` Pluto. m-tree has `files[parashakti-stragglers-relations.json]` with **0 records**. The nested `outgoing/incoming` shape was not parsed.

**Longitude → decan coordinate.** `crates/ql-mef/src/m2.rs:537` `situated_decan`: sign = ⌊λ/30⌋ and decan = ⌊(λ mod 30)/10⌋, giving `Reading72(Decan, sign%4, sign/4, decan, face)`. This equals the graph branch law `#2-3-(1+sign%4)-(sign/4)-(decan)`, with 1 Fire, 2 Earth, 3 Air, 4 Water (`PD/nodes :: #2-3-1 .elementalNature = "Fire - Tejas"` … `#2-3-4 = "Water - Apas"`). Geometry is correct. But `m2_engine.rs:501-509` then reads `catalogue.reading("decan", idx)`, which is the retained C decan table, so the **ruler it reports is the C value** (see D1).

---

## 2. Planet node properties

Property keys (`PD/nodes`). Sun `#2-5-0/1` (51 keys): `ancientOffering, angelicReception, archetypalConnection, chakralGenesis, collectiveFunction, contextFrame, coreNature, correspondenceDistinction, creativeFunction, description, divineEmanation, divineHumanConnection, dualNonDualFunction, harmonicRole, heatWisdomPrinciple, innerOffering, internalStructure, intervalFromRoot, kashimirBijaMantra, kashimirUnderstanding, lifeSustenance, logoicDesignation, mahamayaInterface, medievalSublimation, modalSignature, modernOffering, modernSymbolic, name, octavalExplanation, octavalNature, orphicHymn, personalFunction, planetaryMode, primaryDesignation, qlPosition, quantumOperatorFunction, receptionVariation, saivistMantra, scalarDegree, scaleArchetype, scaleFunction, solarStructuringPrinciples, spandaResonance, subsystem, swedenborgResonance, therapeuticRole, unityPrinciple, vedicMantra, vortexDynamics, …`. Venus through Mars have 32–38 keys each. They share `intervalFromRoot, scalarDegree, scaleArchetype, scaleFunction, planetaryMode, modalSignature, harmonicRole, qlPosition, contextFrame("2/3"), vedicMantra, kashimirBijaMantra, saivistMantra, keplerianVelocity (not Moon), orphicHymn, therapeuticRole, spandaResonance, vortexDynamics…`. Extras: Venus `epogdoonResonance, wholeToneSignificance, cosmosMusicalis, musicalBeauty, primeResonance, aestheticTranslation`; Moon `reflectiveDoubling`; Saturn `topologicalFunction`; Jupiter `primeHarmonicFunction`; Mars `cyclicCompletion`.
- **No planet node has any colour or element property** (0 node keys matching `colo*` across all 595 deep nodes). `chaldeanOrderVerified` is on `#2-5`, not on the planets.
- On `#2-5`: `.chaldeanOrderVerified = "Sun→Venus→Mercury→Moon→Saturn→Jupiter→Mars"`; `.musicalCosmology = "…Sun(1/8)=Alpha/Omega, Venus(2)=Beauty/Transformation, Mercury(3)=Mediation, Moon(4)=Reflection, Saturn(5)=Structure, Jupiter(6)=Expansion, Mars(7)=Catalyst"`; `.planetaryStructure = "6 planets + Sun(0/1) = 7 celestial bodies → 8-fold through Solar duality"`; `.harmonicSeriesResonance` names 2:1, 3:2, 4:3, 5:4, 6:5.
- `#2-5 HAS_INTERNAL_COMPONENT` → each planet carries `chaldeanRole`: Sun "Light-Consciousness Source", Venus "First Harmonizer", Mercury "Swift Messenger", Moon "Temporal Coordinator", Saturn "Deep Structural Anchor", Jupiter "Wise Elaborator", Mars "Cycle Completer".

| coord | name | .scalarDegree | .intervalFromRoot | .scaleFunction | .planetaryMode | .modalSignature (letter) | Cousto Hz (C only, `m2.h:328-337`) |
|---|---|---|---|---|---|---|---|
| `#2-5-0/1` | Sun | "1/8" | "Unison/Octave (1:1/2:1)" | Tonic | Dorian | "The D-D octave…" (D) | 126 |
| `#2-5-2` | Venus | 2 | "Major Second (9:8)" | Supertonic | Hypolydian | "The C-C octave…" (C) | 221 |
| `#2-5-3` | Mercury | 3 | "Major Third (5:4)" | Mediant | Hypophrygian | "The B-B octave…" (B) | 141 |
| `#2-5-4` | Moon | 4 | "Perfect Fourth (4:3)" | Subdominant | Hypodorian | "The A-A octave…" (A) | 210 |
| `#2-5-5` | Saturn | 5 | "Perfect Fifth (3:2)" | Dominant | Mixolydian | "The G-G octave…" (G) | 148 |
| `#2-5-6` | Jupiter | 6 | "Major Sixth (5:3)" | Submediant | Lydian | "The F-F octave…" (F) | 184 |
| `#2-5-7` | Mars | 7 | "Major Seventh (15:8)" | Leading Tone | Phrygian | "The E-E octave…" (E) | 145 |
| `#2-5-8` | Neptune (LD/strag-nodes) | 8 | "Minor Seventh (9:5)" | — | Locrian | — | 211 |
| `#2-5-9` | Pluto (LD/strag-nodes) | 9 | "Major Ninth (9:4)" | — | "Altered Dominant" | — | 140 |

**Planetary just octave.** It comes from `scalarDegree` + `intervalFromRoot` and is repeated verbatim on 36/36 decan `HARMONICALLY_RESONATES_WITH.harmonicInterval`: Sun 1:1 (0¢), Venus 9:8 (203.9¢), Mercury 5:4 (386.3¢), Moon 4:3 (498.0¢), Saturn 3:2 (702.0¢), Jupiter 5:3 (884.4¢), Mars 15:8 (1088.3¢), Sun 2:1 (1200¢). This is the Ptolemaic intense-diatonic major scale, and the digit is the scale degree. Neptune 9:5 and Pluto 9:4 exist only in stragglers.

**Computed observation:** the `modalSignature` letters run D C B A G F E, which is **descending** from the Sun's D. As downward intervals from D these give Venus 9:8, Moon 4:3, Saturn 3:2 and Jupiter 5:3, matching `intervalFromRoot`. Mercury (D↓B = 6:5, not 5:4) and Mars (D↓E = 16:9, not 15:8) do not match. So the letter reading and the ratio reading are two distinct readings that agree on 4 of 6. This matters because the maqam tonic letter selects the planet (§3).

Planetary hour from source: the Chaldean hour succession is the `chaldeanOrderVerified` cycle, i.e. the next hour's ruler digit is d+1 (Mars 7 → Sun 1). 24 hours = +3 mod 7, which yields the weekday rulers Sun→Moon→Mars→Mercury→Jupiter→Venus→Saturn. That matches `PLANETARY_RESONANCE.timingOptimal`: "Sun hour, Sunday…", "Venus hour, Friday…", "Mercury hour, Wednesday…", "Moon phases, Monday…", "Saturn hour, Saturday…", "Jupiter hour, Thursday…", "Mars hour, Tuesday…".

---

## 3. Relation propagation from a planet's ecliptic longitude

| step | relation (deep count) | property keys on relation | verified example | C / Rust mirror |
|---|---|---|---|---|
| λ → decan | geometry `#2-3-e-s-d .degreesRange` (36 nodes; `.planetaryRuler`, `.decanSystem="Chaldean"` 36/36) | — | `#2-3-1-0-0 .degreesRange="0°-10° Aries"`, `.planetaryRuler="Mars"` | `m2.rs:537 situated_decan` (correct) |
| decan → faces | `HAS_ASPECT` 72 (plus `HAS_DECAN` 72) | `qlContextFrame, qlCycle, qlDynamics, qlType, updatedAt` | `#2-3-1-0-0` → `-0` (Light "The Noble Pioneer"), `-1` (Shadow) | `M2_DECAN_DESC` face column |
| decan → ruler | `RULED_BY` 36 (+1 `RESONATES_WITH`, +11 sign-level `RULED_BY`/`TRADITIONALLY_RULED_BY` on `#2-3-e-s`) | `decanicExpression, harmonicTranslation, type="Chaldean Decan Rulership"` | `#2-3-1-0-0 RULED_BY #2-5-7` "Mars at 15:8 (leading tone)…" | `M2_DECAN_DESC.ruling_planet` **disagrees 27/36** (D1) |
| decan → ruler (music) | `HARMONICALLY_RESONATES_WITH` 36 | `harmonicInterval, modalConnection, type` | `#2-3-4-0-2` → Moon, "Perfect Fourth (4:3) - The reflective balance", "Hypodorian mode…" | none |
| decan → ruler (rhythm) | `SPANDA_TEMPORAL_RHYTHM` 36 | `polarityGeneration, temporalDynamic, type` | `#2-3-1-0-0` → Mars, "Spanda breakthrough rhythm" | none |
| ruler → decan | `QUANTUM_FIELD_OPERATOR` 36 (planet→decan) | `quantumDynamic, type` | Mars → `#2-3-1-0-0` | none |
| decan → M0 divine act | `SPECIFICALLY_MANIFESTS` 37 → `#0-3-10-*` (cross-M) | `planet, divineAct, temporalWindow, tarotCard, …` (11) | `#2-3-1-0-0` → `#0-3-10-2` planet "Mars", "Srishti - Creation" | none |
| λ → Shem name | geometry: `#2-4.5-c-p .zodiacalInfluence` 5° arcs (72 nodes, parsed 72/72) | — | `#2-4.5-0-2 Jeliel .zodiacalInfluence="5°-10° Aries - …"` | `M2_SHEM_DESC` (no arc column) |
| Shem → planet (sign ruler, with colour) | 8 kinds: `ENERGIZED_BY`(Mars) 16, `HARMONIZED_BY`(Venus) 15, `STRUCTURED_BY`(Saturn) 14, `EXPANDED_BY`(Jupiter) 13, `REFLECTED_BY`(Moon) 9, `COMMUNICATED_BY`(Mercury) 8, `ANALYZED_BY`(Mercury) 7, `ILLUMINATED_BY`(Sun) 8. That is 90 edges: 54 names ×1, 18 names ×2 (same planet, differing colour text) | `planetaryColor, planetaryInfluence, zodiacalRulership, transformativeAction, practicalEffect, <planet>Mode, <planet>Aspect` | `#2-4.5-0-0/1 ENERGIZED_BY #2-5-7 .planetaryColor="Brilliant red-gold martial flame"`, `.zodiacalRulership="Mars rules 0°-5° Aries"` | `M2_SHEM_DESC.planet_link` **disagrees 63/72** (D3) |
| | the planet is the **domicile ruler of the arc's sign**, 72/72 (Aries/Scorpio→Mars, Taurus/Libra→Venus, Gemini/Virgo→Mercury, Cancer→Moon, Leo→Sun, Sag/Pisces→Jupiter, Cap/Aqu→Saturn) | | | |
| Shem → decan face | `EXPRESSES_THROUGH` 72 (1 per name, 50 distinct faces) | `decanAspect (light 58 / shadow 14; = face digit 72/72), zodiacalResonance, elementalFlow, numericalHarmony, transformativeFunction, practicalApplication` | `#2-4.5-0-0/1` → `#2-3-1-0-0-0` light | `M2_SHEM_DESC.decan_link` = row index (matches 5/72) |
| Shem → element | `MANIFESTS_THROUGH` 72 → `#2-3-1..4` | `elementalPhase, journeyPosition, transformativeRole, practicalExpression, <el>Quality`, `elementalColor` (12), `visualization` (36) | `#2-4.5-0-0/1` → `#2-3-1` "Cardinal Fire Origin" | `M2_SHEM_DESC.element_id` (5-cycle; equals sign element 16/72) |
| maqam → planet | `TONIC_PLANETARY_RESONANCE` 62, `DOMINANT_PLANETARY_RESONANCE` 65. **127/127 targets equal the planet whose `modalSignature` letter is the natural `tonicNote`/`dominantNote`** (D Sun, C Venus, B Mercury, A Moon, G Saturn, F Jupiter, E Mars) | tonic: `tonic_relationship, modal_planetary_signature, chakra_planetary_correspondence, …` (10); dominant: `dominant_relationship, harmonic_tension_resolution, …` (10) | `#2-4.3-1-0 Rast TONIC → #2-5-2` "C tonic creates fundamental resonance with Venus…" | `m2_correspondence_data.inc` (127 rules); `M2_MAQAM_DESC.planet_ruler` is a family constant (D4) |
| maqam → planet (chromatic) | `TONIC_CHROMATIC_BRIDGE` 12 (3 maqamat × 2 planets × 2 duplicate edges); `DOMINANT_CHROMATIC_TENSION` 28 (7 × 2 × 2) | `chromatic_relationship, dual_planetary_influence, …` (8 each) | `#2-4.3-0-2 Awj Iraq` (tonic B♭) → Mercury + Moon, "B♭ creates harmonic bridge between Mercury and Moon…" | **none** (D10) |
| planet → chakra | `PLANETARY_RESONANCE` 7: Saturn→`-1`, Jupiter→`-2`, Mars→`-3`, Venus→`-4`, Mercury→`-5`, Moon→`-6`, Sun→`-7` | `modalContribution, timingOptimal, resonanceType, alchemicalCorrespondence, chakralInfluence, planetaryFunction, practicalApplication` | Jupiter→`#2-5-0/1-2` `.modalContribution="Lydian mode enabling complex nested creative structures…"`, `.timingOptimal="Jupiter hour, Thursday creative work…"` | used by `m2-correspondences-v1.json`; `M2_PLANET_LUT.elem_sig` **disagrees 3/7** (D2) |
| Sun → chakras | `CHAKRAL_VIRTUE_RECEPTION` 7 (→ all seven; properties only on the `-1` edge); also `DIVINE_NAMES_CHAKRAL_RECEPTION` 4, `GENDER_INCLUSIVE_SOLAR_PRACTICE` 5 | `relationshipType="Graduated Virtue-Solar Reception Correspondence"`, per-chakra virtue keys (`rootVirtueGrounding`, `solarGoodnessPower`, …) | Sun→Mūlādhāra `.rootVirtueGrounding` | none |
| Sun → Earth | `HAS_INTERNAL_COMPONENT`, `GENERATES_TERRESTRIAL_GROUNDING` (1 each, no properties) | — | `#2-5-0/1` → `#2-5-0/1-0` | `M2_CHAKRA_LUT[0]` "CHAKRA_EARTH" |
| Earth → Mūlādhāra | `FEEDS_EARTH_ELEMENT` 1, `GROUNDS_CHAKRAL_PATHWAY` 1 | `elementalTransfer="Primary earth element (Pṛthivī) sourcing", schumannResonance="7.83 Hz…", telluricCurrents, …` (7) | — | none |
| chakra ladder | `ASCENDS_TO` 6: `-1→-2→…→-7` | none | — | none |
| chakra → element | node property `.elementalCorrespondence` | — | `-1` "Pṛthivī (Earth)…", `-2` Āpas, `-3` Agni, `-4` Vāyu, `-5` Ākāśa, `-6`/`-7` "Beyond elements" | `M2_CHAKRA_LUT.element_id` **agrees 7/7** (`vendor/…/m2.c:302-311`) |
| other planet↔planet | `QUANTUM_HARMONIC_RESONANCE` (Sun→Venus, Venus→Mercury), `HARMONIC_PROGRESSION` Venus→Mercury, `TEMPORAL_PHASE_COUPLING` Moon→Saturn, `PRIME_HARMONIC_BRIDGE` Saturn↔Jupiter, `ARCHETYPAL_QUANTUM_TRIAD` Mercury→Saturn→Mars→Mercury, `DOMINANT_RESOLUTION`, `TOPOLOGICAL_BREAKTHROUGH` Saturn→Mars, `LUNAR_FEMININE_DIVINE_INTEGRATION` Moon→Sun, Ājñā | `interval`, … | — | none |
| planet → element | `EXPRESSES_THROUGH` Sun→`#2-3-1`, Mars→`#2-3-1` (2 only) | — | — | none |

---

## 4. Gate 1 defects (verified)

C row lines: `c/src/m2_data.inc` decan rows :190-261 (row i at :190+i), planet :265-274, shem :287-358, maqam :373-444. The generated rows mirror `vendor/epi-kernel/reference/src/m2.c` (`M2_DECAN_DESC` :164, `M2_PLANET_LUT` :264, `M2_SHEM_DESC` :321, `M2_MAQAM_DESC` :430). Ledger `fixtures/kernel/m-ledger-v1.json` has 588 discrepancies. The only existing M2 ones are `k6-m2:*` (12). None of D1–D10 is recorded, except that the Uranus absence (`k6-m2:difference-planet`) and the "17 missing paths" (`k6-m2:difference-colour-source`) are partly stated.

**D1 — Decan ruler.** Graph: `.planetaryRuler` equals the canonical Chaldean face sequence (Mars at 0° Aries, then Chaldean order) for **36/36**. `HARMONICALLY_RESONATES_WITH`, `SPANDA_TEMPORAL_RHYTHM` and `QUANTUM_FIELD_OPERATOR` equal it 36/36. `RULED_BY` equals it 35/36. The C table (triplicity-style, `vendor m2.c:166-228…`) **disagrees 27/36**: Ari3 C Jupiter/graph Venus (:194); Leo1 Sun/Saturn (:196); Sag1 Jupiter/Mercury (:202); Sag2 Mars/Moon (:204); Sag3 Sun/Saturn (:206); Tau1 Venus/Mercury (:208); Tau2 Mercury/Moon (:210); Vir1 Mercury/Sun (:214); Vir2 Saturn/Venus (:216); Vir3 Venus/Mercury (:218); Cap1 Saturn/Jupiter (:220); Cap2 Venus/Mars (:222); Cap3 Mercury/Sun (:224); Gem1 Mercury/Jupiter (:226); Gem2 Venus/Mars (:228); Gem3 Saturn/Sun (:230); Lib1 Venus/Moon (:232); Lib3 Mercury/Jupiter (:236); Aqu1 Saturn/Venus (:238); Aqu3 Venus/Moon (:242); Can1 Moon/Venus (:244); Can2 Mars/Mercury (:246); Can3 Jupiter/Moon (:248); Sco2 Jupiter/Sun (:252); Sco3 Moon/Venus (:254); Pis1 Jupiter/Saturn (:256); Pis2 Moon/Jupiter (:258). Agreeing (9): Ari1, Ari2, Leo2, Leo3, Tau3, Lib2, Aqu2, Sco1, Pis3. Live effect: `m2_engine.rs:501-509` `light_decan`/`shadow_decan` report the C ruler.

**D2 — `M2_PLANET_LUT.elem_sig` planet→chakra** (bits 5:3; `m2.h:94`). Graph `PLANETARY_RESONANCE` against C (`m2_data.inc` / `vendor m2.c`):

| planet | graph | C elem_sig chakra | |
|---|---|---|---|
| Sun | `-7` Sahasrāra | 0 CHAKRA_EARTH (130; :265 / :266) | **differs** |
| Moon | `-6` Ājñā | 2 Svādhiṣṭhāna (83; :266 / :269) | **differs** |
| Mercury | `-5` | 5 | agrees |
| Venus | `-4` | 4 | agrees |
| Mars | `-3` | 3 | agrees |
| Jupiter | `-2` Svādhiṣṭhāna | 3 Maṇipūra (90; :270 / :281) | **differs** |
| Saturn | `-1` | 1 | agrees |
| Neptune | `-6` via `HAS_CHAKRAL_ANCHOR` (straggler, uncompiled) | 7 (251; :273 / :290) | differs |
| Pluto | `-7` via `HAS_CHAKRAL_ANCHOR` (straggler, uncompiled) | 1 (204; :274 / :293) | differs |
| Uranus | no node | 6 (240) | unbound |

The same kernel is therefore inconsistent with itself: `m2-correspondences-v1.json` (graph path) puts Jupiter→Svādhiṣṭhāna, while `elem_sig` puts Jupiter→Maṇipūra. M2′ spec §8.5 reproduces the C column ("Sun … EARTH-root", "Moon … SVADHISTHANA", "Jupiter … MANIPURA").

**D3 — `M2_SHEM_DESC.planet_link`.** Graph: the domicile ruler of the name's 5° arc sign (72/72). C `planet_link` agrees on **9/72** and disagrees on **63/72**. The C value is `M2_DECAN_DESC[shem_idx].ruling_planet`, i.e. `decan_link = shem_idx` (identity), so the planet is the C face ruler of an element-grouped row, not a zodiac position. Examples: Jeliel `#2-4.5-0-2` (5-10 Aries) graph Mars / C Sun (:288); Achaiah `#2-4.5-0-7` graph Venus / C Jupiter (:293); Mumiah `#2-4.5-7-9` graph Jupiter / C Moon (:358). The full 63-row list reproduces from `shem.py` logic: compare `ret.tables.shem.rows[i][5]` against Shem→planet edges. Also: `decan_link` equals `EXPRESSES_THROUGH` for 5/72, and `element_id` (5-element cycle) equals the sign element for 16/72. Spec §8.2 describes exactly this synthetic layout ("decans 0-8", "Element-Cycle Agni/Prithvi/…").

**D4 — Maqam planet ruler.** C `planet_ruler` is constant per family (`vendor m2.c:430+`; M2′ §8.1): 0 Sun, 1 Venus, 2 Moon, 3 Mercury, 4 Saturn, 5 Jupiter, 6 Venus, 7 Mars, 8 Neptune, 9 Pluto. Graph has **no family→planet relation** (families `#2-4.3-f` carry only `HAS_INTERNAL_COMPONENT` 72 and `CEREMONIAL_FLOW` 8). Per mode, C ruler = graph tonic 12/72, = graph dominant 17/72, and is in {tonic, dominant} 29/72, so **43/72 have a C ruler that is neither**. Examples: Awj Iraq `#2-4.3-0-2` C Sun, graph dominant Jupiter plus tonic-bridge Mercury/Moon (:374); Nahawand `#2-4.3-5-0` C Jupiter, graph tonic Venus / dominant Saturn (:410); Saba `#2-4.3-8-0/1` C Neptune, graph tonic Sun, dominant-tension Saturn/Jupiter (:431). This is a two-concept conflict (family ruler, authored in spec, vs mode tonic/dominant, in graph), not only a C error.

**D5 — Cancer decan 3 `#2-3-4-0-2`** (node e1652e9a21103e71). `.planetaryRuler="Moon"` (canonical Chaldean). `RULED_BY → #2-5-5` Saturn (`type="Chaldean Decan Rulership"`, "Saturnian depth through cardinal water trials"; relation ids 47a5107fa25e1d9a, bd950222e7fbcdf1). `RESONATES_WITH → #2-5-4` Moon ("Lunar Resonance"; 37f40fc6cd580882, 6b06ab7823cbcb55). HRW, STR and QFO are Moon. `SPECIFICALLY_MANIFESTS` goes to both `#0-3-10-4` (planet "Saturn") and `#0-3-10-7` ("Moon"). `.decanicStructure` on `#2-3-4-0` = "…Moon-ruled deep nurturing (20°-30°)". C `:248` = Jupiter. Three-way conflict; source weight is 5 Moon vs 2 Saturn.

**D6 — Shem `EXPRESSES_THROUGH` vs 5° geometry.** **26/72** point into a different decan of the right sign (0 wrong-sign). All 26 are the arcs 5-10°, 10-15° or 15-20°, shifted one decan later: Jeliel, Sitael, Elemiah, Cahetel, Aladiah, Pahaliah, Nelchael, Yeiayel, Vasariah, Yehuiah, Lehahiah, Haamiah, Rehael, Yeiazel, Yelahiah, Sealiah, Ariel, Daniel, Imamiah, Poiel, Nemamiah, Yeialel, Iahhel, Mehiel, Habuhiah, Jabamiah. Names per decan via the relation are {1:16 decans, 2:10, 3:4, 4:6}. Geometry is 2 per decan. The face is set by `decanAspect` (light 58 / shadow 14), not by the half-decan, which contradicts M2′ §8.2 "2 names per decan… light/shadow doubled-face".

**D7 (new) — Shem `MANIFESTS_THROUGH` element vs sign element.** **7/72** wrong: Yeiazel, Hahahel and Mikael (Libra → Water `#2-3-4`); Mihael (Scorpio → Fire `#2-3-1`); Yeialel, Harahel and Mitzrael (Capricorn → Air `#2-3-3`).

**D8 (new; contradicts audit) — Straggler relations uncompiled.** 9 unique Neptune/Pluto edges in `LD/strag-relations` become 0 m-tree records (§1).

**D9 — Chromatic relations ignored by `m2-correspondences-v1.json`.** 127 rules, 17 gaps. **10/17 gaps have a source chromatic path**: tonic via `TONIC_CHROMATIC_BRIDGE` `0-2` (Mercury+Moon), `8-7` (Sun+Mars), `8-9` (Mercury+Moon); dominant via `DOMINANT_CHROMATIC_TENSION` `0-5` (Mercury+Moon), `2-3` (Moon+Saturn), `7-6`, `8-0/1`, `8-2`, `8-5`, `8-6` (Saturn+Jupiter). **7 are true source gaps**: every E♭↓ tonic (`0-5` Sikah Baladi, `3-0/1` Sikah, `3-2` Huzam, `3-3` Iraq, `3-4` Rahat al-Arwah, `3-5` Awj Ara, `8-8` Segah). Script `scripts/m2-correspondences.py:21` `ROLES` = the two natural kinds only. `k6-m2:difference-colour-source` says "17 missing paths" and must be corrected to 7 + 10 chromatic.

**D10 (minor source hygiene).** Rast's `DOMINANT_PLANETARY_RESONANCE` properties name "Segah" (`.modal_planetary_development="…through Segah dominant resolution"`), a copy artifact. Every chromatic edge is duplicated with the planet order swapped.

Proposed ledger entries (shape of `m-ledger-v1.json.discrepancies[]`; **not written**):

```json
[
 {"id":"g0-m2:decan-ruler","subjects":["#2-3"],"axis":"relation","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"27/36 M2_DECAN_DESC.ruling_planet (c/src/m2_data.inc:190-261; vendor m2.c:164) differ from the Chaldean faces carried 36/36 by #2-3-e-s-d .planetaryRuler, HARMONICALLY_RESONATES_WITH, SPANDA_TEMPORAL_RHYTHM, QUANTUM_FIELD_OPERATOR and 35/36 RULED_BY (decanSystem=Chaldean). m2_engine light/shadow_decan reports the C ruler.",
  "current_authority":{"peer":"bimba","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md","reason":"Graph typed relations and node property agree; the retained C table is a triplicity reading with no source relation."},
  "proposal":{"target_peer":"c","change":"Regenerate decan ruling_planet from #2-3-e-s-d .planetaryRuler/RULED_BY via scripts/m2-catalogue.py; retain the triplicity column only as a named alternate reading if the owner asks.","evidence":[]},
  "decision":null,"promotion":null,"history":[{"state":"open","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md"}]},
 {"id":"g0-m2:planet-chakra","subjects":["#2-5","#2-5-0/1","#2-5-4","#2-5-6","#2-5-8","#2-5-9"],"axis":"relation","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"M2_PLANET_LUT.elem_sig chakra bits: Sun 0(EARTH)/graph -7, Moon 2/graph -6, Jupiter 3/graph -2 (PLANETARY_RESONANCE); Neptune 7/straggler -6, Pluto 1/straggler -7 (HAS_CHAKRAL_ANCHOR). m2-correspondences-v1 already follows the graph, so the kernel disagrees with itself. M2' spec §8.5 reproduces the C column.",
  "current_authority":{"peer":"bimba","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md","reason":"Typed relation with modalContribution/timingOptimal; owner ruling needed because M2' §9.5 calls M2_PLANET_LUT canon."},
  "proposal":{"target_peer":"c","change":"Derive elem_sig chakra from PLANETARY_RESONANCE (and compiled HAS_CHAKRAL_ANCHOR for outer planets); element bits from the chakra's elementalCorrespondence.","evidence":[]},
  "decision":null,"promotion":null,"history":[{"state":"open","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md"}]},
 {"id":"g0-m2:shem-links","subjects":["#2-4.5"],"axis":"relation","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"M2_SHEM_DESC.decan_link=shem_idx (matches EXPRESSES_THROUGH 5/72), planet_link=C decan ruler of that row (matches sign-domicile Shem->planet relations 9/72), element_id=5-cycle (matches sign element 16/72). Source: .zodiacalInfluence 5-degree arcs 72/72 parse; 8 planet relation kinds with planetaryColor.",
  "current_authority":{"peer":"bimba","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md","reason":"Synthetic cycle vs explicit arcs and typed relations."},
  "proposal":{"target_peer":"c","change":"Add arc_start (5-degree index) column; planet_link from Shem->planet relation; decan_link from arc geometry, with EXPRESSES_THROUGH retained as a separate source relation (see g0-m2:shem-expresses).","evidence":[]},
  "decision":null,"promotion":null,"history":[{"state":"open","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md"}]},
 {"id":"g0-m2:maqam-ruler","subjects":["#2-4.3"],"axis":"relation","from_peer":"c","to_peer":"bimba","state":"open",
  "detail":"M2_MAQAM_DESC.planet_ruler is a family constant (M2' §8.1) with no graph relation; it equals graph tonic 12/72, dominant 17/72, neither 43/72. Graph carries per-mode TONIC/DOMINANT_PLANETARY_RESONANCE (127, all by modalSignature letter) plus chromatic bridges.",
  "current_authority":{"peer":"bimba","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md","reason":"Two concepts (family ruler vs mode tonic/dominant); owner ruling on which feeds F_routing."},
  "proposal":{"target_peer":"bimba","change":"Either author a family->planet relation for the §8.1 table (then C mirrors it) or drop planet_ruler and route by mode tonic/dominant; see m2-sky §5 A/B/C.","evidence":[]},
  "decision":null,"promotion":null,"history":[{"state":"open","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md"}]},
 {"id":"g0-m2:cancer-3-ruler","subjects":["#2-3-4-0-2"],"axis":"source","from_peer":"bimba","to_peer":"bimba","state":"open",
  "detail":".planetaryRuler=Moon, RESONATES_WITH/HARMONICALLY_RESONATES_WITH/SPANDA_TEMPORAL_RHYTHM/QUANTUM_FIELD_OPERATOR=Moon, but RULED_BY=Saturn (47a5107fa25e1d9a, bd950222e7fbcdf1); SPECIFICALLY_MANIFESTS to both; C=Jupiter (m2_data.inc:248).",
  "current_authority":{"peer":"bimba","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md","reason":"Internal source contradiction; canonical Chaldean sequence gives Moon."},
  "proposal":{"target_peer":"bimba","change":"Owner/graph correction: retarget RULED_BY to #2-5-4 (or retype the Saturn edge); C follows.","evidence":[]},
  "decision":null,"promotion":null,"history":[{"state":"open","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md"}]},
 {"id":"g0-m2:shem-expresses","subjects":["#2-4.5"],"axis":"relation","from_peer":"bimba","to_peer":"bimba","state":"open",
  "detail":"26/72 EXPRESSES_THROUGH target the next decan of the correct sign (all 5-20 degree arcs); 7/72 MANIFESTS_THROUGH target the wrong element (Libra x3->Water, Scorpio x1->Fire, Capricorn x3->Air). Faces chosen by decanAspect (58 light/14 shadow), contradicting M2' §8.2 '2 names per decan = light/shadow'.",
  "current_authority":{"peer":"bimba","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md","reason":"Arc property vs relation disagree inside source."},
  "proposal":{"target_peer":"bimba","change":"Owner ruling: geometry wins (retarget 26 + 7) or relation wins (arc is not the join). Kernel uses arc geometry until ruled.","evidence":[]},
  "decision":null,"promotion":null,"history":[{"state":"open","reference":"docs/kernel-rebuild/M123-GROUNDING-AUDIT-2026-09-27.md"}]},
 {"id":"g0-m2:straggler-relations","subjects":["#2-5-8","#2-5-9"],"axis":"relation","from_peer":"bimba","to_peer":"c","state":"open",
  "detail":"low-detail/parashakti-stragglers-relations.json (sha 17622c0b...) holds 9 unique edges (HAS_INTERNAL_COMPONENT x2, HARMONICALLY_LEADS_TO Mars->Neptune->Pluto, HAS_CHAKRAL_ANCHOR Neptune->-6 / Pluto->-7, CLOSES_TO_GROUND Pluto->Earth, SPIRALS_THROUGH_VORTEX Jupiter->Pluto->Mercury); m-tree compiled 0 records from the file (nested outgoing/incoming shape).",
  "current_authority":{"peer":"bimba","reference":"docs/KERNEL-RECURSIVE-M-REGISTRY.md","reason":"Registry must retain every pinned source relation."},
  "proposal":{"target_peer":"c","change":"Teach the K2 compiler the straggler relation shape; regenerate m-tree; correct the audit line 'no relations'.","evidence":[]},
  "decision":null,"promotion":null,"history":[{"state":"open","reference":"docs/KERNEL-RECURSIVE-M-REGISTRY.md"}]},
 {"id":"g0-m2:chromatic-paths","subjects":["#2-4.3"],"axis":"relation","from_peer":"bimba","to_peer":"rust","state":"open",
  "detail":"m2-correspondences-v1 reads only TONIC/DOMINANT_PLANETARY_RESONANCE; 10 of its 17 gaps have TONIC_CHROMATIC_BRIDGE (3) or DOMINANT_CHROMATIC_TENSION (7) paths to two planets; 7 E-flat-half-flat tonics have no source path. Supersedes the '17 missing paths' wording in k6-m2:difference-colour-source.",
  "current_authority":{"peer":"bimba","reference":"docs/kernel-rebuild/m2-engine-v1.md","reason":"Source relations exist and are unread."},
  "proposal":{"target_peer":"rust","change":"Extend ROLES in scripts/m2-correspondences.py with the two chromatic kinds as dual-planet rules (standing 'chromatic'); keep the 7 true gaps explicit.","evidence":[]},
  "decision":null,"promotion":null,"history":[{"state":"open","reference":"docs/kernel-rebuild/m2-engine-v1.md"}]}
]
```

---

## 5. M2's own sounding

**Source-defined:**
- The planetary just octave (§2). The hour-ruler cycle and weekday rulers derive from `#2-5.chaldeanOrderVerified` + `PLANETARY_RESONANCE.timingOptimal` (§2).
- Maqam tonic/dominant → planet by letter (127). Chromatic → two planets (40 edges).
- Each maqam's `intervalStructure` (spelled; 115/127 rules have `spelled_steps24`) and C 24-TET `M2_MAQAM_DESC` intervals (ledger `k6-m2:difference-maqam`: not asserted identical).
- `planetaryMode` per planet. `.timeAssociation` exists on 72 maqamat but as prose ("High noon brilliance"), not an hour index.

**Owner spec (Epi-Logos-C-Experiments `Idea/Bimba/Seeds/M/M2'/M2'-SPEC.md`, status active-domain-spec, updated 2026-07-18).** This spec is **not in QL `docs/`**.
- §9.2 `F_routing`: `current_hour [from current_time + kerykeion sunrise/sunset]` → `planetary_hour_ruler [Chaldean order: Sun-Venus-Mercury-Moon-Saturn-Jupiter-Mars cycling]` → `ruling_planet [from M2_PLANET_LUT]` → `active_decan [planets[planet].degree / 10°]` → `shem_pair [from decan_link, 2 names per decan]` → `maqam_family [family ruled by planet, from §8.1 table]` → `maqam_mode [selected by intent + current lens-mode state]` → mantra, asma → DET → M3 → `audio_octet → M1' walk-melody`, `nodal_quartet → M2' Chladni`.
- §6: M2-1′ Vimarśā writes `audio_octet[8]`/`nodal_quartet[4]`; M2′ renderers never synthesise locally; "Scale/mode colour comes from `planetaryChakral` and `diatonic`".
- §9.8: whether maqam/mantra frequencies "become an active synthesis target is decided by the shared audio profile contract". **Undetermined in source.**

**Producers today:**
- `c/src/m2.c:158 ql_m2_vimarsha` = `crates/ql-mef/src/m2_vimarsha.rs read_seed`. This is a 12-TET major-scale formula (`INTERVALS {0,2,4,5,7,9,11}`, base 130.81279 Hz), scaled by M1's `harmonic_ratio`. `musical_mode` 0..6 comes from **M1's** mode (`continuous/coupled.rs:270-280`), not from the planet. The planetary just octave therefore has **no producer**.
- Maqam pitch: `c/src/m2.c:121 ql_m2_maqam_pitch` (C 24-TET) and `:197 ql_m2_correspondence_pitch` (tuning 0 C / 1 Bimba spelling). These are reached via the `m2_condition.rs` `M2ConditionInput{maqam_index, role, tuning, tonic_hz}` (:268-275), where maqam_index and tonic_hz are **supplied, not routed from sky/hour**.
- `coupled.rs:281-335` binds the octet (`FrequencyBinding`) and the condition pitches (`ConditionFrequencyBinding`) onto supplied resonator modes.
- C++ `cpp/src/field_worker.cpp:55 modes()` → `cpp/include/ql/continuous_field.hpp` `ContinuousField::render_audio` renders whatever `frequency_hz` arrives. C++ holds no M2 table.

**Undetermined: concrete alternatives.**
- **Hour basis.**
  - (A) Observer-private topocentric request plus sunrise/sunset added to the sky provider (`swe.rise_trans`). The scene's hour is then private (scope `observer-private`, which `attach_m2` refuses to put into a shared event).
  - (B) An owner-declared *public place of the world* as a named shared observer. This needs a contract change to :113.
  - (C) Weekday ruler only (no observer; timezone from the request). Changes 24× per day are lost.
  - (D) Equal civil hours from local midnight. This is not the source (spec says sunrise/sunset).
- **Ruler's set.**
  - (A) Spec §8.1 family table (C `planet_ruler`). No graph relation (D4). Neptune/Pluto families (14 modes) are unreachable from the 7 hour rulers; Mercury's set is the Sikah family, whose E♭↓ tonics have no graph planet.
  - (B) Modes whose graph tonic is the ruler. Sets: Sun 29, Venus 20, Saturn 7, Mars 3, Jupiter 2, Moon 1, Mercury 0 (only 2 via chromatic bridge).
  - (C) Tonic ∪ dominant ∪ chromatic. Sets: Sun 37, Venus 22, Mercury 6, Moon 15, Saturn 53, Jupiter 9, Mars 5; every classical ruler is reachable.
- **Mode within set ("intent + current lens-mode").**
  - (A) Current MEF condition index (`active_mef_condition`) mod set size. Deterministic, but has no source ordering.
  - (B) Match `planetaryMode` of the ruler to the M1 musical mode. Needs a mode-name ↔ index binding not in source.
  - (C) Owner-supplied intent picks; lens-mode breaks ties.
- **Join with the Vimarśā octet (§9.8).**
  - (A) Separate bus: planetary octave and maqam as `ConditionFrequencyBinding` onto distinct modes (already mechanically supported; disjoint bindings).
  - (B) Octet retuned: replace 12-TET `INTERVALS` by the planetary just ratios with the ruler as tonic. This changes a retained C/Rust formula and needs a ledger entry.
  - (C) Octet unchanged. M2 sounds only via tonic_hz = ruler's Cousto Hz (C `M2_PLANET_LUT.cousto_freq`, no graph source) or via ratio × a declared base.
- **Pitch realisation of a planet.**
  - (A) `intervalFromRoot` ratio above the Sun tonic.
  - (B) `modalSignature` letter pitch (coherent with maqam letters but differs for Mercury and Mars, §2).

---

## 6. Colour

| source colour | where | count | literal example | carried by |
|---|---|---|---|---|
| `planetaryColor` on Shem→planet relations (8 kinds) | `PD/rels` | 90 edges / 72 names | `#2-4.5-2-9 ILLUMINATED_BY #2-5-0/1 .planetaryColor="Royal purple solar authority"` | **no owner** (not in `m2-correspondences`, `m2_condition.rs`, C) |
| `elementalColor` on Shem `MANIFESTS_THROUGH` | `PD/rels` | 12 (names `4-0/1…4-6`, `5-3…5-5`, `6-0/1…6-3`) | `#2-4.5-4-2 → #2-3-3 "Clear blue protective field maintaining truth"` | none |
| `colorEvolution` on choir `BEGINS_IN`/`TRANSITIONS_TO` | `PD/rels` | 9 + 6 | `#2-4.5-0 BEGINS_IN #2-3-1 "White-gold → Rose-gold → Golden → Blue-violet → …"` | none |
| `colorSpectrum` on element self-edges `TEACHES_CONSCIOUSNESS_EVOLUTION` | `PD/rels` | 4 | `#2-3-1 "Full spectrum from white-gold origin through indigo vision to royal purple servi…"` | none |
| chakra `.yantraForm` colour words | `PD/nodes` | 3/7 | `-1` "yellow square", `-2` "silver crescent moon", `-3` "red triangle"; `-4…-7` no colour | `scripts/m2-correspondences.py` phrase admission → `colour_name` → `m2_condition.rs:396-409` (else `"unavailable-no-explicit-source-colour"`; RGBA only from a supplied `RenderPalette`) |
| planet / element / decan node colour properties | `PD/nodes` | **0** | — | — |
| `ELEMENT_COLOURS` (Mahābhūta → hex: Akasha 0x8f6fd8 violet, Vayu 0x59c2cf cyan, Agni 0xd8613c vermilion, Apas 0x63c9a9 aquamarine, Prithvi 0x9b7a4b umber) | C-Experiments `Body/M/pratibimba-app/src/engine/cosmicMath.ts:221` | 5 | presentation palette, not Bimba | pratibimba-app only |
| `Idea/Bimba/Map/datasets/m2-element-registers.json` | C-Experiments commit `d8e917ed` (2026-07-25, DR-L2-ELEM-2); **not an ancestor of pinned `daa660c`**; absent from the pinned tree | registers: mahābhūta `#2-2` (5), alchemical `#2-1` (6), triplicity-branch `#2-3` (6) + correspondences + two body ontologies | **contains no colour values** (only element identity and the `#2-3-N = 1 + sign mod 4` branch law) | not compiled; would need a registry re-pin |

So "colour unavailable" is false at the source level: every Shem arc (hence every 5° of the sky) carries ≥1 literal `planetaryColor` bound to its planet. Owner choice for the scene colour source:
- (A) Shem arc under the planet's longitude → `planetaryColor` literal (sky-driven, 72 arcs; 18 arcs have two literals, so a tie rule is needed).
- (B) Planet → chakra (`PLANETARY_RESONANCE`) → yantra colour. Covers only Saturn, Jupiter and Mars (3/7).
- (C) Element register → `ELEMENT_COLOURS` palette. This is a presentation policy (`RenderPalette`), not source.

Any of these must go through `RenderPalette` for RGBA. No literal-to-RGB conversion exists in any owner.
