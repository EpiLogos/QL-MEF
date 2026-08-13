# QL / MEF

Standalone executable QL/MEF product for EpiLogos.

## Repository ownership

**This repository is the implementation repository for the standalone QL/MEF product.**

The product architecture was developed in `EpiLogos/agent-system-design` draft PR #111, especially `docs/canon/ql-mef-module/`. That Factory-side package remains the governing cross-product target design until it is ratified/merged, but executable product code, product-local fixtures, product-local implementation evidence, releases, and the authoritative Q1–Q7 implementation Wayfinder live here.

The move to this repository does **not** merge QL/MEF with the QL Loop Runtime. The Loop Runtime remains in `EpiLogos/agent-system-design` and consumes this product only through the explicitly versioned integration seam.

## Implementation language

**QL/MEF is a Rust product.** Production implementation in this repository is a Cargo workspace. The intended architectural modules are Rust crates/modules corresponding to `ql-core`, `ql-mef`, `ql-semantic`, `ql-service`, `ql-adapters`, and `ql-fixtures` as they become warranted by the Q1–Q7 programme.

Non-Rust prototype branches are non-authoritative and must not be used as Closure evidence for the implementation programme.

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

- Factory draft PR #111 — standalone QL/MEF target architecture
- Factory #113 — cross-repository identity/provenance floor
- Factory #100 — frozen Loop Runtime seam (required first at Q5)
- Factory #77–#80 — Factory-side QL integration/verification spans
- AIKit #30 — AIKit QL/MEF interoperability

## Development order

`Q1 deterministic kernel → Q2 MEF registry/contracts → Q3 provider/service → Q4 client adapters`

Then `Q3 + frozen runtime seam → Q5 runtime integration`, followed by evidence-led Q6/Q7 deepening and promotion.

Q1–Q4 do not depend on Loop Runtime implementation progress.
