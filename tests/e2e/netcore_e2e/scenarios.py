from __future__ import annotations

import json
import time
from typing import Any, Callable

from .context import E2EContext
from .http import HttpFailure, query_url
from .wait import wait_for


def _as_list(value: Any) -> list[Any]:
    if isinstance(value, list):
        return value
    if isinstance(value, dict):
        for key in ("items", "services", "records", "results"):
            if isinstance(value.get(key), list):
                return value[key]
    return []


def _security_header(headers: dict[str, str]) -> str:
    return headers.get("x-netcore-security-mode", "").lower().replace("_", "-")


def scenario_contracts(ctx: E2EContext) -> None:
    scenario = "contracts"
    for service in ctx.inventory.services:
        base = service.base_url

        def live(service_name: str = service.name, base_url: str = base) -> dict[str, Any]:
            response = ctx.client.get(base_url + "/health/live")
            return {"status": response.status, "elapsed_ms": response.elapsed_ms, "body": response.text()[:400]}

        ctx.check(f"{service.name}: liveness", live, scenario=scenario, service=service.name)

        def ready(service_name: str = service.name, base_url: str = base) -> dict[str, Any]:
            expected = (200,) if ctx.strict_ready else (200, 503)
            response = ctx.client.get(base_url + "/health/ready", expected=expected)
            if ctx.strict_ready and response.status != 200:
                raise AssertionError(f"{service_name} is not ready")
            return {"status": response.status, "elapsed_ms": response.elapsed_ms, "body": response.text()[:600]}

        ctx.check(f"{service.name}: readiness", ready, scenario=scenario, service=service.name)

        def status(base_url: str = base) -> dict[str, Any]:
            response = ctx.client.get(base_url + "/api/v1/status")
            value = response.json()
            if not isinstance(value, dict):
                raise AssertionError("status endpoint did not return a JSON object")
            return {"status": value}

        ctx.check(f"{service.name}: status contract", status, scenario=scenario, service=service.name)

        def openapi(base_url: str = base) -> dict[str, Any]:
            response = ctx.client.get(base_url + "/openapi.json")
            value = response.json()
            if not isinstance(value, dict) or not str(value.get("openapi", "")).startswith("3."):
                raise AssertionError("OpenAPI document is missing or not version 3.x")
            if not isinstance(value.get("paths"), dict) or not value["paths"]:
                raise AssertionError("OpenAPI document has no paths")
            return {"title": value.get("info", {}).get("title"), "paths": len(value["paths"])}

        ctx.check(f"{service.name}: OpenAPI", openapi, scenario=scenario, service=service.name)

        def metrics(base_url: str = base) -> dict[str, Any]:
            response = ctx.client.get(base_url + "/metrics")
            text = response.text()
            if not text.strip() or "netcore" not in text.lower():
                raise AssertionError("metrics endpoint returned no NetCore metric")
            return {"bytes": len(response.body), "sample": text.splitlines()[:8]}

        ctx.check(f"{service.name}: metrics", metrics, scenario=scenario, service=service.name)

        def webui(base_url: str = base) -> dict[str, Any]:
            response = ctx.client.get(base_url + "/")
            content_type = response.headers.get("content-type", "")
            if "html" not in content_type.lower():
                raise AssertionError(f"WebUI root is not HTML: {content_type}")
            mode = _security_header(response.headers)
            if mode and mode != "open-lab":
                raise AssertionError(f"unexpected security mode header: {mode}")
            return {"content_type": content_type, "security_header": mode, "bytes": len(response.body)}

        ctx.check(f"{service.name}: independent WebUI", webui, scenario=scenario, service=service.name)


def scenario_node_gateway(ctx: E2EContext) -> None:
    scenario = "node-gateway"
    if ctx.mock_tbs is None:
        ctx.check("mock TBS connected", lambda: None, scenario=scenario, skip="mock TBS disabled")
        return

    def connected() -> dict[str, Any]:
        nodes = wait_for(
            "mock TBS to become visible in Node Gateway",
            lambda: ctx.client.get(ctx.base("node-gateway") + "/api/v1/nodes").json(),
            lambda value: any(item.get("node_id") == ctx.mock_tbs.node_id and item.get("connected") for item in _as_list(value)),
            timeout=ctx.timeout,
        )
        node = next(item for item in _as_list(nodes) if item.get("node_id") == ctx.mock_tbs.node_id)
        return {"node": node}

    ctx.check("mock TBS connected and capability-advertised", connected, scenario=scenario, service="node-gateway")

    def ping() -> dict[str, Any]:
        response = ctx.client.post(
            ctx.base("node-gateway") + f"/api/v1/nodes/{ctx.mock_tbs.node_id}/ping",
            {},
            expected=(202,),
        )
        return {"response": response.json()}

    ctx.check("application ping reaches mock TBS", ping, scenario=scenario, service="node-gateway")


def scenario_edge_fallback_contract(ctx: E2EContext) -> None:
    scenario = "edge-fallback-contract"
    expected = {service.name for service in ctx.inventory.services if service.name != "node-gateway"}

    def health_matrix() -> dict[str, Any]:
        value = ctx.client.get(ctx.base("node-gateway") + "/api/v1/core-services").json()
        services = _as_list(value)
        names = {item.get("service") for item in services}
        if names != expected:
            raise AssertionError(f"backend health matrix mismatch: expected={sorted(expected)} got={sorted(names)}")
        for item in services:
            if not item.get("fallback_mode"):
                raise AssertionError(f"missing fallback mode: {item}")
            if item.get("level") not in {"unknown", "available", "degraded", "unavailable"}:
                raise AssertionError(f"invalid health level: {item}")
        return {"revision": value.get("revision"), "services": len(services)}

    ctx.check("Node Gateway exposes complete edge-health matrix", health_matrix, scenario=scenario, service="node-gateway")

    if ctx.mock_tbs is None:
        ctx.check("health matrix reaches TBS", lambda: None, scenario=scenario, skip="mock TBS disabled")
        return

    def delivered() -> dict[str, Any]:
        snapshot = wait_for(
            "core service matrix delivered to mock TBS",
            lambda: ctx.mock_tbs.core_services_snapshot,
            lambda value: isinstance(value, dict) and len(_as_list(value)) == len(expected),
            timeout=ctx.timeout,
        )
        return {"revision": snapshot.get("revision"), "services": len(_as_list(snapshot))}

    ctx.check("revisioned health matrix reaches connected TBS", delivered, scenario=scenario, service="node-gateway")


def _upsert_subscriber(ctx: E2EContext, issi: int, gssi: int) -> dict[str, Any]:
    payload = {
        "issi": issi,
        "home_mcc": 1,
        "home_mnc": 333,
        "display_name": f"E2E Subscriber {issi}",
        "organization": "NetCore E2E",
        "device_label": "Mock TBS Terminal",
        "enabled": True,
        "registration_allowed": True,
        "call_priority": 3,
        "emergency_allowed": True,
        "sds_allowed": True,
        "packet_data_allowed": True,
        "default_groups": [gssi],
        "notes": f"created by {ctx.report.run_id}",
    }
    base = ctx.base("subscriber-core")
    existing = ctx.client.get(base + f"/api/v1/subscribers/{issi}", expected=(200, 404))
    if existing.status == 200:
        current = existing.json()
        if current.get("notes") != f"created by {ctx.report.run_id}":
            raise RuntimeError(f"refusing to overwrite existing subscriber fixture ISSI {issi}")
        return ctx.client.put(base + f"/api/v1/subscribers/{issi}", payload).json()
    result = ctx.client.post(base + "/api/v1/subscribers", payload, expected=(201,)).json()
    ctx.add_cleanup(lambda: ctx.client.delete(base + f"/api/v1/subscribers/{issi}", expected=(204, 404)))
    return result


def _upsert_group(ctx: E2EContext, gssi: int) -> dict[str, Any]:
    payload = {
        "gssi": gssi,
        "name": f"E2E Group {gssi}",
        "description": "cross-LXC integration fixture",
        "enabled": True,
        "attach_allowed": True,
        "dgna_allowed": True,
        "call_allowed": True,
        "sds_allowed": True,
        "emergency_allowed": True,
        "call_priority": 3,
        "class_of_usage": 4,
        "area_nodes": [],
        "notes": f"created by {ctx.report.run_id}",
    }
    base = ctx.base("group-core")
    existing = ctx.client.get(base + f"/api/v1/groups/{gssi}", expected=(200, 404))
    if existing.status == 200:
        current = existing.json()
        if current.get("notes") != f"created by {ctx.report.run_id}":
            raise RuntimeError(f"refusing to overwrite existing group fixture GSSI {gssi}")
        return ctx.client.put(base + f"/api/v1/groups/{gssi}", payload).json()
    result = ctx.client.post(base + "/api/v1/groups", payload, expected=(201,)).json()
    ctx.add_cleanup(lambda: ctx.client.delete(base + f"/api/v1/groups/{gssi}", expected=(204, 404)))
    return result


def scenario_subscriber_group(ctx: E2EContext) -> None:
    scenario = "subscriber-group"
    if not ctx.allow_mutations:
        ctx.check("subscriber/group fixtures", lambda: None, scenario=scenario, skip="requires --allow-mutations")
        return
    if ctx.mock_tbs is None:
        ctx.check("subscriber/group propagation", lambda: None, scenario=scenario, skip="requires mock TBS")
        return
    issi_a, issi_b, gssi = ctx.fixture_numbers()

    ctx.check(
        "create/update subscriber fixture A",
        lambda: {"profile": _upsert_subscriber(ctx, issi_a, gssi)},
        scenario=scenario,
        service="subscriber-core",
    )
    ctx.check(
        "create/update subscriber fixture B",
        lambda: {"profile": _upsert_subscriber(ctx, issi_b, gssi)},
        scenario=scenario,
        service="subscriber-core",
    )
    ctx.check(
        "create/update group fixture",
        lambda: {"group": _upsert_group(ctx, gssi)},
        scenario=scenario,
        service="group-core",
    )

    def membership() -> dict[str, Any]:
        base = ctx.base("group-core")
        current = next(
            (
                item
                for item in _as_list(ctx.client.get(base + "/api/v1/memberships").json())
                if item.get("issi") == issi_a and item.get("gssi") == gssi
            ),
            None,
        )
        if current is not None and current.get("notes") != ctx.report.run_id:
            raise RuntimeError(f"refusing to overwrite existing membership {issi_a}/{gssi}")
        payload = {"issi": issi_a, "gssi": gssi, "allowed": True, "auto_attach": True, "locked": False, "notes": ctx.report.run_id}
        value = ctx.client.post(base + "/api/v1/memberships", payload).json()
        if current is None:
            ctx.add_cleanup(lambda: ctx.client.delete(base + f"/api/v1/memberships/{issi_a}/{gssi}", expected=(204, 404)))
        return {"membership": value}

    ctx.check("create membership", membership, scenario=scenario, service="group-core")

    def radio_state() -> dict[str, Any]:
        ctx.mock_tbs.register(issi_a)
        ctx.mock_tbs.register(issi_b)
        ctx.mock_tbs.attach_groups(issi_a, [gssi])
        return {"node_id": ctx.mock_tbs.node_id, "issi": [issi_a, issi_b], "gssi": gssi}

    ctx.check("emit registration and affiliation telemetry", radio_state, scenario=scenario, service="node-gateway")

    def subscriber_observed() -> dict[str, Any]:
        observed = wait_for(
            "subscriber-core observed registration",
            lambda: ctx.client.get(ctx.base("subscriber-core") + "/api/v1/observed").json(),
            lambda value: any(item.get("issi") == issi_a and item.get("registered") for item in _as_list(value)),
            timeout=ctx.timeout,
        )
        record = next(item for item in _as_list(observed) if item.get("issi") == issi_a)
        if gssi not in record.get("groups", []):
            raise AssertionError(f"subscriber observed without GSSI {gssi}: {record}")
        return {"observed": record}

    ctx.check("registration propagated to Subscriber Core", subscriber_observed, scenario=scenario, service="subscriber-core")

    def group_affiliation() -> dict[str, Any]:
        affiliations = wait_for(
            "group-core affiliation",
            lambda: ctx.client.get(ctx.base("group-core") + "/api/v1/affiliations").json(),
            lambda value: any(item.get("issi") == issi_a and gssi in item.get("groups", []) for item in _as_list(value)),
            timeout=ctx.timeout,
        )
        record = next(item for item in _as_list(affiliations) if item.get("issi") == issi_a)
        return {"affiliation": record}

    ctx.check("affiliation propagated to Group Core", group_affiliation, scenario=scenario, service="group-core")

    def policy_sync() -> dict[str, Any]:
        sub = ctx.client.post(ctx.base("subscriber-core") + "/api/v1/sync", {}, expected=(202,)).json()
        grp = ctx.client.post(ctx.base("group-core") + "/api/v1/sync", {}, expected=(202,)).json()
        time.sleep(0.8)
        return {
            "subscriber_sync": sub,
            "group_sync": grp,
            "mock_received_commands": len(ctx.mock_tbs.received_commands),
        }

    ctx.check("policy synchronization round-trip", policy_sync, scenario=scenario, service="node-gateway")


def scenario_call_media_recorder(ctx: E2EContext) -> None:
    scenario = "call-media-recorder"
    if not ctx.allow_mutations:
        ctx.check("call/media/recorder flow", lambda: None, scenario=scenario, skip="requires --allow-mutations")
        return
    if ctx.mock_tbs is None:
        ctx.check("call/media/recorder flow", lambda: None, scenario=scenario, skip="requires mock TBS")
        return
    issi_a, _, gssi = ctx.fixture_numbers()
    call_id = 400 + (issi_a % 100)

    def start_call() -> dict[str, Any]:
        ctx.mock_tbs.start_group_call(call_id=call_id, gssi=gssi, caller_issi=issi_a, ts=2, priority=5)
        ctx.mock_tbs.group_speaker(call_id=call_id, gssi=gssi, speaker_issi=issi_a)
        calls = wait_for(
            "call-control logical call",
            lambda: ctx.client.get(ctx.base("call-control") + "/api/v1/calls").json(),
            lambda value: any(item.get("gssi") == gssi and item.get("phase") in {"active", "partial", "starting"} for item in _as_list(value)),
            timeout=ctx.timeout,
        )
        call = next(item for item in _as_list(calls) if item.get("gssi") == gssi and item.get("phase") in {"active", "partial", "starting"})
        return {"call": call}

    ctx.check("group-call telemetry creates logical call", start_call, scenario=scenario, service="call-control")

    def media_session() -> dict[str, Any]:
        sessions = wait_for(
            "media session",
            lambda: ctx.client.get(ctx.base("media-switch") + "/api/v1/sessions").json(),
            lambda value: any(item.get("gssi") == gssi for item in _as_list(value)),
            timeout=ctx.timeout,
        )
        session = next(item for item in _as_list(sessions) if item.get("gssi") == gssi)
        return {"session": session}

    ctx.check("Media Switch mirrors active call", media_session, scenario=scenario, service="media-switch")

    def frames() -> dict[str, Any]:
        for sequence in range(1, 25):
            ctx.mock_tbs.media_frame(sequence=sequence, logical_ts=2)
            time.sleep(0.015)
        taps = wait_for(
            "media taps",
            lambda: ctx.client.get(ctx.base("media-switch") + "/api/v1/taps?limit=100").json(),
            lambda value: sum(1 for item in _as_list(value) if item.get("source_node_id") == ctx.mock_tbs.node_id) >= 10,
            timeout=ctx.timeout,
        )
        return {"matching_taps": sum(1 for item in _as_list(taps) if item.get("source_node_id") == ctx.mock_tbs.node_id)}

    ctx.check("packed 35-byte speech frames traverse Node Gateway", frames, scenario=scenario, service="media-switch")

    def recorder_active() -> dict[str, Any]:
        active = wait_for(
            "active recorder session",
            lambda: ctx.client.get(ctx.base("recorder") + "/api/v1/active").json(),
            lambda value: any((item.get("metadata") or {}).get("gssi") == gssi for item in _as_list(value)),
            timeout=max(ctx.timeout, 30.0),
        )
        record = next(item for item in _as_list(active) if (item.get("metadata") or {}).get("gssi") == gssi)
        return {"active": record}

    ctx.check("Recorder opens a recording from full-frame tap", recorder_active, scenario=scenario, service="recorder")

    def end_and_verify() -> dict[str, Any]:
        ctx.mock_tbs.end_group_call(call_id=call_id, gssi=gssi)
        recordings = wait_for(
            "finalized recording",
            lambda: ctx.client.get(query_url(ctx.base("recorder"), "/api/v1/recordings", gssi=gssi)).json(),
            lambda value: any(item.get("gssi") == gssi and item.get("ended_at") and int(item.get("frame_count", 0)) >= 10 for item in _as_list(value)),
            timeout=max(ctx.timeout, 45.0),
            interval=0.8,
        )
        recording = next(item for item in _as_list(recordings) if item.get("gssi") == gssi and item.get("ended_at"))
        recording_id = str(recording["id"])
        verify = ctx.client.post(ctx.base("recorder") + f"/api/v1/recordings/{recording_id}/verify", {}, expected=(200,)).json()
        ctx.add_cleanup(
            lambda: ctx.client.post(
                ctx.base("recorder") + f"/api/v1/recordings/{recording_id}/delete",
                {},
                expected=(200, 404, 409),
            )
        )
        return {"recording": recording, "verify": verify}

    ctx.check("call end finalizes and verifies recording", end_and_verify, scenario=scenario, service="recorder")


def scenario_sds(ctx: E2EContext) -> None:
    scenario = "sds"
    if not ctx.allow_mutations:
        ctx.check("SDS routing", lambda: None, scenario=scenario, skip="requires --allow-mutations")
        return
    if ctx.mock_tbs is None:
        ctx.check("SDS routing", lambda: None, scenario=scenario, skip="requires mock TBS")
        return
    issi_a, issi_b, _ = ctx.fixture_numbers()

    def delivered() -> dict[str, Any]:
        payload = {
            "source_issi": 9999,
            "dest_issi": issi_a,
            "is_group": False,
            "sds_type": 4,
            "protocol_id": 130,
            "text": f"NetCore E2E {ctx.report.run_id}",
            "priority": 5,
            "ttl_secs": 120,
            "ingress": "e2e-test",
            "force_nodes": [ctx.mock_tbs.node_id],
        }
        created = ctx.client.post(ctx.base("sds-router") + "/api/v1/messages", payload, expected=(201,)).json()
        message_id = created["id"]
        ctx.add_cleanup(lambda: ctx.client.delete(ctx.base("sds-router") + f"/api/v1/messages/{message_id}"))
        message = wait_for(
            "SDS delivery confirmation",
            lambda: ctx.client.get(ctx.base("sds-router") + f"/api/v1/messages/{message_id}").json(),
            lambda value: value.get("state") in {"delivered", "partial"},
            timeout=ctx.timeout,
        )
        return {"message": message}

    ctx.check("individual SDS delivered through mock TBS", delivered, scenario=scenario, service="sds-router")

    def offline() -> dict[str, Any]:
        payload = {
            "source_issi": 9999,
            "dest_issi": issi_b + 1000,
            "is_group": False,
            "sds_type": 4,
            "protocol_id": 130,
            "text": "offline store-and-forward fixture",
            "priority": 1,
            "ttl_secs": 120,
            "ingress": "e2e-test",
            "force_nodes": [],
        }
        created = ctx.client.post(ctx.base("sds-router") + "/api/v1/messages", payload, expected=(201,)).json()
        ctx.add_cleanup(lambda: ctx.client.delete(ctx.base("sds-router") + f"/api/v1/messages/{created['id']}"))
        if created.get("state") not in {"offline", "queued", "received"}:
            raise AssertionError(f"unexpected store-and-forward state: {created.get('state')}")
        return {"message": created}

    ctx.check("offline destination enters store-and-forward", offline, scenario=scenario, service="sds-router")


def _ipv4_udp_packet(source: str, destination: str, source_port: int = 17007, destination_port: int = 7007, payload: bytes = b"netcore-e2e") -> bytes:
    import ipaddress
    import struct

    src = ipaddress.IPv4Address(source).packed
    dst = ipaddress.IPv4Address(destination).packed
    udp_length = 8 + len(payload)
    total_length = 20 + udp_length
    header = bytearray(struct.pack("!BBHHHBBH4s4s", 0x45, 0, total_length, 0x1234, 0, 64, 17, 0, src, dst))
    checksum = 0
    for index in range(0, len(header), 2):
        checksum += (header[index] << 8) + header[index + 1]
        checksum = (checksum & 0xFFFF) + (checksum >> 16)
    checksum = (~checksum) & 0xFFFF
    header[10:12] = struct.pack("!H", checksum)
    udp = struct.pack("!HHHH", source_port, destination_port, udp_length, 0) + payload
    return bytes(header) + udp



def _pick_free_packet_ipv4(contexts: list[Any], issi: int) -> str:
    used = {str(item.get("ipv4")) for item in contexts if item.get("ipv4")}
    start = 2 + (issi % 253)
    for offset in range(253):
        host = 2 + ((start - 2 + offset) % 253)
        candidate = f"10.44.0.{host}"
        if candidate not in used:
            return candidate
    raise RuntimeError("no free IPv4 address available in the E2E packet-data fixture pool")

def scenario_packet_data(ctx: E2EContext) -> None:
    scenario = "packet-data"
    if not ctx.allow_mutations:
        ctx.check("packet-data flow", lambda: None, scenario=scenario, skip="requires --allow-mutations")
        return
    if ctx.mock_tbs is None:
        ctx.check("packet-data flow", lambda: None, scenario=scenario, skip="requires mock TBS")
        return
    issi_a, _, _ = ctx.fixture_numbers()
    node_id = ctx.mock_tbs.node_id
    packet = ctx.base("packet-core")

    def context_flow() -> dict[str, Any]:
        existing_contexts = _as_list(ctx.client.get(packet + "/api/v1/contexts").json())
        requested_ipv4 = _pick_free_packet_ipv4(existing_contexts, issi_a)
        events = [
            {"kind": "hello", "protocol_version": "netcore-packet-edge-v1", "node_id": node_id, "station_name": "E2E TBS", "mcc": 1, "mnc": 333, "location_area": 1},
            {"kind": "subscriber_location", "node_id": node_id, "issi": issi_a},
            {"kind": "activate_demand", "node_id": node_id, "issi": issi_a, "nsapi": 1, "requested_ipv4": requested_ipv4, "primary_nsapi": None, "snei": 1, "mtu": 1200, "priority": 3},
        ]
        actions: list[Any] = []
        for event in events:
            response = ctx.client.post(packet + "/api/v1/edge/events", event, expected=(202,)).json()
            actions.extend(response.get("actions", []))

        # In authoritative mode ActivateDemand creates the context. In the default
        # shadow mode the local TBS remains authoritative, so the mock edge emits
        # the corresponding ContextActivated event itself.
        current = _as_list(ctx.client.get(packet + "/api/v1/contexts").json())
        context = next((item for item in current if item.get("issi") == issi_a and item.get("nsapi") == 1), None)
        if context is None:
            activated = {
                "kind": "context_activated",
                "node_id": node_id,
                "issi": issi_a,
                "nsapi": 1,
                "ipv4": requested_ipv4,
                "primary_nsapi": None,
                "snei": 1,
                "mtu": 1200,
                "priority": 3,
            }
            ctx.client.post(packet + "/api/v1/edge/events", activated, expected=(202,))

        contexts = wait_for(
            "PDP context creation",
            lambda: ctx.client.get(packet + "/api/v1/contexts").json(),
            lambda value: any(item.get("issi") == issi_a and item.get("nsapi") == 1 for item in _as_list(value)),
            timeout=ctx.timeout,
        )
        context = next(item for item in _as_list(contexts) if item.get("issi") == issi_a and item.get("nsapi") == 1)
        if not context.get("ipv4"):
            raise AssertionError(f"context has no IPv4 address: {context}")
        ctx.add_cleanup(
            lambda: ctx.client.post(
                packet + "/api/v1/edge/events",
                {"kind": "deactivate", "node_id": node_id, "issi": issi_a, "nsapi": 1, "reason": "E2E cleanup"},
                expected=(202, 409),
            )
        )
        return {"context": context, "actions": actions}

    ctx.check("SNDCP activation creates anchored PDP context", context_flow, scenario=scenario, service="packet-core")

    def npdu() -> dict[str, Any]:
        contexts = _as_list(ctx.client.get(packet + "/api/v1/contexts").json())
        context = next(item for item in contexts if item.get("issi") == issi_a and item.get("nsapi") == 1)
        payload = _ipv4_udp_packet(str(context["ipv4"]), "10.0.20.1")
        queued = ctx.client.post(
            packet + "/api/v1/downlink",
            {"issi": issi_a, "nsapi": 1, "payload_hex": payload.hex(), "acknowledged": False, "priority": 3},
            expected=(202,),
        ).json()
        outbox = ctx.client.get(packet + "/api/v1/npdu-outbox?limit=100").json()
        matching_outbox = [item for item in _as_list(outbox) if item.get("issi") == issi_a]
        actions = ctx.client.get(packet + "/api/v1/actions?limit=500").json()
        matching_actions = [item for item in _as_list(actions) if (item.get("payload") or {}).get("issi") == issi_a]
        if not matching_outbox and not matching_actions:
            raise AssertionError("downlink produced neither N-PDU outbox entry nor edge action")
        for item in matching_outbox:
            if item.get("id"):
                outbox_id = str(item["id"])
                ctx.add_cleanup(
                    lambda outbox_id=outbox_id: ctx.client.delete(
                        packet + f"/api/v1/npdu-outbox/{outbox_id}", expected=(204, 404)
                    )
                )
        for item in matching_actions:
            if item.get("id"):
                action_id = str(item["id"])
                ctx.add_cleanup(
                    lambda action_id=action_id: ctx.client.post(
                        packet + f"/api/v1/actions/{action_id}/ack",
                        {"success": True, "message": "E2E cleanup"},
                        expected=(200, 404, 409),
                    )
                )
        return {
            "queued": queued,
            "outbox_count": len(matching_outbox),
            "edge_actions": len(matching_actions),
        }

    ctx.check("IPv4 N-PDU enters packet downlink path", npdu, scenario=scenario, service="packet-core")

    def gateway_view() -> dict[str, Any]:
        contexts = wait_for(
            "IP Gateway context synchronization",
            lambda: ctx.client.get(ctx.base("ip-gateway") + "/api/v1/contexts").json(),
            lambda value: any(item.get("issi") == issi_a for item in _as_list(value)),
            timeout=max(ctx.timeout, 30.0),
        )
        return {"contexts": [item for item in _as_list(contexts) if item.get("issi") == issi_a]}

    ctx.check("IP Gateway learns Packet Core context", gateway_view, scenario=scenario, service="ip-gateway")


def scenario_observability(ctx: E2EContext) -> None:
    scenario = "observability"

    def scrape() -> dict[str, Any]:
        response = ctx.client.post(ctx.base("observability") + "/api/v1/maintenance/scrape-now", {}, expected=(200, 202)).json()
        status = wait_for(
            "observability target scrape",
            lambda: ctx.client.get(ctx.base("observability") + "/api/v1/status").json(),
            lambda value: int(value.get("targets_up", 0)) >= max(1, len(ctx.inventory.services) - 2),
            timeout=max(ctx.timeout, 30.0),
        )
        return {"scrape": response, "status": status}

    ctx.check("central collector scrapes service metrics", scrape, scenario=scenario, service="observability")

    def trace_log() -> dict[str, Any]:
        trace_id = ctx.report.run_id.replace("-", "")[:32]
        log = {
            "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "service": "netcore-e2e",
            "node": None,
            "level": "info",
            "message": "cross-LXC E2E marker",
            "correlation_id": ctx.report.run_id,
            "trace_id": trace_id,
            "fields": {"run_id": ctx.report.run_id},
        }
        ctx.client.post(ctx.base("observability") + "/api/v1/logs/ingest", {"records": [log]}, expected=(202,))
        logs = wait_for(
            "E2E log marker",
            lambda: ctx.client.get(query_url(ctx.base("observability"), "/api/v1/logs", contains="cross-LXC E2E marker", limit=100)).json(),
            lambda value: any(item.get("trace_id") == trace_id for item in _as_list(value)),
            timeout=ctx.timeout,
        )
        return {"trace_id": trace_id, "matching_logs": len(_as_list(logs))}

    ctx.check("structured E2E marker is searchable", trace_log, scenario=scenario, service="observability")



def _find_sensitive_keys(value: Any, forbidden: set[str], path: str = "$") -> list[str]:
    findings: list[str] = []
    if isinstance(value, dict):
        for key, child in value.items():
            key_text = str(key)
            child_path = f"{path}.{key_text}"
            if key_text.lower() in forbidden:
                findings.append(child_path)
            findings.extend(_find_sensitive_keys(child, forbidden, child_path))
    elif isinstance(value, list):
        for index, child in enumerate(value):
            findings.extend(_find_sensitive_keys(child, forbidden, f"{path}[{index}]"))
    return findings


def scenario_control_room_federation(ctx: E2EContext) -> None:
    scenario = "control-room-federation"
    base = ctx.base("control-room")
    expected = set(ctx.services) - {"control-room", "observability"}

    def poll() -> dict[str, Any]:
        accepted = ctx.client.post(base + "/api/v1/services/poll", {}, expected=(202,)).json()
        matrix = wait_for(
            "Control Room service federation poll",
            lambda: ctx.client.get(base + "/api/v1/services").json(),
            lambda value: expected.issubset({str(item.get("name")) for item in _as_list(value)})
            and all(item.get("checked_at") for item in _as_list(value) if item.get("name") in expected),
            timeout=max(ctx.timeout, 30.0),
        )
        services = [item for item in _as_list(matrix) if item.get("name") in expected]
        missing = sorted(expected - {str(item.get("name")) for item in services})
        if missing:
            raise AssertionError(f"Control Room is missing configured services: {missing}")
        offline = sorted(
            str(item.get("name"))
            for item in services
            if item.get("enabled", True) and item.get("live") is False
        )
        if ctx.strict_ready and offline:
            raise AssertionError(f"Control Room reports offline services: {offline}")
        return {
            "poll": accepted,
            "services_total": len(services),
            "healthy": sum(1 for item in services if item.get("status") == "healthy"),
            "degraded": sum(1 for item in services if item.get("status") == "degraded"),
            "offline": offline,
        }

    ctx.check(
        "Control Room federates all configured core services",
        poll,
        scenario=scenario,
        service="control-room",
    )

    def overview() -> dict[str, Any]:
        value = ctx.client.get(base + "/api/v1/control-room/overview").json()
        if not isinstance(value.get("operations"), dict) or not isinstance(value.get("federated"), dict):
            raise AssertionError("Control Room overview lacks operations or federated domain data")
        if value.get("authoritative_state") is not False:
            raise AssertionError("Control Room must remain a non-authoritative aggregation plane")
        return {
            "operations": value.get("operations"),
            "federated_domains": sorted(value.get("federated", {}).keys()),
        }

    ctx.check(
        "Control Room exposes non-authoritative federated overview",
        overview,
        scenario=scenario,
        service="control-room",
    )


def scenario_platform_services(ctx: E2EContext) -> None:
    scenario = "platform-services"
    forbidden = {
        "key_bytes",
        "raw_key",
        "raw_material",
        "secret_value",
        "password",
        "token",
        "challenge_hex",
        "expected_response",
        "expected_response_hex",
        "dck_hex",
        "data_base64",
    }

    def security_view() -> dict[str, Any]:
        base = ctx.base("security-core")
        payload = {
            "config": ctx.client.get(base + "/api/v1/config").json(),
            "policy": ctx.client.get(base + "/api/v1/policy").json(),
            "profiles": ctx.client.get(base + "/api/v1/profiles").json(),
            "actions": ctx.client.get(base + "/api/v1/actions").json(),
        }
        findings = _find_sensitive_keys(payload, forbidden)
        if findings:
            raise AssertionError(f"Security Core management API exposed sensitive fields: {findings}")
        return {"profiles": len(_as_list(payload["profiles"])), "actions": len(_as_list(payload["actions"]))}

    ctx.check(
        "Security Core management view is metadata-only",
        security_view,
        scenario=scenario,
        service="security-core",
    )

    def kmf_view() -> dict[str, Any]:
        base = ctx.base("kmf")
        payload = {
            "config": ctx.client.get(base + "/api/v1/config").json(),
            "policy": ctx.client.get(base + "/api/v1/policy").json(),
            "keys": ctx.client.get(base + "/api/v1/keys").json(),
            "actions": ctx.client.get(base + "/api/v1/otar/actions").json(),
        }
        findings = _find_sensitive_keys(payload, forbidden)
        if findings:
            raise AssertionError(f"KMF management API exposed raw key fields: {findings}")
        return {"keys": len(_as_list(payload["keys"])), "otar_actions": len(_as_list(payload["actions"]))}

    ctx.check(
        "KMF management view is metadata-only",
        kmf_view,
        scenario=scenario,
        service="kmf",
    )

    def transit_view() -> dict[str, Any]:
        base = ctx.base("transit")
        status = ctx.client.get(base + "/api/v1/status").json()
        peers = ctx.client.get(base + "/api/v1/peers").json()
        routes = ctx.client.get(base + "/api/v1/routes").json()
        if not isinstance(status, dict):
            raise AssertionError("Transit status is not an object")
        return {
            "protocol_version": status.get("protocol_version"),
            "peers": len(_as_list(peers)),
            "routes": len(_as_list(routes)),
        }

    ctx.check(
        "Transit exposes peer and route control-plane state",
        transit_view,
        scenario=scenario,
        service="transit",
    )

    def application_view() -> dict[str, Any]:
        base = ctx.base("application-gateway")
        connectors = ctx.client.get(base + "/api/v1/connectors").json()
        secrets = ctx.client.get(base + "/api/v1/secrets").json()
        templates = ctx.client.get(base + "/api/v1/templates").json()
        findings = _find_sensitive_keys(secrets, forbidden)
        if findings:
            raise AssertionError(f"Application Gateway secret-status API exposed values: {findings}")
        return {
            "connectors": len(_as_list(connectors)),
            "secret_statuses": len(_as_list(secrets)),
            "templates": len(_as_list(templates)),
        }

    ctx.check(
        "Application Gateway exposes connector state without secrets",
        application_view,
        scenario=scenario,
        service="application-gateway",
    )

    def media_view() -> dict[str, Any]:
        base = ctx.base("media-library")
        status = ctx.client.get(base + "/api/v1/status").json()
        assets = ctx.client.get(base + "/api/v1/assets?limit=100").json()
        findings = _find_sensitive_keys(assets, forbidden)
        if findings:
            raise AssertionError(f"Media Library asset list exposed uploaded binary data: {findings}")
        return {
            "ready": status.get("ready"),
            "assets": len(_as_list(assets)),
            "broadcast_ready": status.get("broadcast_ready"),
        }

    ctx.check(
        "Media Library exposes metadata without binary payloads",
        media_view,
        scenario=scenario,
        service="media-library",
    )

def scenario_restart_restore(ctx: E2EContext) -> None:
    scenario = "restart-restore"
    if not ctx.allow_restarts:
        ctx.check("restart persistence", lambda: None, scenario=scenario, skip="requires --allow-restarts")
        return
    if not ctx.allow_mutations:
        ctx.check("restart persistence", lambda: None, scenario=scenario, skip="requires --allow-mutations")
        return
    issi_a, _, gssi = ctx.fixture_numbers()

    def subscriber_restart() -> dict[str, Any]:
        before = ctx.client.get(ctx.base("subscriber-core") + f"/api/v1/subscribers/{issi_a}").json()
        ctx.ssh("subscriber-core", f"systemctl restart {ctx.service('subscriber-core').unit} && systemctl is-active --quiet {ctx.service('subscriber-core').unit}")
        wait_for(
            "subscriber-core liveness after restart",
            lambda: ctx.client.get(ctx.base("subscriber-core") + "/health/live", expected=(200,)).status,
            lambda value: value == 200,
            timeout=max(ctx.timeout, 45.0),
        )
        after = ctx.client.get(ctx.base("subscriber-core") + f"/api/v1/subscribers/{issi_a}").json()
        if before.get("revision") != after.get("revision") or after.get("issi") != issi_a:
            raise AssertionError("subscriber record changed or disappeared across restart")
        return {"before": before, "after": after}

    ctx.check("Subscriber Core persists profile across restart", subscriber_restart, scenario=scenario, service="subscriber-core")

    def group_restart() -> dict[str, Any]:
        before = ctx.client.get(ctx.base("group-core") + f"/api/v1/groups/{gssi}").json()
        ctx.ssh("group-core", f"systemctl restart {ctx.service('group-core').unit} && systemctl is-active --quiet {ctx.service('group-core').unit}")
        wait_for(
            "group-core liveness after restart",
            lambda: ctx.client.get(ctx.base("group-core") + "/health/live", expected=(200,)).status,
            lambda value: value == 200,
            timeout=max(ctx.timeout, 45.0),
        )
        after = ctx.client.get(ctx.base("group-core") + f"/api/v1/groups/{gssi}").json()
        if before.get("revision") != after.get("revision") or after.get("gssi") != gssi:
            raise AssertionError("group record changed or disappeared across restart")
        return {"before": before, "after": after}

    ctx.check("Group Core persists group across restart", group_restart, scenario=scenario, service="group-core")


def scenario_fault_matrix(ctx: E2EContext) -> None:
    scenario = "fault-matrix"
    if not ctx.allow_restarts:
        ctx.check("dependency outage matrix", lambda: None, scenario=scenario, skip="requires --allow-restarts")
        return

    pairs = [
        ("call-control", "media-switch"),
        ("packet-core", "ip-gateway"),
        ("media-switch", "recorder"),
    ]
    for victim, dependent in pairs:
        if victim not in ctx.services or dependent not in ctx.services:
            ctx.check(f"{victim} outage degrades {dependent}", lambda: None, scenario=scenario, skip="service missing from inventory")
            continue

        def run_fault(victim_name: str = victim, dependent_name: str = dependent) -> dict[str, Any]:
            victim_service = ctx.service(victim_name)
            dependent_base = ctx.base(dependent_name)
            try:
                ctx.ssh(victim_name, f"systemctl stop {victim_service.unit}")
                wait_for(
                    f"{victim_name} listener to stop",
                    lambda: _http_status_or_zero(ctx, ctx.base(victim_name) + "/health/live"),
                    lambda value: value == 0,
                    timeout=max(ctx.timeout, 35.0),
                )
                degraded = wait_for(
                    f"{dependent_name} readiness degradation",
                    lambda: _http_status_or_zero(ctx, dependent_base + "/health/ready"),
                    lambda value: value in {0, 503},
                    timeout=max(ctx.timeout, 35.0),
                )
            finally:
                ctx.ssh(victim_name, f"systemctl start {victim_service.unit} && systemctl is-active --quiet {victim_service.unit}", check=False)
            recovered = wait_for(
                f"{victim_name} recovery",
                lambda: _http_status_or_zero(ctx, ctx.base(victim_name) + "/health/live"),
                lambda value: value == 200,
                timeout=max(ctx.timeout, 60.0),
            )
            dependent_recovered = wait_for(
                f"{dependent_name} readiness recovery",
                lambda: _http_status_or_zero(ctx, dependent_base + "/health/ready"),
                lambda value: value == 200,
                timeout=max(ctx.timeout, 60.0),
            )
            return {"degraded_status": degraded, "victim_recovered": recovered, "dependent_recovered": dependent_recovered}

        ctx.check(
            f"{victim} outage degrades and recovers {dependent}",
            run_fault,
            scenario=scenario,
            service=victim,
        )



def scenario_edge_service_outages(ctx: E2EContext) -> None:
    """Prove the per-service health/fallback contract for every remote backend.

    The Node Gateway itself cannot be stopped in this scenario because it is
    the control link carrying the matrix to the TBS. Its loss is covered by the
    TBS-side connection/lease state machine and the offline reference tests.
    """

    scenario = "edge-service-outages"
    if not ctx.allow_restarts:
        ctx.check("all backend outage transitions", lambda: None, scenario=scenario, skip="requires --allow-restarts")
        return

    victims = [service.name for service in ctx.inventory.services if service.name != "node-gateway"]
    gateway_url = ctx.base("node-gateway") + "/api/v1/core-services"

    def gateway_level(service_name: str) -> tuple[str | None, int | None]:
        payload = ctx.client.get(gateway_url).json()
        item = next((entry for entry in _as_list(payload) if entry.get("service") == service_name), None)
        return (item.get("level") if item else None, payload.get("revision"))

    def tbs_level(service_name: str) -> tuple[str | None, int | None]:
        if ctx.mock_tbs is None or not isinstance(ctx.mock_tbs.core_services_snapshot, dict):
            return None, None
        payload = ctx.mock_tbs.core_services_snapshot
        item = next((entry for entry in _as_list(payload) if entry.get("service") == service_name), None)
        return (item.get("level") if item else None, payload.get("revision"))

    for victim in victims:
        def run_outage(victim_name: str = victim) -> dict[str, Any]:
            service = ctx.service(victim_name)
            initial_level, initial_revision = wait_for(
                f"{victim_name} initially available in edge matrix",
                lambda: gateway_level(victim_name),
                lambda value: value[0] == "available",
                timeout=max(ctx.timeout, 75.0),
            )
            stopped_revision: int | None = None
            recovered_revision: int | None = None
            try:
                ctx.ssh(victim_name, f"systemctl stop {service.unit}")
                wait_for(
                    f"{victim_name} listener to stop",
                    lambda: _http_status_or_zero(ctx, ctx.base(victim_name) + "/health/live"),
                    lambda value: value == 0,
                    timeout=max(ctx.timeout, 45.0),
                )
                _, stopped_revision = wait_for(
                    f"Node Gateway marks {victim_name} unavailable",
                    lambda: gateway_level(victim_name),
                    lambda value: value[0] == "unavailable" and (value[1] or 0) > (initial_revision or 0),
                    timeout=max(ctx.timeout, 75.0),
                )
                if ctx.mock_tbs is not None:
                    wait_for(
                        f"mock TBS receives {victim_name} unavailable",
                        lambda: tbs_level(victim_name),
                        lambda value: value[0] == "unavailable" and (value[1] or 0) >= (stopped_revision or 0),
                        timeout=max(ctx.timeout, 45.0),
                    )
            finally:
                ctx.ssh(
                    victim_name,
                    f"systemctl start {service.unit} && systemctl is-active --quiet {service.unit}",
                    check=False,
                )

            wait_for(
                f"{victim_name} liveness recovery",
                lambda: _http_status_or_zero(ctx, ctx.base(victim_name) + "/health/live"),
                lambda value: value == 200,
                timeout=max(ctx.timeout, 75.0),
            )
            _, recovered_revision = wait_for(
                f"Node Gateway marks {victim_name} available again",
                lambda: gateway_level(victim_name),
                lambda value: value[0] == "available" and (value[1] or 0) > (stopped_revision or 0),
                timeout=max(ctx.timeout, 90.0),
            )
            if ctx.mock_tbs is not None:
                wait_for(
                    f"mock TBS receives {victim_name} recovery",
                    lambda: tbs_level(victim_name),
                    lambda value: value[0] == "available" and (value[1] or 0) >= (recovered_revision or 0),
                    timeout=max(ctx.timeout, 45.0),
                )
            return {
                "initial_level": initial_level,
                "initial_revision": initial_revision,
                "outage_revision": stopped_revision,
                "recovery_revision": recovered_revision,
                "fallback_transport": "node-gateway-core-services-snapshot",
            }

        ctx.check(
            f"{victim} outage enters fallback and recovers",
            run_outage,
            scenario=scenario,
            service=victim,
        )


def _http_status_or_zero(ctx: E2EContext, url: str) -> int:
    try:
        return ctx.client.get(url, expected=(200, 503)).status
    except BaseException:
        return 0


SCENARIOS: dict[str, Callable[[E2EContext], None]] = {
    "contracts": scenario_contracts,
    "node-gateway": scenario_node_gateway,
    "edge-fallback-contract": scenario_edge_fallback_contract,
    "subscriber-group": scenario_subscriber_group,
    "call-media-recorder": scenario_call_media_recorder,
    "sds": scenario_sds,
    "packet-data": scenario_packet_data,
    "observability": scenario_observability,
    "control-room-federation": scenario_control_room_federation,
    "platform-services": scenario_platform_services,
    "restart-restore": scenario_restart_restore,
    "fault-matrix": scenario_fault_matrix,
    "edge-service-outages": scenario_edge_service_outages,
}

DEFAULT_SMOKE = ["contracts", "node-gateway", "edge-fallback-contract", "control-room-federation"]
DEFAULT_FULL = [
    "contracts",
    "node-gateway",
    "edge-fallback-contract",
    "subscriber-group",
    "call-media-recorder",
    "sds",
    "packet-data",
    "control-room-federation",
    "platform-services",
    "observability",
]
