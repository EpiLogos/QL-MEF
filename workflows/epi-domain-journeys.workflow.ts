import { defineWorkflow } from "@epilogos/factory-workflow";

// QL-MEF #258 — six domain agents, Nara's Day/Flow journeys and native
// Bimba-to-Expression practice. Packets EA0–EA5 of
// docs/integrations/epi-logos/EPI-LOGOS-AGENT-DEVELOPMENT-PROGRAMME.md §4,
// with an independent verifier for V01–V12 (§5.1). The first unit set is the
// programme's first connected delivery (§4.2); the full repertoire and
// coverage continue through the same packets.
//
// Participants are Central agents that exist at authoring time: the product
// Guardians and the Anima/Aletheia sets. The six M-domain identities do not
// exist yet; creating them through the native proposal path is EA0's work.
export default defineWorkflow({
  "source": {
    "ref": "workflow-source:01M3M7KNPFYGD9KTDPBQDHD61N",
    "revision": "epi-domain-journeys-v1",
    "temporalRef": "flow-time:2026-09-28",
    "flowRef": "flow:ql-mef-258-epi-domain-journeys-2026-09-28"
  },
  "workflowKey": "epi-domain-journeys",
  "units": [
    {
      "key": "ea0-team-and-bimba-ground",
      "developmentalConcern": "Seat the six M-domain agents (Anuttara, Paramasiva, Parashakti, Mahamaya, Nara, Epii) as enduring Central identities in one Epi-Logos team with their product-Guardian conjugacy, and make the Bimba service a native Knowledge/ContextSource with Neo4j as provider and MCP optional",
      "requiredDifference": "agent-profile/agent-set records for the six domain agents and the team exist through the native proposal path, each binding its QL faculty operations and source; the bimba-portable application service is reachable natively for exact coordinate, prime/compound, depth, prefixed-property and typed-relation reads, proven with MCP stopped and with wrong-branch controls failing",
      "returnContract": "Return the created identity/team refs, the native Bimba operation surface, and the MCP-stopped proof with its exact commands",
      "subjectRef": "project:quaternal-logic",
      "basisRevision": "8b783af",
      "agentRequirements": {
        "agentRefs": ["agent/ql-guardian", "agent/central-guardian"],
        "agentSetRefs": [],
        "agencyRefs": []
      },
      "praxisRefs": ["skill:factory-bounded-work", "skill:factory-development"],
      "capabilityRefs": ["capability:cap.suite.world-relative-agent-knowledge"],
      "dependencies": [],
      "independenceFrom": ["ea2-journey-modality", "ea4-holography-and-praxis"],
      "permittedEffects": [
        "propose Central agent profiles and agent sets through agent-profile.propose / agent-set.propose",
        "write QL/AIKit Bimba ContextSource source and tests in a registered seat"
      ],
      "verificationObligations": [
        "resolve the team with central.agent-set.resolve",
        "prove Bimba reads with the MCP process stopped",
        "reproduce branch-vs-range, exact-vs-prefix and declared-but-unused depth at the provider cut"
      ],
      "returnAddress": "return:ql-mef-258-ea0",
      "stopConditions": "Stop before adopting an identity as owner-recognised; proposals stay proposals",
      "escalationConditions": "Escalate any identity conflict with the Anima/Aletheia members through the parent"
    },
    {
      "key": "ea1-nara-day-flow",
      "developmentalConcern": "Nara knows the person through their continuing Day/Flow: the relevant history reaches the actual body before its turn, with exact source identity, attribution and corrections",
      "requiredDifference": "The O:I Nara dialogue context fills its occasion and disclosed refs from nara.lived-context.compose over central.document.read results; AIKit prepared context carries the selected passages with their revisions; a fresh body answers a question whose decisive fact exists only in an earlier Day passage, and removing that passage changes the supported reading",
      "returnContract": "Return the delivered-context digests (context_revision), the fresh-body transcript reference and the negative control",
      "subjectRef": "project:O-I",
      "basisRevision": "8b783af",
      "agentRequirements": {
        "agentRefs": ["agent/oi-field-guardian", "agent/aikit-guardian"],
        "agentSetRefs": [],
        "agencyRefs": []
      },
      "praxisRefs": ["skill:factory-bounded-work", "skill:factory-development"],
      "capabilityRefs": ["capability:cap.suite.world-relative-agent-knowledge"],
      "dependencies": [],
      "independenceFrom": ["ea0-team-and-bimba-ground"],
      "permittedEffects": [
        "write the O:I Nara dialogue context path in coordination with the standing env-3 Nara lane owner",
        "write AIKit prepared-context carriage of lived passages"
      ],
      "verificationObligations": [
        "save and inspect the delivered basis, not the assembled request",
        "a fresh body succeeds without the parent supplying the decisive fact"
      ],
      "returnAddress": "return:ql-mef-258-ea1",
      "stopConditions": "Stop before writing into the person's Day source; returns go through central.receiving",
      "escalationConditions": "Escalate shared bridge/context file conflicts to the single integration owner"
    },
    {
      "key": "ea2-journey-modality",
      "developmentalConcern": "One Tarot deck and one I-Ching change history articulate a continuing journey across Days and bodies",
      "requiredDifference": "The native journey (nara.journey.open/apply/read, QL-MEF PR #261) is persisted by its Central owner with a revision check, reopened from normal entry on a later Day, and exercised through distinct draw, symbolic assignment, correction, aliveness and I-Ching acts without reshuffle or recast",
      "returnContract": "Return the persisted journey refs and revisions across two Days and the replay evidence",
      "subjectRef": "project:quaternal-logic",
      "basisRevision": "8b783af",
      "agentRequirements": {
        "agentRefs": ["agent/ql-guardian"],
        "agentSetRefs": [],
        "agencyRefs": []
      },
      "praxisRefs": ["skill:factory-bounded-work", "skill:factory-development"],
      "capabilityRefs": ["capability:cap.suite.world-relative-agent-knowledge"],
      "dependencies": [],
      "independenceFrom": ["ea0-team-and-bimba-ground", "ea4-holography-and-praxis"],
      "permittedEffects": [
        "write QL Nara journey source and tests",
        "persist controlled journeys through Central files with a revision check"
      ],
      "verificationObligations": [
        "cargo test -p ql-mef --test nara_oracle_journey",
        "cross-process replay does not deal or cast twice"
      ],
      "returnAddress": "return:ql-mef-258-ea2",
      "stopConditions": "Stop before importing the owner's private journey; that is the owner's act",
      "escalationConditions": "Escalate any unresolved dealing rule as an exact owner decision"
    },
    {
      "key": "ea3-computation-into-expression",
      "developmentalConcern": "Paramasiva, Parashakti and Mahamaya contribute materially different source-defined work over the same occasion, and Nara presents and operates the journey as a saved native Expression",
      "requiredDifference": "A saved oi.expression/v1 journey Expression carries card, hexagram and source-text entities bound to the journey and its sources, reopens on a later Day with the same subject and scene, and changes when a correction changes the reading; Mahamaya's contribution consumes the admitted identity/composed-state basis from the active #135/#201 identity owners",
      "returnContract": "Return the Expression refs and revisions, the source/scene round trip and the determinant-variation evidence",
      "subjectRef": "project:O-I",
      "basisRevision": "8b783af",
      "agentRequirements": {
        "agentRefs": ["agent/oi-field-guardian", "agent/factory-guardian"],
        "agentSetRefs": [],
        "agencyRefs": []
      },
      "praxisRefs": ["skill:factory-bounded-work", "skill:factory-development"],
      "capabilityRefs": ["capability:cap.suite.world-relative-agent-knowledge"],
      "dependencies": ["ea1-nara-day-flow", "ea2-journey-modality"],
      "independenceFrom": [],
      "permittedEffects": [
        "write O:I Expression/journey scene source in coordination with the standing Nara lane",
        "save controlled Expressions through central.files"
      ],
      "verificationObligations": [
        "hold unrelated inputs constant and vary one determinant",
        "no duplicate hidden live engine when several representations are open"
      ],
      "returnAddress": "return:ql-mef-258-ea3",
      "stopConditions": "Stop before inventing a numerical pipeline; consume the actual producer",
      "escalationConditions": "Escalate renderer/bridge ownership conflicts to the integration owner"
    },
    {
      "key": "ea4-holography-and-praxis",
      "developmentalConcern": "Epii knows, teaches and develops the whole through M5's reflective homes, and Nara's M4 branches have usable repertoires",
      "requiredDifference": "M5-2 to S, M5-3 to M-prime and M5-4 to S-prime are recorded as source-qualified bindings in the registered matrices and graph projection (M5-0 kept at proposal standing); a Logos Methodology and M4 branch SkillSets exist through the native practice lifecycle; Epii round-trips one capability through all views to the same native identity",
      "returnContract": "Return the binding records, matrix checks and the round-trip demonstration",
      "subjectRef": "project:quaternal-logic",
      "basisRevision": "8b783af",
      "agentRequirements": {
        "agentRefs": ["agent/ql-guardian", "agent/aikit-guardian"],
        "agentSetRefs": [],
        "agencyRefs": []
      },
      "praxisRefs": ["skill:factory-bounded-work", "skill:factory-development"],
      "capabilityRefs": ["capability:cap.suite.world-relative-agent-knowledge"],
      "dependencies": [],
      "independenceFrom": ["ea1-nara-day-flow", "ea2-journey-modality"],
      "permittedEffects": ["write QL matrices, registry bindings and practice source"],
      "verificationObligations": [
        "run the registered matrix checks",
        "python3 scripts/check-ux-spine.py --self-test"
      ],
      "returnAddress": "return:ql-mef-258-ea4",
      "stopConditions": "Stop before promoting M5-0 beyond proposal standing",
      "escalationConditions": "Escalate capability-ID conflicts through the matrix protocol owner"
    },
    {
      "key": "ea5-joined-agency",
      "developmentalConcern": "Anima composes and Aletheia receives, interrogates and returns an improved journey Expression or practice which a later encounter uses",
      "requiredDifference": "Actual Anima and Aletheia member contributions on a real journey, a preserved disagreement, and a returned artifact that a fresh participant uses on a related but different occasion",
      "returnContract": "Return member contribution refs, the revised artifact and the later-use evidence",
      "subjectRef": "project:O-I",
      "basisRevision": "8b783af",
      "agentRequirements": {
        "agentRefs": [],
        "agentSetRefs": ["agent-set/anima", "agent-set/aletheia"],
        "agencyRefs": []
      },
      "praxisRefs": ["skill:factory-bounded-work"],
      "capabilityRefs": ["capability:cap.suite.world-relative-agent-knowledge"],
      "dependencies": ["ea3-computation-into-expression"],
      "independenceFrom": [],
      "permittedEffects": ["write returns through the NOW/wiki door and the Expression owner"],
      "verificationObligations": ["a crosswalk entry or stored thought is not functional equivalence"],
      "returnAddress": "return:ql-mef-258-ea5",
      "stopConditions": "Stop before a single body role-plays the team",
      "escalationConditions": "Escalate missing member faculties to the relevant Guardian"
    },
    {
      "key": "verify-first-episode",
      "developmentalConcern": "Independently replay the first connected episode (programme §4.2) against V01–V12 on installed software with real providers",
      "requiredDifference": "Each applicable case stands established by executed evidence or is recorded unavailable with its exact reason; a disconnected source and a plausible wrong-branch result are detected",
      "returnContract": "Return the per-case verifier report with commands and receipts",
      "subjectRef": "project:quaternal-logic",
      "basisRevision": "8b783af",
      "agentRequirements": {
        "agentRefs": ["agent/central-guardian"],
        "agentSetRefs": [],
        "agencyRefs": []
      },
      "praxisRefs": ["skill:factory-bounded-work", "skill:factory-operation"],
      "capabilityRefs": ["capability:cap.suite.world-relative-agent-knowledge"],
      "dependencies": ["ea1-nara-day-flow", "ea2-journey-modality", "ea3-computation-into-expression"],
      "independenceFrom": [],
      "permittedEffects": ["read any installed product surface", "write verifier evidence and returns"],
      "verificationObligations": ["runtime, formal computation, interpretation quality and human experience carry separate evidence"],
      "returnAddress": "return:ql-mef-258-verify",
      "stopConditions": "Stop before writing implementation source; the verifier does not repair",
      "escalationConditions": "Return failures to the owning packet for repair and replay"
    }
  ]
});
