#!/usr/bin/env python3
"""Idempotent OpenLab inventory migration; preserve custom URLs and UI targets."""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import sys
import time
import tomllib

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "system-backend/deployment-core"))
from common import atomic_write, toml_dump


def migrate(data, example):
    by_service = {t["service"]: t for t in example["targets"]}
    present = set()
    for target in data.setdefault("targets", []):
        service = target.get("service", "")
        present.add(service)
        if target.get("labels", {}).get("discovery") == "manual":
            continue
        url = target.get("base_url", "")
        if service in by_service and url.startswith("http://127.0.0.1:"):
            target["base_url"] = by_service[service]["base_url"]
    data["targets"].extend(t for s, t in by_service.items() if s not in present)
    data.setdefault("discovery", example["discovery"].copy())
    # Lower only the old, untouched 100,000-record default for the bounded preview.
    retention = data.setdefault("retention", {})
    if retention.get("max_logs", 100000) == 100000:
        retention["max_logs"] = 10000
    return data


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("config", type=Path)
    args = parser.parse_args()
    example = tomllib.loads((ROOT / "system-backend/observability/config/observability.example.toml").read_text())
    before = args.config.read_text()
    current = tomllib.loads(before)
    updated = migrate(current, example)
    after = toml_dump(updated)
    if tomllib.loads(before) == updated:
        return
    backup = args.config.with_name(args.config.name + ".before-syslog-" + str(time.time_ns()))
    shutil.copy2(args.config, backup)
    st = args.config.stat()
    atomic_write(args.config, after, st.st_mode & 0o777)
    os.chown(args.config, st.st_uid, st.st_gid)
    print(f"Updated OpenLab targets; original config: {backup}")


if __name__ == "__main__":
    main()
