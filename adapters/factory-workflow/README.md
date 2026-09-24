# `@epilogos/ql-vak` — Vāk types for Factory workflows

This package is QL's Vāk vocabulary for Factory's typed workflow authoring. It
contains types only. Factory owns the TypeScript entry, the restricted parser,
lowering, compiled unit identity and Commission. QL owns what the values mean.

`index.d.ts` is rendered from the Rust contract in
`crates/ql-mef/src/vak_workflow_types.rs` (over `vak_profile.rs` and
`context_frame.rs`). Do not edit it by hand. Regenerate and check it with:

```sh
cargo run -q -p ql-cli -- vak workflow-types > adapters/factory-workflow/index.d.ts
cargo run -q -p ql-cli -- vak workflow-types --check adapters/factory-workflow/index.d.ts
cargo test -p ql-mef --test vak_workflow_types
```

The test fails when the file, this package manifest or the authored C′ serde
surface drift from the Rust contract.

## How a workflow uses it

```ts
import { defineWorkflow, unit } from "@epilogos/factory-workflow";
import type { CPrime } from "@epilogos/ql-vak";

unit({
  // ...ordinary Factory unit fields...
  composition: {
    CPF: "dialogical", CT: "CT1", CP: "4.1", CF: "CF2", CFP: "CFP2", CS: "CS2", direction: "forward",
    actor: "agent/anima-logos",
    interpretation: { ref: "ql/interpretation/c-prime", revision: "09f7d29ad6262f85bc2858f7c468f22d0bd398f3" },
    whole: "central:source:project:quaternal-logic:ProjectCentral/now",
    resolvePath: "resolve-scoped-path:example/logos",
    contextResolution: "context-resolution:example/logos",
    sources: ["central:source:project:quaternal-logic:AGENTS.md"],
  } satisfies CPrime,
});
```

Factory admits this import only through its registered domain adapter
(`contracts/factory/workflow-domain-adapters.json` in EpiLogos/Factory, which
vendors this exact file and pins its sha256 and git blob). It lowers each
`composition` to its native `CPrimeExecutionBinding`: the unit's `subjectRef`
becomes the binding subject, `actor` must be one of the unit's declared
participants, and an `authorised-undertaking` must name its `authority`.
Thread forms must match the compiled topology: a chord (CFP1) is pairwise
independent, a melody (CFP2) consumes each predecessor's return through
`inputs`, a fusion (CFP3) waits at a barrier, a canon (CFP5) is nested.

The frame's lens and harmonic basis stay with the QL binding named by
`interpretation`; they are not separate fields here, so nothing is dropped in
lowering. A C′ is a requirement for conduct. It resolves no body, loads no
context and grants no authority.

When this file changes, Factory must re-vendor it: copy it to
`contracts/factory/upstream/ql-vak-workflow-types.d.ts` and update the digest
and blob in the registry. Factory's contract test compares its native value
sets with the unions here.
