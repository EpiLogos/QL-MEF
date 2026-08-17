# Epi-Logos R2 relation map — first-pass review checkpoint

**Status:** first pass; **awaiting human architectural review**  
**Tracking:** QL-MEF #25; Epi source coordination #4; parent Epi #2  
**Acceptance:** not claimed. Do not close #25 or derive implementation tranches from this snapshot without review.

This document is the readable synthesis of the machine maps beside it. It records the current interpretation reached from accepted R1 source, live product contracts, current successor work, and direct inspection of S4′/`ta-onta`. It is deliberately revisable.

## 1. Source line actually consumed

### Epi source

- accepted R1 merge: `EpiLogos/Epi-Logos-C-Experiments` `be54a505728eaa06ddcc268fa53df5dd756bfb5e`
- R1 source-reading head: `8608648f33e697dd5a8c5f499492619a02259af5`
- accepted inputs: source manifest, M/M′ capability matrix, S/S′ technical matrix, Bimba inventory, legacy ledger, machine capability inventory
- live Epi `main` at this pass is the same R1 merge

### Living-map home

- QL-MEF `main`: `a2770024ad7ecce379a5b8dc4ed07df3e425ed05`
- that is PR #26's accepted living-map home; #25 remains open
- QL-MEF #7 is closed and fixes the Meta-Knowledge Graph Projection ≠ Bimba boundary

### O:I relational-development line

- O:I accepted `main`: `44efc5d252c3b9f3a07cda719efbf9704dac2c24`
- O:I #29 remains the relational-field programme
- PR #30 is still **draft**, head `ca78ec48737fcdcb9a57d389d8f1e1977db23f9d`
- PR #30 targets `research/cordis-composable-agency` at `9ae0345e594c2cbe0502cbfeb4c73283eef8aefc`, not `main`
- therefore PR #30 is consumed as the current developmental 12×12 reading, not accepted product ownership authority

### Live accepted product heads used

| O:I | Product | accepted `main` read |
|---|---|---|
| P0 | Central | `78a545214ad70e055fae38ccae2d78443112f283` |
| P1 | Actuation | `03e03acb1ae47d60f26903209112258e60f83627` |
| P2 | AIKit | `43eafa86437b528162a93c09c05399a137f8d6b9` |
| P3 | Software Factory | `621624c7c8a9919604ad36db451f1a30806fc0ce` |
| P4 | Workcell | `9277fdc852a468f1c53f62b766a38e06705fe482` |
| P5 | Quaternal Logic | `a2770024ad7ecce379a5b8dc4ed07df3e425ed05` |

Central, Actuation, Factory and Workcell are on the accepted O:I `pre-local.2` cut. AIKit has advanced beyond that cut with merged Knowledge Navigation operation parity (#79). Current draft successor evidence is used only where it sharpens a boundary without being mistaken for accepted main, especially AIKit #78/#80/#81, Actuation #6, and Factory #142.

---

## 2. A — What the M/M′ field appears to be

The strongest current reading is that **M0–M5 are six semantic/domain owners, while M0′–M5′ are their substantial lived/software expressions**. They are not S-layer names and they are not aliases for the six O:I products.

| Domain | Present software character | First-pass boundary reading |
|---|---|---|
| M0 Anuttara | Bimba language, coordinate identity, relation/pointer field, canonical routes | semantic-graph/domain API + navigation surface |
| M1 Paramasiva | mathematical/harmonic engine, walk/instance state, Spanda, topology | independently testable mathematical/instrument package |
| M2 Parashakti | 72-space correspondences, Vimarsha audio genesis, elemental/decanic/sonic field | independently testable correspondential/audio package |
| M3 Mahamaya | 72→64 reception, symbolic codec, rotation, clock/world-clock | independently testable symbolic/transduction package |
| M4 Nara | protected identity, DAY/NOW lived field, oracle, activity, episodic memory, proposals | stateful privacy-sensitive application/domain package |
| M5 Epii | pedagogy, canon, backend studio, reflected app, control room, Atelier/return | integrative application/service package |

This is stronger than “six UI tabs”. Each domain has source authority, typed capabilities, cross-domain contracts and independent testing seams. What is **not** established is that every one deserves a separate repository or independently deployed product.

The most load-bearing relations currently include:

- M2→M3 `72 → DET → 64`: semantic boundary, not a rendering detail.
- M2→M1 shared Vimarsha audio genesis: real writer/consumer relation.
- M3 symbolic facts can become M4 priors without obtaining personal-write authority.
- M4→M5 is proposal/evidence/review handoff, not automatic promotion.
- M5 return pressure can propose changes across M0–M4 but cannot self-promote into their canon.

The selected evidence-bearing internal field lives in `epi-mmprime-relations.json`; it deliberately does not manufacture a complete 6×6 matrix.

---

## 3. B — What base S appears to have become relative to modern O:I

The base S field is best read as the historical/current **capability ground that made Epi executable**, not as six Epi semantic products.

### S0 — command/kernel/material execution ground

Generic surviving functions are executable/tool discovery, process execution, build/bootstrap and compiled runtime materialisation. AIKit now owns much of operational resolution, Workcell owns physical materialisation, and Factory owns the specifically developmental use of execution. Epi still needs the M0–M5 kernels and coordinate-native domain operations.

### S1 — authored source/vault residency

Generic surviving functions are durable authored files, Markdown/frontmatter/wiki relations, source query/injection and safe mutation. Central now provides human-authored durable ground, AIKit provides generic Knowledge Navigation, and Factory owns project canon/artifacts in development. Hen's Epi-specific CT/residency/content law and Epi canon/vault mutation semantics remain a non-generic residue.

### S2 — graph/semantic/knowledge structure

Generic graph persistence, algorithms, search, reading, routing and provenance can now sit behind AIKit Knowledge operations and Workcell material providers. Bimba identity, coordinate/relation meaning, namespace law and write authority remain Epi. Neo4j is a provider, not the graph's semantic identity.

### S3 — live session/gateway/temporal coordination

Generic sessions, connections, surfaces, service reachability and lifecycle increasingly fit AIKit SessionSpace/AgentSession/connection seams plus Workcell process/service/network lifecycle. DAY/NOW/Kairos meaning and protected episodic semantics remain specifically Epi and still need a clean modern owner.

### S4 — agent runtime/capability governance

Generic Agency/plurality belongs in Actuation; model/harness/capability/session/Surface resolution belongs in AIKit; material hosting belongs in Workcell; Factory owns a developmental reason where the act is a Run. Epi agent identities, VAK/CF semantics if retained, and Epi review/crystallisation routing remain Epi-specific.

### S5 — integration/knowledge return

Generic retrieval/source pools fit AIKit; developmental review/evidence fits Factory; authored recognition/proposal patterns may reuse Central; outward projection/federation fits O:I. Epii pedagogy, Logos Atelier, Bimba/Gnosis meaning and Epi-specific canon return remain Epi.

**Conclusion:** much of base S is being decomposed into modern O:I substrate, but this is capability absorption, not numbered S→P identity. The Epi-specific semantic remainder is substantial.

---

## 4. C — What S′ still contributes beyond generic substrate

S′ is not merely obsolete augmentation. It contains rules by which generic bodies became specifically Epi-aware: coordinate-aware graph law, Hen content/residency law, DAY/NOW/Kairos temporal law, capability/inhabitation law, Epi-aware agent dispatch/evaluation, and Gnosis/return/promotion law.

The useful R2 factorisation is:

```text
generic mechanism
    → reuse/move through O:I owner

Epi semantic policy/profile/application law
    → remain in modern Epi software

provider-specific adapter
    → retain or replace behind the stable seam
```

The later implementation question is therefore not “which S′ directories survive?” but “which S′ invariants must be re-expressed after generic mechanism extraction?”

---

## 5. D — What direct `ta-onta` inspection reveals

`ta-onta` is concrete code, not only planning taxonomy. `composite-entry.ts` registers Khora, Hen, Pleroma, Chronos, Anima and Aletheia spine contributions, wires session-start/shutdown/compaction injection/extraction, and loads all six extension bodies.

**Pleroma** explicitly defines itself as the execution substrate registry. It owns bounded primitive/tool registration and execution-mode enforcement, and explicitly does not own orchestration, vault content or knowledge promotion. That maps naturally toward AIKit capability resolution, Workcell material execution, and provider adapters.

**Anima** explicitly owns orchestration: VAK evaluation, CF dispatch, team/chain/subagent execution and runtime phase shaping. Its own architecture already distinguishes raw PI extension tools, portable Skills and subagent identities. Generic Agency/plurality belongs below this in Actuation; harness/tool/session projection belongs in AIKit. VAK/CF and named Epi constitutional semantics are the candidate Epi-specific remainder, not a reason to move generic orchestration wholesale into Epi.

**Aletheia** explicitly separates Gnosis from Bimba and treats retrieval/crystallisation, thought routing and Epi return as its domain while disclaiming dispatch, vault CRUD, session identity and scheduling. Its generic retrieval/provider portion can be extracted; its Epi T-family, Bimba/Gnosis and canon-return meanings remain domain-specific.

The architectural consequence is strong: `ta-onta` is a grounded parallel architecture, but the evidence argues against porting it monolithically. Its internal contracts already separate concerns now distributed among Actuation, AIKit, Workcell, Factory and modern Epi domain code.

---

## 6. E — Technology/body versus surviving capability

The machine ledger is `technology-capability-disposition.json`. Its central readings are:

- **Obsidian**: useful authored-knowledge body; not wiki semantics or Epi source authority.
- **Neo4j**: useful Bimba graph-store provider; not Bimba identity.
- **MCP**: protocol adapter; not the Bimba domain API or semantic boundary.
- **Redis/Graphiti**: useful live/episodic providers; not temporal or personal semantic owners.
- **PI**: valuable harness provider; not Agent/Agency identity.
- **tmux/cmux**: AIKit SessionSpace providers; not session semantics.
- **Tauri/Theia**: migration/shell evidence; not future M′ authority merely because historical M′ surfaces used them.
- **RAG-Anything/LightRAG/MinerU**: provider bodies behind Gnosis/knowledge operations; not truth or pedagogy semantics.
- **ta-onta**: mixed proof body; extract its generic mechanisms without discarding Epi-specific semantics.

---

## 7. F — Bimba integration shape

The current bridge is:

```text
Epi Bimba semantic/source identity
        │
        ├── materialises through Neo4j/S2 providers
        ├── may expose domain operations through MCP or other adapters
        ├── federates into AIKit Knowledge Navigation as a provider
        ├── may be mapped/refracted by QL-MEF through explicit BimbaBinding/MetaBinding
        ├── may be projected into O:I Explore/CommonReferent/SharedField
        └── may be materialised/operated by Workcell and developed by Factory
```

Every arrow preserves Epi source/write authority.

The strongest near-term generic integration seam appears to be an **AIKit Bimba KnowledgeProvider/application adapter** supporting provider-neutral search/read/relations/route/frame/sources/explain over opaque Bimba refs. That should be read-first and should not imply mutation authority.

QL-MEF should map/refract Bimba through explicit bridge records and derived readings. Its Meta-Knowledge Graph remains a distinct object.

---

## 8. G — Provisional modern Epi package/product topology

At **package/service boundary** altitude, all six M′ domains look substantial enough to preserve independent ownership and tests:

```text
epi-anuttara      M0′  Bimba/domain graph + coordinate navigation
epi-paramasiva    M1′  mathematical/harmonic engine + instrument state
epi-parashakti    M2′  correspondential/MEF/audio domain
epi-mahamaya      M3′  symbolic codec/transduction/world-clock
epi-nara          M4′  protected personal/lived application domain
epi-epii          M5′  pedagogy/canon/studio/control/Atelier return
        \          |          /
          unified Epi application / field
```

Those names are descriptive only; they are not repository-creation instructions.

Evidence **for** a 6+1 shape is real: each M′ has a distinct semantic owner and capability surface; M4 has a strong privacy/state boundary; M0 has a canonical graph/domain boundary; M1–M3 contain independently testable engines; M5 has application/service responsibilities beyond generic O:I substrate; and generic infrastructure extraction makes the six domain bodies clearer.

Evidence **against** prematurely declaring six products/repos plus one application is also real: M1–M3 may be better as isolated packages in one Epi workspace; some surfaces project shared source/profile state rather than independent persistence; independent release/deployment requirements are not proven; and M5′ already has reflected-app/control-room/return responsibilities.

The key unresolved question is: **what semantic responsibility belongs to a seventh unified Epi application that is not already M5′/Epii?**

Three readings remain plausible:

1. true 6+1: M5′ is one peer domain and a distinct identity-neutral Epi application composes all six;
2. 5+Epii: M5′/Epii is itself the semantic composing/return application;
3. six domain packages plus a thin composition shell with no seventh semantic owner.

The current evidence makes **option 3 the safest provisional reading**. It preserves six substantial domain bodies without inventing a seventh semantic owner. This must remain a review judgment, not a target architecture, until human intent is incorporated.

---

## 9. H — Most consequential uncertainties / judgment calls

1. Is M5′ a peer domain under a seventh application, or already the whole-system semantic integrator?
2. Does independent M′ software imply package/service ownership, or independently released product/repository identity?
3. Should modern M0 own a canonical Bimba domain API directly, with Neo4j/MCP strictly behind it?
4. Which deterministic QL/MEF operators should M1/M2 consume from QL-MEF, versus remain Epi-native mathematics/correspondence?
5. Where does shared Epi DAY/NOW/Kairos semantic state live after generic S3 mechanics move behind AIKit/Workcell?
6. What is Hen's surviving Epi-specific authored-content law after generic wiki/source extraction?
7. Where should Aletheia/Gnosis crystallisation live once generic retrieval becomes provider substrate?
8. Is ta-onta VAK/CF enduring Epi agency semantics, an experimental Actuation profile, or both?
9. How should M4 personal ground bridge Central without Central acquiring Nara identity/write ownership?
10. How much of O:I PR #30 should R2 consume before that branch is accepted onto an accepted ancestry?

---

## 10. I — Suspected stale/conflicting architecture

The clearest developmental strata that must not be flattened are:

- old S0–S5 wording that treated the six named Epi agents/domains as S positions;
- Theia/Tauri-era wording that turns a historical shell into future M′ authority;
- `ta-onta` documentation that still says e.g. “Position #2 (Parashakti)” inside S4′ despite the current Cycle-3 correction placing the named semantic domains in M, not S;
- Pleroma-era counts/comments that say nine primitives while its current contract enumerates seven;
- older QL/MEF language assigning one universal software semantic to raw QL positions, versus current QL-MEF invariant-position/refraction distinctions;
- O:I PR #30 labelling its field document “canonical architectural framing” while the PR itself remains draft and is based on a research branch rather than `main`.

These are not all bugs. Several are legitimate design history. The living map should preserve provenance while stating the current operative distinction.

---

## 11. First-pass coverage

This branch records:

- **36 / 36** R1 M′ stable capability refs in the embodiment and O:I cross maps;
- **18** high-value internal M↔M relations, intentionally not a filled 6×6 matrix;
- **6** S/S′ stratum continuity readings;
- **8** explicit Bimba bridge bindings plus distinct knowledge-function categories;
- **13** technology/provider disposition records.

These are coverage counts, not acceptance statistics. Parity labels remain provisional until human review and any resulting source reread.

## 12. Review questions for the human author

The first correction pass should focus on:

1. Is M5′/Epii itself intended as the semantic integrator of the other five M′ domains, or should it remain a peer under a distinct Epi application?
2. Does “independent M′ software” mean primarily package/service ownership, or is independent product/repository identity part of the intended future?
3. Where should shared Epi DAY/NOW/Kairos semantic service live after generic S3 mechanics move behind AIKit/Workcell?
4. Is Hen best understood as a surviving Epi-specific authored-content law/service, or should more of it become generic source/wiki provider contract?
5. Should VAK/CF in `ta-onta` be treated as enduring Epi agency semantics, an experimental profile over generic Actuation, or a source of patterns to re-derive selectively?
6. Does the first-pass reading of M0 as likely owner of a clean canonical Bimba domain API fit the intended Bimba evolution?

No implementation tranche or native ticket creation should proceed from this branch until these judgments have been reviewed.
