#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import re
import signal
import subprocess
import threading
import time
import tomllib
import uuid
from collections import deque
from copy import deepcopy
from datetime import datetime, timedelta, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any
from urllib.error import HTTPError, URLError
from urllib.parse import parse_qs, urlparse
from urllib.request import Request, urlopen

SCHEMA = "netcore-alarm-workflow-state-v1"
EVENT_SCHEMA = "netcore-event-v1"
ACTIVE_STATES = {"open", "acknowledged", "assigned", "in_progress"}
TERMINAL_STATES = {"closed", "cancelled"}
ALLOWED_TRANSITIONS = {
    "open": {"acknowledged", "assigned", "in_progress", "resolved", "cancelled"},
    "acknowledged": {"assigned", "in_progress", "resolved", "cancelled"},
    "assigned": {"acknowledged", "in_progress", "resolved", "cancelled"},
    "in_progress": {"acknowledged", "assigned", "resolved", "cancelled"},
    "resolved": {"closed", "open", "cancelled"},
    "closed": {"open"},
    "cancelled": {"open"},
}
SEVERITY_ORDER = {
    "debug": 0,
    "info": 1,
    "notice": 2,
    "warning": 3,
    "error": 4,
    "critical": 5,
    "emergency": 6,
}


def utc_now() -> datetime:
    return datetime.now(timezone.utc)


def now_iso() -> str:
    return utc_now().isoformat(timespec="milliseconds").replace("+00:00", "Z")


def parse_time(value: Any) -> datetime | None:
    if not isinstance(value, str) or not value:
        return None
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00")).astimezone(timezone.utc)
    except ValueError:
        return None


def safe_identifier(value: Any, maximum: int = 160) -> str:
    text = str(value or "")
    text = "".join(char for char in text if ord(char) >= 32 and ord(char) != 127)
    return text[:maximum]


def compact_text(value: Any, maximum: int = 220) -> str:
    text = re.sub(r"\s+", " ", str(value or "")).strip()
    return text[:maximum]


def nested_get(value: Any, path: str) -> Any:
    current = value
    for part in path.split("."):
        if not isinstance(current, dict) or part not in current:
            return None
        current = current[part]
    return current


def atomic_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = path.with_suffix(path.suffix + ".tmp")
    temp.write_text(json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    temp.replace(path)


def event_matches(pattern: str, event_type: str) -> bool:
    if pattern.endswith("*"):
        return event_type.startswith(pattern[:-1])
    return event_type == pattern


def render_template(template: str, event: dict[str, Any], alarm: dict[str, Any] | None = None) -> str:
    payload = event.get("payload") if isinstance(event.get("payload"), dict) else {}
    subject = event.get("subject") if isinstance(event.get("subject"), dict) else {}
    values: dict[str, Any] = {
        "event_type": event.get("event_type", ""),
        "severity": event.get("severity", ""),
        "subject_id": subject.get("id", ""),
        "subject_type": subject.get("type", ""),
        "source_service": (event.get("source") or {}).get("service", "") if isinstance(event.get("source"), dict) else "",
        "alarm_id": (alarm or {}).get("alarm_id", ""),
        "token": (alarm or {}).get("token", ""),
        "title": (alarm or {}).get("title", ""),
    }
    for key, item in payload.items():
        if isinstance(item, (str, int, float, bool)) or item is None:
            values[f"payload_{key}"] = item
    class Safe(dict):
        def __missing__(self, key: str) -> str:
            return "{" + key + "}"
    try:
        return compact_text(template.format_map(Safe(values)), 500)
    except Exception:
        return compact_text(template, 500)


class Config:
    def __init__(self, raw: dict[str, Any]):
        self.raw = raw

    @property
    def bind(self) -> tuple[str, int]:
        host, port = str(self.raw["server"]["bind"]).rsplit(":", 1)
        return host, int(port)

    @property
    def mqtt(self) -> dict[str, Any]:
        return self.raw.get("mqtt", {})

    @property
    def sds(self) -> dict[str, Any]:
        return self.raw.get("sds_router", {})

    @property
    def storage(self) -> dict[str, Any]:
        return self.raw["storage"]

    @property
    def workflow(self) -> dict[str, Any]:
        return self.raw.get("workflow", {})

    @property
    def recipients(self) -> list[dict[str, Any]]:
        return self.raw.get("recipients", [])

    @property
    def profiles(self) -> list[dict[str, Any]]:
        return self.raw.get("escalation_profiles", [])

    @property
    def rules(self) -> list[dict[str, Any]]:
        return self.raw.get("rules", [])

    @property
    def status_actions(self) -> list[dict[str, Any]]:
        return self.raw.get("status_actions", [])


class AlarmWorkflow:
    def __init__(self, config: Config):
        self.config = config
        self.lock = threading.RLock()
        self.stop_event = threading.Event()
        self.started_at = now_iso()
        self.instance = os.uname().nodename
        self.state_path = Path(str(config.storage["state_file"]))
        self.event_path = Path(str(config.storage["event_log"]))
        self.audit_path = Path(str(config.storage["audit_log"]))
        self.state_path.parent.mkdir(parents=True, exist_ok=True)
        self.events: deque[dict[str, Any]] = deque(maxlen=int(config.workflow.get("event_history_limit", 2000)))
        self.alarms: dict[str, dict[str, Any]] = {}
        self.seen_event_ids: deque[str] = deque(maxlen=int(config.workflow.get("seen_event_limit", 10000)))
        self.seen_event_set: set[str] = set()
        self.sds_cursor = 0
        self.sds_initial_sync_done = False
        self.mqtt_connected = False
        self.mqtt_last_error: str | None = None
        self.sds_router_healthy = False
        self.sds_last_error: str | None = None
        self.last_sds_poll: str | None = None
        self.last_mqtt_message: str | None = None
        self.metrics = {
            "events_ingested": 0,
            "events_duplicate": 0,
            "alarms_created": 0,
            "alarms_deduplicated": 0,
            "alarms_cleared": 0,
            "notifications_queued": 0,
            "notifications_failed": 0,
            "status_actions": 0,
            "text_actions": 0,
        }
        self._load()
        self.recipient_map = {str(item.get("id")): item for item in config.recipients if item.get("id")}
        self.profile_map = {str(item.get("id")): item for item in config.profiles if item.get("id")}
        self.rule_map = {str(item.get("id")): item for item in config.rules if item.get("id")}

    def _load(self) -> None:
        try:
            data = json.loads(self.state_path.read_text(encoding="utf-8"))
            if data.get("schema") == SCHEMA:
                self.alarms = data.get("alarms", {}) if isinstance(data.get("alarms"), dict) else {}
                seen = data.get("seen_event_ids", []) if isinstance(data.get("seen_event_ids"), list) else []
                for event_id in seen[-self.seen_event_ids.maxlen :]:
                    if isinstance(event_id, str):
                        self.seen_event_ids.append(event_id)
                        self.seen_event_set.add(event_id)
                self.sds_cursor = int(data.get("sds_cursor", 0))
                stored_metrics = data.get("metrics")
                if isinstance(stored_metrics, dict):
                    for key in self.metrics:
                        if isinstance(stored_metrics.get(key), int):
                            self.metrics[key] = stored_metrics[key]
        except (OSError, ValueError, TypeError):
            pass
        try:
            lines = self.event_path.read_text(encoding="utf-8").splitlines()[-self.events.maxlen :]
            for line in lines:
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
            data = {
                "schema": SCHEMA,
                "updated_at": now_iso(),
                "alarms": self.alarms,
                "seen_event_ids": list(self.seen_event_ids),
                "sds_cursor": self.sds_cursor,
                "metrics": self.metrics,
            }
        atomic_json(self.state_path, data)

    def audit(self, action: str, actor: str, alarm_id: str | None, detail: dict[str, Any]) -> None:
        record = {
            "audit_id": str(uuid.uuid4()),
            "timestamp": now_iso(),
            "service": "netcore-alarm-workflow",
            "action": action,
            "actor": actor,
            "alarm_id": alarm_id,
            "detail": detail,
        }
        with self.audit_path.open("a", encoding="utf-8") as handle:
            handle.write(json.dumps(record, ensure_ascii=False, separators=(",", ":")) + "\n")

    def publish_mqtt(self, topic: str, payload: dict[str, Any], retain: bool = False, qos: int = 1) -> bool:
        mqtt = self.config.mqtt
        if not mqtt.get("enabled", True):
            return False
        command = [
            "mosquitto_pub",
            "-h",
            str(mqtt.get("host", "127.0.0.1")),
            "-p",
            str(mqtt.get("port", 1883)),
            "-q",
            str(qos),
            "-t",
            topic,
            "-m",
            json.dumps(payload, ensure_ascii=False, separators=(",", ":")),
        ]
        if retain:
            command.append("-r")
        result = subprocess.run(command, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, check=False)
        with self.lock:
            self.mqtt_connected = result.returncode == 0
            self.mqtt_last_error = None if result.returncode == 0 else compact_text(result.stderr, 300)
        return result.returncode == 0

    def emit_event(
        self,
        event_type: str,
        severity: str,
        alarm: dict[str, Any] | None,
        payload: dict[str, Any],
        causation_id: str | None = None,
    ) -> dict[str, Any]:
        subject = {"type": "alarm", "id": alarm["alarm_id"]} if alarm else {"type": "service", "id": "alarm-workflow"}
        event = {
            "schema": EVENT_SCHEMA,
            "event_id": str(uuid.uuid4()),
            "event_type": event_type,
            "source": {"service": "netcore-alarm-workflow", "instance": self.instance},
            "timestamp": now_iso(),
            "severity": severity,
            "subject": subject,
            "payload": payload,
            "deduplication_key": f"netcore-alarm-workflow:{event_type}:{subject['id']}:{payload.get('state', '')}:{payload.get('escalation_level', '')}",
        }
        if alarm:
            event["correlation_id"] = alarm.get("correlation_id") or alarm["alarm_id"]
        if causation_id:
            event["causation_id"] = causation_id
        with self.lock:
            self.events.append(event)
            with self.event_path.open("a", encoding="utf-8") as handle:
                handle.write(json.dumps(event, ensure_ascii=False, separators=(",", ":")) + "\n")
        prefix = str(self.config.mqtt.get("topic_prefix", "netcore/v1")).rstrip("/")
        self.publish_mqtt(f"{prefix}/events/{event_type.replace('.', '/')}", event, retain=False, qos=1)
        if alarm:
            self.publish_mqtt(f"{prefix}/state/alarms/{alarm['alarm_id']}", alarm, retain=True, qos=1)
        return event

    def publish_service_state(self) -> None:
        prefix = str(self.config.mqtt.get("topic_prefix", "netcore/v1")).rstrip("/")
        self.publish_mqtt(f"{prefix}/state/services/alarm-workflow", self.status(), retain=True, qos=1)

    def mark_seen(self, event_id: str) -> bool:
        with self.lock:
            if event_id in self.seen_event_set:
                self.metrics["events_duplicate"] += 1
                return False
            if len(self.seen_event_ids) == self.seen_event_ids.maxlen:
                old = self.seen_event_ids.popleft()
                self.seen_event_set.discard(old)
            self.seen_event_ids.append(event_id)
            self.seen_event_set.add(event_id)
            self.metrics["events_ingested"] += 1
            return True

    def validate_event(self, event: Any) -> dict[str, Any]:
        if not isinstance(event, dict):
            raise ValueError("event must be an object")
        if event.get("schema") != EVENT_SCHEMA:
            raise ValueError("schema must be netcore-event-v1")
        event_id = event.get("event_id")
        event_type = event.get("event_type")
        if not isinstance(event_id, str) or not event_id:
            raise ValueError("event_id missing")
        if not isinstance(event_type, str) or "." not in event_type:
            raise ValueError("event_type invalid")
        if not isinstance(event.get("payload", {}), dict):
            raise ValueError("payload must be an object")
        return event

    def rule_matches(self, rule: dict[str, Any], event: dict[str, Any]) -> bool:
        if not rule.get("enabled", True):
            return False
        pattern = str(rule.get("event_type", ""))
        if not pattern or not event_matches(pattern, str(event.get("event_type", ""))):
            return False
        allowed_severities = rule.get("match_severities", [])
        if allowed_severities and str(event.get("severity", "info")) not in allowed_severities:
            return False
        equals = rule.get("payload_equals", {})
        if isinstance(equals, dict):
            for key, expected in equals.items():
                if nested_get(event, f"payload.{key}") != expected:
                    return False
        return True

    def alarm_dedup_key(self, rule: dict[str, Any], event: dict[str, Any]) -> str:
        fields = rule.get("dedup_fields") or ["subject.id", "payload.alarm_key", "payload.metric", "payload.input"]
        parts = [str(rule.get("alarm_type", rule.get("id", "alarm")))]
        for field in fields:
            value = nested_get(event, str(field))
            if value not in (None, ""):
                parts.append(str(value))
        if len(parts) == 1:
            parts.append(str(event.get("event_type", "unknown")))
        return ":".join(parts)[:320]

    def find_active_by_dedup(self, dedup_key: str) -> dict[str, Any] | None:
        with self.lock:
            candidates = [item for item in self.alarms.values() if item.get("dedup_key") == dedup_key and item.get("state") not in TERMINAL_STATES]
            candidates.sort(key=lambda item: item.get("created_at", ""), reverse=True)
            return candidates[0] if candidates else None

    def _profile_steps(self, profile_id: str) -> list[dict[str, Any]]:
        profile = self.profile_map.get(profile_id, {})
        steps = profile.get("steps", []) if isinstance(profile, dict) else []
        valid = [deepcopy(step) for step in steps if isinstance(step, dict)]
        valid.sort(key=lambda item: int(item.get("after_secs", 0)))
        return valid

    def create_or_update_alarm(self, rule: dict[str, Any], event: dict[str, Any]) -> dict[str, Any]:
        dedup_key = self.alarm_dedup_key(rule, event)
        existing = self.find_active_by_dedup(dedup_key)
        timestamp = now_iso()
        if existing:
            with self.lock:
                existing["occurrences"] = int(existing.get("occurrences", 1)) + 1
                existing["last_occurrence_at"] = timestamp
                existing["updated_at"] = timestamp
                existing["last_source_event"] = deepcopy(event)
                incoming_severity = str(rule.get("severity", "inherit"))
                if incoming_severity == "inherit":
                    incoming_severity = str(event.get("severity", "warning"))
                if SEVERITY_ORDER.get(incoming_severity, 0) > SEVERITY_ORDER.get(str(existing.get("severity", "info")), 0):
                    existing["severity"] = incoming_severity
                self.metrics["alarms_deduplicated"] += 1
                result = deepcopy(existing)
            self.emit_event(
                "alarm.occurrence_added",
                str(result.get("severity", "warning")),
                result,
                {"state": result["state"], "occurrences": result["occurrences"], "dedup_key": dedup_key},
                event.get("event_id"),
            )
            self.persist()
            return result

        alarm_id = str(uuid.uuid4())
        token = alarm_id.split("-")[0].upper()
        severity = str(rule.get("severity", "inherit"))
        if severity == "inherit":
            severity = str(event.get("severity", "warning"))
        profile_id = str(rule.get("escalation_profile", "technical-default"))
        steps = self._profile_steps(profile_id)
        created = utc_now()
        next_escalation = None
        if steps:
            next_escalation = (created + timedelta(seconds=int(steps[0].get("after_secs", 0)))).isoformat(timespec="milliseconds").replace("+00:00", "Z")
        alarm = {
            "alarm_id": alarm_id,
            "token": token,
            "alarm_type": str(rule.get("alarm_type", rule.get("id", "generic"))),
            "title": render_template(str(rule.get("title", "NetCore alarm {subject_id}")), event),
            "description": render_template(str(rule.get("description", "Triggered by {event_type}")), event),
            "severity": severity,
            "priority": int(rule.get("priority", 5)),
            "state": "open",
            "requires_ack": bool(rule.get("requires_ack", True)),
            "created_at": created.isoformat(timespec="milliseconds").replace("+00:00", "Z"),
            "updated_at": timestamp,
            "last_occurrence_at": timestamp,
            "occurrences": 1,
            "dedup_key": dedup_key,
            "source_event_id": event.get("event_id"),
            "correlation_id": event.get("correlation_id") or alarm_id,
            "source": deepcopy(event.get("source")),
            "subject": deepcopy(event.get("subject")),
            "context": deepcopy(event.get("payload", {})),
            "last_source_event": deepcopy(event),
            "rule_id": str(rule.get("id", "manual")),
            "recipients": list(rule.get("recipients", [])),
            "escalation_profile": profile_id,
            "escalation_index": 0,
            "escalation_level": -1,
            "next_escalation_at": next_escalation,
            "acknowledged_at": None,
            "acknowledged_by": None,
            "assigned_at": None,
            "assigned_to": None,
            "started_at": None,
            "resolved_at": None,
            "resolved_by": None,
            "resolution": None,
            "closed_at": None,
            "closed_by": None,
            "notifications": [],
            "history": [{"timestamp": timestamp, "action": "created", "actor": "event-rule", "detail": {"event_type": event.get("event_type")}}],
        }
        with self.lock:
            self.alarms[alarm_id] = alarm
            self.metrics["alarms_created"] += 1
        self.audit("alarm.created", "event-rule", alarm_id, {"rule_id": alarm["rule_id"], "dedup_key": dedup_key})
        self.emit_event("alarm.created", severity, alarm, {"state": "open", "title": alarm["title"], "token": token, "alarm_type": alarm["alarm_type"]}, event.get("event_id"))
        self.persist()
        return deepcopy(alarm)

    def clear_alarm_from_event(self, rule: dict[str, Any], event: dict[str, Any]) -> dict[str, Any] | None:
        dedup_key = self.alarm_dedup_key(rule, event)
        alarm = self.find_active_by_dedup(dedup_key)
        if not alarm:
            return None
        result = self.transition(alarm["alarm_id"], "resolved", "event-clear", "Source condition cleared", event.get("event_id"))
        with self.lock:
            self.metrics["alarms_cleared"] += 1
        if bool(rule.get("auto_close", self.config.workflow.get("auto_close_on_clear", False))):
            result = self.transition(alarm["alarm_id"], "closed", "event-clear", "Automatically closed after source cleared", event.get("event_id"))
        return result

    def ingest_event(self, raw_event: Any, ingress: str = "http") -> list[dict[str, Any]]:
        event = self.validate_event(raw_event)
        source = event.get("source") if isinstance(event.get("source"), dict) else {}
        if source.get("service") == "netcore-alarm-workflow" or str(event.get("event_type", "")).startswith("alarm."):
            return []
        if not self.mark_seen(str(event["event_id"])):
            return []
        actions: list[dict[str, Any]] = []
        for rule in self.config.rules:
            if not self.rule_matches(rule, event):
                continue
            action = str(rule.get("action", "raise"))
            if action == "raise":
                actions.append(self.create_or_update_alarm(rule, event))
            elif action == "clear":
                cleared = self.clear_alarm_from_event(rule, event)
                if cleared:
                    actions.append(cleared)
        self.audit("event.ingested", ingress, None, {"event_id": event["event_id"], "event_type": event["event_type"], "actions": len(actions)})
        self.persist()
        return actions

    def create_manual_alarm(self, payload: dict[str, Any], actor: str = "openlab-api") -> dict[str, Any]:
        event = {
            "schema": EVENT_SCHEMA,
            "event_id": str(uuid.uuid4()),
            "event_type": "alarm.manual_request",
            "source": {"service": actor, "instance": actor},
            "timestamp": now_iso(),
            "severity": str(payload.get("severity", "warning")),
            "subject": {"type": str(payload.get("subject_type", "service")), "id": safe_identifier(payload.get("subject_id", "manual"), 256)},
            "payload": payload.get("context", {}) if isinstance(payload.get("context", {}), dict) else {},
        }
        rule = {
            "id": "manual-api",
            "alarm_type": str(payload.get("alarm_type", "manual")),
            "title": str(payload.get("title", "Manual NetCore alarm")),
            "description": str(payload.get("description", "Created through OPEN-LAB API")),
            "severity": str(payload.get("severity", "warning")),
            "priority": int(payload.get("priority", 5)),
            "requires_ack": bool(payload.get("requires_ack", True)),
            "recipients": payload.get("recipients", ["technik-gruppe"]),
            "escalation_profile": str(payload.get("escalation_profile", "technical-default")),
            "dedup_fields": ["subject.id"],
        }
        alarm = self.create_or_update_alarm(rule, event)
        self.audit("alarm.manual_created", actor, alarm["alarm_id"], {})
        return alarm

    def get_alarm(self, reference: str) -> dict[str, Any] | None:
        reference_upper = reference.upper()
        with self.lock:
            if reference in self.alarms:
                return self.alarms[reference]
            matches = [item for item in self.alarms.values() if str(item.get("token", "")).upper() == reference_upper or str(item.get("alarm_id", "")).upper().startswith(reference_upper)]
            matches.sort(key=lambda item: item.get("created_at", ""), reverse=True)
            return matches[0] if matches else None

    def transition(
        self,
        reference: str,
        target_state: str,
        actor: str,
        note: str = "",
        causation_id: str | None = None,
    ) -> dict[str, Any]:
        target_state = target_state.lower()
        timestamp = now_iso()
        with self.lock:
            alarm = self.get_alarm(reference)
            if not alarm:
                raise KeyError("alarm not found")
            current = str(alarm.get("state", "open"))
            if target_state == current:
                return deepcopy(alarm)
            if target_state not in ALLOWED_TRANSITIONS.get(current, set()):
                raise ValueError(f"transition {current} -> {target_state} is not allowed")
            alarm["state"] = target_state
            alarm["updated_at"] = timestamp
            alarm["history"].append({"timestamp": timestamp, "action": target_state, "actor": actor, "detail": {"note": note}})
            if target_state == "acknowledged":
                alarm["acknowledged_at"] = timestamp
                alarm["acknowledged_by"] = actor
                alarm["next_escalation_at"] = None
            elif target_state == "assigned":
                alarm["assigned_at"] = timestamp
                alarm["assigned_to"] = actor
                if bool(self.config.workflow.get("stop_escalation_on_assignment", True)):
                    alarm["next_escalation_at"] = None
            elif target_state == "in_progress":
                alarm["started_at"] = timestamp
                if not alarm.get("assigned_to"):
                    alarm["assigned_to"] = actor
                    alarm["assigned_at"] = timestamp
                if bool(self.config.workflow.get("stop_escalation_on_assignment", True)):
                    alarm["next_escalation_at"] = None
            elif target_state == "resolved":
                alarm["resolved_at"] = timestamp
                alarm["resolved_by"] = actor
                alarm["resolution"] = note or "resolved"
                alarm["next_escalation_at"] = None
            elif target_state == "closed":
                alarm["closed_at"] = timestamp
                alarm["closed_by"] = actor
                alarm["next_escalation_at"] = None
            elif target_state == "cancelled":
                alarm["closed_at"] = timestamp
                alarm["closed_by"] = actor
                alarm["next_escalation_at"] = None
            elif target_state == "open":
                alarm["acknowledged_at"] = None
                alarm["acknowledged_by"] = None
                alarm["resolved_at"] = None
                alarm["resolved_by"] = None
                alarm["closed_at"] = None
                alarm["closed_by"] = None
                alarm["escalation_index"] = 0
                alarm["escalation_level"] = -1
                steps = self._profile_steps(str(alarm.get("escalation_profile", "technical-default")))
                alarm["next_escalation_at"] = timestamp if steps else None
            result = deepcopy(alarm)
        self.audit(f"alarm.{target_state}", actor, result["alarm_id"], {"from": current, "note": note})
        event_type = {
            "acknowledged": "alarm.acknowledged",
            "assigned": "alarm.assigned",
            "in_progress": "alarm.in_progress",
            "resolved": "alarm.resolved",
            "closed": "alarm.closed",
            "cancelled": "alarm.cancelled",
            "open": "alarm.reopened",
        }[target_state]
        severity = str(result.get("severity", "warning")) if target_state in ACTIVE_STATES else "info"
        self.emit_event(event_type, severity, result, {"state": target_state, "actor": actor, "note": note}, causation_id)
        self.persist()
        return result

    def apply_action(self, reference: str, action: str, actor: str, note: str = "", causation_id: str | None = None) -> dict[str, Any]:
        action = action.lower()
        state_map = {
            "ack": "acknowledged",
            "acknowledge": "acknowledged",
            "take": "assigned",
            "assign": "assigned",
            "start": "in_progress",
            "resolve": "resolved",
            "close": "closed",
            "cancel": "cancelled",
            "reopen": "open",
        }
        if action == "escalate":
            return self.manual_escalate(reference, actor)
        if action == "notify":
            alarm = self.get_alarm(reference)
            if not alarm:
                raise KeyError("alarm not found")
            self.notify_alarm(alarm, list(alarm.get("recipients", [])), int(alarm.get("escalation_level", 0)), actor)
            return deepcopy(alarm)
        if action not in state_map:
            raise ValueError("unsupported action")
        return self.transition(reference, state_map[action], actor, note, causation_id)

    def select_latest_alarm(self, action_cfg: dict[str, Any], source_issi: int) -> dict[str, Any] | None:
        target_states = action_cfg.get("states", ["open", "acknowledged", "assigned", "in_progress"])
        with self.lock:
            candidates = [item for item in self.alarms.values() if item.get("state") in target_states]
            only_requiring_ack = bool(action_cfg.get("requires_ack_only", False))
            if only_requiring_ack:
                candidates = [item for item in candidates if item.get("requires_ack")]
            candidates.sort(key=lambda item: item.get("created_at", ""), reverse=True)
            return candidates[0] if candidates else None

    def handle_status(self, source_issi: int, dest_issi: int, status_code: int, event_id: str | None = None) -> dict[str, Any] | None:
        for item in self.config.status_actions:
            if not item.get("enabled", True) or int(item.get("status_code", -1)) != status_code:
                continue
            alarm = self.select_latest_alarm(item, source_issi)
            if not alarm:
                return None
            actor = f"issi:{source_issi}"
            result = self.apply_action(alarm["alarm_id"], str(item.get("action", "ack")), actor, f"Status {status_code} to ISSI {dest_issi}", event_id)
            with self.lock:
                self.metrics["status_actions"] += 1
            self.emit_event("alarm.status_action_applied", "info", result, {"status_code": status_code, "source_issi": source_issi, "action": item.get("action")}, event_id)
            self.persist()
            return result
        return None

    def handle_text_command(self, source_issi: int, text: str, event_id: str | None = None) -> dict[str, Any] | None:
        match = re.search(r"\b(ACK|TAKE|START|RESOLVE|CLOSE|CANCEL|REOPEN)\s+([A-Fa-f0-9-]{4,36})\b", text)
        if not match:
            return None
        action = match.group(1).lower()
        reference = match.group(2)
        actor = f"issi:{source_issi}"
        result = self.apply_action(reference, action, actor, compact_text(text, 160), event_id)
        with self.lock:
            self.metrics["text_actions"] += 1
        self.emit_event("alarm.text_action_applied", "info", result, {"source_issi": source_issi, "action": action, "reference": reference}, event_id)
        self.persist()
        return result

    def process_sds_event(self, event: dict[str, Any]) -> None:
        if event.get("event_type") != "sds.received":
            return
        payload = event.get("payload") if isinstance(event.get("payload"), dict) else {}
        source_issi = int(payload.get("source_issi", 0) or 0)
        dest_issi = int(payload.get("dest_issi", 0) or 0)
        status_code = payload.get("status_code")
        text = str(payload.get("text", payload.get("text_preview", "")) or "")
        if isinstance(status_code, int):
            try:
                self.handle_status(source_issi, dest_issi, status_code, str(event.get("event_id")))
            except (KeyError, ValueError):
                pass
        if text:
            try:
                self.handle_text_command(source_issi, text, str(event.get("event_id")))
            except (KeyError, ValueError):
                pass

    def http_json(self, url: str, method: str = "GET", payload: dict[str, Any] | None = None) -> Any:
        body = None
        headers = {"Accept": "application/json"}
        if payload is not None:
            body = json.dumps(payload, ensure_ascii=False).encode("utf-8")
            headers["Content-Type"] = "application/json"
        request = Request(url, method=method, data=body, headers=headers)
        timeout = float(self.config.sds.get("timeout_secs", 3))
        with urlopen(request, timeout=timeout) as response:
            data = response.read()
            return json.loads(data) if data else {}

    def sds_router_url(self, path: str) -> str:
        return str(self.config.sds.get("base_url", "http://127.0.0.1:8150")).rstrip("/") + path

    def send_sds(self, alarm: dict[str, Any], recipient: dict[str, Any], escalation_level: int) -> dict[str, Any]:
        destination = int(recipient.get("destination", 0))
        if destination <= 0:
            raise ValueError("recipient destination missing")
        severity_letter = {
            "debug": "D",
            "info": "I",
            "notice": "N",
            "warning": "W",
            "error": "E",
            "critical": "C",
            "emergency": "X",
        }.get(str(alarm.get("severity", "warning")), "W")
        template = str(recipient.get("message_template", "ALARM {severity} {token} {title}. ACK {token}"))
        rendered = template.format_map({
            "severity": severity_letter,
            "token": alarm["token"],
            "title": compact_text(alarm.get("title", "Alarm"), 100),
            "alarm_id": alarm["alarm_id"],
            "state": alarm["state"],
            "occurrences": alarm.get("occurrences", 1),
        })
        message = compact_text(rendered, int(recipient.get("max_text_chars", 180)))
        payload = {
            "source_issi": int(recipient.get("source_issi", self.config.sds.get("source_issi", 9999))),
            "dest_issi": destination,
            "is_group": str(recipient.get("kind", "group_sds")) == "group_sds",
            "sds_type": 4,
            "protocol_id": int(recipient.get("protocol_id", self.config.sds.get("protocol_id", 130))),
            "text": message,
            "priority": int(recipient.get("priority", alarm.get("priority", 5))),
            "ttl_secs": int(recipient.get("ttl_secs", self.config.sds.get("default_ttl_secs", 300))),
            "ingress": f"alarm-workflow:{alarm['alarm_id']}:level-{escalation_level}",
        }
        response = self.http_json(self.sds_router_url("/api/v1/messages"), "POST", payload)
        return {
            "notification_id": str(uuid.uuid4()),
            "recipient_id": recipient["id"],
            "destination": destination,
            "is_group": payload["is_group"],
            "message": message,
            "queued_at": now_iso(),
            "escalation_level": escalation_level,
            "status": "queued",
            "sds_message_id": response.get("id") if isinstance(response, dict) else None,
            "sds_state": response.get("state") if isinstance(response, dict) else None,
            "last_error": None,
        }

    def notify_alarm(self, alarm: dict[str, Any], recipient_ids: list[str], escalation_level: int, actor: str = "scheduler") -> list[dict[str, Any]]:
        results: list[dict[str, Any]] = []
        for recipient_id in recipient_ids:
            recipient = self.recipient_map.get(str(recipient_id))
            if not recipient or not recipient.get("enabled", True):
                results.append({"recipient_id": recipient_id, "status": "skipped", "last_error": "recipient disabled or unknown"})
                continue
            try:
                result = self.send_sds(alarm, recipient, escalation_level)
                with self.lock:
                    current = self.alarms.get(alarm["alarm_id"])
                    if current:
                        current["notifications"].append(result)
                        current["updated_at"] = now_iso()
                    self.metrics["notifications_queued"] += 1
                self.emit_event("alarm.notification_queued", "info", alarm, {"recipient_id": recipient_id, "destination": result["destination"], "sds_message_id": result.get("sds_message_id"), "escalation_level": escalation_level})
                results.append(result)
            except (ValueError, HTTPError, URLError, OSError, TimeoutError) as error:
                result = {
                    "notification_id": str(uuid.uuid4()),
                    "recipient_id": recipient_id,
                    "queued_at": now_iso(),
                    "escalation_level": escalation_level,
                    "status": "failed",
                    "last_error": compact_text(error, 300),
                }
                with self.lock:
                    current = self.alarms.get(alarm["alarm_id"])
                    if current:
                        current["notifications"].append(result)
                        current["updated_at"] = now_iso()
                    self.metrics["notifications_failed"] += 1
                self.emit_event("alarm.notification_failed", "warning", alarm, {"recipient_id": recipient_id, "error": result["last_error"], "escalation_level": escalation_level})
                results.append(result)
        self.audit("alarm.notified", actor, alarm["alarm_id"], {"recipients": recipient_ids, "escalation_level": escalation_level, "results": len(results)})
        self.persist()
        return results

    def execute_due_escalations(self) -> None:
        due: list[tuple[str, int, list[str], dict[str, Any]]] = []
        current_time = utc_now()
        with self.lock:
            for alarm in self.alarms.values():
                if alarm.get("state") not in ACTIVE_STATES:
                    continue
                if alarm.get("state") in {"acknowledged", "assigned", "in_progress"} and bool(self.config.workflow.get("stop_escalation_on_ack", True)):
                    continue
                due_at = parse_time(alarm.get("next_escalation_at"))
                if not due_at or due_at > current_time:
                    continue
                steps = self._profile_steps(str(alarm.get("escalation_profile", "technical-default")))
                index = int(alarm.get("escalation_index", 0))
                if index >= len(steps):
                    alarm["next_escalation_at"] = None
                    continue
                step = steps[index]
                recipients = list(step.get("recipients", [])) or list(alarm.get("recipients", []))
                alarm["escalation_level"] = index
                alarm["escalation_index"] = index + 1
                if index + 1 < len(steps):
                    origin = parse_time(alarm.get("created_at")) or current_time
                    alarm["next_escalation_at"] = (origin + timedelta(seconds=int(steps[index + 1].get("after_secs", 0)))).isoformat(timespec="milliseconds").replace("+00:00", "Z")
                else:
                    alarm["next_escalation_at"] = None
                alarm["updated_at"] = now_iso()
                due.append((alarm["alarm_id"], index, recipients, deepcopy(alarm)))
        for alarm_id, level, recipients, snapshot in due:
            self.notify_alarm(snapshot, recipients, level)
            event_name = "alarm.notification_started" if level == 0 else "alarm.escalated"
            self.emit_event(event_name, str(snapshot.get("severity", "warning")), snapshot, {"state": snapshot["state"], "escalation_level": level, "recipients": recipients})
            self.audit(event_name, "scheduler", alarm_id, {"level": level, "recipients": recipients})
        if due:
            self.persist()

    def manual_escalate(self, reference: str, actor: str) -> dict[str, Any]:
        with self.lock:
            alarm = self.get_alarm(reference)
            if not alarm:
                raise KeyError("alarm not found")
            alarm["next_escalation_at"] = now_iso()
            result = deepcopy(alarm)
        self.execute_due_escalations()
        self.audit("alarm.manual_escalate", actor, result["alarm_id"], {})
        return deepcopy(self.get_alarm(result["alarm_id"]))

    def refresh_delivery_states(self) -> None:
        pending: list[tuple[str, str, str]] = []
        with self.lock:
            for alarm in self.alarms.values():
                for notification in alarm.get("notifications", []):
                    message_id = notification.get("sds_message_id")
                    if message_id and notification.get("status") in {"queued", "in_flight", "offline", "retry_waiting"}:
                        pending.append((alarm["alarm_id"], notification["notification_id"], message_id))
        for alarm_id, notification_id, message_id in pending[:50]:
            try:
                response = self.http_json(self.sds_router_url(f"/api/v1/messages/{message_id}"))
                state = str(response.get("state", "unknown")) if isinstance(response, dict) else "unknown"
                with self.lock:
                    alarm = self.alarms.get(alarm_id)
                    if not alarm:
                        continue
                    notification = next((item for item in alarm.get("notifications", []) if item.get("notification_id") == notification_id), None)
                    if not notification:
                        continue
                    notification["sds_state"] = state
                    notification["last_checked_at"] = now_iso()
                    if state in {"delivered", "partial"}:
                        notification["status"] = "delivered"
                    elif state in {"failed", "dead_letter", "expired", "cancelled"}:
                        notification["status"] = "failed"
                    elif state in {"in_flight", "offline", "retry_waiting", "queued"}:
                        notification["status"] = state
            except (HTTPError, URLError, OSError, ValueError):
                continue

    def sds_poll_loop(self) -> None:
        interval = float(self.config.sds.get("poll_interval_secs", 2))
        process_existing = bool(self.config.sds.get("process_existing_events", False))
        while not self.stop_event.is_set():
            try:
                events = self.http_json(self.sds_router_url("/api/v1/events/netcore?limit=500"))
                if not isinstance(events, list):
                    raise ValueError("SDS event endpoint did not return a list")
                sequences = [int(item.get("sequence", 0) or 0) for item in events if isinstance(item, dict)]
                maximum = max(sequences, default=self.sds_cursor)
                if not self.sds_initial_sync_done and self.sds_cursor == 0 and not process_existing:
                    with self.lock:
                        self.sds_cursor = maximum
                    self.sds_initial_sync_done = True
                else:
                    ordered = sorted((item for item in events if isinstance(item, dict)), key=lambda item: int(item.get("sequence", 0) or 0))
                    for event in ordered:
                        sequence = int(event.get("sequence", 0) or 0)
                        if sequence <= self.sds_cursor:
                            continue
                        self.process_sds_event(event)
                        with self.lock:
                            self.sds_cursor = max(self.sds_cursor, sequence)
                    self.sds_initial_sync_done = True
                with self.lock:
                    self.sds_router_healthy = True
                    self.sds_last_error = None
                    self.last_sds_poll = now_iso()
                self.persist()
            except (HTTPError, URLError, OSError, ValueError, TypeError) as error:
                with self.lock:
                    self.sds_router_healthy = False
                    self.sds_last_error = compact_text(error, 300)
            self.stop_event.wait(interval)

    def mqtt_loop(self) -> None:
        mqtt = self.config.mqtt
        if not mqtt.get("enabled", True):
            return
        topics = mqtt.get("subscribe_topics", [f"{mqtt.get('topic_prefix', 'netcore/v1')}/events/#"])
        while not self.stop_event.is_set():
            command = [
                "mosquitto_sub",
                "-h",
                str(mqtt.get("host", "127.0.0.1")),
                "-p",
                str(mqtt.get("port", 1883)),
                "-q",
                str(mqtt.get("qos", 1)),
                "-v",
            ]
            for topic in topics:
                command.extend(["-t", str(topic)])
            process: subprocess.Popen[str] | None = None
            try:
                process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, bufsize=1)
                with self.lock:
                    self.mqtt_connected = True
                    self.mqtt_last_error = None
                assert process.stdout is not None
                while not self.stop_event.is_set() and process.poll() is None:
                    line = process.stdout.readline()
                    if not line:
                        break
                    try:
                        topic, raw = line.rstrip("\n").split(" ", 1)
                        event = json.loads(raw)
                        self.last_mqtt_message = now_iso()
                        self.ingest_event(event, f"mqtt:{topic}")
                    except (ValueError, TypeError):
                        continue
            except OSError as error:
                with self.lock:
                    self.mqtt_last_error = compact_text(error, 300)
            finally:
                with self.lock:
                    self.mqtt_connected = False
                if process is not None and process.poll() is None:
                    process.terminate()
                    try:
                        process.wait(timeout=2)
                    except subprocess.TimeoutExpired:
                        process.kill()
            self.stop_event.wait(float(mqtt.get("reconnect_secs", 3)))

    def scheduler_loop(self) -> None:
        interval = float(self.config.workflow.get("scheduler_interval_secs", 2))
        delivery_counter = 0
        while not self.stop_event.is_set():
            self.execute_due_escalations()
            delivery_counter += 1
            if delivery_counter >= max(1, int(10 / max(interval, 0.5))):
                self.refresh_delivery_states()
                self.publish_service_state()
                delivery_counter = 0
            self.stop_event.wait(interval)

    def start(self) -> None:
        for target, name in [
            (self.mqtt_loop, "alarm-mqtt"),
            (self.sds_poll_loop, "alarm-sds-poll"),
            (self.scheduler_loop, "alarm-scheduler"),
        ]:
            threading.Thread(target=target, name=name, daemon=True).start()

    def stop(self) -> None:
        self.stop_event.set()
        self.persist()

    def list_alarms(self, state: str | None = None, limit: int = 500) -> list[dict[str, Any]]:
        with self.lock:
            items = [deepcopy(item) for item in self.alarms.values() if not state or item.get("state") == state]
        items.sort(key=lambda item: item.get("created_at", ""), reverse=True)
        return items[: max(1, min(limit, 5000))]

    def status(self) -> dict[str, Any]:
        with self.lock:
            values = list(self.alarms.values())
            counts = {state: sum(1 for item in values if item.get("state") == state) for state in ["open", "acknowledged", "assigned", "in_progress", "resolved", "closed", "cancelled"]}
            return {
                "service": "netcore-alarm-workflow",
                "phase": 8,
                "security_mode": "open_lab",
                "warning": "OPEN LAB: no login, token or TLS; every reachable client can create, acknowledge, resolve and close alarms",
                "started_at": self.started_at,
                "mqtt_connected": self.mqtt_connected,
                "mqtt_last_error": self.mqtt_last_error,
                "mqtt_last_message": self.last_mqtt_message,
                "sds_router_healthy": self.sds_router_healthy,
                "sds_last_error": self.sds_last_error,
                "last_sds_poll": self.last_sds_poll,
                "sds_cursor": self.sds_cursor,
                "alarms_total": len(values),
                "alarms_active": sum(1 for item in values if item.get("state") in ACTIVE_STATES),
                "counts": counts,
                "rules": len(self.config.rules),
                "recipients": len(self.config.recipients),
                "escalation_profiles": len(self.config.profiles),
                "status_actions": len(self.config.status_actions),
                "metrics": deepcopy(self.metrics),
            }

    def prometheus(self) -> str:
        status = self.status()
        lines = [
            "# HELP netcore_alarm_workflow_up Service readiness dependencies",
            "# TYPE netcore_alarm_workflow_up gauge",
            f"netcore_alarm_workflow_up {1 if status['mqtt_connected'] and status['sds_router_healthy'] else 0}",
            "# HELP netcore_alarm_active Active alarms by state",
            "# TYPE netcore_alarm_active gauge",
        ]
        for state, count in status["counts"].items():
            lines.append(f'netcore_alarm_active{{state="{state}"}} {count}')
        for key, value in status["metrics"].items():
            lines.append(f"netcore_alarm_{key} {value}")
        return "\n".join(lines) + "\n"


HTML = r'''<!doctype html>
<html lang="de"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>NetCore Alarm Workflow</title>
<style>
:root{color-scheme:dark}body{font-family:system-ui;background:#0d131b;color:#eaf2ff;margin:0;padding:1.5rem}h1,h2{margin:.2rem 0 1rem}.warn{background:#5a3914;border:1px solid #c8872f;padding:.8rem;border-radius:.6rem}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(145px,1fr));gap:.7rem;margin:1rem 0}.card{background:#172230;padding:.8rem;border:1px solid #2e4156;border-radius:.7rem}.v{font-size:1.7rem;font-weight:700}table{border-collapse:collapse;width:100%;background:#121c28}th,td{padding:.55rem;border-bottom:1px solid #2a3b4d;text-align:left;vertical-align:top}button{background:#28415d;color:#fff;border:1px solid #55789a;border-radius:.35rem;padding:.35rem .55rem;margin:.1rem;cursor:pointer}.critical,.emergency{color:#ff6d6d}.warning{color:#ffd166}.info{color:#7bdff2}.muted{color:#aab7c6;font-size:.9rem}code{background:#26374a;padding:.1rem .25rem;border-radius:.2rem}.ok{color:#79df91}.bad{color:#ff7777}</style></head>
<body><h1>NetCore Alarm Workflow · OPEN LAB</h1><div class="warn">Keine Anmeldung, Tokens oder TLS. Jeder erreichbare Client kann Alarme verändern. Nur im isolierten Testnetz verwenden.</div>
<div id="health"></div><div class="grid" id="cards"></div>
<h2>Aktive Alarme</h2><table><thead><tr><th>Alarm</th><th>Schwere</th><th>Status</th><th>Quelle</th><th>Eskalation</th><th>Aktionen</th></tr></thead><tbody id="alarms"></tbody></table>
<h2>Ereignisse</h2><pre id="events"></pre>
<script>
const esc=s=>String(s??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
async function api(path,opt){let r=await fetch(path,opt);let t=await r.text();if(!r.ok)throw Error(t);return t?JSON.parse(t):{}}
async function act(id,a){let note=prompt('Notiz (optional)','')||'';try{await api(`/api/v1/alarms/${id}/${a}`,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({actor:'webui-openlab',note})});load()}catch(e){alert(e.message)}}
async function load(){try{let [s,a,e]=await Promise.all([api('/api/v1/status'),api('/api/v1/alarms?active=true&limit=200'),api('/api/v1/events?limit=30')]);document.querySelector('#health').innerHTML=`<p>MQTT <span class="${s.mqtt_connected?'ok':'bad'}">${s.mqtt_connected?'verbunden':'getrennt'}</span> · SDS Router <span class="${s.sds_router_healthy?'ok':'bad'}">${s.sds_router_healthy?'erreichbar':'nicht erreichbar'}</span></p>`;document.querySelector('#cards').innerHTML=[['Aktiv',s.alarms_active],['Offen',s.counts.open],['Quittiert',s.counts.acknowledged],['Übernommen',s.counts.assigned+s.counts.in_progress],['Gelöst',s.counts.resolved],['Regeln',s.rules]].map(x=>`<div class="card"><div class="muted">${x[0]}</div><div class="v">${x[1]}</div></div>`).join('');document.querySelector('#alarms').innerHTML=a.map(x=>`<tr><td><b>${esc(x.token)} · ${esc(x.title)}</b><div class="muted">${esc(x.alarm_type)} · ${esc(x.created_at)} · ${x.occurrences}×</div></td><td class="${esc(x.severity)}">${esc(x.severity)}</td><td>${esc(x.state)}<div class="muted">${esc(x.assigned_to||x.acknowledged_by||'')}</div></td><td><code>${esc((x.subject||{}).id||'')}</code><div class="muted">${esc((x.source||{}).service||'')}</div></td><td>Level ${x.escalation_level}<div class="muted">${esc(x.next_escalation_at||'gestoppt')}</div></td><td><button onclick="act('${x.alarm_id}','ack')">ACK</button><button onclick="act('${x.alarm_id}','take')">Übernehmen</button><button onclick="act('${x.alarm_id}','start')">Start</button><button onclick="act('${x.alarm_id}','resolve')">Lösen</button><button onclick="act('${x.alarm_id}','close')">Schließen</button><button onclick="act('${x.alarm_id}','escalate')">Eskalieren</button></td></tr>`).join('');document.querySelector('#events').textContent=e.map(x=>`${x.timestamp} ${x.event_type} ${(x.subject||{}).id||''}`).join('\n')}catch(e){document.querySelector('#health').innerHTML='<p class="bad">'+esc(e.message)+'</p>'}}
load();setInterval(load,3000);
</script></body></html>'''


class Handler(BaseHTTPRequestHandler):
    app: AlarmWorkflow

    def log_message(self, format: str, *args: Any) -> None:
        return

    def send_bytes(self, data: bytes, content_type: str, status: int = 200) -> None:
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(data)

    def send_json(self, value: Any, status: int = 200) -> None:
        self.send_bytes(json.dumps(value, ensure_ascii=False).encode("utf-8"), "application/json; charset=utf-8", status)

    def read_json(self) -> dict[str, Any]:
        length = int(self.headers.get("Content-Length", "0"))
        maximum = int(self.app.config.raw.get("limits", {}).get("max_body_bytes", 2_097_152))
        if length <= 0 or length > maximum:
            raise ValueError("invalid Content-Length")
        value = json.loads(self.rfile.read(length))
        if not isinstance(value, dict):
            raise ValueError("JSON body must be an object")
        return value

    def do_GET(self) -> None:
        parsed = urlparse(self.path)
        path = parsed.path
        query = parse_qs(parsed.query)
        if path == "/":
            return self.send_bytes(HTML.encode("utf-8"), "text/html; charset=utf-8")
        if path == "/health/live":
            return self.send_json({"status": "ok", "service": "netcore-alarm-workflow", "mode": "open_lab"})
        if path == "/health/ready":
            status = self.app.status()
            ready = status["mqtt_connected"] and status["sds_router_healthy"]
            return self.send_json({"status": "ready" if ready else "degraded", "service": "netcore-alarm-workflow", "dependencies": {"mqtt": status["mqtt_connected"], "sds_router": status["sds_router_healthy"]}}, 200 if ready else 503)
        if path == "/metrics":
            return self.send_bytes(self.app.prometheus().encode("utf-8"), "text/plain; version=0.0.4; charset=utf-8")
        if path == "/api/v1/status":
            return self.send_json(self.app.status())
        if path == "/api/v1/alarms":
            state = query.get("state", [None])[0]
            active = query.get("active", ["false"])[0].lower() == "true"
            limit = int(query.get("limit", ["500"])[0])
            values = self.app.list_alarms(state, limit)
            if active:
                values = [item for item in values if item.get("state") in ACTIVE_STATES]
            return self.send_json(values)
        if path.startswith("/api/v1/alarms/"):
            reference = path.rsplit("/", 1)[-1]
            alarm = self.app.get_alarm(reference)
            return self.send_json(deepcopy(alarm), 200) if alarm else self.send_json({"error": "alarm not found"}, 404)
        if path == "/api/v1/events":
            limit = max(1, min(int(query.get("limit", ["200"])[0]), 2000))
            with self.app.lock:
                values = list(self.app.events)[-limit:][::-1]
            return self.send_json(values)
        if path == "/api/v1/rules":
            return self.send_json(self.app.config.rules)
        if path == "/api/v1/recipients":
            return self.send_json(self.app.config.recipients)
        if path == "/api/v1/escalation-profiles":
            return self.send_json(self.app.config.profiles)
        if path == "/api/v1/status-actions":
            return self.send_json(self.app.config.status_actions)
        if path == "/openapi.json":
            return self.send_json({
                "openapi": "3.0.3",
                "info": {"title": "NetCore Alarm Workflow OPEN LAB", "version": "1.0.0"},
                "paths": {
                    "/api/v1/status": {"get": {}},
                    "/api/v1/alarms": {"get": {}, "post": {}},
                    "/api/v1/alarms/{id}": {"get": {}},
                    "/api/v1/alarms/{id}/{action}": {"post": {}},
                    "/api/v1/ingest-event": {"post": {}},
                    "/api/v1/ingest-status": {"post": {}},
                    "/api/v1/events": {"get": {}},
                    "/health/live": {"get": {}},
                    "/health/ready": {"get": {}},
                    "/metrics": {"get": {}},
                },
            })
        return self.send_json({"error": "not found"}, 404)

    def do_POST(self) -> None:
        path = urlparse(self.path).path
        try:
            payload = self.read_json()
            if path == "/api/v1/alarms":
                return self.send_json(self.app.create_manual_alarm(payload), 201)
            if path == "/api/v1/ingest-event":
                return self.send_json({"actions": self.app.ingest_event(payload, "openlab-api")}, 202)
            if path == "/api/v1/ingest-status":
                result = self.app.handle_status(int(payload.get("source_issi", 0)), int(payload.get("dest_issi", 0)), int(payload["status_code"]), str(payload.get("event_id") or uuid.uuid4()))
                return self.send_json(result or {"status": "no_matching_action"}, 200 if result else 202)
            match = re.fullmatch(r"/api/v1/alarms/([^/]+)/(ack|take|assign|start|resolve|close|cancel|reopen|escalate|notify)", path)
            if match:
                actor = safe_identifier(payload.get("actor", "openlab-api"), 160)
                note = compact_text(payload.get("note", ""), 500)
                result = self.app.apply_action(match.group(1), match.group(2), actor, note)
                return self.send_json(result)
            return self.send_json({"error": "not found"}, 404)
        except KeyError as error:
            return self.send_json({"error": str(error)}, 404)
        except (ValueError, TypeError, json.JSONDecodeError) as error:
            return self.send_json({"error": compact_text(error, 400)}, 400)
        except (HTTPError, URLError, OSError) as error:
            return self.send_json({"error": compact_text(error, 400)}, 502)


def load_config(path: Path) -> Config:
    with path.open("rb") as handle:
        raw = tomllib.load(handle)
    return Config(raw)


def main() -> int:
    parser = argparse.ArgumentParser(description="NetCore SDS/Status/Alarm workflow service")
    parser.add_argument("--config", default="/etc/netcore/alarm-workflow.toml")
    args = parser.parse_args()
    config = load_config(Path(args.config))
    app = AlarmWorkflow(config)
    Handler.app = app
    server = ThreadingHTTPServer(config.bind, Handler)
    server.daemon_threads = True

    def shutdown(_signum: int, _frame: Any) -> None:
        app.stop()
        threading.Thread(target=server.shutdown, daemon=True).start()

    signal.signal(signal.SIGTERM, shutdown)
    signal.signal(signal.SIGINT, shutdown)
    app.start()
    try:
        server.serve_forever(poll_interval=0.5)
    finally:
        app.stop()
        server.server_close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
