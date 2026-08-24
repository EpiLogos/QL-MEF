# QL-MEF holographic kernel orientation

**Status:** R4 kernel-core development orientation  
**Kernel Wayfinder:** #78  
**Programme:** #51  
**Implementation:** #56 / PR #76  
**Formal/harmonic continuation:** #31 / #39 / PR #19  
**Frozen Epi C source:** `daa660cbc1b8c5da83828698665a753852cb0287`

## 0. What this work is

QL-MEF is nativeising the **existing Epi/QL holographic kernel**.

The foundational system is already authored and implemented in the historical Epi corpus. R4 is not designing a replacement theory and is not assembling a new kernel from independently promoted helper functions.

The source-shaped implementation has already been copied and locked under:

```text
vendor/epi-kernel/reference/
```

The native task is:

```text
existing determined kernel
        ↓ preserve data / law / provenance / parity
QL-MEF-native addresses, refs and operators
        ↓ refactor historical coupling where useful
one coherent kernel core
```

Correct source data, finite relation structures, LUTs and mature algorithms remain ground. Pointer layout, package residency and duplicated helper structure may change when QL-MEF/O:I primitives give the same system a cleaner native form.

## 1. The tap-root

The kernel begins before M-domain semantics and before the O:I product reflection:

```text
# / 0/1 <-> 1/0
        ↓
#0 #1 #2 #3 #4 #5
        ↓
C / P / L / S / T / M
        ↓
direct / prime-conjugate face
        ↓
harmonic / relational field
        ↓
P/P' + L/L'
        ↓
MEF / Context Frames / musical derivation
        ↓
bioquaternion / epogdoon / tick / resonance / energy dynamics
```

Everything larger in Epi or O:I addresses, refracts, composes or embodies this field.

### Raw bedrock

`Psychoid_Hash` and `Psychoid_0..Psychoid_5` are the pre-family substrate. In the historical C ontology the six positions use `FAMILY_NONE`.

The native kernel must therefore make `#` and `#0..#5` explicit rather than deriving QL identity from M1, M data or a consumer-specific projection.

### Six coordinate families

The historical coordinate-family order is:

```text
#0  C
#1  P
#2  L
#3  S
#4  T
#5  M
```

Each family manifests the same six positional bedrock values. The historical arena instantiates 6 × 6 direct family coordinates and cross-links same-position members across the six families.

A native kernel address therefore has at least:

```text
position    #0..#5
family      NONE | C | P | L | S | T | M
face        direct | prime
```

The precise native type names are implementation work. The structure is not.

## 2. `#`, conjugacy and complement

The prime face preserves coordinate index:

```text
X_i <-> X_i'
```

The historical implementation already distinguishes this from positional mirror/complement:

```text
conjugacy / inversion-Spanda
    X_i <-> X_i'

mirror / complement
    i <-> 5-i

cross-face completion
    X_i <-> X_(5-i)'
```

These relations may compose. They are not aliases.

P/P' and L/L' are the most-developed explicit expressions of the general two-faced coordinate system. Their development does not restrict prime capacity to only P and L.

At the mature computational level the conjugate/slash reflection is expressed by quaternion conjugation:

```text
q -> q*
```

That operation is a computational realisation inside the larger `#` relation; it does not redefine the root as a scalar position permutation.

## 3. Harmonic relation grammar

The six positions carry the established finite pairing families:

```text
A = (0,1) (2,3) (4,5)
B = (1,2) (3,4) (5,0)
C = (0,5) (1,4) (2,3)
```

Across the direct/prime faces the developed musical grammar includes:

```text
same-position cross
    n <-> n'

transform
    n <-> (n+1)'

require
    n <-> (n-1)'

complete
    n <-> (5-n)'

conjugate-face invariance
    A / B / C reproduced on the prime face
```

PR #19 already implements these relations in the Rust QL core. #39 owns durable operator naming/provenance where historical software and musical `D1/D2/D3` vocabularies overlap.

R4 must expose the C/kernel relation field in a form that can resolve the same operators. It must not invent a competing relation vocabulary.

## 4. P/P', L/L' and MEF

### P/P'

P carries the six-position direct/prime traversal and its developed Torus/Klein topology.

The 12-state kernel cycle is therefore read through:

```text
P0..P5
P0'..P5'
```

without changing the six-position identity merely to mark the prime face.

### L/L'

The MEF lens field carries:

```text
L0..L5
L0'..L5'
```

with six local QL positions per lens:

```text
12 × 6 = 72
```

Same-index Day/Night lens twins, same-face complement and cross-face Möbius partnership remain distinct available relations.

### Context Frames

The accepted seven Context Frames compose over the same QL/MEF field. R4 does not create another CF engine. The C kernel needs stable CF addresses/refs where its historical VĀK/context machinery touches the same system; the typed Rust CF grammar in PR #19 supplies the current formal implementation line.

## 5. The historical C body is the specimen, not the target layout

The frozen source includes the whole foundational implementation body, especially:

```text
ontology.h
psychoid_numbers.h
families.c
pointer_web.h / pointer_web.c
VĀK / Context-Frame material
kernel.h / kernel.c
shared numeric/topological helpers
M0..M5 consumers and source data
```

`families` and `pointer_web` are important because they show the source kernel as one relation field: family, position, direct/prime helix, lens, relation role, harmonic interval/ratio and Context-Frame relation are already connected.

Native QL-MEF does not need to preserve every historical pointer mechanism in order to preserve that field.

Use the existing QL-MEF concepts where they fit:

```text
stable address / Ref
stable operator / relation identity
source + version provenance
explicit readiness / evidence
```

Retain historical 128-byte coordinate/tagged-pointer representation where it provides ABI/parity value. Put the durable semantic/addressing contract above or alongside it so callers do not depend on raw pointer layout as the meaning of the kernel.

## 6. Role of `ql-c/primitive`

PR #59 / completed #54 produced valid parity-proven helper operations.

They are **supporting implementation primitives inside the kernel**:

```text
finite position/ring/state operations
resonance addressing
harmonic ratios
tick helpers
```

They do not replace:

```text
# / #0..#5
family identity
direct/prime face
harmonic relation field
MEF / CF
holographic coordinate structure
```

R4 may freely consume the helpers where they implement the same law.

## 7. C and Rust are two useful native bodies

QL-MEF already contains real native Rust QL/MEF work. The target is not to translate mature C computation into Rust or mirror every Rust service type in C.

The shared contract is the formal identity of the kernel objects both sides touch:

```text
position
family
face
relation/operator
pair/square/cross relation
lens/MEF address
Context-Frame address
source/provenance
```

Cross-language fixtures should prove agreement where both bodies represent the same object or relation.

Then responsibilities can remain natural:

```text
C
    mature numerical/harmonic/bioquaternion/tick/resonance/energy body

Rust
    native structural Ref/operator/MEF/CF/query/service body
```

That split may evolve from evidence; it is not a requirement for artificial symmetry.

## 8. M/Epi and S/O:I grow from the kernel

### M / Epi

The M family is one coordinate family in the kernel. Epi gives M/M' its rich semantic/genealogical content and deep instruments.

QL-MEF owning the generalized kernel does not move Epi semantic authority.

### S / O:I

The S family is likewise part of the kernel before the current O:I implementation existed. The current O:I six-product field is the developed technological reflection carried through S under the current constitutional mapping.

QL-MEF at S5 is the recursive/formal product centre in that O:I field; that downstream embodiment does not make S the source of QL.

Capability/praxis relation work can later use the same kernel grammar to inspect real product/capability relations. It remains downstream of the stable kernel core.

## 9. Source and theory references

The small foundational reference bundle should be versioned and easy to resolve from QL-MEF.

Start from the sources already used by the implementation:

```text
epi_logos_coordinate_system.md
mef-12-lenses-sublens-reference.md
ql-musical-derivation-v3.md
```

plus the locked executable C source.

Keep exact upstream revision/hash and status. Richer essay/research material enters when the development edge requires it; the basic kernel does not wait for a complete research-library import.

## 10. R4 completion

PR #76 completes this orientation when the native kernel can be traversed coherently as:

```text
# / raw position
    -> family
    -> face
    -> relation
    -> P/L/MEF/CF expression
    -> mature computational operation
    -> source/provenance
```

and the equivalent Rust/C identities agree through fixtures.

After that:

```text
#57
    rebase real Epi consumers

#31 / #39 / PR #19
    continue richer harmonic/MEF/music development

Factory/O:I capability work
    consume the stable relation grammar downstream
```

The centre is stable enough to grow outward when implementation reflects this form. Future theory work extends the edges; it does not repeatedly recreate the tap-root.