#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import sys
from pathlib import Path

CORE = Path("/usr/local/lib/netcore-alarm-workflow/netcore_alarm_workflow.py")
WEBUI = Path("/usr/local/share/netcore-alarm-workflow/index.html")


def load_webui() -> str:
    html = WEBUI.read_text(encoding="utf-8")
    # Korrigiert die deutsche Flexion im dynamischen Alarm-Headline-Text.
    html = html.replace(
        "$('#headline').textContent=crit.length?`${crit.length} kritische Alarm${crit.length>1?'e':''}`:active.length?`${active.length} aktive Alarm${active.length>1?'e':''}`:'Keine aktiven Alarme';",
        "$('#headline').textContent=crit.length?(crit.length===1?'1 kritischer Alarm':`${crit.length} kritische Alarme`):active.length?(active.length===1?'1 aktiver Alarm':`${active.length} aktive Alarme`):'Kein aktiver Alarm';",
    )
    return html


def main() -> int:
    spec = importlib.util.spec_from_file_location("netcore_alarm_workflow_core", CORE)
    if spec is None or spec.loader is None:
        raise SystemExit(f"cannot load alarm-workflow core from {CORE}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)

    try:
        module.HTML = load_webui()
    except OSError as error:
        print(f"warning: cannot load WebUI {WEBUI}: {error}; using embedded fallback UI", file=sys.stderr)

    result = module.main()
    return int(result or 0)


if __name__ == "__main__":
    raise SystemExit(main())
