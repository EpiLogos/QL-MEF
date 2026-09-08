# QL capability matrix

Standing: **agent-inference · source-recovered draft for review**. Basis: local checkout `781981374eaa1f02952249dcdbca1af671d1ece4`, 6 September 2026.

This matrix addresses concrete product powers alongside the [whole account](ql.html#whole). Its 18 capability families link the need, operation and useful result to native sources and a governing account unit. It is a selected functional inventory; operation families do not establish every possible integration.




## Native CLI catalog and parity

Discovery command: `target/debug/ql capabilities --json` (built with `cargo build -p ql-cli --bin ql`). The emitted command identifiers are mapped below to their maintained capability records.

<!-- cli-catalog:start -->
| CLI identity | Capability |
| --- | --- |
| `context-frame.list` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.context-frames](#cap-ql-context-frames) |
| `kernel.apply` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.kernel-operators](#cap-ql-kernel-operators) |
| `kernel.capabilities` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.kernel-operators](#cap-ql-kernel-operators) |
| `matheme.derive` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) |
| `matheme.shadow` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) |
| `mef.lenses` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.lens-registry](#cap-ql-lens-registry) |
| `service.capabilities` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.service-negotiation](#cap-ql-service-negotiation) |
| `service.negotiate` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.service-negotiation](#cap-ql-service-negotiation) |
| `vak.capabilities` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.vak-source](#cap-ql-vak-source) |
| `vak.context` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.vak-context](#cap-ql-vak-context) |
| `vak.locate` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.vak-source](#cap-ql-vak-source) |
| `verify` | [cap.ql.cli-discovery](#cap-ql-cli-discovery) · [cap.ql.cli-verification](#cap-ql-cli-verification) |
<!-- cli-catalog:end -->

Parity: all 12 identifiers emitted by `ql capabilities --json` map to at least one capability. The catalog does not imply that every QL capability is a CLI command: pairing, shapes, external frames, adapters, wiki contracts and kernel parity remain library-owned. Service reading dispatch is recorded as a `gap`: `service.negotiate` exposes its availability, while the observed native provider is absent and the CLI executes no reading.

## Seed × field contribution

[View declarations](capability-matrix.json) · [Editable CSV](capability-matrix.csv). Select a populated cell for its source and capability links. Unassessed cells carry no assertion.

| Seed | O:I whole | Central | Actuation | AIKit | Software Factory | Workcell |
| --- | --- | --- | --- | --- | --- | --- |
| [Why?](ql.html#whole/why) | [1 capabilities](#field-q0-S) | Unassessed | Unassessed | Unassessed | Unassessed | Unassessed |
| [What?](ql.html#whole/what) | [8 capabilities](#field-q1-S) | Unassessed | Unassessed | Unassessed | Unassessed | Unassessed |
| [How?](ql.html#whole/how) | [4 capabilities](#field-q2-S) | Unassessed | Unassessed | [3 capabilities](#field-q2-S2) | Unassessed | Unassessed |
| [Who / Whereby?](ql.html#whole/whereby) | [3 capabilities](#field-q3-S) | [3 capabilities](#field-q3-S0) | Unassessed | Unassessed | [3 capabilities](#field-q3-S3) | Unassessed |
| [Where / When?](ql.html#whole/context) | [1 capabilities](#field-q4-S) | Unassessed | Unassessed | Unassessed | Unassessed | Unassessed |
| [Why-For?](ql.html#whole/purpose) | [1 capabilities](#field-q5-S) | Unassessed | Unassessed | Unassessed | Unassessed | Unassessed |

<details>
<summary>Read the field contributions and their capability links</summary>

<a id="field-q0-S"></a>

### Why? → O:I whole

People and Agents use patterns to understand a subject, but the pattern can remain implicit or become a label detached from the work. Quaternal Logic gives relational understanding an explicit form that can be inspected, compared and tested. Its research begins in the Epi-Logos inquiry into mind, relation and interiority; its software makes sufficiently specified parts of that inquiry operational.

[kernel parity](#cap-ql-kernel-parity)

[Source account passage](ql.html#whole/why) · placement: agent-inference.

<a id="field-q1-S"></a>

### What? → O:I whole

Quaternal Logic / MEF is a Rust software product with a native ql CLI, a shared formal kernel, twelve MEF lenses and versioned service and client contracts. It computes defined structural relations, exposes canonical registries, retrieves source-bearing Vāk context and supports attributable readings of existing subjects. The broader research programme develops further operations through explicit contracts and evidence.

[cli discovery](#cap-ql-cli-discovery) · [kernel operators](#cap-ql-kernel-operators) · [pairing](#cap-ql-pairing) · [lens registry](#cap-ql-lens-registry) · [context frames](#cap-ql-context-frames) · [vak source](#cap-ql-vak-source) · [service negotiation](#cap-ql-service-negotiation) · [client modes](#cap-ql-client-modes)

[Source account passage](ql.html#whole/what) · placement: agent-inference.

<a id="field-q2-S"></a>

### How? → O:I whole

A user or client selects a subject, preserves its identity and revision, then chooses a formal operation or a refractive perspective. QL validates the structure and returns the result with its operation, version and source basis. Deterministic kernel operations act directly; richer readings use an explicitly available provider. A client can compare the reading with its original material and retain the difference that proves useful.

[service readings](#cap-ql-service-readings) · [wiki refraction](#cap-ql-wiki-refraction) · [wiki portals](#cap-ql-wiki-portals) · [living wiki](#cap-ql-living-wiki)

[Source account passage](ql.html#whole/how) · placement: agent-inference.

<a id="field-q2-S2"></a>

### How? → AIKit

An AIKit client can supply a bounded WikiFrame: the target wiki, revision, selected nodes and explicit mapping information.

[wiki refraction](#cap-ql-wiki-refraction) · [wiki portals](#cap-ql-wiki-portals) · [living wiki](#cap-ql-living-wiki)

[Source account passage](ql.html#q2/wiki-encounter) · placement: agent-inference.

<a id="field-q3-S"></a>

### Who / Whereby? → O:I whole

QL serves people organising knowledge, developers exposing relational views and researchers testing formal propositions. Stable addresses, a whole anchor, complementary relations and multiple lenses let them inspect different aspects of the same subject. The subject remains owned by its originating product, while QL owns the formal reading and its provenance.

[shape](#cap-ql-shape) · [external frame](#cap-ql-external-frame) · [wiki structure](#cap-ql-wiki-structure)

[Source account passage](ql.html#whole/whereby) · placement: agent-inference.

<a id="field-q3-S0"></a>

### Who / Whereby? → Central

Central holds durable source; AIKit interprets and navigates knowledge; other native products enact their own responsibilities.

[pairing](#cap-ql-pairing) · [external frame](#cap-ql-external-frame) · [client modes](#cap-ql-client-modes)

[Source account passage](ql.html#q3/identity) · placement: agent-inference.

<a id="field-q3-S3"></a>

### Who / Whereby? → Software Factory

A Factory Run, AIKit Resource or Wiki subject carries its native identity into a QL request.

[pairing](#cap-ql-pairing) · [external frame](#cap-ql-external-frame) · [client modes](#cap-ql-client-modes)

[Source account passage](ql.html#q3/identity) · placement: agent-inference.

<a id="field-q4-S"></a>

### Where / When? → O:I whole

QL can be used at a terminal, through Rust libraries or as an optional provider within a larger O:I workflow. It is useful when a person needs to disclose a structure, compare relationships, or test whether a proposed distinction changes an outcome. Ordinary work and ordinary sources remain usable before any QL reading is requested; the chosen scope and available evidence determine the depth of a reading.

[vak context](#cap-ql-vak-context)

[Source account passage](ql.html#whole/context) · placement: agent-inference.

<a id="field-q5-S"></a>

### Why-For? → O:I whole

QL is for developing a more explicit and relationally aware encounter with knowledge and agentic work. It gives O:I and Epi-Logos a common formal resource while leaving each product responsible for its own actions and meaning. Its success lies in useful understanding and demonstrable operations: a reading that clarifies a decision, a structure that preserves relationships, or an experiment that teaches us which distinctions carry real consequences.

[cli verification](#cap-ql-cli-verification)

[Source account passage](ql.html#whole/purpose) · placement: agent-inference.

</details>


## Functional capabilities

| Capability | Need | Native operation | Result |
|---|---|---|---|
| [cap.ql.cli-discovery](#cap-ql-cli-discovery) | Find a usable operation | ql capabilities and version | Versioned command and kernel/service inventory |
| [cap.ql.kernel-operators](#cap-ql-kernel-operators) | Obtain a reproducible structural result | ql kernel apply; apply_operator | Typed result with input/output and operation version |
| [cap.ql.pairing](#cap-ql-pairing) | Distinguish constructions with coincident vertices | A/B/C pairing and D1/D2/D3 completion | Pair family and derivation retained |
| [cap.ql.shape](#cap-ql-shape) | Work with the actual available structure | QlShape and versioned shape contracts | Valid partials; 4x4/6x6 and relational-sixfold fields |
| [cap.ql.lens-registry](#cap-ql-lens-registry) | Inspect available lenses before reading | ql mef lenses; MEF registry | Twelve lens identities with face and square |
| [cap.ql.context-frames](#cap-ql-context-frames) | Select a defined cut of a relation | ql context-frame list; ContextFrameId | Named CF1-CF7 grammar and selected/unselected partitions |
| [cap.ql.external-frame](#cap-ql-external-frame) | Compare a domain composition with formal context | External Context Frame reading | Exact partial ambiguous or no reading with mapping provenance |
| [cap.ql.vak-source](#cap-ql-vak-source) | Keep research language attributable | ql vak locate | Source-locked entry with coordinate and provenance |
| [cap.ql.vak-context](#cap-ql-vak-context) | Recover relevant formulation relationships | ql vak context with depth zero through two | Bounded source entries and relations |
| [cap.ql.service-negotiation](#cap-ql-service-negotiation) | Know what a provider can actually supply | ql service capabilities/negotiate; QlService | Support deterministic flag and provider state |
| [cap.ql.service-readings](#cap-ql-service-readings) | Locate refract relate or synthesise a subject | QlService ServiceRequest dispatch | Reading preserving subject provider and result provenance |
| [cap.ql.client-modes](#cap-ql-client-modes) | Continue ordinary work when reading is optional | Disabled Optional Required adapter modes | Native subject preserved and availability handled explicitly |
| [cap.ql.wiki-structure](#cap-ql-wiki-structure) | Use ordinary nodes and disclosed partial wholes | Wiki structural contracts and anchor relations | Stable structural representation without completeness gating |
| [cap.ql.wiki-refraction](#cap-ql-wiki-refraction) | Inspect selected wiki relationships formally | WikiFrame refraction | Derived reading with target revision and operator basis |
| [cap.ql.wiki-portals](#cap-ql-wiki-portals) | Traverse explicit cross-wiki relationships | MetaBinding and portal traversal | Cross-wiki traversal preserving identities |
| [cap.ql.living-wiki](#cap-ql-living-wiki) | Orient and expand within a bounded knowledge field | Living-wiki methods and entry aperture | Attributable partial or qualified contextual reading |
| [cap.ql.cli-verification](#cap-ql-cli-verification) | Obtain a bounded local verification result | ql verify | Named deterministic verification checks |
| [cap.ql.kernel-parity](#cap-ql-kernel-parity) | Keep C and Rust relations aligned to one source | Shared holographic contract and parity fixtures | Explicit common relation identities and source provenance |

## cap-ql-cli-discovery

**Need:** Find a usable operation.

**Operation:** ql capabilities and version.

**Outcome:** Versioned command and kernel/service inventory.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/product](ql.html#q1/product). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/87.json) · [Code](../../crates/ql-cli/src/lib.rs) · [Tests](../../crates/ql-cli/src/lib.rs).

## cap-ql-kernel-operators

**Need:** Obtain a reproducible structural result.

**Operation:** ql kernel apply; apply_operator.

**Outcome:** Typed result with input/output and operation version.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/kernel](ql.html#q1/kernel). [Source](../../docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md) · [Code](../../crates/ql-core/src/apply.rs) · [Tests](../../crates/ql-core/tests/operator_invariants.rs).

## cap-ql-pairing

**Need:** Distinguish constructions with coincident vertices.

**Operation:** A/B/C pairing and D1/D2/D3 completion.

**Outcome:** Pair family and derivation retained.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/kernel](ql.html#q1/kernel). [Source](../../docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md) · [Code](../../crates/ql-core/src/pairing.rs) · [Tests](../../crates/ql-core/tests/pairing_grammar.rs).

## cap-ql-shape

**Need:** Work with the actual available structure.

**Operation:** QlShape and versioned shape contracts.

**Outcome:** Valid partials; 4x4/6x6 and relational-sixfold fields.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q3/structure](ql.html#q3/structure). [Source](../../fixtures/kernel/ql-shape-contract-v1.json) · [Code](../../crates/ql-core/src/shape.rs) · [Tests](../../crates/ql-core/tests/wiki_structural_contract.rs).

## cap-ql-lens-registry

**Need:** Inspect available lenses before reading.

**Operation:** ql mef lenses; MEF registry.

**Outcome:** Twelve lens identities with face and square.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/readings](ql.html#q1/readings). [Source](../../skills/ql-foundations/SKILL.md) · [Code](../../crates/ql-mef/src/registry.rs) · [Tests](../../crates/ql-mef/tests/identity_and_provenance.rs).

## cap-ql-context-frames

**Need:** Select a defined cut of a relation.

**Operation:** ql context-frame list; ContextFrameId.

**Outcome:** Named CF1-CF7 grammar and selected/unselected partitions.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/readings](ql.html#q1/readings). [Source](../../docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md) · [Code](../../crates/ql-mef/src/context_frame.rs) · [Tests](../../crates/ql-mef/tests/context_frame_promotion.rs).

## cap-ql-external-frame

**Need:** Compare a domain composition with formal context.

**Operation:** External Context Frame reading.

**Outcome:** Exact partial ambiguous or no reading with mapping provenance.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q3/identity](ql.html#q3/identity). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/66.json) · [Code](../../crates/ql-mef/src/context_frame_target.rs) · [Tests](../../crates/ql-mef/tests/context_frame_external_reading.rs).

## cap-ql-vak-source

**Need:** Keep research language attributable.

**Operation:** ql vak locate.

**Outcome:** Source-locked entry with coordinate and provenance.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q1/readings](ql.html#q1/readings). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/87.json) · [Code](../../crates/ql-mef/src/vak.rs) · [Tests](../../crates/ql-cli/src/lib.rs).

## cap-ql-vak-context

**Need:** Recover relevant formulation relationships.

**Operation:** ql vak context with depth zero through two.

**Outcome:** Bounded source entries and relations.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q4/deployment](ql.html#q4/deployment). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/87.json) · [Code](../../crates/ql-cli/src/lib.rs) · [Tests](../../crates/ql-cli/src/lib.rs).

## cap-ql-service-negotiation

**Need:** Know what a provider can actually supply.

**Operation:** ql service capabilities/negotiate; QlService.

**Outcome:** Support deterministic flag and provider state.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q1/services](ql.html#q1/services). [Source](../../skills/ql-operation/SKILL.md) · [Code](../../crates/ql-service/src/negotiation.rs) · [Tests](../../crates/ql-service/tests/provider_service.rs).

## cap-ql-service-readings

**Need:** Locate refract relate or synthesise a subject.

**Operation:** QlService ServiceRequest dispatch.

**Outcome:** Reading preserving subject provider and result provenance.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q2/refract](ql.html#q2/refract). [Source](../../skills/ql-operation/SKILL.md) · [Code](../../crates/ql-service/src/service.rs) · [Tests](../../crates/ql-service/tests/provider_service.rs).

## cap-ql-client-modes

**Need:** Continue ordinary work when reading is optional.

**Operation:** Disabled Optional Required adapter modes.

**Outcome:** Native subject preserved and availability handled explicitly.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q1/services](ql.html#q1/services). [Source](../../skills/ql-operation/SKILL.md) · [Code](../../crates/ql-adapters/src/core.rs) · [Tests](../../crates/ql-adapters/tests/noql_matrix.rs).

## cap-ql-wiki-structure

**Need:** Use ordinary nodes and disclosed partial wholes.

**Operation:** Wiki structural contracts and anchor relations.

**Outcome:** Stable structural representation without completeness gating.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q3/structure](ql.html#q3/structure). [Source](../../docs/wiki-structural-contract-v2.md) · [Code](../../crates/ql-core/src/structural.rs) · [Tests](../../crates/ql-core/tests/wiki_structural_contract.rs).

## cap-ql-wiki-refraction

**Need:** Inspect selected wiki relationships formally.

**Operation:** WikiFrame refraction.

**Outcome:** Derived reading with target revision and operator basis.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q2/wiki-encounter](ql.html#q2/wiki-encounter). [Source](../../docs/wiki-structural-contract-v2.md) · [Code](../../crates/ql-wiki/src/refraction.rs) · [Tests](../../crates/ql-wiki/tests/wiki_refraction.rs).

## cap-ql-wiki-portals

**Need:** Traverse explicit cross-wiki relationships.

**Operation:** MetaBinding and portal traversal.

**Outcome:** Cross-wiki traversal preserving identities.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q2/wiki-encounter](ql.html#q2/wiki-encounter). [Source](../../docs/wiki-structural-contract-v2.md) · [Code](../../crates/ql-wiki/src/portal.rs) · [Tests](../../crates/ql-wiki/tests/meta_portal.rs).

## cap-ql-living-wiki

**Need:** Orient and expand within a bounded knowledge field.

**Operation:** Living-wiki methods and entry aperture.

**Outcome:** Attributable partial or qualified contextual reading.

**Current basis:** implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised.

Account: [q2/wiki-encounter](ql.html#q2/wiki-encounter). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/79.json) · [Code](../../crates/ql-wiki/src/living_methods.rs) · [Tests](../../crates/ql-wiki/src/living_methods.rs).

## cap-ql-cli-verification

**Need:** Obtain a bounded local verification result.

**Operation:** ql verify.

**Outcome:** Named deterministic verification checks.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q5/verification](ql.html#q5/verification). [Source](../../../github-recovery-mirror/mirror/QL-MEF/issues/87.json) · [Code](../../crates/ql-cli/src/lib.rs) · [Tests](../../crates/ql-cli/src/lib.rs).

## cap-ql-kernel-parity

**Need:** Keep C and Rust relations aligned to one source.

**Operation:** Shared holographic contract and parity fixtures.

**Outcome:** Explicit common relation identities and source provenance.

**Current basis:** implemented; selected native tests passed 2026-09-06.

Account: [q0/provenance](ql.html#q0/provenance). [Source](../../docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md) · [Code](../../crates/ql-core/src/kernel.rs) · [Tests](../../crates/ql-mef/tests/holographic_kernel_contract.rs).

## Suite directed product field

The `suite-relations` view uses the same CSV contract as `product-field`. Its H/A identifiers retain the human-facing and agent-facing orientations of the six products. Source-defined relation readings and annotations remain attached to each determination in `extensions`; coverage is independent of implementation status. The manifest declares the selected scope and axis meaning.

The `suite-relations` view uses the same CSV contract as `product-field`. Its H/A identifiers retain the human-facing and agent-facing orientations of the six products. Source-defined relation readings and annotations remain attached to each determination in `extensions`; coverage is independent of implementation status. The manifest declares the selected scope and axis meaning.

## Verification scope

Executed: `cargo test -p ql-cli -p ql-core -p ql-mef --all-targets`, all selected tests passed on 6 September 2026. Service/adapter/wiki tests are source traces outside that executed selection. This observation covers native structural behaviour and selected CLI contracts, not model performance or philosophical validation. The source HTML and CSV row identities still require the planned AIKit format adapter for native semantic extraction.

## Lossless CSV appendix

```csv
id,record_type,view_id,row_id,column_id,capability_refs,need,operation,outcome,implementation_status,standing,source_refs,code_refs,test_refs,account_ref,relation,coverage,extensions,question
cap.ql.cli-discovery,capability,,,,[],Find a usable operation,ql capabilities and version,Versioned command and kernel/service inventory,implemented; selected native tests passed 2026-09-06,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/87.json,crates/ql-cli/src/lib.rs,crates/ql-cli/src/lib.rs,ql.html#q1/product,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""kernel.capabilities"", ""matheme.derive"", ""matheme.shadow"", ""kernel.apply"", ""mef.lenses"", ""context-frame.list"", ""vak.capabilities"", ""vak.locate"", ""vak.context"", ""service.capabilities"", ""service.negotiate"", ""verify""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The native capabilities response advertises this complete emitted command catalog.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87"", ""QL-MEF:agent/ql-multi-matrix-profile""], ""code_basis"": {""crates/ql-cli/src/lib.rs"": ""012922d73ae798df7ea3a88ba3194c9e7987c9c96525c395892aabbe3e28a215""}, ""updated_at"": ""2026-09-08""}, ""last_reconciled_at"": ""2026-09-08T10:44:17.878137+00:00""}",
cap.ql.kernel-operators,capability,,,,[],Obtain a reproducible structural result,ql kernel apply; apply_operator,Typed result with input/output and operation version,implemented; selected native tests passed 2026-09-06,agent-inference,docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md,crates/ql-core/src/apply.rs,crates/ql-core/tests/operator_invariants.rs,ql.html#q1/kernel,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""kernel.capabilities"", ""kernel.apply""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI reports kernel operations and applies advertised deterministic address operators.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-core/src/apply.rs"": ""20db087667368b6e80541792105408b72f595b6a9611557628b3b2f2a4380bd1""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.pairing,capability,,,,[],Distinguish constructions with coincident vertices,A/B/C pairing and D1/D2/D3 completion,Pair family and derivation retained,implemented; selected native tests passed 2026-09-06,agent-inference,docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md,crates/ql-core/src/pairing.rs,crates/ql-core/tests/pairing_grammar.rs,ql.html#q1/kernel,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Pair-family construction is implemented through the ql-core library; the CLI does not emit a pairing command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-core/src/pairing.rs"": ""480e0ec36ed4a5fe6cbe14d3a91af71cf32772a3ef7c6dd30257af47be975df9""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.shape,capability,,,,[],Work with the actual available structure,QlShape and versioned shape contracts,Valid partials; 4x4/6x6 and relational-sixfold fields,implemented; selected native tests passed 2026-09-06,agent-inference,fixtures/kernel/ql-shape-contract-v1.json,crates/ql-core/src/shape.rs,crates/ql-core/tests/wiki_structural_contract.rs,ql.html#q3/structure,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""QlShape contracts are native library APIs; the CLI does not emit a shape command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-core/src/shape.rs"": ""d08444485d431fe75e85c9d7e81078d9e44c132acdcdf4afe7326990d9dcaa8b""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.lens-registry,capability,,,,[],Inspect available lenses before reading,ql mef lenses; MEF registry,Twelve lens identities with face and square,implemented; selected native tests passed 2026-09-06,agent-inference,skills/ql-foundations/SKILL.md,crates/ql-mef/src/registry.rs,crates/ql-mef/tests/identity_and_provenance.rs,ql.html#q1/readings,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""mef.lenses""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI emits the MEF lens registry.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-mef/src/registry.rs"": ""646385949853bd05f2b9daac49539f07147f1ccf65d591290d4e3533434ceef7""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.context-frames,capability,,,,[],Select a defined cut of a relation,ql context-frame list; ContextFrameId,Named CF1-CF7 grammar and selected/unselected partitions,implemented; selected native tests passed 2026-09-06,agent-inference,docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md,crates/ql-mef/src/context_frame.rs,crates/ql-mef/tests/context_frame_promotion.rs,ql.html#q1/readings,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""context-frame.list""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI emits the named Context Frame registry.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-mef/src/context_frame.rs"": ""cd4b2fef86a3f8b604b0048608d5946543ac2ce3cb6786c5272a744a6cc1b15c""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.external-frame,capability,,,,[],Compare a domain composition with formal context,External Context Frame reading,Exact partial ambiguous or no reading with mapping provenance,implemented; selected native tests passed 2026-09-06,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/66.json,crates/ql-mef/src/context_frame_target.rs,crates/ql-mef/tests/context_frame_external_reading.rs,ql.html#q3/identity,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""External Context Frame readings are library contracts; the CLI does not emit an external-frame command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-mef/src/context_frame_target.rs"": ""0394a6f9604024ce2d92908d6950c289b454d771e5a4b26536d2ce6754fb3b9c""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.vak-source,capability,,,,[],Keep research language attributable,ql vak locate,Source-locked entry with coordinate and provenance,implemented; selected native tests passed 2026-09-06,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/87.json,crates/ql-mef/src/vak.rs,crates/ql-cli/src/lib.rs,ql.html#q1/readings,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""vak.capabilities"", ""vak.locate""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI reports and locates source-locked Vāk entries.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-mef/src/vak.rs"": ""6345a97753399a325c0a3b7f821ebf67b552888445fb0d13410b6177f0d63de7""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.vak-context,capability,,,,[],Recover relevant formulation relationships,ql vak context with depth zero through two,Bounded source entries and relations,implemented; selected native tests passed 2026-09-06,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/87.json,crates/ql-cli/src/lib.rs,crates/ql-cli/src/lib.rs,ql.html#q4/deployment,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""vak.context""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI emits bounded source-locked Vāk context readings.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87"", ""QL-MEF:agent/ql-multi-matrix-profile""], ""code_basis"": {""crates/ql-cli/src/lib.rs"": ""012922d73ae798df7ea3a88ba3194c9e7987c9c96525c395892aabbe3e28a215""}, ""updated_at"": ""2026-09-08""}, ""last_reconciled_at"": ""2026-09-08T10:44:17.878137+00:00""}",
cap.ql.service-negotiation,capability,,,,[],Know what a provider can actually supply,ql service capabilities/negotiate; QlService,Support deterministic flag and provider state,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,skills/ql-operation/SKILL.md,crates/ql-service/src/negotiation.rs,crates/ql-service/tests/provider_service.rs,ql.html#q1/services,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""service.capabilities"", ""service.negotiate""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI reports provider state and negotiates the advertised service operations.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-service/src/negotiation.rs"": ""a06f9a97dbe4bafcca41575ea2909acea9770554f35f4581b2515badf6edf79e""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.service-readings,capability,,,,[],Locate refract relate or synthesise a subject,QlService ServiceRequest dispatch,Reading preserving subject provider and result provenance,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,skills/ql-operation/SKILL.md,crates/ql-service/src/service.rs,crates/ql-service/tests/provider_service.rs,ql.html#q2/refract,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""gap"", ""reason"": ""The service contract exists, but this CLI only negotiates availability and the observed provider is absent; it does not execute readings.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-service/src/service.rs"": ""c51aed38de62fd2152308c69ac02b55c3dfbb8dc8216e0dfeb2bbaf6be695884""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.client-modes,capability,,,,[],Continue ordinary work when reading is optional,Disabled Optional Required adapter modes,Native subject preserved and availability handled explicitly,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,skills/ql-operation/SKILL.md,crates/ql-adapters/src/core.rs,crates/ql-adapters/tests/noql_matrix.rs,ql.html#q1/services,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Disabled, optional, and required adapter modes are ql-adapters library contracts.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-adapters/src/core.rs"": ""95996a65e0aab39f6990c45889b271b912239f123c9078fbda5c407aaa978ad0""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.wiki-structure,capability,,,,[],Use ordinary nodes and disclosed partial wholes,Wiki structural contracts and anchor relations,Stable structural representation without completeness gating,implemented; selected native tests passed 2026-09-06,agent-inference,docs/wiki-structural-contract-v2.md,crates/ql-core/src/structural.rs,crates/ql-core/tests/wiki_structural_contract.rs,ql.html#q3/structure,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Wiki structural contracts are exposed by ql-core libraries, with no emitted CLI command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-core/src/structural.rs"": ""e725fc99652b4f25b80cbc82e86b8c99faf5c1be6ae3bd9c1e188150f5a2c1f3""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.wiki-refraction,capability,,,,[],Inspect selected wiki relationships formally,WikiFrame refraction,Derived reading with target revision and operator basis,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,docs/wiki-structural-contract-v2.md,crates/ql-wiki/src/refraction.rs,crates/ql-wiki/tests/wiki_refraction.rs,ql.html#q2/wiki-encounter,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""WikiFrame refraction is a ql-wiki library contract, with no emitted CLI command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-wiki/src/refraction.rs"": ""52cf52cd79f77bb7a31f36b1a547103ea9361653aefc60c7dbac2a1319d8729a""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.wiki-portals,capability,,,,[],Traverse explicit cross-wiki relationships,MetaBinding and portal traversal,Cross-wiki traversal preserving identities,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,docs/wiki-structural-contract-v2.md,crates/ql-wiki/src/portal.rs,crates/ql-wiki/tests/meta_portal.rs,ql.html#q2/wiki-encounter,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Portal traversal is a ql-wiki library contract, with no emitted CLI command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-wiki/src/portal.rs"": ""919ce316d0f35420202426ffbb76de04b72dfd0668845fcd16bb2be420b07908""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.living-wiki,capability,,,,[],Orient and expand within a bounded knowledge field,Living-wiki methods and entry aperture,Attributable partial or qualified contextual reading,implemented contracts; source-inspected 2026-09-06; provider/runtime acceptance not exercised,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/79.json,crates/ql-wiki/src/living_methods.rs,crates/ql-wiki/src/living_methods.rs,ql.html#q2/wiki-encounter,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Living-wiki methods are ql-wiki library contracts, with no emitted CLI command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-wiki/src/living_methods.rs"": ""bf594f7d398b5cac883c14f0739c35eb4d838f41db25571326ef6ef33c70a9e3""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
cap.ql.cli-verification,capability,,,,[],Obtain a bounded local verification result,ql verify,Named deterministic verification checks,implemented; selected native tests passed 2026-09-06,agent-inference,../github-recovery-mirror/mirror/QL-MEF/issues/87.json,crates/ql-cli/src/lib.rs,crates/ql-cli/src/lib.rs,ql.html#q5/verification,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [""verify""], ""cli_exposure"": {""kind"": ""direct"", ""reason"": ""The CLI emits its bounded native verification result.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87"", ""QL-MEF:agent/ql-multi-matrix-profile""], ""code_basis"": {""crates/ql-cli/src/lib.rs"": ""012922d73ae798df7ea3a88ba3194c9e7987c9c96525c395892aabbe3e28a215""}, ""updated_at"": ""2026-09-08""}, ""last_reconciled_at"": ""2026-09-08T10:44:17.878137+00:00""}",
cap.ql.kernel-parity,capability,,,,[],Keep C and Rust relations aligned to one source,Shared holographic contract and parity fixtures,Explicit common relation identities and source provenance,implemented; selected native tests passed 2026-09-06,agent-inference,docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md,crates/ql-core/src/kernel.rs,crates/ql-mef/tests/holographic_kernel_contract.rs,ql.html#q0/provenance,,,"{""basis"": ""Source-recovered capability account; original verification limits retained in Markdown."", ""cli_commands"": [], ""cli_exposure"": {""kind"": ""library"", ""reason"": ""Kernel parity is maintained through ql-core and fixture contracts, with no emitted CLI command.""}, ""converted_from_sha256"": ""86d000724352ae0b39b4a596596bd181b78a9f5b53435404b7d851d90e22920a"", ""maintenance"": {""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""QL-MEF#87""], ""code_basis"": {""crates/ql-core/src/kernel.rs"": ""d615307e8408749847707e60fae2aeb70d216ee710e0d0fb9a9dfe2861081156""}, ""updated_at"": ""2026-09-06""}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H0->H5,relation,suite-relations,H0,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""Central"", ""dst_product"": ""QL"", ""ql"": ""B3|C1"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H0->A5,relation,suite-relations,H0,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""Central"", ""dst_product"": ""QL"", ""ql"": ""D2-require|D2-complete"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H1->H5,relation,suite-relations,H1,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""Actuation"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H1->A5,relation,suite-relations,H1,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""Actuation"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H2->H5,relation,suite-relations,H2,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""AIKit"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H2->A5,relation,suite-relations,H2,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""AIKit"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H3->H5,relation,suite-relations,H3,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""Factory"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H3->A5,relation,suite-relations,H3,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""Factory"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H4->H5,relation,suite-relations,H4,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""Workcell"", ""dst_product"": ""QL"", ""ql"": ""A3"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H4->A5,relation,suite-relations,H4,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""Workcell"", ""dst_product"": ""QL"", ""ql"": ""D2-transform"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H0,relation,suite-relations,H5,H0,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""QL"", ""dst_product"": ""Central"", ""ql"": ""B3|C1"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H1,relation,suite-relations,H5,H1,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Actuation"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H2,relation,suite-relations,H5,H2,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""AIKit"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H3,relation,suite-relations,H5,H3,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Factory"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H4,relation,suite-relations,H5,H4,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""QL"", ""dst_product"": ""Workcell"", ""ql"": ""A3"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->H5,relation,suite-relations,H5,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,self:QL,I,"{""src_product"": ""QL"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""self:QL"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A0,relation,suite-relations,H5,A0,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""QL"", ""dst_product"": ""Central"", ""ql"": ""D2-transform|D2-complete"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A1,relation,suite-relations,H5,A1,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Actuation"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A2,relation,suite-relations,H5,A2,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""AIKit"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A3,relation,suite-relations,H5,A3,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Factory"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A4,relation,suite-relations,H5,A4,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""QL"", ""dst_product"": ""Workcell"", ""ql"": ""D2-require"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
H5->A5,relation,suite-relations,H5,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,conjugation:QL,H,"{""src_product"": ""QL"", ""dst_product"": ""QL"", ""ql"": ""D1"", ""cf_view"": """", ""seam"": ""conjugation:QL"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A0->H5,relation,suite-relations,A0,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""Central"", ""dst_product"": ""QL"", ""ql"": ""D2-transform.inverse|D2-complete.inverse"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A0->A5,relation,suite-relations,A0,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""Central"", ""dst_product"": ""QL"", ""ql"": ""D3:B3|D3:C1"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A1->H5,relation,suite-relations,A1,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""Actuation"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A1->A5,relation,suite-relations,A1,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""Actuation"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A2->H5,relation,suite-relations,A2,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""AIKit"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A2->A5,relation,suite-relations,A2,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""AIKit"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A3->H5,relation,suite-relations,A3,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""Factory"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A3->A5,relation,suite-relations,A3,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""Factory"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A4->H5,relation,suite-relations,A4,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""Workcell"", ""dst_product"": ""QL"", ""ql"": ""D2-require.inverse"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A4->A5,relation,suite-relations,A4,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""Workcell"", ""dst_product"": ""QL"", ""ql"": ""D3:A3"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H0,relation,suite-relations,A5,H0,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""QL"", ""dst_product"": ""Central"", ""ql"": ""D2-require.inverse|D2-complete.inverse"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H1,relation,suite-relations,A5,H1,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Actuation"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H2,relation,suite-relations,A5,H2,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""AIKit"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H3,relation,suite-relations,A5,H3,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Factory"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H4,relation,suite-relations,A5,H4,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""QL"", ""dst_product"": ""Workcell"", ""ql"": ""D2-transform.inverse"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->H5,relation,suite-relations,A5,H5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,conjugation:QL,H,"{""src_product"": ""QL"", ""dst_product"": ""QL"", ""ql"": ""D1.inverse"", ""cf_view"": """", ""seam"": ""conjugation:QL"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A0,relation,suite-relations,A5,A0,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,05/50:ground-synthesis,H,"{""src_product"": ""QL"", ""dst_product"": ""Central"", ""ql"": ""D3:B3|D3:C1"", ""cf_view"": ""CF7"", ""seam"": ""05/50:ground-synthesis"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Central#24(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A1,relation,suite-relations,A5,A1,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,15:agency-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Actuation"", ""ql"": """", ""cf_view"": """", ""seam"": ""15:agency-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Actuation#1;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A2,relation,suite-relations,A5,A2,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,25:context-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""AIKit"", ""ql"": """", ""cf_view"": """", ""seam"": ""25:context-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/ai-kit#58(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A3,relation,suite-relations,A5,A3,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,35:development-ql,S,"{""src_product"": ""QL"", ""dst_product"": ""Factory"", ""ql"": """", ""cf_view"": """", ""seam"": ""35:development-ql"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/agent-system-design#132(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A4,relation,suite-relations,A5,A4,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,45:actuality-formal-intelligence,H,"{""src_product"": ""QL"", ""dst_product"": ""Workcell"", ""ql"": ""D3:A3"", ""cf_view"": ""CF5/CF6-field"", ""seam"": ""45:actuality-formal-intelligence"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/Workcell#18(PR);EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
A5->A5,relation,suite-relations,A5,A5,[],,,,,agent-inference,O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR),,,,self:QL,I,"{""src_product"": ""QL"", ""dst_product"": ""QL"", ""ql"": """", ""cf_view"": """", ""seam"": ""self:QL"", ""defined_in"": ""O-I:docs/CANONICAL-PRODUCT-FIELD.md|QL-MEF#19(PR)"", ""tracked_by"": ""EpiLogos/O-I#29;EpiLogos/QL-MEF#19(PR)"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
rel.ql.q0.S,relation,product-field,q0,S,"[""cap.ql.kernel-parity""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-why,,,ql.html#whole/why,"People and Agents use patterns to understand a subject, but the pattern can remain implicit or become a label detached from the work. Quaternal Logic gives relational understanding an explicit form that can be inspected, compared and tested. Its research begins in the Epi-Logos inquiry into mind, relation and interiority; its software makes sufficiently specified parts of that inquiry operational.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q0"", ""source_unit"": ""whole-why"", ""seed_sha256"": ""3869e8d7edfd0673735a78ad9ff3a02303c7cf74fcf63c2d9b392dc5afd30316"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",Why?
rel.ql.q1.S,relation,product-field,q1,S,"[""cap.ql.cli-discovery"", ""cap.ql.kernel-operators"", ""cap.ql.pairing"", ""cap.ql.lens-registry"", ""cap.ql.context-frames"", ""cap.ql.vak-source"", ""cap.ql.service-negotiation"", ""cap.ql.client-modes""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-what,,,ql.html#whole/what,"Quaternal Logic / MEF is a Rust software product with a native ql CLI, a shared formal kernel, twelve MEF lenses and versioned service and client contracts. It computes defined structural relations, exposes canonical registries, retrieves source-bearing Vāk context and supports attributable readings of existing subjects. The broader research programme develops further operations through explicit contracts and evidence.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q1"", ""source_unit"": ""whole-what"", ""seed_sha256"": ""ed702daab4d4f5a6a5d743844c8cda1a9e687b5302c16b4e48b3731aee8af483"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",What?
rel.ql.q2.S,relation,product-field,q2,S,"[""cap.ql.service-readings"", ""cap.ql.wiki-refraction"", ""cap.ql.wiki-portals"", ""cap.ql.living-wiki""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-how,,,ql.html#whole/how,"A user or client selects a subject, preserves its identity and revision, then chooses a formal operation or a refractive perspective. QL validates the structure and returns the result with its operation, version and source basis. Deterministic kernel operations act directly; richer readings use an explicitly available provider. A client can compare the reading with its original material and retain the difference that proves useful.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q2"", ""source_unit"": ""whole-how"", ""seed_sha256"": ""b64f4d4d716b918dbf2863312ad9e7c3115cce9773a7788d547894567a6e0c83"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",How?
rel.ql.q3.S,relation,product-field,q3,S,"[""cap.ql.shape"", ""cap.ql.external-frame"", ""cap.ql.wiki-structure""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-whereby,,,ql.html#whole/whereby,"QL serves people organising knowledge, developers exposing relational views and researchers testing formal propositions. Stable addresses, a whole anchor, complementary relations and multiple lenses let them inspect different aspects of the same subject. The subject remains owned by its originating product, while QL owns the formal reading and its provenance.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q3"", ""source_unit"": ""whole-whereby"", ""seed_sha256"": ""124c1f33c42b0140a1d750f54501605e96b42cd630f16f17a532028c2a381e4b"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",Who / Whereby?
rel.ql.q4.S,relation,product-field,q4,S,"[""cap.ql.vak-context""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-context,,,ql.html#whole/context,"QL can be used at a terminal, through Rust libraries or as an optional provider within a larger O:I workflow. It is useful when a person needs to disclose a structure, compare relationships, or test whether a proposed distinction changes an outcome. Ordinary work and ordinary sources remain usable before any QL reading is requested; the chosen scope and available evidence determine the depth of a reading.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q4"", ""source_unit"": ""whole-context"", ""seed_sha256"": ""3164ed3d0ccf6d573724b7859bb8dc70ab6b89c92e20590bc7360511f885053b"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",Where / When?
rel.ql.q5.S,relation,product-field,q5,S,"[""cap.ql.cli-verification""]",,,,,agent-inference,ProjectCentral/user/ql.html#whole-purpose,,,ql.html#whole/purpose,"QL is for developing a more explicit and relationally aware encounter with knowledge and agentic work. It gives O:I and Epi-Logos a common formal resource while leaving each product responsible for its own actions and meaning. Its success lies in useful understanding and demonstrable operations: a reading that clarifies a decision, a structure that preserves relationships, or an experiment that teaches us which distinctions carry real consequences.",,"{""basis"": ""Exact overview seed text. Capability links follow their existing governing expanded account units; cell placement is editorial inference."", ""seed_ref"": ""ql:seed:q5"", ""source_unit"": ""whole-purpose"", ""seed_sha256"": ""0b2bff63c1b59bed272e2b4a2c7863e946064f70525302b179d8c9e2b17ae630"", ""source_standing"": ""agent-inference"", ""reconciled_change_ref"": ""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",Why-For?
rel.ql.q3.S0,relation,product-field,q3,S0,"[""cap.ql.pairing"", ""cap.ql.external-frame"", ""cap.ql.client-modes""]",,,,,agent-inference,ProjectCentral/user/ql.html#q3-identity,,,ql.html#q3/identity,Central holds durable source; AIKit interprets and navigates knowledge; other native products enact their own responsibilities.,,"{""basis"": ""Exact account sentence; cell placement is editorial inference."", ""source_unit"": ""q3-identity"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
rel.ql.q2.S2,relation,product-field,q2,S2,"[""cap.ql.wiki-refraction"", ""cap.ql.wiki-portals"", ""cap.ql.living-wiki""]",,,,,agent-inference,ProjectCentral/user/ql.html#q2-wiki-encounter,,,ql.html#q2/wiki-encounter,"An AIKit client can supply a bounded WikiFrame: the target wiki, revision, selected nodes and explicit mapping information.",,"{""basis"": ""Exact account sentence; cell placement is editorial inference."", ""source_unit"": ""q2-wiki-encounter"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
rel.ql.q3.S3,relation,product-field,q3,S3,"[""cap.ql.pairing"", ""cap.ql.external-frame"", ""cap.ql.client-modes""]",,,,,agent-inference,ProjectCentral/user/ql.html#q3-identity,,,ql.html#q3/identity,"A Factory Run, AIKit Resource or Wiki subject carries its native identity into a QL request.",,"{""basis"": ""Exact account sentence; cell placement is editorial inference."", ""source_unit"": ""q3-identity"", ""maintenance"": {""updated_at"": ""2026-09-06"", ""change_refs"": [""codex:thread:01a07608-d2ec-7b10-9713-74c445adf8a5""]}, ""last_reconciled_at"": ""2026-09-06T14:30:24.557139+00:00""}",
```
