from pathlib import Path
import re

def edit(path,old,new):
 p=Path(path);s=p.read_text();assert s.count(old)==1,(path,old[:100],s.count(old));p.write_text(s.replace(old,new))
def paragraph(path,prefix,new):
 p=Path(path);s=p.read_text();pattern=r'(?m)^'+re.escape(prefix)+r'[^\n]*(?:\n(?!\n|#)[^\n]+)*';s,n=re.subn(pattern,lambda m:new,s);assert n==1,(path,prefix,n);p.write_text(s)
p='docs/QL-VAK-KERNEL-RECONCILIATION.md'
paragraph(p,'**Standing:**','**Standing:** canonical architectural reconciliation and executable QL-owned C′ composition contract. Native QL verification is separate from downstream installation, native Agent execution and human acceptance.  ')
edit(p,'Keep seven epistemic standings separate:','Keep epistemic standings separate:')
paragraph(p,"Parentheses in AIKit's",'''Parentheses in AIKit's `Frame { expression }` preserve nested syntax. QL `FullVakBinding` supplies the explicit semantic correlation through the caller's native node ref, ResolvePath identity, accepted syntax revision, source-grounded reading, self↔other form, contextual field and interpreter. This is interpretation of the existing AST, not another AST/parser. A readable scope does not authorize an Action.''')
paragraph(p,'The source-recovered C′ entry is especially important.', '''The source-recovered C′ entry is especially important. The vendored `include/ontology.h` describes the P4 lemniscate entry through CPF/CT/CP/CF/CFP/CS; `families.c` supplies historical reflective-slot evidence. `CPrimeContext` now operationalises that reflective relation over an explicit whole-ground and existing QL coordinate. C′ is not a seventh base family, nor merely C's conjugate face. The operations in §7 reuse existing kernel family/relation IDs; they do not restore or replace the native C centre owned by #125–#127. Historical pointer layout remains implementation evidence, not a runtime storage prescription.''')
edit(p,'**Current harmonic identity conformance is implemented. Full scoped runtime propagation is not established by that test and remains #138.**','**The scoped operation is implemented in `vak_composition`: the same `FramedReading` carries the actual selected member/address, active frame, harmonic determination, derived geometry and MEF view into `Determination` and `Returned`.**')
edit(p,'#122 owns the Development Field carrier and portable relation-field binding; this document does not preempt its concrete types.','#122/#137 supplied the accepted Development Field carrier; the contextual operations reuse `ShapeBinding`, `RelationFieldComposition`, `RelationFieldDerivation` and `AnchorReturn`.')
s=Path(p).read_text();s=s[:s.index('## 7. C′ through Ta-Onta:')]+'''## 7. Operative C′: one reflective relation, six distinct operations

The current implementation is `ql_mef::vak_composition`, with public contract `ql.vak-composition/v1`. This operational module belongs to the one QL Kernel; it is not a Vāk kernel wrapped around another kernel. Its structs compose the accepted structural carrier with existing Context Frames, music, MEF rotation and the full Vāk registry.

C is the existing category-family head of a positioned whole. C′ is that whole's **explicit reflective/contextual use**, carrying its ground, current focus, discriminations, source-field selection, frame and compositional thread. A direct/conjugate face is one determination within that relation, not the definition of C′. No cast relates the reflective-family slots to L5 offices, base family heads, M roots, S′ offices, Śiva operators or Śakti horizons.

| Family / accepted relation ID | Executable operation | Actual consequence |
|---|---|---|
| CPF / `ql.kernel.vak.cpf/v1` | `CPrimeContext::cpf` | Explicitly selects the direct/conjugate leading use and eligible Vāk operations. Excluded operations cannot determine through this context. Inversion preserves the position; it is not positional complement. |
| CT / `ql.kernel.vak.ct/v1` | `CPrimeContext::ct` | Selects eligible source-qualified `VakContextField` content. Both producing and inherited/thread readings must be admitted. These seven source fields are not the seven CF identities. |
| CP / `ql.kernel.vak.cp/v1` | `cp`, `cp_at`, `position`, `position_member` | Traverses the actual recursive participant relation, then can address a disclosed member with explicit local/absolute basis. An explicit member remains itself when CF/lens changes. |
| CF / `ql.kernel.context-frame/v1` | `cf`, `reframe` | Produces an immutable new framed use. The active canonical cut governs selection and harmonic settlement; the transition and source basis remain available. |
| CFP / `ql.kernel.vak.cfp/v1` | `CPrimeContext::cfp` | Binds every ordered step of an actual source `VakRPath` to an explicitly interpreted whole/path, computes its framed readings and retains them in the producing determination. This is not a scheduler. |
| CS / `ql.kernel.vak.cs/v1` | `CPrimeContext::cs` | Completes the declared contextual thread through a determination and an explicit checked ground/whole-anchor Return. It does not guess a destination from position five. |

The operative CT meaning is **Context-Type/content-field selection**. Historical Context-Time expansions remain source descriptions, not mandatory aliases in this contract. The operative CS is **Context-Sequence/completion**; a historical Context-System spelling does not define another current object. CFP is source-qualified compositional threading; the obsolete `Z` literal is not an unimplemented mandatory thread. Old payload layouts have been resolved against current meaning rather than reproduced.

The current seven CFs are settled, exactly as listed in §5. The old Ta-Onta `(4/5/0)` occurrence is **superseded/noncanonical historical material**, outside the canonical set. It is neither an eighth frame, an alias, a missing operation nor a blocker. Exact lookup continues to return no canonical identity for that occurrence.

### Ta-Onta remains the ratified S′ whole

The [current Ta-Onta capability matrix](integrations/epi-logos/EPI-TA-ONTA-AGENT-WORLD-CAPABILITY-MATRIX.md) governs: S0′ Khora is ground/continuity; S1′ Hen semantic/artifact form; S2′ Pleroma operative affordances; S3′ Chronos temporal becoming; S4′ Anima situated Agency; S5′ Aletheia disclosure/metabolism/Return. C′ supplies reflective composition **to** this inhabiting world; C′ and S′ are not aliases.

Canonical Agents, M/M′ domains/instruments and native product owners retain their own identities. A Context Frame does not select an Agent by a universal index. The historical `shared/vak_address.ts` (blob `82701b20007d26fb2a722c1ccb541d5c351ad3e9`) and Anima contract (blob `893021ab7b36623de63b7fb4f8f4e0e11fb34d88`) remain read-only source/implementation evidence. Their old plugin, task-record and dispatch arrangements are not a current reconstruction assignment. The present requirement is attributable participating whole, frame, source and Return preservation.

## 8. Local → recursive composition → generation → Return

`VakComposition::bind_whole` accepts an actual `StructuralConstellation`, its existing `ShapeBinding`, a native subject/whole-ground, active frame, source revision/evidence and optional `FullVakBinding`. Public mutable input carriers are revalidated at admission. Missing members are not invented.

`compose` takes two already-addressed whole uses. When both are local constellations it constructs the accepted `RelationFieldComposition` and retains the complete `RelationFieldDerivation`. A composed field may itself be either source of another composition, and may remain a recursively composed field after Return. No conversion to a fabricated constellation and no numeric-dimension constructor are required.

The recursive carrier retains row/column **use refs**, actual source bodies, bindings, source Return routes and derivations. Its addresses are recursive pairs of actual member/whole addresses. `((a,b),c)` and `(a,(b,c))` retain distinct grouped identities. The additive recursive address ref uses length-framed child shape refs under `ql:carrier:1.1.0:relation-field:…`; unchanged immediate constellation×constellation fields keep the accepted v1 ref. This extends operational carrier addressing, not the canonical shape-grain ontology or the base C positional laws. Positive local carrier admission remains unchanged: empty and `Other` axes are refused even inside recursion.

`read` computes the active harmonic cut. For a parent field, each child pitch is determined relative to the **parent's current frame pitch**. Changing that frame therefore changes intervals and derived circle phases without changing the participating child or its frame. An explicit CP member focus remains the same disclosed member; its interval from the new frame is recomputed. MEF viewing rotation preserves that same absolute determination while exposing its local position through another existing lens. Name/Power, direct/conjugate and local/absolute positions remain separately inspectable.

Geometry here is the exact address/form plus harmonic circle-phase disclosure of this determination. The operation does not claim that C++ toroidal/material/acoustic embodiment has been completed. Existing shapes and canonical relations remain the formal authority; continuous embodiments are consumers under the separate kernel rebuild.

`determine` computes a new addressed `Determination` with its framed reading, all source refs, source revisions, contextual thread and producing basis. An optional `AgentContribution` must carry actor, result, input and evidence refs and an attributable full-profile expression. QL does not infer semantic text or an Agent's understanding from a coordinate.

`return_result` and CS produce `Returned` through the source whole-anchor to an explicit own, parent, child, other or conjugate ground at #0. Parent/child relations are checked against actual recursive participation; a reframed/positioned use preserves its original participating whole relation, but a newly offered whole is not silently treated as its source. Other/conjugate relations require attribution, and conjugate grounds retain the opposite face. The complete producing determination, source/target bindings, target basis and existing Return routes survive.

`offer_as_whole` makes the returned determination independently addressable with the same actual form, including recursively composed fields. It requires `DERIVED` provenance from that Return. Its declared existing ground remains explicit; generated content is not promoted to human-authored source or recognised canon. The native owner decides adoption.

## 9. Full language, native paths and public calls

The 109-node Anuttara source field remains unchanged. `FullVakBinding` carries the same AIKit syntax version, explicitly accepted owner revision, ResolvePath identity and native expression-node ref, together with `VakExpressionReadingV1`, a source `SelfOtherForm`, `VakContextField`, interpreter and optional expected ground. M0-3 self↔other, M0-4 contextual relation and M0-5 operative support remain separate source relations of the same determination. All source refs resolve through the existing `VakRegistry`; the full profile is not reduced to the generic six operators/horizons.

A native AST declaration is not an execution receipt. `record_native_observation` accepts the existing `VakExecutionObservationV1`, validates it through the existing observed-path machinery under the explicitly accepted syntax revision, and correlates the producing actor/result, native step, Action/Method, World/Project/Focus and source-bearing expression. Its correlation is `DERIVED`; supplied native execution evidence remains `OBSERVED`. The original #83 validation entrypoints retain their immutable acceptance revision. No general parser, Agent launch loop or native execution authority is duplicated.

Public Rust entrypoints are `VakComposition::{bind_whole,compose,reframe,position,position_member,read,determine,return_result,offer_as_whole,record_native_observation}` and `CPrimeContext::{enter,cpf,ct,cp,cp_at,cf,cfp,determine,cs}`. The small `whole`, `determination` and `returned` lookups expose immutable results.

The native CLI and JSON library adapter expose:

```sh
ql vak compose fixtures/kernel/vak-composition-v1.json --json
```

The envelope is `{"contract":"ql.vak-composition/v1","steps":[…]}`. Operations are `whole`, `compose`, `reframe`, `read`, `position`, `determine`, `return`, `offer`, `enter`, `cpf`, `ct`, `cp`, `cf`, `cfp`, and `inspect-context`. `determine`/`return` can explicitly name a C′ context. Each operation consumes native refs plus explicit frames/basis, not a QL-local expression string. JSON is a transport over these operations, not a replacement AIKit AST.

`ql_cli::vak_composition::execute_request` evaluates the same envelope in memory. Detailed typed native-observation correlation is available in the QL library rather than a second JSON copy of every native Action record. The specimen and `crates/ql-cli/tests/vak_composition.rs` show concrete public inputs and inspectable outputs. Replaying the same request preserves returned formal/source identity. Independent native deployments own durable storage and authority.

Input bounds are 64 recursive levels, 4,096 graph objects/expanded reading nodes, 64 contextual transitions, and a 16 MiB CLI request. Invalid refs, undeclared members, unsupported grains, inadmissible contextual combinations, forged parent relations and cyclic/future references fail without a successful result receipt.

## 10. Relation to the coordinate-complete rebuild and native owners

At this implementation inspection #136 remains open at `88cdcbafd9924b09b7f130dae8f31ef756f0f45d`; `docs/KERNEL-REBUILD-WAYFINDER.md` is still branch-qualified, not main. #125 owns native C restoration; #126 the shared recursive M registry; #127 the M ledger/parity protocol; #134 deep M0/M4/M5 closure. Their current issues are open.

This feature does not introduce another M tree or promote a new base structural position/family/grain. It consumes the accepted source registry, QL relation IDs, structural carrier and frame laws and adds Rust operational use/composition state. Required future changes to those canonical structural laws must return to C parity through the existing rebuild. Native C execution of these new higher-order operations and continuous C++ embodiment are not claimed by this Rust/CLI implementation.

AIKit owns AST/parser/operative resolution; Factory owns Commission/Journey/Run/WorkflowUnit/ExecutionDisposition and developmental scheduling; Wiki consumes QL; Central owns authored source; Actuation owns native agency; Workcell owns material execution. AIKit #267, Factory #217 and O:I #216 are downstream application/rollout concerns, not serial completion gates or work undertaken here. No new issues or Wayfinder are required.

## 11. Verification and epistemic boundary

The existing QL Rust lane runs format, workspace/all-target check, Clippy with denied warnings and all workspace tests. The new native QL tests exercise local/recursive forms, all six reflective operations, source R-path correlation, frame-dependent consequences, CP versus CF, MEF rotation, retained Return/source identity, callable CLI replay and lawful refusal. Existing structural, seven-CF and 109-source tests continue unchanged.

The CLI test actually launches the `ql` binary against the specimen; it does not merely validate JSON. Native-observation admission tests use explicitly synthetic records and prove validator behavior, not external Factory/Agent execution. The PR/merge checks supply actual run evidence. No local structural or software result proves metaphysical claims, downstream installation, model intelligence, material embodiment or human recognition.
''';Path(p).write_text(s)
# Compact canonical reference: keep historical representations as history, not
# unresolved current compatibility work.
p='docs/HOLOGRAPHIC-KERNEL-FORMAL-REFERENCE.md'
edit(p,'The reconciliation records their explicit relationships and the unresolved historical CT/CS/CFP aliases.','The reconciliation records their current operative relations: CPF discrimination, CT source-field selection, CP actual position/path, CF framing, CFP source R-path composition and CS explicit Return. Historical CT/CS/CFP spellings are not mandatory unresolved aliases.')
paragraph(p,"The historical Ta-Onta mirror's additional",'''The historical Ta-Onta `(4/5/0)` occurrence is superseded/noncanonical and outside this seven-frame set. No eighth frame, alias or compatibility blocker exists. `ql_mef::vak_composition` implements explicit scoped harmonic → geometric → MEF → generative propagation over these unchanged identities; see the canonical reconciliation for public operations and verification boundaries.''')
p='docs/QL-STRUCTURAL-CARRIER-CONTRACT-V1.md'
paragraph(p,'This contract intentionally stops before', '''The v1 structural carrier remains the unchanged leaf-composition and source/Return contract. QL #138 now operationalises it through `ql_mef::vak_composition`: immutable framed uses, recursively paired whole fields, explicit transitions and generated determinations with checked Return. Local-local composition still uses the exact v1 `RelationFieldComposition`/`RelationFieldDerivation`. Higher-order address grouping uses length-framed child refs under the additive `ql:carrier:1.1.0:relation-field:…` namespace so different bracketings cannot collide. No new `QlShape` variant, positional grain or dimension-based constructor is introduced. Each actual source whole and all existing Return routes remain available. The current behavior and public entrypoints are in [the canonical reconciliation](QL-VAK-KERNEL-RECONCILIATION.md), not a second structural grammar.''')
p='README.md'
edit(p,'complete scoped runtime propagation remains an explicitly tracked implementation frontier.','scoped runtime propagation is executable through `ql_mef::vak_composition`, including recursive fields, explicit CP/CF operations, generative determinations and checked Return.')
edit(p,'The follow-on work is QL-MEF #138, AIKit #267, Factory #217 and late installed-suite O:I #216, not a new grammar in each consumer.','QL-MEF #138 supplies the executable producer contract. AIKit #267, Factory #217 and O:I #216 are separate downstream applications, not this feature\'s execution sequence or completion gates.')
edit(p,'The deeper C′ work must reuse that accepted seam rather than anticipating or replacing it with another ontology.','The implemented C′ operations reuse that accepted seam; composed fields remain addressable wholes when nested and after Return, without a replacement shape ontology.\n\nRun a native local → recursive → reframed → generated → Return → new-whole path with:\n\n```sh\nql vak compose fixtures/kernel/vak-composition-v1.json --json\n```\n\nThe returned JSON exposes actual participant refs, active frames, member/address selection, harmonic intervals, geometric phases, MEF positions, producing basis and explicit Return grounds. Agent-provided interpretation remains attributable and generated results remain `DERIVED`.')
edit(p,'the deeper C′ scope and full-suite integration gates are named in the reconciliation above.','C′ composition is callable through the QL library/CLI, while native C restoration, deeper M coverage, C++ embodiment and downstream suite integration retain their distinct ownership in the reconciliation above.')
p='docs/integrations/epi-logos/EPI-VAK-OPERATIVE-SYNTAX-ARCHITECTURE.md'
s=Path(p).read_text();s+='''\n\n## QL-owned contextual implementation — #138\n\nThe [QL Kernel reconciliation](../../QL-VAK-KERNEL-RECONCILIATION.md) now gives the executable C′/Context-Frame producer contract. `FullVakBinding` interprets the existing AIKit native expression node and ResolvePath under an explicitly accepted syntax revision; it does not copy the general AST/parser. The same source-bearing determination is framed, harmonically/geometrically/MEF-readable, composable as a whole within a whole, and capable of explicit generative Return.\n\n`VakComposition::record_native_observation` correlates the existing native observed-path contract with that determination and retains it through Return. Admission of supplied execution evidence is not execution by QL. Synthetic validator tests are not native Agent acceptance. The immutable #83 receipt remains historical evidence for the implementation actually tested there. The current seven CFs are unchanged; the historical `(4/5/0)` occurrence is superseded/noncanonical, with no missing eighth frame.\n''';Path(p).write_text(s)
