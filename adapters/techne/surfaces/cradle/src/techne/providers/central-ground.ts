/**
 * The T9 central-ground provider: a real `TechneSource` over the actual
 * Central ground, composed for the #220 integrated-field proving walk.
 *
 * The subject is the Technē dual-reading convergence as it really exists in
 * native sources — QL-MEF #220 and its consumed ground (#212–#219, the TB0
 * pin, the dual-reading and Vāk locks, the lane revisions, the O:I #65
 * absorption and the Central/ProjectCentral records). Nothing in the composed
 * reading is invented:
 *
 *   - every relation is asserted by a named source (issue bodies, lane
 *     receipts, the pin record, the locks themselves);
 *   - every time fact comes from a native read (the `ctrl` capture of
 *     central.now.list / central.time.policy / central.day.read, lane head
 *     commit instants, receipt-comment instants);
 *   - the single place facet carries exactly what a native source asserts
 *     (the adopted civil-time policy scope) with its uncertainty stated and
 *     no geometry manufactured;
 *   - standing strings are the sources' own self-declarations, carried
 *     verbatim.
 *
 * The capture fixture is a provenance-stamped record of real `ctrl` output;
 * `refreshLiveControl()` re-reads the same native Actions on this machine so
 * the provider can be proven against live ground, not only the capture.
 * Routing-only discipline applies to every disclosed Action: the adapter
 * routes, only the native owner executes.
 */
import {
  type AgencyRole,
  type NativeActionRef,
  type TechneDisclosure,
  type TechnePlaceFacet,
  type TechneReading,
  type TechneSourceProvenance,
  type TechneTemporalFacet,
  type TechneWholeRelation,
  validateReading,
} from '../contract.ts';
import { createTechneSource, type TechneSource } from '../adapter.ts';

// ---------------------------------------------------------------------------
// The capture (real `ctrl` reads, provenance-stamped)
// ---------------------------------------------------------------------------

export interface CentralGroundCapture {
  schema: 'ql.techne.capture/v1';
  capture_ref: string;
  captured_at: number;
  captured_local: string;
  provenance: { command: string; native_owner: string; authority: string }[];
  now_list: { data: { records: CaptureNowRecord[] } };
  time_policy: { data: CaptureTimePolicy };
  day_read: { data: CaptureDayReading };
}

export interface CaptureNowRecord {
  now_ref: string;
  purpose: string;
  created_at_unix_seconds: number;
  lifecycle: string;
  task_ref?: string;
  source_refs?: string[];
}

export interface CaptureTimePolicy {
  civil_date: string;
  local_time: string;
  observed_at_unix_seconds: number;
  policy: { timezone: string; scope_ref: string; schema: string };
}

export interface CaptureDayReading {
  day_ref: string;
  relations_revision?: string;
}

// ---------------------------------------------------------------------------
// The real convergence ground (ref table with sources' own standing)
// ---------------------------------------------------------------------------

export const CONTROL = 'central:source:control:root';
export const QL_REPO = 'git:EpiLogos/QL-MEF';
/** The converged joined field: `techne/tb0-surfaces-port` at the fan-out
 * convergence head (the #212 integrator note records it). */
export const CONVERGED_HEAD = '07cfe4a8ca55f78a72191b5abcc5ec695b16866a';
export const TB0_HEAD = '2c704dcc6911a4d0a11b20a23a71c4d222228e14';

export interface LaneHead {
  issue_ref: string;
  issue_number: number;
  lane: string;
  branch: string;
  head: string;
  head_instant: string;
  receipt_instant: string;
}

/** The seven consumed lanes: exact heads and receipt instants as recorded on
 * the issues (receipt comments) and in the local git history (head commits). */
export const LANES: readonly LaneHead[] = [
  { issue_ref: 'github:EpiLogos/QL-MEF/issues/213', issue_number: 213, lane: "M0′ Project / Wiki / Graph", branch: 'techne/m0-project-graph', head: '2d4a9360a65965d392592462c11a913871292a44', head_instant: '2026-09-16T23:33:54+01:00', receipt_instant: '2026-09-16T22:35:17Z' },
  { issue_ref: 'github:EpiLogos/QL-MEF/issues/214', issue_number: 214, lane: "M1′ Canvas / Constellation", branch: 'techne/m1-canvas', head: 'aa920e65284aee66b64417254087020d9b087fa4', head_instant: '2026-09-16T23:37:18+01:00', receipt_instant: '2026-09-16T22:38:20Z' },
  { issue_ref: 'github:EpiLogos/QL-MEF/issues/215', issue_number: 215, lane: "M2′ Relation Field / Timeline", branch: 'techne/m2-relation-field', head: '374bf3197a59e42c958b078b913bcdee79cd9584', head_instant: '2026-09-16T23:43:51+01:00', receipt_instant: '2026-09-16T22:45:01Z' },
  { issue_ref: 'github:EpiLogos/QL-MEF/issues/216', issue_number: 216, lane: "M3′ Journey / Scenes", branch: 'techne/m3-journey-scenes', head: '9267e0856de78f3e2291ff04ee66dd7686314b31', head_instant: '2026-09-16T23:44:31+01:00', receipt_instant: '2026-09-16T22:46:13Z' },
  { issue_ref: 'github:EpiLogos/QL-MEF/issues/217', issue_number: 217, lane: "M4′ World / Places", branch: 'techne/m4-world-places', head: '239bc5b94ea87c8ee0fff98f7f57536e0b693018', head_instant: '2026-09-16T23:27:10+01:00', receipt_instant: '2026-09-16T22:27:59Z' },
  { issue_ref: 'github:EpiLogos/QL-MEF/issues/218', issue_number: 218, lane: "M5′ Palace / Integral Whole", branch: 'techne/m5-palace', head: 'bd891081324a68a21c542126bdbfbefa706671ca', head_instant: '2026-09-16T23:38:52+01:00', receipt_instant: '2026-09-16T22:40:50Z' },
  { issue_ref: 'github:EpiLogos/QL-MEF/issues/219', issue_number: 219, lane: '3:3 Expression / Agency / desktop cross-cut', branch: 'techne/x3-expression-agency', head: '2ae7537341b76c6b2914c3e19a051a52815c9230', head_instant: '2026-09-16T23:37:55+01:00', receipt_instant: '2026-09-16T22:39:21Z' },
];

export const LOCK_DOC = `${CONTROL}:Work/Quaternal-Logic/docs/L5-TECHNE-DUAL-READING-LOCK.md`;
export const TB0_PIN_DOC = `${CONTROL}:Work/Quaternal-Logic/docs/L5-TECHNE-TB0-CONNECTIVE-BASE.md`;
export const VAK_LOCK_DOC = `${CONTROL}:Work/Quaternal-Logic/docs/integrations/epi-logos/TA-ONTA-VAK-COMPOSITIONAL-EXECUTION-LOCK.md`;
export const TB0_FIXTURE = `${CONTROL}:Work/Quaternal-Logic/fixtures/techne/tb0-connective-base-v1.json`;
export const OI_ABSORPTION_DOC = 'git:EpiLogos/O-I:docs/experience/TECHNE-DUAL-READING.md';
export const OI_CAMPAIGN = 'github:EpiLogos/O-I/issues/65';
export const TB0_ISSUE = 'github:EpiLogos/QL-MEF/issues/212';
export const T9_ISSUE = 'github:EpiLogos/QL-MEF/issues/220';
export const QL_GUARDIAN_EXPRESSION = `${CONTROL}:Control/agents/expressions/ql-guardian`;
export const CIVIL_TIME_POLICY = `${CONTROL}:Control/user/civil-time-policy.json`;
export const SURFACES_PORT_RECORD = `${CONTROL}:Work/Quaternal-Logic/ProjectCentral/now/agents/l5-techne-surfaces-ported-to-ql-mef-home-2026-09-16.json`;
export const SUBJECT_REF = T9_ISSUE;
export const WHOLE_REF = `${QL_REPO}:techne/tb0-surfaces-port@${CONVERGED_HEAD.slice(0, 7)}`;

/** The real AIKit session context this provider runs inside (from
 * `aikit status` on this machine at capture time). Native grammar:
 * `aikit:agent-session:<context id>`. */
export const AGENT_SESSION_REF = 'aikit:agent-session:ctx_01M2PCRX0QEG1YSV1WBNWTPDHT';

// ---------------------------------------------------------------------------
// Reading composition
// ---------------------------------------------------------------------------

function isoFromUnix(seconds: number): string {
  return new Date(seconds * 1000).toISOString();
}

/** Relations asserted by named sources. `standing` is the source's own claim
 * class: `sourced` = a receipt/commit/record says so; `authored` = the
 * owner-ratified lock asserts it as canonical architecture. */
function groundRelations(lanes: readonly LaneHead[]): TechneWholeRelation[] {
  const relations: TechneWholeRelation[] = [];
  relations.push({
    relation: 'pins',
    from_ref: TB0_ISSUE,
    to_ref: TB0_PIN_DOC,
    relation_ref: 'central:relation:t9:tb0-pins-connective-base',
    standing: 'sourced',
    source_ref: 'github:EpiLogos/QL-MEF/issues/212#comment-2026-09-16T18:35:21Z',
    evidence_refs: [TB0_ISSUE, TB0_PIN_DOC, `git-commit:${TB0_HEAD}`],
  });
  for (const lane of lanes) {
    relations.push({
      relation: 'parent-connective-base',
      from_ref: lane.issue_ref,
      to_ref: TB0_ISSUE,
      relation_ref: `central:relation:t9:lane-${lane.issue_number}-parent-base`,
      standing: 'sourced',
      source_ref: `${lane.issue_ref} (body: "Parent connective base: #212")`,
      evidence_refs: [lane.issue_ref, TB0_ISSUE],
    });
    relations.push({
      relation: 'implemented-in',
      from_ref: lane.issue_ref,
      to_ref: `${QL_REPO}:${lane.branch}@${lane.head.slice(0, 7)}`,
      relation_ref: `central:relation:t9:lane-${lane.issue_number}-implemented-in`,
      standing: 'sourced',
      source_ref: `${lane.issue_ref} receipt comment ${lane.receipt_instant}`,
      evidence_refs: [`git-commit:${lane.head}`],
      temporal_facet_ref: `tf:occ:lane-${lane.issue_number}`,
    });
    relations.push({
      relation: 'governed-by',
      from_ref: lane.issue_ref,
      to_ref: LOCK_DOC,
      relation_ref: `central:relation:t9:lane-${lane.issue_number}-governed-by-lock`,
      standing: 'sourced',
      source_ref: `${lane.issue_ref} (body: "Conceptual authority: docs/L5-TECHNE-DUAL-READING-LOCK.md")`,
      evidence_refs: [lane.issue_ref, LOCK_DOC],
    });
  }
  // The Vāk lock governs the execution-language lanes (their bodies name it).
  for (const n of [216, 219]) {
    relations.push({
      relation: 'executes',
      from_ref: `github:EpiLogos/QL-MEF/issues/${n}`,
      to_ref: VAK_LOCK_DOC,
      relation_ref: `central:relation:t9:lane-${n}-executes-vak-lock`,
      standing: 'sourced',
      source_ref: `github:EpiLogos/QL-MEF/issues/${n} (body: "Execution language authority")`,
      evidence_refs: [`github:EpiLogos/QL-MEF/issues/${n}`, VAK_LOCK_DOC],
    });
  }
  relations.push({
    relation: 'consumes',
    from_ref: T9_ISSUE,
    to_ref: TB0_ISSUE,
    relation_ref: 'central:relation:t9:consumes-tb0',
    standing: 'sourced',
    source_ref: `${T9_ISSUE} (body: "Dependencies: #212 …")`,
    evidence_refs: [T9_ISSUE, TB0_ISSUE],
  });
  for (const lane of lanes) {
    relations.push({
      relation: 'consumes',
      from_ref: T9_ISSUE,
      to_ref: lane.issue_ref,
      relation_ref: `central:relation:t9:consumes-lane-${lane.issue_number}`,
      standing: 'sourced',
      source_ref: `${T9_ISSUE} (body dependency list)`,
      evidence_refs: [T9_ISSUE, lane.issue_ref],
    });
  }
  relations.push({
    relation: 'merges',
    from_ref: `${QL_REPO}:techne/tb0-surfaces-port@${CONVERGED_HEAD.slice(0, 7)}`,
    to_ref: `${QL_REPO}:techne/tb0-surfaces-port@957d2bc`,
    relation_ref: 'central:relation:t9:convergence-merges-lanes',
    standing: 'sourced',
    source_ref: 'github:EpiLogos/QL-MEF/issues/212#comment-2026-09-16T23:05:53Z (fan-out convergence note)',
    evidence_refs: [`git-commit:${CONVERGED_HEAD}`],
  });
  relations.push({
    relation: 'feeds',
    from_ref: T9_ISSUE,
    to_ref: OI_CAMPAIGN,
    relation_ref: 'central:relation:t9:feeds-oi65',
    standing: 'sourced',
    source_ref: `${T9_ISSUE} (body: "This lane feeds the existing O:I #65 convergence/experience campaign")`,
    evidence_refs: [T9_ISSUE, OI_CAMPAIGN],
  });
  relations.push({
    relation: 'absorbs',
    from_ref: OI_ABSORPTION_DOC,
    to_ref: LOCK_DOC,
    relation_ref: 'central:relation:t9:oi65-absorbs-lock',
    standing: 'sourced',
    source_ref: 'github:EpiLogos/O-I issue 65 comment 2026-09-16T14:56Z (Technē convergence amendment adopted via PR #357)',
    evidence_refs: [OI_ABSORPTION_DOC, LOCK_DOC, 'github:EpiLogos/O-I/pull/357'],
  });
  relations.push({
    relation: 'records',
    from_ref: SURFACES_PORT_RECORD,
    to_ref: `${QL_REPO}:techne/tb0-surfaces-port@957d2bc`,
    relation_ref: 'central:relation:t9:projectcentral-records-port',
    standing: 'sourced',
    source_ref: `${SURFACES_PORT_RECORD} (central.project-now.handoff/v1)`,
    evidence_refs: [SURFACES_PORT_RECORD],
  });
  // Trans-temporal by nature: the Guardian's enduring stewardship of the QL
  // field, asserted by the Guardian expression ground. Deliberately no
  // temporal qualification — the contract treats absence as trans-temporal.
  relations.push({
    relation: 'stewards',
    from_ref: QL_GUARDIAN_EXPRESSION,
    to_ref: `${CONTROL}/Work/Quaternal-Logic`,
    relation_ref: 'central:relation:t9:ql-guardian-stewards-field',
    standing: 'authored',
    source_ref: `${CONTROL}:Control/agents/expressions/GUARDIANS.md ("ql-guardian/ — Quaternal Logic Guardian, transcendent relation")`,
    evidence_refs: [QL_GUARDIAN_EXPRESSION],
  });
  return relations;
}

function groundTemporal(capture: CentralGroundCapture, lanes: readonly LaneHead[]): TechneTemporalFacet[] {
  const temporal: TechneTemporalFacet[] = [];
  temporal.push({
    facet_ref: 'tf:policy',
    kind: 'valid',
    instant: isoFromUnix(capture.time_policy.data.observed_at_unix_seconds),
    precision: 'second',
    timezone_policy_ref: CIVIL_TIME_POLICY,
    uncertainty: 'the policy governs the capture instant; the policy file asserts no adoption instant',
    source_ref: CIVIL_TIME_POLICY,
  });
  temporal.push({
    facet_ref: 'tf:day:latest-closed',
    kind: 'day',
    day_ref: capture.day_read.data.day_ref,
    precision: 'day',
    timezone_policy_ref: CIVIL_TIME_POLICY,
    source_ref: 'central:action:central.day.read (capture)',
  });
  temporal.push({
    facet_ref: 'tf:capture',
    kind: 'session',
    session_ref: AGENT_SESSION_REF,
    instant: isoFromUnix(capture.captured_at),
    precision: 'second',
    timezone_policy_ref: CIVIL_TIME_POLICY,
    source_ref: capture.capture_ref,
  });
  // Three real root NOW clearings from the live capture — the concurrent
  // development state the ground actually holds.
  for (const [i, record] of capture.now_list.data.records.slice(0, 3).entries()) {
    temporal.push({
      facet_ref: `tf:now:${i}`,
      kind: 'now',
      now_ref: record.now_ref,
      instant: isoFromUnix(record.created_at_unix_seconds),
      precision: 'second',
      timezone_policy_ref: CIVIL_TIME_POLICY,
      source_ref: 'central:action:central.now.list (capture)',
    });
  }
  // Occurrence (head commit instant) vs receipt (issue comment instant) for
  // every lane — the distinction the relation field must keep.
  for (const lane of lanes) {
    temporal.push({
      facet_ref: `tf:occ:lane-${lane.issue_number}`,
      kind: 'occurrence',
      instant: lane.head_instant,
      precision: 'second',
      timezone_policy_ref: CIVIL_TIME_POLICY,
      source_ref: `git:${lane.branch}@${lane.head.slice(0, 7)}`,
    });
    temporal.push({
      facet_ref: `tf:rec:lane-${lane.issue_number}`,
      kind: 'receipt',
      instant: lane.receipt_instant,
      precision: 'minute',
      timezone_policy_ref: CIVIL_TIME_POLICY,
      source_ref: `${lane.issue_ref}#lane-receipt`,
    });
  }
  return temporal;
}

function groundProvenance(capture: CentralGroundCapture): TechneSourceProvenance[] {
  return [
    {
      source_ref: LOCK_DOC,
      source_revision: CONVERGED_HEAD,
      native_owner: 'ql-mef',
      selector: { unit: 'other', kind: 'file', value: 'docs/L5-TECHNE-DUAL-READING-LOCK.md' },
      standing: 'owner-ratified-conceptual-architecture',
      evidence_refs: [`git-commit:${CONVERGED_HEAD}`],
    },
    {
      source_ref: TB0_PIN_DOC,
      source_revision: CONVERGED_HEAD,
      native_owner: 'ql-mef',
      selector: { unit: 'other', kind: 'file', value: 'docs/L5-TECHNE-TB0-CONNECTIVE-BASE.md' },
      standing: 'implementation-record',
      evidence_refs: [`git-commit:${TB0_HEAD}`, 'github:EpiLogos/QL-MEF/pull/224'],
    },
    {
      source_ref: VAK_LOCK_DOC,
      source_revision: CONVERGED_HEAD,
      native_owner: 'ql-mef',
      selector: { unit: 'other', kind: 'file', value: 'docs/integrations/epi-logos/TA-ONTA-VAK-COMPOSITIONAL-EXECUTION-LOCK.md' },
      standing: 'owner-directed-integration-lock',
      evidence_refs: [`git-commit:${CONVERGED_HEAD}`],
    },
    {
      source_ref: capture.capture_ref,
      source_revision: `captured-at:${capture.captured_at}`,
      native_owner: 'central/ctrl',
      selector: { unit: 'timestamp_range', from: capture.captured_local, to: capture.captured_local },
      standing: 'captured-native-read',
      evidence_refs: capture.provenance.map((p) => p.command),
    },
    {
      source_ref: SURFACES_PORT_RECORD,
      source_revision: null,
      native_owner: 'central/projectcentral',
      selector: { unit: 'other', kind: 'file', value: 'ProjectCentral/now/agents/l5-techne-surfaces-ported-to-ql-mef-home-2026-09-16.json' },
      standing: 'agent-maintained-now-record',
      evidence_refs: ['central.project-now.handoff/v1'],
    },
    {
      source_ref: OI_ABSORPTION_DOC,
      source_revision: 'git:EpiLogos/O-I@35fa86d',
      native_owner: 'o-i',
      selector: { unit: 'other', kind: 'file', value: 'docs/experience/TECHNE-DUAL-READING.md' },
      standing: 'campaign-amendment',
      evidence_refs: ['github:EpiLogos/O-I/pull/357'],
    },
    {
      source_ref: QL_GUARDIAN_EXPRESSION,
      source_revision: null,
      native_owner: 'central/expressions',
      selector: { unit: 'other', kind: 'file', value: 'Control/agents/expressions/ql-guardian' },
      standing: 'authored-guardian-expression-ground',
      evidence_refs: [`${QL_GUARDIAN_EXPRESSION}/OFFICE.md`],
    },
  ];
}

function groundActions(): NativeActionRef[] {
  return [
    {
      action_ref: 'central.now.list',
      native_owner: 'central/ctrl',
      authority: 'registered-read-action',
      summary: 'List the root NOW field (read-only ground read).',
      expected_effects: ['reads the moving NOW field; mutates nothing'],
    },
    {
      action_ref: 'central.day.read',
      native_owner: 'central/ctrl',
      authority: 'registered-read-action',
      summary: 'Read a Central DAY record (read-only ground read).',
      expected_effects: ['reads the dated day record; mutates nothing'],
    },
    {
      action_ref: 'central.time.policy',
      native_owner: 'central/ctrl',
      authority: 'registered-read-action',
      summary: 'Read the civil-time policy and current local time (read-only).',
      expected_effects: ['reads the adopted time policy; mutates nothing'],
    },
    {
      action_ref: 'ql.vak.compose',
      native_owner: 'ql/kernel',
      authority: 'registered-read-action',
      summary: 'Compose C′ passages over the kernel (compute-only; no native mutation).',
      expected_effects: ['returns a composition result graph; mutates nothing'],
    },
    {
      action_ref: 'git.commit',
      native_owner: 'ql-mef-repository',
      authority: 'repo-ground-write',
      summary: 'Commit the T9 integration record into the repository ground.',
      expected_effects: ['new commit revising docs/ and the cradle tree; receipt is the commit SHA'],
      input_schema_ref: 'git:commit/v1',
    },
    {
      action_ref: 'projectcentral.now.return',
      native_owner: 'central/ctrl',
      authority: 'governed-write',
      summary: 'Allocate a bounded, attributed session return in the QL-MEF ProjectCentral NOW field.',
      expected_effects: ['new central.project-now.handoff/v1 record under ProjectCentral/now/agents/'],
      input_schema_ref: 'central.project-now.handoff/v1',
    },
    {
      action_ref: 'oi.expression.open',
      native_owner: 'oi/desktop',
      authority: 'owner-disclosed',
      summary: 'Open the conjugate 3:3 reading of the current subject (the cut-crossing Action).',
      expected_effects: ['crosses the session to the expressions reading; subject/selection basis ride unchanged'],
    },
  ];
}

function groundAgency(): AgencyRole[] {
  return [
    {
      role: 'guardian',
      m_index: 5,
      guardian_ref: QL_GUARDIAN_EXPRESSION,
      authority: 'stewardship',
    },
    {
      role: 'anima',
      m_index: 5,
      reading: '3:3-conjugate',
      instrument: 'expressions',
      guardian_ref: QL_GUARDIAN_EXPRESSION,
      agent_session_ref: AGENT_SESSION_REF,
      authority: 'registered-read-action',
    },
    {
      role: 'aletheia',
      m_index: 5,
      reading: '4:2-deep',
      instrument: 'palace',
      guardian_ref: QL_GUARDIAN_EXPRESSION,
      agent_session_ref: AGENT_SESSION_REF,
      authority: 'registered-read-action',
    },
    {
      role: 'techne',
      m_index: 5,
      reading: '4:2-deep',
      instrument: 'palace',
      guardian_ref: QL_GUARDIAN_EXPRESSION,
      agent_session_ref: AGENT_SESSION_REF,
      authority: 'registered-read-action',
    },
  ];
}

function groundDisclosure(): TechneDisclosure {
  const instruments: TechneDisclosure['instruments'] = [
    { instrument: 'project', available: true, m_prime: 0, reading: '4:2-deep' },
    { instrument: 'canvas', available: true, m_prime: 1, reading: '4:2-deep' },
    { instrument: 'timeline', available: true, m_prime: 2, reading: '4:2-deep' },
    { instrument: 'journey', available: true, m_prime: 3, reading: '4:2-deep' },
    { instrument: 'place', available: true, m_prime: 4, reading: '4:2-deep' },
    { instrument: 'palace', available: true, m_prime: 5, reading: '4:2-deep' },
    { instrument: 'expressions', available: true, reading: '3:3-conjugate' },
  ];
  return {
    instruments,
    degraded: [
      { instrument: 'place', reason: 'the native ground asserts one civil-time frame, not surveyed geography: the place facet carries the policy scope with uncertainty stated and no geometry; deeper geographic depth has no installed provider' },
      { instrument: 'journey', reason: 'scene composition routes real scene_create proposals in the oi.expression grammar; live scene persistence waits for the refined surfaces to port back into the O:I desktop (techne/convergence hosts the pre-lane generation)' },
      { instrument: 'canvas', reason: 'authored Views and the presentation store are session-scoped; durable view persistence wants the O:I host seam' },
      { instrument: 'palace', reason: 'composition binding and arrangements are session-scoped presentation state; no native producer binds integral compositions yet' },
      { instrument: 'expressions', reason: 'the crossing and cue machinery are proven at session level in this tree; a live running Expression substrate waits for the O:I desktop port-back' },
    ],
    application_cuts: [
      { cut: '4:2-deep', available: true },
      { cut: '3:3-conjugate', available: true },
    ],
    suggestions: [
      { instrument: 'project', reason: 'begin at the ground instrument: the bounded whole is the converged TB0 field' },
      { instrument: 'expressions', reason: 'the 3:3 conjugate reading of this subject is the #65 campaign absorption (TD/V cases)' },
      { instrument: 'palace', reason: 'close the walk here: the integral articulation and the governed Return leg' },
    ],
  };
}

/** Compose the real central-ground reading. Throws when the composition
 * violates the TB0-1 contract — a malformed provider is a defect, not a
 * degraded surface. */
export function composeCentralGroundReading(
  capture: CentralGroundCapture,
  options: { converged_head: string; lanes?: readonly LaneHead[] } = { converged_head: CONVERGED_HEAD },
): TechneReading {
  const lanes = options.lanes ?? LANES;
  const head = options.converged_head;
  const reading: TechneReading = {
    contract: 'ql.techne/v1',
    reading_ref: 'ql.techne:reading:central-ground-t9@1',
    snapshot: { revision: head, basis_ref: `${QL_REPO}:techne/tb0-surfaces-port@${head.slice(0, 7)}` },
    subject: {
      subject_ref: SUBJECT_REF,
      native_owner: 'github',
      native_revision: 'open (receipts 2026-09-16/17)',
      readings: [{ ref: LOCK_DOC, revision: head }],
      kind: 'issue',
      standing: 'open-convergence-lane',
    },
    whole: {
      whole_ref: `${QL_REPO}:techne/tb0-surfaces-port@${head.slice(0, 7)}`,
      member_refs: [
        LOCK_DOC,
        TB0_PIN_DOC,
        VAK_LOCK_DOC,
        TB0_FIXTURE,
        TB0_ISSUE,
        ...lanes.map((l) => l.issue_ref),
        ...lanes.map((l) => `${QL_REPO}:${l.branch}@${l.head.slice(0, 7)}`),
        OI_ABSORPTION_DOC,
        OI_CAMPAIGN,
        SURFACES_PORT_RECORD,
        QL_GUARDIAN_EXPRESSION,
        CIVIL_TIME_POLICY,
      ],
      relations: groundRelations(lanes),
      focus_refs: [SUBJECT_REF],
    },
    ql: {
      m_coordinate_ref: 'ql:structural:5.0.0',
      shape_ref: 'ql:shape:1.0.0:6x6:direct-conjugate',
      lens_ref: 'mef:lens:L5@1',
      context_frame_ref: 'mef:context-frame:CF4',
      refraction_summary: 'The one M′ field read through the L5 office: the six deep instruments M0′–M5′ and the conjugate 3:3 Expression reading over the same converged ground.',
      vak_source_ref: 'ql:vak:source:daa660cbc1b8c5da83828698665a753852cb0287',
      return_ref: `${QL_REPO}:techne/tb0-surfaces-port@${head.slice(0, 7)}`,
      warrant: {
        result_class: 'deterministic',
        evidence_refs: ['ql:vak:source:daa660cbc1b8c5da83828698665a753852cb0287', 'ql capabilities (live kernel read, capture session)'],
        provenance_ref: capture.capture_ref,
      },
    },
    temporal: groundTemporal(capture, lanes),
    spatial: groundSpatial(capture),
    provenance: groundProvenance(capture),
    expressions: [
      {
        expression_ref: QL_GUARDIAN_EXPRESSION,
        revision: null,
        scene_ref: null,
        composition_ref: null,
        profile_ref: null,
      },
    ],
    actions: groundActions(),
    agency: groundAgency(),
    disclosure: groundDisclosure(),
  };
  const errors = validateReading(reading);
  if (!errors.valid) {
    throw new Error(`central-ground provider composed an invalid reading: ${errors.errors.join('; ')}`);
  }
  return reading;
}

/** The one place facet the native ground actually asserts: the adopted
 * civil-time policy scopes the ground to the Europe/London frame. Region
 * precision, uncertainty stated, no geometry manufactured. */
function groundSpatial(_capture: CentralGroundCapture): TechnePlaceFacet[] {
  return [
    {
      place_ref: 'iana:timezone:Europe/London',
      relation: 'OPERATED_IN',
      identity: { names: [{ name: 'Europe/London (IANA timezone region)' }] },
      precision: 'region',
      uncertainty: 'civil-time policy scope; no surveyed geometry; coordinates intentionally absent',
      source_ref: CIVIL_TIME_POLICY,
    },
  ];
}

// ---------------------------------------------------------------------------
// The source seam
// ---------------------------------------------------------------------------

/** Register the central-ground reading as the session's TechneSource.
 * Renewal (the 5→0 re-open) recomposes from a fresh capture — the caller
 * re-reads the live native Actions (`central.now.list`, `central.time.policy`,
 * `central.day.read`) and passes the new capture in; this src module stays
 * pure and the native reads stay with the session that performs them. */
export function centralGroundSource(capture: CentralGroundCapture, convergedHead = CONVERGED_HEAD): TechneSource {
  const reading = composeCentralGroundReading(capture, { converged_head: convergedHead });
  return createTechneSource({
    ref: reading.reading_ref,
    title: 'Central ground — the Technē dual-reading convergence (T9)',
    read: async () => reading,
  });
}
