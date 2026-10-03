#!/usr/bin/env python3
"""Qualify selected original worker exchanges and actual stderr; runs no worker.

No missing, interrupted, truncated or non-Apple row becomes an SDK statistic.
Unselected FIELD totals are fixed source-owned Rust/native frontier witnesses,
not a retained full transcript or independent replay of discarded exchanges.
"""
import argparse
import hashlib
import json
from pathlib import Path

MAX_MESSAGE = 32 * 1024 * 1024
STDERR_BOUND = 8192
FIELD_KEYS = ("field_sample_count", "field_mode_count", "field_shape_vector_count",
              "field_samples_elapsed", "field_generation")


def bounded(path, bound):
    if path.is_symlink() or not path.is_file():
        raise ValueError("original regular evidence file required")
    with path.open("rb") as stream:
        data = stream.read(bound + 1)
    if len(data) > bound:
        raise ValueError("original evidence exceeds finite custody bound")
    return data


def load(path):
    return json.loads(bounded(path, 8192))


def integer(value):
    if type(value) is not int or not 0 <= value <= 2**64 - 1:
        raise ValueError("actual bounded unsigned integer required")
    return value


def decimal(value):
    if not isinstance(value, str) or not value or not value.isascii():
        raise ValueError("canonical native decimal text required")
    n = int(value)
    if str(n) != value or not 0 <= n <= 2**64 - 1:
        raise ValueError("noncanonical native counter")
    return n


def sha(data):
    return "sha256:" + hashlib.sha256(data).hexdigest()


def qualify(directory, require_apple=False):
    opened = load(directory / "opened.json")
    if opened["schema"] != "ql.native-worker-malloc-custody/v1" or opened["phase"] != "actual-worker-opened":
        raise ValueError("actual worker opening required")
    pid = integer(opened["worker_pid"])
    if pid == 0 or integer(opened["host_pid"]) == 0:
        raise ValueError("actual native host/worker PID required")
    selected = [decimal(n) for n in opened["selected"]]
    if not 1 <= len(selected) <= 3 or selected != sorted(set(selected)) or selected[0] == 0:
        raise ValueError("one to three strictly increasing actual request ordinals")
    if opened["request_reply_bound_bytes"] != MAX_MESSAGE or opened["stderr_bound_bytes"] != STDERR_BOUND:
        raise ValueError("changed native transport/custody bounds")
    if integer(opened["writer_job_bound"]) != 12 \
            or integer(opened["selected_raw_capacity_bound_bytes"]) != 3 * (2 * MAX_MESSAGE + 1) \
            or integer(opened["stderr_part_capacity_bound_bytes"]) != 9 * 2048 \
            or opened["delivery_order"] != "actual-result-channel-before-selected-byte-handoff":
        raise ValueError("changed bounded off-transport writer contract")
    platform = opened["counter_platform"]
    if platform not in ("Apple-SDK-observation-required", "unavailable-non-Apple"):
        raise ValueError("unknown native statistics platform")
    if require_apple and platform != "Apple-SDK-observation-required":
        raise ValueError("actual Apple SDK observations unavailable")
    closed = load(directory / "closed.json")
    if closed["schema"] != "ql.native-worker-malloc-custody-close/v1" or closed["worker_pid"] != pid             or closed["phase"] not in ("normal-owner-drop", "poisoned-owner-drop")             or closed["owned_child_waited"] is not True or closed["evidence_write_failed"] is not False:
        raise ValueError("incomplete/failed actual worker lifecycle custody")
    if closed["lost_custody"] is not False or closed["writer_completed"] is not True \
            or closed["selected_pairs_queued"] != [True] * len(selected) \
            or any(type(x) is not bool for x in closed["selected_pairs_queued"]):
        raise ValueError("incomplete or lost off-transport selected byte handoff")
    if closed["submitted"] != [True] * len(selected) or any(type(x) is not bool for x in closed["submitted"]):
        raise ValueError("selected request not actually submitted")
    if not isinstance(closed["actual_exit_success"], bool):
        raise ValueError("actual child wait status missing")
    for key in ("actual_exit_code", "actual_exit_signal"):
        value = closed[key]
        if value is not None and (type(value) is not int or value < 0):
            raise ValueError("invalid actual child exit evidence")
    stderr = bounded(directory / "worker-stderr.bin", STDERR_BOUND)
    status = load(directory / "stderr-status.json")
    if status["schema"] != "ql.native-worker-malloc-stderr-custody/v1" or status["worker_pid"] != pid             or status["eof_observed"] is not True or status["truncated"] is not False             or status["counter_overflow"] is not False or status["file_write_failed"] is not False             or integer(status["retained_bytes"]) != len(stderr) or decimal(status["observed_bytes"]) != len(stderr):
        raise ValueError("incomplete/truncated/failed original stderr custody")
    observations = {}
    for line in stderr.splitlines(keepends=True):
        if not line.endswith(b"\n"):
            raise ValueError("partial original stderr row")
        try:
            row = json.loads(line)
        except (ValueError, UnicodeDecodeError):
            if b"ql.native-worker-malloc-observation/v1" in line:
                raise ValueError("malformed apparent native allocator evidence") from None
            continue  # Ordinary non-JSON stderr is retained but supplies no SDK row.
        if not isinstance(row, dict) or row.get("schema") != "ql.native-worker-malloc-observation/v1":
            continue
        if platform == "unavailable-non-Apple":
            raise ValueError("non-Apple custody contains substituted SDK evidence")
        if len(line) >= 2048:
            raise ValueError("native observation exceeds actual writer bound")
        n = decimal(row["completed_request_ordinal"])
        if n in observations or n not in selected or row["pid"] != pid                 or row["phase"] != "after-request-destruction" or row["malloc_zone_scope"] != "all"                 or row["frame_counter_overflow"] is not False:
            raise ValueError("detached/duplicate/unselected native allocator row")
        for key in ("blocks_in_use", "size_in_use", "max_size_in_use", "size_allocated"):
            integer(row[key])
        if not isinstance(row["exception_reply"], bool) or not isinstance(row["state_committed"], bool)                 or not isinstance(row["performance_owner_active"], bool):
            raise ValueError("actual native owner/refusal state required")
        integer(row["retained_line_capacity"])
        observations[n] = row
    if platform == "Apple-SDK-observation-required":
        if sorted(observations) != selected or closed["observed"] != [True] * len(selected)                 or closed["close_wait_exhausted"] is not False:
            raise ValueError("missing actual selected Apple SDK row")
    elif observations or closed["observed"] != [False] * len(selected):
        raise ValueError("non-Apple custody acquired substituted SDK observations")
    if any(type(x) is not bool for x in closed["observed"]) or len(closed["observed"]) != len(selected):
        raise ValueError("invalid actual observation completion flags")
    pairs = []
    for n in selected:
        request_raw = bounded(directory / f"request-{n}.json", MAX_MESSAGE)
        reply_raw = bounded(directory / f"reply-{n}.jsonl", MAX_MESSAGE)
        meta = load(directory / f"exchange-{n}.json")
        if meta["schema"] != "ql.native-worker-malloc-selected-exchange/v1" or meta["worker_pid"] != pid                 or decimal(meta["request_ordinal"]) != n or meta["request_bytes"] != len(request_raw)                 or meta["reply_bytes"] != len(reply_raw) or meta["request_sha256"] != sha(request_raw)                 or meta["reply_sha256"] != sha(reply_raw) or meta["request_fully_written"] is not True                 or meta["response_framed_complete"] is not True or meta["response_parsed"] is not True                 or not reply_raw.endswith(b"\n"):
            raise ValueError("selected native request/reply incomplete, changed or detached")
        request_capacity = integer(meta["request_capacity_bytes"])
        reply_capacity = integer(meta["reply_capacity_bytes"])
        if not len(request_raw) <= request_capacity <= MAX_MESSAGE \
                or not len(reply_raw) <= reply_capacity <= MAX_MESSAGE + 1:
            raise ValueError("selected raw buffers exceed actual capacity custody bound")
        request, reply = json.loads(request_raw), json.loads(reply_raw)
        facts, frontier = meta["facts"], meta["frontier"]
        domain = "field" if request.get("schema") == "ql.field-control/v1" else                  "performance" if request.get("schema") == "ql.performance-control/v1" else "unknown"
        if facts["domain"] != domain or facts["operation"] != request.get("operation")                 or facts["original_requested_frames"] != request.get("frames"):
            raise ValueError("selected request operands detached from actual bytes")
        success = reply.get("schema") == "ql.continuous-field/v1"
        rendered = len(reply["audio"]) if domain == "field" and success else 0
        if integer(frontier["rendered_field_frames"]) != rendered or frontier["frame_counter_overflow"] is not False:
            raise ValueError("actual rendered FIELD frame count detached")
        total = decimal(frontier["rendered_field_frames_total"])
        if total < rendered:
            raise ValueError("source-owned FIELD frontier regressed")
        field = frontier["field"]
        if field is not None:
            if set(field) != set(FIELD_KEYS):
                raise ValueError("incomplete actual FIELD frontier")
            for key in FIELD_KEYS[:3]:
                integer(field[key])
            for key in FIELD_KEYS[3:]:
                decimal(field[key])
        if success:
            if field is None or integer(field["field_sample_count"]) != len(reply["targets"])                     or integer(field["field_mode_count"]) != len(reply["amplitudes_metres"])                     or field["field_samples_elapsed"] != reply["samples_elapsed"]                     or field["field_generation"] != reply["generation"]:
                raise ValueError("frontier detached from complete selected native FIELD reply")
            if request["operation"] == "initialize":
                actual_shapes = sum(len(x["mode_shapes"]) for x in request["field"]["samples"])
                if field["field_shape_vector_count"] != actual_shapes:
                    raise ValueError("native shape count detached from original initialization")
            elif request["operation"] == "replace-shapes":
                if field["field_shape_vector_count"] != sum(len(x) for x in request["shapes"]):
                    raise ValueError("native shape count detached from actual replacement")
        row = observations.get(n)
        if row:
            for key in ("request_bytes", "serialized_reply_bytes", "requested_field_frames", "rendered_field_frames"):
                integer(row[key])
        if row:
            if row["request_bytes"] != len(request_raw) or row["serialized_reply_bytes"] != len(reply_raw) - 1                     or row["operation"] != facts["operation"] or row["domain"] != domain                     or row["rendered_field_frames"] != rendered or decimal(row["rendered_field_frames_total"]) != total                     or row["retained_line_capacity"] < len(request_raw):
                raise ValueError("allocator observation detached from original native bytes/frontier")
            if row["exception_reply"] != (reply.get("schema") == "ql.field-error/v1"):
                raise ValueError("actual native exception reply standing detached")
            if reply.get("schema") == "ql.field-error/v1" and row["state_committed"] != reply["state_committed"]:
                raise ValueError("actual native refusal commit standing detached")
            if domain == "field" and success and request["operation"] == "advance"                     and (row["requested_field_frames"] != request["frames"] or row["requested_field_frames"] != rendered):
                raise ValueError("native requested FIELD frames detached from actual accepted advance")
            expected_field = field if field is not None else dict.fromkeys(FIELD_KEYS)
            if any(row[key] != expected_field[key] for key in FIELD_KEYS):
                raise ValueError("allocator observation detached from actual resident/absent FIELD state")
        pairs.append({"ordinal":str(n),"request_sha256":sha(request_raw),"reply_sha256":sha(reply_raw),
                      "request_bytes":len(request_raw),"reply_bytes":len(reply_raw),"frontier":frontier})
    expected_names = {"opened.json", "closed.json", "stderr-status.json", "worker-stderr.bin"}
    for n in selected:
        expected_names.update((f"request-{n}.json",f"reply-{n}.jsonl",f"exchange-{n}.json"))
    if {p.name for p in directory.iterdir()} != expected_names:
        raise ValueError("unexpected/unselected custody member")
    return {"schema":"ql.native-worker-malloc-selected-byte-qualification/v1", "worker_pid":pid,
            "counter_platform":platform,"pairs":pairs,"stderr_sha256":sha(stderr),
            "observations":[observations[n] for n in selected if n in observations],
            "standing":"actual selected transport bytes; Apple allocator observations only when present; growth cause undetermined"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory",type=Path)
    parser.add_argument("--require-apple",action="store_true")
    args=parser.parse_args()
    print(json.dumps(qualify(args.directory,args.require_apple),separators=(",",":")))


if __name__ == "__main__":
    main()
