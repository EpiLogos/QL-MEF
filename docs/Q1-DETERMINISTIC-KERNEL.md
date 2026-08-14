# Q1 — Minimal deterministic QL kernel (Rust)

Factory coordination: `EpiLogos/agent-system-design#116`.

Q1 is implemented in the `ql-core` Rust crate. It contains no semantic inference and no QL Loop Runtime dependency.

## Executable surface

- versioned `QlFormRef` values for `sixfold@1`, `four-plus-two@1`, and `direct-conjugate@1`;
- validated `QlPosition` P0–P5;
- explicit `QlFace::{Direct, Conjugate}`;
- canonical `QlAddress` object and string form `qladdr:<frame>@<version>/<face>/P<position>/d<depth>`;
- deterministic conjugation of address face;
- deterministic sum-to-five complement pairing `(0,5)`, `(1,4)`, `(2,3)`;
- deterministic 4+2 classification with P0/P5 implicate and P1–P4 explicate;
- deterministic result provenance carrying schema/kernel version, operation, canonical input, and canonical output;
- kernel capabilities that advertise no stochastic or research operations.

## Deliberate exclusions

Q1 does not implement MEF semantic readings, `locate`, `refract`, `relate`, `synthesise`, Loop Runtime recurrence, 36/64/harmonic operators, or client-product primitive translations. Unsupported operator names fail explicitly.

## Verification design

The Rust integration suite exhausts every P0–P5 position across both faces and representative depths, proves conjugation/complement involution properties over that finite domain, checks exact 4+2 classification, replays deterministic operations, exercises invalid-address fixtures, and rejects a harmonic research operator anti-fixture.
