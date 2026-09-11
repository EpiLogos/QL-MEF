# Lens identity, positional relations and boundary encodings

Status: settled clarification of existing behaviour, 2026-09-11.
Provenance: the owner's correction that `L0, L0', L1, L1', ...` is intended;
accepted K5 #150/#129 and the M1-to-M2 producer proof/documentation in #158.
This document changes no lens law, production API, engine version or registry.

## A lens is not its place in a list

`LensId` identifies one of six positions on one of two faces. `LensRef` adds
the existing registry revision; `SublensRef` adds the local position inside
that lens. The positional relations are implemented by `src/lens.rs`, not by
sorting the displayed labels or advancing an array cursor.

For example, L1 has conjugate twin L1', same-face complement L4, and Mobius
partner L4'. These are different operations. The twin preserves position and
changes face; the complement preserves face and exchanges complementary
positions; the Mobius relation composes those two changes. Each returns to
the original lens when applied twice. The existing square memberships and
other domain relations remain with their native owners.

`L0 -> L0' -> L1 -> L1'` is an intended traversal. A grouped traversal or another
explicit visit order can also visit the same field. No single enumeration is
the mandatory temporal, processual or musical walk. Changing visit order must
not rename the visited lenses or redefine their relations. This does **not**
assert that different sequences of state-changing operations produce the same
result: identity preservation and process history are separate questions.

## Existing numerical representations

| Boundary | Meaning of its number | Example |
|---|---|---|
| `LensId::index()` | Position 0 through 5; direct/prime twins share it | L0 and L0' both have index 0 |
| `LensId::slot()` / `LensId::ALL` | Interleaved native slot, used by M1 v1 `lens12` | Slot 1 is L0'; slot 2 is L1 |
| Retained M2 C MEF lens row / `VimarshaSeed.lens` | Six direct rows followed by six prime rows | Row 1 is L1; row 6 is L0' |
| `Reading72` in the MEF register | Retained lens row plus local position | L0' at local position 0 has carrier index 36 |

These coexist without contradiction. A number in one representation is not a
universal identity or an instruction to walk the field in that order. A slot
contract is nevertheless stable for callers that use it; it must not be silently
reinterpreted just because another boundary uses a different representation.

Use the existing identity-bearing bridge, not a second lookup table:

```rust
let sublens = SublensRef::canonical(lens, local_position)?;
let retained = Reading72::from_sublens(sublens);
let restored = retained.mef_sublens()?;
assert_eq!(restored, sublens);
// retained.axes()[0] is the retained M2 lens row where that API requires one.
```

The example omits caller imports/error conversion. Production owners remain
[`lens.rs`](../../crates/ql-mef/src/lens.rs),
[`sublens.rs`](../../crates/ql-mef/src/sublens.rs),
[`m2.rs`](../../crates/ql-mef/src/m2.rs) and
[`m2_vimarsha.rs`](../../crates/ql-mef/src/m2_vimarsha.rs).
The [M1 engine contract](../KERNEL-M1-ENGINE-CONTRACT.md) and
[M2 parent handoff](M2-PARENT-HANDOFF.md) retain their own operational scope.
M1 clock phase/Hopf fibre and musical basis do not redefine the selected lens.

## Withdrawn diagnosis, not an outstanding engine fix

The claim that `lens12=1 -> L0'` is itself a defect is withdrawn. That is the
intended M1 v1 result. The earlier grouped-order prose was corrected in #158;
the native M1 implementation did not require reordering.

The conversation artifact `m1-lens-order-correction.zip`, including its proposed
1.0.1 reorder and grouped-M1 test expectation, must not be applied. Its passing
test enforced the wrong premise; it did not establish a failing production
handoff. This clarification does not publish that patch, bump the engine
version, add an open ledger discrepancy, or reopen accepted K5.

For a future suspected mismatch, identify the source lens/ref and local
position, the sending and receiving contracts, and the actual conversion/call
path. Demonstrate lost identity, relation or state before changing behaviour.
A direct comparison of unlike raw integer encodings is not that demonstration.

## Executable protection

[`lens_identity_contract.rs`](../../crates/ql-mef/tests/lens_identity_contract.rs)
checks all twelve lens identities' twin/complement/Mobius relations, the same
72 sublens identities through interleaved/grouped/independent visit orders,
all 72 forward/inverse native boundary conversions, invalid register/axis
rejection, and intentional M1 slot-1 selection in both engine and traversal
across both bases and all four clock-phase/Hopf-fibre combinations. Explicit
compatibility witnesses prevent a changed array from changing the test's
expected identity along with the implementation.

The existing [`m1_m2_parent_join.rs`](../../crates/ql-mef/tests/m1_m2_parent_join.rs)
continues to exercise real M1/M2 production for Cosmic, Personal and deep
callers with event/generation, ratio, clock and source provenance. There is no
new bridge or parent-local pitch law in these tests.

```sh
cargo test -p ql-mef --test lens_identity_contract --locked
cargo test -p ql-mef --test m1_m2_parent_join --locked
```

These tests belong to the existing workspace CI. They establish finite
identity/encoding and producer behaviour, not installed desktop, audio or
Epii-session acceptance. M0' remains the Bimba map and M5' the same Epii agent;
this clarification changes none of the #154 parent/deep assignments.
