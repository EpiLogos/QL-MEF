#!/usr/bin/env python3
"""Independent #293 controlled trials of the real ql M2 JSON producer.

The caller supplies an admitted, already-built production executable. This
driver never builds, installs, launches the app, or substitutes a producer.
These finite producer results do not certify device audio, a physical body,
installed UI, or complete #281 acceptance.
"""
import argparse
import copy
import hashlib
import json
import math
import os
from pathlib import Path
import subprocess
import sys
import time

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def require(value, message):
    if not value:
        raise AssertionError(message)

def near(actual, expected):
    require(len(actual) == len(expected), "wrong vector cardinality")
    for a, b in zip(actual, expected):
        require(math.isfinite(a) and math.isclose(a, b, rel_tol=2e-6, abs_tol=1e-6),
                f"actual {a} != expected {b}")

def inspect(frame, request, missing_source=False):
    require(frame["identity"] == request["stamp"]["identity"], "lost event identity")
    condition = frame["condition"]
    require(condition is not None, "missing selected condition")
    require(condition["identity"] == frame["identity"], "detached condition identity")
    drive = frame["vimarsha"]["reading"]
    require(condition["drive"] == drive, "condition substituted Vimarsha producer output")
    require(len(drive["audio_octet_hz"]) == 8, "audio octet cardinality changed")
    require(len(drive["nodal_quartet"]) == 4, "nodal quartet cardinality changed")
    require(all(math.isfinite(x) and x > 0 for x in drive["audio_octet_hz"]),
            "invalid native intended frequency")
    require(all(isinstance(x, dict) for x in drive["nodal_quartet"]),
            "nodal quartet became oscillator frequencies")
    require(condition["source_revision"] == request["_expected_source_revision"],
            "candidate source revision differs from admitted source")
    require(condition["registry_revision"] == request["registry_revision"],
            "condition detached from request registry")
    path = condition["source_path"]
    if missing_source:
        require(path is None, "missing source role inherited an unrelated descendant")
        require(any("No unique source-held" in x for x in condition["gaps"]),
                "missing correspondence was silently presented as source-qualified")
        require(condition["colour"]["source_name"] is None and
                condition["colour"]["linear_rgba"] is None,
                "missing correspondence fabricated a source colour")
        return condition
    require(path is not None, "missing exact descendant source path")
    require(path["maqam_index"] == 3 and path["maqam_name"] == "Kirdan",
            "source descendant replaced by a shallow catalogue label")
    require(path["maqam_coordinate"] == "#2-4.3-0-4", "Kirdan descendant lost")
    require(path["maqam_node_id"] == "ec7c57f12cefdfd8", "wrong Kirdan native identity")
    require(path["role"] == request["condition"]["role"], "source role was flattened")
    expected = {
        "tonic": ("#2-5-5", "6e19f8d6726f28e1", "#2-5-0/1-1",
                  "5ae70b600c50f1fe", "#2-2-2-5-5", "9741de25da067f85", "earth",
                  "393c61f330a58829", "8f12c607bcd1940b",
                  "TONIC_PLANETARY_RESONANCE", 7013, 8286,
                  "4b647bfe6df6ee8b83fdc23ff66797fcc309d5776d402580f03589324045757d",
                  "03e55a2bde54fa06f9b906576a8ea663c3bf825dd2f0916e5e5a089cab5f054f"),
        "dominant": ("#2-5-0/1", "11a998cf5731a05a", "#2-5-0/1-7",
                     "ec1ec94554bf1320", None, None, None,
                     "7c0bddf74eb66825", "e90c590da39a8471",
                     "DOMINANT_PLANETARY_RESONANCE", 7011, 8199,
                     "868d4872039cf03ea1783e07c8c6a7db400efaf03258bc12998bad8730856917",
                     "c7125c1984099d21103319236dffea5f4882f05468087f57aaf7d9ea68e44387"),
    }[request["condition"]["role"]]
    for key, value in zip(("planet_coordinate", "planet_node_id", "chakra_coordinate",
                           "chakra_node_id", "tattva_coordinate", "tattva_node_id",
                           "material_fibre"), expected[:7]):
        require(path[key] == value, "wrong source branch: " + key)
    require(len(path["musical_relations"]) == 1 and
            len(path["planetary_relations"]) == 1, "lost exact source relation path")
    for edge, edge_id, kind, from_ref, to_ref, record, payload in (
        (path["musical_relations"][0], expected[7], expected[9],
         path["maqam_coordinate"], expected[0], expected[10], expected[12]),
        (path["planetary_relations"][0], expected[8], "PLANETARY_RESONANCE",
         expected[0], expected[2], expected[11], expected[13]),
    ):
        require(edge["id"] == edge_id and edge["kind"] == kind and
                edge["from_coordinate"] == from_ref and edge["to_coordinate"] == to_ref and
                edge["record_index"] == record and edge["payload_sha256"] == payload and
                edge["sha256"] == condition["source_revision"],
                "lost or substituted typed source relation/provenance")
    require(any(c["coordinate"] == "#2-4.3-0-4" and c["property"] == "c_1_name" and
                c["literal"] == "Kirdan" and c["pointer"] == "/708/c_1_name"
                for c in path["claims"]), "exact descendant property claim was lost")
    require(all(c["sha256"] == condition["source_revision"] for c in path["claims"]),
            "mixed-revision source claims")
    return condition

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--producer", type=Path, required=True)
    ap.add_argument("--fixture", type=Path, required=True)
    ap.add_argument("--output", type=Path, required=True)
    ap.add_argument("--candidate-revision", required=True)
    ap.add_argument("--source-revision", required=True)
    args = ap.parse_args()
    root = Path(__file__).resolve().parent
    output = args.output.resolve()
    require(output == root or root in output.parents, "output must stay in V boundary")
    require(args.producer.is_file() and os.access(args.producer, os.X_OK),
            "supply an admitted production executable")
    baseline = json.loads(args.fixture.read_text())
    require(baseline["condition"]["maqam_index"] == 3 and
            baseline["condition"]["role"] == "tonic" and
            baseline["condition"]["tuning"] == "retained24_tet",
            "use the declared controlled native Kirdan fixture")
    output.mkdir(parents=True, exist_ok=False)
    receipt = {
        "standing": "independent-controlled-native-producer-only",
        "parent": "https://github.com/EpiLogos/QL-MEF/issues/281",
        "verifier_ticket": "https://github.com/EpiLogos/QL-MEF/issues/293",
        "verifier": "/root/v_independent_293",
        "candidate_revision": args.candidate_revision,
        "producer_path": str(args.producer.resolve()),
        "producer_sha256": digest(args.producer),
        "driver_sha256": digest(__file__),
        "fixture_path": str(args.fixture.resolve()),
        "fixture_sha256": digest(args.fixture),
        "request_registry_revision": baseline["registry_revision"],
        "expected_source_revision": args.source_revision,
        "method": "real compiled M2Request JSON adapter; copied input mutations only",
        "limits": "No device, physical-solver, installed-host, privacy or H verdict.",
        "trials": [],
    }
    failures = 0
    frames = {}

    def run(name, mutate=None, rejection=None, verify=None, missing_source=False):
        nonlocal failures
        request = copy.deepcopy(baseline)
        if mutate:
            mutate(request)
        path = output / (name + ".request.json")
        path.write_text(json.dumps(request, ensure_ascii=False, indent=2) + "\n")
        started = time.monotonic()
        trial = {"name": name, "request_sha256": digest(path)}
        try:
            completed = subprocess.run([str(args.producer.resolve()), str(path)],
                                       cwd=root, capture_output=True, text=True,
                                       timeout=15, check=False)
            duration = time.monotonic() - started
            (output / (name + ".stdout.json")).write_text(completed.stdout)
            (output / (name + ".stderr.txt")).write_text(completed.stderr)
            trial.update({"duration_seconds": duration,
                          "exit_code": completed.returncode,
                          "stdout_sha256": digest(output / (name + ".stdout.json")),
                          "stderr_sha256": digest(output / (name + ".stderr.txt"))})
            if rejection:
                require(completed.returncode != 0, "invalid input was accepted")
                require(rejection in completed.stderr, "wrong explicit refusal")
                require(not completed.stdout.strip(), "failed producer emitted successful frame")
                trial["expected"] = {"refusal_contains": rejection, "successful_frame": False}
            else:
                require(completed.returncode == 0, "valid production request failed")
                frame = json.loads(completed.stdout)
                request["_expected_source_revision"] = args.source_revision
                condition = inspect(frame, request, missing_source)
                if verify:
                    verify(frame, condition)
                frames[name] = frame
                trial["expected"] = "source path, source revision, stamped native drive preserved"
            trial["pass"] = True
        except Exception as error:
            trial.update({"pass": False, "error": str(error),
                          "duration_seconds": time.monotonic() - started})
            if isinstance(error, subprocess.TimeoutExpired):
                for suffix, value in (("stdout.json", error.stdout), ("stderr.txt", error.stderr)):
                    raw = value or b""
                    if isinstance(raw, str):
                        raw = raw.encode()
                    captured = output / (name + "." + suffix)
                    captured.write_bytes(raw)
                    trial[suffix.split(".")[0] + "_sha256"] = digest(captured)
                trial["timeout_seconds"] = 15
            failures += 1
        receipt["trials"].append(trial)

    run("baseline")
    # Later checks depend on this exact baseline; a failed baseline cannot
    # produce a substituted expected frame or a broadened green result.
    if "baseline" not in frames:
        receipt["failed"] = failures
        (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
        return 1
    ref = frames["baseline"]
    run("producer-disconnected", lambda r: r.update(vimarsha=None),
        "joint M2 condition requires active Vimarsha drive")
    run("wrong-face-at-consumer", lambda r: r["vimarsha"].update(lens=6),
        "joint condition and MEF/Vimarsha selection disagree")
    run("lost-selected-mef", lambda r: r.update(mef_conditions=[]),
        "joint condition and MEF/Vimarsha selection disagree")
    run("stale-producer-generation",
        lambda r: r["vimarsha"]["stamp"]["identity"].update(profile_generation=2),
        "cross-event or stale profile generation")
    run("stale-registry", lambda r: r.update(registry_revision="0" * 64),
        "stale registry revision")
    run("source-path-label-injection",
        lambda r: r["condition"].update(source_path={"maqam_coordinate": "#2-4.3"}),
        "unknown field `source_path`")
    def verify_dominant(f, c):
        require(c["source_path"] != ref["condition"]["source_path"],
                "dominant selection retained tonic correspondence")
        require(c["drive"] == ref["condition"]["drive"],
                "correspondence role overwrote active M1/Vimarsha drive")
    run("valid-dominant-branch", lambda r: r["condition"].update(role="dominant"),
        verify=verify_dominant)
    # Index 4 has no unique source-held tonic or dominant edge. A partial M2
    # reading may expose the gap; admission to a source-qualified body/audio
    # consumer must separately refuse this incomplete path.
    run("missing-source-descendant-gap", lambda r: r["condition"].update(maqam_index=4),
        missing_source=True)
    def doubled_carrier(r):
        r["vimarsha"]["harmonic_ratio"] = [3, 1]
    def verify_carrier(f, c):
        near(c["drive"]["audio_octet_hz"],
             [2 * x for x in ref["condition"]["drive"]["audio_octet_hz"]])
        require(c["musical"]["pitches_hz"] == ref["condition"]["musical"]["pitches_hz"],
                "M1 carrier gesture silently rewrote selected tuning")
        require(c["source_path"] == ref["condition"]["source_path"],
                "carrier gesture rewrote correspondence")
    run("carrier-doubled", doubled_carrier, verify=verify_carrier)
    def doubled_tonic(r):
        r["condition"]["tonic_hz"] *= 2
    def verify_tonic(f, c):
        near(c["musical"]["pitches_hz"],
             [2 * x for x in ref["condition"]["musical"]["pitches_hz"]])
        require(c["drive"] == ref["condition"]["drive"],
                "tuning target silently replaced M1/Vimarsha drive")
    run("tuning-target-doubled", doubled_tonic, verify=verify_tonic)
    def prime_reading(r):
        r["vimarsha"]["lens"] = 6
        r["condition"]["active_mef_condition"] = 36
        r["mef_conditions"] = [36]
    def verify_prime(f, c):
        require(c["drive"]["seed"]["lens"] == 6, "prime lens was flattened")
        require(c["active_mef_condition"] == 36, "prime MEF cell was flattened")
        require(c["drive"]["audio_octet_hz"] != ref["condition"]["drive"]["audio_octet_hz"],
                "changed native reading produced label-only drive")
        require(c["source_path"] == ref["condition"]["source_path"],
                "lens re-reading erased source descendant")
    run("legitimate-prime-reading", prime_reading, verify=verify_prime)
    def shadow_phase(r):
        r["tick12"] = 6
        r["degree720"] = 361
    def verify_shadow(f, c):
        require(c["drive"]["seed"]["tick12"] == 6, "opposite phase face was collapsed")
        near(c["drive"]["audio_octet_hz"],
             [x * 2 * 2 ** (1 / 144) for x in ref["condition"]["drive"]["audio_octet_hz"]])
        require(c["source_path"] == ref["condition"]["source_path"],
                "phase movement rewrote exact source descendant")
        require(c["musical"]["pitches_hz"] == ref["condition"]["musical"]["pitches_hz"],
                "phase movement rewrote the selected tuning target")
    run("opposite-phase-face", shadow_phase, verify=verify_shadow)
    def unsupported_tuning(r):
        r["condition"]["tuning"] = "bimba_spelled24_tet"
    def verify_unavailable(f, c):
        require(c["musical"]["pitches_hz"] == [], "unsupported source tuning silently fell back")
        require(any("unsupported" in x for x in c["gaps"]),
                "unsupported tuning lacked explicit gap")
    run("no-silent-tuning-fallback", unsupported_tuning, verify=verify_unavailable)
    receipt["failed"] = failures
    receipt["passed"] = len(receipt["trials"]) - failures
    (output / "receipt.json").write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({"receipt": str(output / "receipt.json"), "passed": receipt["passed"],
                      "failed": failures, "standing": receipt["standing"]}))
    return 1 if failures else 0

if __name__ == "__main__":
    sys.exit(main())
