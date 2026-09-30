---
name: ql-law
description: The settled law of QL-MEF in one page — coordinate grammar (M prefix, the nesting threshold, context frames), position semantics, branch roles, the M3 clock facts and the owner's final rulings — each with its source. Use when about to spell, parse or generate any M coordinate, before treating a structural or numerical question as open, and before asking the owner anything about M1–M5. If the answer is here, it is settled; cite the source instead of re-deriving or asking.
---

# QL law

## Contract metadata

- Semantic ref: `ql:skill:law`
- Native owner: `EpiLogos/QL-MEF`
- Verification: the nesting threshold is enforced by `scripts/coordinate-grammar.py check` (run by `scripts/verify invariants`); the rest is cited, not re-proved here
- Risk class: structural (no authority, no mutation, no gate)

Each line is one settled law and where it is written. **CE** = Epi-Logos-C-Experiments (`~/Documents/epi/Epi-Logos-C-Experiments`), **Deep** = CE `Idea/Bimba/Map/datasets/`, **Seeds** = CE `Idea/Bimba/Seeds/M/`; CE, Deep and Seeds paths below are written from the CE root and are not in this repo. Unprefixed paths are in this repo. When a source here conflicts with a later ruling, the later ruling is listed and the older text is named under *Superseded*.

## Authority

- The live Bimba map (Neo4j) is the authority; where a spec or table disagrees, the map wins. — #254 ruling D10 (comment 5872629938); #255; `docs/kernel-rebuild/BIMBA-MAP-SOURCE.md`
- The Deep essays and correspondence tables (the CE `Idea/Bimba/Map/datasets/*-deep` folders) are a seed archive: read them to derive or recover, never at runtime. — CE `Idea/Bimba/Map/datasets/AGENTS.md`; `ProjectCentral/user/telos/integrated-expression/goal.md`
- The owner is not a source for anything these sources hold; derive it. — `goal.md`; `AGENTS.md` "Read this first"

## Coordinate grammar

- The prefix is `M` (`M2-4.3`). `#` is the legacy spelling of the same coordinate. Bare `#` alone is the root / kernel taproot, not a prefix. — Seeds `M-M-prime-coordinate-mapping-inaugural.md` §0; CE `Idea/Bimba/Map/AGENTS.md` "Ownership". The registry still stores `#`: `docs/kernel-rebuild/M-SPELLING-MIGRATION.md`
- **Nesting threshold:** after a position-4 segment the separator is `.`, never `-`: `M4.0`, `M3-4.0-1`, `M2-4.3`, `M1-4.5-0`, `M4.4.4.4`. No other position takes a dot. — CE `Idea/Bimba/Map/datasets/paramasiva-deep/Quaternal_Logic_Lived_Topology.md` §IV ("The dot after 4 is constitutional law"); CE `Idea/Bimba/Map/M1/M1-4/M1-4.md` `q_1_fractal_coordinate_system`, `q_3_nested_quaternity_with_dot`. Enforced: `scripts/coordinate-grammar.py`; open map defects are the `k2-grammar-nesting-threshold:*` ledger records
- Three operators: hyphen `-` = structural link, dot `.` = flowering (after 4 only), slash `/` = reflective identity (Möbius equivalence, not a connection). — CE `Idea/Bimba/Map/datasets/paramasiva-deep/Quaternal_Logic_Geometric_Epistemology_v2.md` "Fractal Architecture"; M1-4.md
- Context frames are written in parentheses (`M2-5-(0/1)-1`, `M3-4-(5/0)`), and a position-4 frame keeps its `4.` outside them (`M0-4.(5/0)`, `M0-4.(4.0/1-4.4/5)`). The registry drops the brackets (`#2-5-0/1-1`) and keeps the map spelling as the alias. — CE `Idea/Bimba/Map/AGENTS.md`; Seeds `Legacy/plans/2026-06-02-m-prime-cycle-3-design-reconciliation/45-bimba-map-indexing-and-dox-okf-unification.md` "Coordinate algebra"; `scripts/bimba_map.py` `ql_spelling`
- `0/1` is the non-dual anchor; `5/0` is the Möbius twist, in which synthesis sediments into the next ground. — Lived Topology §III, §V
- In file and wikilink names, `/` is written `∕` (U+2215). Frontmatter keeps `/`. — CE `Idea/Bimba/Map/AGENTS.md`
- A prime (`M3′`) is the Pratibimba, the functional coded surface of the same coordinate. The address is conserved and the phase flips, so `C3′` never resolves as `C3`. — Seeds `M-SYSTEM-INDEX.md`; CE `Idea/Bimba/Seeds/M/plans/2026-07-03-cycle-3-recapture-register.md` (DR-FLIP-1)

## Positions

- Six positions, 4+2. The explicate four are 1 what (material), 2 how (process), 3 which (form) and 4 why/where (context, the nesting threshold). The implicate two are 0 from-where (ground) and 5 to-where (synthesis). — M1-4.md `q_1_epistemology`; Seeds `q-vocabulary-canon.md`
- Matheme: `0/1 = 4+2 = 5→0 = 0/1`. Six positions is a compile-time law (`QL_POSITIONS == 6`). — Seeds `epi-logos-kernel-spec.md`; `4-5-0-CONTEMPLATION-INTEGRATION-PLAN.md` §1
- Context frames run `(0000) → (0/1) → (0/1/2) → (0/1/2/3) → (4.0-4.5) → (5/0)`. `(4.0–4.5)` is the only frame with an internal rhythm: 4 whole, .0 ground, .1–.4 the quaternary again, .5 transcendence nested in immanence. — Lived Topology §IV–§V
- Subsystem frames: M0 `(00/00)`, M1 `(0/1)`, M2 `(0/1/2)`, M3 `(0/1/2/3)`, M4 `(4.0/1-4.4/5)`, M5 `(5/0)`. — Seeds `M'-SYSTEM-SPEC.md` "Subsystem CF Assignment"
- Pair families, the Klein phase map and the P/P′ meanings: `skills/ql-foundations/SKILL.md` (not repeated here).

## Branches

- M0 Anuttara, M1 Paramasiva, M2 Parashakti, M3 Mahamaya, M4 Nara, M5 Epii. — CE `Idea/Bimba/Map/M*/AGENTS.md`
- **M1 generates the sound**: the Prakāśa torus oscillator and the M1-3 Spanda bi-phasal oscillator. M2 needs no base pitch of its own. — #254 ruling D13 (M1′-SPEC §14.1)
- **M2 is the synth** that shapes M1's signal. The live sky and decan rhythm modulate it; the aperture lenses and Vimarśa's reading filter and resonate it; tattva sets texture and the cymatic skin. — #254 "M2 as real musical software"
- **M2 is the sky.** `M2-5` holds the planets `M2-5-2`…`-9` and the Sun `M2-5-(0/1)`, whose children are Earth `M2-5-(0/1)-0` and the chakras `-1`…`-7`. A dated sky is M2-5's state. — `AGENTS.md` "Read this first"
- **M3-5 is a geometric codon–hexagram clock**, not an astrological or solar one. The 30°×12 zodiac is one lens over it. — `AGENTS.md`; `docs/origami work/M3/M3-MAHAMAYA-DEEP-CAPABILITY-COORDINATE-MATRIX.md` §11

## M3 clock

- 384 = 360 + 24 = 64 × 6. The clock has 360 degree nodes plus 24 palindromic backbone nodes at `degree % 15 == 0`, and 384 is also the count of one-line changes. — Seeds `M3'/M3'-SPEC.md` §8.6 (`_Static_assert(360 + 24 == 64 * 6)`)
- 16+2 = the 18-lens canon: 16 static divisor lenses, plus the Fibonacci/Pisano ground (60 × 6° = 360, LCM(6,5,12) = 60), plus the Anuttara void ring. — owner ruling 2026-09-07: `crates/ql-mef/src/pole_state.rs` `APERTURE_CLOCK_CANON_VERSION`; `c/src/m3_domain.c`; ledger `k7-m3:difference-aperture-source`
- The zodiac is lens 9: `divisions[9]` = 30° × 12, counted from λ 0° = Aries. — `c/src/m3_domain.c`; `crates/ql-mef/src/m3_inscription.rs`; CE `Idea/Bimba/Map/datasets/mahamaya-deep/pleroma-30-syzygy-lens6-integration.md` §0
- The M2 MEF lenses (12) and the M3 clock lenses are separate namespaces and never merge. — M3′-SPEC §8.15
- 472 rotational poses = 40 non-dual × 7 + 24 dual × 8, generated from pair composition rather than letter cycling. — M3′-SPEC §7, §8.5; matrix `M3-C13`
- Three matrices = the quaternion axes: complementary → i, moving/resting → j, same-quality → k. — matrix §6; M3′-SPEC §8.2–§8.3
- Coin: A = 6, T = 9, C = 8, G = 7 (odd = yang; T Fire and G Air are yang). — #254 coin ruling; `M3-COIN-1` (`vendor/epi-kernel/corrections/M3-COIN-1.patch`, PRs #111/#112)

## Owner rulings on #254 (final; never re-ask)

- D0: re-author the planetary just ratios, Shem `planetaryColor` and the codon↔hexagram pairing into the map.
- D5: one tick = 360/12 = 30°.
- D6: the torus is the clock; planets sit at θ = λ.
- D10: the map wins.
- D12: the hour ruler uses the observer's place and time (Nara's location, or a declared world place).
- D30: every magnitude is a live, tunable parameter.
- The rest (D1–D4, D7–D9, D11, D14–D29) proceed on the #254 map's recommendations.

Source: QL-MEF #254 comment 5872629938.

## Superseded (do not cite as current)

- `16+1` lenses (M3′-SPEC §8.0/§8.15, 2026-07-15): superseded by the 18-lens ruling of 2026-09-07.
- `C = 7 / G = 8` (M3′-SPEC §8.1, and the upstream map before the 28 Sep 2026 coin migrations `migration/bimba-map/2026-09-28-d0-reauthor-and-coin.cypher` and `migration/bimba-map/2026-09-28-coin-prose.cypher`): superseded by M3-COIN-1.
- `M4-4-4-4` / `M4-0…M4-5` spellings (Seeds `M4'/M4'-SPEC.md` §7.1, inaugural): superseded by the nesting threshold (`M4.4.4.4`, `M4.0…M4.5`).
- The M2′ spec's planet tables, where they disagree with the map (D10).
