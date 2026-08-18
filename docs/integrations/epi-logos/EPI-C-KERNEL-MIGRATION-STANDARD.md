# Epi C Kernel Migration Standard

**Status:** migration authority for the deep Epi computational rebase  
**Programme relation:** QL-MEF #30 / #33; parallel to, and not a gate on, the M4′ Nara lived-product build  
**Reference Epi revision:** `daa660cbc1b8c5da83828698665a753852cb0287`  
**QL-MEF migration base:** `d0e012b9a2080b75b9583d5fcc672775cce3a7ca`

## 1. Why this migration exists

The historical Epi implementation already contains a substantial computational body. The purpose of this migration is not to replace that work with a new architecture because a newer repository or language exists. It is to make the existing computation legible, testable and progressively rebased onto the stable generalized QL/MEF primitive foundation so that future formal work is performed on one dependable computational ground.

The migration therefore begins from returned implementation reality: preserve the current C computation, establish evidence around it, separate reusable primitive law from Epi-specific semantic composition, and only then refactor.

## 2. Computational authority: C first

`Body/S/S0/epi-lib` is the reference computational authority for this programme.

The intended end-state remains C-centred unless a particular operation has a concrete reason to live elsewhere. QL-MEF being a Rust workspace today does **not** imply that the deep computational kernel should be translated into Rust.

Rules:

1. Preserve existing C data and algorithms before changing representation.
2. Prefer a stable C library/API for deterministic kernel, relation, topology, harmonic, MEF and deep-domain computation where the existing implementation is already C.
3. Rust may expose safe adapters, service contracts or host integration over the C ABI. It is not the default destination for computation.
4. A C → Rust rewrite requires an explicit maintenance/runtime reason, a named owner decision and parity evidence. Language modernization is not a reason by itself.
5. During rebasing, do not combine semantic redesign with implementation migration.

## 3. `epi` is the computational front door

The Epi CLI is intended to be the ordinary human- and machine-facing route into the computational heart across the library.

The desired relation is:

```text
human / script / agent
        ↓
       epi
thin command + query + serialization + provenance layer
        ↓
stable C library / QL-MEF computational API
        ↓
kernel + shared primitives + M0–M5 computation
```

The CLI may provide human-readable and structured machine-readable forms, but it must not become a second implementation of the domain. Commands dispatch into the library; they do not carry hidden mathematical or semantic law of their own.

A TUI may consume the same CLI/library capabilities. It is optional presentation, not computational or semantic authority.

## 4. `portal-core` is evidence, not the target architecture

`Body/S/S0/portal-core` contains useful later Rust work: projections, consumer-facing types, tests, Nara boundaries, VĀK representations and other implementation evidence. It was not, however, a satisfactory general computational/CLI architecture and is not the body being promoted here.

For this migration its status is:

- **secondary implementation witness** where it demonstrates a useful computation, consumer expectation or parity oracle;
- **adapter/projection evidence** where it has a later interface worth preserving;
- **not canonical computational authority** merely because code is newer or Rust;
- **not a bulk source to copy forward**;
- **not the basis for resurrecting the old CLI/TUI design**.

Any individual `portal-core` computation proposed for promotion must be classified on its own merits and traced to its source/provenance.

## 5. Implementation ownership is not semantic ownership

Moving a generalized operation into QL-MEF does not automatically move the semantic world that uses it.

A useful test is: *can this operation be stated and used without knowing that Paramasiva, Parashakti, Mahamaya, Nara or Epii exist?*

If yes, it may be a generalized QL-MEF primitive. If no, it normally remains Epi-domain composition even if it consumes QL-MEF computation.

Examples:

- generalized position, conjugation, relation, modular/harmonic or MEF operations may be QL-MEF-owned after explicit promotion;
- canonical Epi Agents, M/M′ identities, Nara, Mahamaya and other Epi semantic interpretation remain Epi-owned;
- canonical datasets must retain their authored/source authority even if their storage or loader moves.

## 6. Frozen reference before refactor

The reference corpus is locked in `migration/epi-kernel/source-lock.json` and copied under `vendor/epi-kernel/reference/`.

The freeze exists so that "equivalent" keeps a concrete meaning after source begins to change. Reference files are never silently edited to make a parity test pass. If a new upstream reference is adopted, the source lock changes explicitly and the difference is reviewed as a source transition.

The Epi source tree SHA and file blob identities are provenance evidence. The migration may additionally maintain generated content hashes for local verification.

## 7. Migration ladder

The chronological deep-computation programme is:

### R0 — source freeze

Copy and lock the C headers/source and relevant original tests. Make the frozen reference buildable without semantic change.

### R1 — parity harness

Run the old/reference computation and the promoted/native computation from the same canonical inputs. Exhaust finite domains where practical. Use explicit numeric tolerances and invariants for continuous/floating operations.

### R2 — primitive extraction

Extract shared dependencies in dependency order rather than M-number order: coordinate/identity mechanics, relation/QL law, modular/numeric primitives, topology, harmonic/cycle/kernel, MEF/lens/context primitives.

### R3 — data normalization

Distinguish canonical source data, deterministic derived tables, implementation optimization LUTs, mutable runtime state and research propositions. Give each source/version/hash/provenance.

### R4 — shared kernel reconstitution

Replace duplicated low-level computation with the stable C primitive/kernel floor under dual execution and parity.

### R5 — deep module rebase

Make M0–M5 consume the native primitive floor while preserving their existing computation and semantic ownership.

### R6 — internal refactor

Only after parity is strong enough, improve internal M0–M5 organization. This is where structural cleanup becomes safe rather than speculative.

### R7 — consumer flip

Switch Epi production consumers to the promoted QL-MEF computational body. Retain the historical body as a reference oracle until confidence is sufficient to archive it.

### R8 — research deepening

Resume richer musical, cymatic, topological and other formal development on the unified computational foundation, keeping research status explicit.

## 8. Parity is a gate, not decoration

For every promoted deterministic operation record:

- semantic/source authority;
- reference implementation and revision;
- promoted implementation and revision;
- operation name/version;
- canonical inputs;
- comparison method;
- exhaustive domain or fixture coverage;
- tolerances/invariants where exact equality is inappropriate;
- readiness status.

Use exhaustive comparison for small finite spaces such as six positions, twelvefold structures, pair domains, codons and MEF cells where the actual operation permits it.

A promoted path does not replace the reference production path until its declared parity gate passes.

## 9. Data survives the migration

Do not rewrite a dataset merely because its current representation is a C table.

Classify it first as one of:

- canonical authored/source data;
- deterministic derived data;
- generated optimization/LUT;
- mutable runtime state;
- research/provisional data.

Then preserve the values and provenance. A generated LUT should eventually be reproducible from its declared source where possible; a canonical authored table should remain recognizably the same authored object.

## 10. Drift guards

The following are migration failures unless explicitly justified and reviewed:

- translating the kernel to Rust simply because QL-MEF currently uses Cargo;
- treating `portal-core` as the new canonical kernel;
- putting mathematical/domain computation into CLI/TUI presentation code;
- redesigning M0–M5 semantics while claiming a parity migration;
- moving Epi semantic authority into QL-MEF because an implementation dependency moved;
- deleting the old implementation before the relevant parity evidence exists;
- using deep-kernel completeness as a new blocker for the separately ordered M4′ Nara product vertical.

## 11. Relationship to current Pratibimba work

QL-MEF #33 and the Epi/O:I primitive bridge establish that the real existing C kernel can already be reached by current product work. This deeper migration follows from that evidence; it does not invalidate the bridge or change the authored product order.

Prompt B / M4′ Nara should consume the smallest real primitives it needs while this programme independently makes the deeper computational substrate coherent.
