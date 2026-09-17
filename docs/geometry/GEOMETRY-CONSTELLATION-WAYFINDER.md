# Geometry constellation completion Wayfinder

Status: **#204–#207 and #222 landed; canonical Geometry plus Vāk/Wiki runtime propagation complete; source-grounded handoff closed 2026-09-17**  
Landed via: `#208` and `#223`  
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

The existing direct/conjugate relation system exposes:

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

The source-grounded closeout adds two discriminations without changing code:

- `4×6` and `6×4` are the two directional mixed-axis relation/refraction fields between the established quaternary/form and senary/process axes; their 24-address cardinality does not mint a new 24-element ontology;
- the M2→M3 dimensional seam is already authored as four persistent carriers with internal `18 = 3×3×2 → 16 = 4×4` epogdoon transduction. M3's quaternionic/SU(2) orientation and the decadic matrix are related views, not an asserted shape isomorphism.

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

That is the shared carrier floor. Downstream runtime propagation is separately proven by G4b rather than inferred from the carrier alone.

## G4b — Wiki runtime propagation — #222/#223 — implemented

The post-#208 consumer audit found that `crates/ql-wiki` still validated pair/D1/D2/D3 structural fields directly while its runtime `WikiReading` did not retain the newly canonical `ShapeBinding` shape/basis/derivation metadata. #223 closes that gap through the existing Wiki extension/refraction path.

The landed route is deliberately thin:

```text
QlShape::shape_ref
  <-> ql-core canonical shape-ref resolver
        -> Wiki ShapeBinding transport view
           -> existing Wiki target extension channel
              -> existing Wiki refraction engine
                 -> shape-aware Wiki reading
                    {ql_form_refs, operator_refs, exact shape-binding extension}
```

Rules now enforced in runtime:

- `ql-core` is the only owner that resolves versioned shape refs; Wiki does not parse shape strings locally;
- Wiki `ParticipationForm` remains its source-participation vocabulary and is not identified with `QlShape`;
- the Wiki transport view carries subject/whole identity, basis refs, members, derivation, operator, Return refs and caller provenance, while semantic relation assertions continue through Wiki's existing relation structures;
- compressed one-fold readings are admitted only when their retained member body reconstructs a lawful two- or three-fold `StructuralConstellation` and the existing `QlShapeCompression` produces the exact operator/derivation supplied;
- ordinary Wiki refraction with no shape extension remains byte/structure compatible with the existing contract;
- no new service, source registry, Geometry owner or provider semantics are introduced.

## G5 — conformance and admission — complete

#223 closes against the intended repository checks:

1. canonical shape refs round-trip through the kernel resolver, while aliases/unknown versions/anonymous grains fail closed;
2. a core `ShapeBinding` can project into the Wiki transport view without changing subject, whole, basis, member, derivation, operator, Return or provenance identity;
3. a compressed threefold→onefold view validates through the actual kernel compression law and is carried into every shape-aware Wiki reading;
4. invalid shape refs and false compression derivations fail before provider execution;
5. ordinary Wiki refraction remains unchanged when no shape binding is present;
6. existing structural/carrier/Vāk/Wiki tests remain green;
7. current-source census records the new resolver as part of the existing Geometry owner and the Wiki layer as an infrastructural consumer;
8. format/clippy/CI pass with no historical K4 proof rewritten.

## G6 — source-grounded closure and downstream handoff — closed here

The four items previously listed as open Geometry research are resolved differently after tracing the authored Bimba/Spanda field and the Antykathera etymological archaeology. They are **not reasons to reopen L5.3 Geometry**.

### G6.1 — `4×6 / 6×4`: office resolved, semantic cells remain source-owned

The Spanda/Bimba corpus already establishes the 4/6 dual-track expansion, while Second Spanda fixes the quaternary `64 = 4³` and senary `36 = 6²` bodies. In the decadic block matrix:

```text
4×6  = form/quaternary → process/senary directional field
6×4  = process/senary → form/quaternary directional field
```

The two 24-address blocks are therefore mixed-axis relation/refraction fields. The exact meaning of a cell is not a missing kernel theorem: it comes from the actual row/column subjects, typed relation, source standing and consumer context. Runtime embodiment belongs to **#214 M1′ Canvas/Constellation** and **#215 M2′ Relation Field/Timeline**, with #219 carrying the Expression cross-cut.

### G6.2 — quaternionic/dimensional relation: already grounded, do not assert an isomorphism

`FOLD-AND-RULING-GRAMMAR.md` already supplies the dimensional passage:

```text
72 = 4×18
      ↓ 8/9
64 = 4×16
```

The four carriers persist while the internal Paraśakti ternary/conjugate organisation `18 = 3×3×2` becomes the Mahāmāyā quaternary body `16 = 4×4`. M3's receiving/orientation field is already quaternionic/SU(2). The `10×10` decadic matrix is a two-dimensional refraction preserving `64|36`; it is not declared topologically identical to `4×4×4`.

The remaining obligation is presentation/conformance through the existing instruments, not discovery of another Geometry model.

### G6.3 — live resolution switching: existing product acceptance, not Geometry research

The shared carrier already permits:

```text
presented one-fold 0/1
  ↕ reopen retained basis
2/3-fold disclosed constellation
  ↕ bounded expansion
wider source-qualified constellation / relation field
```

This is now an explicit acceptance inheritance for:

- **#214 M1′ Canvas / Constellation** — semantic zoom and bounded expansion over exact `ShapeBinding` refs;
- **#219 X3** — M1′ Canvas ↔ 3:3 Expression topology/geometry switching without changing subject/source/constellation identity.

UI opening/closing changes disclosure only. It does not mutate semantic membership merely because nodes are visually expanded or collapsed.

### G6.4 — `18→3→1`: semantic exemplar already authored; real Epii proof belongs downstream

The Antykathera arbitration archaeology supplies the concrete earned field:

```text
6:   Continuity → Criterion → Delineation → Arbitration → Con-text → Resolution
6′:  Indeterminacy → Distinction → Difference → Crisis → Diaphaneity → Reconciliation

6+6′ generated slash:
Origin → Measure/Limit → Perspective/Frame → Decision/Hybris → Regard → Anamnesis/Return
```

Its own form-growth grammar then compresses:

```text
6       :      6+6′      :       6′
0       :        /       :        1
              ↓
             0/1
```

That is the semantic exemplar for the executable composite route:

```text
18-fold
  ↓ EighteenFoldGeometry 18→3
{6, relation-6, 6′}
  ↓ QlShapeCompression 3→1
0/1
```

The kernel must **not** hard-code `Arbitration`, `Crisis`, `Hybris`, `Regard` or any other source vocabulary. It carries basis and reversible compression. The source-qualified Wiki/Vāk/Epii reading supplies the semantic field.

A real Agent/Epii proof is therefore inherited by **#201 Ta-Onta / Expression SDK**: use an actual Bimba/source-qualified whole, preserve the 18 lower determinations through `18→3→1`, reason or disclose at one resolution, reopen it, and Return without source/subject drift.

### G6.5 — closure boundary

No new Geometry ticket is created by this closeout.

```text
L5.3 Geometry
  CLOSED: canonical shapes, Second Spanda, 18→3, 2/3→1,
          Vāk/Wiki transport and source-grounded interpretation boundary

#214 / #215
  ACTIVE: constructive and relation-field embodiment

#219
  ACTIVE: live 3:3 / 4:2 resolution and Expression cross-cut

#201
  ACTIVE: Bimba/M/M′/Epii source-qualified semantic inhabitation
```

If later evidence changes the authored Spanda, Bimba or etymological relation, revise that source explicitly and then reconcile Geometry. Do not keep a permanent miscellaneous G6 bucket around as a substitute for provenance.
