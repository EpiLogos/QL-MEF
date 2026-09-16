# Geometry constellation completion Wayfinder

Status: **#204–#207 landed; runtime consumer completion active as #222, 2026-09-16**  
Branch: `agent/geometry-g4-wiki-shape-propagation`  
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

## G4 — shared Vāk propagation — #207 — implemented

No new Vāk payload is introduced. Existing consumer flow is retained:

```text
QlShape
  -> ShapeBinding
     {shape_ref, basis_refs, derivation_ref, operator_ref, ...}
  -> Vāk Whole
  -> FramedReading
     {binding, GeometryReading, ...}
```

Acceptance coverage proves that a one-fold `0/1` binding can retain a threefold basis and the exact `3→1` compression derivation/operator through the existing `ShapeBinding` contract. Explicit member focus reopens the retained body while the default read presents the whole anchor.

That is the shared carrier floor. It does not by itself prove that every downstream product surface has wired the new fields into its own runtime response.

## G4b — Wiki runtime propagation — #222 — implementation active

The post-merge consumer audit found that `crates/ql-wiki` still validated pair/D1/D2/D3 structural fields directly while its runtime `WikiReading` did not retain the newly canonical `ShapeBinding` shape/basis/derivation metadata. The prior Wiki contract-document update therefore described a valid shared seam but did not yet constitute runtime propagation.

The completion route is deliberately thin:

```text
QlShape::shape_ref
  <-> ql-core canonical shape-ref resolver
        -> Wiki ShapeBinding transport view
           -> existing Wiki target extension channel
              -> existing Wiki refraction engine
                 -> shape-aware Wiki reading
                    {ql_form_refs, operator_refs, exact shape-binding extension}
```

Rules:

- `ql-core` is the only owner that resolves versioned shape refs; Wiki does not parse shape strings locally;
- Wiki `ParticipationForm` remains its source-participation vocabulary and is not identified with `QlShape`;
- the Wiki transport view carries subject/whole identity, basis refs, members, derivation, operator, Return refs and caller provenance, while semantic relation assertions continue through Wiki's existing relation structures;
- compressed one-fold readings are admitted only when their retained member body reconstructs a lawful two- or three-fold `StructuralConstellation` and the existing `QlShapeCompression` produces the exact operator/derivation supplied;
- ordinary Wiki refraction with no shape extension remains byte/structure compatible with the existing contract;
- no new service, source registry, Geometry owner or provider semantics are introduced.

## G5 — conformance and admission — current for #222

Repository checks required before the G4b merge:

1. canonical shape refs round-trip through the kernel resolver, while aliases/unknown versions/anonymous grains fail closed;
2. a core `ShapeBinding` can project into the Wiki transport view without changing subject, whole, basis, member, derivation, operator, Return or provenance identity;
3. a compressed threefold→onefold view validates through the actual kernel compression law and is carried into every shape-aware Wiki reading;
4. invalid shape refs and false compression derivations fail before provider execution;
5. ordinary Wiki refraction remains unchanged when no shape binding is present;
6. existing structural/carrier/Vāk/Wiki tests remain green;
7. current-source census records the new resolver as part of the existing Geometry owner and the Wiki layer as an infrastructural consumer;
8. format/clippy/CI pass with no historical K4 proof rewritten.

## G6 — next research discriminators after this merge

These are research on top of the canonical carrier and its first runtime consumers, not missing base implementation:

- determine the exact operator interpretation of the `4×6` and `6×4` decadic cross-blocks;
- test how quaternionic transformation carries the `4^3` body into decadic/other dimensional presentations;
- exercise resolution switching in the live Research Canvas/Expression UX so a `0/1` node can be opened through its preserved 2/3-fold basis and wider constellation;
- relate the 18→3 compression to the root `{0, /, 1} → 0/1` reading in actual Epii/agent reasoning traces without hard-coding semantic content into `ql-core`.

Those follow from the admitted Geometry system and its consumer contract. They do not redefine the L5.3 owner.
