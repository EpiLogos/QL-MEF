#!/usr/bin/env python3
"""Run a Bimba map migration: @probe/@report read-only; @apply inside one transaction.

  run.py FILE            dry run: every @apply executes in a transaction that is rolled back
  run.py FILE --commit   apply: the same transaction is committed
Counts come from Neo4j's own update statistics.
"""
import json, re, sys, urllib.request
BASE = "http://100.92.62.101:7474/db/neo4j/tx"
def post(url, statements, method="POST"):
    body = json.dumps({"statements": [{"statement": s, "includeStats": True} for s in statements]}).encode()
    req = urllib.request.Request(url, data=body, method=method, headers={"Content-Type": "application/json"})
    reply = json.load(urllib.request.urlopen(req, timeout=300))
    if reply.get("errors"): raise SystemExit(reply["errors"])
    return reply, req
def sections(text):
    out, name, buf = [], None, []
    for line in text.splitlines():
        m = re.match(r"^// @(probe|apply|report)\b(.*)", line)
        if m:
            if name: out.append((name, buf)); buf = []
            name = (m.group(1), m.group(2).strip()); continue
        if name and line.strip() and not line.startswith("//"): buf.append(line)
    if name: out.append((name, buf))
    return [(n, [s.strip().rstrip(";") for s in re.split(r";\s*$", "\n".join(b), flags=re.M) if s.strip()]) for n, b in out]
def main():
    path, commit = sys.argv[1], "--commit" in sys.argv
    secs = sections(open(path).read())
    reply, _ = post(BASE, [])
    tx = reply["commit"].rsplit("/commit", 1)[0]
    totals = {}
    for (kind, label), stmts in secs:
        if kind != "apply": continue
        reply, _ = post(tx, stmts)
        stats = {}
        for r in reply["results"]:
            for k, v in r.get("stats", {}).items():
                if isinstance(v, int) and v: stats[k] = stats.get(k, 0) + v
        print(f"apply {label}: {len(stmts)} statements -> {stats}")
    if commit:
        post(tx + "/commit", []); print("COMMITTED")
    else:
        urllib.request.urlopen(urllib.request.Request(tx, method="DELETE"), timeout=60); print("rolled back (dry run)")
    for (kind, label), stmts in secs:
        if kind in ("probe", "report"):
            for s in stmts:
                reply, _ = post(BASE + "/commit", [s])
                print(f"{kind} {label}:", [d["row"] for d in reply["results"][0]["data"]])
main()
