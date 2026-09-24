# Anima and Aletheia workflows

Three Factory workflow sources, authored in QL with the Vāk types from
`adapters/factory-workflow` (`@epilogos/ql-vak`). They are ordinary
`*.workflow.ts` files: Factory parses them as restricted data, never executes
them, lowers each unit's `composition` (C′) to its native execution binding and
compiles them into its usual WorkflowUnits. Nothing here is commissioned.

| Source | What it carries | Cast |
|---|---|---|
| `expression-development.workflow.ts` | Nous opens the ground (CF1) → Logos articulates (CF2) → Eros exchanges with the available Expression powers (CF3) ∥ Mythos finds the organising image (CF4) → Anima conducts (CF5) → Psyche keeps continuity (CF6) → Sophia integrates (CF7) → Aletheia discloses and returns | `agent-set/anima` members, `agent/aletheia` |
| `techne-constellation.workflow.ts` | Anansi blueprints a root-wiki constellation → Moirai, Mercurius and Janus read it as a fusion → Agora examines the aggregate → Zeithoven proposes the next form → Aletheia returns the revision through NOW | `agent-set/aletheia` members |
| `development-and-knowledge.workflow.ts` | Logos scopes and plans → Technē helper implements → Eros verifies → Mythos debugs → Sophia finishes the branch → Aletheia returns the learning through the NOW/wiki door | `agent-set/anima` members, `agent/aletheia` |

Thread forms are real topology, checked by Factory: melody (CFP2) voices each
consume the previous voice's return through `inputs`; the Eros ∥ Mythos chord
(CFP1) is declared independent and resolves at a barrier; the Technē fusion
(CFP3) waits at `readings-fuse`; Aletheia's return is a single voice (CFP0).

## Check

```sh
factory workflow check workflows/expression-development.workflow.ts --json
factory workflow check workflows/techne-constellation.workflow.ts --json
factory workflow check workflows/development-and-knowledge.workflow.ts --json
```

`check` reports each unit's lowered composition (frame, thread form, actor,
QL binding). Factory's own test suite checks copies of these three files
(`factory/tests/fixtures/ql-vak-workflows/`); keep them identical when either
side changes.

## Standing of the references

- Participants (`agent-set/anima`, `agent-set/aletheia`, `agent/anima-*`,
  `agent/aletheia-*`) are Central agent-set and agent refs. Commission resolves
  them from Central; the sources do not mint them.
- `praxisRefs` name the QL Skills (`skill/ql/...`) and the personal practice
  Skills (`skill/personal/...`). They are requirements, not loaded context.
- `capabilityRefs` are requirements for Execution Intelligence, not grants.
- `resolvePath` / `contextResolution` name the AIKit resolution each voice
  needs; AIKit resolves the actual identity at dispatch.
- Subjects and basis revisions are the ones current when these were written
  (`project:O-I` at `f7082e5d`, `central:wiki:root` at wiki revision 50,
  `project:quaternal-logic` at `c624c52`). Commission an explicit successor
  (`source.successorOf`) for a specific Expression, constellation or change.

## Commission (after both PRs merge and the Central agent sets exist)

Read the agent-set records first; do not guess their paths or revisions:

```sh
ctrl --json action run central.agent-set.resolve '{"scope":"root","ref":"anima"}'
ctrl --json action run central.agent-set.resolve '{"scope":"root","ref":"aletheia"}'
ctrl --json action run central.agent-set.list '{"scope":"root"}'   # source_path + revision
```

Write a `factory.commission-request/v1` whose `participantRequirements` name
`agent-set/anima` and `agent-set/aletheia` (plus the root act's agent, e.g.
`agent/anima`) with `sourceOwner: "central"`, `sourceRef:
"central:source:control:root:<source_path>"` and `sourceRevision: <revision>`,
then:

```sh
factory project locate <project-root> --json          # the owner's state path
factory workflow commission <state.json> <request.json> workflows/expression-development.workflow.ts --json
factory workflow inspect <state.json> <run-ref> --json
```

Commission stamps the source and its lowered C′ before any attempt. Normal
`factory attempt` Actions own start, dispatch, verification and Return.
