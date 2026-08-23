# QL / MEF

Standalone executable QL/MEF product for EpiLogos.

## Repository ownership

**This repository is the implementation repository for the standalone QL/MEF product.**

The product architecture was developed in `EpiLogos/agent-system-design` draft PR #111, especially `docs/canon/ql-mef-module/`. That Factory-side package remains the governing cross-product target design until it is ratified/merged, but executable product code, product-local fixtures, product-local implementation evidence, releases, and the authoritative Q1–Q7 implementation Wayfinder live here.

The move to this repository does **not** merge QL/MEF with the QL Loop Runtime. The Loop Runtime remains in `EpiLogos/agent-system-design` and consumes this product only through the explicitly versioned integration seam.

## Implementation bodies

QL/MEF's service/product implementation is a Rust Cargo workspace. The historical holographic C kernel has also been nativeised here under `c/` as the source-proven foundational kernel body rather than rewritten into Rust. C and Rust share the versioned holographic-kernel semantic contract and conformance fixtures.

The intended Rust architectural modules are crates/modules corresponding to `ql-core`, `ql-mef`, `ql-semantic`, `ql-service`, `ql-adapters`, and `ql-fixtures` as they become warranted by the Q1–Q7 programme.

## Foundational kernel reference

The compact human-readable authority for the current kernel field is [`docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md`](docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md), paired with the machine-readable `fixtures/kernel/holographic-kernel-contract-v1.tsv` contract.

It records the current source-proven identities for Hash/raw bedrock, C/P/L/S/T/M × direct/prime, A/B/C and the 3×3 square grammar, VĀK, MEF and the seven Context Frames. Richer harmonic/musical development grows from that centre under the continuing formal programme.

## Product boundary

QL/MEF owns:

- canonical executable QL references, forms, addresses and sufficiently specified deterministic operators;
- the complete twelve-lens MEF registry and sublens/refraction contracts;
- QL/MEF provider capability negotiation;
- `locate`, `refract`, `relate`, and `synthesise` service operations;
- provider/readings provenance;
- evidence-led promotion of deeper operators.

It does not own Factory `Project`/`Run`/`Action` identities, AIKit `ContextResolution`, agent host/harness mechanics, Loop Runtime recurrence, Workcell materialisation, or Central Control.

> Alignment, not translation. Refraction, not renaming.

## Governing sources

- Native kernel Wayfinder #78 and kernel nativeisation #56
- [`docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md`](docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md) + `fixtures/kernel/holographic-kernel-contract-v1.tsv`
- Factory draft PR #111 — standalone QL/MEF target architecture
- Factory #113 — cross-repository identity/provenance floor
- Factory #100 — frozen Loop Runtime seam (required first at Q5)
- Factory #77–#80 — Factory-side QL integration/verification spans
- AIKit #30 — AIKit QL/MEF interoperability

## Development order

`Q1 deterministic kernel → Q2 MEF registry/contracts → Q3 provider/service → Q4 client adapters`

Then `Q3 + frozen runtime seam → Q5 runtime integration`, followed by evidence-led Q6/Q7 deepening and promotion.

The native holographic kernel is the shared foundational centre beneath those staged product surfaces; deeper QL/MEF work extends its returned relations rather than reconstructing another substrate.
