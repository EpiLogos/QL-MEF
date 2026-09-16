/**
 * Techne open requests (L5 Technē integration) — the K9 open-request
 * pattern generalised to the DisclosureSession: a lane or affordance
 * requests that the current session be disclosed as a Technē surface, and
 * the shell's one consumer (workspace/store.ts) opens or re-activates the
 * kind-"techne" binding. Requests carry a session that already exists;
 * nothing here mints subjects or sessions.
 *
 * TB0-1 / issue #219: the coordinate-preserving cross-cut actions ride this
 * seam too. `resolveCrossing` decides, from the reading's own disclosure
 * (`application_cuts[]` + per-instrument entries — never hard-coded), whether
 * a crossing can actually be entered and with what reason it is refused;
 * the session store's `crossCut` performs it byte-exact. Six standard legs
 * are named: open as Expression (enter 3:3), open deep instrument (enter
 * 4:2), open source/graph depth (the M0 depth), open Agent/Epii depth (a
 * summonable aperture that changes no cut), return to Journey position (M3)
 * and return to Project/Wiki ground (M0). No second command language: legs
 * ride the session's own navigation and, where the reading discloses one,
 * the native ActionRef they correspond to.
 */
import { disclosedCut, instrumentReading, mPrimeOf, type ApplicationCutDisclosure, type DisclosureSession, type TechneDisclosure, type TechneInstrument, type TechneReading, type TechneReadingKind } from "./contract.ts";
import { techneSurfaceBinding } from "./session.ts";

export interface TechneOpenRequest {
  session: DisclosureSession;
  title?: string;
}

const openListeners = new Set<(request: TechneOpenRequest) => void>();

export function requestTechneOpen(session: DisclosureSession, title?: string): void {
  const request: TechneOpenRequest = { session, title };
  for (const listener of openListeners) listener(request);
}

export function subscribeTechneOpen(listener: (request: TechneOpenRequest) => void): () => void {
  openListeners.add(listener);
  return () => {
    openListeners.delete(listener);
  };
}

/** The standard coordinate-preserving cross-cut legs (issue #219). */
export type TechneCrossingKind =
  | "open-as-expression"
  | "open-deep-instrument"
  | "open-source-graph-depth"
  | "open-agent-depth"
  | "return-to-journey-position"
  | "return-to-project-ground";

export interface TechneCrossing {
  kind: TechneCrossingKind;
  /** The instrument disclosed after the crossing; unchanged for
   * `open-agent-depth` (the Agent/Epii depth is a summonable aperture, not
   * another reading). */
  target: TechneInstrument;
  /** The cut the session occupies after the crossing. */
  cut: TechneReadingKind;
  /** False only for the Agent/Epii depth, which changes no cut. */
  crossesCut: boolean;
  /** The disclosed native ActionRef this leg corresponds to, when the
   * reading discloses one; the leg never executes it — routing stays with
   * the adapter's ActionRoute seam. */
  action_ref: string | null;
  /** The bound Expression Scene the leg carries (the M3 Journey position),
   * when the reading binds one and the leg names it. */
  scene_focus_ref: string | null;
}

export type TechneCrossingResolution =
  | { available: true; crossing: TechneCrossing }
  | { available: false; reason: string };

const AGENT_ACTION_OWNERS = ["actuation", "aikit", "epii"];

function disclosedActionRef(reading: TechneReading, actionRef: string): string | null {
  return reading.actions?.some((action) => action.action_ref === actionRef) ? actionRef : null;
}

function disclosedAgentActionRef(reading: TechneReading): string | null {
  return reading.actions?.find((action) => AGENT_ACTION_OWNERS.some((owner) => action.native_owner.startsWith(owner)))?.action_ref ?? null;
}

function instrumentEntry(disclosure: TechneDisclosure | null | undefined, instrument: TechneInstrument) {
  return disclosure?.instruments.find((entry) => entry.instrument === instrument);
}

/** The deep instrument a 4:2 entry defaults to: the first suggested deep
 * instrument, else the first available deep instrument, else project ground.
 * Derived from the disclosure only — never hard-coded per route. */
export function deepEntryInstrument(disclosure: TechneDisclosure | null | undefined): TechneInstrument {
  const deep = disclosure?.instruments.filter((entry) => mPrimeOf(entry.instrument) !== null);
  const suggested = disclosure?.suggestions?.map((note) => note.instrument).find((instrument) => mPrimeOf(instrument) !== null);
  if (suggested) return suggested;
  return deep?.find((entry) => entry.available)?.instrument ?? "project";
}

/** The cut-level disclosure entry for one cut, and the per-instrument entry
 * for `instrument`. Both govern: a cut is enterable only when the reading's
 * own disclosure says so — an unavailable leg carries its recorded reason,
 * never a fake disabled-but-secretly-working surface. `allowSameCutHop`
 * marks the depth/Return legs: when the session already occupies the
 * target's cut, the leg degrades honestly to a same-cut navigation hop
 * (gated on the instrument entry) instead of a crossing refusal — M0 is
 * both the 4:2 ground and a summonable depth of the 3:3 reading. */
function resolveCutLeg(reading: TechneReading, session: DisclosureSession, kind: TechneCrossingKind, target: TechneInstrument, actionRef: string | null, sceneFocus: string | null, allowSameCutHop = false): TechneCrossingResolution {
  const cut = instrumentReading(target);
  const current_cut = session.application_cut ?? instrumentReading(session.instrument);
  if (cut === current_cut) {
    if (!allowSameCutHop) {
      return { available: false, reason: `the session already occupies the ${cut} reading (${session.instrument}) — crossing requires the other cut` };
    }
    const hop = instrumentEntry(reading.disclosure, target);
    if (hop && !hop.available) {
      return { available: false, reason: hop.reason ?? `${target} is unavailable for this subject` };
    }
    return {
      available: true,
      crossing: { kind, target, cut, crossesCut: false, action_ref: actionRef, scene_focus_ref: sceneFocus },
    };
  }
  const cutEntry: ApplicationCutDisclosure | undefined = disclosedCut(reading.disclosure, cut);
  if (cutEntry && !cutEntry.available) {
    return { available: false, reason: cutEntry.reason ?? `the ${cut} reading is unavailable for this subject` };
  }
  const entry = instrumentEntry(reading.disclosure, target);
  if (entry && !entry.available) {
    return { available: false, reason: entry.reason ?? `${target} is unavailable for this subject` };
  }
  return {
    available: true,
    crossing: { kind, target, cut, crossesCut: true, action_ref: actionRef, scene_focus_ref: sceneFocus },
  };
}

/**
 * Resolve one standard cross-cut leg from the reading's own disclosure.
 * Availability is disclosure-driven end to end; a refused leg carries the
 * disclosure's recorded reason. The caller performs the refused-free crossing
 * through the session store's `crossCut` (or, for `open-agent-depth`, keeps
 * the session as-is) and emits it with `requestTechneCrossing`.
 */
export function resolveCrossing(reading: TechneReading, session: DisclosureSession, kind: TechneCrossingKind, targetInstrument?: TechneInstrument): TechneCrossingResolution {
  if (session.subject_ref !== reading.subject.subject_ref) {
    return { available: false, reason: `the session names ${session.subject_ref}, the reading discloses ${reading.subject.subject_ref} — one subject, one crossing` };
  }
  switch (kind) {
    case "open-as-expression":
      return resolveCutLeg(reading, session, kind, "expressions", disclosedActionRef(reading, "oi.expression.open"), null);
    case "open-deep-instrument": {
      const target = targetInstrument ?? deepEntryInstrument(reading.disclosure);
      if (mPrimeOf(target) === null) {
        return { available: false, reason: `${target} is not a deep instrument — the 4:2 entry names one of M0′–M5′` };
      }
      return resolveCutLeg(reading, session, kind, target, null, null);
    }
    case "open-source-graph-depth":
      return resolveCutLeg(reading, session, kind, "project", null, null, true);
    case "return-to-project-ground":
      return resolveCutLeg(reading, session, kind, "project", null, null, true);
    case "return-to-journey-position": {
      const sceneFocus = reading.expressions?.find((binding) => !!binding.scene_ref)?.scene_ref ?? null;
      return resolveCutLeg(reading, session, kind, "journey", null, sceneFocus);
    }
    case "open-agent-depth": {
      // The Agent/Epii depth is summonable without leaving the cut: one
      // companion session co-references the changing aperture. Availability
      // is disclosed material only — a native agent-owned action, or the
      // companion session ref the selection already carries.
      const actionRef = disclosedAgentActionRef(reading);
      if (!actionRef && !session.selection.agent_session_ref) {
        return { available: false, reason: "no agent-owned action is disclosed and the selection carries no companion agent-session ref — the Agent depth is unavailable here" };
      }
      return {
        available: true,
        crossing: {
          kind,
          target: session.instrument,
          cut: session.application_cut ?? instrumentReading(session.instrument),
          crossesCut: false,
          action_ref: actionRef,
          scene_focus_ref: session.scene_focus_ref ?? null,
        },
      };
    }
  }
}

export interface TechneCrossingRequest {
  crossing: TechneCrossing;
  /** The session AFTER the crossing (the caller crossed the session store
   * first); for `open-agent-depth` the unchanged session. */
  session: DisclosureSession;
  title?: string;
}

const crossingListeners = new Set<(request: TechneCrossingRequest) => void>();

/** Request the shell disclose the crossed session (opens or re-activates the
 * kind-"techne" binding, per the existing open grammar). */
export function requestTechneCrossing(request: TechneCrossingRequest): void {
  for (const listener of crossingListeners) listener(request);
}

export function subscribeTechneCrossing(listener: (request: TechneCrossingRequest) => void): () => void {
  crossingListeners.add(listener);
  return () => {
    crossingListeners.delete(listener);
  };
}

export { techneSurfaceBinding };
