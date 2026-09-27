# Nara identity composition: recovered rules and decisions

**Standing: draft-for-review; generated proposal, not adopted law.** Prepared 2026-09-27 for [QL-MEF #201](https://github.com/EpiLogos/QL-MEF/issues/201#issuecomment-5857484170). Code references describe the `feat/nara-identity-expression-20260927` working tree based on `56921a5`; line numbers are an inspection aid, not immutable source identity. This note does not change the matrix, source contracts or receiver policy.

## Fixed source and accepted boundary

| Determination | Source and consequence |
| --- | --- |
| Elemental carrier | [M4 matrix](origami%20work/M4/M4-NARA-DEEP-CAPABILITY-COORDINATE-MATRIX.md), lines 115, 374–384, 1107–1126: quaternion components are **[Earth, Fire, Water, Air]**; C/Sensation/Earth, T/U/Intuition/Fire, A/Feeling/Water, G/Thinking/Air. Older alternate orders are drift. |
| Distinct roles | Matrix lines 126–127, 1135–1138 separates exact identity/provenance hash, M3 form address, orientation and lived composition, while leaving final identity compression open. A hash must not become a quaternion or inferred archetype. |
| Composition order | `crates/ql-mef/src/nara.rs:80,246` implements normalized Hamilton multiplication **Q_identity · Q_transit · Q_activity** in that order. Keep the three source revisions distinct. |
| Seven centres and Earth | [#134 contract](kernel-rebuild/K10-NARA-M4-CONTRACT.md), lines 9–26, 50–54: continue accepted K8 reception with seven independently sourced constitutions and distinct EarthBody; QL owns domain truth, O:I owns Expression, Central owns personal source. |
| Native relations | `vendor/epi-kernel/reference/src/m2.c:264` gives planet→chakra relationships below. `:302` retains the separate Earth ground and canonical centre elemental/tattva descriptors. These descriptors are not natal zodiac elements. |

| Receiver ordinal / native chakra ID | Centre | Native planetary evidence |
| --- | --- | --- |
| 0 / 1 | Muladhara | Saturn, Pluto |
| 1 / 2 | Svadhisthana | Moon |
| 2 / 3 | Manipura | Mars, Jupiter |
| 3 / 4 | Anahata | Venus |
| 4 / 5 | Vishuddha | Mercury |
| 5 / 6 | Ajna | Uranus |
| 6 / 7 | Sahasrara | Neptune |

Sun remains the solar parent, not chakra-mapped; Earth is not an eleventh planet or eighth centre. The native chakra descriptors retain Muladhara→Prithvi/35, Svadhisthana→Apas/34, Manipura→Agni/33, Anahata→Vayu/32, Vishuddha→Akasha/31, and Ajna/Sahasrara beyond-element markers. Body-zone, sense and action references must retain their own canonical source relations; this table does not invent anatomy.

The accepted receive formula is **d_c = Σᵢ w_ci x_ci; a_c = |〈normalize(Q_composed), normalize(o_c)〉|; r_c = d_c g_c a_c; out_c = r_c h_c** (`nara.rs:492–523`). Here `x_ci` is that centre's source-qualified M1/M2/M3 triple; `w_ci`, orientation `o_c`, resonance gain `g_c` and reradiation gain `h_c` are supplied constitution fields (`:148–174`). This acceptance establishes evaluation, not the missing laws that select those inputs. `continuous/personal.rs:91–97` receives against the exact current coupled basis without advancing it.

## Recovered natal policy: available, explicitly natal-only

Source: EpiLogos/Epi-Logos-C-Experiments, commit `daa660cbc1b8c5da83828698665a753852cb0287`, blob `0496c08d8cab700afaca78a493d97a07c774b332`, `Body/S/S0/portal-core/src/personal_identity.rs:86–135,230–302`. The present port pins this identity in `crates/ql-mef/src/nara/intake_composition.rs:11–15,235–239` under `ql.nara-natal-keplerian-dignity-efwa/v1`.

For each actual natal longitude λ in [0,360), sign=floor(λ/30); use its zodiac element, not the planet's native elemental descriptor. Contribution is **retained M2 keplerian_weight × dignity × EFWA unit vector**. Native ID order is Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune, Pluto; retained weights are 35999, 47270, 14739, 3600, 1886, 299, 120, 42, 21, 14 (`vendor/epi-kernel/reference/include/m2.h:342`). These are retained symbolic model coefficients, not current measured velocities.

Dignity precedence is domicile 1.20, else exaltation 1.10, else opposite domicile/detriment 0.90, else opposite exaltation/fall 0.85, else 1.00; Mercury in Virgo therefore receives domicile, not exaltation (`intake_composition.rs:81`). Preserve the recovered domicile/exaltation tables, including their modern outer-planet entries, rather than substituting another school's tables. Sum into raw EFWA; L1 normalization gives display balance, L2 normalization gives **q_natal**, not complete Q_identity (`:215–253`). No retrograde multiplier is recovered here.

Actual provider input requires all ten unique native planet IDs, matching body names and finite valid longitudes (`:108–145`). Unknown time stays unavailable; approximate time stays conditional. The current port partitions the same raw evidence into seven native centre memberships (`:178–229`); it does not determine orientations or dynamics.

## Recoverable draft policies: choose explicitly, do not silently ratify

The original Nara-Personal context files remain **development handoff drafts v0.1**. Their filesystem source is `Work/personal/Nara-Personal/context/` in Central; they are not amendments to the ratified M4 matrix.

| Draft | Exact recoverable proposal and remaining choice |
| --- | --- |
| `nara-m4-0-0-birthdate-encoding-spec.md:551–630,653–679` | Full 12×6 MEF pass, then elemental extraction; provisional factors fullPass .35, anchor 1, inverse .50, square .25, Möbius .40, spanda .35, tritone .30. Datum weights synthesis 6; full-name/date-total 5; vowels 4; consonants/day 3; month/year/word 2; letter/inclusion 1. The master-number modifier remains “×1.5 **or** strong flag”; calibration/affinity and normalization choices require a selected version. Birth intake alone does not implement this encoding. |
| `nara-m4-0-identity-branch-integration-map.md:1408–1451,1915–1923` | Provisional layer weights birth .25, astrology .35, Jungian .15, Gene Keys .20, Human Design .25, followed by available-layer/reliability/user-desire treatment; alternative core normalize(.40 birth + .60 astrology), expanded core primacy .45–.70. Confidence, resonance bonuses, conflicts, absences and music's core/expanded choice remain unresolved. |
| `M4-prime-SPEC.md:216–230,362` | Recovers the natal elemental algorithm but explicitly records tension between natal `q_personal` and integrated `q_Nara`. Proposed naming resolution: retain **q_natal** as one constituent and reserve **Q_identity** for an explicitly selected multi-layer policy. |

**Recommendation D1:** preserve typed source evidence now; select/version birth-MEF and integrated-quintessence policies separately. The matrix's open final compression law (`:1135–1138`) prevents presenting these draft numbers as ratified defaults. Acceptance must show each available layer's contribution, missing-layer treatment, disagreements and source revisions; a changed import must recompute its own contribution without inventing other offices. Owner seam: `nara/intake.rs` → a QL identity-policy adapter → `PersonalConstitution.identity`; imports/self-report are evidence routes, not automatic quaternion calculations.

## Smallest proposed centre orientation policy

**Recommendation D2, proposed ID `ql.nara-centre-natal-orientation/v1`:** for centre c, take its existing `centre_evidence[c].raw_efwa_evidence = e_c`; set **o_c = e_c / ||e_c||₂** in fixed EFWA order. This adds no numeric coefficients and preserves the native planet membership and recovered natal weights. Attach the natal snapshot, profile revision, natal policy and centre-policy revisions to the derived constitution source.

This determines only a **natal baseline orientation** for each independently calculated centre. Centres can legitimately have equal orientations; independence does not mean forced difference. Single-planet centres reduce to their sign's elemental direction, so corrections within the same sign can leave orientation unchanged. No longitude-to-angle interpolation is implied. Sun stays in global natal evidence without being copied into every centre. Reusing natal evidence for global posture and local orientation must be disclosed as two roles of one source, not two independent confirmations.

Fail on missing/duplicate planets, changed native membership, nonfinite evidence or zero/numerically negligible norm using the existing `BioQuaternion::normalized` refusal (`nara.rs:65`). Never replace failure with a neutral quaternion, copy a neighbouring centre or silently renormalize an incomplete chart. A conditional approximate-time orientation remains conditional; this does not quantify uncertainty over the whole birth-time interval. Earth orientation is separately sourced.

Implementation seam: extend the QL intake composition adapter beside `intake_composition.rs:223`, then pass explicit orientations into `ReceiverConstitution` (`nara.rs:148`); do not calculate them in O:I. **Acceptance:** native two-profile same-event reception; a controlled planet crossing changes only its mapped local orientation before coupling; unchanged centres remain equal; within-sign corrections may leave orientation equal; Sun affects global natal only; zero/missing evidence refuses. D2 alone does not justify invoking the full receiver while D1/D3–D5 lack admitted values.

## Remaining numerical decisions and owner seams

The parallel M1–M3 lead has now published [K² Expression binding](https://github.com/EpiLogos/QL-MEF/blob/3db025b519ac98c03744a1c5e0f81ba3ed5e4a8e/docs/kernel-rebuild/K2-EXPRESSION-BINDING.md). This supplies the world body's actual pitch, nodal shape and phase `(address72+i+1)π/36`, native PCM and surface evolution, and an inspectable influence reading. Its personal reception validates the exact event basis and retains seven independently supplied inputs. It explicitly does not change the material field. The same contract leaves identity-to-constitution, event-to-centre contributions and centre presentation open. D6 below therefore concerns **personal centre sound and coupling**, not an absence of a working world-body oscillator. These two phase domains must remain distinct.

| Decision | Missing determination / concrete next decision | Native owner and required acceptance |
| --- | --- | --- |
| D3: world drive | Identify each centre's real M1/M2/M3 observable, physical/symbolic units, conversion/normalization, sign and allowed range before selecting `world_weights`. No equal thirds or [1,1,1] fallback. | M1–M3 lead supplies source-qualified `PersonalEventInput`; Nara validates at `nara.rs:312,344,498`. Prove same event/basis for two identities; dimensions and conversion provenance inspectable; alter one native contribution and measure its specified receiver effect. |
| D4: gains and modes | Select resonance/reradiation gain meaning, units, admissible ranges, amplitude interpretation and modal basis. Retained planet weights or Cousto frequencies do not determine gain. | QL constitution `nara.rs:148–174`; embodied modes `nara/domain/embodied.rs:65–84`. `domain_operations.rs:95–98` currently continues resonance as amplitude with phase absent and modes empty. Prove zero-input behavior, declared normalization, finite bounds and a real modal change reaching the consumer. |
| D5: coupling and body | Adopt a source-qualified pairwise graph, direction/sign/units, phase offsets, temporal evolution, initial conditions and Earth/body/sense/action relations. Distinct centres are not seven copies of one scalar; a structural adjacency list alone does not establish a dynamical law. | `nara/domain/embodied.rs:87–145`; continuation `domain_operations.rs:98–102` leaves coupling/body relations empty. QL owns dynamics; coordinate shared engine writes with M1–M3 lead. Perturb one centre and prove direct versus coupled effects, temporal stability and independently addressable body relations. |
| D6: sonal phase | Specify audible frequency derivation, oscillator phase origin/evolution/reset, overtone/mode policy, timbre and amplitude mapping, native sample-rate scheduling and interruption. M2's discrete phase code is not an oscillator phase in radians; Cousto table values alone do not define the instrument. | M2/native continuous audio owner with O:I shared Expression consumer; [Expression contract](kernel-rebuild/EXPRESSION-FIELD-PROJECTION.md):80–84. Prove emitted audio consumes admitted native state; phase continuity, centre-selective change, interruption and native/renderer identity agree. Spatial chakra centres must not be substituted by seven cymatic stations. |

## Evidence boundary and return

The controlled 2026-09-27 native replay (`desktop/cradle/tests/nara-identity-native.py` in O:I; task NOW `T/native-replay/repaired-run-2/receipt.json`) records 15 checks / 18 kernel operations: real chart/sky agreement, natal normalization/partitions, two identities, correction, persistence/reopen and refusal contracts. It does **not** establish D1–D6, engine consumption, sonal acceptance or a completed personal Expression. Preserve that boundary when returning the installed journey.

Owner review can accept D2 independently as a narrowly named natal orientation policy, select the draft versions under D1, and request exact native observable/dynamics contracts for D3–D6. Acceptance must be recorded at the relevant source owner; this generated note does not adopt itself or authorize fabricated values while those decisions are pending.
