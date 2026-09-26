#!/usr/bin/env python3
"""Opt-in HTTP smoke test with an isolated, locally built SDS Router.

Run after `cargo build -p netcore-sds-router` from the repository root:
    python system-backend/alert-service/tests/integration_router.py
    python system-backend/alert-service/tests/integration_router.py --router-binary /path/to/netcore-sds-router

Uses only temporary state, a fake Control Room and loopback sockets. The gateway
port stays bound without listening, so the process cannot contact any TBS. No
existing service is stopped; only child processes created here are terminated.
This file is intentionally outside unittest discovery because it needs Rust.
"""
import argparse
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import time
from urllib.error import HTTPError, URLError

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from service import AlertService, HttpClient
from store import iso, timestamp


class FakeControlRoom:
    def request(self, method, path, data=None):
        if method != "GET":
            raise AssertionError("The alert service must only read the Control Room")
        if path == "/api/nodes":
            return [{"node_id": "isolated-smoke-tbs", "connected": True,
                     "transport_connected": True, "last_seen": iso()}]
        if path == "/api/subscribers?online=true":
            return {"subscribers": [{"issi": 4_010_002, "node_id": "isolated-smoke-tbs",
                                      "online": True, "last_location": {
                                          "latitude": 50.0, "longitude": 8.0,
                                          "updated_at": iso()}}]}
        raise AssertionError(f"Unexpected Control Room request: {path}")


class IsolatedRouter:
    def __init__(self, binary, directory, gateway_port):
        with socket.socket() as port_reservation:
            port_reservation.bind(("127.0.0.1", 0))
            port = port_reservation.getsockname()[1]
        self.binary = binary
        self.directory = directory
        self.config = directory / "sds-router.toml"
        self.log = tempfile.TemporaryFile(mode="w+b")
        self.process = None
        self.client = HttpClient(f"http://127.0.0.1:{port}", timeout=2)
        self.config.write_text(
            f'[server]\nbind = "127.0.0.1:{port}"\n'
            f'[node_gateway]\nurl = "ws://127.0.0.1:{gateway_port}/ws/backend"\nreconnect_secs = 1\n'
            f'[storage]\ndatabase_path = {json.dumps(str(directory / "messages.json"))}\n'
            f'backup_path = {json.dumps(str(directory / "messages.json.bak"))}\n', encoding="utf-8")

    def start(self):
        if self.process is not None:
            raise RuntimeError("Owned router process has not been stopped")
        self.process = subprocess.Popen(
            [str(self.binary), "--config", str(self.config)], cwd=self.directory,
            stdin=subprocess.DEVNULL, stdout=self.log, stderr=subprocess.STDOUT,
            creationflags=subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0)
        deadline = time.monotonic() + 15
        while time.monotonic() < deadline:
            if self.process.poll() is not None:
                break
            try:
                if self.client.request("GET", "/health/live").get("status") == "live":
                    return
            except (URLError, OSError):
                time.sleep(0.05)
        self.log.seek(0)
        raise AssertionError("Isolated SDS Router did not become ready:\n" + self.log.read().decode(errors="replace"))

    def stop(self):
        if self.process is not None:
            if self.process.poll() is None:
                self.process.terminate()
                try:
                    self.process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    self.process.kill()
                    self.process.wait(timeout=5)
            self.process = None

    def restart(self):
        self.stop()
        self.start()

    def close(self):
        self.stop()
        self.log.close()


def expect_error(client, status, method, path, body=None, contains=None):
    try:
        client.request(method, path, body)
    except HTTPError as error:
        payload = error.read().decode()
        if error.code != status or (contains and contains not in payload):
            raise AssertionError(f"Expected HTTP {status} containing {contains!r}; got {error.code}: {payload}") from error
    else:
        raise AssertionError(f"Expected HTTP {status}: {method} {path}")


def run_smoke(binary):
    with tempfile.TemporaryDirectory(prefix="netcore-alert-router-smoke-") as temp, socket.socket() as blocked_gateway:
        # Do not listen: all connections fail, and no unrelated process can bind
        # this port while the test is running.
        if os.name == "nt":
            blocked_gateway.setsockopt(socket.SOL_SOCKET, socket.SO_EXCLUSIVEADDRUSE, 1)
        blocked_gateway.bind(("127.0.0.1", 0))
        router = IsolatedRouter(binary, Path(temp), blocked_gateway.getsockname()[1])
        service = None
        try:
            router.start()
            client = router.client
            status = client.request("GET", "/api/v1/status")
            assert status["durable_idempotency"] is True and status["at_most_once"] is True
            assert status["node_gateway_connected"] is False
            config = {
                "storage": {"database": ":memory:"}, "server": {},
                "netcore": {"control_room_url": "http://127.0.0.1:1", "sds_router_url": client.base_url,
                            "source_issi": 4_010_001},
                "nina": {"enabled": False},
                "delivery": {"enabled": True, "ttl_seconds": 300, "max_text_length": 120},
            }
            service = AlertService(config, control=FakeControlRoom(), router=client)
            alert = service.create_manual({"title": "Isolierter HTTP-Test", "description": "Keine Funkverbindung",
                                           "latitude": 50.0, "longitude": 8.0, "radius_m": 1000,
                                           "expires_at": iso(timestamp() + 600), "severity": "Moderate"})
            service.tick()
            rows = service.store.deliveries()
            assert len(rows) == 1 and rows[0]["state"] == "submitted", rows
            request = json.loads(rows[0]["request_json"])
            assert request["force_nodes"] == ["isolated-smoke-tbs"]
            assert request["at_most_once"] is True and request["expires_at"]
            message_id = rows[0]["router_id"]
            message_path = "/api/v1/messages/" + message_id
            key_path = "/api/v1/idempotency/" + request["idempotency_key"]
            message = client.request("GET", message_path)
            assert message["state"] == "queued" and message["id"] == message_id
            assert abs(timestamp(message["expires_at"]) - timestamp(request["expires_at"])) < 0.001
            assert len(message["delivery_legs"]) == 1 and message["delivery_legs"][0]["attempts"] == 0
            duplicate = client.request("POST", "/api/v1/messages", request)
            assert duplicate["id"] == message_id
            lookup = client.request("GET", key_path)
            assert lookup["message_id"] == message_id and lookup["retained"] is True
            expect_error(client, 409, "POST", "/api/v1/messages", {**request, "text": "changed"}, "idempotency_key_conflict")
            expect_error(client, 409, "POST", message_path + "/retry", {})
            expect_error(client, 409, "POST", message_path + "/requeue", {})

            # Exercise the service's real cancellation/reconciliation code.
            service.delete_manual(alert["id"])
            service.tick()
            assert service.store.deliveries()[0]["state"] == "cancelled"
            assert client.request("GET", message_path)["state"] == "cancelled"
            router.restart()
            assert client.request("GET", key_path)["state"] == "cancelled"
            assert client.request("POST", "/api/v1/messages", request)["id"] == message_id
            client.request("DELETE", message_path)
            router.restart()
            tombstone = client.request("GET", key_path)
            assert tombstone["message_id"] == message_id and tombstone["retained"] is False
            expect_error(client, 409, "POST", "/api/v1/messages", request, "idempotency_key_already_used")
            expect_error(client, 404, "GET", message_path)
            expired = {**request, "idempotency_key": "alert:expired-smoke", "expires_at": iso(timestamp() - 60)}
            expect_error(client, 409, "POST", "/api/v1/messages", expired, "message_expired")
            expect_error(client, 404, "GET", "/api/v1/idempotency/alert:expired-smoke")
            status = client.request("GET", "/api/v1/status")
            assert status["messages_total"] == 0 and status["node_gateway_connected"] is False
            # A two-byte status previously panicked while extracting a text
            # reference. The process stayed live, but every state API lost its
            # HTTP response because the shared mutex was poisoned.
            short_messages = [
                {"sds_type": 0, "status_code": 1},
                {"sds_type": 1, "payload_hex": "0001"},
                {"sds_type": 4, "payload_hex": "82"},
            ]
            for index, payload in enumerate(short_messages, 1):
                message = client.request("POST", "/api/v1/messages", {
                    "source_issi": 4_010_001, "dest_issi": 4_010_002, **payload,
                })
                assert message["message_reference"] is None
                assert client.request("GET", "/health/live")["status"] == "live"
                assert client.request("GET", "/api/v1/status")["messages_total"] == index
                assert client.request("GET", "/api/v1/nodes") == []
                # No gateway is connected in this test: readiness must answer
                # with a valid HTTP 503, rather than closing the connection.
                expect_error(client, 503, "GET", "/health/ready", contains="node_gateway_connected")
            print("PASS: real AlertService/SDS HTTP submission, replay/conflict, absolute deadline, cancellation, two restarts, durable tombstone, short SDS/status without API failure; no gateway or radio connected.")
        finally:
            if service is not None:
                service.store.close()
            router.close()


def main():
    repository = Path(__file__).resolve().parents[3]
    default_binary = repository / "target" / "debug" / ("netcore-sds-router.exe" if os.name == "nt" else "netcore-sds-router")
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--router-binary", type=Path, default=default_binary)
    args = parser.parse_args()
    binary = args.router_binary.resolve()
    if not binary.is_file():
        parser.error(f"SDS Router not built: {binary}. Run cargo build -p netcore-sds-router or pass --router-binary.")
    run_smoke(binary)


if __name__ == "__main__":
    main()
