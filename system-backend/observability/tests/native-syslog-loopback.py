"""Regress IP-bound NMS + the strictly local, durable Syslog preview bridge.

Use an isolated Linux runner: both 127.0.0.1:8210 and 127.0.0.2:8210 must be
free. The fixed port is intentional because load_config enforces that endpoint.
No monitored LAN targets or discovery are enabled; all test state is temporary.
"""
import contextlib
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import time
import tomllib
import urllib.request
import uuid

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[1]
sys.path.insert(0, str(ROOT / "logging"))
sys.path.insert(0, str(REPO / "system-backend/deployment-core"))
from log_store import Store, forward_once, load_config
from common import toml_dump

PRIMARY = "http://127.0.0.2:8210"
PREVIEW = "http://127.0.0.1:8210"


def main():
    binary = Path(os.environ.get(
        "OBSERVABILITY_BINARY", REPO / "target/debug/netcore-observability"
    )).resolve()
    if not binary.is_file():
        raise RuntimeError(f"Build the real netcore-observability binary first: {binary}")
    # Refuse an occupied test endpoint instead of accidentally querying another
    # process. Hold both probes together to also detect a wildcard listener.
    with contextlib.ExitStack() as probes:
        for address in ("127.0.0.1", "127.0.0.2"):
            probe = probes.enter_context(socket.socket())
            try:
                probe.bind((address, 8210))
            except OSError as error:
                raise RuntimeError(
                    f"Native loopback test needs an isolated runner; {address}:8210 "
                    f"is unavailable: {error}"
                ) from error

    http = urllib.request.build_opener(urllib.request.ProxyHandler({}))

    def get(base, path):
        with http.open(base + path, timeout=3) as response:
            return json.load(response)

    with tempfile.TemporaryDirectory(prefix="netcore-native-loopback-") as temporary:
        base = Path(temporary)
        cfg = tomllib.loads((ROOT / "config/observability.example.toml").read_text())
        cfg["server"]["bind"] = "127.0.0.2:8210"
        cfg["targets"] = []
        cfg["collection"].update(scrape_on_start=False, scrape_interval_secs=3600)
        cfg["discovery"]["enabled"] = False
        cfg["storage"] = {
            "state_path": str(base / "state.json"),
            "backup_path": str(base / "backup.json"),
            "diagnostic_dir": str(base / "diagnostics"),
        }
        config_path = base / "config.toml"
        config_path.write_text(toml_dump(cfg))
        original_config = config_path.read_bytes()
        inventory = base / "hosts.json"
        inventory.write_text(json.dumps({"hosts": [
            {"address": "127.0.0.2", "name": "Loopback-Test"}
        ]}))
        logcfg = json.loads((ROOT / "config/syslog.example.json").read_text())
        logcfg.update(
            state_dir=str(base / "logs"), inventory=str(inventory),
            archive_mount=str(base / "unused-share"), collector_id="native-loopback",
            allowed_networks=["127.0.0.0/8"], nms_url=PREVIEW,
        )
        log_config_path = base / "syslog.json"
        log_config_path.write_text(json.dumps(logcfg))
        # Exercise production validation, rather than bypassing the exact local
        # nms_url requirement by constructing Store from an unchecked dict.
        store = Store(load_config(log_config_path))
        proc = None
        out = (base / "native.out").open("w")
        try:
            proc = subprocess.Popen(
                [str(binary), "--config", str(config_path)], stdout=out, stderr=out
            )
            deadline = time.monotonic() + 10
            while time.monotonic() < deadline:
                if proc.poll() is not None:
                    raise RuntimeError("NMS exited before its primary API became ready")
                try:
                    get(PRIMARY, "/health/ready")
                    break
                except OSError:
                    time.sleep(.05)
            else:
                raise RuntimeError("NMS primary API did not become ready within 10 seconds")
            assert get(PRIMARY, "/api/v1/targets") == [], "Test must have no LAN targets"
            marker = "native-loopback-" + uuid.uuid4().hex
            store.append({
                "source_ip": "127.0.0.2", "program": "loopback-test",
                "message": marker, "severity": "err", "transport": "test",
            })
            assert store.status()["preview_pending"] == 1
            raw_before = {path.name: path.read_bytes() for path in store.raw.iterdir()}
            try:
                accepted = forward_once(store)
            except OSError as error:
                assert store.status()["preview_pending"] == 1
                assert raw_before == {
                    path.name: path.read_bytes() for path in store.raw.iterdir()
                }
                raise RuntimeError(
                    "IP-bound NMS is reachable, but validated Syslog preview cannot "
                    f"reach {PREVIEW}; durable preview and raw record were retained"
                ) from error
            assert accepted == 1 and store.status()["preview_pending"] == 0
            assert raw_before == {path.name: path.read_bytes() for path in store.raw.iterdir()}
            rows = get(PRIMARY, "/api/v1/logs?contains=" + marker)
            assert len(rows) == 1 and rows[0]["message"] == marker
            assert rows[0]["node"] == "Loopback-Test" and rows[0]["level"] == "error"
            assert get(PREVIEW, "/api/v1/logs?contains=" + marker) == rows, (
                "Primary and preview listeners must share the same NMS state"
            )
            assert config_path.read_bytes() == original_config, "Configured bind must be preserved"
            assert get(PRIMARY, "/api/v1/config")["server"]["bind"] == "127.0.0.2:8210"
            assert get(PREVIEW, "/api/v1/config")["server"]["bind"] == "127.0.0.2:8210"
            print("Native IP-bound NMS + validated localhost preview + SQLite/raw retention: OK")
        except Exception as error:
            out.flush()
            raise RuntimeError(f"{error}\nNMS output:\n{(base / 'native.out').read_text()}") from error
        finally:
            if proc is not None and proc.poll() is None:
                proc.terminate()
                try:
                    proc.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait(timeout=5)
            out.close()
            store.db.close()


if __name__ == "__main__":
    main()
