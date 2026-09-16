# Geometry constellation completion Wayfinder

Status: **active implementation map for #204, 2026-09-16**  
Branch: `agent/geometry-constellation-spanda-completion`  
Canonical spec: [`CANONICAL-CONSTELLATION-RESOLUTION.md`](CANONICAL-CONSTELLATION-RESOLUTION.md)

This lane completes the existing L5.3 Geometry office. It does not create a second shape ontology, a new Vāk layer, or a replacement for the accepted structural carrier.

## G0 — source and ownership lock — complete

The following are the governing existing owners:

- `StructuralConstellation` / `ConstellationGrain` — positive local constellation structure;
- `QlShape` — canonical executable Geometry morphology;
- `RelationFieldComposition` / `ShapeBinding` — source-preserving whole/field carrier;
- `vak_composition::FramedReading` — framed propagation of the same binding/Geometry into Vāk determinations;
- `matheme::eq2` — Second-Spanda executable equation;
- M3 `FoldMotif` / `Codon64` — actual `4^3 = 2^6 = 64` form body;
- `SixBySixField` — actual `6^2 = 36` field;
- L5.3 Geometry — product office in which these forms are disclosed.

The authored M1 source already states `2^6 = 4^3 = 64`; this lane fixes the executable projection rather than proposing a new equation.

## G1 — constellation/fold canon — #205 — implemented

Required Geometry sequence:

```text
1  2  3  4  5  6  7  8  9  10  11  12
```

Implementation:

- anchor-only has Geometry fold-count 1 while retaining zero positional members;
- 7-fold is promoted into `ConstellationGrain::PartialConjugate7`;
- existing 8–12 conjugate forms remain unchanged;
- `QlShape::fold_count()` gives the Geometry reading;
- existing v1 refs remain stable; added forms use the Geometry extension version.

Compression:

- 2→1 and 3→1 are represented by `QlShapeCompression`;
- `3-fold 123 → 1-fold 0/1` carries the existing `4-fold 1234` as its recognition superset;
- derivation/operator refs make the disclosed basis reopenable rather than erased.

## G2 — Second-Spanda dual body — #206 — implemented

Canonical executable equation:

```text
100%
= 2^6 + 6^2
= 4^3 + 6^2
= 64 + 36
= 4×4×4 + 6×6
```

Implementation:

- `FourByFourByFourField` derives 3 sites from the existing M3 `FoldMotif`, four states per site, and verifies 64 against `Codon64::COUNT`;
- `SixBySixField` remains the 36 side;
- `SecondSpandaGeometry` composes the two canonical bodies;
- `matheme::binary_register()` independently computes `2^6` and asserts equality with the M3 `4^3` body;
- `quaternary_cubic_register()` exports the second canonical read;
- matheme conformance contract raised to 1.2.0.

## G3 — 18-fold and decadic views — #205/#206 — implemented

The existing direct/conjugate relation system now exposes:

```text
6 + 6′ + relational-6 = 18
18 / 6 = 3
```

`EighteenFoldGeometry` retains all three sixfold basis refs and the explicit 18→3 compression operator.

The derived decadic projection is:

```text
(4+6)^2 = 16 + 24 + 24 + 36 = 100
                       64 | 36
```

`TenByTenProjection` preserves that block structure. It is a view of the canonical `4^3 | 6^2` Second-Spanda body, not its replacement.

## G4 — existing-consumer propagation — #207 — implemented at the shared seam

No new Vāk payload is introduced. Existing consumer flow is retained:

```text
QlShape
  -> ShapeBinding
     {shape_ref, basis_refs, derivation_ref, operator_ref, ...}
  -> Vāk Whole
  -> FramedReading
     {binding, GeometryReading, ...}
```

Acceptance coverage proves that a one-fold `0/1` binding can retain a threefold basis and the exact `3→1` compression derivation/operator through the existing `ShapeBinding` contract.

Downstream Wiki/refraction/Research Canvas/Epii/Expression consumers therefore receive the same binding contract they already consume. They may expose, collapse or reopen Geometry without reconstructing shape arithmetic or losing the subject identity.

## G5 — conformance and admission — current

Repository checks required before merge:

1. `ql-core` unit + integration tests, especially `geometry_constellation_resolution`;
2. `ql-mef` Second-Spanda conformance test;
3. existing structural/carrier/Vāk tests remain green;
4. fixture consumers accept matheme contract 1.2.0 and the additive shape Geometry extension;
5. no public API consumer breaks from the added `ConstellationGrain::PartialConjugate7` exhaustive match;
6. format/clippy/CI as required by the repository.

Any exhaustive-match break is fixed at its actual consumer. The new variant is not hidden behind `Other` merely to avoid propagation work.

## G6 — next research discriminators after this merge

These are now research on top of a complete canonical carrier, not missing base implementation:

- determine the exact operator interpretation of the `4×6` and `6×4` decadic cross-blocks;
- test how quaternionic transformation carries the `4^3` body into decadic/other dimensional presentations;
- exercise resolution switching in the live Wiki/Research Canvas/Expression UX so a `0/1` node can be opened through its preserved 2/3-fold basis and wider constellation;
- relate the 18→3 compression to the root `{0, /, 1} → 0/1` reading in actual Epii/agent reasoning traces without hard-coding semantic content into `ql-core`.

Those follow from this implementation. They do not block the Geometry base admitted by #204.
