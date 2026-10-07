#!/usr/bin/env python3
"""Compare complete Git tip trees directly, including file modes and binary blobs.

This deliberately does not use a merge-base diff. Unchanged blob IDs prove equal
contents; differing IDs require a separate semantic review before integration.
"""
from __future__ import annotations

import argparse
from collections import Counter
import csv
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def git(*args: str) -> bytes:
    return subprocess.check_output(["git", "-C", str(ROOT), *args])


def tree(ref: str) -> dict[str, tuple[str, str, str]]:
    result = {}
    for item in git("ls-tree", "-rz", "--full-tree", ref).split(b"\0"):
        if not item:
            continue
        meta, path = item.split(b"\t", 1)
        mode, kind, sha = meta.decode("ascii").split()
        result[path.decode("utf-8")] = (mode, kind, sha)
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--main", required=True, help="Reviewed main commit, not a moving branch")
    parser.add_argument("--historical", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    refs = {key: git("rev-parse", "--verify", value + "^{commit}").decode().strip()
            for key, value in (("main", args.main), ("historical", args.historical))}
    current, old = tree(refs["main"]), tree(refs["historical"])
    rows = []
    for path in sorted(current.keys() | old.keys()):
        a, b = current.get(path), old.get(path)
        status = "historical_only" if a is None else "main_only" if b is None else "identical" if a == b else "different"
        rows.append([path, status, *(a or ("", "", "")), *(b or ("", "", ""))])
    args.output.mkdir(parents=True, exist_ok=True)
    header = ["path", "status", "main_mode", "main_type", "main_blob", "historical_mode", "historical_type", "historical_blob"]
    for filename, selected in (("source-comparison.csv", rows),
                               ("source-differences.csv", [r for r in rows if r[1] != "identical"])):
        with (args.output / filename).open("w", newline="", encoding="utf-8") as handle:
            writer = csv.writer(handle, lineterminator="\n")
            writer.writerow(header)
            writer.writerows(selected)
    summary = {"commits": refs,
               "trees": {key: git("rev-parse", value + "^{tree}").decode().strip() for key, value in refs.items()},
               "entries": {"main": len(current), "historical": len(old), "union": len(rows)},
               "counts": dict(sorted(Counter(row[1] for row in rows).items())),
               "method": "Direct recursive tip-tree comparison; file modes, types and content object IDs; no merge-base diff",
               "scope": "Repository source evidence only; no live installation, binary or radio acceptance"}
    (args.output / "source-summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
