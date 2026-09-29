# Nara identity composition: recovered rules and decisions

**Standing: draft-for-review; generated proposal, not adopted law.** Prepared 2026-09-27 for [QL-MEF #201](https://github.com/EpiLogos/QL-MEF/issues/201#issuecomment-5857484170). Code references describe the `feat/nara-identity-expression-20260927` working tree based on `56921a5`; line numbers are an inspection aid, not immutable source identity. This note does not change the matrix, source contracts or receiver policy.

## Fixed source and accepted boundary

| Determination | Source and consequence |
| --- | --- |
| Elemental carrier | [M4 matrix](origami%20work/M4/M4-NARA-DEEP-CAPABILITY-COORDINATE-MATRIX.md), lines 115, 374–384, 1107–1126: quaternion components are **[Earth, Fire, Water, Air]**; C/Sensation/Earth, T/U/Intuition/Fire, A/Feeling/Water, G/Thinking/Air. Older alternate orders are drift. |
| Distinct roles | Matrix lines 126–127, 1135–1138 separates exact identity/provenance hash, M3 form address, orientation and lived composition, while leaving final identity compression open. A hash must not become a quaternion or inferred archetype. |
| Composition order | `crates/ql-mef/src/nara.rs:80,246` implements normalized Hamilton multiplication **Q_identity · Q_transit · Q_activity** in that order. Keep the three source revisions distinct. |
| Seven centres and Earth | [#134 contract](kernel-rebuild/K10-NARA-M4-CONTRACT.md), lines 9–26, 50–54: continue accepted K8 reception with seven independently sourced constitutions and distinct EarthBody; QL owns domain truth, O:I owns Expression, Central owns personal source. |
| Native relations | The pinned Bimba graph's `PLANETARY_RESONANCE` relations determine planet→chakra membership, as clarified by the [owner's current correction](https://github.com/EpiLogos/QL-MEF/issues/201#issuecomment-5859688868). `ql_m2_planet_chakra_route` in `c/src/m2.c` and `m2::planet_chakra_route` in Rust traverse that same compiled graph. The frozen `vendor/epi-kernel/reference/src/m2.c` `elem_sig` values are preserved as historical evidence; they conflict with the graph and do not determine current reception. |

| Receiver ordinal / native chakra ID | Centre | Native planetary evidence |
| --- | --- | --- |
| 0 / 1 | Muladhara | Saturn |
| 1 / 2 | Svadhisthana | Jupiter |
| 2 / 3 | Manipura | Mars |
| 3 / 4 | Anahata | Venus |
| 4 / 5 | Vishuddha | Mercury |
| 5 / 6 | Ajna | Moon |
| 6 / 7 | Sahasrara | Sun |

Sun retains its solar-parent role and its explicit Sahasrara `PLANETARY_RESONANCE` route. Its separate `CHAKRAL_VIRTUE_RECEPTION` relations to all seven centres are not interchangeable with that route. Earth is not an eleventh planet or eighth centre. Uranus, Neptune and Pluto remain in the natal calculation, but have no admitted `PLANETARY_RESONANCE` route in this graph revision; their frozen C assignments are not substituted. The native chakra descriptors retain Muladhara→Prithvi/35, Svadhisthana→Apas/34, Manipura→Agni/33, Anahata→Vayu/32, Vishuddha→Akasha/31, and Ajna/Sahasrara beyond-element markers. Body-zone, sense and action references retain their own canonical source relations.

The route resolves `#2-5-0/1-1` through `#2-5-0/1-7` from the existing `fixtures/kernel/m-tree-v1.json` graph, preserving both source assertions for each edge. The deep relation records are `3986` (Saturn), `4004` (Jupiter), `4016` (Mars), `3942` (Venus), `3959` (Mercury), `3970` (Moon), and `3856` (Sun) in `Idea/Bimba/Map/datasets/parashakti-deep/relations.json` at the pinned `daa660c` revision. Equivalent low-detail assertions keep their distinct relation IDs. The intake packet exposes `planetary_chakra_route` with these identities and revisions; `retained_m2_chakra_id` names the historical descriptor separately. Zero-based receiver ordinal remains a storage address alongside the canonical chakra ID and coordinate.

The existing receive implementation evaluates **d_c = Σᵢ w_ci x_ci; a_c = |〈normalize(Q_composed), normalize(o_c)〉|; r_c = d_c g_c a_c; out_c = r_c h_c** (`nara.rs`, `PersonalFieldInstance::receive`). Its caller-supplied M1/M2/M3 triple is an implementation boundary, not canonical chakra routing. The owner's current correction rejects treating that triple as the source law; the graph-backed planetary routes above must supply the personal instrument's actual receiving relationships. Retain the accepted lifecycle/event/currentness protections while repairing this source mismatch.

## Recovered natal policy: available, explicitly natal-only

The retained `nara.personal-receive` Epi agent operation still
accepts a caller's complete `PersonalConstitution` and `PersonalEventInput`.
Its result now explicitly marks `canonical_receiver_semantics: false` and
`commissioned_personal_expression_acceptance: false`, with the
[owner's correction](https://github.com/EpiLogos/QL-MEF/issues/201#issuecomment-5859688868)
as standing provenance. `tests/nara_personal_field.rs` remains a controlled
isolation/currentness regression; its supplied quaternion layers, receiver
triples and coefficients do not constitute source-derived personal physics.
The current intake, graph-routed natal/decan evidence and reversible Expression
presentation path do not consume that caller-supplied receiver fixture. Retain
the generic typed primitive without promoting it back into operative meaning.

Source: EpiLogos/Epi-Logos-C-Experiments, commit `daa660cbc1b8c5da83828698665a753852cb0287`, blob `0496c08d8cab700afaca78a493d97a07c774b332`, `Body/S/S0/portal-core/src/personal_identity.rs:86–135,230–302`. The present port pins this identity in `crates/ql-mef/src/nara/intake_composition.rs:11–15,235–239` under `ql.nara-natal-keplerian-dignity-efwa/v1`.

For each actual natal longitude λ in [0,360), sign=floor(λ/30); use its zodiac element, not the planet's native elemental descriptor. Contribution is **retained M2 keplerian_weight × dignity × EFWA unit vector**. Native ID order is Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune, Pluto; retained weights are 35999, 47270, 14739, 3600, 1886, 299, 120, 42, 21, 14 (`vendor/epi-kernel/reference/include/m2.h:342`). These are retained symbolic model coefficients, not current measured velocities.

Dignity precedence is domicile 1.20, else exaltation 1.10, else opposite domicile/detriment 0.90, else opposite exaltation/fall 0.85, else 1.00; Mercury in Virgo therefore receives domicile, not exaltation (`intake_composition.rs:81`). Preserve the recovered domicile/exaltation tables, including their modern outer-planet entries, rather than substituting another school's tables. Sum into raw EFWA; L1 normalization gives display balance, L2 normalization gives **q_natal**, not complete Q_identity (`:215–253`). No retrograde multiplier is recovered here.

Actual provider input requires all ten unique native planet IDs, matching body names and finite valid longitudes (`natal_composition`). Unknown time stays unavailable; approximate time stays conditional. The port now partitions evidence through the graph-backed seven classical-planet routes, retaining all ten planets in global natal evidence. `centre_natal_orientation` computes an L2 natal elemental direction for each centre; this is not a complete constitution or a dynamics calculation. Every mapped planet correction changes only its routed local evidence; an outer-planet correction changes global natal evidence while leaving these seven direct partitions unchanged.

## Recoverable draft policies: choose explicitly, do not silently ratify

The original Nara-Personal context files remain **development handoff drafts v0.1**. Their filesystem source is `Work/personal/Nara-Personal/context/` in Central; they are not amendments to the ratified M4 matrix.

| Draft | Exact recoverable proposal and remaining choice |
| --- | --- |
| `nara-m4-0-0-birthdate-encoding-spec.md:551–630,653–679` | Full 12×6 MEF pass, then elemental extraction; provisional factors fullPass .35, anchor 1, inverse .50, square .25, Möbius .40, spanda .35, tritone .30. Datum weights synthesis 6; full-name/date-total 5; vowels 4; consonants/day 3; month/year/word 2; letter/inclusion 1. The master-number modifier remains “×1.5 **or** strong flag”; calibration/affinity and normalization choices require a selected version. Birth intake alone does not implement this encoding. |
| `nara-m4-0-identity-branch-integration-map.md:1408–1451,1915–1923` | Provisional layer weights birth .25, astrology .35, Jungian .15, Gene Keys .20, Human Design .25, followed by available-layer/reliability/user-desire treatment; alternative core normalize(.40 birth + .60 astrology), expanded core primacy .45–.70. Confidence, resonance bonuses, conflicts, absences and music's core/expanded choice remain unresolved. |
| `M4-prime-SPEC.md:216–230,362` | Recovers the natal elemental algorithm but explicitly records tension between natal `q_personal` and integrated `q_Nara`. Proposed naming resolution: retain **q_natal** as one constituent and reserve **Q_identity** for an explicitly selected multi-layer policy. |

**Recommendation D1:** preserve typed source evidence now; select/version birth-MEF and integrated-quintessence policies separately. The matrix's open final compression law (`:1135–1138`) prevents presenting these draft numbers as ratified defaults. Acceptance must show each available layer's contribution, missing-layer treatment, disagreements and source revisions; a changed import must recompute its own contribution without inventing other offices. Owner seam: `nara/intake.rs` → a QL identity-policy adapter → `PersonalConstitution.identity`; imports/self-report are evidence routes, not automatic quaternion calculations.

**Current bounded implementation:** `nara/identity_composition.rs` exposes the
explicitly selected `draft-core-birthdate-decanic-40-60-v1` policy through the
existing optional `IdentityProfile.composition_policy`. It applies the source
integration map §§35–37 formula **normalize(0.40 q_birthdate + 0.60 q_decan)**
to the actual native unit EFWA inputs. An explicit `encoding_policy` and both
inputs are required; absence is preserved without weight redistribution.
The reading remains `unselected` when no composition policy was chosen and
`unavailable` when a selected policy lacks an input. This produces a named
draft `q_core`, not a ratified universal identity law or receiver constitution.
Extended blending, centre gains, transit/activity and coupling are not inferred
from this selected core formula.

## Smallest proposed centre orientation policy

**Recommendation D2, proposed ID `ql.nara-centre-natal-orientation/v1`:** for centre c, take its existing `centre_evidence[c].raw_efwa_evidence = e_c`; set **o_c = e_c / ||e_c||₂** in fixed EFWA order. The intake adapter already exposes this separately named natal direction through the existing native elemental mapper. Promoting it to a complete receiver constitution remains distinct. It adds no numeric coefficients and now uses canonical graph planet membership with recovered natal weights. Attach the natal snapshot, profile revision, natal policy and centre-policy revisions to any derived constitution source.

This determines only a **natal baseline orientation** for each independently calculated centre. Centres can legitimately have equal orientations; independence does not mean forced difference. The seven direct routes each carry one classical planet, so corrections within the same sign can leave orientation unchanged. No longitude-to-angle interpolation is implied. Sun contributes to global natal evidence and its canonical crown route; the separate all-centre reception relations do not authorize copying its natal component into all seven partitions. Reusing natal evidence for global posture and local orientation must be disclosed as two roles of one source, not two independent confirmations.

The normalized vector describes the **routed natal planet's elemental evidence**, not the centre's intrinsic element or geometric axis. In particular, Mercury's natal sign does not turn Viśuddha's Ākāśa into an EFWA intrinsic axis; Moon and Sun natal components do not replace Ājñā/Sahasrāra's beyond-element standing. Promoting these readings into `ReceiverConstitution.orientation` remains a separately attributable policy choice. D2 does not settle axes for centres 5–7.

Fail on missing/duplicate planets, changed native membership, nonfinite evidence or zero/numerically negligible norm using the existing `BioQuaternion::normalized` refusal (`nara.rs:65`). Never replace failure with a neutral quaternion, copy a neighbouring centre or silently renormalize an incomplete chart. A conditional approximate-time orientation remains conditional; this does not quantify uncertainty over the whole birth-time interval. Earth orientation is separately sourced.

Implementation seam: the QL intake composition adapter supplies the natal directions; bind any admitted constitution through the native owner and do not calculate it in O:I. **Acceptance:** native two-profile same-event reception; a controlled planet crossing changes only its graph-routed local orientation before coupling; unchanged centres remain equal; within-sign corrections may leave orientation equal; Sun changes crown and global natal; unrouted outer planets change global natal only; zero/missing evidence refuses. D2 alone does not establish full personal engine consumption.

## Remaining numerical decisions and owner seams

The [superseded K² Expression binding](https://github.com/EpiLogos/QL-MEF/blob/3db025b519ac98c03744a1c5e0f81ba3ed5e4a8e/docs/kernel-rebuild/K2-EXPRESSION-BINDING.md) contains executable oscillator, PCM, surface and exact-event infrastructure. Its phase assignment `(address72+i+1)π/36`, played-body interpretation and per-centre M1/M2/M3 triple are **not admitted canonical meaning** for this personal instrument. The [owner's correction](https://github.com/EpiLogos/QL-MEF/issues/201#issuecomment-5859688868) retracts that centre model. Preserve useful numerical owners and lifecycle/event/currentness protections, while deriving actual reception through the M2 graph and independently admitting any sonal policy. A running oscillator proves numerical operation, not source-correct Nara phase or coupling.

| Decision | Missing determination / concrete next decision | Native owner and required acceptance |
| --- | --- | --- |
| D3: world drive | Replace the unsourced M1/M2/M3-triple interpretation with the existing M2 paths: current/natal planet, decan ruler, maqam→planet→chakra, solar reception and Earth grounding. The direct planetary route is now recovered in C/Rust and intake. Recover the remaining typed routes and their native calculations before naming a numerical gap. | Coordinate with the M1–M3 native owner; retain exact event/basis for two identities and relation/source revisions. Alter a native routed contribution and measure its specified receiving effect. No equal thirds or [1,1,1] fallback. |
| D4: gains and modes | Select resonance/reradiation gain meaning, units, admissible ranges, amplitude interpretation and modal basis. Retained planet weights or Cousto frequencies do not determine gain. | QL constitution `nara.rs:148–174`; embodied modes `nara/domain/embodied.rs:65–84`. `domain_operations.rs:95–98` currently continues resonance as amplitude with phase absent and modes empty. Prove zero-input behavior, declared normalization, finite bounds and a real modal change reaching the consumer. |
| D5: coupling and body | Use the recovered `ASCENDS_TO` chain, solar `CHAKRAL_VIRTUE_RECEPTION`, and Earth `FEEDS_EARTH_ELEMENT` / `GROUNDS_CHAKRAL_PATHWAY` with their exact source identities; trace their properties and native dynamics before choosing any evolution law. Distinct centres are not seven copies of one scalar. | `nara/domain/embodied.rs` and `domain_operations.rs` retain the coupling seam. Source-qualified anatomy and Earth grounding are now bound through `domain_body.rs`; phase, modal and coupling fields remain unpopulated by that continuation. QL owns dynamics; coordinate shared engine writes with M1–M3 lead. Perturb one centre and prove direct versus coupled effects, temporal stability and independently addressable body relations. |
| D6: sonal phase | Specify audible frequency derivation, oscillator phase origin/evolution/reset, overtone/mode policy, timbre and amplitude mapping, native sample-rate scheduling and interruption. M2's discrete phase code is not an oscillator phase in radians; Cousto table values alone do not define the instrument. | M2/native continuous audio owner with O:I shared Expression consumer; [Expression contract](kernel-rebuild/EXPRESSION-FIELD-PROJECTION.md):80–84. Prove emitted audio consumes admitted native state; phase continuity, centre-selective change, interruption and native/renderer identity agree. Spatial chakra centres must not be substituted by seven cymatic stations. |

## Evidence boundary and return

The controlled 2026-09-27 native replay (`desktop/cradle/tests/nara-identity-native.py` in O:I; task NOW `T/native-replay/repaired-run-2/receipt.json`) records 15 checks / 18 kernel operations: real chart/sky agreement, natal normalization/partitions, two identities, correction, persistence/reopen and refusal contracts. It does **not** establish D1–D6, engine consumption, sonal acceptance or a completed personal Expression. Preserve that boundary when returning the installed journey.

Owner review can accept D2 independently as a narrowly named natal orientation policy, select the draft versions under D1, and request exact native observable/dynamics contracts for D3–D6. Acceptance must be recorded at the relevant source owner; this generated note does not adopt itself or authorize fabricated values while those decisions are pending.
