# Quaternal Logic / MEF

Quaternal Logic / MEF is a Rust workspace and command-line tool, `ql`, that implements the formal structures of Quaternal Logic as typed, versioned operations. It holds O:I's **reflection** facet: a formal way of reading the rest of an agent's world.

Repository: `EpiLogos/QL-MEF`. The product name is Quaternal Logic, and the local checkout is usually `Work/Quaternal-Logic`.

## What it does today

`ql` is a deterministic kernel plus a registry of reading lenses. Every result is JSON and records the schema, kernel or registry version, the operation, and its canonical input.

**The coordinate kernel** (`ql-core`). Quaternal Logic places things at six positions, P0 to P5. Each position has a direct face and a conjugate face. An address names one such place, for example `qladdr:sixfold@1/direct/P2/d0`. The kernel ships three deterministic operators:

| Operator | What it does | Example |
|---|---|---|
| `conjugate-address` | flips the face | `…/direct/P2/d0` → `…/conjugate/P2/d0` |
| `complement-address` | pairs positions that sum to five: (0,5), (1,4), (2,3) | `…/direct/P2/d0` → `…/direct/P3/d0` |
| `classify-four-plus-two` | marks P0 and P5 as the two framing positions ("implicate") and P1–P4 as the four worked positions ("explicate") | P5 → `implicate` |

Unsupported operators and invalid positions fail with an explicit error. The kernel advertises no stochastic and no research operations.

**The Meta-Epistemic Framework (MEF)** (`ql-mef`). A *lens* is a fixed perspective from which one subject can be read, such as causal, logical, processual or phenomenological. MEF has twelve lenses: six "day" lenses, each paired with a "night" twin. Each lens has six sublenses, so there are 72 in all. The lenses fall into three groups of four, called squares (articulation, encounter, becoming). `ql mef lenses` lists them: L0 Quaternal / L0′ Archetypal-Numerical, L1 Causal / L1′ Phenomenal, L2 Logical / L2′ Alchemical-Elemental, L3 Processual / L3′ Chronological, L4 Phenomenological / L4′ Scientific, L5 Para Vāk / L5′ Divine Logos.

A *refraction* is a reading of a subject through a lens. The subject keeps its own identity: a Factory Run read through a lens is still that Run, with a reading attached.

**Other commands that work now:**

- `ql context-frame list` lists the seven Context Frames, CF1–CF7.
- `ql vak compose <request.json>` runs a composition over the Vāk registry. That registry has 109 entries and is pinned to a revision of `EpiLogos/Epi-Logos-C-Experiments`. Results the tool works out are marked `DERIVED`, and caller input is marked `PROPOSED`.
- `ql vak workflow-types` emits the TypeScript types that Software Factory's workflow compiler admits.
- `ql matheme derive` and `ql matheme shadow` print the definitional arithmetic of the kernel.
- `ql kernel coverage | ledger | validate-ledger` report how much of the formal structure has a computational binding. They report gaps as well as coverage: 907 coordinates are currently without one.
- `ql techne reading <target.json>` returns a reading of a supplied subject.
- `ql service capabilities` and `ql service negotiate <op>` say which reading operations a provider supports.
- `ql verify`, `ql capabilities`, `ql system` and `ql config-contribution` are for checking an install and for disclosure to `oi`.

Some groups are for specialists and are not needed for ordinary use: `ql scene …`, `ql nara …` (a personal-field instrument; `nara calculate` needs `uv` and network access to build a Python environment) and `ql epi-agent …`.

**In development, stated plainly:**

- **Service operations.** Of the five, `capabilities`, `locate` and `refract` are supported. `relate` and `synthesise` have no provider yet and report `supported: false`. The CLI negotiates these operations but cannot yet run `locate` or `refract` on a subject you supply.
- **Configuration.** The configuration surface only discloses settings. Every attempt to change one is refused.
- **Instruments.** Several `epi-agent` instruments are marked research-only or unavailable, each with a reason.
- **Runtime acceptance.** Six capabilities have implemented contracts, but no provider or runtime has yet exercised them: service negotiation, service readings, client modes, wiki refraction, wiki portals and living wiki.
- **Long-running work.** The kernel rebuild (K8–K10), the Technē instruments, the physical-music line and the Epi world fidelity audit are open programmes.

## How it fits O:I

O:I gives each facet of an agent's world its own product. Quaternal Logic / MEF holds **reflection**: a formal way of reading the other five, asking what shaped an act and what should change. Reflection is optional. The other five products work without QL, and nothing in them requires a QL provider to be present.

How it meets its neighbours, through contracts rather than shared code (no sibling crate depends on a `ql-*` crate, and QL never reads another product's files directly):

| Facet / product | Contract | Direction |
|---|---|---|
| Capability / AIKit | AIKit calls the installed `ql` binary for capability negotiation (`aikit.ql-cli-provider/v1`). QL validates AIKit's operative syntax (`aikit.operative-resolve/v1`). | both |
| Development / Software Factory | Factory vendors `ql vak workflow-types` as the `@epilogos/ql-vak` workflow adapter. QL reads Factory's `factory.vak-orchestration/v1` and `factory.run-thought-consumption/v1` records as input. A weekly cross-product check confirms that classic Factory operation does not require a QL provider. | both |
| Agency / Actuation | Actuation binds an exact `ql` binary and calls `ql vak compose … --json` (`ql.vak-composition/v1`). | Actuation → QL |
| Ground / Central | The Nara instrument reads `central.document-reading/v1` results and `central.now-clearing/v1` references passed in by a host. It never writes to the person's source. | Central → QL |
| Environment / Workcell | No code connection today. | — |
| O:I | `ql system` and `ql config-contribution` emit `oi.product-settings-disclosure/v2` and `oi.configuration-contribution-v1`. `oi ql …` dispatches to `ql`. | QL → O:I |

Every reading of another product works on JSON that a caller or host supplies. Readers for the other products' records are not general yet. There are specific seams for Factory, AIKit and Central, and a generic Context-Frame reading of any six-part mapping (`ql.mef.context-frame-reading/1.0.0`), which never takes authority over what it reads.

**In the Cradle.** In the O:I desktop, QL appears as instruments that are present when invoked, and is never a prerequisite for ordinary use. The Cradle kernel takes QL's scene material and source basis as one of its owner readings.

## Install and quick start

The builds are pre-releases. `oi` itself reports that they "have not passed physical acceptance".

Through O:I (the ordinary route; this installs the suite's recorded build):

```sh
oi install quaternal-logic
ql verify
```

From source (developer route). This needs stable Rust 1.85 or newer, because the workspace uses edition 2024. It also needs Python at build time, which `ql-mef/build.rs` uses:

```sh
git clone https://github.com/EpiLogos/QL-MEF Quaternal-Logic
cd Quaternal-Logic
cargo install --path crates/ql-cli --locked     # or: cargo build --workspace --locked --release
ql verify                                        # expect: native verification: ok (kernel 0.1.0-q1, 7 checks)
```

The release page carries `quaternal-logic-v0.1.0-prelocal.6`. That release is a component archive recorded in O:I's suite manifest, not a standalone `ql` download.

First commands:

```sh
ql --help
ql mef lenses
ql context-frame list
ql kernel apply complement-address 'qladdr:sixfold@1/direct/P2/d0' --json
ql vak compose fixtures/kernel/vak-composition-v1.json --json
ql service capabilities --json
```

`ql vak compose` runs a native local → recursive → reframed → generated → Return → new-whole path. The returned JSON exposes actual participant refs, active frames, member/address selection, harmonic intervals, geometric phases, MEF positions, producing basis and explicit Return grounds. Agent-provided interpretation remains attributable and generated results remain `DERIVED`.

To verify from a checkout, run `bash scripts/verify full`. To run the test suite, run `cargo test --workspace --all-targets --locked`.

## Repository layout

| Path | What it holds |
|---|---|
| `crates/ql-core` | the deterministic kernel: positions, faces, addresses, forms, pairing, operators |
| `crates/ql-mef` | the MEF registry, lenses and sublenses, Context Frames, refraction and provenance contracts, matheme, Vāk composition, M1/M3 engines, scene and Nara field |
| `crates/ql-semantic` | transport-independent provider contracts |
| `crates/ql-service` | the service boundary: capabilities, locate, refract, relate, synthesise |
| `crates/ql-adapters` | optional client adapters for Factory and AIKit, which keep client identity |
| `crates/ql-wiki` | OKF meta-wiki support and the `ql-wiki-refraction` binary |
| `crates/ql-cli` | the `ql` binary |
| `c/`, `cpp/`, `vendor/epi-kernel` | native C/C++ kernel strata and the frozen C specimen kept for parity |
| `providers/`, `adapters/`, `skills/`, `workflows/`, `schemas/`, `fixtures/` | Python providers, client adapters, QL-owned Skills (catalogued by AIKit as `skill/ql/…`), workflows, JSON schemas and test fixtures |

The seam documents are `docs/Q1-DETERMINISTIC-KERNEL.md`, `docs/Q2-MEF-REGISTRY.md`, `docs/Q3-PROVIDER-SERVICE.md` and `docs/Q4-CLIENT-ADAPTERS.md`. [Architecture navigation](docs/ARCHITECTURE-NAVIGATION.md) connects each formal or native operation to its scene or source consumer, and to the ongoing Epi fidelity audit. It does not settle numerical or domain mappings, and it does not promote a generic profile reading over the coordinate-bound world and Personal Pratibimba.

---

## The epistemic discipline: operational parity

A QL name is not an implementation result.

If a formal distinction is supposed to matter technically, it should eventually produce an **operationally discriminable consequence** rather than functioning as decorative terminology.

For example, attaching:

```text
position = 5
meaning = return
```

to a record does not by itself establish a meaningful return operation.

A stronger implementation would make a difference such as:

```text
encountered result
      ↓
retained attributable difference
      ↓
recognition / determination
      ↓
subsequent ground is actually conditioned by that difference
```

The question is always:

> **Where does the claimed formal relation remain active in the actual operation, and where is it only named?**

This is the project's research firewall. It lets the software get deeper without confusing semantic resemblance with proof.

## Negative and null results are legitimate

The programme is experimental, not a predetermined demonstration that QL must win.

Several outcomes are informative:

- a classic approach can outperform a QL-informed approach;
- two QL forms proposed as different can prove operationally equivalent;
- a QL distinction can improve explanation, navigation or human understanding without improving execution;
- a formal relation can remain research-only because no adequate computational contract has been found;
- an implementation can falsify a proposed software correspondence without settling the underlying metaphysical question.

The code therefore tests **technical articulations of the research**. It does not prove the metaphysics simply by compiling, producing a pattern, or passing a deterministic fixture.

## Minimal O:I does not require QL

Quaternal Logic is one native centre in the O:I research field, not a prerequisite for technological agency.

A minimal O:I can consist of durable authored ground plus actuated model capacity. AIKit, Factory, Workcell and other clients must remain coherent when no QL provider is present.

Maximal research can use QL deeply: formal location, relation, refraction, synthesis, QL-native recurrence experiments, MEF readings, structural Wiki fields, or later operators whose operational meaning becomes sufficiently exact.

This asymmetry is important. Optionality protects both sides: ordinary software is not held hostage to unfinished formal research, while QL research is not forced to pretend every distinction has already earned a production runtime meaning.

## Alignment and refraction

Client objects remain themselves when QL is applied to them.

A Factory `Run`, AIKit `Resource`, Wiki object or other subject is not renamed into a QL noun merely because it is being analysed through QL/MEF. The product returns a reading **of that subject**, preserving the subject's native identity and provenance.

This is what the existing shorthand means:

> **Alignment, not translation. Refraction, not renaming.**

The reason is structural: a lens is supposed to reveal another relation of an existing whole. If applying the lens destroys the original identity and replaces it with a parallel QL ontology, there is no longer a stable subject whose different readings can be compared.

## MEF as a manifold of disclosure

MEF supplies twelve lenses, six Day and six Night, each with an internal sixfold articulation. The lenses are intended as refractive perspectives over a subject, not twelve buckets into which software objects must be permanently classified.

Different lenses can disclose different relations — investigative, causal, logical, processual, chronological, phenomenological, scientific/knowledge-work, expressive and further tradition-specific readings — while the subject retains one identity.

The executable product therefore separates:

```text
subject identity
≠ lens identity
≠ provider identity
≠ reading
≠ provenance
```

That separation is what allows several providers or experimental methods to produce comparable readings without rewriting the client system.

## Current product boundary

QL/MEF owns:

- canonical executable QL references, forms, addresses and sufficiently specified deterministic operators;
- the complete twelve-lens MEF registry and sublens/refraction contracts;
- QL/MEF provider capability negotiation;
- `locate`, `refract`, `relate` and `synthesise` service operations where implemented by the accepted product line;
- provider and reading provenance;
- evidence-led promotion of deeper operators.

It does **not** own:

- Factory `Project`, `Run`, `Action`, Candidate or Recognition identity;
- AIKit `ContextResolution`, model/harness/session or Surface ownership;
- Actuation Agent/Agency/determination/Return semantics;
- Workcell materialisation;
- Central Control;
- the QL Loop Runtime's host recurrence mechanics merely because that runtime can consume QL readings.

## Canon, executable formalisation and research

The project keeps several layers distinct:

```text
QL / Epi-Logos source and canon
    formal and philosophical propositions
            ↓ sufficiently specified subset
executable QL/MEF product
    typed refs · operators · lens registry · provider/service contracts
            ↓ client integration
Factory / AIKit / Wiki / experiment / other client
            ↓ observed consequence
Evidence
            ↺ may revise the software correspondence
```

The executable product is versioned and developable. A later implementation can improve the technical articulation without claiming that the philosophical source itself was versioned like a library.

Conversely, source provenance does not excuse weak software semantics. Once a relation is promoted into the executable product, its contract and evidence must be explicit enough for another client to test.

## Development direction

The staged product direction remains:

```text
specified deterministic kernel
        ↓
MEF registry and refraction contracts
        ↓
provider / service operations
        ↓
client adapters
        ↓
evidence-led runtime and deeper-operator integration
```

This is a promotion/dependency sequence, not a second ontology or a claim that Context Frames first arise at MEF. The faithful L5 projection above governs the whole relation.

The order is epistemic as well as technical: establish what can be stated deterministically, then expose provider seams for interpretations that require more than the structural kernel, then let real clients and experiments pressure the distinction.

## Foundational executable kernel

The current executable formal field is shared across the Rust and source-proven C bodies. Its compact human reference is [`docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md`](docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md), paired with `fixtures/kernel/holographic-kernel-contract-v1.tsv`. The same kernel now exposes the versioned `ql.shape@1.0.0` contract in `fixtures/kernel/ql-shape-contract-v1.json`: positive partial/developed wholes, A/B/C pair provenance, D1→D3 conjugate completion, D3 `4×4`, direct/conjugate `6×6`, `6 / 6′ → 6+6′` relational-generation sites and Return through the `0/1` anchor.

Vāk, MEF, Context Frames, SemanticWiki clients and richer harmonic development all operate or refract this same kernel field; they do not establish parallel shape vocabularies.

## QL Kernel: the architectural entry for maintainers

**QL Kernel** names the whole canonical object, not merely `ql-core`. Begin with [QL Kernel — Vāk, C′ and Context-Frame reconciliation](docs/QL-VAK-KERNEL-RECONCILIATION.md), the current #123 architecture carrier:

```text
QL Kernel
  L5-0  Syntax
  L5-1  Root = psychoids + coordinate-family heads
  L5-2  Harmonics — Context Frames already active here
  L5-3  Geometry
  L5-4  Meta Epistemic Framework
  L5-5  Techne
```

L5 is the concrescence of the existing **L0 / L0′ / L5′ / L5 articulation square**, not a speech subsystem around another kernel. These offices project onto existing MEF refs without renaming source Anuttara, Para Vāk or their sublens labels. Context-Frame identity propagates from harmonic determination into Geometry, Meta Epistemic Framework and generative use; scoped runtime propagation is executable through `ql_mef::vak_composition`, including recursive fields, explicit CP/CF operations, generative determinations and checked Return.

The reconciliation distinguishes the full 109-node Anuttara language, AIKit's already-owned general Search/Resolve grammar, C′ reflective context, Ta-Onta S′ world composition and native Factory orchestration. [The accepted #83 receipt](docs/integrations/epi-logos/EPI-VAK-ACCEPTANCE-RECEIPT.md) governs the returned Vāk implementation. QL-MEF #138 supplies the executable producer contract. AIKit #267, Factory #217 and O:I #216 are separate downstream applications, not this feature's execution sequence or completion gates.

Development Field #122 has separately landed its [portable structural carrier](docs/QL-STRUCTURAL-CARRIER-CONTRACT-V1.md) in PR #137, main commit `10fb4bade8ad6a6df67f80cb6ef913592ac654be`. `ShapeBinding`, `RelationFieldComposition` and `RelationFieldDerivation` supply attributable form, non-fabricating addresses and complete Return provenance. The implemented C′ operations reuse that accepted seam; composed fields remain addressable wholes when nested and after Return, without a replacement shape ontology.

## Current development state

The current Rust product exposes `ql-core`, `ql-mef`, `ql-semantic`, `ql-service`, `ql-adapters`, `ql-wiki` and the `ql` CLI, with adapters and fixtures as warranted by the accepted development programme. Native C is also an active structural implementation stratum: #135 and its #124–#134 ladder govern the coordinate-complete C/Rust/C++/Bimba rebuild. The frozen vendored C specimen is historical/parity evidence; that does not make the native C centre permanently historical.

Current `main` and accepted evidence determine present implementation truth. Accepted #83/#142 Vāk/Search work is inherited; C′ composition is callable through the QL library/CLI, while native C restoration, deeper M coverage, C++ embodiment and downstream suite integration retain their distinct ownership in the reconciliation above. Unmerged, stacked or draft work remains branch-qualified evidence, not accepted capability merely because its own CI is green.

In particular:

- structural or conformance evidence does not automatically establish LLM capability improvement;
- research-only operators remain research-only until explicitly promoted;
- the Epi/O:I living relation map is a research/review checkpoint, not a new generic O:I ontology;
- independent client identities, Bimba graph identity and provider projection identities must remain distinct.

## Repository ownership

This repository is the implementation repository for the standalone QL/MEF product.

The product architecture was developed in `EpiLogos/Factory` (renamed from `agent-system-design`), especially the standalone QL/MEF design package under its canonical documentation. Factory-side design history remains important provenance, while executable product code, product-local fixtures, releases and product implementation evidence belong here.

The move to this repository does not merge QL/MEF with the QL Loop Runtime. Runtime recurrence is a separate experimental concern now developed under Actuation and consumes this product only through explicit versioned seams.

## Read with

- the canonical standalone QL/MEF design package in [`EpiLogos/Factory`](https://github.com/EpiLogos/Factory/tree/main/docs/canon/ql-mef-module);
- the wider Epi-Logos source work in [`EpiLogos/Epi-Logos-C-Experiments`](https://github.com/EpiLogos/Epi-Logos-C-Experiments);
- the active issue/PR programme in this repository for current development state.

The governing test for product language is the same as for implementation: **do not let a correct formal name perform explanatory work that only an actual relation can justify.**

---

## Background

O:I stands for Objective : Internality. It names the means through which a life knows and acts within a world: memory, language, tools, permissions and other people. Those means are internal because every act proceeds through them, and objective because each can be examined and changed. Quaternal Logic is the formal account of that relation developed in the essay [*Confronting the Limit: Determination, Subjectivity and Mind as Objective Internality*](https://oi.epi-logos.org/essay/).

Quaternal Logic arises from a longer Epi-Logos inquiry into mind and reality whose sources include **depth psychology, Eastern metaphysics, recursive/archetypal relational work, phenomenology, process thought and related traditions**.

The programme investigates awareness, relation, interiority, manifestation, mediation, return and agency from a starting point that is not identical with the prevailing material/computational ontology of the contemporary AI industry.

This does not mean the software is entitled to treat those philosophical sources as established scientific facts. It means they generated distinctions and formal propositions that can now be made more explicit, implemented where possible, and exposed to technical consequence.

**Objective Internality** is one important bridge into that work. O:I uses the term operationally for the objectively inspectable structures that become internal to an actor's situated operation. Epi-Logos asks the deeper question of how such a relation sits inside a wider account of mind and interiority. Quaternal Logic supplies a formal research field in which those questions can be articulated without requiring ordinary O:I software to settle them.

The deeper Epi source and domain work remains in [`EpiLogos/Epi-Logos-C-Experiments`](https://github.com/EpiLogos/Epi-Logos-C-Experiments). This repository should consume that provenance without silently promoting every source proposition into executable canon.
