---
name: ql-mef-refraction-adapter-authoring
description: Author bounded QL/MEF client adapters and refraction experiments through public service contracts without making QL mandatory or promoting experimental readings into canon.
---

# QL/MEF refraction and adapter authoring

Use this Skill when extending a client adapter, refraction surface or bounded formal experiment around QL/MEF.

## Contract metadata

- Semantic ref: `ql-mef:refraction-developer`
- Native owner: `EpiLogos/QL-MEF`
- Public adapter seam: `ql-adapters`
- Public service seam: `ql-service`
- Conformance specimens: `crates/ql-adapters/tests/interop_contract.rs`, `crates/ql-adapters/tests/noql_matrix.rs`
- Verification: Rust workspace tests and native Skill validator
- Risk class: formal/adapter development; experimental evidence is non-canonical until explicit promotion

## Authoring law

Alignment, not translation. Refraction, not renaming. A client object keeps its native identity and revision. QL/MEF adds a typed reading/attachment and provenance across a replaceable service boundary.

## Adapter procedure

1. Identify the client-owned subject and the exact public object/revision to expose. Do not create a QL-specific clone as the new source of truth.
2. Represent the client input as a `ClientSubject`/typed adapter subject with its native ref, revision and optional subject/frame/context information.
3. Reuse `QlService` and the stable operation family. An adapter should translate only at the boundary and must not depend on private provider implementation state.
4. Expose explicit `Disabled`/`Optional`/`Required` policy. Prove disabled preserves client material exactly; optional failure is non-fatal; required failure is visible.
5. Preserve shared ref/revision across different client surfaces when they refer to the same subject. Do not collapse Factory or AIKit ownership into QL/MEF.
6. Preserve provider/result provenance on returned readings and keep deterministic versus semantic-stochastic result classes inspectable.
7. Add interoperability and no-QL tests before claiming the adapter usable. Include invalid coordinate/refraction cases and degraded/incompatible provider states.
8. Run workspace verification and submit the source revision to QL/MEF-owner review.

## Bounded formal experiment procedure

1. State the proposed operator/refraction relation as an explicit hypothesis/Claim and name the currently canonical QL forms it depends on.
2. Define deterministic fixtures where the hypothesis is sufficiently specified; otherwise mark semantic/model-dependent outputs as experimental readings rather than proofs.
3. Run the experiment against pinned source, provider/version/config/model and input revisions. Preserve negative and contradictory outcomes.
4. Compare results with existing canonical operators and MEF registry constraints. Do not redefine a lens or position because one software experiment is convenient.
5. Produce an evidence packet: hypothesis, method, fixtures, traces/results, provenance, failure cases and interpretation limits.
6. Keep the implementation behind an experimental/versioned seam until native-owner review explicitly promotes it.

## Promotion boundary

```text
bounded hypothesis
  -> experiment / adapter revision
  -> deterministic + semantic evidence
  -> QL/MEF owner review
  -> explicit promotion, revision or rejection
```

Benchmark wins, repeated use, model confidence and Factory Runs are evidence only. They never mutate QL canon automatically.
