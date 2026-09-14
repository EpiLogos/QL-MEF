# Quaternal Logic CI topology

Quaternal Logic participates in the O:I suite as one native product. CI therefore follows the same composition law as the other native owners rather than treating each historical kernel tranche as an independent pull-request gate.

## Native development loop

`scripts/verify quick [-p <package> ...]` is the bounded inner-loop verifier. It is intended for Agent/human development before publishing a candidate.

`scripts/verify full` is the product source-verification contract published by `.oi/product.json`. It composes the canonical Rust workspace gate, the native C/C++ parity and packaging floor, and deterministic repository invariants.

## Pull requests

`.github/workflows/ql-mef-rust.yml` is the only pull-request workflow and the only required branch-protection check. Internally it runs independent offices in parallel and returns one stable `ql-mef-rust` verdict:

- `core-rust` — format, lint, complete Rust workspace tests and native CLI self-certification;
- `native-c` — applicable native C/C++ parity, packaging and installed-consumer proof;
- `repository-invariants` — source/registry/census/skills/QL-promotion/CI-topology locks.

Superseded heads are cancelled. No historical kernel campaign may add another `pull_request` trigger.

## Accepted-main product acceptance

Domain campaigns such as K8/K9, graph, sky, M-ledger, native-C evidence, Skills and documentation maintenance observe accepted `main` (with manual dispatch where useful). They retain richer receipts without duplicating the merge gate or blocking development on unrelated provider/browser acceptance.

## Cross-product acceptance

Any proof that reads a sibling repository is a reusable/on-demand worker commissioned by `.github/workflows/cross-product.yml`. The coordinator runs weekly and manually, never per push or pull request. A red result therefore means a pinned owner seam or suite relation moved; it does not cause every Quaternal Logic change to rebuild the wider O:I product field.

## Drift prevention

`scripts/check-ci-topology.py` is part of the protected invariant gate. It rejects:

- a second pull-request workflow;
- cross-product workers that reacquire push/PR schedules;
- sibling-repository reads outside the cross-product office;
- workers not commissioned by the central cross-product workflow;
- removal of superseded-head cancellation from the canonical PR gate.

The topology is routing only. Native tests and owner contracts remain authoritative for the meaning of their evidence.
