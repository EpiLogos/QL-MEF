#!/usr/bin/env python3
"""Generate the S/S′ current-state migration (owner ruling, 20 Oct 2026).

The map still held S as the June technical stack (Terminal, Obsidian, Neo4j,
Gateway, PiAgent, Sync) and S′ as each stratum's reflective laws. Since 3 Sep
QL has read S as the six O:I products and S′ as the six Ta-Onta organs
(docs/integrations/epi-logos/epi-ssprime-embodiment.json: the technical strata
are "capability lineage, not current topology"; TA-ONTA-FULL-FIELD-LOCK.md, owner-
ratified 12 Sep). The owner ruled on 20 Oct:

  1. Update the map to that state, with clean property coverage and parity.
  2. Keep S/S′ and M/M′ distinct: M5-2 and M5-4 both link to the S/S′ nodes,
     for different reasons (M5-2 the spine, M5-4 the agents' mediation).
  3. The models (language models, Kev) are S2 AIKit capabilities; the Nara
     and Epii agents use them, not exclusively.
  4. M0′–M5′ belong to M5-3, which is O:I (the cradle and the DLW); the six
     layers are four modes, 3:1 = 3:3: Live holds M1′–M3′, Expressions is M4′,
     Technē is M5′, and M0′ is the graph systems and the planetary continuity
     between local and collective.
  5. Factory depends on Actuation and AIKit, so its mode reads the S
     coordinates in the (0/1/2/3) context frame.

What this writes:
  - S, S0–S5: the six products. S′, S0′–S5′: the six organs. One property set
    across all fourteen, with q_0_ground … q_5_integration; each old value
    kept in c_1_prior_* and s_0_lineage_record.
  - S0-0′ … S5-5′: the organs' 36 first-level positions (lock §2, SP00–SP55),
    old names kept as priors.
  - S0-0 … S5-5: kept as capability lineage and marked so. No product has a
    six-position inner structure on record yet; nothing is invented for it.
  - M5-2 and M5-4: their child-level edges into S/S′ are replaced by edges to
    the same fourteen nodes: M5-2 HAS_COORDINATE_SCOPE (spine), M5-4
    MEDIATES_PROTOCOL_LAYER (agents). Stale stack clauses in their text are
    corrected in place.
  - M5-4.4 and M5-4.5 USE_CAPABILITIES_OF S2.
  - M′, M0′–M5′ and M5-3: the app layers and their modes.

Every node write is guarded on the node's current sync_version and name; the
rollback restores every value read. Read-only against the map (HTTP, READ).
"""
from __future__ import annotations

import hashlib
import json
import re
import subprocess
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
QL = HERE.parent.parent
WORK = Path.home() / "Central" / "Work"
ENDPOINT = "http://100.92.62.101:7474/db/neo4j/query/v2"
STAMP = "2026-10-20"
NAME = "s-sprime-current"
OUT = HERE / f"{STAMP}-{NAME}.cypher"
ROLLBACK = HERE / f"{STAMP}-{NAME}.rollback.cypher"
VERSION = "s-sprime-current-2026-10-20"
STATUS = "s_sprime_current_2026_10_20"
SOURCE = "owner ruling 2026-10-20 (claude-code:agency-ground)"
LOCK = "docs/integrations/epi-logos/TA-ONTA-FULL-FIELD-LOCK.md"
EMBODIMENT = "docs/integrations/epi-logos/epi-ssprime-embodiment.json"
MARK = f"{STAMP}-{NAME}"
O_I = "O:I, the cradle and the DLW (EpiLogos/O-I: desktop/cradle and packages/live-shell)"

# ---------------------------------------------------------------- the products
PRODUCTS = {
    0: dict(
        name="Central", repo="Central", github="EpiLogos/Central", cli="ctrl",
        description="The personal root: a plain-file ground, Control/ for what a person has written about themselves, their agents and their machines and Work/ for their projects, with ctrl as one doorway of named Actions over it for people, scripts and agents.",
        core="The ground every other product addresses: the root World and its projects, their sources, and the human Day and agent NOW.",
        owns=["root World and its projects (Work/<Name> with ProjectCentral)", "source identity, placement and revisions of authored files", "human Day and agent NOW, at root and project scope", "receiving: returned work queued for a person's review", "authored governance and the root wiki", "the World Positions that agents occupy"],
        runtime="ctrl: named Actions that declare their inputs, effects and results; compare-and-swap writes with revision history.",
        crud="authored files, projects, Day and NOW, receiving and review",
        q=["Central stands on ordinary files a person can read and edit with any editor; ctrl is a doorway, not a gate.",
           "What it is: the personal root folder (Control/ and Work/) and ctrl, whose named Actions mean the same thing to a person, a script and an agent.",
           "How it acts: Actions read or change files with compare-and-swap writes and revision history; the day opens and closes, and NOW holds the work in flight.",
           "Its pattern: two registers under one law, the root and each project, both carrying the same user, agents and NOW shape.",
           "Where it sits: S0, the ground the other products address. Actuation's positions, Factory's review queue, AIKit's context and Workcell's seats resolve against Central's World. Its organ affinity is S0′ Khora.",
           "What returns to it: work returns through receiving for a person's review, the day closes, and what a session learned returns to the wiki through the return door."],
    ),
    1: dict(
        name="Actuation", repo="Actuation", github="EpiLogos/Actuation", cli="actuation",
        description="Records which agent holds which position, on whose authority and within what bounds, and the events and results reported for each act, in append-only files. It does not run agents; harnesses, Workcell and AIKit do the running.",
        core="Standing to act: Agent, Agency and authority, stated, checked and recorded.",
        owns=["Agent identity", "Agency: an Agent placed for an act, in a world, with authority and bounds", "authority grants: issued, resolved, revoked", "occupancy of World Positions", "each act's reported events, results and return path"],
        runtime="actuation: authority issue/resolve/revoke, agency actualise, occupancy claim/release/verify, append-only ledgers with receipts.",
        crud="authority grants, agency admissions, occupancy and act records",
        q=["Actuation stands on explicit grants: authority is never implied by saving, preparing or starting something.",
           "What it is: the record of the Agent (an enduring identity), the Agency (that Agent placed for an act) and the Body (the session, process, model and harness that carry the act this time).",
           "How it acts: it admits an Agency only when a grant matches its world binding and bounds, and keeps a ledger of who occupies which World Position.",
           "Its pattern: append-only ledgers whose receipts keep identity, lineage, authority and the return path; a change of body never creates a new Agent.",
           "Where it sits: S1, between Central's positions and the products that run acts (AIKit, Workcell, Factory). Its organ affinity is S1′ Hen.",
           "What returns: each act's events and results, attributed to the Agency that acted."],
    ),
    2: dict(
        name="AIKit", repo="ai-kit", github="EpiLogos/ai-kit", cli="aikit",
        description="Works out, for each project, agent and task, which skills, tools, models and knowledge sources apply, and makes exactly that set available to the agent harness. The models themselves, language models and Kev (the local decision model behind aikit decide), are AIKit capabilities that agents use.",
        core="The powers an agent can use, resolved for the act in hand: capabilities, context and the harness projection.",
        owns=["the capability catalogue: Skills, Methods, scripts, hooks, MCP tools, models and Actions", "context resolution across user, host, project, session, task and client scopes", "projection into harnesses, in immutable generations with rollback", "knowledge: the project map, the semantic wiki, search and resolve", "models and providers, including language models and Kev for decide", "NOW context preparation (Redis, Jev)"],
        runtime="aikit: world status, compose plan/apply/rollback, harness projection, decide (Kev or another elected provider), knowledge and context services.",
        crud="capability composition, harness projections, knowledge indexes and prepared context",
        q=["AIKit stands on the capability catalogue: every power an agent could use, with its authored standing.",
           "What it is: Capability (any usable power), Context (the stack of scopes for one act) and Projection (the harness-specific form of what was resolved).",
           "How it acts: it resolves the effective set for a context, composes it as a new generation, projects it into the harness, and answers typed decide questions through the elected provider (none, Kev on this machine, an endpoint or a hosted model).",
           "Its pattern: scoped composition with rollback; the same capability is resolved differently per scope and is never copied.",
           "Where it sits: S2, the powers layer. The language models and Kev that the Nara and Epii agents use live here, not exclusively, beside every other agent's skills and models. Its organ affinity is S2′ Pleroma.",
           "What returns: usage and invocation evidence for Actuation, knowledge for the wiki, and a record of the context each turn actually received."],
    ),
    3: dict(
        name="Software Factory", repo="Factory", github="EpiLogos/Factory", cli="factory",
        description="Keeps a file-based record of agent-driven software work: what was asked for and why, the runs and attempts made against it, what each produced, the evidence gathered and where a person's judgement was asked for and recorded.",
        core="Commissioned development, from request to evidence and a person's judgement.",
        owns=["Commissions, Projects, Journeys and Runs", "typed workflows of units and barriers", "attempts as immutable records", "custody: which World Position carries which piece of work", "development evidence and telemetry"],
        runtime="factory: development commission, workflow check/compile, attempt prepare/return/receiving, custody, telemetry.",
        crud="commissions, runs, attempts, custody and development evidence",
        q=["Factory stands on the Commission: a recorded request for development with its purpose, frontier, run placement, agent and scope.",
           "What it is: Project, Journey, Run and attempts, held as JSON files with locked, atomic writes.",
           "How it acts: typed workflows whose units declare what they change, their permitted effects, their verification and where their result returns; intent is recorded before work is handed to the owning product.",
           "Its pattern: every retry is a new record, never a replacement; results go to Central's review queue and Factory records the outcome without deciding it.",
           "Where it sits: S3, on S0 Central (positions and review), S1 Actuation (custody and agency) and S2 AIKit (skills and models). Its work reads the S coordinates in the (0/1/2/3) context frame, the frame of the DLW's Factory mode. Its organ affinity is S3′ Chronos.",
           "What returns: evidence and results for a person's judgement."],
    ),
    4: dict(
        name="Workcell", repo="Workcell", github="EpiLogos/Workcell", cli="workcell",
        description="Turns a request for a place to work, such as a workspace at a revision, a shell or a set of services, into a real workspace, process or service on this machine or a connected one, and records what was actually provided.",
        core="Material execution: where an act lands on a machine.",
        owns=["execution demands: required, preferred and optional needs", "discovery of what a machine can offer", "prepared worlds, their receipts and recovery", "the binding graph from requirement to provider, offer and resource", "seats: the machine's persistent development tracks"],
        runtime="workcell: discover, plan, prepare, observe, expose, collect, release, recover; run ledger and receipts.",
        crud="prepared worlds, receipts, run ledgers and seat claims",
        q=["Workcell stands on what a machine can actually offer, discovered rather than assumed.",
           "What it is: execution demands met by providers, kept as a binding graph of requirement, provider, offer and resource.",
           "How it acts: it discovers, plans (stating every preference it had to drop), prepares with a receipt, observes and collects, then releases or preserves, and can recover a world from its receipt.",
           "Its pattern: each bound resource carries a presence state (present, missing, released, suspended, snapshotted, stale).",
           "Where it sits: S4, the material layer: harnesses and agents run in what Workcell provides. Its organ affinity is S4′ Anima.",
           "What returns: receipts, collected outputs, and worlds released or preserved."],
    ),
    5: dict(
        name="Quaternal Logic", repo="Quaternal-Logic", github="EpiLogos/QL-MEF", cli="ql",
        description="Implements the formal structures of Quaternal Logic as typed, versioned operations: a deterministic coordinate kernel and a registry of reading lenses. It holds O:I's reflection facet, a formal way of reading the rest of an agent's world.",
        core="The suite's reflection: the coordinate kernel, its lenses and the grammar the other products use to read their material.",
        owns=["the QL kernel: positions P0-P5, direct and conjugate faces, deterministic operators", "M/M′ semantics and the coordinate registry generated from the Bimba map", "Vāk, Context Frames, harmonics, geometry and refraction", "the Ta-Onta semantic profile", "decision frames: the typed questions agents put"],
        runtime="ql: kernel operators, lens readings, context frames, team space and Technē readings; every result is JSON naming schema, version, operation and input.",
        crud="kernel fixtures, registries and readings generated from the map",
        q=["Quaternal Logic stands on the Bimba map: its registry and every map-derived table are generated from a read-only read of the map, which stays the authority.",
           "What it is: six positions, P0-P5, each with a direct and a conjugate face, addressed and operated deterministically, with lenses that read material through them.",
           "How it acts: conjugate, complement and four-plus-two classification are kernel operators; unsupported operations fail explicitly and no stochastic operator is advertised.",
           "Its pattern: the M/S/S′/M′ fourfold, Vāk and the Context Frames, as the grammar other products use to read their own material.",
           "Where it sits: S5, the reflection of the suite. It defines the questions agents put (decision frames); the models that answer them run in AIKit. Its organ affinity is S5′ Aletheia.",
           "What returns: settled readings that renew the ground; a gap is named as a question rather than filled."],
    ),
}

# ------------------------------------------------------------------ the organs
ORGANS = {
    0: dict(name="Khora", participation="Central root meta-project and sources; AIKit context, SessionSpace and body resolution; Actuation Agent and Agency; Workcell material facts; O:I entry and restore.",
            q2="Bootstrap, actual material availability, identity and session continuity, interruption, compaction and exit, and safe entry of sources and tools."),
    1: dict(name="Hen", participation="Central source and document operations; AIKit Wiki, OKF, knowledge and constellation machinery; QL form and property law.",
            q2="Open human source, declared artifact forms, QL wholes, pairs, triads, sixfolds and direct-conjugate Wiki constellations, entity capture and reviewed graduation, source-before-index lineage, and thought and Return artifacts."),
    2: dict(name="Pleroma", participation="AIKit resource, capability and praxis resolution; native product Actions; Actuation authority; Workcell execution.",
            q2="Discovery of models, harnesses, providers and components; native tools, Skills, Methods and SkillSets; instruments and Surface access; execution modes, contributed faculties, declared fallback and permission envelopes. Skills say when, why and how a power is useful; native owners enforce permission and execute it."),
    3: dict(name="Chronos", participation="Central's Day and NOW; AIKit temporal context; M1, M2 (Kerykeion), M3 and M4 domain times; Factory run history; O:I shared transport.",
            q2="Lifecycle triggers, archive and re-entry, original versus reinterpreted occasion, source freshness, Kairos, decan and day-arc readings, scheduling of confirmed work and shared-world time."),
    4: dict(name="Anima", participation="QL semantics; AIKit resolution, Skills and Methods; Actuation agency; Factory commissioned development; Workcell material execution; native surface actions.",
            q2="Anima governs the composed act through C′ Vāk, with Nous, Logos, Eros, Mythos, Psyche and Sophia as its members: Nous opens ground, Logos distributes usable form, Eros performs according to need, Mythos recognises organising pattern, Psyche keeps contextual continuity, Sophia integrates toward renewed ground."),
    5: dict(name="Aletheia", participation="Native Activity, Evidence and Return; Central receiving; AIKit knowledge, history and praxis; Factory development evidence; M5′ Epii's evaluation and pedagogy.",
            q2="Knowledge ingestion and query, thought routing, enrichment, episodic retrieval, alignment and review, with its specialists: Anansi investigates gaps, Janus relates temporal occasions, Moirai supplies evidence, query and reflection modes, Mercurius translates across domains, Agora brings plural contributions into evaluation, Zeithoven develops the next score."),
}

# ------------------------------------------------------------- the M′ layers
M_PRIME = {
    0: dict(m="M0 Anuttara", layer="Graph systems and planetary continuity", mode="M0′: graph systems and the planetary continuity between local and collective",
            description="The app layer of M0 Anuttara: graph systems (the Bimba map and the knowledge graphs drawn as ground) and the planetary continuity between local and collective, a person's own World joined to the shared field."),
    1: dict(m="M1 Paramaśiva", layer="The torus: generation", mode="Live",
            description="The app layer of M1 Paramaśiva: generation. The torus is the only source of sound; Live plays it with M2′ and M3′."),
    2: dict(m="M2 Paraśakti", layer="The synth and the sky: modulation", mode="Live",
            description="The app layer of M2 Paraśakti: modulation. The synth, moved by the live sky and tuned by the lenses; part of Live."),
    3: dict(m="M3 Mahāmāyā", layer="The codon clock and the deck: form and time", mode="Live",
            description="The app layer of M3 Mahāmāyā: form and time. The codons, the clock and the deck; part of Live."),
    4: dict(m="M4 Nara", layer="Expressions", mode="Expressions",
            description="The app layer of M4 Nara: Expressions, the lived application, where a scene holds M1′–M3′ with a person's own occasion."),
    5: dict(m="M5 Epii", layer="Technē", mode="Technē",
            description="The app layer of M5 Epii: Technē, the deep-working application of knowledge, construction and review. It contains the Backend Studio (M5-2′), the Frontend Studio (M5-3′) and the Agentic Control Room (M5-4′)."),
}
MODES = {
    "m_prime_modes": "The six M′ layers are four modes, 3:1 = 3:3: Live holds M1′, M2′ and M3′; Expressions is M4′; Technē is M5′; M0′ is the graph systems and the planetary continuity between local and collective.",
    "s_frame_modes": {"Central": "S0", "Factory": "S(0/1/2/3): Factory depends on S0 Central, S1 Actuation and S2 AIKit"},
}

SPARE_ROOT = {"S": "S Product Stack", "S'": "S' Ta-Onta Agent World"}


def query(statement: str) -> list:
    body = json.dumps({"statement": statement, "accessMode": "READ"}).encode()
    req = urllib.request.Request(ENDPOINT, data=body, headers={"Content-Type": "application/json"})
    reply = json.load(urllib.request.urlopen(req, timeout=120))
    if reply.get("errors"):
        raise SystemExit(reply["errors"])
    data = reply["data"]
    return [dict(zip(data["fields"], row)) for row in data["values"]]


def lit(v) -> str:
    if isinstance(v, dict):
        return "{" + ", ".join(f"{k}: {lit(x)}" for k, x in v.items()) + "}"
    return json.dumps(v, ensure_ascii=False)


def git_show(repo: str, path: str) -> tuple[str, bytes]:
    rev = subprocess.check_output(["git", "-C", str(WORK / repo), "rev-parse", "--short=9", "origin/main"], text=True).strip()
    data = subprocess.check_output(["git", "-C", str(WORK / repo), "show", f"origin/main:{path}"])
    return rev, data


def sp_rows() -> dict:
    text = (QL / LOCK).read_text(encoding="utf-8")
    sec = text[text.index("## 2. Every recovered first-level position"):text.index("## 3. Root-level")]
    out = {}
    for line in sec.splitlines():
        if not re.match(r"^\| SP\d\d ", line):
            continue
        sp, hist, bind, conf = [c.strip() for c in line.strip().strip("|").split("|")]
        m = re.match(r"historical:S(\d)-(\d)′ — ([^.]+)\. (.*)", hist)
        i, j = int(m.group(1)), int(m.group(2))
        coord = f"S{i}.{j}'" if i == 4 else f"S{i}-{j}'"
        out[coord] = dict(sp=sp, locator=f"historical:S{i}-{j}′", title=m.group(3), function=m.group(4), binding=bind, conformance=conf)
    assert len(out) == 36, len(out)
    return out


def main() -> None:
    nodes = {r["c"]: r["p"] for r in query(
        "MATCH (n:Bimba) WHERE n.coordinate =~ '^S.*' OR n.coordinate IN ['M5-2','M5-3','M5-4',\"M'\",\"M0'\",\"M1'\",\"M2'\",\"M3'\",\"M4'\",\"M5'\"] "
        "RETURN n.coordinate AS c, properties(n) AS p")}
    for p in nodes.values():  # never written: SET += touches only the keys it names
        for k in [k for k in p if "embedding" in k]:
            del p[k]
    edges = query(
        "MATCH (a:Bimba)-[r]->(b:Bimba) WHERE a.coordinate IN ['M5-2','M5-4'] AND b.coordinate =~ '^S.*' "
        "RETURN a.coordinate AS a, type(r) AS t, b.coordinate AS b, properties(r) AS p ORDER BY a, t, b")
    uses = query("MATCH (a:Bimba)-[r:USES_CAPABILITIES_OF]->(b:Bimba) RETURN count(r) AS n")[0]["n"]
    assert uses == 0, "USES_CAPABILITIES_OF already present"
    sp = sp_rows()
    lock_rev, lock_bytes = git_show("Quaternal-Logic", LOCK)

    fwd = [f"// S/S′ current state (owner ruling, {STAMP}). Generated by migration/bimba-map/generate_s_sprime_current.py; do not hand-edit.", ""]
    back = [f"// Rollback of {OUT.name}: restores every value read and the replaced edges.", ""]
    fwd += ["// @probe before",
            "MATCH (n:Bimba) WHERE n.coordinate IN ['S','S0','S1','S2','S3','S4','S5',\"S'\",\"S0'\",\"S1'\",\"S2'\",\"S3'\",\"S4'\",\"S5'\"] RETURN n.coordinate, n.c_1_name, n.sync_version ORDER BY n.coordinate;",
            "MATCH (a:Bimba)-[r]->(b:Bimba) WHERE a.coordinate IN ['M5-2','M5-4'] AND b.coordinate =~ '^S.*' RETURN a.coordinate, type(r), count(r);", ""]

    def node_write(coord: str, new: dict, drop: list[str], label: str) -> None:
        old = nodes[coord]
        guard = f"n.sync_version = {lit(old.get('sync_version'))} AND n.c_1_name = {lit(old.get('c_1_name'))}" if old.get("sync_version") is not None else f"n.c_1_name = {lit(old.get('c_1_name'))}"
        drop = [k for k in drop if k in old and k not in new]
        stmt = f"MATCH (n:Bimba {{coordinate:{lit(coord)}}}) WHERE {guard} SET n += {lit(new)}"
        if drop:
            stmt += " REMOVE " + ", ".join(f"n.{k}" for k in drop)
        fwd_sections.setdefault(label, []).append(stmt + ";")
        restore = {k: old[k] for k in list(new) + drop if k in old}
        added = [k for k in new if k not in old]
        r = f"MATCH (n:Bimba {{coordinate:{lit(coord)}}})"
        if restore:
            r += f" SET n += {lit(restore)}"
        if added:
            r += " REMOVE " + ", ".join(f"n.{k}" for k in added)
        back_sections.setdefault(label, []).append(r + ";")

    fwd_sections: dict[str, list] = {}
    back_sections: dict[str, list] = {}
    q_keys = ["q_0_ground", "q_1_definition", "q_2_operation", "q_3_pattern", "q_4_context", "q_5_integration"]

    def lineage_record(old: dict) -> str:
        keep = {k: v for k, v in old.items() if k == "c_1_name" or k == "c_1_description" or k == "c_0_core_nature" or k.startswith("q_") or k.startswith("s_4_") or k in ("c_1_structure", "vault_path", "s_5_runtime_use")}
        return json.dumps(keep, ensure_ascii=False, sort_keys=True)

    def stale_keys(old: dict) -> list[str]:
        return [k for k in old if (k.startswith("q_") and k not in q_keys) or k in ("s_4_input_contracts", "s_4_output_contracts", "s_4_queryable_properties", "s_4_translation_schema", "s_4_s5_agentic_protocol_owner")]

    # ---- the products, S0–S5
    for i, p in PRODUCTS.items():
        rev, readme = git_show(p["repo"], "README.md")
        old = nodes[f"S{i}"]
        new = {
            "c_1_name": p["name"], "c_1_description": p["description"], "c_0_core_nature": p["core"],
            "c_1_prior_name": old["c_1_name"], "c_1_prior_description": old["c_1_description"],
            "c_1_form": f"Work/{p['repo']}/README.md", "vault_path": f"Work/{p['repo']}/README.md",
            "c_1_structure": json.dumps({"authority_spec": f"{p['github']}:README.md@{rev}", "layer_name": "product", "promotion_contract": VERSION}),
            "artifact_kind": "repository_document", "content_hash": "sha256:" + hashlib.sha256(readme).hexdigest(),
            "promotion_source": SOURCE, "s_4_function_role": f"Direct S{i} product", "s_5_runtime_use": p["runtime"], "s_2_crud_scope": p["crud"],
            "s_3_repository": p["github"], "s_3_checkout": f"Work/{p['repo']}", "s_3_cli": p["cli"], "s_3_source_revision": rev,
            "s_3_owns": p["owns"], "s_3_organ_affinity": f"S{i}′ {ORGANS[i]['name']} (structural affinity, not exclusive ownership)",
            "s_0_lineage_record": lineage_record(old), "s_0_lineage": f"Until {STAMP} this node was the technical stratum {old['c_1_name']} ({EMBODIMENT}: capability lineage, not current topology).",
            "sync_status": STATUS, "sync_version": VERSION,
            **dict(zip(q_keys, p["q"])),
        }
        node_write(f"S{i}", new, stale_keys(old), "products")

    # ---- the organs, S0′–S5′
    for i, o in ORGANS.items():
        coord = f"S{i}'"
        old = nodes[coord]
        positions = [f"{v['sp']} {v['title']}" for k, v in sorted(sp.items()) if v["sp"].startswith(f"SP{i}")]
        purpose = re.search(rf"\| S{i}′ {o['name']} \| ([^|]+) \|", (QL / LOCK).read_text(encoding="utf-8")).group(1).strip()
        new = {
            "c_1_name": o["name"], "c_1_description": purpose, "c_0_core_nature": f"Ta-Onta organ S{i}′: {purpose}",
            "c_1_prior_name": old["c_1_name"], "c_1_prior_description": old["c_1_description"],
            "c_1_form": LOCK, "vault_path": LOCK,
            "c_1_structure": json.dumps({"authority_spec": f"EpiLogos/QL-MEF:{LOCK}@{lock_rev} §1-§3", "layer_name": "organ", "promotion_contract": VERSION}),
            "artifact_kind": "repository_document", "content_hash": "sha256:" + hashlib.sha256(lock_bytes).hexdigest(),
            "promotion_source": SOURCE, "s_4_function_role": f"Ta-Onta organ S{i}′", "s_5_runtime_use": o["participation"],
            "s_2_crud_scope": "agent-world composition over native owners; the organ writes nothing of its own",
            "s_3_repository": "EpiLogos/QL-MEF", "s_3_checkout": "Work/Quaternal-Logic", "s_3_cli": "ql", "s_3_source_revision": lock_rev,
            "s_3_owns": positions, "s_3_organ_affinity": f"S{i} {PRODUCTS[i]['name']} (structural affinity, not exclusive ownership)",
            "s_0_lineage_record": lineage_record(old), "s_0_lineage": f"Until {STAMP} this node was the reflective stratum {old['c_1_name']}; its six children are the organ's recovered positions ({LOCK} §2).",
            "sync_status": STATUS, "sync_version": VERSION,
            "q_0_ground": f"{o['name']} stands on the native owners it specialises: {o['participation']}",
            "q_1_definition": f"What it is: {purpose}",
            "q_2_operation": f"How it acts: {o['q2']}",
            "q_3_pattern": "Its pattern: six first-level positions recovered from the historical strata, " + "; ".join(positions) + f" ({LOCK} §2).",
            "q_4_context": f"Where it sits: S{i}′ of Ta-Onta, the Epi agent world over the products. Same-index affinity with S{i} {PRODUCTS[i]['name']} is structural, not ownership. M5-2 holds it in Epii's spine; M5-4 is where the agents act through it.",
            "q_5_integration": "What returns: Ta-Onta specialises the products' open capabilities for Epi and reproduces none of their storage, parsers, workflow engines, harnesses or desktop; what an act yields returns through the native owners.",
        }
        node_write(coord, new, stale_keys(old), "organs")

    # ---- the family roots
    roots = {
        "S": dict(name=SPARE_ROOT["S"], description="The six O:I products that make Epi-Logos technically real: S0 Central, S1 Actuation, S2 AIKit, S3 Software Factory, S4 Workcell and S5 Quaternal Logic. Each owns its domain; capabilities are composed across them.",
                  layer="family-root", authority=f"EpiLogos/QL-MEF:{EMBODIMENT}", role="Product coordinate namespace root",
                  q=["S stands on the products' own repositories: each product is a native owner with its own CLI, records and tests.",
                     "What it is: the six products, Central, Actuation, AIKit, Software Factory, Workcell and Quaternal Logic.",
                     "How it acts: a capability is made technically real by the products that own it, composed across them, never re-implemented by another.",
                     "Its pattern: S0 the ground, S1 standing to act, S2 the powers, S3 commissioned work, S4 material execution, S5 reflection.",
                     "Where it sits: beside M and M′, distinct from them. M5-2 holds S in Epii's spine; M5-4 is where the agents act through it. Its prime is S′, Ta-Onta.",
                     "What returns: evidence and results return to their owners, and through Central's receiving to a person."]),
        "S'": dict(name=SPARE_ROOT["S'"], description="Ta-Onta, the Epi agent world: six organs that specialise the products' open capabilities for Epi: S0′ Khora, S1′ Hen, S2′ Pleroma, S3′ Chronos, S4′ Anima and S5′ Aletheia.",
                   layer="family-root", authority=f"EpiLogos/QL-MEF:{LOCK}", role="Ta-Onta organ namespace root",
                   q=["S′ stands on S: every organ consumes the products' native capabilities and owns no storage of its own.",
                      "What it is: the six organs of the Epi agent world, Khora, Hen, Pleroma, Chronos, Anima and Aletheia.",
                      "How it acts: an act is entered (Khora), given form (Hen), given its powers (Pleroma), placed in its times (Chronos), composed (Anima) and disclosed and returned (Aletheia).",
                      "Its pattern: six organs of six recovered positions each (SP00–SP55); same-index affinity with the products is structural, not ownership.",
                      "Where it sits: the prime of S, distinct from M and M′. M5-2 holds S′ in Epii's spine; M5-4, with the six agents at M5-4.0–M5-4.5, is where the agents act through it.",
                      "What returns: what actually happened, compared with intent and turned into knowledge, learning and Recognition."]),
    }
    for coord, r in roots.items():
        old = nodes[coord]
        doc = r["authority"].split(":", 1)[1]
        new = {
            "c_1_name": r["name"], "c_1_description": r["description"], "c_0_core_nature": r["description"],
            "c_1_prior_name": old["c_1_name"], "c_1_prior_description": old["c_1_description"],
            "c_1_form": doc, "vault_path": doc, "artifact_kind": "repository_document",
            "content_hash": "sha256:" + hashlib.sha256(git_show("Quaternal-Logic", doc)[1]).hexdigest(),
            "c_1_structure": json.dumps({"authority_spec": f"{r['authority']}@{lock_rev}", "layer_name": r["layer"], "promotion_contract": VERSION}),
            "promotion_source": SOURCE, "s_4_function_role": r["role"],
            "s_0_lineage_record": lineage_record(old), "s_0_lineage": f"Until {STAMP} this root named the technical stack ({EMBODIMENT}).",
            "sync_status": STATUS, "sync_version": VERSION,
            **dict(zip(q_keys, r["q"])),
        }
        node_write(coord, new, stale_keys(old), "roots")

    # ---- the organ positions, S0-0′ … S5-5′
    for coord, v in sorted(sp.items()):
        old = nodes[coord]
        new = {
            "c_1_name": v["title"], "c_1_description": v["function"],
            "c_1_prior_name": old["c_1_name"], "c_1_prior_description": old["c_1_description"],
            "s_5_runtime_use": v["binding"], "s_3_conformance": v["conformance"], "s_3_position_ref": v["sp"],
            "s_0_lineage": f"{v['locator']} ({LOCK} §2)", "promotion_source": SOURCE,
            "sync_status": STATUS, "sync_version": VERSION,
        }
        node_write(coord, new, ["s_4_s5_agentic_protocol_owner"], "organ positions")

    # ---- the historical strata, S0-0 … S5-5: kept as lineage
    for coord in sorted(c for c in nodes if re.match(r"^S\d[-.]\d$", c)):
        old = nodes[coord]
        parent = nodes[old["coordinate_parent"]]
        new = {
            "s_0_standing": "capability lineage",
            "s_0_lineage": f"Historical technical stratum of {parent['c_1_name']} ({old['coordinate_parent']}, June 2026). Superseded as topology on 2026-09-03 ({EMBODIMENT}); kept for its recovered capability. It is not a position of the product now at {old['coordinate_parent']}, whose six positions are not yet on record.",
            "sync_status": STATUS, "sync_version": VERSION,
        }
        node_write(coord, new, ["s_4_s5_agentic_protocol_owner"], "lineage")

    # ---- M5-2, M5-4: text and scope
    def replaced(coord: str, key: str, pairs: list[tuple[str, str]]) -> str:
        text = nodes[coord][key]
        for a, b in pairs:
            assert text.count(a) == 1, (coord, key, a)
            text = text.replace(a, b)
        return text

    node_write("M5-2", {
        "q_1_system_spine_lattice": replaced("M5-2", "q_1_system_spine_lattice", [("the trilaminar technological matrix — S0 ground through S5 world-boundary plus their reflective primes — the databases, knowledge-graph infrastructure, gateway surfaces, services, schema-law, and code-provenance that constitute the material body of the Epi-Logos system", "the six products (S0 Central, S1 Actuation, S2 AIKit, S3 Software Factory, S4 Workcell, S5 Quaternal Logic) and the six Ta-Onta organs of S′ (Khora, Hen, Pleroma, Chronos, Anima, Aletheia), with the databases, knowledge graph, services, schema law and code provenance that constitute the material body of the Epi-Logos system")]),
        "q_3_s_sprime_nodal_structure": replaced("M5-2", "q_3_s_sprime_nodal_structure", [("S0 terminal/CLI ground, S1 vault/material container, S2 graph substrate, S3 gateway control-plane, S4 agent runtime, S5 world-boundary — each with its reflective prime, each carrying runtime properties (repo paths, CRUD owners, gateway methods, tests, provenance)", "S0 Central, S1 Actuation, S2 AIKit, S3 Software Factory, S4 Workcell, S5 Quaternal Logic, with the Ta-Onta organs as their primes, each carrying runtime properties (repositories, owners, commands, tests, provenance)")]),
        "s_2_coordinate_scope": "S/S′: the six products and the six Ta-Onta organs, held as Epii's system spine",
        "promotion_source": SOURCE, "sync_status": STATUS, "sync_version": VERSION,
    }, [], "M5")
    node_write("M5-4", {
        "q_1_agentic_protocol_seam": replaced("M5-4", "q_1_agentic_protocol_seam", [("Scope is S4/S4' agent-inhabitation plus S5/S5' world-return and improvement protocols (s_4_s5_coordinate_scope; prefixes S4, S4', S5, S5')", "Scope is the whole S/S′ field, the six products and the six Ta-Onta organs, as the agents act through them (s_4_s5_coordinate_scope; prefixes S, S′)")]),
        "s_4_s5_coordinate_scope": "S/S′: the six products and the six Ta-Onta organs, as the agents act through them",
        "t_4_protocol_scope": "S/S′ (products and organs), M′ agentic affordances, operational capacities",
        "c_1_coordinate_role": "System agentic protocols: mediation of S/S′ (products and organs), M′, review, runtime agency and operational capacities.",
        "c_1_prior_coordinate_role": nodes["M5-4"]["c_1_coordinate_role"],
        "s_2_query_prefixes": ["S", "S'"],
        "promotion_source": SOURCE, "sync_status": STATUS, "sync_version": VERSION,
    }, [], "M5")

    # ---- M5-3, M′, M0′–M5′: the app and its layers
    node_write("M5-3", {
        "c_1_coordinate_role": "System expression: Epii's direct M view into the full M′ field as O:I renders it, the cradle and the DLW.",
        "c_1_prior_coordinate_role": nodes["M5-3"]["c_1_coordinate_role"],
        "q_1_mprime_expression_plane": replaced("M5-3", "q_1_mprime_expression_plane", [
            ("the full M' rendered Pratibimba/Theia coordinate field", "the full M' rendered Pratibimba coordinate field as O:I renders it (the cradle and the DLW)"),
            ("M0' graph surface, M1' played torus, M2' cymatic/MEF expression, M3' wheel/codec, M4' Nara field, M5' Epii IDE", "M0' graph systems and planetary continuity, M1' the torus, M2' the synth and the sky, M3' the codon clock and the deck, M4' Expressions, M5' Technē")]),
        "q_3_rendered_pratibimba_lattice": replaced("M5-3", "q_3_rendered_pratibimba_lattice", [
            ("the complete coordinate field of Theia/Electron surfaces", "the complete coordinate field of O:I's surfaces (the cradle and the DLW)"),
            ("plus Body/M/epi-theia extension nodes, diagrams, panels, widgets, UI routes, and kernel-bridge consumers", "plus O:I's devices, panes, widgets, routes and kernel-bridge consumers")]),
        "m_3_application": O_I,
        "m_3_modes": json.dumps(MODES, ensure_ascii=False),
        "promotion_source": SOURCE, "sync_status": STATUS, "sync_version": VERSION,
    }, [], "M5")
    node_write("M'", {
        "c_1_description": "Rendered, playable Pratibimba expression of the M/Bimba map across M0'-M5' in O:I (the cradle and the DLW).",
        "c_1_prior_description": nodes["M'"]["c_1_description"],
        "m_0_repo_path": "Work/O-I (desktop/cradle, packages/live-shell)",
        "m_0_prior_repo_path": nodes["M'"]["m_0_repo_path"],
        "m_3_rendered_affordance": "O:I expression plane: the cradle and the DLW",
        "m_3_prior_rendered_affordance": nodes["M'"]["m_3_rendered_affordance"],
        "m_3_modes": json.dumps(MODES, ensure_ascii=False),
        "promotion_source": SOURCE, "sync_status": STATUS, "sync_version": VERSION,
    }, [], "M prime")
    for i, m in M_PRIME.items():
        node_write(f"M{i}'", {
            "c_1_description": m["description"], "c_1_coordinate_role": f"App layer of {m['m']}",
            "m_2_parent_subsystem": f"M{i}", "m_3_application_layer": m["layer"], "m_3_mode": m["mode"], "m_3_application": O_I,
            "promotion_source": SOURCE, "sync_status": STATUS, "sync_version": VERSION,
        }, [], "M prime")

    # ---- edges
    fourteen = ["S", "S'"] + [f"S{i}" for i in range(6)] + [f"S{i}'" for i in range(6)]
    keep = {("M5-2", "GOVERNS_COORDINATE_FAMILY", "S"), ("M5-2", "GOVERNS_COORDINATE_FAMILY", "S'"),
            ("M5-2", "HAS_COORDINATE_SCOPE", "S"), ("M5-2", "HAS_COORDINATE_SCOPE", "S'")}
    drop = [e for e in edges if (e["a"], e["t"], e["b"]) not in keep]
    edge_lines, edge_back = [], []
    for e in drop:
        edge_lines.append(f"MATCH (:Bimba {{coordinate:{lit(e['a'])}}})-[r:{e['t']}]->(:Bimba {{coordinate:{lit(e['b'])}}}) DELETE r;")
        edge_back.append(f"MATCH (a:Bimba {{coordinate:{lit(e['a'])}}}), (b:Bimba {{coordinate:{lit(e['b'])}}}) CREATE (a)-[:{e['t']} {lit(e['p'])}]->(b);")
    spine = "Epii's system spine: every product and organ held as coordinate structure"
    agents = "agentic mediation: the agents act through the products and organs"
    for b in fourteen:
        if ("M5-2", "HAS_COORDINATE_SCOPE", b) not in keep:
            edge_lines.append(f"MATCH (a:Bimba {{coordinate:'M5-2'}}), (b:Bimba {{coordinate:{lit(b)}}}) CREATE (a)-[:HAS_COORDINATE_SCOPE {{reason:{lit(spine)}, c_3_migration:{lit(MARK)}}}]->(b);")
        edge_lines.append(f"MATCH (a:Bimba {{coordinate:'M5-4'}}), (b:Bimba {{coordinate:{lit(b)}}}) CREATE (a)-[:MEDIATES_PROTOCOL_LAYER {{reason:{lit(agents)}, c_3_migration:{lit(MARK)}}}]->(b);")
    for a, uses_what in [("M5-4.4", "the Nara agent's voice uses AIKit's language models; it may use Kev or any other AIKit capability"),
                         ("M5-4.5", "the Epii agent's scoring and synthesis use AIKit's Kev and language models; it may use any other AIKit capability")]:
        edge_lines.append(f"MATCH (a:Bimba {{coordinate:{lit(a)}}}), (b:Bimba {{coordinate:'S2'}}) CREATE (a)-[:USES_CAPABILITIES_OF {{what:{lit(uses_what)}, exclusive:false, c_3_migration:{lit(MARK)}}}]->(b);")
    edge_back.insert(0, f"MATCH ()-[r]->() WHERE r.c_3_migration = {lit(MARK)} DELETE r;")

    for label, stmts in fwd_sections.items():
        fwd += [f"// @apply {label}", *stmts, ""]
    fwd += ["// @apply edges", *edge_lines, ""]
    fwd += ["// @report after",
            "MATCH (n:Bimba) WHERE n.coordinate IN ['S','S0','S1','S2','S3','S4','S5',\"S'\",\"S0'\",\"S1'\",\"S2'\",\"S3'\",\"S4'\",\"S5'\"] RETURN n.coordinate, n.c_1_name, n.sync_version ORDER BY n.coordinate;",
            f"MATCH (n:Bimba) WHERE n.sync_version = {lit(VERSION)} RETURN count(n);",
            "MATCH (a:Bimba)-[r]->(b:Bimba) WHERE a.coordinate IN ['M5-2','M5-4'] AND b.coordinate =~ '^S.*' RETURN a.coordinate, type(r), count(r);",
            f"MATCH ()-[r]->() WHERE r.c_3_migration = {lit(MARK)} RETURN type(r), count(r);", ""]
    back += ["// @apply restore", *edge_back]
    for label, stmts in back_sections.items():
        back += stmts
    back.append("")
    OUT.write_text("\n".join(fwd), encoding="utf-8")
    ROLLBACK.write_text("\n".join(back), encoding="utf-8")
    print(f"{OUT.name}: {sum(len(s) for s in fwd_sections.values())} node writes, {len(drop)} edges replaced, {len(edge_lines) - len(drop)} edges created")


if __name__ == "__main__":
    main()
