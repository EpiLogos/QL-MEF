/**
 * Refinement-home kernel provider binding (L5 Technē surfaces port).
 *
 * Canonical provider: EpiLogos/O-I `desktop/cradle/src/kernel/KernelProvider.tsx`
 * (transport detection, boot state, typed KernelOp channel). The refinement
 * home has no kernel transport of its own: the default is an honest
 * `unavailable`, and a host (the Cradle line, or a future refinement shell)
 * injects a real transport through `TechneKernelTransportProvider`.
 */
import { createContext, useContext, useMemo, type ReactNode } from "react";
import type { KernelTransportStatus } from "./types";

export interface KernelApi {
  transport: KernelTransportStatus;
}

const TechneKernelTransportContext = createContext<KernelApi>({
  transport: { kind: "unavailable", reason: "the refinement home discloses no kernel transport — the host injects one" },
});

export function TechneKernelTransportProvider({ transport, children }: { transport: KernelTransportStatus; children: ReactNode }) {
  const value = useMemo(() => ({ transport }), [transport]);
  return <TechneKernelTransportContext.Provider value={value}>{children}</TechneKernelTransportContext.Provider>;
}

export function useKernel(): KernelApi {
  return useContext(TechneKernelTransportContext);
}
