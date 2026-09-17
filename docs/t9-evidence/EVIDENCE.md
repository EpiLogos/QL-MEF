# T9 integrated-field walk — evidence record

**Standing:** executed evidence of QL-MEF #220, 2026-09-17, session
`aikit:agent-session:ctx_01M2PCRX0QEG1YSV1WBNWTPDHT` (actor `zcode-t9`).
**Companion ledger:** `docs/L5-TECHNE-T9-INTEGRATION-LEDGER.md`.
**Proof-class discipline:** each item below names its class; no class substitutes for another.

---

## Starting identity (real subject/world)

```text
Project / World          project:quaternal-logic (Work/Quaternal-Logic) at control:root
subject_ref              github:EpiLogos/QL-MEF/issues/220
bounded-whole ref        git:EpiLogos/QL-MEF:techne/tb0-surfaces-port@07cfe4a
source refs + standing   docs/L5-TECHNE-DUAL-READING-LOCK.md @07cfe4a (owner-ratified-conceptual-architecture)
                         docs/L5-TECHNE-TB0-CONNECTIVE-BASE.md @07cfe4a (implementation-record)
                         docs/integrations/epi-logos/TA-ONTA-VAK-COMPOSITIONAL-EXECUTION-LOCK.md @07cfe4a
                         github issues #212–#219 (agent-recorded lane receipts, exact instants in the reading)
                         git:EpiLogos/O-I:docs/experience/TECHNE-DUAL-READING.md @35fa86d (campaign-amendment, PR #357)
                         central:source:control:root:Work/Quaternal-Logic/ProjectCentral/now/agents/l5-techne-surfaces-ported-to-ql-mef-home-2026-09-16.json
                         central:source:control:root:Control/agents/expressions/ql-guardian (authored Guardian expression ground)
                         central:source:control:root:Control/user/civil-time-policy.json (architecture-contract)
TechneReading revision   ql.techne:reading:central-ground-t9@1, snapshot 07cfe4a8ca55f78a72191b5abcc5ec695b16866a
QL / M′ refs             ql:structural:5.0.0, lens mef:lens:L5@1, CF mef:context-frame:CF4, vak source daa660cbc1b8c5da83828698665a753852cb0287
DAY / NOW / Session      central:day:control:root:2026-09-15 (latest closed DAY); 3 root NOW clearings from the live capture;
                         session aikit:agent-session:ctx_01M2PCRX0QEG1YSV1WBNWTPDHT
Expression / Scene refs  ql-guardian expression binding; three scenes composed in the substrate grammar
                         (…:scene:the-connective-base-pins | seven-lanes-fan-out | convergence-and-return)
native Actions           central.now.list, central.day.read, central.time.policy (registered-read-action);
                         ql.vak.compose (registered-read-action, compute-only);
                         git.commit (repo-ground-write); projectcentral.now.return (governed-write);
                         oi.expression.open (owner-disclosed crossing)
provider revisions       ctrl 0.1.0 c7018cc · aikit 0.1.0 · ql 0.1.0 (kernel 0.1.0-q1) · actuation 0.2.0
desktop / build          O:I desktop installed (org.epilogos.oi.cradle), not hosting the refined surfaces yet
                         (techne/convergence @ ec3100c1 = pre-lane generation) — the port-back seam
capture fixture          adapters/techne/surfaces/cradle/fixtures/central-ground-t9-capture-v1.json
                         (sha256 2755b7eaf4a57d1e…, 12 NOW records, provenance-stamped)
```

## Path evidence (class: native provider integration + repository/contract tests)

| Leg | What was proven | Evidence |
|---|---|---|
| M0′ | one bounded whole over the real ground; LIST/GRAPH project the same refs; provenance inspectable (revision + selector + standing); semantic zoom narrows without re-query | `techne-t9-integrated-walk.test.mjs` "M0′" |
| session | ground session opens with derived 4:2 cut, whole/occasion/Return-target ground, native agent-session ref | "session" test |
| M1′ | authored View + frames leave the reading byte-identical; governed proposal routes `projectcentral.now.return` (routed receipt, owner central/ctrl); undisclosed action refused with reason | "M1′" test |
| M2′ | `implemented-in` dated through its own occurrence facet; Guardian `stewards` trans-temporal with no manufactured date; standing verbatim; occurrence ≠ receipt per lane; sourced relation warrants the 3:3 crossing | "M2′" test |
| M3′ | three scenes composed from real refs (CT1/CT2/CT4 burdens met; CT5 honestly not claimed); composition copies no semantic whole; CS0 vs CS5 change the real traversal (V1); crossing carries the exact scene ref; return lands on the exact beat | "M3′" test |
| M4′ | carried occasion = real DAY/NOW/policy refs; one factual `OPERATED_IN` place facet with uncertainty, no geometry; solar/Nara ladder rungs carry availability + reason; Nara crossing keeps the occasion byte-exact | "M4′" test |
| M5′ | palace regions compose the real ref families (ground/relation present); composition derives and binds; Return route = the project register's governed-write action; re-open resolves to the project instrument | "M5′" test |
| continuity | project → timeline → expressions → palace → project holds ONE session; subject/basis/selection/AgentSession/whole/occasion/Return-target byte-exact at every hop; same-cut crossing refused | "continuity" test |
| 5→0 | the accepted Return is visible in the live project register via `projectcentral.now.inspect` — renewed M0′ ground | "5→0" test (passes only because the Return below really ran) |

## Executed native acts (class: native provider integration)

1. **Live kernel Vāk composition.** `ql vak compose` ran twice (exit 0):
   `docs/t9-evidence/vak-compose-baseline.result.json` (verbatim specimen routing) and
   `docs/t9-evidence/vak-compose-t9-provenance.request.json` → `.result.json` (origin-step
   provenance re-pointed at the real session and converged ground; 13 ops). Two malformed
   variants were REFUSED BY THE KERNEL with its own law — kept as negative-case evidence:
   "return must route through the constellation whole-anchor" and "returned content remains
   DERIVED from its Return".
2. **Accepted Return.** `ctrl --json action run projectcentral.now.return` executed with
   receipt: id `t9-integrated-field-walked-one-real-2026-09-17`,
   `recorded_at_unix_seconds 1789608137`, provenance `agent-authored-bounded-return`,
   actor `zcode-t9`, project register Quaternal-Logic. This is the real attributable
   difference of the walk; the renewed-ground test observes it live.
3. **Repository revision.** This branch (provider + capture + walk test + ledger + this
   record) is committed on `techne/t9-integrated-field` — the M1′ accepted semantic
   relation executed as a real repo revision; the commit SHA is the receipt.

## Agency (class: Agent-native operation, contract level)

The reading carries the situated role floor at coordinate 5 with real refs:
`guardian` = the ql-guardian expression ground (stewardship, spans both readings);
`anima` (3:3, expressions), `aletheia` (4:2, palace), `technē` (4:2, palace) — all naming the
native AIKit session `aikit:agent-session:ctx_01M2PCRX0QEG1YSV1WBNWTPDHT` and validated by
the TB0 role law. The continuity test proves human-path Actions and one companion session
co-reference. **Live `actuation agency actualise` is NOT claimed:** the machinery is
installed and its admission law was exercised (strict WorldBinding / metagency-grant /
determination-ancestry / identity-evidence demands); constructing a full valid record is the
named Actuation-side leg.

## Performance / resource lifecycle (class: model-level runtime evidence)

The X3 surface lifecycle suite (in the 349-test cradle run) proves: hidden → suspended
(pending frame cancelled) → re-focus resumed → dispose released exactly once; embodiment
alias registers/releases as ONE host across crossings; no renderer/animation loop/WebGL
context/persistence in the expressions lane (static law test); presentation stores release
listeners on unsubscribe. **Installed-desktop profiling is NOT claimed** — the refined
surfaces are not hosted by the O:I desktop yet (port-back seam, ledger §4.1); TD7's
hidden-globe/WebGL/media-subscription rounds remain for the installed host.

## Human acceptance (class: H/EX — the owner's)

NOT ANSWERED HERE. The eight dual-reading questions (lock §24 TD8 / issue §13) are put to
the owner with the walk's artifacts; H/EX is never self-certified.

## Known limitations (truthful)

See ledger §2 (additive candidates open through #212) and §4 (seven named seams with owners).
