#!/usr/bin/env python3
"""Regenerate standalone Python service HTML and the isolated Rust Brew assets.

Run after editing service web-ui/*.html or shared service-design assets.
No service imports this build-time helper on its deployed host.
"""
from __future__ import annotations
import argparse
import re
from pathlib import Path
from embed_service_design import REPO_ROOT, ASSETS, render

PAGES = (
    ("system-backend/asset-management/web-ui/index.html", "Asset Management", "open-lab", "ASSET_MANAGEMENT_HTML", ("system-backend/asset-management/src/netcore_asset_management.py",)),
    ("system-backend/directory/web-ui/index.html", "Directory", "open-lab", "INDEX_HTML", ("system-backend/directory/netcore-directory.py", "misc/ID-Server/netcore_directory_server.py")),
    ("system-backend/sip-switch/web-ui/index.html", "SIP Switch", "open-lab", "INDEX_HTML", ("system-backend/sip-switch/src/netcore_sip_switch.py",)),
    ("system-backend/tbs-connect/web-ui/login.html", "TBS Connect", "session", "LOGIN_HTML", ("system-backend/tbs-connect/server.py", "misc/brew-server.py")),
    ("system-backend/tbs-connect/web-ui/index.html", "TBS Connect", "session", "DASHBOARD_HTML", ("system-backend/tbs-connect/server.py", "misc/brew-server.py")),
)


def synchronize(check: bool = False) -> list[str]:
    changes: list[str] = []
    planned: dict[Path, str] = {}
    for source, name, access, constant, targets in PAGES:
        html = render((REPO_ROOT / source).read_text(), name, access)
        if '"""' in html:
            raise ValueError(f"{source}: HTML contains the Python raw-string delimiter")
        block = f'# BEGIN NETCORE GENERATED {constant}\n{constant} = r"""{html}"""\n# END NETCORE GENERATED {constant}'
        pattern = rf'# BEGIN NETCORE GENERATED {constant}\n.*?\n# END NETCORE GENERATED {constant}'
        for target in targets:
            path = REPO_ROOT / target
            text = planned.get(path, path.read_text())
            text, count = re.subn(pattern, lambda _: block, text, count=1, flags=re.DOTALL)
            if count != 1:
                raise ValueError(f"{target}: generated marker for {constant} missing")
            planned[path] = text
    # Rust Brew's Docker context is deliberately self-contained.
    rust = REPO_ROOT / "misc/brew-server/web-ui"
    for filename in ("service-design.css", "service-design.js", "netcore-logo.data-uri"):
        planned[rust / "assets" / filename] = (ASSETS / filename).read_text()
    planned[rust / "service-design.rs"] = (ASSETS.parent / "service-design.rs").read_text()
    for path, text in planned.items():
        if not path.exists() or path.read_text() != text:
            changes.append(str(path.relative_to(REPO_ROOT)))
            if not check:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(text)
    return changes


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Fail if generated pages/assets are stale")
    args = parser.parse_args()
    changes = synchronize(args.check)
    if changes:
        print("\n".join(changes))
    if args.check and changes:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
