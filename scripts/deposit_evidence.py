#!/usr/bin/env python3
"""Decide whether a VM-A deposit directory can be submitted again.

A live request or any commitment mapping must not be posted over. A request
that Failed, or is absent after a database restart, and has no mapping, never
landed. That is not a new contract deploy.
"""

import sys

LIVE = {"Queued", "Processing", "Submitted", "Confirmed"}
DEAD = {"Failed", "absent"}


def evidence_state(statuses, mapping_count):
    if len(statuses) != 4 or mapping_count < 0:
        return "unknown"
    if mapping_count or any(status in LIVE for status in statuses):
        return "live"
    if all(status in DEAD for status in statuses):
        return "dead"
    return "unknown"


def main(argv):
    if len(argv) != 3:
        print("usage: deposit_evidence.py status,status,status,status mapping_count", file=sys.stderr)
        return 2
    statuses = argv[1].split(",")
    try:
        mapping_count = int(argv[2])
    except ValueError:
        print("unknown")
        return 0
    print(evidence_state(statuses, mapping_count))
    return 0


if __name__ == "__main__":
    if len(sys.argv) == 1:
        assert evidence_state(["Failed", "Failed", "Failed", "Failed"], 0) == "dead"
        assert evidence_state(["absent", "absent", "absent", "absent"], 0) == "dead"
        assert evidence_state(["Failed", "absent", "Failed", "absent"], 0) == "dead"
        assert evidence_state(["Queued", "Failed", "Failed", "Failed"], 0) == "live"
        assert evidence_state(["Submitted", "Processing", "Queued", "Confirmed"], 0) == "live"
        assert evidence_state(["Failed", "Failed", "Failed", "Failed"], 1) == "live"
        assert evidence_state(["Failed", "Failed", "Failed"], 0) == "unknown"
        assert evidence_state(["Failed", "Failed", "Failed", "weird"], 0) == "unknown"
        print("ok")
    else:
        sys.exit(main(sys.argv))
