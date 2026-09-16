# QL Wiki structural contract v2 — relation/conjugation reconciliation

**Contract version:** `ql:structural:2.0.0`  
**Scope:** content-free structure implementable locally by generic Wiki engines  
**Status:** normative for Wiki structural clients; does not rewrite historical musical derivations

## 1. Corrected structural algebra

The invariant within-face pair families are:

```text
A = {(0,1), (2,3), (4,5)}
B = {(1,2), (3,4), (5,0)}
C = {(0,5), (1,4), (2,3)}
```

Family and pair index are both part of operator identity. Therefore `A[1]=(2,3)` and `C[2]=(2,3)` may have the same vertices while remaining different structural fields.

Conjugation degree is a **separate axis**:

```text
D1  straight same-position conjugate opposition
    x ↔ x′

D2  one-sided expansion of a selected A/B/C pair
    {x,x′,y} OR {x,y,y′}

D3  both-sided expansion of a selected A/B/C pair
    {x,x′,y,y′}
```

D1 is therefore not an A/B/C family. A pair instance can disclose the two D1 oppositions of its endpoints, but the D1 operator identity is position-local and family-free.

All nine D3 fields are addressable as `A0..A2`, `B0..B2`, `C0..C2` under the versioned operator refs. Identical vertex sets do not collapse relation-family provenance.

## 2. Reconciliation with the current musical derivation

The preserved musical derivation v3 used the names differently:

```text
musical D1
    same-position cross n ↔ n′

musical D2
    cross-position cross with transform / require / complete subtypes
    n ↔ (n+1)′
    n ↔ (n-1)′
    n ↔ (5-n)′

musical D3
    A/B/C repeated internally on the primed helix
```

The old D1 and structural D1 are compatible at the underlying same-position conjugation relation.

The old D2 and D3 are **not equivalent** to structural-v2 D2 and D3:

- old D2 names a family of cross-index pairing operators;
- structural-v2 D2 names one-sided conjugate *expansion degree* of an already selected A/B/C pair;
- old D3 names helix-invariance / primed-face A/B/C pairing;
- structural-v2 D3 names both-sided conjugate *expansion degree* of an already selected A/B/C pair.

This is semantic overload in the historical naming, not a mathematical identity. The historical musical document remains evidence of that derivation and is not silently edited by this contract.

## 3. Supersession policy

For generic Wiki topology and future interoperable structural clients:

```text
ql:structural:2.0.0:conjugation:D1:position-N
ql:structural:2.0.0:field:FAMILY:PAIR:D2:left|right
ql:structural:2.0.0:field:FAMILY:PAIR:D3
```

are canonical.

Legacy musical labels such as `D2-transform`, `D2-require`, `D2-complete`, and musical `D3=primed A/B/C` are **not accepted as aliases** for the v2 structural operators. A caller that wants those musical operators must name the historical/musical operator explicitly under a future musical registry rather than relying on an ambiguous bare `D2`/`D3` token.

No v1 executable QL kernel operator is removed: Q1's existing `conjugate-address`, `complement-address`, and `classify-four-plus-two` remain unchanged. Structural v2 is independently versioned because this correction concerns a relation-field contract that Q1 did not previously expose.

## 4. Whole-anchor and constellation canon

The central `0/1` whole-anchor is a structural parent/ground and is **not a seventh positional member**. In L5.3 Geometry that same anchor is the **1-fold whole**: geometric fold-count and positional-member count are distinct readings of one QL object.

The representation permits arbitrary partial constellations up to six positions on two faces. Canonical named grains are:

```text
anchor-only      central 0/1 whole-anchor, zero positional members / Geometry 1-fold
2-fold           any two direct positional participations
3-fold 123       direct {1,2,3}
3-fold 450       direct {4,5,0}
4-fold 1234      direct {1,2,3,4}
4+1 ground       direct {0,1,2,3,4}
4+1 synthesis    direct {1,2,3,4,5}
6-fold           direct {0,1,2,3,4,5}
7-fold           all six direct + exactly 1 corresponding conjugate position
8-fold           all six direct + exactly 2 corresponding conjugate positions
9-fold           all six direct + exactly 3 corresponding conjugate positions
10-fold          all six direct + exactly 4 corresponding conjugate positions
11-fold          all six direct + exactly 5 corresponding conjugate positions
12-fold          all six direct + all six conjugates
```

Which conjugate positions are present remains explicit data. The names state the canonical fold/grain of the disclosed constellation; the actual coordinates remain the object.

A conjugate positional participation requires the same direct position to be present in this v2 constellation representation. Richer higher-order Geometry — including the `18 = 6 + 6′ + relational-6` closure and matrix/product forms — composes these constellations through the existing QL shape/carrier system rather than pretending eighteen positional members exist in one structural constellation.

## 5. Return canon

Return is a path through the whole-anchor, never shorthand `5 → 0`:

```text
source / #5-like completion
    → whole-anchor
    → explicit target #0 ground
```

The target ground retains:

- stable target ref;
- target position `#0`;
- direct or conjugate face;
- ground kind: own / parent / child / other / conjugate.

The anchor ref must match the containing constellation. This gives clients enough provenance to distinguish a return to the local ground from parent/child/other/conjugate return without inventing content semantics.

## 6. Dependency firewall

This contract is deterministic, content-free Rust in `ql-core`. Generic clients may implement it locally from the versioned fixture/schema and do **not** need a QL-MEF network call for ordinary Wiki correctness.

MEF lenses, harmonics, musical interval realisations, L5.3 Geometry compression/disclosure and semantic refraction remain QL-MEF-owned developed intelligence layered above this floor. The structural whole-anchor and constellation grains are the common carrier on which those readings operate.
