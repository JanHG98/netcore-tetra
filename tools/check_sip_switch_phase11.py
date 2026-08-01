#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
import threading
import time
import tomllib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    "system-backend/sip-switch/src/netcore_sip_switch.py",
    "system-backend/sip-switch/agi/netcore-sip-route.py",
    "system-backend/sip-switch/config/sip-switch.example.toml",
    "system-backend/sip-switch/systemd/netcore-sip-switch.service",
    "system-backend/sip-switch/install/install.sh",
    "system-backend/sip-switch/install/update.sh",
    "system-backend/sip-switch/install/uninstall.sh",
    "system-backend/sip-switch/install/configure-openlab.sh",
    "system-backend/sip-switch/install/add-tbs-openlab.sh",
    "system-backend/sip-switch/install/print-tbs-config.sh",
    "system-backend/sip-switch/README.md",
    "Docs/PHASE_11_CENTRAL_SIP_SWITCH.md",
]


def request(url: str, method: str = "GET", payload: dict | None = None):
    data = None if payload is None else json.dumps(payload).encode()
    req = Request(url, data=data, method=method, headers={"Content-Type": "application/json"})
    try:
        with urlopen(req, timeout=3) as response:
            return response.status, json.loads(response.read().decode())
    except HTTPError as error:
        return error.code, json.loads(error.read().decode())


class MobilityHandler(BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def do_GET(self):
        if self.path == "/health/live":
            body = {"status": "live"}
            code = 200
        elif self.path == "/api/v1/subscribers/4010001/route":
            body = {
                "issi": 4010001,
                "state": "confirmed",
                "registered": True,
                "serving_node": "SRV-M-TBS-01",
                "node_connected": True,
                "route_generation": 7,
            }
            code = 200
        else:
            body = {"error": "not found"}
            code = 404
        raw = json.dumps(body).encode()
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)


def main() -> int:
    errors: list[str] = []
    for rel in REQUIRED:
        if not (ROOT / rel).is_file():
            errors.append(f"missing {rel}")
    with (ROOT / "system-backend/sip-switch/config/sip-switch.example.toml").open("rb") as handle:
        cfg = tomllib.load(handle)
    if not cfg["server"]["bind"].endswith(":8300"):
        errors.append("wrong management port")
    if cfg["security"]["mode"] != "open_lab":
        errors.append("not open_lab")
    if cfg["pbx"]["mode"] not in {"ip_trunk", "registration"}:
        errors.append("invalid PBX mode")
    event_source = (ROOT / "system-backend/shared/contracts/src/event.rs").read_text()
    for event in ["sip.route_resolved", "sip.route_failed", "sip.call_started", "sip.call_answered", "sip.call_ended", "sip.tbs_contact_up", "sip.tbs_contact_down"]:
        if event not in event_source:
            errors.append(f"missing event {event}")
    for script in (ROOT / "system-backend/sip-switch/install").glob("*.sh"):
        if not os.access(script, os.X_OK):
            errors.append(f"not executable {script.relative_to(ROOT)}")
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1

    mobility = ThreadingHTTPServer(("127.0.0.1", 0), MobilityHandler)
    mobility_thread = threading.Thread(target=mobility.serve_forever, daemon=True)
    mobility_thread.start()
    mobility_port = mobility.server_address[1]

    with tempfile.TemporaryDirectory() as raw_td:
        td = Path(raw_td)
        fake_asterisk = td / "asterisk"
        fake_asterisk.write_text("""#!/usr/bin/env bash
set -e
cmd="${2:-}"
case "$cmd" in
  "core show version") echo "Asterisk 20. test" ;;
  "pjsip show endpoint netcore-pbx") echo "Endpoint: netcore-pbx" ;;
  "pjsip show aor tbs-srv-m-tbs-01-aor") echo "Aor: tbs-srv-m-tbs-01-aor"; echo "Contact: tbs-srv-m-tbs-01-aor/sip:tbs@127.0.0.1:5062 Avail" ;;
  "pjsip show registrations") echo "netcore-pbx-registration Registered" ;;
  "core reload") echo "Reloaded" ;;
  *) echo "OK $cmd" ;;
esac
""", encoding="utf-8")
        fake_asterisk.chmod(0o755)
        asterisk_dir = td / "asterisk-conf"
        asterisk_dir.mkdir()
        port = 18300
        config = f'''[service]
name="netcore-sip-switch"
phase=11
mode="open_lab"
[server]
bind="127.0.0.1:{port}"
[security]
mode="open_lab"
[storage]
state_file="{td/'state.json'}"
event_log="{td/'events.ndjson'}"
audit_log="{td/'audit.ndjson'}"
[mqtt]
enabled=false
host="127.0.0.1"
port=1883
topic_prefix="netcore/v1"
client_id="test"
[mobility_core]
enabled=true
base_url="http://127.0.0.1:{mobility_port}"
timeout_secs=1
[asterisk]
enabled=true
binary="{fake_asterisk}"
config_dir="{asterisk_dir}"
agi_script="netcore-sip-route.py"
sip_bind="127.0.0.1:15060"
rtp_start=12000
rtp_end=12100
[management]
event_history_limit=200
probe_interval_secs=60
[pbx]
mode="ip_trunk"
endpoint_id="netcore-pbx"
host="127.0.0.1"
port=5060
transport="udp"
username=""
auth_username=""
password=""
from_user="netcore"
from_domain=""
contact_user="netcore"
allow="ulaw"
match=["127.0.0.1"]
[routing]
tetra_number_prefix=""
strip_tetra_prefix=false
pbx_outbound_prefix="91"
strip_pbx_outbound_prefix=true
accept_stale_routes=false
require_tbs_contact=true
dial_timeout_secs=60
[[tbs]]
node_id="SRV-M-TBS-01"
endpoint_id="tbs-srv-m-tbs-01"
username="tbs-01"
password="lab"
enabled=true
max_contacts=1
aliases=["TBS-01"]
'''
        cp = td / "config.toml"
        cp.write_text(config)
        service = ROOT / "system-backend/sip-switch/src/netcore_sip_switch.py"
        render = subprocess.run([sys.executable, str(service), "--config", str(cp), "--render-asterisk"], capture_output=True, text=True)
        if render.returncode:
            raise RuntimeError(render.stderr or render.stdout)
        pjsip = (asterisk_dir / "netcore-pjsip.conf").read_text()
        dialplan = (asterisk_dir / "netcore-extensions.conf").read_text()
        assert "[tbs-srv-m-tbs-01]" in pjsip
        assert "type=registration" not in pjsip
        assert "PJSIP_DIAL_CONTACTS" in dialplan
        assert "netcore-from-pbx" in dialplan and "netcore-from-tbs" in dialplan

        proc = subprocess.Popen([sys.executable, str(service), "--config", str(cp)], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        try:
            for _ in range(60):
                try:
                    urlopen(f"http://127.0.0.1:{port}/health/live", timeout=.2)
                    break
                except Exception:
                    time.sleep(.1)
            else:
                raise RuntimeError(proc.stderr.read().decode())
            status_code, inbound = request(f"http://127.0.0.1:{port}/api/v1/resolve", "POST", {
                "direction": "inbound", "number": "4010001", "caller": "600", "commit": True, "check_contact": True,
            })
            assert status_code == 200, inbound
            assert inbound["action"] == "tbs" and inbound["endpoint"] == "tbs-srv-m-tbs-01"
            token = inbound["call_token"]
            for state in ("dialing", "answered", "ended"):
                code, _ = request(f"http://127.0.0.1:{port}/api/v1/calls/{token}/state", "POST", {"state": state})
                assert code == 200
            code, outbound = request(f"http://127.0.0.1:{port}/api/v1/resolve", "POST", {
                "direction": "outbound", "number": "91600", "source_endpoint": "tbs-srv-m-tbs-01", "commit": True,
            })
            assert code == 200 and outbound["action"] == "pbx" and outbound["destination"] == "600"
            _, calls = request(f"http://127.0.0.1:{port}/api/v1/calls")
            assert any(call["call_token"] == token and call["state"] == "ended" for call in calls)
            _, status = request(f"http://127.0.0.1:{port}/api/v1/status")
            assert status["media_mode"] == "edge_media" and status["central_media_ready"] is False
            assert b"NetCore SIP Switch" in urlopen(f"http://127.0.0.1:{port}/").read()
        finally:
            proc.terminate()
            proc.wait(timeout=5)
    mobility.shutdown()
    mobility.server_close()
    print("OK: Phase 11 SIP Switch routing, Asterisk render, Mobility-Core resolution and call lifecycle")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
