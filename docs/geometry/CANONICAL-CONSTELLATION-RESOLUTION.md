# Canonical constellation resolution — L5.3 Geometry

Status: **canonical architecture and executable Geometry contract extension, 2026-09-16**  
Programme: #204, with implementation tranches #205–#207.  
Owner: the existing QL Kernel / L5.3 **Geometry** office. This document does not create a new layer, service, ontology, or replacement constellation system.

Read with:

- `docs/QL-VAK-KERNEL-RECONCILIATION.md` — L5.3 Geometry is the formal/topological disclosure office of the one QL Kernel;
- `docs/wiki-structural-contract-v2.md` — structural whole-anchor, constellation grains and D1/D2/D3 carrier floor;
- `docs/geometry/FOLD-AND-RULING-GRAMMAR.md` — geometric and M1→M2→M3 research foundation;
- `docs/origami work/M1/M1-PARAMASIVA-DEEP-CAPABILITY-COORDINATE-MATRIX.md` — authored M1/Spanda canon, including `2^6 = 4^3 = 64`;
- `fixtures/kernel/ql-shape-contract-v1.json` and `fixtures/kernel/matheme-derivation-contract-v1.tsv` — portable executable boundary.

## 1. One existing Geometry system

The QL-MEF product already has the six L5 offices:

```text
L5.0  Syntax
L5.1  Root
L5.2  Harmonics
L5.3  Geometry
L5.4  Meta Epistemic Framework
L5.5  Techne
```

Geometry is where QL relations become partial constellations, quaternities, matrix/product fields, folds, circles, torus/Klein/Möbius/wave forms and renderable topology. The work here completes that existing office so its executable shape vocabulary matches the QL forms already present across StructuralConstellation, Spanda, the M2 Paraśakti field and the M3 Mahāmāyā fold body.

The central distinction is between **structural membership** and **geometric resolution**. A structural constellation counts actual positional participations. Geometry asks what fold or higher-order form that disclosed whole is presently showing.

Thus the central `0/1` whole-anchor remains zero positional members in structural-v2 while being the **1-fold QL whole** in Geometry. This is not an extra position. It is one whole capable of carrying and re-disclosing its basis.

## 2. The fold/compression ladder

The canonical local fold ladder is:

```text
1-fold   = 0/1 whole-anchor
2-fold   = elementary differentiated relation
3-fold   = triadic articulation: 123 or 450
4-fold   = 123 + its recognised fourth / existing 1234 quaternity
4+1      = fivefold ground or synthesis forms
4+2      = sixfold base-frame
7-fold   = six direct + one corresponding conjugate
8-fold   = six direct + two corresponding conjugates
9-fold   = six direct + three corresponding conjugates
10-fold  = six direct + four corresponding conjugates
11-fold  = six direct + five corresponding conjugates
12-fold  = six direct + all six conjugates
```

The 7-fold is therefore not an unnamed `Other` shape. It completes the already-existing conjugate expansion sequence 7→12 in `ConstellationGrain`.

### 2.1 `3:1` is the compression/recognition law

A QL `0/1` node may present as one fold while retaining a disclosed two- or three-fold basis. The point is not that the basis disappears; the one-fold is the recognised whole of that basis and can reopen it.

For the canonical 123 triad:

```text
a  b  c                  disclosed 3-fold
 \ | /
  [abc]                   recognised one-fold 0/1

{a,b,c,[abc]}             existing 4-fold 1234 recognition superset
```

This is the operational significance of `3:1`: three differentiated determinations can be carried by one recognised `0/1` whole. The existing `ShapeBinding` is already capable of preserving this relation: the presented `shape_ref` can be the one-fold anchor, `basis_refs` retain the disclosed participants, and `derivation_ref` + `operator_ref` retain the compression operation. `FramedReading` returns the full binding unchanged, so Vāk/Wiki consumers do not need another resolution schema.

The same one-fold presentation can retain a two-fold basis. The present implementation admits 2→1 and 3→1 compression and names the `3→1` recognition operator explicitly. Further compression laws are built from the same shape system rather than a separate tree format.

## 3. The 18-fold closure and recursive return

The existing `RelationalSixfold` already gives the six same-position relation sites between the direct and conjugate sixfolds:

```text
6        direct basis
6′       conjugate basis
6↔6′     six same-position relation sites
```

Geometry therefore has the higher-order form:

```text
6 + 6′ + relational-6 = 18
18 / 6 = 3
```

The compressed threefold is the triad of **direct sixfold / conjugate sixfold / their relation-sixfold**. The implementation retains the three basis refs and names the 18→3 compression operator. The structural constellation remains capped at the real twelve positional members; the 18-fold is a higher-order `QlShape`, not eighteen invented positions.

This gives the recursive geometric route behind the larger constellation family:

```text
18-fold
  ↓ divide by its sixfold grain
3-fold of {6, 6′, 6↔6′}
  ↓ recognition/compression
1-fold 0/1
```

At the deepest authored reading the triadic relation is again `0 / 1`: source, relation, determination recognised as one `0/1` event. The code in this tranche establishes the generic geometric compression carriers; semantic use of that root triad remains supplied by the QL/Vāk context that invokes it.

## 4. Second Spanda — canonical dual Geometry

The canonical Second Spanda was already authored as:

```text
100% = 2^6 + 6^2
     = 64 + 36
```

and the M1 QL flowering source already carries the identity:

```text
2^6 = 4^3 = 64
```

The executable matheme previously retained only the `2^6` reading. Geometry now restores the complete form:

```text
100%
 = 2^6 + 6^2
 = 4^3 + 6^2
 = 64 + 36
 = (4×4×4) + (6×6)
```

These two sides already have concrete owners.

### 4.1 M3 / Mahāmāyā — `4×4×4 = 2^6 = 64`

The current M3 fold body is three articulated sites. Each site carries two binary properties — polarity and mobility — and therefore has four states. The existing `FoldMotif`/`Codon64` is exactly:

```text
3 sites × 2 binary properties
= 6 bits
= 4^3
= 2^6
= 64 canonical form states
```

`FourByFourByFourField::canonical()` derives its cardinality from `FoldMotif::SITES` and proves equality with `Codon64::COUNT`. The 64 side is therefore not an abstract integer register placed beside M3; it is the existing M3 form body read geometrically.

### 4.2 M2 / Paraśakti — `6×6 = 36`

The 36 side remains the existing `SixBySixField`: six direct positions against six conjugate positions. Second Spanda therefore couples an M3 cubic/quaternary field and an M2 senary relational field without flattening their different dimensional forms.

The existing M2→M3 seam remains:

```text
M2: 72 = 4×18 = 4×(3×3×2)
             ↓ 8/9
M3: 64 = 4×16 = 4×(4×4)
```

and Third Spanda remains the later development:

```text
64 + 72 + 1 = 137
```

where the M2 36 field has entered its doubled/conjugate 72 reading while the M3 64 form field remains the receiving register.

## 5. The decadic `10×10` projection

The full decadic field is now an explicit Geometry projection of Second Spanda:

```text
(4+6)^2
= 4^2 + 4×6 + 6×4 + 6^2
= 16 + 24 + 24 + 36
= 100
```

The first three blocks preserve the 64 side:

```text
16 + 24 + 24 = 64
```

and the final `6×6` block remains 36. Therefore:

```text
10×10  ->  64 | 36
```

is executable as a cardinal/shape projection of the same Second-Spanda totality.

The canonical body is still `4×4×4 | 6×6`. The `10×10` field is a derived whole-field view through the `4|6` split. The live research question is now finer: what exact QL transformations inhabit the `4×6` and `6×4` cross-blocks, and how do those transforms correspond to the dimensional change between the cubic 64 field and its decadic projection? The implementation makes the projection and its partition first-class so that question can be tested without confusing it with the already-settled equation.

## 6. Propagation to existing consumers

No consumer receives a new ontology.

```text
StructuralConstellation
    ↓ grain()
QlShape / Geometry
    ↓ ShapeBinding
RelationFieldComposition / Vāk Whole
    ↓ FramedReading.binding + GeometryReading.shape_ref
Wiki / refraction / Epii / Expression consumers
```

The propagation rules are:

- `StructuralConstellation::grain()` now names the 7-fold directly;
- `QlShape::fold_count()` supplies the geometric fold reading, including anchor-as-1-fold;
- `QlShapeCompression` supplies reversible 2→1 and 3→1 derivation/operator refs;
- `ShapeBinding` carries the presented shape plus actual basis/derivation/operator provenance;
- `FramedReading` already returns that whole binding, so compression metadata survives Vāk composition and downstream refraction;
- `FourByFourByFourField`, `SixBySixField`, `EighteenFoldGeometry`, `TenByTenProjection` and `SecondSpandaGeometry` are all `ql-core` Geometry types consumed by the existing QL-MEF matheme and instruments;
- the matheme's exported `binary_register`, `quaternary_cubic_register`, `self_register`, `decomposed_totality` and `decadic_projection` now expose the complete Second-Spanda accounting.

Consumers should therefore request or retain the `QlShape`/binding they are given rather than rebuild fold arithmetic locally. The Wiki/Research-Canvas/Expression surfaces can choose a resolution — one-fold, disclosed constellation, relation field or higher Geometry — while retaining the same subject and derivation.

## 7. Executable acceptance

The tranche is accepted when the repository proves all of the following:

```text
anchor-only -> Geometry fold 1
6 direct + 1 conjugate -> named fold 7
6 direct + 6 conjugate -> fold 12
2-fold -> presented 0/1 with compression provenance
3-fold 123 -> presented 0/1 + 1234 recognition superset
4×4×4 -> 64 and equals the existing M3 Codon64 count
6×6 -> 36
4×4×4 + 6×6 -> 100
10×10 blocks -> [16,24,24,36] -> 64|36 -> 100
6 + 6′ + relational-6 -> 18 -> compressed fold 3
matheme binary register == quaternary cubic register == 64
ShapeBinding preserves disclosed basis + compression derivation/operator
```

That is the Geometry completion required by #204. Deeper research can now operate on actual canonical shapes rather than reconstructing these relations ad hoc in each consumer.
