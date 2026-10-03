#!/usr/bin/env python3
"""Qualify retained genuine worker input/stdout/stderr; launches no process.

The guarded Mac owner supplies these exact complete byte streams and the actual
opt-in request ordinals. This does not manufacture malloc values or diagnose a
leak. Private inputs remain in the owner's evidence directory.
"""
import argparse
import hashlib
import json
from pathlib import Path


def lines(path, bound):
    with path.open("rb") as stream:
        while True:
            line = stream.readline(bound + 2)
            if not line:
                return
            if not line.endswith(b"\n") or len(line) > bound + 1:
                raise ValueError("incomplete/oversized retained native line")
            yield line[:-1]


def decimal(text):
    if not isinstance(text, str) or not text or not text.isascii():
        raise ValueError("native exact decimal text required")
    value = int(text)
    if str(value) != text or not 0 <= value <= 2**64 - 1:
        raise ValueError("noncanonical native exact counter")
    return value


def digest(path):
    result = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(65536), b""):
            result.update(chunk)
    return result.hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("requests", type=Path)
    parser.add_argument("replies", type=Path)
    parser.add_argument("stderr", type=Path)
    parser.add_argument("--selected", required=True)
    args = parser.parse_args()
    selected = [decimal(x) for x in args.selected.split(",")]
    if not 1 <= len(selected) <= 3 or any(x == 0 for x in selected) or \
            selected != sorted(set(selected)):
        raise ValueError("one to three strictly increasing positive ordinals")
    observations = {}
    for raw in lines(args.stderr, 8192):
        if not raw.startswith(b'{"schema":"ql.native-worker-malloc-observation/v1"'):
            continue
        if len(raw) >= 2048:
            raise ValueError("native observation exceeds fixed writer bound")
        value = json.loads(raw)
        ordinal = decimal(value["completed_request_ordinal"])
        if ordinal in observations or ordinal not in selected:
            raise ValueError("duplicate/unselected native allocation observation")
        if value["phase"] != "after-request-destruction" or \
                value["malloc_zone_scope"] != "all" or value["frame_counter_overflow"]:
            raise ValueError("wrong native allocation scope/phase or counter overflow")
        for field in ("blocks_in_use", "size_in_use", "max_size_in_use", "size_allocated"):
            if type(value[field]) is not int or value[field] < 0:
                raise ValueError("actual nonnegative SDK statistics required")
        observations[ordinal] = value
    if sorted(observations) != selected:
        raise ValueError("missing actual native allocation observation")
    replies = iter(lines(args.replies, 64 * 1024 * 1024))
    total_frames = 0
    field_reading = None
    shape_count = None
    for ordinal, request_raw in enumerate(lines(args.requests, 32 * 1024 * 1024), 1):
        reply_raw = next(replies, None)
        if reply_raw is None:
            raise ValueError("actual request has no complete native reply")
        request, reply = json.loads(request_raw), json.loads(reply_raw)
        is_field = request.get("schema") == "ql.field-control/v1"
        success = reply.get("schema") == "ql.continuous-field/v1"
        rendered = len(reply["audio"]) if is_field and success else 0
        total_frames += rendered
        if success:
            field_reading = reply
            if request["operation"] == "initialize":
                shape_count = sum(len(x["mode_shapes"]) for x in request["field"]["samples"])
            elif request["operation"] == "replace-shapes":
                shape_count = sum(len(x) for x in request["shapes"])
        if ordinal not in observations:
            continue
        value = observations[ordinal]
        if value["request_bytes"] != len(request_raw) or \
                value["serialized_reply_bytes"] != len(reply_raw) or \
                value["rendered_field_frames"] != rendered or \
                decimal(value["rendered_field_frames_total"]) != total_frames:
            raise ValueError("allocation observation detached from genuine worker bytes/frames")
        if value["operation"] != request["operation"] or value["domain"] != \
                ("field" if is_field else "performance"):
            raise ValueError("allocation observation operation/domain mismatch")
        if is_field and success and request["operation"] == "advance" and (
                value["requested_field_frames"] != request["frames"] or
                value["requested_field_frames"] != rendered):
            raise ValueError("native requested FIELD frames detached from actual control/output")
        if field_reading is None and any(value[key] is not None for key in (
                "field_sample_count", "field_mode_count", "field_shape_vector_count",
                "field_samples_elapsed", "field_generation")):
            raise ValueError("absent FIELD owner acquired fabricated native fields")
        if field_reading is not None and (
                decimal(value["field_samples_elapsed"]) != decimal(field_reading["samples_elapsed"]) or
                decimal(value["field_generation"]) != decimal(field_reading["generation"]) or
                value["field_sample_count"] != len(field_reading["targets"]) or
                value["field_mode_count"] != len(field_reading["amplitudes_metres"]) or
                value["field_shape_vector_count"] != shape_count):
            raise ValueError("observation detached from actual resident FIELD owner")
    if next(replies, None) is not None:
        raise ValueError("unpaired actual native reply")
    if len({x["pid"] for x in observations.values()}) != 1 or \
            next(iter(observations.values()))["pid"] <= 0:
        raise ValueError("one actual qualified worker PID required")
    print(json.dumps({"schema": "ql.native-worker-malloc-byte-qualification/v1",
                      "request_sha256": digest(args.requests),
                      "reply_sha256": digest(args.replies),
                      "stderr_sha256": digest(args.stderr),
                      "selected": selected, "observations": list(observations.values()),
                      "standing": "retained actual worker bytes/statistics; growth cause undetermined"},
                     separators=(",", ":")))


if __name__ == "__main__":
    main()
