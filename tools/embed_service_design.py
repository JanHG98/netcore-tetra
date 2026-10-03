#!/usr/bin/env python3
"""Build-time embedding for standalone service pages (no runtime dependency).

Use render(html, name, access) from repository tooling. Re-running the function
on an embedded page is harmless. The original markup/handlers stay unchanged.
"""
from __future__ import annotations

import argparse
import base64
import json
import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
ASSETS = REPO_ROOT / "system-backend/shared/web-ui/assets"
MARKER = 'id="netcore-service-design"'


def render(html: str, name: str, access: str = "open-lab") -> str:
    """Return a standalone HTML page with the same bundle as Rust services."""
    if MARKER in html:
        return html
    config = json.dumps({"name": name, "access": access, "logo": (ASSETS / "netcore-logo.data-uri").read_text().strip()}, ensure_ascii=False, separators=(",", ":"))
    config = config.replace("<", "\\u003c").replace(">", "\\u003e").replace("&", "\\u0026")
    style = (ASSETS / "service-design.css").read_text()
    script = (ASSETS / "service-design.js").read_text()
    theme_init = (ASSETS / "service-theme-init.js").read_text()
    init = f'<script id="netcore-service-init">{theme_init}</script>'
    open_head = re.search(r"<head\b[^>]*>", html, re.IGNORECASE)
    html = html[:open_head.end()] + init + html[open_head.end():] if open_head else init + html
    head = f'<style {MARKER}>{style}</style><script type="application/json" id="netcore-service-config">{config}</script>'
    close_head = re.search(r"</head\s*>", html, re.IGNORECASE)
    html = html[:close_head.start()] + head + html[close_head.start():] if close_head else head + html
    shell = f'<script id="netcore-service-shell">{script}</script>'
    close_body = re.search(r"</body\s*>", html, re.IGNORECASE)
    return html[:close_body.start()] + shell + html[close_body.start():] if close_body else html + shell


def refresh(html: str, name: str | None = None, access: str | None = None) -> str:
    """Refresh an existing embedded bundle, keeping its service identity by default."""
    config_match = re.search(r'<script\b[^>]*\bid="netcore-service-config"[^>]*>(.*?)</script\s*>', html, re.IGNORECASE | re.DOTALL)
    existing = json.loads(config_match.group(1)) if config_match else {}
    for tag, ident in [("style", "netcore-service-design"), ("script", "netcore-service-config"), ("script", "netcore-service-init"), ("script", "netcore-service-shell")]:
        html = re.sub(rf'<{tag}\b[^>]*\bid="{ident}"[^>]*>.*?</{tag}\s*>', "", html, flags=re.IGNORECASE | re.DOTALL)
    return render(html, name or existing.get("name", "NetCore Dienst"), access or existing.get("access", "open-lab"))


def sync_logo() -> None:
    """Refresh the embedded original PNG without resampling or re-drawing it."""
    source = REPO_ROOT / "crates/tetra-entities/src/net_dashboard/ui/netcore-logo.png"
    (ASSETS / "netcore-logo.data-uri").write_text("data:image/png;base64," + base64.b64encode(source.read_bytes()).decode("ascii") + "\n")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("html", nargs="?", type=Path)
    parser.add_argument("--name")
    parser.add_argument("--access", choices=["open-lab", "session", "access-key", "optional-basic", "http-basic"])
    parser.add_argument("--output", type=Path)
    parser.add_argument("--sync-logo", action="store_true")
    parser.add_argument("--refresh", action="store_true", help="Replace an existing bundle with the current assets")
    args = parser.parse_args()
    if args.sync_logo:
        sync_logo()
    if args.html:
        rendered = refresh(args.html.read_text(), args.name, args.access) if args.refresh else render(args.html.read_text(), args.name or "NetCore Dienst", args.access or "open-lab")
        (args.output or args.html).write_text(rendered)


if __name__ == "__main__":
    main()
