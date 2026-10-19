# M3 source bindings and semantic parity

K7 / #131 support contract. The native engine and its C/Rust execution evidence
remain separate from this independent source observation. This is not an accepted
K4 census, a second M registry, a live Neo4j observation, or a C++ embodiment test.

## Current source comparison — 19 October 2026

The current lock `fixtures/kernel/m3-source-bindings-v1.json` observes Bimba
source revision `4afb7c6de1e0a5b0bca26a541ab7b5a655b624e2f476bd3cc70306eba88661a4`:
1,004 M3 nodes and 5,071 incident qualified relations, from a live READ on
19 October 2026 (2,149 nodes; 11,929 relations in the whole map). Two map
migrations changed M3 since 1 October:

- `migration/bimba-map/2026-10-19-m3-generation-cells.cypher` completes the
  M3-3-2 layer under the generation law. It adds the 8 Matrix 3 cells whose
  lower trigram is Xun, sets Matrix 1's Zhen and Gen pairs to the HMS table, and
  gives every cell its lower and upper pair, its role and its full
  classification. The layer holds 192 cells, 336 `YIELDS_CODON` edges (one per
  distinct yield) and 360 `USES_PAIR` edges (a cell whose two trigrams share a
  pair has one edge carrying both roles).
- `migration/bimba-map/2026-10-19-m3-coin-naming.cypher` names the nucleotides by
  their coin values: A Old Yin (6), T Old Yang (9), C Young Yin (8), G Young Yang
  (7), carried as `m_3_2_coin_value`, `m_3_2_iching_line`, `m_3_2_polarity` and
  `m_3_2_mobility`.

The comparison retains **182 unresolved findings**: 75 line-change targets, 45
incomplete or non-distinct six-line fields, 60 nuclear-register disagreements, and
one each for clock-placeholder and orientation-count. The resonance-admissibility
finding is gone: every matrix is full. The current comparator also emits the 305
open line-change qualification records admitted on 2 October; none is decided
here. The 383 source `LINE_CHANGE` edges are unchanged.

## Source comparison — 1 October 2026 (predecessor)

The current lock `fixtures/kernel/m3-source-bindings-v1.json` observes Bimba
source revision `907c46bc8a65b47e12f14aa4d8b444263dc956a1a7b4b6d038e57223d6073288`:
996 M3 nodes and 4,952 incident qualified relations. A fresh live READ on
1 October 2026 returned that same full-world revision (2,141 nodes; 11,810
relations). This qualifies source custody, not receiving or installed parity.

The current comparison retains **183 unresolved findings**: 75 line-change
targets, 45 incomplete or non-distinct six-line fields, 60 nuclear-register
disagreements, and one each for clock-placeholder, orientation-count and
resonance-admissibility. Its **383 source LINE_CHANGE edges** are distinct from
the native 384 XOR transitions and from 322 source YIELDS_CODON edges. A native
finite proof does not supply a missing source edge or resolve these disagreements.
Do not rewrite Bimba to make this comparator green.

See `EPI-WORLD-FIDELITY-AUDIT-2026-09-30.md` and the existing recovery map for
the current source-to-Expression obligations. The independent original-source
challenge on 1 October confirmed this distinction. The following K2 observation
is retained as dated predecessor evidence; its 563-record counts and pinned CE
source are not the current lock.

## Historical K2 join at daa660c

`scripts/m3-source-parity.py` reads the full M3 node and relation payloads from the
K2-pinned Epi source revision `daa660cbc1b8c5da83828698665a753852cb0287` and joins them
onto `fixtures/kernel/m-tree-v1.json`. It checks both complete files by byte count,
SHA-256 and Git blob ID, then checks all 996 node records and 4,891 relation records
by their individual payload hashes, order, exact endpoints and existing identities.
No source property, qualified relation, null endpoint or duplicate source edge is
discarded. Cross-M endpoints keep their existing K2 IDs. This is a serialized-source
observation, not a claim that the current live graph has the same state.

The checked-in `fixtures/kernel/m3-source-bindings-v1.json` is a **lock manifest**,
not a reduced replacement graph. It pins the complete generated binding projection,
semantic audit, role/edge counts, native authority files and per-field projections.
A normal run checks the lock. `--refresh-lock` is an explicit authoring operation;
CI never calls it, and a refreshed digest alone is not reconciliation authority.

```sh
python3 scripts/m3-source-parity.py --source-root /path/to/pinned-epi-source
M3_SOURCE_ROOT=/path/to/pinned-epi-source \
  python3 -m unittest discover -s scripts/tests -p test_m3_source_parity.py -v
```

The dedicated workflow checks out both repositories read-only and publishes an
exact-head artifact containing:

- `bindings.json`: all full source payloads joined to the existing K2 coordinate,
  parent, relation and source-record identities;
- `audit.json`: typed clock, matrix, hexagram, transcription and phase projections,
  all open discrepancies, counts and the explicit limits of this observation;
- `ledger-discrepancies.json`: existing K3-shaped **open** discrepancy records for
  reconciliation in the canonical M ledger, plus the test and revision receipts.

The source audit does not edit `fixtures/kernel/m-ledger-v1.json`. Tests append its
records to an in-memory copy and run the real ledger schema/content validator;
assessments, bindings, evidence and capability rows remain unchanged. This avoids a
competing ledger writer while the native M3 lane is active. Before #131 closure the
engine lane must ingest/reconcile these records in the canonical ledger, preserving
open status where no authorised resolution exists. An artifact is not itself an
accepted ledger resolution.

## Registers must not be collapsed

The 64 hexagrams have separate source binary strings, King Wen ordinals, explicit
upper/lower trigram relations, and their relation-derived six-bit addresses. Matrix
cell suffixes are source H ordinals, not native addresses. Each of the 184 cells
retains every `USES_Pair`, both positive/negative `YIELDS_CODON` relations and its
explicit `RESOLVES_TO` edge. Null pair targets remain unresolved rather than being
filled with the nearest plausible pair. Equal matrix cardinality is not parity.

DNA codons, phase codons, RNA forms, Tarot cards, amino/translation-signal nodes and
karyotype nodes are distinct identities connected by typed relations. In particular,
`#3-4.0` is not `#3-4-0`. The 37 explicit RNA forms are checked through their exact
T-to-U transcription edges; the 27 no-T codons remain shared forms. The resulting
101 distinct DNA/RNA strings do not require an invented set of 64 new RNA nodes.
The historic phase-polarity flip is not evidence for this transcription operation.

All 65 `TRANSLATES_TO` edges across the 64 phase codons survive: ATG points to both
Methionine and Start. The two conditional translation edges retain their conditions
in `bindings.json`. Native amino-array indices must not be treated as source amino
coordinates: the source has its own explicit identity order. Major/card/cascade and
karyotype relations are also many-valued. For example, the source Fool has four
`PROVIDES_VESSEL_FOR` edges (including qualified repeats) and six cascade edges;
there is no deduplication into a fabricated one-card/one-chromosome bijection.

The clock projection binds each of 360 dynamic coordinates to an independent one
of the 24 backbone coordinates through both `ANCHORED_BY` and its governor inverse,
then verifies the complete clockwise and polar-opposite cycles. Its degree-zero
identity remains exactly `#3-5-5/0-0/360`. A degree flag at every fifteenth step is
not the backbone coordinate. The 720-degree operational double-cover and its poses
must be executed by the native engine; this source join does not infer them from
passing 360-degree topology checks.

## Historical differences in the daa660c lock

That predecessor audit emitted **563 open records**, not 563 newly authorised source
corrections. A passing audit means every recorded difference is still visible and
matches the reviewed lock; it does not mean that all source/native semantics agree.

| Field | Open records | Observed difference |
|---|---:|---|
| Stored hexagram binary code | 3 | Three `binaryCode` strings disagree with the explicit upper/lower trigram composition. The strings have 61 distinct values; the relation-derived addresses cover all 64. |
| Line-change graph | 258 | These source targets differ from the native one-line XOR target when both endpoints are resolved independently through their trigram relations. All 384 original edges remain retained. |
| Nuclear register | 60 | Source nuclear strings/coordinates differ from the native inner-line extraction. Both readings are recorded; no convention change or source rewrite is silently inferred. |
| Pair descriptors | 12 | Frozen source S/D values differ from the existing M3-COIN-1-corrected C table. The audit tests its native comparison values against that actual retained table. |
| Codon/phase charge records | 112 | Frozen four-charge records differ from the ratified A=6, T=9, C=8, G=7 derivation. Both complete charge vectors remain visible. |
| Matrix pair endpoints | 114 | `USES_Pair` targets are null in the source. These are unresolved edges, not zero-valued pairs. **Resolved 19 Oct 2026:** the generation-law migration gave every cell of the map's M3-3-2 layer its lower and upper pair and its role; the native law reproduces all 192 cells (see "The generation law" below). |
| Matrix pair role | 1 | `#3-3-2-2-24` has only its upper-pair edge. **Resolved 19 Oct 2026:** the cell now carries both pairs. |
| Resonance admissibility | 1 | Source Matrix3 absence is `[6,14,22,30,38,46,54,62]` in relation-derived hexagram addresses; the retained native gap set is `[5,21,26,34,42,53,58,61]`. Both contain eight entries, but they are not the same set. **Resolved 19 Oct 2026:** both were artefacts. The map lacked the 8 Matrix 3 cells whose lower trigram is Xun (the migration adds them: 192 cells, no absence). The native gap set came from a legacy table (`M3_RES_MATRIX`, 56 identity entries and 8 invented holes) that was not the Same-Quality matrix; it is retired and no native gap set remains. |
| Orientation count | 1 | Source `#3-4.0-2-8` / TCT says eight states. The native palindromic classifier/profile says seven. The frozen source sums to 473; the existing lawful native surface remains 472. |
| Legacy clock placeholder warrant | 1 | The old C LUT explicitly used a computed hexagram approximation and zero-valued symbolic placeholders. Those fields and fifteenth-degree flags are not proof of source-backed symbolic projections or independent backbone identities. |

The source code audit is confined to these explicit comparisons. Its full property
retention is not an assertion that every semantic prose field, biological reading,
experiential claim or cross-M embodiment has been operationally verified.

## The generation law and the matrix actions

The map's M3-3-2 layer holds one generation cell per matrix and hexagram:
3 × 64 = 192. A matrix sends each trigram to a nucleotide pair (the palindromic
trigrams Qian, Kun, Kan, Li go to TT, AA, CC, GG in every matrix; the other four
go to the two mixed pairs of the matrix's base partition, in both orders). For a
hexagram with lower pair `ab` and upper pair `cd`, the positive codon is `a·c·d`
and the negative codon is `a·b·d`; the cell is non-dual exactly when `b = c`.
Each matrix has 16 non-dual and 48 dual cells; the field has 384 polarity slots,
336 distinct yields and 96 non-dual slots; the fixed points of the cells are the
40 non-dual codons. Source: `fixtures/kernel/m3-generation-field-v1.json`;
kernel: `crates/ql-core/src/pole/generation.rs` and `ql_m3_generate` in C.

The three matrices act on a codon as their base partner at every site:
Complementary (A↔T, C↔G) `⊕0x15`, Moving/Resting (A↔G, T↔C) `⊕0x3F`,
Same-Quality (A↔C, T↔G) `⊕0x2A` in the kernel encoding (A = 0b00, T = 0b01,
C = 0b10, G = 0b11). On a hexagram each matrix sends every trigram to the
trigram whose pair is the partner of its pair. Matrix 1 ∘ Matrix 2 = Matrix 3 at
both scales. There are no gaps.

The earlier native operators were wrong and are retired. A March legacy spec
(FR 2.3.9) built 64/64/56-entry tables from the map's per-matrix cell counts: it
called `codon ⊕ 0x3F` "Complementary" (that is Matrix 2's partner), called trigram
exchange "Moving/Resting" (it has no base-partner meaning), and called an
identity table with 8 holes "Same-Quality", narrating the holes as evolutionary
gaps. The vendor reference still carries those tables (`vendor/` is frozen); the
native kernel does not read them. The I Ching's own hexagram complement and
trigram exchange remain named I Ching relations; they are not the matrices.
"Equal matrix cardinality is not parity" held here too.

## Native-engine consumption

Use `Audit(project(...)).run()` only as build/test tooling. Runtime code should use
the shared K2 IDs and the native M3 contract, not parse Python output on a hot path.
The artifact supplies source-backed projections and exact relation IDs for native
parity fixtures. Every projection records its source and derived register separately;
consumers must preserve that distinction and refer disputed cases to the M ledger.
C/Rust parity evidence can establish native computation, but cannot erase these
Bimba discrepancies or imply K4, Neo4j, C++ or experiential readiness.

The 20 regression tests cover lossless round trips, both file and record tampering,
shared ID preservation, exact separator spellings, malformed topology, transcription
retargeting, multi-valued translations/cascades, native pair-table correspondence,
the 473/472 distinction, source/native gap differences and canonical-ledger
compatibility without readiness promotion. Missing external source causes a visible
skip during generic offline test discovery; the explicit test entrypoint and the
workflow require the source and do not permit that skip to stand for execution.
