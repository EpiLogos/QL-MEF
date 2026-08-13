# Q1 — Minimal deterministic kernel

**Status:** implementation surface for Factory coordination ticket `EpiLogos/agent-system-design#116`.

Q1 intentionally contains no semantic generation and no Loop Runtime dependency.

## Executable surface

- versioned `QLFormRef` registry for `sixfold`, `four-plus-two`, and `direct-conjugate`;
- exact raw positions `P0..P5` represented numerically as `0..5`;
- explicit `direct | conjugate` face;
- stable canonical `QLAddress` object plus string form `qladdr:<frame>@<version>/<face>/P<position>/d<depth>`;
- twelve versioned `LensRef` identities only (Q2 owns sublens/refraction semantics);
- deterministic `conjugate-address` operator;
- deterministic complement operator using the canonical sum-to-five pairs `(0,5)`, `(1,4)`, `(2,3)`;
- deterministic `4+2` classification: P0/P5 implicate; P1..P4 explicate;
- deterministic fixture provider capability declaration;
- JSON schema references and positive/negative fixture corpora.

## Deliberate exclusions

Q1 does **not** implement:

- `locate`, `refract`, `relate`, or `synthesise`;
- semantic/stochastic inference;
- MEF sublens content or role bindings;
- QL Loop Runtime recurrence/Closure/re-entry;
- 36/64/harmonic operators;
- recursive depth semantics beyond preserving a validated non-negative address depth coordinate;
- client-product primitive translations.

These exclusions are part of correctness: unsupported structures fail explicitly rather than being invented for symmetry.

## Provenance

Every deterministic operator result records schema version, kernel version, deterministic mode, operation name, canonical input address, and canonical output/value.
