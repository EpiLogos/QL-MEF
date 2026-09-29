# Track: land the carrier infrastructure

Why: the managed install, companions, worker I/O and `replace-shapes` carry no
meaning and are already built. They should land so the meaning has a home.

Tickets:
- QL-MEF #251 (draft; split into QL-A per #254 §6);
- O:I #545 (split into O:I-A).

Depends: nothing. Parallel-with: map-and-kernel.

Done-when:
- QL-A and O:I-A are merged, and the QL pin in `surfaces.json` has moved in the same O:I PR.
- `kernel-k8-continuous.yml` and `kernel-m2.yml` were dispatched green.
- The `k2.rs` semantics were dropped, not carried forward.

State (29 Sep): the host infrastructure landed in #272, and #251 is closed. That covers:
- the managed install of the companions;
- worker I/O;
- replace-shapes and strike;
- the host operations;
- the browser adapters.
The K² semantics are gone, and the scene is the instrument.
