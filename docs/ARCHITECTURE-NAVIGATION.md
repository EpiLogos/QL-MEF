---
role: architecture
standing: agent-inference
scope: QL-MEF native operations and composed O:I consumer boundaries
updated: 2026-10-01
---
# QL-MEF architecture navigation

This is an implementation-facing navigation companion for O:I #65/#220 and the
existing documentation programme. It recovers native owners and successors;
it does not adopt a new design or claim the whole running experience complete.
Inspected native checkout revision: `74dc6dc066fb5d9854657305045e3a786956b11a`. Active repair source may advance
that cut; identify the file revision before relying on its returned result.

## Governing source and successors

- [KERNEL-REBUILD-WAYFINDER](KERNEL-REBUILD-WAYFINDER.md)
- [M123-SCENE-STRUCTURAL-MAP](kernel-rebuild/M123-SCENE-STRUCTURAL-MAP.md)
- [LIVING-INSTRUMENT-ARCHITECTURE](kernel-rebuild/LIVING-INSTRUMENT-ARCHITECTURE.md)

Directory names are routes, not authority. Target design, amendment, historical
baseline, current implementation and observed result keep their own standing.

| Concern | Public operation / entry | Native source | Boundary and lifecycle |
| --- | --- | --- | --- |
| Formal operation discovery | `ql capabilities --json`; `ql kernel capabilities --json` | `crates/ql-cli/src/lib.rs`; `crates/ql-mef/src/lib.rs` | CLI dispatch and formal kernel exports are distinct from scene state or O:I rendering. Availability of a provider-backed reading must be negotiated separately. |
| Context-frame registry | `ql context-frame list --json`; ContextFrameId | `crates/ql-cli/src/lib.rs`; `crates/ql-mef/src/context_frame.rs` | Registry reading selects a defined formal cut; it does not change the originating subject or adopt the personal-world mappings. CLI tests exercise execute_cli and the real front-door binary. |
| Scene material | Native scene-world construction / instrument state | `crates/ql-mef/src/scene.rs`; `crates/ql-mef/tests/scene_instrument.rs` | Native structural/computational state, retained Expression state and installed lived encounter keep separate proof. |
| Coordinate-bound world / Personal Pratibimba | Read the [ongoing fidelity programme](https://github.com/EpiLogos/QL-MEF/issues/258) and its current source/recovery return | The programme's named active audit cut, qualified below | The audit owns numerical/domain mappings. Shared locus, private person, actual occasion and Expression instance remain distinct; do not substitute a generic profile page. |
| Consumer / evidence | Native scene-world material and source basis → O:I Expression | O:I `desktop/cradle/kernel/src/expression_world.rs`; current domain audit | Source coverage, finite computational proof, retained save/readback and installed lived encounter are different acceptance cuts. |

## Diagram and consumer relation

The maintained suite companion is
`source:project:O-I:docs/architecture/shared-participation.md`; its editable diagram is
`source:project:O-I:docs/architecture/shared-participation.mmd`. The O:I architecture entry
contains six question-specific companions, full-size rendered SVGs, an indexed
basis for every arrow, exact source hashes and independent navigation evidence.
Resolve that source in the current O:I checkout before substituting a cached or
historical copy. Its solid arrows are inspected relations, not installed
acceptance; proposed joins remain explicitly proposed.

Existing capability records link this companion through the optional
`extensions.documentation` protocol. These links change discoverability, not
capability IDs, coordinate placements, implementation status or source authority.

## Verification and open joins

The audit document is still unpublished at this navigation cut. Its owner is
working on `map/m3-5-clock-inscription`; the audit is absent from this PR and
the inspected `74dc6dc` baseline. Begin at QL-MEF #258 and use the owner's
latest source/candidate return rather than assuming a repository-local file
exists. The consumed audit bytes have SHA-256
`8daa594183b0875da3755f01c33301e14fb751c5dce5d17c42e1b7dc35fae430`.
Replace the active-source locator with its published revision when that owner
lands the audit. This disclosure is separate from the O:I diagram dependency.

The ongoing fidelity audit distinguishes a coordinate-bound world and Personal
Pratibimba from a generic profile. Its latest return reports published native
oracle-journey and lived-context operations, with O:I/Central receiving still
missing in the inspected consumers. It also retains ordinary run 15's bounded
native loading proof and subsequent `files_list` refusal after autosave CAS.
Neither native JSON nor an acknowledged CAS proves ordinary file/restart
continuity. Consume the audit's exact source/operation refs and qualification;
this navigation account determines no numerical or domain mapping.

Read each native test and its actual runner conditions, then the corresponding
dated Return. A test definition is not an executed result; a process/receipt is
not human Recognition. The suite's architecture verification records real
Mermaid rendering, source/link checks and the fresh-agent navigation task.
The four repair lanes continue to own their code, installed replay and open
architectural decisions. Preserve a missing join as missing until that proof
or decision is returned.
