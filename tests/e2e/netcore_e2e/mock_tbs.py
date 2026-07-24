from __future__ import annotations

import json
import threading
import time
from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Any

from .websocket import WebSocketClient, WebSocketError

PROTOCOL = "netcore-control-room-node-v1"


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="milliseconds").replace("+00:00", "Z")


@dataclass
class MockTbs:
    url: str
    node_id: str
    station_name: str = "NetCore E2E TBS"
    site: str = "e2e-lab"
    mcc: int = 1
    mnc: int = 333
    location_area: int = 1
    main_carrier: int = 720
    secondary_carrier: int | None = 721
    colour_code: int = 1
    system_code: int = 1
    heartbeat_interval: float = 4.0
    ws: WebSocketClient | None = field(default=None, init=False)
    reader_thread: threading.Thread | None = field(default=None, init=False)
    heartbeat_thread: threading.Thread | None = field(default=None, init=False)
    stop_event: threading.Event = field(default_factory=threading.Event, init=False)
    hello_accepted: threading.Event = field(default_factory=threading.Event, init=False)
    errors: list[str] = field(default_factory=list, init=False)
    received_commands: list[dict[str, Any]] = field(default_factory=list, init=False)
    downlink_media_frames: int = field(default=0, init=False)
    core_services_snapshot: dict[str, Any] | None = field(default=None, init=False)
    seq: int = field(default=0, init=False)
    next_call_id: int = field(default=100, init=False)
    calls: dict[int, dict[str, Any]] = field(default_factory=dict, init=False)
    groups_by_issi: dict[int, set[int]] = field(default_factory=dict, init=False)

    def __enter__(self) -> "MockTbs":
        self.start()
        return self

    def __exit__(self, exc_type: Any, exc: Any, tb: Any) -> None:
        self.stop()

    def start(self) -> None:
        self.stop_event.clear()
        self.ws = WebSocketClient(self.url, subprotocol=PROTOCOL, timeout=8.0)
        self.ws.connect()
        self.send(
            {
                "kind": "hello",
                "hello": {
                    "protocol_version": PROTOCOL,
                    "node": {
                        "node_id": self.node_id,
                        "station_name": self.station_name,
                        "site": self.site,
                        "stack_version": "e2e-mock-1",
                        "mcc": self.mcc,
                        "mnc": self.mnc,
                        "location_area": self.location_area,
                        "main_carrier": self.main_carrier,
                        "secondary_carrier": self.secondary_carrier,
                        "colour_code": self.colour_code,
                        "system_code": self.system_code,
                    },
                    "capabilities": {
                        "telemetry": True,
                        "command": True,
                        "sds": True,
                        "raw_sds": True,
                        "dgna": True,
                        "kick_ms": True,
                        "emergency_clear": True,
                        "live_sds": True,
                        "service_control": True,
                        "brew_bridge": False,
                        "dual_carrier": self.secondary_carrier is not None,
                        "packet_data": True,
                        "legacy_wap_sds": True,
                        "multi_pdch": True,
                        "subscriber_policy": True,
                        "group_policy": True,
                        "call_control": True,
                        "call_restore_context": True,
                        "media_bridge": True,
                    },
                    "started_at": now_iso(),
                },
            }
        )
        self.reader_thread = threading.Thread(target=self._reader, name=f"mock-tbs-reader-{self.node_id}", daemon=True)
        self.reader_thread.start()
        if not self.hello_accepted.wait(5.0):
            self.stop()
            raise WebSocketError(f"Node Gateway did not accept hello for {self.node_id}; errors={self.errors}")
        self.heartbeat_thread = threading.Thread(target=self._heartbeats, name=f"mock-tbs-heartbeat-{self.node_id}", daemon=True)
        self.heartbeat_thread.start()

    def stop(self) -> None:
        self.stop_event.set()
        if self.ws is not None:
            self.ws.close()
        for thread in (self.reader_thread, self.heartbeat_thread):
            if thread and thread.is_alive():
                thread.join(timeout=1.0)
        self.ws = None

    def send(self, message: dict[str, Any]) -> None:
        if self.ws is None:
            raise WebSocketError("mock TBS is not connected")
        self.ws.send_binary(json.dumps(message, separators=(",", ":")).encode("utf-8"))

    def heartbeat(self) -> None:
        self.seq += 1
        self.send(
            {
                "kind": "heartbeat",
                "heartbeat": {
                    "node_id": self.node_id,
                    "seq": self.seq,
                    "timestamp": now_iso(),
                    "connected": True,
                },
            }
        )

    def telemetry(self, event: dict[str, Any]) -> None:
        self.seq += 1
        self.send(
            {
                "kind": "telemetry",
                "envelope": {
                    "node_id": self.node_id,
                    "seq": self.seq,
                    "timestamp": now_iso(),
                    "event": event,
                },
            }
        )

    def register(self, issi: int) -> None:
        self.telemetry({"MsRegistration": {"issi": issi}})

    def deregister(self, issi: int) -> None:
        self.telemetry({"MsDeregistration": {"issi": issi}})

    def attach_groups(self, issi: int, gssis: list[int]) -> None:
        current = self.groups_by_issi.setdefault(issi, set())
        current.update(gssis)
        self.telemetry({"MsGroupAttach": {"issi": issi, "gssis": gssis}})
        self.telemetry({"MsGroupsSnapshot": {"issi": issi, "gssis": sorted(current)}})

    def detach_groups(self, issi: int, gssis: list[int]) -> None:
        current = self.groups_by_issi.setdefault(issi, set())
        current.difference_update(gssis)
        self.telemetry({"MsGroupDetach": {"issi": issi, "gssis": gssis}})
        self.telemetry({"MsGroupsSnapshot": {"issi": issi, "gssis": sorted(current)}})

    def start_group_call(self, *, call_id: int, gssi: int, caller_issi: int, ts: int = 1, priority: int = 3) -> None:
        self.calls[call_id] = {"kind": "group", "gssi": gssi, "source_issi": caller_issi, "ts": ts}
        self.telemetry(
            {
                "GroupCallStarted": {
                    "call_id": call_id,
                    "gssi": gssi,
                    "caller_issi": caller_issi,
                    "ts": ts,
                    "carrier_num": self.main_carrier,
                    "priority": priority,
                    "source": "local",
                }
            }
        )

    def group_speaker(self, *, call_id: int, gssi: int, speaker_issi: int) -> None:
        self.telemetry(
            {
                "GroupCallSpeakerChanged": {
                    "call_id": call_id,
                    "gssi": gssi,
                    "speaker_issi": speaker_issi,
                    "source": "local",
                }
            }
        )

    def end_group_call(self, *, call_id: int, gssi: int) -> None:
        self.telemetry({"GroupCallEnded": {"call_id": call_id, "gssi": gssi}})
        self.calls.pop(call_id, None)

    def media_frame(self, *, sequence: int, logical_ts: int = 1, payload: bytes | None = None) -> None:
        payload = payload if payload is not None else bytes((index + sequence) % 256 for index in range(35))
        if len(payload) != 35:
            raise ValueError("TETRA ACELP frame must contain exactly 35 bytes")
        self.send(
            {
                "kind": "media_frame",
                "frame": {
                    "node_id": self.node_id,
                    "sequence": sequence,
                    "timestamp": now_iso(),
                    "carrier_num": self.main_carrier,
                    "logical_ts": logical_ts,
                    "codec": "tetra_acelp0",
                    "payload": list(payload),
                },
            }
        )

    def _heartbeats(self) -> None:
        while not self.stop_event.wait(self.heartbeat_interval):
            try:
                self.heartbeat()
            except BaseException as error:
                self.errors.append(f"heartbeat: {error}")
                return

    def _reader(self) -> None:
        assert self.ws is not None
        while not self.stop_event.is_set():
            try:
                frame = self.ws.recv()
            except (OSError, WebSocketError) as error:
                if not self.stop_event.is_set():
                    self.errors.append(f"reader: {error}")
                return
            if frame.opcode == 0x8:
                return
            if frame.opcode == 0x9:
                self.ws.send_pong(frame.payload)
                continue
            if frame.opcode not in {0x1, 0x2}:
                continue
            try:
                message = json.loads(frame.payload)
                self._handle_downlink(message)
            except BaseException as error:
                self.errors.append(f"decode/handle: {error}; payload={frame.payload[:300]!r}")

    def _handle_downlink(self, message: dict[str, Any]) -> None:
        kind = message.get("kind")
        if kind == "hello_ack":
            if message.get("accepted"):
                self.hello_accepted.set()
            else:
                self.errors.append(f"hello rejected: {message.get('message')}")
            return
        if kind == "ping":
            self.heartbeat()
            return
        if kind == "media_frame":
            self.downlink_media_frames += 1
            return
        if kind == "core_services":
            self.core_services_snapshot = message.get("snapshot") or {}
            return
        if kind != "command":
            return
        envelope = message.get("envelope") or {}
        command = envelope.get("command") or {}
        self.received_commands.append(envelope)
        self._send_ack(envelope)
        self._send_command_response(envelope, command)

    def _send_ack(self, envelope: dict[str, Any]) -> None:
        self.send(
            {
                "kind": "control_ack",
                "ack": {
                    "command_id": envelope.get("command_id", "unknown"),
                    "node_id": self.node_id,
                    "accepted": True,
                    "target_entity": None,
                    "message": "accepted by netcore E2E mock TBS",
                    "timestamp": now_iso(),
                },
            }
        )

    def _send_response(self, envelope: dict[str, Any], response: dict[str, Any]) -> None:
        self.send(
            {
                "kind": "control_response",
                "envelope": {
                    "command_id": envelope.get("command_id"),
                    "node_id": self.node_id,
                    "target_entity": None,
                    "timestamp": now_iso(),
                    "response": response,
                },
            }
        )

    def _send_command_response(self, envelope: dict[str, Any], command: dict[str, Any]) -> None:
        if not command:
            return
        variant, payload = next(iter(command.items()))
        payload = payload or {}
        handle = int(payload.get("handle", 0))
        if variant == "SubscriberAccessPolicyApply":
            self._send_response(
                envelope,
                {"SubscriberAccessPolicyApplied": {
                    "handle": handle,
                    "revision": int(payload.get("revision", 0)),
                    "success": True,
                    "allow_all": bool(payload.get("allow_all", False)),
                    "allowed_count": len(payload.get("allowed_issis", [])),
                    "disconnected_count": 0,
                    "message": "policy applied by E2E mock",
                }},
            )
        elif variant == "GroupAccessPolicyApply":
            self._send_response(
                envelope,
                {"GroupAccessPolicyApplied": {
                    "handle": handle,
                    "revision": int(payload.get("revision", 0)),
                    "success": True,
                    "group_count": len(payload.get("groups", [])),
                    "membership_count": len(payload.get("memberships", [])),
                    "attached_count": 0,
                    "detached_count": 0,
                    "message": "group policy applied by E2E mock",
                }},
            )
        elif variant == "GroupDgnaApply":
            issi, gssi, attach = int(payload["issi"]), int(payload["gssi"]), bool(payload["attach"])
            if attach:
                self.attach_groups(issi, [gssi])
            else:
                self.detach_groups(issi, [gssi])
            self._send_response(
                envelope,
                {"GroupDgnaApplied": {
                    "handle": handle, "issi": issi, "gssi": gssi, "attach": attach,
                    "success": True, "message": "DGNA applied by E2E mock",
                }},
            )
        elif variant in {"SendSds", "SendRawSdsType4"}:
            self._send_response(envelope, {"SendSdsResponse": {"handle": handle, "success": True}})
        elif variant in {"DeliverSds", "SendStatus"}:
            source = int(payload.get("source_ssi", 0))
            dest = int(payload.get("dest_ssi", 0))
            self.telemetry({"SdsActivity": {"source_issi": source, "dest_issi": dest, "source": "network"}})
            self._send_response(
                envelope,
                {"SdsDeliveryResponse": {"handle": handle, "success": True, "message": "delivered by E2E mock"}},
            )
        elif variant in {"CallControlGroupStart", "CallControlIndividualStart"}:
            call_id = self.next_call_id
            self.next_call_id += 1
            ts = 1 + (call_id % 3)
            operation_id = str(payload.get("operation_id", "e2e"))
            if variant == "CallControlGroupStart":
                kind = "Group"
                self.start_group_call(
                    call_id=call_id,
                    gssi=int(payload["gssi"]),
                    caller_issi=int(payload["source_issi"]),
                    ts=ts,
                    priority=int(payload.get("priority", 0)),
                )
            else:
                kind = "Individual"
                self.calls[call_id] = {
                    "kind": "individual",
                    "calling_issi": int(payload["calling_issi"]),
                    "called_issi": int(payload["called_issi"]),
                    "ts": ts,
                }
                self.telemetry({"IndividualCallStarted": {
                    "call_id": call_id,
                    "calling_issi": int(payload["calling_issi"]),
                    "called_issi": int(payload["called_issi"]),
                    "simplex": bool(payload.get("simplex", True)),
                    "ts": ts,
                    "carrier_num": self.main_carrier,
                    "priority": int(payload.get("priority", 0)),
                    "source": "network",
                }})
            self._send_response(
                envelope,
                {"CallControlLegStarted": {
                    "handle": handle, "operation_id": operation_id, "kind": kind, "success": True,
                    "call_id": call_id, "timeslot": ts, "usage": 4, "floor_holder": None,
                    "message": "call leg started by E2E mock",
                }},
            )
        elif variant == "CallControlRelease":
            call_id = int(payload["call_id"])
            call = self.calls.pop(call_id, None)
            if call and call.get("kind") == "group":
                self.telemetry({"GroupCallEnded": {"call_id": call_id, "gssi": int(call["gssi"])}})
            elif call:
                self.telemetry({"IndividualCallEnded": {"call_id": call_id}})
            self._send_response(
                envelope,
                {"CallControlLegReleased": {"handle": handle, "call_id": call_id, "success": True, "message": "released by E2E mock"}},
            )
        elif variant in {"CallControlFloorRequest", "CallControlFloorRelease"}:
            call_id = int(payload["call_id"])
            floor_holder = int(payload.get("source_issi", 0)) if variant == "CallControlFloorRequest" else None
            call = self.calls.get(call_id)
            if call and call.get("kind") == "group" and floor_holder:
                self.group_speaker(call_id=call_id, gssi=int(call["gssi"]), speaker_issi=floor_holder)
            self._send_response(
                envelope,
                {"CallControlFloorChanged": {
                    "handle": handle, "call_id": call_id, "success": True,
                    "floor_holder": floor_holder, "queued_issi": None, "message": "floor changed by E2E mock",
                }},
            )
        elif variant in {"PacketDataContextDeactivate", "PacketDataContextModify", "PacketDataWake", "PacketDataEndOfData"}:
            self._send_response(
                envelope,
                {"PacketDataActionResult": {
                    "handle": handle,
                    "action": variant,
                    "issi": int(payload.get("issi", 0)),
                    "nsapi": payload.get("nsapi"),
                    "success": True,
                    "message": "packet action applied by E2E mock",
                }},
            )
        elif variant == "KickMs":
            issi = int(payload["issi"])
            self.deregister(issi)
            self._send_response(envelope, {"KickMsResponse": {"issi": issi, "success": True}})
