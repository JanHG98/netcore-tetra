#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen


def read_env() -> dict[str, str]:
    env: dict[str, str] = {}
    while True:
        line = sys.stdin.readline()
        if not line or line == "\n":
            break
        key, _, value = line.rstrip("\r\n").partition(":")
        env[key.strip()] = value.strip()
    return env


def command(line: str) -> str:
    print(line, flush=True)
    return sys.stdin.readline().rstrip("\r\n")


def quote(value: object) -> str:
    text = str(value if value is not None else "")
    text = text.replace("\\", "\\\\").replace('"', '\\"').replace("\r", " ").replace("\n", " ")
    return f'"{text}"'


def setvar(name: str, value: object) -> None:
    command(f"SET VARIABLE {name} {quote(value)}")


def verbose(message: str, level: int = 1) -> None:
    command(f"VERBOSE {quote(message)} {level}")


def base_url() -> str:
    path = Path("/etc/netcore/sip-switch-agi.env")
    try:
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.startswith("NETCORE_SIP_SWITCH_URL="):
                return line.split("=", 1)[1].strip().rstrip("/")
    except OSError:
        pass
    return os.environ.get("NETCORE_SIP_SWITCH_URL", "http://127.0.0.1:8300").rstrip("/")


def api(path: str, payload: dict[str, object]) -> tuple[int, dict[str, object]]:
    request = Request(base_url() + path, data=json.dumps(payload).encode("utf-8"), headers={"Content-Type": "application/json"}, method="POST")
    try:
        with urlopen(request, timeout=4) as response:
            return response.status, json.loads(response.read().decode("utf-8"))
    except HTTPError as error:
        try:
            return error.code, json.loads(error.read().decode("utf-8"))
        except ValueError:
            return error.code, {"error": str(error)}
    except (URLError, TimeoutError, OSError) as error:
        return 0, {"error": str(error)}


def endpoint_from_channel(channel: str) -> str:
    # PJSIP/tbs-srv-m-tbs-01-0000002a -> tbs-srv-m-tbs-01
    match = re.match(r"^PJSIP/(.+?)-[0-9A-Fa-f]{8}$", channel)
    return match.group(1) if match else ""


def main() -> None:
    agi = read_env()
    mode = sys.argv[1] if len(sys.argv) > 1 else ""
    if mode == "resolve":
        direction = sys.argv[2] if len(sys.argv) > 2 else "inbound"
        number = sys.argv[3] if len(sys.argv) > 3 else agi.get("agi_extension", "")
        caller = sys.argv[4] if len(sys.argv) > 4 else agi.get("agi_callerid", "")
        payload = {
            "direction": direction,
            "number": number,
            "caller": caller,
            "source_endpoint": endpoint_from_channel(agi.get("agi_channel", "")),
            "commit": True,
            "check_contact": True,
        }
        code, result = api("/api/v1/resolve", payload)
        setvar("NETCORE_ACTION", result.get("action", "reject"))
        setvar("NETCORE_ENDPOINT", result.get("endpoint", ""))
        setvar("NETCORE_DESTINATION", result.get("destination", ""))
        setvar("NETCORE_NODE_ID", result.get("node_id", ""))
        setvar("NETCORE_ISSI", result.get("issi", ""))
        setvar("NETCORE_REASON", result.get("reason", result.get("error", "unknown")))
        setvar("NETCORE_CALL_TOKEN", result.get("call_token", ""))
        setvar("NETCORE_DIAL_TIMEOUT", result.get("dial_timeout_secs", 60))
        verbose(f"NetCore route {direction} {number}: HTTP {code} action={result.get('action')} endpoint={result.get('endpoint')} reason={result.get('reason')}")
        return
    if mode == "state":
        token = sys.argv[2] if len(sys.argv) > 2 else ""
        state = sys.argv[3] if len(sys.argv) > 3 else "updated"
        note = sys.argv[4] if len(sys.argv) > 4 else ""
        if token:
            api(f"/api/v1/calls/{token}/state", {
                "state": state,
                "channel": agi.get("agi_channel"),
                "uniqueid": agi.get("agi_uniqueid"),
                "note": note,
            })
        return
    if mode == "hangup":
        token = sys.argv[2] if len(sys.argv) > 2 else ""
        dial_status = sys.argv[3] if len(sys.argv) > 3 else ""
        hangup_cause = sys.argv[4] if len(sys.argv) > 4 else ""
        state = "ended" if dial_status in {"ANSWER", ""} else "failed"
        if token:
            api(f"/api/v1/calls/{token}/state", {
                "state": state,
                "dial_status": dial_status,
                "hangup_cause": hangup_cause,
                "channel": agi.get("agi_channel"),
                "uniqueid": agi.get("agi_uniqueid"),
            })
        return
    verbose(f"Unknown NetCore AGI mode: {mode}", 2)


if __name__ == "__main__":
    main()
