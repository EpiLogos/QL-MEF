# CI convergence return

This change reconciles the Quaternal Logic kernel-rebuild CI additions with the suite CI law already established across O:I, Central and AIKit.

The historical exception was real: O:I #97 deliberately excluded the moving QL/Epi computational programme from the live-main convergence matrix while that programme was unstable. The kernel rebuild subsequently accumulated independent pull-request workflows around its own acceptance tranches even after `ql-mef-rust` had become the protected single gate.

The earlier unmerged `agent/ci-cache-wip` commit `f8141ee34dc04a47376240f72a6cbe49b8583d23` correctly identified workspace-test duplication and caching/cancellation requirements. Its CI intent is retained here against current main without reviving the stale code ancestry.

Returned structure:

- one protected pull-request workflow: `ql-mef-rust.yml`;
- one repository-native quick/full verification entrypoint: `scripts/verify`;
- native Rust, C/C++ and deterministic invariant offices composed inside that gate;
- product-local evidence campaigns executed from accepted main or manually;
- every sibling-repository proof commissioned by weekly/on-demand `cross-product.yml`;
- no loss of K8/K9/AW/Bimba/Epi-C/Vāk/M2/M3/UX evidence campaigns;
- a topology checker in the protected gate to prevent re-divergence.

The companion O:I convergence removes Quaternal Logic's `parallel-native-owner-exception`, makes the current QL main a normal member of the six-product current-main field, and consumes the native `.oi/product.json` source-verification contract rather than the historical build manifest.
