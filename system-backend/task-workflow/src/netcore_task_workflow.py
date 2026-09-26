#!/usr/bin/env python3
from __future__ import annotations

import argparse
import html
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
from urllib.parse import parse_qs, quote, urlencode, urlparse
from urllib.request import Request, urlopen

STATE_SCHEMA = "netcore-task-workflow-state-v1"
EVENT_SCHEMA = "netcore-event-v1"
TASK_SCHEMA = "netcore-task-v1"
ACTIVE_STATES = {"open", "assigned", "accepted", "in_progress", "blocked"}
TERMINAL_STATES = {"completed", "cancelled", "expired"}
ACTION_TO_STATE = {
    "assign": "assigned",
    "accept": "accepted",
    "start": "in_progress",
    "block": "blocked",
    "complete": "completed",
    "cancel": "cancelled",
    "reopen": "open",
}
ALLOWED_TRANSITIONS = {
    "open": {"assigned", "accepted", "in_progress", "cancelled", "expired"},
    "assigned": {"accepted", "in_progress", "blocked", "completed", "cancelled", "expired"},
    "accepted": {"assigned", "in_progress", "blocked", "completed", "cancelled", "expired"},
    "in_progress": {"assigned", "blocked", "completed", "cancelled", "expired"},
    "blocked": {"assigned", "accepted", "in_progress", "completed", "cancelled", "expired"},
    "completed": {"open"},
    "cancelled": {"open"},
    "expired": {"open"},
}
SEVERITY_ORDER = {"debug": 0, "info": 1, "notice": 2, "warning": 3, "error": 4, "critical": 5, "emergency": 6}


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


def compact_text(value: Any, maximum: int = 300) -> str:
    return re.sub(r"\s+", " ", str(value or "")).strip()[:maximum]


def safe_id(value: Any, maximum: int = 160) -> str:
    return "".join(c for c in str(value or "") if c.isalnum() or c in "-_.:")[:maximum]


def to_int(value: Any, default: int = 0, minimum: int | None = None, maximum: int | None = None) -> int:
    try:
        result = int(value)
    except (TypeError, ValueError):
        result = default
    if minimum is not None:
        result = max(minimum, result)
    if maximum is not None:
        result = min(maximum, result)
    return result


def to_bool(value: Any, default: bool = False) -> bool:
    if isinstance(value, bool):
        return value
    if value is None:
        return default
    return str(value).strip().lower() in {"1", "true", "yes", "on", "ja"}


def atomic_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = path.with_suffix(path.suffix + ".tmp")
    temp.write_text(json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    temp.replace(path)


def task_token() -> str:
    return "T" + uuid.uuid4().hex[:7].upper()


def actor_from_issi(issi: int | None) -> str:
    return f"issi:{issi}" if issi else "openlab"


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
    def wap(self) -> dict[str, Any]:
        return self.raw.get("wap", {})

    @property
    def templates(self) -> list[dict[str, Any]]:
        return self.raw.get("templates", [])

    @property
    def status_actions(self) -> list[dict[str, Any]]:
        return self.raw.get("status_actions", [])


class TaskWorkflow:
    def __init__(self, config: Config):
        self.config = config
        self.lock = threading.RLock()
        self.stop_event = threading.Event()
        self.instance = os.uname().nodename
        self.started_at = now_iso()
        self.state_path = Path(str(config.storage["state_file"]))
        self.event_path = Path(str(config.storage["event_log"]))
        self.audit_path = Path(str(config.storage["audit_log"]))
        self.state_path.parent.mkdir(parents=True, exist_ok=True)
        self.tasks: dict[str, dict[str, Any]] = {}
        self.events: deque[dict[str, Any]] = deque(maxlen=int(config.workflow.get("event_history_limit", 2000)))
        self.seen_event_ids: deque[str] = deque(maxlen=int(config.workflow.get("seen_event_limit", 10000)))
        self.seen_event_set: set[str] = set()
        self.mqtt_connected = False
        self.mqtt_last_error: str | None = None
        self.sds_router_healthy = False
        self.sds_last_error: str | None = None
        self.metrics = {
            "tasks_created": 0,
            "tasks_completed": 0,
            "tasks_cancelled": 0,
            "tasks_expired": 0,
            "transitions": 0,
            "notifications_queued": 0,
            "notifications_failed": 0,
            "sds_actions": 0,
            "status_actions": 0,
            "wap_requests": 0,
        }
        self.template_map = {str(item.get("id")): item for item in config.templates if item.get("id")}
        self.status_map = {to_int(item.get("status_code")): item for item in config.status_actions if item.get("status_code") is not None}
        self._load()

    def _load(self) -> None:
        try:
            data = json.loads(self.state_path.read_text(encoding="utf-8"))
            if data.get("schema") == STATE_SCHEMA:
                self.tasks = data.get("tasks", {}) if isinstance(data.get("tasks"), dict) else {}
                for event_id in data.get("seen_event_ids", [])[-self.seen_event_ids.maxlen :]:
                    if isinstance(event_id, str):
                        self.seen_event_ids.append(event_id)
                        self.seen_event_set.add(event_id)
                stored = data.get("metrics", {})
                if isinstance(stored, dict):
                    for key in self.metrics:
                        if isinstance(stored.get(key), int):
                            self.metrics[key] = stored[key]
        except (OSError, ValueError, TypeError):
            pass
        try:
            for line in self.event_path.read_text(encoding="utf-8").splitlines()[-self.events.maxlen :]:
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
                "tasks": self.tasks,
                "seen_event_ids": list(self.seen_event_ids),
                "metrics": self.metrics,
            })

    def audit(self, action: str, actor: str, task_id: str | None, detail: dict[str, Any]) -> None:
        record = {
            "audit_id": str(uuid.uuid4()),
            "timestamp": now_iso(),
            "service": "netcore-task-workflow",
            "action": action,
            "actor": actor,
            "task_id": task_id,
            "detail": detail,
        }
        with self.audit_path.open("a", encoding="utf-8") as handle:
            handle.write(json.dumps(record, ensure_ascii=False, separators=(",", ":")) + "\n")

    def publish_mqtt(self, topic: str, payload: dict[str, Any], retain: bool = False, qos: int = 1) -> bool:
        mqtt = self.config.mqtt
        if not mqtt.get("enabled", True):
            return True
        command = [
            "mosquitto_pub", "-h", str(mqtt.get("host", "127.0.0.1")), "-p", str(mqtt.get("port", 1883)),
            "-q", str(qos), "-t", topic, "-m", json.dumps(payload, ensure_ascii=False, separators=(",", ":")),
        ]
        if retain:
            command.append("-r")
        try:
            result = subprocess.run(command, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, timeout=5, check=False)
            if result.returncode == 0:
                self.mqtt_connected = True
                self.mqtt_last_error = None
                return True
            self.mqtt_connected = False
            self.mqtt_last_error = compact_text(result.stderr, 300)
        except (OSError, subprocess.TimeoutExpired) as error:
            self.mqtt_connected = False
            self.mqtt_last_error = compact_text(error, 300)
        return False

    def emit_event(self, event_type: str, severity: str, task: dict[str, Any], payload: dict[str, Any] | None = None, causation_id: str | None = None) -> dict[str, Any]:
        event = {
            "schema": EVENT_SCHEMA,
            "event_id": str(uuid.uuid4()),
            "event_type": event_type,
            "source": {"service": "netcore-task-workflow", "instance": self.instance},
            "timestamp": now_iso(),
            "severity": severity if severity in SEVERITY_ORDER else "info",
            "subject": {"type": "task", "id": task["task_id"]},
            "payload": {
                "task_id": task["task_id"],
                "token": task["token"],
                "task_type": task["task_type"],
                "title": task["title"],
                "state": task["state"],
                "priority": task["priority"],
                **(payload or {}),
            },
        }
        if causation_id:
            event["causation_id"] = causation_id
        with self.lock:
            self.events.append(event)
            with self.event_path.open("a", encoding="utf-8") as handle:
                handle.write(json.dumps(event, ensure_ascii=False, separators=(",", ":")) + "\n")
        prefix = str(self.config.mqtt.get("topic_prefix", "netcore/v1"))
        self.publish_mqtt(f"{prefix}/events/{event_type.replace('.', '/')}", event, False, 1)
        self.publish_mqtt(f"{prefix}/state/tasks/{task['task_id']}", task, True, 1)
        return event

    def _template(self, template_id: str) -> dict[str, Any]:
        template = self.template_map.get(template_id)
        if not template:
            raise ValueError(f"unknown template: {template_id}")
        return template

    def _timeline(self, actor: str, action: str, note: str = "", detail: dict[str, Any] | None = None) -> dict[str, Any]:
        return {"timestamp": now_iso(), "actor": actor, "action": action, "note": compact_text(note, 500), "detail": detail or {}}

    def create_task(self, payload: dict[str, Any], actor: str = "openlab-api") -> dict[str, Any]:
        template_id = safe_id(payload.get("template_id") or payload.get("task_type") or "technical_fault")
        template = self._template(template_id)
        title = compact_text(payload.get("title") or template.get("name") or template_id, 120)
        if not title:
            raise ValueError("title missing")
        form_data = payload.get("form_data", {}) if isinstance(payload.get("form_data"), dict) else {}
        for field in template.get("fields", []):
            field_id = str(field.get("id", ""))
            if field.get("required") and compact_text(form_data.get(field_id), 500) == "":
                raise ValueError(f"required field missing: {field_id}")
        task_id = str(uuid.uuid4())
        assigned_issi = to_int(payload.get("assigned_issi"), 0, 0, 16777215) or None
        assigned_gssi = to_int(payload.get("assigned_gssi"), 0, 0, 16777215) or None
        due_at = payload.get("due_at") if parse_time(payload.get("due_at")) else None
        expires_at = payload.get("expires_at") if parse_time(payload.get("expires_at")) else None
        state = "assigned" if assigned_issi or assigned_gssi else "open"
        created = now_iso()
        task = {
            "schema": TASK_SCHEMA,
            "task_id": task_id,
            "token": task_token(),
            "template_id": template_id,
            "task_type": safe_id(payload.get("task_type") or template_id),
            "title": title,
            "description": compact_text(payload.get("description") or template.get("description"), 1000),
            "priority": to_int(payload.get("priority"), int(template.get("default_priority", 5)), 0, 15),
            "severity": str(payload.get("severity") or template.get("default_severity", "notice")),
            "state": state,
            "requires_ack": to_bool(payload.get("requires_ack"), bool(template.get("requires_ack", True))),
            "created_at": created,
            "updated_at": created,
            "created_by": actor,
            "assigned_issi": assigned_issi,
            "assigned_gssi": assigned_gssi,
            "accepted_by_issi": None,
            "owner": None,
            "due_at": due_at,
            "expires_at": expires_at,
            "location": compact_text(payload.get("location"), 160),
            "form_data": {safe_id(k, 80): compact_text(v, 500) for k, v in form_data.items()},
            "comments": [],
            "timeline": [self._timeline(actor, "created", payload.get("note", ""))],
            "notifications": [],
            "source_alarm_id": payload.get("source_alarm_id"),
        }
        with self.lock:
            self.tasks[task_id] = task
            self.metrics["tasks_created"] += 1
            self.persist()
        self.audit("task.created", actor, task_id, {"template_id": template_id, "state": state})
        self.emit_event("task.created", task["severity"], task)
        if to_bool(payload.get("notify"), True):
            self.notify_task(task_id, actor, "created")
        return deepcopy(task)

    def get_task(self, task_ref: str) -> dict[str, Any] | None:
        with self.lock:
            if task_ref in self.tasks:
                return deepcopy(self.tasks[task_ref])
            token = task_ref.strip().upper()
            for task in self.tasks.values():
                if str(task.get("token", "")).upper() == token:
                    return deepcopy(task)
        return None

    def list_tasks(self, state: str | None = None, issi: int | None = None, active: bool | None = None, limit: int = 200) -> list[dict[str, Any]]:
        with self.lock:
            values = [deepcopy(item) for item in self.tasks.values()]
        if state:
            values = [item for item in values if item.get("state") == state]
        if active is True:
            values = [item for item in values if item.get("state") in ACTIVE_STATES]
        elif active is False:
            values = [item for item in values if item.get("state") not in ACTIVE_STATES]
        if issi:
            values = [item for item in values if item.get("assigned_issi") in (None, issi) or item.get("accepted_by_issi") == issi]
        values.sort(key=lambda item: (item.get("updated_at", ""), item.get("created_at", "")), reverse=True)
        return values[: max(1, min(limit, 1000))]

    def transition(self, task_ref: str, action: str, actor: str, note: str = "", payload: dict[str, Any] | None = None) -> dict[str, Any]:
        payload = payload or {}
        action = action.strip().lower()
        if action == "comment":
            with self.lock:
                task = self._resolve_locked(task_ref)
                comment = {"timestamp": now_iso(), "actor": actor, "text": compact_text(note or payload.get("comment"), 500)}
                if not comment["text"]:
                    raise ValueError("comment missing")
                task["comments"].append(comment)
                task["timeline"].append(self._timeline(actor, "comment", comment["text"]))
                task["updated_at"] = now_iso()
                self.persist()
                result = deepcopy(task)
            self.audit("task.comment", actor, result["task_id"], {"comment": comment["text"]})
            self.emit_event("task.comment_added", "info", result, {"actor": actor})
            return result
        if action not in ACTION_TO_STATE:
            raise ValueError(f"unsupported action: {action}")
        target_state = ACTION_TO_STATE[action]
        with self.lock:
            task = self._resolve_locked(task_ref)
            previous = str(task.get("state"))
            if target_state not in ALLOWED_TRANSITIONS.get(previous, set()):
                raise ValueError(f"transition {previous} -> {target_state} not allowed")
            actor_issi = self._actor_issi(actor)
            if action == "assign":
                task["assigned_issi"] = to_int(payload.get("assigned_issi"), 0, 0, 16777215) or task.get("assigned_issi")
                task["assigned_gssi"] = to_int(payload.get("assigned_gssi"), 0, 0, 16777215) or task.get("assigned_gssi")
                task["owner"] = actor
            elif action == "accept":
                if actor_issi:
                    task["accepted_by_issi"] = actor_issi
                task["owner"] = actor
            elif action in {"start", "complete", "block"} and not task.get("owner"):
                task["owner"] = actor
            task["state"] = target_state
            task["updated_at"] = now_iso()
            task["timeline"].append(self._timeline(actor, action, note, payload))
            if action == "complete":
                task["completed_at"] = task["updated_at"]
                self.metrics["tasks_completed"] += 1
            elif action == "cancel":
                task["cancelled_at"] = task["updated_at"]
                self.metrics["tasks_cancelled"] += 1
            self.metrics["transitions"] += 1
            self.persist()
            result = deepcopy(task)
        self.audit(f"task.{action}", actor, result["task_id"], {"previous_state": previous, "state": target_state, "note": compact_text(note, 500)})
        severity = "info" if target_state in TERMINAL_STATES else result.get("severity", "notice")
        event_type = "task.reopened" if action == "reopen" else f"task.{target_state}"
        self.emit_event(event_type, severity, result, {"previous_state": previous, "actor": actor})
        if bool(self.config.workflow.get("notify_on_state_change", True)):
            self.notify_task(result["task_id"], actor, action)
        return result

    def _resolve_locked(self, task_ref: str) -> dict[str, Any]:
        if task_ref in self.tasks:
            return self.tasks[task_ref]
        token = task_ref.strip().upper()
        for task in self.tasks.values():
            if str(task.get("token", "")).upper() == token:
                return task
        raise KeyError("task not found")

    @staticmethod
    def _actor_issi(actor: str) -> int | None:
        match = re.fullmatch(r"issi:(\d+)", actor)
        return int(match.group(1)) if match else None

    def sds_router_url(self, path: str) -> str:
        return str(self.config.sds.get("base_url", "http://127.0.0.1:8150")).rstrip("/") + path

    def http_json(self, url: str, method: str = "GET", payload: dict[str, Any] | None = None, timeout: float = 4.0) -> Any:
        data = None if payload is None else json.dumps(payload).encode("utf-8")
        request = Request(url, data=data, method=method, headers={"Content-Type": "application/json"})
        with urlopen(request, timeout=timeout) as response:
            raw = response.read()
            return json.loads(raw.decode("utf-8")) if raw else {}

    def notification_text(self, task: dict[str, Any], reason: str) -> str:
        state = str(task.get("state", "open")).upper()
        title = compact_text(task.get("title"), 80)
        return compact_text(f"TASK {task['token']} P{task['priority']} {state} {title}. TAKE {task['token']}", int(self.config.sds.get("max_text_length", 160)))

    def notify_task(self, task_ref: str, actor: str, reason: str = "manual") -> dict[str, Any]:
        if not self.config.sds.get("enabled", True):
            return {"status": "disabled"}
        with self.lock:
            task = self._resolve_locked(task_ref)
            task_copy = deepcopy(task)
        destinations: list[tuple[int, bool]] = []
        if task_copy.get("assigned_issi"):
            destinations.append((int(task_copy["assigned_issi"]), False))
        if task_copy.get("assigned_gssi"):
            destinations.append((int(task_copy["assigned_gssi"]), True))
        if not destinations:
            fallback = to_int(self.config.sds.get("default_destination"), 0)
            if fallback:
                destinations.append((fallback, bool(self.config.sds.get("default_is_group", True))))
        results = []
        for destination, is_group in destinations:
            payload = {
                "source_issi": to_int(self.config.sds.get("source_issi"), 9999, 1, 16777215),
                "dest_issi": destination,
                "is_group": is_group,
                "sds_type": 4,
                "protocol_id": to_int(self.config.sds.get("protocol_id"), 130, 0, 255),
                "status_code": None,
                "payload_hex": "",
                "text": self.notification_text(task_copy, reason),
                "priority": to_int(task_copy.get("priority"), 5, 0, 15),
                "ttl_secs": to_int(self.config.sds.get("ttl_secs"), 600, 5, 86400),
                "ingress": "task-workflow",
                "force_nodes": [],
            }
            try:
                response = self.http_json(self.sds_router_url("/api/v1/messages"), "POST", payload)
                message_id = response.get("id") or response.get("message_id")
                result = {"timestamp": now_iso(), "reason": reason, "destination": destination, "is_group": is_group, "status": "queued", "message_id": message_id}
                self.sds_router_healthy = True
                self.sds_last_error = None
                self.metrics["notifications_queued"] += 1
            except (OSError, HTTPError, URLError, ValueError) as error:
                result = {"timestamp": now_iso(), "reason": reason, "destination": destination, "is_group": is_group, "status": "failed", "error": compact_text(error, 300)}
                self.sds_router_healthy = False
                self.sds_last_error = result["error"]
                self.metrics["notifications_failed"] += 1
            results.append(result)
        with self.lock:
            task = self._resolve_locked(task_ref)
            task["notifications"].extend(results)
            task["updated_at"] = now_iso()
            self.persist()
            task_copy = deepcopy(task)
        self.audit("task.notification", actor, task_copy["task_id"], {"reason": reason, "results": results})
        self.emit_event("task.notification_queued" if all(item["status"] == "queued" for item in results) else "task.notification_failed", "info" if all(item["status"] == "queued" for item in results) else "warning", task_copy, {"reason": reason, "results": results})
        return {"task_id": task_copy["task_id"], "results": results}

    def handle_text_action(self, source_issi: int, text: str, event_id: str | None = None) -> dict[str, Any] | None:
        match = re.match(r"^\s*(TAKE|ACCEPT|START|BLOCK|DONE|COMPLETE|CANCEL|REOPEN|INFO)\s+(T?[0-9A-F]{7,8})\b(?:\s+(.*))?$", text, re.IGNORECASE)
        if not match:
            return None
        command, reference, note = match.groups()
        action_map = {"TAKE": "accept", "ACCEPT": "accept", "START": "start", "BLOCK": "block", "DONE": "complete", "COMPLETE": "complete", "CANCEL": "cancel", "REOPEN": "reopen"}
        command = command.upper()
        task = self.get_task(reference.upper())
        if not task:
            return None
        if command == "INFO":
            self.notify_task(task["task_id"], actor_from_issi(source_issi), "info-request")
            return task
        result = self.transition(task["task_id"], action_map[command], actor_from_issi(source_issi), note or "", {"source": "sds", "event_id": event_id})
        self.metrics["sds_actions"] += 1
        return result

    def select_latest_for_issi(self, source_issi: int) -> dict[str, Any] | None:
        tasks = self.list_tasks(issi=source_issi, active=True, limit=200)
        return tasks[0] if tasks else None

    def handle_status_action(self, source_issi: int, status_code: int, event_id: str | None = None) -> dict[str, Any] | None:
        cfg = self.status_map.get(status_code)
        if not cfg:
            return None
        task = self.select_latest_for_issi(source_issi)
        if not task:
            return None
        result = self.transition(task["task_id"], str(cfg.get("action")), actor_from_issi(source_issi), f"Status {status_code}", {"source": "status", "event_id": event_id})
        self.metrics["status_actions"] += 1
        return result

    def ingest_sds_event(self, event: dict[str, Any]) -> dict[str, Any] | None:
        event_id = str(event.get("event_id") or uuid.uuid4())
        with self.lock:
            if event_id in self.seen_event_set:
                return None
            self.seen_event_ids.append(event_id)
            self.seen_event_set.add(event_id)
            while len(self.seen_event_set) > self.seen_event_ids.maxlen:
                self.seen_event_set = set(self.seen_event_ids)
        payload = event.get("payload") if isinstance(event.get("payload"), dict) else {}
        source_issi = to_int(payload.get("source_issi"), 0)
        if source_issi <= 0:
            return None
        status_code = payload.get("status_code")
        if status_code is not None:
            return self.handle_status_action(source_issi, to_int(status_code), event_id)
        text = compact_text(payload.get("text"), 500)
        if text:
            return self.handle_text_action(source_issi, text, event_id)
        return None

    def expire_loop(self) -> None:
        interval = max(2, to_int(self.config.workflow.get("expire_check_interval_secs"), 5))
        while not self.stop_event.wait(interval):
            now = utc_now()
            expired = []
            with self.lock:
                for task in self.tasks.values():
                    if task.get("state") not in ACTIVE_STATES:
                        continue
                    deadline = parse_time(task.get("expires_at"))
                    if deadline and deadline <= now:
                        expired.append(task["task_id"])
            for task_id in expired:
                try:
                    with self.lock:
                        task = self._resolve_locked(task_id)
                        previous = task["state"]
                        task["state"] = "expired"
                        task["updated_at"] = now_iso()
                        task["timeline"].append(self._timeline("system", "expired", "Ablaufzeit erreicht"))
                        self.metrics["tasks_expired"] += 1
                        self.persist()
                        result = deepcopy(task)
                    self.audit("task.expired", "system", task_id, {"previous_state": previous})
                    self.emit_event("task.expired", "warning", result, {"previous_state": previous})
                except (KeyError, ValueError):
                    continue

    def dependency_loop(self) -> None:
        while not self.stop_event.wait(5):
            if self.config.sds.get("enabled", True):
                try:
                    self.http_json(self.sds_router_url("/health/live"), timeout=2)
                    self.sds_router_healthy = True
                    self.sds_last_error = None
                except Exception as error:
                    self.sds_router_healthy = False
                    self.sds_last_error = compact_text(error, 300)
            else:
                self.sds_router_healthy = True

    def mqtt_loop(self) -> None:
        mqtt = self.config.mqtt
        if not mqtt.get("enabled", True):
            self.mqtt_connected = True
            return
        topic = f"{mqtt.get('topic_prefix', 'netcore/v1')}/events/sds/#"
        while not self.stop_event.is_set():
            command = ["mosquitto_sub", "-h", str(mqtt.get("host", "127.0.0.1")), "-p", str(mqtt.get("port", 1883)), "-q", "1", "-v", "-t", topic]
            try:
                process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                self.mqtt_connected = True
                self.mqtt_last_error = None
                while not self.stop_event.is_set() and process.poll() is None:
                    line = process.stdout.readline() if process.stdout else ""
                    if not line:
                        break
                    try:
                        _topic, raw = line.rstrip("\n").split(" ", 1)
                        event = json.loads(raw)
                        if isinstance(event, dict):
                            self.ingest_sds_event(event)
                    except (ValueError, TypeError):
                        continue
                process.terminate()
                try:
                    process.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    process.kill()
            except OSError as error:
                self.mqtt_connected = False
                self.mqtt_last_error = compact_text(error, 300)
            self.stop_event.wait(2)

    def status(self) -> dict[str, Any]:
        tasks = self.list_tasks(limit=10000)
        by_state: dict[str, int] = {}
        for task in tasks:
            by_state[task["state"]] = by_state.get(task["state"], 0) + 1
        return {
            "service": "netcore-task-workflow",
            "phase": 9,
            "mode": "open_lab",
            "started_at": self.started_at,
            "tasks_total": len(tasks),
            "tasks_active": sum(1 for item in tasks if item["state"] in ACTIVE_STATES),
            "by_state": by_state,
            "mqtt_connected": self.mqtt_connected,
            "mqtt_last_error": self.mqtt_last_error,
            "sds_router_healthy": self.sds_router_healthy,
            "sds_last_error": self.sds_last_error,
            "templates": len(self.template_map),
            "metrics": deepcopy(self.metrics),
            "wap": {"xhtml": "/x", "wml": "/w"},
        }

    def prometheus(self) -> str:
        status = self.status()
        lines = [
            "# HELP netcore_task_workflow_up Service process state",
            "# TYPE netcore_task_workflow_up gauge",
            "netcore_task_workflow_up 1",
            "# HELP netcore_task_workflow_tasks Tasks by state",
            "# TYPE netcore_task_workflow_tasks gauge",
        ]
        for state, value in sorted(status["by_state"].items()):
            lines.append(f'netcore_task_workflow_tasks{{state="{state}"}} {value}')
        lines.extend([
            f"netcore_task_workflow_mqtt_connected {1 if self.mqtt_connected else 0}",
            f"netcore_task_workflow_sds_router_healthy {1 if self.sds_router_healthy else 0}",
        ])
        for key, value in sorted(self.metrics.items()):
            lines.append(f"netcore_task_workflow_{key} {value}")
        return "\n".join(lines) + "\n"


CSS = "body{font-family:system-ui;background:#10151d;color:#eef;margin:1.5rem}a{color:#8cc8ff}table{border-collapse:collapse;width:100%}td,th{padding:.55rem;border-bottom:1px solid #344;text-align:left}input,textarea,select,button{padding:.45rem;background:#1d2835;color:#eef;border:1px solid #456}code{background:#263445;padding:.1rem .25rem}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(220px,1fr));gap:.75rem}.card{border:1px solid #344;padding:1rem;margin:.75rem 0}.ok{color:#75dc8d}.bad{color:#ff8585}"
HTML = f'''<!doctype html><html><head><meta charset="utf-8"><title>NetCore Task Workflow</title><style>{CSS}</style></head><body><h1>NetCore Task Workflow · OPEN LAB</h1><p>Strukturierte Aufträge, WAP-Formulare, SDS-Quittierung und Status-Workflow.</p><div id="status"></div><h2>Auftrag anlegen</h2><form id="newTask" class="grid"><label>Vorlage<select name="template_id" id="template"></select></label><label>Titel<input name="title"></label><label>Priorität<input name="priority" type="number" min="0" max="15" value="5"></label><label>Ziel-ISSI<input name="assigned_issi" type="number"></label><label>Ziel-GSSI<input name="assigned_gssi" type="number"></label><label class="wide">Beschreibung<textarea name="description"></textarea></label><div id="templateFields" class="wide"></div><button>Auftrag anlegen</button></form><h2>Aufträge</h2><table><thead><tr><th>Token</th><th>Auftrag</th><th>Status</th><th>Ziel</th><th>Aktualisiert</th><th>Aktionen</th></tr></thead><tbody id="tasks"></tbody></table><h2>WAP</h2><p><a href="/x">XHTML</a> · <a href="/w">WML</a></p><script>
let templatesData=[];
async function api(u,o){{let r=await fetch(u,o);let t=await r.text();let j=t?JSON.parse(t):{{}};if(!r.ok)throw Error(j.error||r.statusText);return j}}
function renderTemplateFields(){{let t=templatesData.find(x=>x.id===template.value);templateFields.innerHTML=(t?.fields||[]).map(f=>`<label>${{f.label||f.id}}<input name="f_${{f.id}}" ${{f.required?'required':''}}></label>`).join('')}}
async function refresh(){{let s=await api('/api/v1/status'),ts=await api('/api/v1/tasks?limit=300'),tpl=await api('/api/v1/templates');status.innerHTML=`${{s.tasks_active}} aktiv / ${{s.tasks_total}} gesamt · MQTT ${{s.mqtt_connected?'online':'offline'}} · SDS ${{s.sds_router_healthy?'online':'offline'}}`;templatesData=tpl;if(!template.options.length){{template.innerHTML=tpl.map(x=>`<option value="${{x.id}}">${{x.name}}</option>`).join('');renderTemplateFields()}}tasks.innerHTML=ts.map(t=>`<tr><td><code>${{t.token}}</code></td><td>${{t.title}}<br><small>${{t.task_type}}</small></td><td>${{t.state}}</td><td>${{t.assigned_issi||t.assigned_gssi||'-'}}</td><td>${{t.updated_at}}</td><td>${{['accept','start','block','complete','cancel'].map(a=>`<button onclick="act('${{t.task_id}}','${{a}}')">${{a}}</button>`).join(' ')}}</td></tr>`).join('')}}
async function act(id,a){{try{{await api(`/api/v1/tasks/${{id}}/${{a}}`,{{method:'POST',headers:{{'Content-Type':'application/json'}},body:'{{}}'}});refresh()}}catch(e){{alert(e.message)}}}}
newTask.onsubmit=async e=>{{e.preventDefault();let f=new FormData(newTask),p=Object.fromEntries(f.entries());p.priority=Number(p.priority);p.assigned_issi=p.assigned_issi?Number(p.assigned_issi):null;p.assigned_gssi=p.assigned_gssi?Number(p.assigned_gssi):null;p.form_data={{}};for(let [k,v] of f.entries()){{if(k.startsWith('f_')){{p.form_data[k.slice(2)]=v;delete p[k]}}}}try{{await api('/api/v1/tasks',{{method:'POST',headers:{{'Content-Type':'application/json'}},body:JSON.stringify(p)}});newTask.reset();refresh()}}catch(e){{alert(e.message)}}}};template.onchange=renderTemplateFields;refresh();setInterval(refresh,5000)</script></body></html>'''


def x(value: Any) -> str:
    return html.escape(str(value or ""), quote=True)


def wml_doc(title: str, body: str) -> bytes:
    return (f'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN" "http://www.wapforum.org/DTD/wml_1.1.xml">\n<wml><card id="main" title="{x(title)}"><p>{body}</p></card></wml>').encode("utf-8")


def xhtml_doc(title: str, body: str) -> bytes:
    return (f'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML Basic 1.1//EN" "http://www.w3.org/TR/xhtml-basic/xhtml-basic11.dtd">\n<html xmlns="http://www.w3.org/1999/xhtml"><head><title>{x(title)}</title></head><body>{body}</body></html>').encode("utf-8")


class Handler(BaseHTTPRequestHandler):
    app: TaskWorkflow

    def log_message(self, fmt: str, *args: Any) -> None:
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

    def read_body(self) -> tuple[dict[str, Any], str]:
        length = to_int(self.headers.get("Content-Length"), 0, 0, 1_000_000)
        raw = self.rfile.read(length) if length else b""
        content_type = self.headers.get("Content-Type", "").split(";", 1)[0].strip().lower()
        if content_type == "application/json":
            return (json.loads(raw.decode("utf-8")) if raw else {}), content_type
        values = parse_qs(raw.decode("utf-8", "replace"), keep_blank_values=True)
        return {key: items[-1] if items else "" for key, items in values.items()}, content_type

    def wap_params(self) -> tuple[str, dict[str, list[str]], int | None]:
        parsed = urlparse(self.path)
        query = parse_qs(parsed.query, keep_blank_values=True)
        issi = to_int(query.get("issi", [0])[-1], 0) or None
        return parsed.path, query, issi

    def wap_link(self, fmt: str, path: str, issi: int | None = None, **params: Any) -> str:
        query = {key: value for key, value in params.items() if value not in (None, "")}
        if issi:
            query["issi"] = issi
        suffix = "?" + urlencode(query) if query else ""
        return f"/{fmt}{path}{suffix}"

    def wap_index(self, fmt: str, issi: int | None) -> bytes:
        tasks = self.app.list_tasks(issi=issi, active=True, limit=int(self.app.config.wap.get("page_size", 6)))
        links = []
        for task in tasks:
            links.append(f'<a href="{x(self.wap_link(fmt, "/task", issi, id=task["task_id"]))}">{x(task["token"])} {x(task["title"][:30])}</a><br/>')
        new_link = self.wap_link(fmt, "/new", issi)
        body = f'<b>NetCore Auftraege</b><br/>Aktiv:{len(tasks)}<br/>{"".join(links) or "Keine aktiven Auftraege<br/>"}<a href="{x(new_link)}">Neu</a>'
        return wml_doc("Auftraege", body) if fmt == "w" else xhtml_doc("Auftraege", body)

    def wap_task(self, fmt: str, task: dict[str, Any], issi: int | None) -> bytes:
        actions = []
        for action, label in [("accept", "Nehmen"), ("start", "Start"), ("block", "Block"), ("complete", "Fertig"), ("cancel", "Abbruch")]:
            if ACTION_TO_STATE[action] in ALLOWED_TRANSITIONS.get(task["state"], set()):
                actions.append(f'<a href="{x(self.wap_link(fmt, "/action", issi, id=task["task_id"], action=action))}">{label}</a>')
        body = f'<b>{x(task["token"])} {x(task["title"][:60])}</b><br/>Status:{x(task["state"])} P{task["priority"]}<br/>{x(task.get("description", "")[:160])}<br/>{" ".join(actions)}<br/><a href="{x(self.wap_link(fmt, "", issi))}">Liste</a>'
        return wml_doc(task["token"], body) if fmt == "w" else xhtml_doc(task["token"], body)

    def wap_new(self, fmt: str, issi: int | None, template_id: str | None = None) -> bytes:
        if not template_id:
            body = '<b>Vorlage waehlen</b><br/>' + '<br/>'.join(f'<a href="{x(self.wap_link(fmt, "/new", issi, template=tpl_id))}">{x(tpl.get("name", tpl_id))}</a>' for tpl_id, tpl in self.app.template_map.items())
            return wml_doc("Neuer Auftrag", body) if fmt == "w" else xhtml_doc("Neuer Auftrag", body)
        template = self.app._template(template_id)
        action = self.wap_link(fmt, "/submit", issi)
        fields = []
        if fmt == "w":
            # WML 1.1 has no HTML <form>. Values are collected in variables and
            # posted through <go>/<postfield>, which old Openwave terminals understand.
            fields.append('Titel:<input name="title" size="20"/><br/>')
            fields.append('Ziel ISSI:<input name="assigned_issi" format="*N" size="8"/><br/>')
            fields.append('Ziel GSSI:<input name="assigned_gssi" format="*N" size="8"/><br/>')
            postfields = [f'<postfield name="template_id" value="{x(template_id)}"/>',
                          '<postfield name="title" value="$(title)"/>',
                          '<postfield name="assigned_issi" value="$(assigned_issi)"/>',
                          '<postfield name="assigned_gssi" value="$(assigned_gssi)"/>']
            for field in template.get("fields", []):
                field_id = safe_id(field.get("id"), 80)
                fields.append(f'{x(field.get("label", field_id))}:<input name="f_{x(field_id)}" size="20"/><br/>')
                postfields.append(f'<postfield name="f_{x(field_id)}" value="$(f_{x(field_id)})"/>')
            body = f'{"".join(fields)}<anchor>Senden<go href="{x(action)}" method="post">{"".join(postfields)}</go></anchor>'
            return wml_doc("Neuer Auftrag", body)
        fields.append(f'<input type="hidden" name="template_id" value="{x(template_id)}"/>')
        fields.append('<label>Titel<input name="title"/></label><br/>')
        fields.append('<label>Ziel ISSI<input name="assigned_issi" type="number"/></label><br/>')
        fields.append('<label>Ziel GSSI<input name="assigned_gssi" type="number"/></label><br/>')
        for field in template.get("fields", []):
            required = ' required="required"' if field.get("required") else ""
            fields.append(f'<label>{x(field.get("label", field.get("id")))}<input name="f_{x(field.get("id"))}"{required}/></label><br/>')
        body = f'<form action="{x(action)}" method="post">{"".join(fields)}<input type="submit" value="Senden"/></form>'
        return xhtml_doc("Neuer Auftrag", body)

    def handle_wap(self, method: str) -> bool:
        path, query, issi = self.wap_params()
        fmt = "w" if path == "/w" or path.startswith("/w/") else "x" if path == "/x" or path.startswith("/x/") else ""
        if not fmt:
            return False
        self.app.metrics["wap_requests"] += 1
        suffix = path[2:] or ""
        try:
            if method == "GET" and suffix in ("", "/"):
                data = self.wap_index(fmt, issi)
            elif method == "GET" and suffix == "/task":
                task = self.app.get_task(query.get("id", [""])[-1])
                if not task:
                    raise KeyError("task not found")
                data = self.wap_task(fmt, task, issi)
            elif method == "GET" and suffix == "/new":
                data = self.wap_new(fmt, issi, query.get("template", [None])[-1])
            elif method == "GET" and suffix == "/action":
                task_id = query.get("id", [""])[-1]
                action = query.get("action", [""])[-1]
                task = self.app.transition(task_id, action, actor_from_issi(issi), "WAP", {"source": "wap"})
                data = self.wap_task(fmt, task, issi)
            elif method == "POST" and suffix == "/submit":
                form, _ = self.read_body()
                form_data = {key[2:]: value for key, value in form.items() if key.startswith("f_")}
                task = self.app.create_task({
                    "template_id": form.get("template_id"),
                    "title": form.get("title"),
                    "assigned_issi": form.get("assigned_issi"),
                    "assigned_gssi": form.get("assigned_gssi"),
                    "form_data": form_data,
                    "notify": True,
                }, actor_from_issi(issi))
                data = self.wap_task(fmt, task, issi)
            else:
                return False
            content_type = "text/vnd.wap.wml; charset=UTF-8" if fmt == "w" else "application/vnd.wap.xhtml+xml; charset=UTF-8"
            self.send_bytes(data, content_type)
            return True
        except KeyError as error:
            data = wml_doc("Fehler", x(error)) if fmt == "w" else xhtml_doc("Fehler", x(error))
            self.send_bytes(data, "text/vnd.wap.wml; charset=UTF-8" if fmt == "w" else "application/vnd.wap.xhtml+xml; charset=UTF-8", 404)
            return True
        except ValueError as error:
            data = wml_doc("Fehler", x(error)) if fmt == "w" else xhtml_doc("Fehler", x(error))
            self.send_bytes(data, "text/vnd.wap.wml; charset=UTF-8" if fmt == "w" else "application/vnd.wap.xhtml+xml; charset=UTF-8", 400)
            return True

    def do_GET(self) -> None:
        if self.handle_wap("GET"):
            return
        parsed = urlparse(self.path)
        path = parsed.path
        query = parse_qs(parsed.query)
        if path == "/":
            return self.send_bytes(HTML.encode("utf-8"), "text/html; charset=utf-8")
        if path == "/health/live":
            return self.send_json({"status": "ok", "service": "netcore-task-workflow", "mode": "open_lab"})
        if path == "/health/ready":
            status = self.app.status()
            ready = (not self.app.config.mqtt.get("enabled", True) or status["mqtt_connected"]) and (not self.app.config.sds.get("enabled", True) or status["sds_router_healthy"])
            return self.send_json({"status": "ready" if ready else "degraded", "dependencies": {"mqtt": status["mqtt_connected"], "sds_router": status["sds_router_healthy"]}}, 200 if ready else 503)
        if path == "/metrics":
            return self.send_bytes(self.app.prometheus().encode("utf-8"), "text/plain; version=0.0.4; charset=utf-8")
        if path == "/openapi.json":
            return self.send_json({
                "openapi": "3.0.3",
                "info": {"title": "NetCore Task Workflow OPEN LAB", "version": "1.0.0"},
                "paths": {
                    "/api/v1/status": {"get": {}},
                    "/api/v1/templates": {"get": {}},
                    "/api/v1/tasks": {"get": {}, "post": {}},
                    "/api/v1/tasks/{id}": {"get": {}},
                    "/api/v1/tasks/{id}/{action}": {"post": {}},
                    "/api/v1/ingest-sds": {"post": {}},
                    "/x": {"get": {}},
                    "/w": {"get": {}},
                    "/health/live": {"get": {}},
                    "/health/ready": {"get": {}},
                    "/metrics": {"get": {}},
                },
            })
        if path == "/api/v1/status":
            return self.send_json(self.app.status())
        if path == "/api/v1/templates":
            return self.send_json(self.app.config.templates)
        if path == "/api/v1/events":
            limit = to_int(query.get("limit", [200])[-1], 200, 1, 2000)
            return self.send_json(list(self.app.events)[-limit:])
        if path == "/api/v1/tasks":
            state = query.get("state", [None])[-1]
            active_raw = query.get("active", [None])[-1]
            active = None if active_raw is None else to_bool(active_raw)
            issi = to_int(query.get("issi", [0])[-1], 0) or None
            limit = to_int(query.get("limit", [200])[-1], 200, 1, 1000)
            return self.send_json(self.app.list_tasks(state, issi, active, limit))
        match = re.fullmatch(r"/api/v1/tasks/([^/]+)", path)
        if match:
            task = self.app.get_task(match.group(1))
            return self.send_json(task, 200) if task else self.send_json({"error": "task not found"}, 404)
        return self.send_json({"error": "not found"}, 404)

    def do_POST(self) -> None:
        if self.handle_wap("POST"):
            return
        parsed = urlparse(self.path)
        path = parsed.path
        try:
            payload, _ = self.read_body()
            if path == "/api/v1/tasks":
                return self.send_json(self.app.create_task(payload, compact_text(payload.get("actor") or "openlab-api", 120)), 201)
            if path == "/api/v1/ingest-sds":
                result = self.app.ingest_sds_event(payload)
                return self.send_json(result or {"status": "no_matching_action"}, 200 if result else 202)
            match = re.fullmatch(r"/api/v1/tasks/([^/]+)/(assign|accept|start|block|complete|cancel|reopen|comment|notify)", path)
            if match:
                task_ref, action = match.groups()
                actor = compact_text(payload.get("actor") or "openlab-api", 120)
                if action == "notify":
                    return self.send_json(self.app.notify_task(task_ref, actor, "manual"))
                return self.send_json(self.app.transition(task_ref, action, actor, payload.get("note", ""), payload))
            return self.send_json({"error": "not found"}, 404)
        except KeyError as error:
            return self.send_json({"error": compact_text(error, 300)}, 404)
        except (ValueError, json.JSONDecodeError) as error:
            return self.send_json({"error": compact_text(error, 500)}, 400)
        except (HTTPError, URLError, OSError) as error:
            return self.send_json({"error": compact_text(error, 500)}, 502)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", default="/etc/netcore/task-workflow.toml")
    args = parser.parse_args()
    with open(args.config, "rb") as handle:
        raw = tomllib.load(handle)
    if raw.get("security", {}).get("mode") != "open_lab":
        raise SystemExit("Phase 9 supports open_lab only")
    app = TaskWorkflow(Config(raw))
    Handler.app = app
    threads = [
        threading.Thread(target=app.expire_loop, daemon=True),
        threading.Thread(target=app.dependency_loop, daemon=True),
        threading.Thread(target=app.mqtt_loop, daemon=True),
    ]
    for thread in threads:
        thread.start()
    host, port = app.config.bind
    server = ThreadingHTTPServer((host, port), Handler)
    def stop(*_: Any) -> None:
        app.stop_event.set()
        threading.Thread(target=server.shutdown, daemon=True).start()
    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    server.serve_forever()


if __name__ == "__main__":
    main()
