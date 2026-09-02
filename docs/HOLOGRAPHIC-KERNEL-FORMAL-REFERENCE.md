# Holographic Kernel Formal Reference

Status: **accepted foundational kernel contract**  
Contract: `ql.holographic-kernel-contract/v1` / semantic version `1.1.0`  
Machine-readable companion: `fixtures/kernel/holographic-kernel-contract-v1.tsv`

This reference names the shared formal identities currently implemented by the native C holographic kernel and the Rust QL/MEF body. It is the small agent-resolvable reference for the foundational field; richer musical, refractive and semantic development continues under #31 from these identities.

## Provenance

The historical computational specimen is frozen from:

- repository: `EpiLogos/Epi-Logos-C-Experiments`
- revision: `daa660cbc1b8c5da83828698665a753852cb0287`
- `pointer_web.c` blob: `3eeae6f9c8cc65c5a610df1a49143b3c65bdd320`

The shared contract retains those refs directly. Source layout is implementation evidence; semantic identity is carried by the native kernel contract and parity tests.

## One kernel field

The foundational composition is:

```text
# / 0/1 <-> 1/0
        ↓
#0 #1 #2 #3 #4 #5
        ↓
C / P / L / S / T / M
        ↓
direct / prime-conjugate
        ↓
harmonic relation field
        ↓
P/P' + L/L'
        ↓
MEF / Context Frames
```

`NONE` identifies Hash/raw psychoid bedrock before a C/P/L/S/T/M family manifestation. Position and face remain explicit dimensions of the same field.

Same-position direct/prime conjugacy is `n <-> n'`. Mirror/complement is `n <-> 5-n` on the same face. Cross-complete is `n <-> (5-n)'`. Those relations remain separately addressable even when they share positional vertices.

## A/B/C and the 3×3 square apparatus

The three canonical pair families are:

```text
A = (0,1) (2,3) (4,5)
B = (1,2) (3,4) (5,0)
C = (0,5) (1,4) (2,3)
```

Family × pair index gives the nine canonical square entries. Current executable conformance preserves:

```text
9 independently addressable entries
8 oriented structures
7 unordered address tetrads
```

Coincident vertex sets do not collapse family, pair-index or orientation provenance.

### D1 / D2 / D3

Within one selected A/B/C pair, D1/D2/D3 are the square-completion coordinates:

```text
D1  selected direct pair                    2 coordinates
D2  one-sided conjugate expansion           3 coordinates
D3  complete conjugate square               4 coordinates
```

The retained cross derivation also uses D1/D2/D3 as derivational coordinates. Public executable identity is therefore carried by the semantic kernel relation ID, while the exact D-coordinate remains provenance.

Rust exposes this directly:

```text
CanonicalCrossPass::operator_ref()    -> semantic kernel relation ID
CanonicalCrossPass::derivation_ref()  -> exact D1/D2/D3 derivation ref
```

The canonical cross relation identities are:

```text
ql.kernel.cross.same-position/v1
ql.kernel.cross.transform/v1
ql.kernel.cross.require/v1
ql.kernel.cross.complete/v1
ql.kernel.conjugate-invariance.A/v1
ql.kernel.conjugate-invariance.B/v1
ql.kernel.conjugate-invariance.C/v1
```

The corresponding positional laws are:

```text
same-position  n <-> n'
transform      n <-> (n+1)'
require        n <-> (n-1)'
complete       n <-> (5-n)'
```

Pair C, mirror/complement and cross-complete retain distinct operator identities and derivation provenance.

## VĀK reflective language

VĀK is the universal six-family reflective instruction language of the frozen kernel:

```text
0 CPF  Category-Position-Frame
1 CT   Context-Time / Content Types
2 CP   Context-Position
3 CF   Context-Frame
4 CFP  Context-Frame-Position / Paths
5 CS   Context-Sequence
```

The instruction has five source-defined fields:

```text
vak_family
vak_index
target_branch
target_pos
is_inverted
```

`is_inverted` carries the direct/prime dimension of the VĀK instruction.

The frozen M0 implementation provides concrete specialisations for all six families:

```text
CPF  discrimination / inversion
CT   QL-frame selection
CP   void-arithmetic position anchor
CF   Context-Frame / Vimarsa invocation
CFP  R-factor thread
CS   Logos-cycle completion
```

Historical arena pointer materialisation is a separate implementation relation. In the frozen `families_wire_reflective()` path, `cf` and `cs` are materialised while `cpf`, `ct`, `cp` and `cfp` are not materialised as arena pointer slots. This does not reduce the six-family VĀK language or its registered semantic handlers.

## One Context-Frame identity

VĀK `CF` is the reflective operation over the canonical Context-Frame system. It does not introduce another Context-Frame ontology.

The shared semantic identity is:

```text
ql.kernel.context-frame/v1
```

C expresses this with:

```text
QL_KERNEL_REL_VAK_CF == QL_KERNEL_REL_CONTEXT_FRAME
```

Rust expresses it with:

```text
VakFamily::Cf.relation_id() == KernelRelationId::ContextFrame
```

The canonical seven frames are:

```text
CF1  (00/00)
CF2  (0/1)
CF3  (0/1/2)
CF4  (0/1/2/3)
CF5  (4.0/1-4.4/5)
CF6  (4.5/0)
CF7  (5/0)
```

They resolve over the same 12-lens MEF field: six local positions per lens, 72 addresses, with absolute position `(lens + local) mod 6`.

## Conformance authority

The machine-readable contract and executable tests jointly determine implementation conformance:

- `fixtures/kernel/holographic-kernel-contract-v1.tsv`
- `crates/ql-core/tests/pairing_grammar.rs`
- `crates/ql-mef/tests/holographic_kernel_contract.rs`
- `migration/epi-kernel/r4-holographic-kernel-parity.c`
- `migration/epi-kernel/r4-vak-parity.c`

The contract preserves source provenance, shared family/face/relation identities, VĀK semantics versus pointer-web materialisation, A/B/C and D-degree definitions, MEF cardinality and the seven Context Frames.

Richer musical ratios, modes, voice-leading, refractive consequences and later research extensions belong to the continuing #31 development edge and must derive from this field without minting a second positional, conjugation, square, VĀK, MEF or Context-Frame substrate.
