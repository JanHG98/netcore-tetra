#!/usr/bin/env python3
"""Validate the static XHTML/WML reference portal without external packages."""

from __future__ import annotations

from collections import deque
from pathlib import Path
import re
import sys
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parent
EXPECTED_PAGES = 21


def validate_family(directory: str, extension: str) -> list[str]:
    errors: list[str] = []
    base = ROOT / directory
    files = sorted(base.glob(f"*.{extension}"))
    if len(files) != EXPECTED_PAGES:
        errors.append(
            f"{directory}: expected {EXPECTED_PAGES} files, found {len(files)}"
        )

    graph: dict[str, list[str]] = {}
    for path in files:
        try:
            ET.parse(path)
        except ET.ParseError as exc:
            errors.append(f"{path.name}: invalid XML: {exc}")

        text = path.read_text(encoding="utf-8")
        targets = re.findall(r'href="([^"]+)"', text)
        graph[path.name] = targets
        for target in targets:
            if not target.endswith(f".{extension}"):
                errors.append(f"{path.name}: cross-format link {target}")
            if not (base / target).is_file():
                errors.append(f"{path.name}: missing target {target}")

    start = f"index.{extension}"
    visited: set[str] = set()
    queue: deque[str] = deque([start])
    while queue:
        current = queue.popleft()
        if current in visited:
            continue
        visited.add(current)
        queue.extend(graph.get(current, []))

    missing = sorted(set(graph) - visited)
    if missing:
        errors.append(f"{directory}: unreachable pages: {', '.join(missing)}")
    return errors


def main() -> int:
    errors = validate_family("xhtml", "xhtml")
    errors.extend(validate_family("wml", "wml"))
    if errors:
        print("WAP portal validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print(
        "WAP portal validation OK: 21 XHTML + 21 WML pages, valid XML, "
        "same-format links, all pages reachable."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
