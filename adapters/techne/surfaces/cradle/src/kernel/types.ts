/**
 * Refinement-home TYPE SUBSET of the Cradle kernel type hub.
 *
 * Canonical file: EpiLogos/O-I `desktop/cradle/src/kernel/types.ts` (the full
 * hub carries the whole KernelOp/KernelOpResult grammar and pulls the
 * Cradle's configuration/workspace/files/encounter/receiving types with it).
 * This ported surface tree needs exactly two type-only members; they are
 * copied VERBATIM from the canonical file so the surfaces typecheck
 * standalone. Drift is a port bug — re-copy the two definitions, never
 * edit them here.
 */

export interface CentralLocation { schema: "central.path-ref/v1"; ref: string; root: string; path: string }
export interface KnowledgeAddress { kind: "wiki" | "source" | "project-map"; value: string }

// --- KernelOp transport channel (verbatim shapes; canonical file carries the
// full typed op grammar, which the refinement home does not need) ---

export type KernelTransportStatus =
  | { kind: "tauri" }
  | { kind: "bridge"; url: string }
  | { kind: "unavailable"; reason: string };

export interface KernelOutcome {
  result: string;
  data?: unknown;
  receipts?: Array<Record<string, unknown>>;
}

export interface KernelOpCall {
  outcome: KernelOutcome | null;
  error?: string;
}
