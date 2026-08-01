#!/usr/bin/env python3
from __future__ import annotations

import argparse
import html
import json
import os
import re
import shlex
import signal
import subprocess
import threading
import time
import tomllib
import uuid
from collections import deque
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any
from urllib.error import HTTPError, URLError
from urllib.parse import parse_qs, urlparse
from urllib.request import Request, urlopen

STATE_SCHEMA = "netcore-sip-switch-state-v1"
EVENT_SCHEMA = "netcore-event-v1"
SERVICE = "netcore-sip-switch"
SAFE_ID = re.compile(r"^[A-Za-z0-9_.:-]+$")
SAFE_NUMBER = re.compile(r"^[0-9*#+]+$")
TERMINAL_STATES = {"ended", "failed", "rejected", "cancelled"}


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="milliseconds").replace("+00:00", "Z")


def compact(value: Any, maximum: int = 500) -> str:
    return re.sub(r"\s+", " ", str(value or "")).strip()[:maximum]


def safe_id(value: Any, maximum: int = 120) -> str:
    text = re.sub(r"[^A-Za-z0-9_.:-]+", "-", str(value or "").strip()).strip("-")
    return text[:maximum]


def bool_value(value: Any, default: bool = False) -> bool:
    if isinstance(value, bool):
        return value
    if value is None:
        return default
    return str(value).strip().lower() in {"1", "true", "yes", "on", "ja"}


def int_value(value: Any, default: int = 0) -> int:
    try:
        return int(value)
    except (TypeError, ValueError):
        return default


def atomic_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = path.with_suffix(path.suffix + ".tmp")
    temp.write_text(json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    temp.replace(path)


def http_json(method: str, url: str, payload: dict[str, Any] | None = None, timeout: float = 2.0) -> tuple[int, Any]:
    data = None
    headers = {"Accept": "application/json"}
    if payload is not None:
        data = json.dumps(payload).encode("utf-8")
        headers["Content-Type"] = "application/json"
    request = Request(url, data=data, headers=headers, method=method)
    try:
        with urlopen(request, timeout=timeout) as response:
            raw = response.read()
            return response.status, json.loads(raw.decode("utf-8")) if raw else None
    except HTTPError as error:
        raw = error.read()
        try:
            body = json.loads(raw.decode("utf-8")) if raw else {"error": error.reason}
        except ValueError:
            body = {"error": compact(raw.decode("utf-8", "replace"))}
        return error.code, body
    except (URLError, TimeoutError, OSError) as error:
        return 0, {"error": compact(error)}


class Config:
    def __init__(self, raw: dict[str, Any], path: Path):
        self.raw = raw
        self.path = path

    @property
    def bind(self) -> tuple[str, int]:
        host, port = str(self.raw["server"]["bind"]).rsplit(":", 1)
        return host, int(port)

    @property
    def storage(self) -> dict[str, Any]:
        return self.raw["storage"]

    @property
    def mqtt(self) -> dict[str, Any]:
        return self.raw.get("mqtt", {})

    @property
    def mobility(self) -> dict[str, Any]:
        return self.raw.get("mobility_core", {})

    @property
    def asterisk(self) -> dict[str, Any]:
        return self.raw.get("asterisk", {})

    @property
    def pbx(self) -> dict[str, Any]:
        return self.raw.get("pbx", {})

    @property
    def routing(self) -> dict[str, Any]:
        return self.raw.get("routing", {})

    @property
    def tbs(self) -> list[dict[str, Any]]:
        items = self.raw.get("tbs", [])
        return items if isinstance(items, list) else []

    @property
    def mappings(self) -> list[dict[str, Any]]:
        items = self.raw.get("number_mappings", [])
        return items if isinstance(items, list) else []


class SipSwitch:
    def __init__(self, config: Config):
        self.config = config
        self.lock = threading.RLock()
        self.stop_event = threading.Event()
        self.started_at = now_iso()
        self.instance = os.uname().nodename
        storage = config.storage
        self.state_path = Path(str(storage["state_file"]))
        self.event_path = Path(str(storage["event_log"]))
        self.audit_path = Path(str(storage["audit_log"]))
        self.state_path.parent.mkdir(parents=True, exist_ok=True)
        limit = int(config.raw.get("management", {}).get("event_history_limit", 3000))
        self.events: deque[dict[str, Any]] = deque(maxlen=limit)
        self.decisions: deque[dict[str, Any]] = deque(maxlen=limit)
        self.calls: dict[str, dict[str, Any]] = {}
        self.endpoint_state: dict[str, dict[str, Any]] = {}
        self.health = {
            "asterisk": False,
            "mobility_core": False,
            "pbx": False,
            "last_probe_at": None,
            "last_error": None,
        }
        self.metrics = {
            "route_requests": 0,
            "routes_resolved": 0,
            "routes_rejected": 0,
            "calls_started": 0,
            "calls_answered": 0,
            "calls_ended": 0,
            "asterisk_reloads": 0,
            "mobility_failures": 0,
        }
        self._load()

    def _load(self) -> None:
        try:
            data = json.loads(self.state_path.read_text(encoding="utf-8"))
            if data.get("schema") == STATE_SCHEMA:
                if isinstance(data.get("calls"), dict):
                    self.calls = data["calls"]
                if isinstance(data.get("endpoint_state"), dict):
                    self.endpoint_state = data["endpoint_state"]
                if isinstance(data.get("metrics"), dict):
                    for key in self.metrics:
                        if isinstance(data["metrics"].get(key), int):
                            self.metrics[key] = data["metrics"][key]
        except (OSError, ValueError, TypeError):
            pass
        try:
            for line in self.event_path.read_text(encoding="utf-8").splitlines()[-self.events.maxlen:]:
                try:
                    item = json.loads(line)
                    if isinstance(item, dict):
                        self.events.append(item)
                except ValueError:
                    continue
        except OSError:
            pass

    def persist(self) -> None:
        with self.lock:
            atomic_json(self.state_path, {
                "schema": STATE_SCHEMA,
                "updated_at": now_iso(),
                "calls": self.calls,
                "endpoint_state": self.endpoint_state,
                "metrics": self.metrics,
            })

    def audit(self, action: str, actor: str, subject_id: str, detail: dict[str, Any]) -> None:
        record = {
            "audit_id": str(uuid.uuid4()),
            "timestamp": now_iso(),
            "service": SERVICE,
            "actor": actor,
            "action": action,
            "subject_type": "sip_call" if subject_id else "service",
            "subject_id": subject_id or SERVICE,
            "detail": detail,
        }
        with self.audit_path.open("a", encoding="utf-8") as handle:
            handle.write(json.dumps(record, ensure_ascii=False, separators=(",", ":")) + "\n")

    def publish_mqtt(self, topic: str, payload: dict[str, Any], retain: bool = False, qos: int = 1) -> bool:
        cfg = self.config.mqtt
        if not cfg.get("enabled", True):
            return True
        cmd = [
            "mosquitto_pub", "-h", str(cfg.get("host", "127.0.0.1")),
            "-p", str(cfg.get("port", 1883)), "-q", str(qos),
            "-t", topic, "-m", json.dumps(payload, ensure_ascii=False, separators=(",", ":")),
        ]
        if retain:
            cmd.append("-r")
        try:
            result = subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, timeout=5, check=False)
            return result.returncode == 0
        except (OSError, subprocess.TimeoutExpired):
            return False

    def emit_event(self, event_type: str, severity: str, subject_type: str, subject_id: str, payload: dict[str, Any]) -> dict[str, Any]:
        event = {
            "schema": EVENT_SCHEMA,
            "event_id": str(uuid.uuid4()),
            "event_type": event_type,
            "source": {"service": SERVICE, "instance": self.instance},
            "timestamp": now_iso(),
            "severity": severity,
            "subject": {"type": subject_type, "id": str(subject_id)},
            "payload": payload,
            "deduplication_key": f"{event_type}:{subject_id}:{payload.get('state', payload.get('reason', ''))}",
        }
        with self.lock:
            self.events.append(event)
            with self.event_path.open("a", encoding="utf-8") as handle:
                handle.write(json.dumps(event, ensure_ascii=False, separators=(",", ":")) + "\n")
        prefix = str(self.config.mqtt.get("topic_prefix", "netcore/v1")).rstrip("/")
        topic = f"{prefix}/events/{event_type.replace('.', '/')}"
        self.publish_mqtt(topic, event, retain=False, qos=1)
        return event

    def tbs_by_node(self, node_id: str) -> dict[str, Any] | None:
        wanted = node_id.strip().lower()
        for item in self.config.tbs:
            aliases = [str(item.get("node_id", "")), str(item.get("endpoint_id", ""))]
            aliases.extend(str(value) for value in item.get("aliases", []) if value)
            if wanted in {value.strip().lower() for value in aliases if value}:
                return item
        return None

    def mapping_for(self, number: str) -> dict[str, Any] | None:
        for item in self.config.mappings:
            if not bool_value(item.get("enabled", True), True):
                continue
            pattern = str(item.get("number", "")).strip()
            if pattern.endswith("*") and number.startswith(pattern[:-1]):
                return item
            if pattern == number:
                return item
        return None

    def normalize_tetra_number(self, number: str) -> tuple[int | None, str]:
        cleaned = re.sub(r"[^0-9]", "", number)
        mapping = self.mapping_for(cleaned)
        if mapping:
            target_type = str(mapping.get("target_type", "issi")).lower()
            if target_type != "issi":
                return None, f"mapping target_type {target_type} is not supported by the edge-media switch"
            target = int_value(mapping.get("target"), -1)
            return (target if 0 <= target <= 16_777_215 else None), "explicit_mapping"
        prefix = str(self.config.routing.get("tetra_number_prefix", "")).strip()
        if prefix:
            if not cleaned.startswith(prefix):
                return None, "number does not match tetra_number_prefix"
            if bool_value(self.config.routing.get("strip_tetra_prefix", True), True):
                cleaned = cleaned[len(prefix):]
        if not cleaned or not cleaned.isdigit():
            return None, "destination is not numeric"
        value = int(cleaned)
        if not 0 <= value <= 16_777_215:
            return None, "destination is outside 24-bit SSI range"
        return value, "implicit_issi"

    def normalize_pbx_number(self, number: str) -> tuple[str | None, str]:
        cleaned = re.sub(r"[^0-9*#+]", "", number)
        prefix = str(self.config.routing.get("pbx_outbound_prefix", "")).strip()
        if prefix:
            if not cleaned.startswith(prefix):
                return None, "number does not match pbx_outbound_prefix"
            if bool_value(self.config.routing.get("strip_pbx_outbound_prefix", True), True):
                cleaned = cleaned[len(prefix):]
        if not cleaned or not SAFE_NUMBER.fullmatch(cleaned):
            return None, "invalid PBX destination"
        return cleaned, "pbx_route"

    def mobility_route(self, issi: int) -> tuple[bool, dict[str, Any], str]:
        cfg = self.config.mobility
        if not bool_value(cfg.get("enabled", True), True):
            return False, {}, "mobility_core_disabled"
        base = str(cfg.get("base_url", "http://127.0.0.1:8090")).rstrip("/")
        timeout = float(cfg.get("timeout_secs", 2))
        code, body = http_json("GET", f"{base}/api/v1/subscribers/{issi}/route", timeout=timeout)
        if code != 200 or not isinstance(body, dict):
            self.metrics["mobility_failures"] += 1
            return False, body if isinstance(body, dict) else {}, f"mobility_http_{code or 'unreachable'}"
        state = str(body.get("state", "unknown")).lower()
        accepted = {"confirmed"}
        if bool_value(self.config.routing.get("accept_stale_routes", False)):
            accepted.add("stale")
        node = body.get("serving_node") or body.get("node_id")
        if state not in accepted or not node or not bool_value(body.get("registered", state == "confirmed"), state == "confirmed"):
            return False, body, f"mobility_route_{state}"
        return True, body, "ok"

    def run_asterisk(self, command: str, timeout: float = 4.0) -> tuple[bool, str]:
        binary = str(self.config.asterisk.get("binary", "/usr/sbin/asterisk"))
        try:
            result = subprocess.run([binary, "-rx", command], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=timeout, check=False)
            return result.returncode == 0, result.stdout
        except (OSError, subprocess.TimeoutExpired) as error:
            return False, compact(error)

    def endpoint_has_contact(self, endpoint_id: str, aor_id: str | None = None) -> tuple[bool, str]:
        # For inbound registrations Asterisk resolves the AoR from the user part
        # of the REGISTER To-URI. Therefore the managed TBS AoR is named after
        # the configured registration username, not after an arbitrary endpoint ID.
        lookup_aor = safe_id(aor_id or endpoint_id)
        ok, output = self.run_asterisk(f"pjsip show aor {lookup_aor}")
        if not ok:
            return False, compact(output)
        lowered = output.lower()
        present = "contact:" in lowered or "contact " in lowered
        if "no objects found" in lowered or "0 contacts" in lowered:
            present = False
        return present, compact(output, 1000)

    def pbx_available(self) -> tuple[bool, str]:
        pbx = self.config.pbx
        mode = str(pbx.get("mode", "ip_trunk")).lower()
        endpoint = safe_id(pbx.get("endpoint_id", "netcore-pbx"))
        if mode == "registration":
            ok, output = self.run_asterisk("pjsip show registrations")
            if not ok:
                return False, compact(output)
            wanted = str(pbx.get("registration_id", f"{endpoint}-registration"))
            lower = output.lower()
            return wanted.lower() in lower and ("registered" in lower or "reged" in lower), compact(output, 1000)
        ok, output = self.run_asterisk(f"pjsip show endpoint {endpoint}")
        return ok and "no objects found" not in output.lower(), compact(output, 1000)

    def resolve(self, direction: str, number: str, caller: str = "", source_endpoint: str = "", commit: bool = True, check_contact: bool = True) -> dict[str, Any]:
        direction = direction.strip().lower()
        number = compact(number, 80)
        caller = compact(caller, 80)
        source_endpoint = safe_id(source_endpoint)
        self.metrics["route_requests"] += 1
        token = uuid.uuid4().hex[:16]
        result: dict[str, Any] = {
            "schema": "netcore-sip-route-v1",
            "call_token": token,
            "direction": direction,
            "number": number,
            "caller": caller,
            "source_endpoint": source_endpoint,
            "action": "reject",
            "reason": "unresolved",
            "destination": "",
            "endpoint": "",
            "aor": "",
            "node_id": None,
            "issi": None,
            "dial_timeout_secs": int(self.config.routing.get("dial_timeout_secs", 60)),
            "created_at": now_iso(),
        }
        if direction == "inbound":
            issi, source = self.normalize_tetra_number(number)
            if issi is None:
                result["reason"] = source
            else:
                ok, route, reason = self.mobility_route(issi)
                result["issi"] = issi
                result["mobility"] = route
                if not ok:
                    result["reason"] = reason
                else:
                    node_id = str(route.get("serving_node") or route.get("node_id"))
                    tbs = self.tbs_by_node(node_id)
                    if not tbs or not bool_value(tbs.get("enabled", True), True):
                        result["reason"] = "serving_tbs_not_configured"
                        result["node_id"] = node_id
                    else:
                        endpoint = safe_id(tbs.get("endpoint_id") or f"tbs-{node_id.lower()}")
                        aor_id = safe_id(tbs.get("username") or endpoint)
                        contact_ok, contact_detail = self.endpoint_has_contact(endpoint, aor_id) if check_contact else (True, "contact check skipped")
                        result["contact"] = {"available": contact_ok, "detail": contact_detail}
                        if bool_value(self.config.routing.get("require_tbs_contact", True), True) and not contact_ok:
                            result["reason"] = "serving_tbs_has_no_sip_contact"
                            result["node_id"] = node_id
                            result["endpoint"] = endpoint
                            result["aor"] = aor_id
                        else:
                            result.update({
                                "action": "tbs",
                                "reason": source,
                                "destination": str(issi),
                                "endpoint": endpoint,
                                "aor": aor_id,
                                "node_id": node_id,
                            })
        elif direction == "outbound":
            destination, reason = self.normalize_pbx_number(number)
            if destination is None:
                result["reason"] = reason
            else:
                result.update({
                    "action": "pbx",
                    "reason": reason,
                    "destination": destination,
                    "endpoint": safe_id(self.config.pbx.get("endpoint_id", "netcore-pbx")),
                    "aor": safe_id(f"{self.config.pbx.get('endpoint_id', 'netcore-pbx')}-aor"),
                    "node_id": self.node_for_endpoint(source_endpoint),
                })
        else:
            result["reason"] = "direction must be inbound or outbound"

        accepted = result["action"] in {"tbs", "pbx"}
        if accepted:
            self.metrics["routes_resolved"] += 1
        else:
            self.metrics["routes_rejected"] += 1
        with self.lock:
            self.decisions.appendleft(result)
            if commit:
                call = {
                    **result,
                    "state": "routed" if accepted else "rejected",
                    "updated_at": now_iso(),
                    "answered_at": None,
                    "ended_at": now_iso() if not accepted else None,
                    "dial_status": None,
                    "hangup_cause": None,
                }
                self.calls[token] = call
                self.persist()
        event_type = "sip.route_resolved" if accepted else "sip.route_failed"
        self.emit_event(event_type, "info" if accepted else "warning", "sip_call", token, {
            "direction": direction,
            "number": number,
            "caller": caller,
            "action": result["action"],
            "endpoint": result["endpoint"],
            "aor": result["aor"],
            "node_id": result["node_id"],
            "issi": result["issi"],
            "reason": result["reason"],
        })
        self.audit("route_resolve", "agi" if commit else "api-test", token, result)
        return result

    def node_for_endpoint(self, endpoint: str) -> str | None:
        endpoint = endpoint.lower()
        for item in self.config.tbs:
            if str(item.get("endpoint_id", "")).lower() == endpoint:
                return str(item.get("node_id", "")) or None
        return None

    def update_call(self, token: str, state: str, detail: dict[str, Any]) -> tuple[bool, dict[str, Any]]:
        state = safe_id(state, 40).lower()
        with self.lock:
            call = self.calls.get(token)
            if not call:
                return False, {"error": "call token not found"}
            previous = call.get("state")
            call["state"] = state
            call["updated_at"] = now_iso()
            call.update({key: value for key, value in detail.items() if key in {"dial_status", "hangup_cause", "channel", "uniqueid", "linkedid"}})
            if state == "dialing" and previous != "dialing":
                self.metrics["calls_started"] += 1
            if state == "answered" and not call.get("answered_at"):
                call["answered_at"] = now_iso()
                self.metrics["calls_answered"] += 1
            if state in TERMINAL_STATES and not call.get("ended_at"):
                call["ended_at"] = now_iso()
                self.metrics["calls_ended"] += 1
            self.persist()
            snapshot = dict(call)
        event_type = {
            "dialing": "sip.call_started",
            "answered": "sip.call_answered",
            "ended": "sip.call_ended",
            "failed": "sip.call_failed",
        }.get(state, "sip.call_updated")
        self.emit_event(event_type, "info" if state not in {"failed", "rejected"} else "warning", "sip_call", token, {
            "previous_state": previous,
            "state": state,
            **detail,
        })
        self.audit("call_state", "agi", token, {"previous": previous, "state": state, **detail})
        return True, snapshot

    def render_asterisk(self) -> dict[str, Any]:
        cfg = self.config.asterisk
        config_dir = Path(str(cfg.get("config_dir", "/etc/asterisk")))
        config_dir.mkdir(parents=True, exist_ok=True)
        pjsip = self._render_pjsip()
        extensions = self._render_extensions()
        rtp = self._render_rtp()
        files = {
            config_dir / "netcore-pjsip.conf": pjsip,
            config_dir / "netcore-extensions.conf": extensions,
            config_dir / "netcore-rtp.conf": rtp,
        }
        for path, content in files.items():
            temp = path.with_suffix(path.suffix + ".tmp")
            temp.write_text(content, encoding="utf-8")
            temp.replace(path)
        self.audit("asterisk_render", "api", SERVICE, {"files": [str(path) for path in files]})
        self.emit_event("sip.asterisk_config_rendered", "info", "service", SERVICE, {"files": [str(path) for path in files]})
        return {"rendered": [str(path) for path in files], "tbs": len(self.config.tbs)}

    @staticmethod
    def _asterisk_value(value: Any) -> str:
        text = str(value or "").strip()
        if any(char in text for char in "\r\n[]"):
            raise ValueError("unsafe character in Asterisk configuration value")
        return text

    def _render_pjsip(self) -> str:
        cfg = self.config.asterisk
        pbx = self.config.pbx
        transport = str(pbx.get("transport", "udp")).lower()
        if transport not in {"udp", "tcp"}:
            raise ValueError("OPEN LAB SIP switch supports udp or tcp transport")
        bind = self._asterisk_value(cfg.get("sip_bind", "0.0.0.0:5060"))
        lines = [
            "; Generated by netcore-sip-switch. Do not edit by hand.",
            "[netcore-transport]",
            "type=transport",
            f"protocol={transport}",
            f"bind={bind}",
            "allow_reload=yes",
            "",
        ]
        endpoint = safe_id(pbx.get("endpoint_id", "netcore-pbx"))
        aor = f"{endpoint}-aor"
        auth = f"{endpoint}-auth"
        host = self._asterisk_value(pbx.get("host", "127.0.0.1"))
        port = int_value(pbx.get("port"), 5060)
        username = self._asterisk_value(pbx.get("username", ""))
        auth_username = self._asterisk_value(pbx.get("auth_username", username))
        password = self._asterisk_value(pbx.get("password", ""))
        mode = str(pbx.get("mode", "ip_trunk")).lower()
        allow = self._asterisk_value(pbx.get("allow", "ulaw")) or "ulaw"
        lines.extend([
            f"[{endpoint}]",
            "type=endpoint",
            "transport=netcore-transport",
            "context=netcore-from-pbx",
            "disallow=all",
            f"allow={allow}",
            f"aors={aor}",
            "direct_media=no",
            "rewrite_contact=yes",
            "rtp_symmetric=yes",
            "force_rport=yes",
            "trust_id_inbound=yes",
            "send_pai=yes",
        ])
        if password:
            lines.append(f"outbound_auth={auth}")
        from_user = self._asterisk_value(pbx.get("from_user", username))
        from_domain = self._asterisk_value(pbx.get("from_domain", host))
        if from_user:
            lines.append(f"from_user={from_user}")
        if from_domain:
            lines.append(f"from_domain={from_domain}")
        lines.extend(["", f"[{aor}]", "type=aor", f"contact=sip:{host}:{port}", "qualify_frequency=15", ""])
        if password:
            lines.extend([f"[{auth}]", "type=auth", "auth_type=userpass", f"username={auth_username}", f"password={password}", ""])
        matches = pbx.get("match", [])
        if not isinstance(matches, list) or not matches:
            matches = [host]
        lines.extend([f"[{endpoint}-identify]", "type=identify", f"endpoint={endpoint}"])
        for match in matches:
            lines.append(f"match={self._asterisk_value(match)}")
        lines.append("")
        if mode == "registration":
            registration_id = safe_id(pbx.get("registration_id", f"{endpoint}-registration"))
            contact_user = self._asterisk_value(pbx.get("contact_user", username or "netcore-tetra"))
            client_user = username or contact_user
            lines.extend([
                f"[{registration_id}]",
                "type=registration",
                "transport=netcore-transport",
                f"server_uri=sip:{host}:{port}",
                f"client_uri=sip:{client_user}@{host}",
                f"contact_user={contact_user}",
                "retry_interval=5",
                "forbidden_retry_interval=30",
                f"expiration={int_value(pbx.get("registration_expiration_secs"), 30)}",
            ])
            if password:
                lines.append(f"outbound_auth={auth}")
            lines.append("")
        elif mode != "ip_trunk":
            raise ValueError("pbx.mode must be ip_trunk or registration")

        for item in self.config.tbs:
            if not bool_value(item.get("enabled", True), True):
                continue
            node_id = self._asterisk_value(item.get("node_id", ""))
            endpoint_id = safe_id(item.get("endpoint_id") or f"tbs-{node_id.lower()}")
            username = self._asterisk_value(item.get("username", endpoint_id))
            password = self._asterisk_value(item.get("password", ""))
            if not node_id or not endpoint_id or not username:
                raise ValueError("each enabled TBS needs node_id, endpoint_id and username")
            aor_id = safe_id(username)
            lines.extend([
                f"[{endpoint_id}]",
                "type=endpoint",
                "transport=netcore-transport",
                "context=netcore-from-tbs",
                "disallow=all",
                "allow=ulaw",
                "identify_by=auth_username,username",
                f"auth={endpoint_id}-auth",
                f"aors={aor_id}",
                "direct_media=no",
                "rewrite_contact=yes",
                "rtp_symmetric=yes",
                "force_rport=yes",
                "trust_id_inbound=yes",
                "send_pai=yes",
                "",
                f"[{endpoint_id}-auth]",
                "type=auth",
                "auth_type=userpass",
                f"username={username}",
                f"password={password}",
                "",
                f"[{aor_id}]",
                "type=aor",
                f"max_contacts={max(1, int_value(item.get('max_contacts'), 1))}",
                "remove_existing=yes",
                "qualify_frequency=10",
                "qualify_timeout=3.0",
                "",
            ])
        return "\n".join(lines).rstrip() + "\n"

    def _render_extensions(self) -> str:
        agi = str(self.config.asterisk.get("agi_script", "netcore-sip-route.py"))
        timeout = int(self.config.routing.get("dial_timeout_secs", 60))
        return f"""; Generated by netcore-sip-switch. Do not edit by hand.
[netcore-from-pbx]
exten => _X!,1,NoOp(NetCore PBX -> TETRA ${{EXTEN}})
 same => n,Set(NETCORE_CALL_TOKEN=)
 same => n,AGI({agi},resolve,inbound,${{EXTEN}},${{CALLERID(num)}})
 same => n,GotoIf($[\"${{NETCORE_ACTION}}\"=\"tbs\"]?route)
 same => n,AGI({agi},state,${{NETCORE_CALL_TOKEN}},rejected)
 same => n,Hangup(20)
 same => n(route),Set(NETCORE_DIAL=${{PJSIP_DIAL_CONTACTS(${{NETCORE_ENDPOINT}},${{NETCORE_AOR}},${{NETCORE_DESTINATION}})}})
 same => n,GotoIf($[\"${{NETCORE_DIAL}}\"=\"\"]?unavailable)
 same => n,AGI({agi},state,${{NETCORE_CALL_TOKEN}},dialing)
 same => n,Dial(${{NETCORE_DIAL}},${{NETCORE_DIAL_TIMEOUT}},U(netcore-mark-answer^${{NETCORE_CALL_TOKEN}}))
 same => n,Goto(done)
 same => n(unavailable),AGI({agi},state,${{NETCORE_CALL_TOKEN}},failed,no_contact)
 same => n(done),Hangup()
exten => h,1,AGI({agi},hangup,${{NETCORE_CALL_TOKEN}},${{DIALSTATUS}},${{HANGUPCAUSE}})

[netcore-from-tbs]
exten => _X!,1,NoOp(NetCore TBS -> PBX ${{EXTEN}})
 same => n,Set(NETCORE_CALL_TOKEN=)
 same => n,AGI({agi},resolve,outbound,${{EXTEN}},${{CALLERID(num)}})
 same => n,GotoIf($[\"${{NETCORE_ACTION}}\"=\"pbx\"]?route)
 same => n,AGI({agi},state,${{NETCORE_CALL_TOKEN}},rejected)
 same => n,Hangup(20)
 same => n(route),AGI({agi},state,${{NETCORE_CALL_TOKEN}},dialing)
 same => n,Dial(PJSIP/${{NETCORE_DESTINATION}}@${{NETCORE_ENDPOINT}},${{NETCORE_DIAL_TIMEOUT}},U(netcore-mark-answer^${{NETCORE_CALL_TOKEN}}))
 same => n,Hangup()
exten => h,1,AGI({agi},hangup,${{NETCORE_CALL_TOKEN}},${{DIALSTATUS}},${{HANGUPCAUSE}})

[netcore-mark-answer]
exten => s,1,AGI({agi},state,${{ARG1}},answered)
 same => n,Return()
"""

    def _render_rtp(self) -> str:
        start = int_value(self.config.asterisk.get("rtp_start"), 10000)
        end = int_value(self.config.asterisk.get("rtp_end"), 20000)
        if start <= 0 or end <= start or end > 65535:
            raise ValueError("invalid RTP port range")
        return f"; Generated by netcore-sip-switch.\n[general]\nrtpstart={start}\nrtpend={end}\nstrictrtp=yes\n"

    def reload_asterisk(self, restart: bool = False) -> dict[str, Any]:
        command = "core restart now" if restart else "core reload"
        ok, output = self.run_asterisk(command, timeout=15)
        if ok:
            self.metrics["asterisk_reloads"] += 1
        self.emit_event("sip.asterisk_reloaded" if ok else "sip.asterisk_reload_failed", "info" if ok else "error", "service", SERVICE, {"restart": restart, "output": compact(output, 1000)})
        return {"ok": ok, "command": command, "output": output}

    def probe(self) -> None:
        previous = dict(self.endpoint_state)
        asterisk_ok, _ = self.run_asterisk("core show version")
        mobility_base = str(self.config.mobility.get("base_url", "http://127.0.0.1:8090")).rstrip("/")
        mobility_code, _ = http_json("GET", f"{mobility_base}/health/live", timeout=float(self.config.mobility.get("timeout_secs", 2)))
        pbx_ok, pbx_detail = self.pbx_available() if asterisk_ok else (False, "asterisk unavailable")
        current: dict[str, dict[str, Any]] = {}
        for tbs in self.config.tbs:
            endpoint = safe_id(tbs.get("endpoint_id") or f"tbs-{str(tbs.get('node_id', '')).lower()}")
            aor_id = safe_id(tbs.get("username") or endpoint)
            registered, detail = self.endpoint_has_contact(endpoint, aor_id) if asterisk_ok and bool_value(tbs.get("enabled", True), True) else (False, "disabled")
            current[endpoint] = {
                "node_id": tbs.get("node_id"),
                "endpoint_id": endpoint,
                "aor_id": aor_id,
                "enabled": bool_value(tbs.get("enabled", True), True),
                "registered": registered,
                "detail": detail,
                "checked_at": now_iso(),
            }
            old = previous.get(endpoint, {}).get("registered")
            if old is not None and old != registered:
                self.emit_event("sip.tbs_contact_up" if registered else "sip.tbs_contact_down", "info" if registered else "warning", "tbs", str(tbs.get("node_id", endpoint)), {"endpoint": endpoint, "state": "up" if registered else "down"})
        with self.lock:
            self.endpoint_state = current
            self.health.update({
                "asterisk": asterisk_ok,
                "mobility_core": mobility_code == 200,
                "pbx": pbx_ok,
                "last_probe_at": now_iso(),
                "last_error": None if asterisk_ok and mobility_code == 200 else "one or more dependencies unavailable",
                "pbx_detail": pbx_detail,
            })
            self.persist()
        prefix = str(self.config.mqtt.get("topic_prefix", "netcore/v1")).rstrip("/")
        self.publish_mqtt(f"{prefix}/state/services/sip-switch", self.status(), retain=True, qos=1)

    def background(self) -> None:
        interval = max(3, int_value(self.config.raw.get("management", {}).get("probe_interval_secs"), 10))
        while not self.stop_event.is_set():
            try:
                self.probe()
            except Exception as error:  # service must keep monitoring after one bad probe
                with self.lock:
                    self.health["last_error"] = compact(error)
                    self.health["last_probe_at"] = now_iso()
            self.stop_event.wait(interval)

    def status(self) -> dict[str, Any]:
        with self.lock:
            active = sum(1 for item in self.calls.values() if item.get("state") not in TERMINAL_STATES)
            return {
                "service": SERVICE,
                "phase": 11,
                "mode": "open_lab",
                "started_at": self.started_at,
                "instance": self.instance,
                "health": dict(self.health),
                "calls_total": len(self.calls),
                "calls_active": active,
                "tbs_configured": len(self.config.tbs),
                "tbs_registered": sum(1 for item in self.endpoint_state.values() if item.get("registered")),
                "pbx_mode": self.config.pbx.get("mode", "ip_trunk"),
                "pbx_endpoint": self.config.pbx.get("endpoint_id", "netcore-pbx"),
                "media_mode": "edge_media",
                "central_media_ready": False,
                "metrics": dict(self.metrics),
            }

    def metrics_text(self) -> str:
        status = self.status()
        lines = [
            "# HELP netcore_sip_switch_up Service liveness.",
            "# TYPE netcore_sip_switch_up gauge",
            "netcore_sip_switch_up 1",
            f"netcore_sip_switch_asterisk_up {1 if status['health'].get('asterisk') else 0}",
            f"netcore_sip_switch_mobility_up {1 if status['health'].get('mobility_core') else 0}",
            f"netcore_sip_switch_pbx_up {1 if status['health'].get('pbx') else 0}",
            f"netcore_sip_switch_calls_active {status['calls_active']}",
            f"netcore_sip_switch_tbs_registered {status['tbs_registered']}",
        ]
        for key, value in self.metrics.items():
            lines.append(f"netcore_sip_switch_{key} {value}")
        return "\n".join(lines) + "\n"


class Handler(BaseHTTPRequestHandler):
    server_version = "NetCoreSipSwitch/1"

    @property
    def app(self) -> SipSwitch:
        return self.server.app  # type: ignore[attr-defined]

    def log_message(self, fmt: str, *args: Any) -> None:
        print(f"{self.address_string()} - {fmt % args}")

    def send_json(self, code: int, value: Any) -> None:
        body = json.dumps(value, ensure_ascii=False, indent=2).encode("utf-8")
        self.send_response(code)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()
        self.wfile.write(body)

    def send_text(self, code: int, value: str, content_type: str = "text/plain; charset=utf-8") -> None:
        body = value.encode("utf-8")
        self.send_response(code)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()
        self.wfile.write(body)

    def body_json(self) -> dict[str, Any]:
        length = int(self.headers.get("Content-Length", "0") or 0)
        raw = self.rfile.read(length) if length else b"{}"
        value = json.loads(raw.decode("utf-8"))
        if not isinstance(value, dict):
            raise ValueError("JSON object required")
        return value

    def do_OPTIONS(self) -> None:
        self.send_response(204)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.send_header("Access-Control-Allow-Methods", "GET,POST,OPTIONS")
        self.end_headers()

    def do_GET(self) -> None:
        parsed = urlparse(self.path)
        query = parse_qs(parsed.query)
        path = parsed.path
        if path == "/":
            self.send_text(200, INDEX_HTML, "text/html; charset=utf-8")
        elif path == "/health/live":
            self.send_json(200, {"status": "live", "service": SERVICE})
        elif path == "/health/ready":
            status = self.app.status()
            ready = status["health"].get("asterisk") and status["health"].get("mobility_core") and status["health"].get("pbx")
            self.send_json(200 if ready else 503, status)
        elif path == "/api/v1/status":
            self.send_json(200, self.app.status())
        elif path == "/api/v1/tbs":
            result = []
            for item in self.app.config.tbs:
                endpoint = safe_id(item.get("endpoint_id") or f"tbs-{str(item.get('node_id', '')).lower()}")
                result.append({**item, "runtime": self.app.endpoint_state.get(endpoint, {})})
            self.send_json(200, result)
        elif path == "/api/v1/mappings":
            self.send_json(200, self.app.config.mappings)
        elif path == "/api/v1/calls":
            limit = min(1000, int_value(query.get("limit", [200])[0], 200))
            calls = sorted(self.app.calls.values(), key=lambda item: item.get("created_at", ""), reverse=True)[:limit]
            self.send_json(200, calls)
        elif path == "/api/v1/decisions":
            limit = min(1000, int_value(query.get("limit", [200])[0], 200))
            self.send_json(200, list(self.app.decisions)[:limit])
        elif path == "/api/v1/events":
            limit = min(1000, int_value(query.get("limit", [200])[0], 200))
            self.send_json(200, list(self.app.events)[-limit:])
        elif path == "/api/v1/config":
            redacted = json.loads(json.dumps(self.app.config.raw))
            if isinstance(redacted.get("pbx"), dict) and redacted["pbx"].get("password"):
                redacted["pbx"]["password"] = "***"
            for item in redacted.get("tbs", []):
                if item.get("password"):
                    item["password"] = "***"
            self.send_json(200, redacted)
        elif path == "/api/v1/resolve":
            direction = query.get("direction", ["inbound"])[0]
            number = query.get("number", [""])[0]
            caller = query.get("caller", [""])[0]
            check_contact = bool_value(query.get("check_contact", ["false"])[0])
            self.send_json(200, self.app.resolve(direction, number, caller, commit=False, check_contact=check_contact))
        elif path == "/metrics":
            self.send_text(200, self.app.metrics_text(), "text/plain; version=0.0.4; charset=utf-8")
        elif path == "/openapi.json":
            self.send_json(200, OPENAPI)
        else:
            self.send_json(404, {"error": "not found"})

    def do_POST(self) -> None:
        parsed = urlparse(self.path)
        path = parsed.path
        try:
            body = self.body_json()
        except (ValueError, json.JSONDecodeError) as error:
            self.send_json(400, {"error": compact(error)})
            return
        if path == "/api/v1/resolve":
            result = self.app.resolve(
                str(body.get("direction", "inbound")),
                str(body.get("number", "")),
                str(body.get("caller", "")),
                str(body.get("source_endpoint", "")),
                commit=bool_value(body.get("commit", True), True),
                check_contact=bool_value(body.get("check_contact", True), True),
            )
            self.send_json(200 if result["action"] != "reject" else 409, result)
        elif path.startswith("/api/v1/calls/") and path.endswith("/state"):
            token = path.split("/")[4]
            ok, result = self.app.update_call(token, str(body.get("state", "updated")), body)
            self.send_json(200 if ok else 404, result)
        elif path == "/api/v1/actions/render-asterisk":
            try:
                self.send_json(200, self.app.render_asterisk())
            except (OSError, ValueError) as error:
                self.send_json(409, {"error": compact(error)})
        elif path == "/api/v1/actions/reload-asterisk":
            result = self.app.reload_asterisk(restart=bool_value(body.get("restart", False)))
            self.send_json(200 if result["ok"] else 503, result)
        elif path == "/api/v1/actions/probe":
            self.app.probe()
            self.send_json(200, self.app.status())
        else:
            self.send_json(404, {"error": "not found"})


INDEX_HTML = r"""<!doctype html><html lang="de"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>NetCore SIP Switch</title><style>:root{color-scheme:dark;--bg:#071118;--panel:#111f29;--line:#294250;--text:#ecf5f8;--muted:#9fb2bd;--ok:#57d98e;--warn:#ffca58;--bad:#ff6d6d;--accent:#36a3ff}*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font:14px system-ui}.lab{padding:10px 20px;background:#8e2020;color:#fff;text-align:center;font-weight:800}header{padding:20px 26px;background:#0e1a22;border-bottom:1px solid var(--line);display:flex;justify-content:space-between;gap:15px}h1,h2{margin:0 0 10px}.wrap{padding:20px;display:grid;gap:16px}.cards{display:grid;grid-template-columns:repeat(auto-fit,minmax(150px,1fr));gap:10px}.card,.panel{background:var(--panel);border:1px solid var(--line);border-radius:10px;padding:14px}.value{font-size:24px;font-weight:760}.muted{color:var(--muted)}.ok{color:var(--ok)}.bad{color:var(--bad)}.warn{color:var(--warn)}.toolbar{display:flex;gap:8px;flex-wrap:wrap;margin:10px 0}input,select,button{background:#192c37;color:var(--text);border:1px solid var(--line);border-radius:6px;padding:8px}button{cursor:pointer}.primary{background:#1268aa}.danger{background:#893039}table{width:100%;border-collapse:collapse}th,td{text-align:left;padding:8px;border-bottom:1px solid var(--line);vertical-align:top}.tablewrap{overflow:auto}pre{white-space:pre-wrap;max-height:320px;overflow:auto}@media(max-width:750px){header{display:block}.toolbar>*{flex:1 1 150px}}</style></head><body><div class="lab">⚠ OPEN LAB – keine Anmeldung, keine Tokens, kein TLS. Asterisk dient nur als SIP-B2BUA/Router; keine zweite PBX.</div><header><div><h1>NetCore SIP Switch</h1><div class="muted">Ein PBX-Trunk, zentrale TBS-Registrierungen und Mobility-Core-Routing</div></div><div id="health">Status …</div></header><main class="wrap"><section class="cards" id="cards"></section><section class="panel"><h2>Route testen</h2><div class="toolbar"><select id="direction"><option value="inbound">PBX → TETRA</option><option value="outbound">TBS → PBX</option></select><input id="number" placeholder="Rufnummer / ISSI"><input id="caller" placeholder="Anrufer optional"><button class="primary" onclick="testRoute()">Auflösen</button><button onclick="renderAsterisk()">Asterisk neu rendern</button><button onclick="reloadAsterisk()">Asterisk neu laden</button></div><pre id="routeResult" class="muted"></pre></section><section class="panel"><h2>Basisstationen</h2><div class="tablewrap"><table><thead><tr><th>Node</th><th>Endpoint</th><th>Benutzer</th><th>Kontakt</th><th>Detail</th></tr></thead><tbody id="tbsRows"></tbody></table></div></section><section class="panel"><h2>Letzte Routen</h2><div class="tablewrap"><table><thead><tr><th>Zeit</th><th>Richtung</th><th>Ziel</th><th>Aktion</th><th>Route</th><th>Grund</th></tr></thead><tbody id="decisionRows"></tbody></table></div></section><section class="panel"><h2>SIP-Rufe</h2><div class="tablewrap"><table><thead><tr><th>Token</th><th>Richtung</th><th>Ziel</th><th>Status</th><th>TBS</th><th>Dialstatus</th></tr></thead><tbody id="callRows"></tbody></table></div></section><section class="panel"><h2>Ereignisse</h2><pre id="events" class="muted"></pre></section></main><script>const el=id=>document.getElementById(id);async function api(path,opt){const r=await fetch(path,opt);let v={};try{v=await r.json()}catch{}if(!r.ok)throw new Error(v.error||v.reason||r.statusText);return v}function esc(v){return String(v??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]))}async function refresh(){try{const[s,t,d,c,e]=await Promise.all([api('/api/v1/status'),api('/api/v1/tbs'),api('/api/v1/decisions?limit=30'),api('/api/v1/calls?limit=30'),api('/api/v1/events?limit=30')]);const h=s.health;el('health').innerHTML=(h.asterisk?'<span class="ok">● Asterisk</span>':'<span class="bad">● Asterisk</span>')+' &nbsp; '+(h.mobility_core?'<span class="ok">● Mobility</span>':'<span class="bad">● Mobility</span>')+' &nbsp; '+(h.pbx?'<span class="ok">● PBX</span>':'<span class="bad">● PBX</span>');el('cards').innerHTML=[['Rufe aktiv',s.calls_active],['Rufe gesamt',s.calls_total],['TBS konfiguriert',s.tbs_configured],['TBS registriert',s.tbs_registered],['Routen OK',s.metrics.routes_resolved],['Routen abgelehnt',s.metrics.routes_rejected],['Medienmodus',s.media_mode],['PBX-Modus',s.pbx_mode]].map(x=>`<div class="card"><div class="muted">${x[0]}</div><div class="value">${esc(x[1])}</div></div>`).join('');el('tbsRows').innerHTML=t.map(x=>`<tr><td>${esc(x.node_id)}</td><td>${esc(x.endpoint_id)}</td><td>${esc(x.username)}</td><td>${x.runtime?.registered?'<span class="ok">registriert</span>':'<span class="bad">kein Kontakt</span>'}</td><td class="muted">${esc(x.runtime?.detail||'')}</td></tr>`).join('');el('decisionRows').innerHTML=d.map(x=>`<tr><td>${esc(x.created_at)}</td><td>${esc(x.direction)}</td><td>${esc(x.number)}</td><td>${esc(x.action)}</td><td>${esc(x.node_id||x.endpoint)}</td><td>${esc(x.reason)}</td></tr>`).join('');el('callRows').innerHTML=c.map(x=>`<tr><td>${esc(x.call_token)}</td><td>${esc(x.direction)}</td><td>${esc(x.number)}</td><td>${esc(x.state)}</td><td>${esc(x.node_id||'–')}</td><td>${esc(x.dial_status||'–')}</td></tr>`).join('');el('events').textContent=e.map(x=>`${x.timestamp} ${x.event_type} ${x.subject?.id||''}`).join('\n')}catch(e){el('health').innerHTML='<span class="bad">UI-Fehler: '+esc(e.message)+'</span>'}}async function testRoute(){try{const q=new URLSearchParams({direction:direction.value,number:number.value,caller:caller.value,check_contact:'false'});const r=await api('/api/v1/resolve?'+q);el('routeResult').textContent=JSON.stringify(r,null,2)}catch(e){el('routeResult').textContent=e.message}}async function renderAsterisk(){try{const r=await api('/api/v1/actions/render-asterisk',{method:'POST',headers:{'Content-Type':'application/json'},body:'{}'});alert(JSON.stringify(r))}catch(e){alert(e.message)}}async function reloadAsterisk(){try{const r=await api('/api/v1/actions/reload-asterisk',{method:'POST',headers:{'Content-Type':'application/json'},body:'{}'});alert(r.output||'OK');refresh()}catch(e){alert(e.message)}}refresh();setInterval(refresh,3000)</script></body></html>"""

OPENAPI = {
    "openapi": "3.0.3",
    "info": {"title": "NetCore SIP Switch", "version": "1.0.0"},
    "paths": {
        "/health/live": {"get": {}},
        "/health/ready": {"get": {}},
        "/api/v1/status": {"get": {}},
        "/api/v1/tbs": {"get": {}},
        "/api/v1/mappings": {"get": {}},
        "/api/v1/calls": {"get": {}},
        "/api/v1/decisions": {"get": {}},
        "/api/v1/events": {"get": {}},
        "/api/v1/resolve": {"get": {}, "post": {}},
        "/api/v1/calls/{token}/state": {"post": {}},
        "/api/v1/actions/render-asterisk": {"post": {}},
        "/api/v1/actions/reload-asterisk": {"post": {}},
        "/metrics": {"get": {}},
    },
}


def load_config(path: Path) -> Config:
    with path.open("rb") as handle:
        raw = tomllib.load(handle)
    if str(raw.get("security", {}).get("mode", "")) != "open_lab":
        raise SystemExit("Phase 11 supports security.mode = open_lab only")
    return Config(raw, path)


def main() -> None:
    parser = argparse.ArgumentParser(description="NetCore SIP Switch")
    parser.add_argument("--config", default="/etc/netcore/sip-switch.toml")
    parser.add_argument("--render-asterisk", action="store_true")
    parser.add_argument("--probe-once", action="store_true")
    args = parser.parse_args()
    config = load_config(Path(args.config))
    app = SipSwitch(config)
    if args.render_asterisk:
        print(json.dumps(app.render_asterisk(), ensure_ascii=False, indent=2))
        return
    if args.probe_once:
        app.probe()
        print(json.dumps(app.status(), ensure_ascii=False, indent=2))
        return
    host, port = config.bind
    server = ThreadingHTTPServer((host, port), Handler)
    server.app = app  # type: ignore[attr-defined]
    monitor = threading.Thread(target=app.background, name="sip-switch-monitor", daemon=True)
    monitor.start()

    def stop(*_: Any) -> None:
        app.stop_event.set()
        threading.Thread(target=server.shutdown, daemon=True).start()

    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    print(f"{SERVICE} OPEN LAB listening on {host}:{port}")
    try:
        server.serve_forever(poll_interval=0.5)
    finally:
        app.stop_event.set()
        app.persist()
        server.server_close()


if __name__ == "__main__":
    main()
