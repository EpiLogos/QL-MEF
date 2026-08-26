# Epi-Logos / Pratibimba Product Build Order

**Status:** active R2 build-order correction; human review remains QL-MEF #25 / draft PR #27  
**Parent Wayfinder:** QL-MEF #30 / `EPI-LOGOS-DEVELOPMENT-WAYFINDER.md`  
**Product host:** O:I desktop #23, especially #25/#26  
**Primary experiential sources:** Epi `M'-SYSTEM-SPEC.md`, `M4'/M4'-SPEC.md`, `M'-TAURI-PORT-SPEC.md`  
**Computational sources:** Epi `Body/S/S0/epi-lib`, `Body/S/S0/portal-core`, M0–M5 source bodies  

## 1. Why this file exists

R2 relation mapping correctly preserves M/M′, the wider S/S′ field, Ta-Onta residency, QL-MEF authority and O:I ownership, but it temporarily allowed a **reasoning traversal** to masquerade as a **chronological product build order**.

That is wrong.

The fact that QL/MEF, M1/M2 harmonic structures or other formal systems are foundational to Epi's meaning does **not** imply that the first product tranche should be a deep M1/M2 implementation. The authored Pratibimba design already answers the human-product question:

```text
Pratibimba is encountered from the lived personal side.
M4′ / Nara is the primary daily experiential root.
M5′ / Epii and M0′ / Anuttara complete the personal 4/5/0 return.
M1′ / M2′ / M3′ form the structural/cosmic instrument feeding that lived field.
The six M′ domains remain available as deep workspaces.
```

At the same time, O:I now has the generic local human-and-agent application body that the historical Epi shell was previously forced to supply for itself.

Therefore the reconstitution is:

> **O:I-hosted, Nara-rooted Pratibimba, built outward from a minimal Epi computational/primitive bridge, with the cosmic M1′–M3′ systems introduced first as instruments feeding lived context and only later expanded into their full deep workspaces.**

## 2. Authority and supersession

Preserve three different kinds of authority.

### 2.1 Experiential / product-design authority

The Epi M′ corpus remains authoritative for what Pratibimba is meant to feel like and do:

- non-numbered 0/1 parent;
- lean daily-driver shell versus deep subsystem workspaces;
- Personal as the 4/5/0 lived-return face;
- Cosmic as the integrated 1/2/3 structural/cymatic instrument;
- M4′ Nara journal/flow as the primary lived stream;
- technical internals hidden by default and summonable through explanation;
- M5′ Epii as conversational pedagogy/review/self-articulation;
- M0′ Anuttara/Bimba as grounding, source and structural navigation;
- the six M′ domains as deep instruments/workspaces rather than six dashboard cards.

### 2.2 Current generic application-host authority

Current O:I desktop architecture supersedes the historical Epi shell **for generic host concerns**:

```text
application/window lifecycle
privileged local Rust bridge
native Surface/Component hosting
SessionSpace/provider integration
agent/harness/session presentation
project/application navigation
root/situated agent encounter
terminal / trajectory / events drawer
package/contribution registration
cross-product composition
```

Those generic concerns belong to O:I + AIKit + Actuation + Workcell as currently defined. Epi should not rebuild a second generic desktop/harness/session control plane merely because older M′ specs mention Theia, Electron, a dedicated Tauri shell or an OmniPanel implementation.

Historical shell documents remain **design/function evidence**. Their shell-technology choices are superseded where they conflict with the current O:I application decision.

### 2.3 Epi semantic authority

O:I hosting does not move Epi meaning into O:I. Epi retains:

```text
Bimba / EpiAddress / M/M′
canonical M-domain Agents
VAK / CF / CFP / Day-Night′ / Kairos
Matheme / harmonic-profile semantics specific to Epi
Nara protected personal field and promotion law
M0–M5 domain computation/content
M0′–M5′ Pratibimba instrument semantics
Ta-Onta S4′ constitutional inhabitation
Aletheia S4.5′ disclosure/crystallisation
Epii S5′ / M5′ review, pedagogy and return semantics
```

## 3. Product shape

The practical application relation is:

```text
O:I DESKTOP / AGENT IDE HOST
│
├─ global navigation / search / project-app rail
├─ primary canvas
├─ right context + situated/root-agent encounter
└─ optional lower drawer: trajectory / terminal / events / processes / logs

        ↓ Epi mode / Pratibimba contribution

PRATIBIMBA
│
├─ PERSONAL — first daily face
│   ├─ M4′ Nara flow / journal / today
│   ├─ DAY/NOW + current resonance
│   ├─ lean protected identity context
│   ├─ M5′ Epii pedagogy / review / return
│   └─ M0′ Anuttara / Bimba source-ground orientation
│
├─ COSMIC — integrated instrument
│   └─ M1′ + M2′ + M3′ shared structural/harmonic/cymatic/clock outputs
│
└─ DEEP WORKSPACES
    ├─ M0′ Anuttara / Bimba
    ├─ M1′ Paramasiva
    ├─ M2′ Parashakti
    ├─ M3′ Mahamaya
    ├─ M4′ Nara
    └─ M5′ Epii
```

The O:I host supplies the application/harness body. Pratibimba supplies the Epi world, instruments and lived composition.

## 4. Build-order invariant

### Phase 0 — consume the O:I host; do not create another shell

Start from the actual current O:I desktop (`desktop/core`, `desktop/src-tauri`, `desktop/ui`) and O:I #23/#25/#26.

First prove an **Epi mode / native contribution** can inhabit the existing host regions and stable-ref/action model. Extend the host only where the existing O:I desktop programme already calls for the required capability.

Do not begin by recreating:

- an Epi-only SessionSpace;
- a second agent chat/session store;
- a second terminal/process manager;
- a second capability/plugin registry;
- an Epi-specific generic IDE shell;
- the old Electron/Theia/Tauri process topology.

### Phase 1 — adopt the existing Epi kernel/library and expose the minimum primitive contract

Before rich UI, make the existing real Epi computation usable from the new host.

Prefer **adoption/bridging before rewrite**:

```text
Body/S/S0/epi-lib        C domain/kernel body
Body/S/S0/portal-core    Rust domain/projection body
        ↓
small typed Epi-owned bridge/API
        ↓
O:I-hosted Pratibimba contribution
```

The first bridge should expose only the shared primitives needed for the daily surface and later instruments, with source/provenance and readiness:

- canonical Bimba / EpiAddress / M/M′ identity;
- the M0–M5 family roots and six canonical M-domain Agent identities;
- current QL position/relation/lens/Context-Frame handles available from the accepted substrate;
- VAK address grammar: CPF / CT / CP / CF / CFP / CS;
- `MathemeHarmonicProfile` or its proven canonical successor;
- DAY/NOW / tick / temporal-condition handles;
- coordinate and source/provenance handles;
- Nara identity/current-field/day-episode handles required by M4′;
- Mahamaya/oracle/transcription handles only where already executable;
- explicit implemented / partial / stub / research status.

This is **not** “finish M1 and M2 first”. It is the smallest primitive/computation floor required so a Pratibimba surface can know what world it is in.

QL-MEF supplies canonical generalisable QL/MEF/harmonic operators where authority already exists. Epi retains Epi-specific correspondential, psychoid and instrument content. Narrow missing QL capabilities should be opened as dependencies when a real Pratibimba slice requires them; the whole future QL/MEF programme is not a gate on Nara.

### Phase 2 — M4′ Nara first lived vertical

Build the first genuine Pratibimba experience in the O:I host from Nara outward.

Minimum daily surface:

```text
flow / journal editor
DAY/NOW header
current resonance / harmonic context
lean protected identity sidebar
protected day-as-episode persistence
highlight / selection / sendoff
quick Nara/Anima agent encounter through the O:I agent region
source/provenance / Explain on demand
```

Follow the M4′ surface law:

> the journal knows the cosmic context without turning that context into a technical lecture.

Do not default to raw quaternions, giant correspondence tables, graph telemetry or full M1/M2/M3 inspectors.

### Phase 3 — complete the Personal 4/5/0 return

Once Nara is usable, complete the personal relation:

```text
M4′ Nara
   lived condition / episode / question
        ↓
M5′ Epii
   pedagogy / review / explanation / proposal / return
        ↓
M0′ Anuttara / Bimba
   source-ground / coordinate / canon orientation
        ↺
```

This is the primary everyday Epi experience.

The right-hand O:I agent encounter is the natural host for Nara/Anima/Epii conversation. The agent opens/summons relevant Epi surfaces in the primary canvas rather than requiring a permanently exposed technical control room.

### Phase 4 — introduce M1′–M3′ as one Cosmic instrument

Make the computational/cosmic system available first through the lean integrated Cosmic face:

```text
M1′ relational / mathematical movement
 +
M2′ harmonic / correspondential / vibrational field
 +
M3′ symbolic / clock / transcription field
        ↓
one current Matheme/cosmic instrument state
        ↓
Nara lived context + explicit Cosmic view
```

Consume the existing kernel outputs and shared profile. Do not require the full deep M1′, M2′ and M3′ workspaces before the clock/resonance context can inform Nara.

### Phase 5 — deepen the six M′ instruments

After the parent experience has a real working loop, expand each deep workspace according to its source spec:

- M0′ full Bimba explorer;
- M1′ full mathematical-musical/topological instrument;
- M2′ full MEF/correspondential/cymatic instrument;
- M3′ full clock/cosmos/transcription instrument;
- M4′ deep Nara dashboard, oracle, dream, activity, field and protected history;
- M5′ deep Epii pedagogy/canon/developer/Logos Atelier workbench.

Depth is demand-driven by the already-working lived surface, not a prerequisite to seeing the product.

### Phase 6 — deepen agentic inhabitation and research loops

Ta-Onta S4′ enters where an agent actually inhabits Epi's world and must act according to Epi constitutional form:

```text
S4.0′ Khora
S4.1′ Hen
S4.2′ Pleroma
S4.3′ Chronos
S4.4′ Anima
S4.5′ Aletheia
        ↓
S5′ Epii return
```

Generic model/harness/session/tool/material mechanics continue to come from AIKit/Actuation/Workcell/O:I. Do not port Ta-Onta as a second generic harness.

Advanced kernel completion, currently stubbed M4 faculties, richer M1/M2/M3 research operators, autoresearch and more ambitious self-development follow real product dependency and evidence.

## 5. Dependency order versus semantic depth

Keep this distinction explicit:

```text
SEMANTIC / FORMAL DEPTH
Bimba / Matheme / QL / M0-M5 / S/S′ may be foundational to meaning.

PRODUCT BUILD ORDER
O:I host
  → minimum Epi primitive/kernel bridge
  → Nara M4′ daily experience
  → Personal 4/5/0
  → Cosmic M1′+M2′+M3′ instrument preview
  → deep M0′–M5′ workspaces
  → advanced inhabitation/research.
```

A dependency may be foundational without being a user-facing first tranche. A deep subsystem may be important without blocking the first useful experience.

## 6. Vertical-slice rule

For each implementation slice, begin from the next missing **experienced capability**, then descend only as far as required:

```text
experienced Pratibimba capability
        ↓
relevant M′ source contract
        ↓
minimum M-domain/kernel primitive required
        ↓
minimum wider S/S′ embodiment required
        ↓
Ta-Onta carrier only if agentic inhabitation is active
        ↓
current O:I-native owner/body
        ↓
implementation + parity + source/provenance
        ↓
return to the experienced surface
```

The M/S/O:I relation maps remain reasoning and provenance instruments. They do **not** dictate chronological implementation order.

## 7. First acceptance milestone

The first Epi reconstitution milestone is not “M1/M2 formal engines complete”. It is:

```text
O:I desktop launches
  ↓
Epi / Pratibimba mode is a native hosted contribution
  ↓
existing Epi kernel/library is reachable through a typed, tested bridge
  ↓
Nara M4′ daily surface opens
  ↓
user can write into a protected day/episode
  ↓
DAY/NOW + current harmonic/VAK/coordinate context is real, not fixture-only
  ↓
right-side situated agent can co-refer to the current Nara object/context
  ↓
selected text can be highlighted/sent through a governed Action
  ↓
Epii/Anuttara explanation or review can be summoned without source-authority collapse
  ↓
restart preserves the right canonical state and provenance
```

That milestone proves the architecture in the direction the product is actually meant to be lived.

## 8. Guards against another drift

Reject a proposed tranche if it:

- begins from M1/M2 depth merely because those systems are formally rich;
- creates a new Epi desktop before testing the current O:I host;
- creates duplicate generic session/harness/tool/process semantics;
- rebuilds renderer-local QL/VAK/Matheme rules instead of consuming shared contracts;
- exposes raw technical interior as the default Nara UX;
- treats Cosmic as three separate dashboards;
- treats old Theia/Tauri shell technology as more authoritative than its preserved experience/function design;
- treats O:I hosting as transfer of Epi semantic ownership;
- requires every deep subsystem to be complete before the first lived Nara loop works.

The reconstitution should make the existing Epi design **simpler to realise**, because O:I now supplies the generic agency/application cradle that the earlier Epi system had to build around itself.