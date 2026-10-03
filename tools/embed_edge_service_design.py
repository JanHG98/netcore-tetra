#!/usr/bin/env python3
"""Regenerate standalone Hardware/RF HTML from source templates and shared assets.

The deployed services contain everything in their Python HTML constants. This
helper runs in the repository only; it introduces no runtime dependencies.
"""
from __future__ import annotations

import argparse
import re

from embed_service_design import REPO_ROOT, render

PAGES = (
    ("hardware-gateway", "Hardware-Gateway", "netcore_hardware_gateway.py"),
    ("rf-monitor", "RF-Monitor", "netcore_rf_monitor.py"),
)


def synchronize(check: bool = False) -> list[str]:
    changes: list[str] = []
    for service, name, filename in PAGES:
        source = REPO_ROOT / "system-backend" / service / "web-ui" / "index.html"
        target = REPO_ROOT / "system-backend" / service / "src" / filename
        html = render(source.read_text(), name, "open-lab")
        if '"""' in html:
            raise ValueError(f"{source}: HTML contains the Python raw-string delimiter")
        block = f'# BEGIN NETCORE GENERATED HTML\nHTML = r"""{html}"""\n# END NETCORE GENERATED HTML'
        text, count = re.subn(
            r"# BEGIN NETCORE GENERATED HTML\n.*?\n# END NETCORE GENERATED HTML",
            lambda _: block,
            target.read_text(),
            count=1,
            flags=re.DOTALL,
        )
        if count != 1:
            raise ValueError(f"{target}: generated HTML marker missing")
        if target.read_text() != text:
            changes.append(str(target.relative_to(REPO_ROOT)))
            if not check:
                target.write_text(text)
    return changes


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Fail if generated pages are stale")
    args = parser.parse_args()
    changes = synchronize(args.check)
    if changes:
        print("\n".join(changes))
    if args.check and changes:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
