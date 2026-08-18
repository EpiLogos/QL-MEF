# Epi C Kernel Migration Ledger

This ledger records **movement of implementation and evidence**. It does not silently reassign semantic authority.

Statuses used here:

- `reference-frozen` — exact historical source identity is locked;
- `reference-build-pending` — preserved but not yet proven buildable in this repository;
- `parity-pending` — native/promoted implementation must not replace the reference path yet;
- `classification-pending` — source contains mixed data/law/semantics and must be separated before promotion;
- `stays-epi-semantic` — semantic composition remains Epi-owned even if lower primitives move;
- `secondary-witness` — useful later implementation evidence, not target authority.

| Source / concern | What it is now | Intended disposition | Current state | Required evidence before replacement |
|---|---|---|---|---|
| `include/ontology.h` | shared Epi coordinate / ontology structures | classify structural mechanics vs Epi semantic identity; extract only generalized mechanics | reference-frozen / classification-pending | structural identity fixtures + ownership review |
| `src/arena.c`, `include/arena.h` | coordinate/tensor arena materialization | preserve C; decide whether generic allocation mechanics belong below Epi semantics | reference-frozen / reference-build-pending | allocation/lifetime tests |
| `src/families.c` | family/mirror instantiation | preserve first; separate generalized coordinate wiring from Epi family meaning | reference-frozen / classification-pending | structural parity over canonical family graph |
| `src/pointer_web.c`, `include/pointer_web.h` | pointer/family/holographic coordinate web | likely shared structural primitive after provenance classification | reference-frozen / parity-pending | exhaustive topology/crosslink fixtures where finite |
| `src/engine.c`, `include/engine.h` | orchestration over existing C domains | keep computation in C; reduce hidden orchestration as stable module APIs emerge | reference-frozen / classification-pending | command/operation equivalence |
| `src/kernel.c`, `include/kernel.h` | bioquaternionic / epogdoon tick kernel | first executable reference and early generalized C-kernel candidate | reference-frozen / reference-build-pending | full tick-domain parity + numeric invariants |
| `src/psychoid_numbers.c`, `include/psychoid_numbers.h` | psychoid/number computation | preserve; classify general numeric law vs Epi interpretation | reference-frozen / classification-pending | exhaustive finite numeric fixtures |
| `src/qv_data.c` | Quintessential View data | preserve values/provenance; determine canonical vs derived portions | reference-frozen / classification-pending | hash/value parity and source authority |
| `include/vak.h` | existing VĀK C contract | preserve as Epi/formal source evidence; align with accepted VĀK grammar without inventing replacement | reference-frozen / classification-pending | field/grammar roundtrips + provenance |
| `src/m0.c`, `include/m0.h` | Anuttara/M0 deep computation and data | rebase onto promoted primitives; M0 semantics remain Epi-owned | reference-frozen / stays-epi-semantic | operation/data parity by declared capability |
| `src/m1.c`, `include/m1.h` | Paramasiva/M1 matrices, quaternion/Spanda/QL-related computation | rebase onto promoted primitives; do not translate wholesale | reference-frozen / stays-epi-semantic | matrix/ring/operator parity |
| `src/m2.c`, `include/m2.h` | Parashakti/M2 72-fold, lens/tattva/correspondence body | split generalized MEF/harmonic mechanics from Epi interpretation | reference-frozen / classification-pending / stays-epi-semantic | full relevant 72-fold/lens fixtures |
| `src/m3.c`, `include/m3.h` | Mahamaya/M3 pair, trigram/hexagram/codon computation | rebase shared binary/pair/cycle mechanics; semantics stay Epi | reference-frozen / classification-pending / stays-epi-semantic | exhaustive pair/64-domain parity where applicable |
| `src/m3_clock_lut.c` | large clock/rotation lookup body | classify generated LUT vs canonical source data before moving | reference-frozen / classification-pending | reproducibility or exact value/hash provenance |
| `src/m4.c`, `include/m4.h` | Nara/M4 computation with incomplete areas | preserve implemented computation; do not fossilize stubs as canon | reference-frozen / stays-epi-semantic | implemented-vs-stub inventory + targeted parity |
| `src/m5.c`, `include/m5.h` | Epii/M5 Logos/QV guarded return | preserve formal computation; human canon mutation authority remains distinct | reference-frozen / stays-epi-semantic | operation parity + authority boundary tests |
| `src/main.c` | historical executable entry | source evidence for command reach, not the desired final CLI UX | reference-frozen / classification-pending | command inventory only; no UI inheritance assumption |
| `epi` CLI | desired human/machine computational front door | thin Epi-owned command/query/serialization/provenance layer over stable C APIs | design commitment / implementation pending | CLI calls library paths; no duplicate math/domain law |
| `portal-core` | later Rust projections/types/consumer code | mine selectively as secondary witness; do not promote wholesale | secondary-witness | per-capability provenance and parity only |
| QL-MEF Rust crates | current accepted QL/MEF product surface | may wrap or interoperate with stable C computational APIs; no default C→Rust rewrite | implemented current product / migration relation pending | explicit ABI/API contract + cross-language tests |

## Programme gates

### R0 closes when

- the exact C reference revision and tree identities are locked;
- headers and source are present unchanged under the reference path;
- relevant historical tests are either copied or explicitly addressable by locked tree identity;
- a repeatable reference build/smoke path exists;
- no promoted/native implementation is being claimed yet.

### R1 closes when

- reference operations can be invoked deterministically;
- comparison fixtures are versioned;
- finite domains are exhaustively covered where appropriate;
- floating/continuous comparisons declare tolerances/invariants;
- provenance identifies both sides of every parity result.

### R2 and later

Promote one dependency edge at a time. A deep M module may consume a promoted primitive while the rest of the module remains unchanged. The migration does not require M0 → M5 chronological rewriting.
