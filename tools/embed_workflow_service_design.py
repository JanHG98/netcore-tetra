#!/usr/bin/env python3
"""Refresh the standalone workflow WebUIs and their identical Python fallbacks.

The functional markup in each index.html is the editable source. Shared shell
blocks are generated from system-backend/shared/web-ui/assets. Use --check in
validation to catch a stale embedded fallback or an outdated shared bundle.
"""
from __future__ import annotations

import argparse
import ast
import re
from pathlib import Path

from embed_service_design import REPO_ROOT, refresh


PAGES = (
    ("alarm-workflow", "Alarm Workflow", "open-lab"),
    ("task-workflow", "Task Workflow", "open-lab"),
    ("alert-service", "Warnzentrale", "access-key"),
)


def refreshed(source: str, name: str, access: str) -> str:
    layout = re.search(r'<style id="(?:workflow|warnzentrale)-layout">.*?</style>', source, re.DOTALL)
    extra = layout.group(0) if layout else ""
    if extra:
        source = source.replace(extra, "", 1)
    source = re.sub(r'<script>document.documentElement.dataset.netcoreUi=[\'\"]service[\'\"];?</script>', "", source, count=1)
    page = refresh(source, name, access)
    if access == "access-key":
        page = re.sub(r'<script id="netcore-service-init">.*?</script>', '<script id="netcore-service-init" src="/service-theme-init.js"></script>', page, flags=re.DOTALL)
        page = re.sub(r'<script id="netcore-service-shell">.*?</script>', '<script id="netcore-service-shell" src="/service-design.js" defer></script>', page, flags=re.DOTALL)
        if 'data-netcore-ui="service"' not in page[:page.index('<head>')]:
            page = page.replace('<html lang="de">', '<html lang="de" data-netcore-ui="service">', 1)
    return page.replace("</head>", extra + "</head>", 1)



def fallback(core: str, page: str) -> str:
    for node in ast.parse(core).body:
        if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "HTML" for t in node.targets):
            lines = core.splitlines(keepends=True)
            return "".join(lines[:node.lineno - 1]) + "HTML = r'''" + page + "'''\n" + "".join(lines[node.end_lineno:])
    raise ValueError("No embedded HTML assignment found")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    stale = []
    for service, name, access in PAGES:
        directory = REPO_ROOT / "system-backend" / service
        path = directory / ("static/index.html" if service == "alert-service" else "web-ui/index.html")
        page = refreshed(path.read_text(), name, access)
        outputs = [(path, page)]
        if service == "alert-service":
            for asset in ("service-design.js", "service-theme-init.js"):
                outputs.append((directory / "static" / asset, (REPO_ROOT / "system-backend/shared/web-ui/assets" / asset).read_text()))
        if service != "alert-service":
            core = directory / "src" / ("netcore_" + service.replace("-", "_") + ".py")
            outputs.append((core, fallback(core.read_text(), page)))
        for target, value in outputs:
            if not target.exists() or target.read_text() != value:
                stale.append(str(target.relative_to(REPO_ROOT)))
                if not args.check:
                    target.write_text(value)
    if args.check and stale:
        parser.exit(1, "Outdated workflow WebUI assets: " + ", ".join(stale) + "\nRun python tools/embed_workflow_service_design.py\n")
    print("Workflow WebUI bundles and embedded fallbacks are synchronized.")


if __name__ == "__main__":
    main()
