#!/usr/bin/env python3
"""Run the QL owner's evaluation and numerical learning instruments explicitly.

These commands use the existing native QL and AIKit owners. Model lifecycle is
managed by Workcell. There is no background loop or automatic provider election.
"""
import argparse
import importlib
import sys

sys.dont_write_bytecode = True

COMMANDS = {
    "evaluate": ("ql_agent_evaluation", "run a frozen suite through native QL and optional AIKit execution"),
    "metrics": ("ql_agent_receipt_metrics", "qualify actual provider distributions, latency and input costs"),
    "calibrate": ("ql_agent_calibration", "compare explicit thresholds using validation inference and native M5 Return"),
    "return": ("ql_agent_experiment_return", "compare evaluations and return numerical evidence through native M5"),
    "body-parity": ("ql_agent_body_evidence", "replay actual trained Prime/Pi source uptake through native QL"),
}


def main():
    parser = argparse.ArgumentParser(description=__doc__, epilog="\n".join(
        name + ": " + detail for name, (_, detail) in COMMANDS.items()),
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("command", choices=COMMANDS)
    parser.add_argument("arguments", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    sys.argv = [sys.argv[0] + " " + args.command, *args.arguments]
    importlib.import_module(COMMANDS[args.command][0]).main()


if __name__ == "__main__":
    main()
