#!/usr/bin/env python3
"""Opt-in integration test for Gateway telemetry -> Control Room -> GPS warning.

Run after `cargo build -p netcore-control-room` from the repository root:
    python system-backend/alert-service/tests/integration_control_room.py
    python system-backend/alert-service/tests/integration_control_room.py --control-room-binary /path/to/netcore-control-room

Uses a real isolated Control Room, a standard-library fake Gateway WebSocket and
an in-memory SDS Router double. All sockets are loopback; federation and NINA are
disabled. No existing service, external API, gateway or radio is contacted.
"""
import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import socket
import struct
import subprocess
import sys
import tempfile
import threading
import time
from urllib.error import URLError
from urllib.request import ProxyHandler, build_opener, install_opener

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from service import AlertService, HttpClient
from store import iso, timestamp

NODE_ID = "isolated-control-room-tbs"
ISSI = 4_010_002
IDENTITY = {
    "node_id": NODE_ID, "station_name": "Isolated integration test", "site": None,
    "stack_version": "integration-test", "mcc": 262, "mnc": 1, "location_area": 1,
    "main_carrier": 1, "secondary_carrier": None, "colour_code": 1, "system_code": 1,
}
CAPABILITIES = {key: True for key in (
    "telemetry", "command", "sds", "raw_sds", "dgna", "kick_ms", "emergency_clear",
    "live_sds", "service_control", "brew_bridge", "dual_carrier",
)}
CAPABILITIES.update({key: False for key in (
    "packet_data", "legacy_wap_sds", "multi_pdch", "subscriber_policy", "group_policy",
    "call_control", "call_restore_context", "media_bridge",
)})


def gateway_snapshot():
    # Shapes match GatewaySnapshot/NodeSnapshot in node-gateway/src/state.rs and
    # the shared ControlRoomNodeIdentity/Capabilities protocol structs.
    now = iso()
    return {"kind": "snapshot", "snapshot": {
        "status": {
            "service": "netcore-node-gateway", "started_at": now, "security_mode": "open_lab",
            "warning": "ISOLATED TEST ONLY", "remote_management_enabled": False,
            "node_path": "/ws/node", "backend_path": "/ws/backend", "known_nodes": 1,
            "connected_nodes": 1, "stale_nodes": 0, "backend_clients": 1,
            "monitored_services": 0, "available_services": 0, "degraded_services": 0,
            "unavailable_services": 0, "total_node_sessions": 1, "total_node_messages": 0,
            "total_commands": 0, "total_media_frames": 0, "total_disconnects": 0,
        },
        "nodes": [{
            "node_id": NODE_ID, "session_id": "isolated-session", "peer": "127.0.0.1:12345",
            "connected": True, "stale": False, "connected_at": now, "last_seen": now,
            "disconnected_at": None, "disconnect_reason": None, "heartbeat_seq": 0,
            "message_count": 0, "telemetry_count": 0, "control_ack_count": 0,
            "control_response_count": 0, "media_frame_count": 0, "error_count": 0,
            "last_message_kind": "hello", "last_telemetry": None,
            "identity": IDENTITY, "capabilities": CAPABILITIES,
        }],
        "core_services": {"revision": 0, "generated_at": now, "services": []},
    }}


def wait_for(description, callback, timeout=10):
    deadline = time.monotonic() + timeout
    last = None
    while time.monotonic() < deadline:
        try:
            last = callback()
            if last:
                return last
        except (URLError, OSError) as error:
            last = str(error)
        time.sleep(0.05)
    raise AssertionError(f"Timed out waiting for {description}; last result: {last!r}")


class FakeGateway:
    """One loopback WebSocket connection using the real Gateway JSON envelopes."""
    def __init__(self):
        self.listener = socket.socket()
        if os.name == "nt":
            self.listener.setsockopt(socket.SOL_SOCKET, socket.SO_EXCLUSIVEADDRUSE, 1)
        self.listener.bind(("127.0.0.1", 0))
        self.listener.listen(1)
        self.port = self.listener.getsockname()[1]
        self.connection = None
        self.send_lock = threading.Lock()
        self.ready = threading.Event()
        self.stopping = threading.Event()
        self.failure = None
        self.client_requests = []
        self.seq = 0
        self.thread = threading.Thread(target=self._run, daemon=True)
        self.thread.start()

    @staticmethod
    def _read_exact(connection, count):
        result = bytearray()
        while len(result) < count:
            part = connection.recv(count - len(result))
            if not part:
                raise EOFError("WebSocket peer closed")
            result.extend(part)
        return bytes(result)

    def _send_frame(self, payload, opcode=1):
        size = len(payload)
        header = bytes([0x80 | opcode])
        if size < 126:
            header += bytes([size])
        elif size < 65536:
            header += bytes([126]) + struct.pack("!H", size)
        else:
            header += bytes([127]) + struct.pack("!Q", size)
        with self.send_lock:
            self.connection.sendall(header + payload)

    def send(self, event):
        self._send_frame(json.dumps(event).encode("utf-8"))

    def _run(self):
        try:
            self.connection, _ = self.listener.accept()
            self.connection.settimeout(5)
            request = bytearray()
            while not request.endswith(b"\r\n\r\n"):
                request.extend(self._read_exact(self.connection, 1))
                if len(request) > 16384:
                    raise AssertionError("Oversized WebSocket handshake")
            lines = request.decode("ascii").split("\r\n")
            assert lines[0].split()[:2] == ["GET", "/ws/backend"], lines[0]
            headers = {key.lower(): value.strip() for key, value in
                       (line.split(":", 1) for line in lines[1:] if ":" in line)}
            protocol = headers.get("sec-websocket-protocol")
            assert protocol == "netcore-node-gateway-backend-v1", protocol
            digest = hashlib.sha1((headers["sec-websocket-key"] +
                                   "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").encode()).digest()
            response = ("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\n"
                        "Connection: Upgrade\r\nSec-WebSocket-Protocol: " + protocol +
                        "\r\nSec-WebSocket-Accept: " +
                        base64.b64encode(digest).decode() + "\r\n\r\n")
            self.connection.sendall(response.encode("ascii"))
            self.connection.settimeout(None)
            self.send(gateway_snapshot())
            self.ready.set()
            while not self.stopping.is_set():
                first, second = self._read_exact(self.connection, 2)
                assert first & 0x80, "Fragmented client messages are not expected in this fixture"
                assert second & 0x80, "Client WebSocket frames must be masked"
                size = second & 0x7f
                if size == 126:
                    size = struct.unpack("!H", self._read_exact(self.connection, 2))[0]
                elif size == 127:
                    size = struct.unpack("!Q", self._read_exact(self.connection, 8))[0]
                assert size <= 65536, "Unexpected large client frame"
                mask = self._read_exact(self.connection, 4)
                raw = self._read_exact(self.connection, size)
                payload = bytes(value ^ mask[index % 4] for index, value in enumerate(raw))
                opcode = first & 0xf
                if opcode == 8:
                    return
                if opcode == 9:
                    self._send_frame(payload, opcode=10)
                elif opcode == 1:
                    message = json.loads(payload)
                    self.client_requests.append(message)
                    assert message["kind"] in {"ping", "subscribe"}, message
                    self.send({"kind": "action_result", "request_id": message.get("request_id"),
                               "command_id": None, "ok": True, "message": "pong"})
        except Exception as error:
            if not self.stopping.is_set():
                self.failure = error
            self.ready.set()

    def hello(self):
        self.send({"kind": "node_message", "node_id": NODE_ID, "message": {
            "kind": "hello", "hello": {"protocol_version": "netcore-control-room-node-v1",
                                        "node": IDENTITY, "capabilities": CAPABILITIES,
                                        "started_at": iso()}}})

    def telemetry(self, event):
        self.seq += 1
        self.send({"kind": "node_message", "node_id": NODE_ID, "message": {
            "kind": "telemetry", "envelope": {
                "node_id": NODE_ID, "seq": self.seq, "timestamp": iso(), "event": event}}})

    def gps(self):
        self.telemetry({"SdsLog": {"direction": "rx", "source_issi": ISSI,
                                  "dest_issi": 4_010_001, "is_group": False, "protocol_id": 10,
                                  "text": "LIP position: 50.0, 8.0"}})

    def disconnect(self):
        self.stopping.set()
        self.listener.close()
        if self.connection is not None:
            try:
                self.connection.shutdown(socket.SHUT_RDWR)
            except OSError:
                pass
            self.connection.close()
        self.thread.join(timeout=5)


class FakeRouter:
    def __init__(self):
        self.posts = []

    def request(self, method, path, data=None):
        if (method, path) == ("GET", "/api/v1/status"):
            return {"durable_idempotency": True, "at_most_once": True}
        if (method, path) == ("POST", "/api/v1/messages"):
            self.posts.append(data)
            return {"id": f"isolated-message-{len(self.posts)}", "state": "delivered"}
        if method == "GET" and path.startswith("/api/v1/messages/"):
            return {"id": path.rsplit("/", 1)[1], "state": "delivered"}
        raise AssertionError(f"Unexpected Router request: {method} {path}")


def run_smoke(binary):
    # Keep even a host's proxy environment from routing loopback test traffic out.
    install_opener(build_opener(ProxyHandler({})))
    with tempfile.TemporaryDirectory(prefix="netcore-alert-control-smoke-") as temp:
        directory = Path(temp)
        gateway, service, process = FakeGateway(), None, None
        with socket.socket() as reservation:
            reservation.bind(("127.0.0.1", 0))
            port = reservation.getsockname()[1]
        client = HttpClient(f"http://127.0.0.1:{port}", timeout=2)
        config_path = directory / "control-room.toml"
        config_path.write_text(
            'services = []\n'
            f'[server]\nbind = "127.0.0.1:{port}"\n'
            '[persistence]\nenabled = false\n[auth]\nenabled = false\n'
            '[federation]\nenabled = false\n'
            '[operations]\n'
            f'state_path = {json.dumps(str(directory / "operations.json"))}\n'
            f'backup_path = {json.dumps(str(directory / "operations.json.bak"))}\n'
            '[node_gateway]\nenabled = true\n'
            f'url = "ws://127.0.0.1:{gateway.port}/ws/backend"\n'
            'reconnect_secs = 1\ntimeout_secs = 3\nstale_after_secs = 30\n', encoding="utf-8")
        with tempfile.TemporaryFile(mode="w+b") as log:
            try:
                process = subprocess.Popen([str(binary), "--config", str(config_path)], cwd=directory,
                                           stdin=subprocess.DEVNULL, stdout=log, stderr=subprocess.STDOUT,
                                           creationflags=subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0)
                wait_for("Control Room HTTP readiness", lambda: client.request("GET", "/health/live"))
                assert gateway.ready.wait(timeout=10), "Control Room did not subscribe to the fake Gateway"
                assert gateway.failure is None, gateway.failure
                wait_for("initial Gateway snapshot", lambda: len(client.request("GET", "/api/nodes")) == 1)
                assert client.request("GET", "/api/subscribers?online=true")["subscribers"] == []
                gateway.hello()
                gateway.telemetry({"MsRegistration": {"issi": ISSI}})
                gateway.gps()
                rows = wait_for("GPS telemetry in real Control Room", lambda: [row for row in
                                client.request("GET", "/api/subscribers?online=true")["subscribers"]
                                if row["issi"] == ISSI and row.get("last_location")])
                assert rows[0]["node_id"] == NODE_ID
                assert rows[0]["last_location"]["latitude"] == 50.0
                assert rows[0]["last_location"]["longitude"] == 8.0
                router = FakeRouter()
                service = AlertService({
                    "storage": {"database": ":memory:"}, "server": {},
                    "netcore": {"control_room_url": client.base_url, "sds_router_url": "http://127.0.0.1:1",
                                "source_issi": 4_010_001},
                    "nina": {"enabled": False},
                    "delivery": {"enabled": True, "ttl_seconds": 300, "max_text_length": 120},
                }, control=client, router=router)

                def create_alert(title):
                    return service.create_manual({"title": title, "description": "No radio connected",
                                                  "latitude": 50.0, "longitude": 8.0, "radius_m": 1000,
                                                  "expires_at": iso(timestamp() + 600), "severity": "Moderate"})

                create_alert("Isolated Gateway integration")
                service.tick()
                assert len(router.posts) == 1, service.snapshot()
                assert router.posts[0]["force_nodes"] == [NODE_ID]
                assert router.posts[0]["dest_issi"] == ISSI
                assert router.posts[0]["at_most_once"] is True
                service.tick()
                assert service.store.deliveries()[0]["state"] == "accepted"
                gateway.telemetry({"MsDeregistration": {"issi": ISSI}})
                wait_for("subscriber logout", lambda: not client.request("GET", "/api/subscribers?online=true")["subscribers"])
                service.tick()
                gateway.telemetry({"MsRegistration": {"issi": ISSI}})
                gateway.gps()
                wait_for("subscriber relogin", lambda: client.request("GET", "/api/subscribers?online=true")["subscribers"])
                service.tick()
                assert len(router.posts) == 1, "Repeated ticks or relogin must not duplicate warning"
                gateway.disconnect()
                def disconnected_node():
                    nodes = client.request("GET", "/api/nodes")
                    return len(nodes) == 1 and nodes[0]["connected"] is False and nodes[0]["transport_connected"] is False
                wait_for("offline state after Gateway disconnect", disconnected_node)
                create_alert("Must not send while Gateway is offline")
                service.tick()
                assert service.devices == []
                assert len(router.posts) == 1, "Stale Gateway state must not dispatch a new warning"
                assert gateway.failure is None, gateway.failure
                print("PASS: real Control Room receives Gateway snapshot, registration and GPS; "
                      "AlertService submits one SDS request to the Router double for the serving TBS; "
                      "relogin does not repeat; Gateway disconnect blocks new warnings. "
                      "All traffic isolated to loopback; no radio delivery tested.")
            except Exception:
                log.seek(0)
                print(log.read().decode(errors="replace"), file=sys.stderr)
                raise
            finally:
                gateway.disconnect()
                if service is not None:
                    service.store.close()
                if process is not None and process.poll() is None:
                    process.terminate()
                    try:
                        process.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        process.kill()
                        process.wait(timeout=5)


def main():
    repository = Path(__file__).resolve().parents[3]
    default_binary = repository / "target" / "debug" / ("netcore-control-room.exe" if os.name == "nt" else "netcore-control-room")
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--control-room-binary", type=Path, default=default_binary)
    args = parser.parse_args()
    binary = args.control_room_binary.resolve()
    if not binary.is_file():
        parser.error(f"Control Room not built: {binary}. Run cargo build -p netcore-control-room or pass --control-room-binary.")
    run_smoke(binary)


if __name__ == "__main__":
    main()
