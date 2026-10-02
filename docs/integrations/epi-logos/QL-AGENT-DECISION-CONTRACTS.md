# QL agent event and decision contracts — QD0

Execution is governed by [QL-native decision grammar, harmonics and shared
Prime/Pi bodies](https://github.com/EpiLogos/QL-MEF/issues/291), extending the
existing [domain-agent programme](https://github.com/EpiLogos/QL-MEF/issues/258)
and [Ta-Onta SDK](https://github.com/EpiLogos/QL-MEF/issues/201). This account
records the wire/provenance boundary, native projection/admission interfaces and
evaluation tooling. Provider execution, model benchmarks, trained models and body
integration retain their later acceptance gates. The coordination issue tracks
executed evidence and the whole QD0–QD5 programme.

The authoritative wire grammar is `scripts/ql_agent_contracts.py`, with its
checked export `schemas/ql-agent-contracts-v1.schema.json`. Four schemas share
source, fact, label, provider and constraint types. Strict objects refuse unknown
fields and provider-specific ontology. The reference validator additionally
checks cross-document provenance and dispositions. This reference tooling does
not author any positional, musical or semantic QL table.

## Owners

| Owner | Responsibility |
| --- | --- |
| ql-core | Canonical coordinate, relation, conjugation, completion and shape laws |
| ql-mef | Event projection, decision frames, semantic admission, harmonic readout |
| ql-adapters | Translation to existing AIKit decision execution |
| ql-cli | Native inspection, projection, validation and operation receipts |
| AIKit | Model/provider execution, material, attribution and portable packaging |
| Actuation | Prime body, loop, authority, Activity and child inheritance |
| Pi extension | Typed native calls and event translation to the same QL owner |

QL source includes `ql-core/src/structural.rs`, `pairing.rs`, `shape.rs` and
`ql-mef/src/lens.rs`, `context_frame.rs`, `music.rs`, `music_completion.rs`.
QD1 consumes their actual public APIs. The Python reference validator establishes
neither eligibility of a formal operation nor whether a rule ref is authoritative.
Those are kernel admission, never a string-presence check.

## Event and semantic identity

`ql.agent-event/v1` carries a projected QL `event_ref`, generation, occasion
refs, exact source refs/revisions, kind, subject, material, observed formal facts,
observed native state and provenance. Optional `bindings` retain body, session and
native event refs. This is a bounded projection over owner-native events, not a
universal event database.

`event_ref` identifies the projected event. A harness event id goes in
`bindings.native_event_refs`; body and session identities are not the QL event
identity. Prime and Pi can bind the same projected event while retaining their
different native occurrences.

The event basis digest covers all event fields except `bindings`. Source
revision, event generation, kind, text or observed-state changes invalidate the
basis. Changing only native bindings does not. A frame digest covers the whole
frame, including legal labels, descriptions, kernel basis and constraints. Each
determination and harmonic readout retains the exact digests it consumes.

Bodies consume owner-issued digests rather than independently recomputing QL
meaning. The QD0 reference encoder uses sorted JSON object keys, compact
separators, unescaped Unicode and SHA-256 over UTF-8 bytes. NaN and infinity
are refused. QD1 must pin the native encoding, including numeric-value handling,
against these fixtures before producing interoperable native receipts.

## Determination standing

Observed, derived, learned, validated and unresolved remain distinct. Validation
is admission of an observed or learned fact; its original origin remains
recorded. It cannot relabel a derivation as an observation.

- Events permit only observed facts. Deterministic frame derivations require
  an explicit rule ref and basis refs.
- Frames retain all supplied observations and introduce no learned values.
  An unresolved head cannot repredict any determined field.
- Learned proposals retain original label selections, evidence spans and optional
  supplied confidence. Confidence is not textual evidence.
- Every learned proposal or refusal retains provider, model, model revision and
  runtime revision. Controlled adverse payloads in tests identify themselves as
  inputs, never as inference receipts.
- Validated learned values must equal their retained proposal. Illegal labels,
  invalid cardinality and invisible repairs cannot become admitted values.
- Every requested head has exactly one terminal disposition: admitted,
  unresolved or refused. Empty selection is representable abstention; it cannot
  become an admitted definite reading.

Evidence offsets count Unicode codepoints in `material.text`, not UTF-8 bytes or
UTF-16 code units. A span retains the material ref/revision and exact substring.
Missing or different source, out-of-range offsets and fabricated text are refused.

`status` describes the full result. `determined` has no unresolved/refused heads;
`partial` has admitted learned heads and remaining heads; `refused` retains an
actual refusal. `unavailable` carries no model output. `unresolved`, `refused`,
`stale` and `unavailable` cannot carry learned admission. Stale application is
refused by basis comparison; a `stale` audit disposition is not operative.
A zero-head frame is `determined` and carries neither learned results nor
provider attribution. QD1 must independently prove zero provider calls.

## Decision frame and constraints

`ql.agent-decision-frame/v1` separates observed/derived `determined` facts from
`unresolved` heads. Each head supplies owner-qualified labels and descriptions,
minimum/maximum cardinality and an ambiguity policy. Exact source and kernel
bases travel with the frame. Initial head families and future restrictions
remain those in the governing Wayfinder; the fixture's lens head is a bounded
wire specimen, not the complete semantic grammar.

The portable constraint grammar has four explicit forms:

| Kind | Fields and intended kernel evaluation |
| --- | --- |
| implication | When the antecedent selection conjunction holds, the consequent conjunction must hold |
| exclusion | The `forbidden_together` conjunction must not hold |
| legal-combination | A fully resolved selection must match one of the exact `allowed_tuples` |
| candidate-restriction | Every chosen candidate must be within `allowed.label_ids` |

For ordinary selection predicates, `any` means a nonempty intersection; `all`
means all listed labels are chosen; `exact` means set equality. Empty ordinary
predicates require `exact`. Allowed combination rows cover the same heads and use
exact matching. A candidate restriction uses the `all` wire marker to bound the
whole selected set; it is a restriction, not the ordinary `all` predicate.
Unresolved antecedents cannot manufacture certainty.

QD0 checks declared heads/labels, unique ids, cardinality, portable shape and
receipt attribution. It does not evaluate these predicates against QL law.
QD1 owns generation and evaluation, including unresolved applicability. A model
claim that it satisfied constraints is not a kernel receipt. Application receipts
must name a declared constraint; duplicate/conflicting ids are refused.

## Harmonic readout

`ql.harmonic-event/v1` has separate `formal` and `harmonic` facts. Formal facts
must already belong to the consumed determination. Harmonic facts require
derived origin and rule refs. Event, source, kernel and determination digest
must match exactly. QD0 validates that boundary, not pitch or ratio correctness.

QD1 computes the rendering with the existing `MusicalBasis`, lens anchors,
ratios, A/B/C squares, traversal classifier, cross-operator intervals,
Context-Frame/modal mappings and D1–D3 completion. No model labels or Python/
TypeScript music tables replace those owners.

## Fixtures and executed checks

`fixtures/agent-decision/v1/` retains a controlled textual concern, event, lens
decision frame, unavailable determination and actual native label observations.
The fixture ref is stable and repository-relative; it contains no machine-local
Central path. Labels and descriptions came from `ql mef lenses --json`; the
Context-Frame receipt came from `ql context-frame list --json`. The retained
receipts disclose historical observation, not current installed/source parity.

Wire/provenance checks run without ML or a QL binary:

```sh
python3 scripts/ql_agent_contracts.py --check-schema
python3 -m unittest discover -s scripts/tests -p 'test_ql_agent_contracts.py' -v
python3 -m unittest discover -s scripts/tests -p 'test_ql_agent_evaluation.py' -v
python3 scripts/ql_agent_contracts.py \
  --event fixtures/agent-decision/v1/event-v1.json \
  --frame fixtures/agent-decision/v1/decision-frame-v1.json \
  --determination fixtures/agent-decision/v1/unavailable-determination-v1.json
```

Native label observation/fixture generation is explicit and retains a binary
digest. It does not invoke a decision provider:

```sh
python3 scripts/ql_agent_fixtures.py --ql target/debug/ql \
  --output ProjectCentral/now/tmp/qd0-native-observation --check-labels
```

The existing `scripts/verify` gate includes wire checks in `invariants` and
fresh native owner comparison in `core` after the native CLI exists. Tests do
not overwrite published schema or fixtures. Removing a cross-document guard
must break its adverse test; removing the real QL executable must fail the
native observation, not silently substitute a fixture reply.

## Native projection and admission

`ql-mef::agent_event` owns deterministic event projection and response admission.
The CLI consumes that API:

```sh
ql agent-event project request.json --json
ql agent-event frame request.json --json
ql agent-event harmonic request.json --json
ql agent-event validate admission.json --json
```

A projection request contains `event` and optional `requested_heads` (`lens`,
`context-frame`, `faculty`, `operation`). An empty list requests deterministic
reading only. `decision_head_ids` identifies remaining eligible semantic heads;
missing formal inputs remain separate in `missing_inputs` and input-required
frame heads. Empty semantic material never produces eligible decision heads.

Typed observations use native current lens/shape refs, rotation coordinates,
Context Frame structure, relation traversal/pair state, completion degree,
constellation members and musical basis. Derivation calls the existing kernel
owners. Full A/B/C overlaps remain in `relation-candidates`; an explicit legal
pair selection restricts completion without deleting those overlaps. D2
source/target expansion retains traversal reversal. Ratios come from
`MusicalBasis`, and shape/compression from native structural members.

An admission request contains `projection` (the original projection request)
and `response` (`ql.agent-decision-response/v1`). The response retains event and
frame digests, kernel basis, outcome, provider/model/runtime identity, original
proposals and optional exact evidence spans. QL rebuilds the current frame;
providers cannot replace its labels, constraints or admission rules.

`ql.agent-decision-admission/v1` preserves the exact response beside the current
projection. Stale, malformed or unrequested responses remain inspectable without
becoming operative. Impossible selections and native combination failures get
exact refusals. Unavailable service, omitted heads and empty selections remain
unresolved. A deterministic event bypasses response admission and carries no
provider in its determination.

Original frame derivations remain an exact prefix of determination derivations.
Additional consequences must reference `frame_ref#proposal_head` of a validated
semantic selection. A newly derived field can discharge its original unresolved
head without relabelling the model's abstention. Multiple accepted readings
retain separate native harmonic candidate projections; they do not acquire a
single invented lens or Context Frame.

The compiled kernel basis fingerprints consumed QL owners plus projection and
admission source. No runtime Git HEAD is substituted for executing bytes. This
API starts no classifier, tool or body session. Prime/Pi integration and stock
model measurements remain separate required work. New source is qualified by
real native/CLI execution, not by the wire checks alone.

## Numerical evaluation

`scripts/ql_agent_evaluation.py` consumes a frozen JSONL suite of
`ql.agent-evaluation-case/v1` records. Each declares its id, structural/template
family, train/validation/test split, native projection request and independently
sourced expected fields, candidate sets and status. The runner refuses changed
suite bytes, family leakage, omitted native heads and non-native expected labels.

Pass `--suite`, its exact `--suite-digest sha256:...`, `--split test`, `--ql` and
a new `--output` directory. Without a provider command it evaluates the actual
deterministic/unresolved native result. An explicit `--provider-command` receives
only native event/frame material and must return the QL response contract through
the AIKit execution adapter. Its placement, deadline and resource admission are
AIKit/operator responsibilities; this research command is outside body entry.
Every response passes through the actual native admission CLI before scoring.

Case receipts retain native results, original responses and expectations behind
the evaluation boundary. Metrics include exact/set accuracy, multi-label F1,
ambiguity, abstention, unsupported certainty, refusal, bypass, operation routing,
actual input bytes and observed provider process elapsed quantiles. Unmeasured
invariance, warm/cold model latency, body parity and tool success stay explicit.
Candidate-set accuracy scores operative sets per eligible head. Unsupported
certainty and abstention use eligible heads whose gold set is empty; raw proposed
certainty and operative certainty are separate. Early admission refusals have
their own attempt-based rate and cannot receive abstention credit. Explicitly
expected kernel refusals remain valid negative outcomes.
These metric checks are not a stock-model benchmark or a frozen held-out suite.
