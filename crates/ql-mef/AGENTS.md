# Working on QL-MEF lens identity

Before changing a lens index, traversal, or M1/M2 handoff, read
[the settled identity/encoding clarification](../../docs/kernel-rebuild/LENS-IDENTITY-AND-ENCODINGS.md)
and the actual owners in `src/lens.rs`, `src/sublens.rs`, and `src/m2.rs`.

A lens has a position, face and relations; a slot or a visit rank does not
define that whole structure. `L0 -> L0' -> L1 -> L1'` is an intended walk.
M1 v1 `lens12=1` means `L0'`, not `L1`. Do not reorder `LensId::ALL` or
reinterpret M1's published slots to match a retained M2 table. Use the existing
`Reading72::from_sublens` / `mef_sublens` bridge when crossing that boundary.
Different walks remain available; no enumeration is the mandatory global walk.

The proposed conversation artifact `m1-lens-order-correction.zip` is withdrawn;
its grouped-M1 expectation was not a demonstrated product defect. Do not apply
it or reopen K5 on that basis. Diagnose any future mismatch against both named
boundary contracts and a concrete identity-preserving call path, not equal
bare integers. Preserve current schemas and reviewed ledger/source standing.

Run `cargo test -p ql-mef --test lens_identity_contract --locked` and the existing
`m1_m2_parent_join` suite when touching this relation. Tests must keep identity,
encoding and traversal distinct; they must not require all walks to have the
same temporal result.
