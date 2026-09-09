# QL structural carrier contract v1

This contract is the smallest portable carrier needed for external systems to use QL form without turning QL into an owner of their semantic objects.

The whole remains the **QL Kernel**. `QlShape` is the existing QL-owned ShapeDefinition. The carrier added here does not introduce a second shape ontology, a new semantic layer, or a substitute Vāk grammar.

## What is ratified from the current kernel

The current kernel already provides the load-bearing form law this carrier depends on:

- `StructuralConstellation` admits the whole anchor and positive disclosed grains: twofold, the current developed threefold/fourfold and 4+1 forms, sixfold, progressive conjugation, and twelvefold.
- `RelationFamily` plus D1/D2/D3 preserves route provenance; equal vertex sets do not erase A/B/C or pair identity.
- `FourByFourField` is the special D3 4×4 completion.
- `SixBySixField` is the special direct-sixfold × conjugate-sixfold 6×6 field.
- `RelationalSixfold` preserves the `6 / 6′ -> 6+6′` operator, basis refs and Return through `0/1`.
- `QlShapeAddress` is already an address at which a relation may be considered; it does not assert semantic content.

These remain canonical. The carrier composes them; it does not reinterpret them.

## Additive carrier seam

`ShapeBinding` is caller-owned attribution around QL form. It carries `subject_ref`, `shape_ref`, `whole_ref`, basis/member bindings, optional derivation/operator refs, Return refs, relation bindings, and opaque caller/source/standing provenance. Binding a subject does not grant QL authority over that subject and does not promote the caller's claim into QL-owned semantics.

`RelationFieldComposition` composes two actual `StructuralConstellation` wholes into a row-major address field. There is deliberately no constructor from numeric dimensions. A 3×4 or 6×12 cardinality is therefore a consequence of the two disclosed axes, never the reason they count as QL wholes.

The generic field has its own carrier ref. In particular, a generic 6×6 cannot claim `ql:shape:1.0.0:6x6:direct-conjugate`; that ref remains reserved for the canonical direct/conjugate field.

Addresses are generated deterministically from canonicalised coordinates, not from external labels. They carry no generated semantic cell. External semantic determinations arrive only as `ShapeRelationBinding` values with caller-supplied relation and evidence refs. Missing addresses, one-way/asymmetric determinations and multiple determinations at one address remain valid.

`RelationFieldDerivation` retains both source whole refs, source QL shape refs, grains, the composition operator, generated field ref, source Return operators and the `0/1` Return basis instead of flattening the operation to anonymous dimensions.

## Portable consumer contract

`fixtures/kernel/ql-structural-carrier-contract-v1.json` is the language-neutral pin. Consumers can vendor or checksum that fixture without a live QL service. It freezes only structural facts and an external caller-supplied specimen; external product names and semantic taxonomies do not enter the core contract.

## Boundary with the deeper Vāk/C′ work

This contract intentionally stops before full Vāk lensing, C/C′ intelligence grammar, Context-Frame orchestration, Ta-Onta semantics or application-specific intelligence. Those questions remain the separate #123 programme. If that work later changes the canonical carrier law, it must revise this contract explicitly rather than silently overloading these v1 refs.
