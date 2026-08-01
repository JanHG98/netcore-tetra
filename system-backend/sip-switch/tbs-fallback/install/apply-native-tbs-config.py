#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import shutil
import time
from pathlib import Path


def parse_snippet(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    active = False
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if line == "[asterisk]":
            active = True
            continue
        if active and line.startswith("["):
            break
        if not active or not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key.strip()] = value.strip()
    if not values:
        raise ValueError("snippet contains no [asterisk] values")
    return values


def update_section(text: str, values: dict[str, str]) -> str:
    lines = text.splitlines()
    start = next((i for i, line in enumerate(lines) if line.strip() == "[asterisk]"), None)
    if start is None:
        if lines and lines[-1].strip():
            lines.append("")
        lines.append("[asterisk]")
        lines.extend(f"{key} = {value}" for key, value in values.items())
        return "\n".join(lines) + "\n"
    end = len(lines)
    for i in range(start + 1, len(lines)):
        if re.match(r"^\s*\[[^]]+\]\s*$", lines[i]):
            end = i
            break
    found: set[str] = set()
    for i in range(start + 1, end):
        match = re.match(r"^(\s*)([A-Za-z0-9_]+)(\s*=.*)$", lines[i])
        if not match:
            continue
        key = match.group(2)
        if key in values:
            lines[i] = f"{match.group(1)}{key} = {values[key]}"
            found.add(key)
    insert_at = end
    missing = [key for key in values if key not in found]
    lines[insert_at:insert_at] = [f"{key} = {values[key]}" for key in missing]
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", default="/etc/netcore/config.toml")
    parser.add_argument("--snippet", default="/etc/netcore/tbs-asterisk-local-snippet.toml")
    args = parser.parse_args()
    config = Path(args.config)
    snippet = Path(args.snippet)
    if not config.is_file():
        raise SystemExit(f"config not found: {config}")
    values = parse_snippet(snippet)
    backup = config.with_name(config.name + f".pre-phase11c-{time.strftime('%Y%m%d-%H%M%S')}")
    shutil.copy2(config, backup)
    updated = update_section(config.read_text(encoding="utf-8"), values)
    tmp = config.with_suffix(config.suffix + ".tmp")
    tmp.write_text(updated, encoding="utf-8")
    tmp.replace(config)
    print(f"updated {config}; backup {backup}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
