/**
 * Refinement-home kernel transport binding (L5 Technē surfaces port).
 *
 * Canonical transport: EpiLogos/O-I `desktop/cradle/src/kernel/bridge.ts` —
 * the typed KernelOp channel over Tauri invoke / walk bridge. This binding
 * carries the SAME call contract (`kernelOp(transport, op)` → KernelOpCall,
 * same three transport branches) so the ported surfaces typecheck and refine
 * standalone; the canonical file governs. QL-MEF acquires no renderer or
 * window ownership (dual-reading lock §II).
 */
import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import type { KernelOpCall, KernelOutcome, KernelTransportStatus } from "./types";

function normaliseOutcome(outcome: KernelOutcome): KernelOutcome {
  return { ...outcome, receipts: outcome.receipts ?? [] };
}

export async function kernelOp(
  transport: KernelTransportStatus,
  op: { op: string; [key: string]: unknown },
): Promise<KernelOpCall> {
  try {
    if (transport.kind === "tauri") {
      const outcome = await tauriInvoke<KernelOutcome>("kernel_op", { op });
      return { outcome: outcome ? normaliseOutcome(outcome) : null };
    }
    if (transport.kind === "bridge") {
      const response = await fetch(`${transport.url}/op`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify(op),
      });
      const body = (await response.json()) as { ok: boolean; outcome?: KernelOutcome; error?: string };
      if (!body.ok) return { outcome: null, error: body.error ?? "the kernel refused the operation" };
      return { outcome: body.outcome ? normaliseOutcome(body.outcome) : null };
    }
    return { outcome: null, error: transport.reason };
  } catch (error) {
    return { outcome: null, error: String(error) };
  }
}
