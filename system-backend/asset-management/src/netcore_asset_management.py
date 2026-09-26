#!/usr/bin/env python3
from __future__ import annotations

import argparse
import csv
import html
import io
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
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any
from urllib.error import HTTPError, URLError
from urllib.parse import parse_qs, quote, urlparse
from urllib.request import Request, urlopen

STATE_SCHEMA = "netcore-asset-management-state-v1"
EVENT_SCHEMA = "netcore-event-v1"
ASSET_SCHEMA = "netcore-asset-v1"
PERSON_SCHEMA = "netcore-person-v1"
ASSIGNMENT_SCHEMA = "netcore-assignment-v1"
ACTIVE_ASSIGNMENT = "active"
ASSET_KINDS = {
    "tetra_radio", "tbs", "server", "rack", "rf_component", "power", "gateway",
    "vehicle", "accessory", "tool", "generic",
}
ASSET_STATUSES = {"in_stock", "assigned", "maintenance", "repair", "retired", "lost"}
MAINTENANCE_STATUSES = {"planned", "in_progress", "completed", "cancelled"}


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="milliseconds").replace("+00:00", "Z")


def compact(value: Any, maximum: int = 500) -> str:
    return re.sub(r"\s+", " ", str(value or "")).strip()[:maximum]


def safe_id(value: Any, maximum: int = 160) -> str:
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


def parse_time(value: Any) -> datetime | None:
    if not isinstance(value, str) or not value:
        return None
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00")).astimezone(timezone.utc)
    except ValueError:
        return None


class Config:
    def __init__(self, raw: dict[str, Any]):
        self.raw = raw

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
    def upstreams(self) -> dict[str, Any]:
        return self.raw.get("upstreams", {})

    @property
    def management(self) -> dict[str, Any]:
        return self.raw.get("management", {})


class AssetManagement:
    def __init__(self, config: Config):
        self.config = config
        self.lock = threading.RLock()
        self.stop_event = threading.Event()
        self.instance = os.uname().nodename
        self.started_at = now_iso()
        storage = config.storage
        self.state_path = Path(str(storage["state_file"]))
        self.event_path = Path(str(storage["event_log"]))
        self.audit_path = Path(str(storage["audit_log"]))
        self.state_path.parent.mkdir(parents=True, exist_ok=True)
        self.assets: dict[str, dict[str, Any]] = {}
        self.persons: dict[str, dict[str, Any]] = {}
        self.assignments: dict[str, dict[str, Any]] = {}
        self.maintenance: dict[str, dict[str, Any]] = {}
        self.events: deque[dict[str, Any]] = deque(maxlen=int(config.management.get("event_history_limit", 3000)))
        self.external = {
            "subscriber_profiles": [],
            "observed_subscribers": [],
            "mobility_subscribers": [],
            "last_sync_at": None,
        }
        self.upstream_health: dict[str, dict[str, Any]] = {}
        self.mqtt_connected = False
        self.mqtt_last_error: str | None = None
        self.metrics = {
            "assets_created": 0, "assets_updated": 0, "persons_created": 0,
            "assignments_created": 0, "assignments_returned": 0,
            "maintenance_created": 0, "maintenance_completed": 0,
            "upstream_syncs": 0, "upstream_sync_failures": 0,
        }
        self._load()

    def _load(self) -> None:
        try:
            data = json.loads(self.state_path.read_text(encoding="utf-8"))
            if data.get("schema") == STATE_SCHEMA:
                for name in ("assets", "persons", "assignments", "maintenance"):
                    value = data.get(name)
                    if isinstance(value, dict):
                        setattr(self, name, value)
                if isinstance(data.get("external"), dict):
                    self.external.update(data["external"])
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
                "assets": self.assets,
                "persons": self.persons,
                "assignments": self.assignments,
                "maintenance": self.maintenance,
                "external": self.external,
                "metrics": self.metrics,
            })

    def audit(self, action: str, actor: str, subject_type: str, subject_id: str, detail: dict[str, Any]) -> None:
        record = {
            "audit_id": str(uuid.uuid4()), "timestamp": now_iso(),
            "service": "netcore-asset-management", "actor": actor,
            "action": action, "subject_type": subject_type, "subject_id": subject_id,
            "detail": detail,
        }
        with self.audit_path.open("a", encoding="utf-8") as handle:
            handle.write(json.dumps(record, ensure_ascii=False, separators=(",", ":")) + "\n")

    def publish_mqtt(self, topic: str, payload: dict[str, Any], retain: bool = False, qos: int = 1) -> bool:
        cfg = self.config.mqtt
        if not cfg.get("enabled", True):
            self.mqtt_connected = True
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
            if result.returncode == 0:
                self.mqtt_connected = True
                self.mqtt_last_error = None
                return True
            self.mqtt_connected = False
            self.mqtt_last_error = compact(result.stderr, 300)
        except (OSError, subprocess.TimeoutExpired) as error:
            self.mqtt_connected = False
            self.mqtt_last_error = compact(error, 300)
        return False

    def emit_event(self, event_type: str, severity: str, subject_type: str, subject_id: str, payload: dict[str, Any]) -> dict[str, Any]:
        event = {
            "schema": EVENT_SCHEMA,
            "event_id": str(uuid.uuid4()),
            "event_type": event_type,
            "source": {"service": "netcore-asset-management", "instance": self.instance},
            "timestamp": now_iso(),
            "severity": severity,
            "subject": {"type": subject_type, "id": subject_id},
            "payload": payload,
            "deduplication_key": f"netcore-asset-management:{event_type}:{subject_id}:{uuid.uuid4().hex[:10]}",
            "labels": {"phase": "10", "mode": "open_lab"},
        }
        with self.lock:
            self.events.append(event)
            with self.event_path.open("a", encoding="utf-8") as handle:
                handle.write(json.dumps(event, ensure_ascii=False, separators=(",", ":")) + "\n")
        prefix = str(self.config.mqtt.get("topic_prefix", "netcore/v1")).rstrip("/")
        self.publish_mqtt(f"{prefix}/events/{event_type.replace('.', '/')}", event, retain=False)
        return event

    def state_topic(self, kind: str, object_id: str) -> str:
        prefix = str(self.config.mqtt.get("topic_prefix", "netcore/v1")).rstrip("/")
        return f"{prefix}/state/{kind}/{safe_id(object_id)}"

    def publish_state(self, kind: str, object_id: str, value: dict[str, Any]) -> None:
        self.publish_mqtt(self.state_topic(kind, object_id), value, retain=True)

    def normalize_asset(self, payload: dict[str, Any], existing: dict[str, Any] | None = None) -> dict[str, Any]:
        current = deepcopy(existing or {})
        asset_id = safe_id(payload.get("asset_id") or current.get("asset_id") or payload.get("inventory_id") or uuid.uuid4())
        if not asset_id:
            raise ValueError("asset_id or inventory_id required")
        kind = str(payload.get("kind", current.get("kind", "generic"))).strip().lower()
        if kind not in ASSET_KINDS:
            raise ValueError(f"unsupported asset kind: {kind}")
        status = str(payload.get("status", current.get("status", "in_stock"))).strip().lower()
        if status not in ASSET_STATUSES:
            raise ValueError(f"unsupported asset status: {status}")
        now = now_iso()
        asset = {
            "schema": ASSET_SCHEMA,
            "asset_id": asset_id,
            "inventory_id": compact(payload.get("inventory_id", current.get("inventory_id", asset_id)), 80),
            "kind": kind,
            "status": status,
            "manufacturer": compact(payload.get("manufacturer", current.get("manufacturer")), 100),
            "model": compact(payload.get("model", current.get("model")), 120),
            "serial_number": compact(payload.get("serial_number", current.get("serial_number")), 120),
            "firmware_version": compact(payload.get("firmware_version", current.get("firmware_version")), 80),
            "codeplug_version": compact(payload.get("codeplug_version", current.get("codeplug_version")), 80),
            "device_tei": payload.get("device_tei", current.get("device_tei")),
            "issi": payload.get("issi", current.get("issi")),
            "organization": compact(payload.get("organization", current.get("organization")), 120),
            "location": compact(payload.get("location", current.get("location")), 160),
            "tags": sorted({compact(x, 40) for x in payload.get("tags", current.get("tags", [])) if compact(x, 40)}),
            "notes": compact(payload.get("notes", current.get("notes")), 2000),
            "current_assignment_id": current.get("current_assignment_id"),
            "network_snapshot": current.get("network_snapshot", {}),
            "created_at": current.get("created_at", now),
            "updated_at": now,
        }
        if asset["issi"] not in (None, ""):
            asset["issi"] = int_value(asset["issi"], 0)
            if asset["issi"] <= 0:
                raise ValueError("issi must be a positive integer")
        if asset["device_tei"] not in (None, ""):
            asset["device_tei"] = int_value(asset["device_tei"], 0)
        return asset

    def normalize_person(self, payload: dict[str, Any], existing: dict[str, Any] | None = None) -> dict[str, Any]:
        current = deepcopy(existing or {})
        person_id = safe_id(payload.get("person_id") or current.get("person_id") or payload.get("username") or uuid.uuid4())
        if not person_id:
            raise ValueError("person_id or username required")
        now = now_iso()
        rui_issi = payload.get("rui_issi", current.get("rui_issi"))
        if rui_issi not in (None, ""):
            rui_issi = int_value(rui_issi, 0)
            if rui_issi <= 0:
                raise ValueError("rui_issi must be a positive integer")
        return {
            "schema": PERSON_SCHEMA,
            "person_id": person_id,
            "username": compact(payload.get("username", current.get("username", person_id)), 80),
            "display_name": compact(payload.get("display_name", current.get("display_name")), 160),
            "organization": compact(payload.get("organization", current.get("organization")), 120),
            "role": compact(payload.get("role", current.get("role")), 100),
            "email": compact(payload.get("email", current.get("email")), 160),
            "phone": compact(payload.get("phone", current.get("phone")), 80),
            "active": bool_value(payload.get("active", current.get("active", True)), True),
            "rui_username": compact(payload.get("rui_username", current.get("rui_username")), 80),
            "rui_issi": rui_issi,
            "pin_stored": False,
            "notes": compact(payload.get("notes", current.get("notes")), 2000),
            "created_at": current.get("created_at", now),
            "updated_at": now,
        }

    def create_asset(self, payload: dict[str, Any], actor: str = "openlab") -> dict[str, Any]:
        with self.lock:
            asset = self.normalize_asset(payload)
            if asset["asset_id"] in self.assets:
                raise ValueError("asset_id already exists")
            if asset.get("serial_number") and any(x.get("serial_number") == asset["serial_number"] for x in self.assets.values()):
                raise ValueError("serial_number already exists")
            self.assets[asset["asset_id"]] = asset
            self.metrics["assets_created"] += 1
            self.persist()
        self.audit("asset.create", actor, "asset", asset["asset_id"], asset)
        self.emit_event("asset.created", "info", "asset", asset["asset_id"], {"asset": asset})
        self.publish_state("assets", asset["asset_id"], asset)
        return deepcopy(asset)

    def update_asset(self, asset_id: str, payload: dict[str, Any], actor: str = "openlab") -> dict[str, Any]:
        with self.lock:
            existing = self.assets.get(asset_id)
            if not existing:
                raise KeyError(asset_id)
            merged = dict(payload)
            merged["asset_id"] = asset_id
            asset = self.normalize_asset(merged, existing)
            self.assets[asset_id] = asset
            self.metrics["assets_updated"] += 1
            self.persist()
        self.audit("asset.update", actor, "asset", asset_id, {"before": existing, "after": asset})
        event = "asset.retired" if asset["status"] == "retired" and existing.get("status") != "retired" else "asset.updated"
        self.emit_event(event, "notice" if event == "asset.retired" else "info", "asset", asset_id, {"asset": asset})
        self.publish_state("assets", asset_id, asset)
        return deepcopy(asset)

    def delete_asset(self, asset_id: str, actor: str = "openlab") -> None:
        with self.lock:
            asset = self.assets.get(asset_id)
            if not asset:
                raise KeyError(asset_id)
            if asset.get("current_assignment_id"):
                raise ValueError("assigned asset cannot be deleted")
            del self.assets[asset_id]
            self.persist()
        self.audit("asset.delete", actor, "asset", asset_id, asset)
        self.emit_event("asset.deleted", "warning", "asset", asset_id, {"asset": asset})
        self.publish_mqtt(self.state_topic("assets", asset_id), {}, retain=True)

    def create_person(self, payload: dict[str, Any], actor: str = "openlab") -> dict[str, Any]:
        with self.lock:
            person = self.normalize_person(payload)
            if person["person_id"] in self.persons:
                raise ValueError("person_id already exists")
            self.persons[person["person_id"]] = person
            self.metrics["persons_created"] += 1
            self.persist()
        self.audit("person.create", actor, "person", person["person_id"], person)
        self.emit_event("person.created", "info", "person", person["person_id"], {"person": person})
        self.publish_state("persons", person["person_id"], person)
        return deepcopy(person)

    def update_person(self, person_id: str, payload: dict[str, Any], actor: str = "openlab") -> dict[str, Any]:
        with self.lock:
            existing = self.persons.get(person_id)
            if not existing:
                raise KeyError(person_id)
            merged = dict(payload)
            merged["person_id"] = person_id
            person = self.normalize_person(merged, existing)
            self.persons[person_id] = person
            self.persist()
        self.audit("person.update", actor, "person", person_id, {"before": existing, "after": person})
        event = "person.deactivated" if not person["active"] and existing.get("active", True) else "person.updated"
        self.emit_event(event, "notice" if event == "person.deactivated" else "info", "person", person_id, {"person": person})
        self.publish_state("persons", person_id, person)
        return deepcopy(person)

    def delete_person(self, person_id: str, actor: str = "openlab") -> None:
        with self.lock:
            person = self.persons.get(person_id)
            if not person:
                raise KeyError(person_id)
            if any(x.get("person_id") == person_id and x.get("status") == ACTIVE_ASSIGNMENT for x in self.assignments.values()):
                raise ValueError("person has active assignments")
            del self.persons[person_id]
            self.persist()
        self.audit("person.delete", actor, "person", person_id, person)
        self.emit_event("person.deleted", "warning", "person", person_id, {"person": person})
        self.publish_mqtt(self.state_topic("persons", person_id), {}, retain=True)

    def assign(self, payload: dict[str, Any], actor: str = "openlab") -> dict[str, Any]:
        asset_id = safe_id(payload.get("asset_id"))
        person_id = safe_id(payload.get("person_id"))
        with self.lock:
            asset = self.assets.get(asset_id)
            person = self.persons.get(person_id)
            if not asset:
                raise ValueError("unknown asset_id")
            if not person:
                raise ValueError("unknown person_id")
            if not person.get("active", True):
                raise ValueError("person is inactive")
            if asset.get("current_assignment_id"):
                raise ValueError("asset already assigned")
            if asset.get("status") in {"maintenance", "repair", "retired", "lost"}:
                raise ValueError(f"asset status {asset.get('status')} is not assignable")
            assignment_id = str(uuid.uuid4())
            assignment = {
                "schema": ASSIGNMENT_SCHEMA,
                "assignment_id": assignment_id,
                "asset_id": asset_id,
                "person_id": person_id,
                "status": ACTIVE_ASSIGNMENT,
                "issued_at": now_iso(),
                "expected_return_at": payload.get("expected_return_at"),
                "returned_at": None,
                "issued_by": actor,
                "returned_by": None,
                "issue_note": compact(payload.get("issue_note"), 1000),
                "return_note": "",
                "rui_context": {
                    "rui_username": person.get("rui_username") or None,
                    "rui_issi": person.get("rui_issi"),
                    "radio_issi": asset.get("issi"),
                    "pin_stored": False,
                    "network_login_executed": False,
                },
            }
            self.assignments[assignment_id] = assignment
            asset["current_assignment_id"] = assignment_id
            asset["status"] = "assigned"
            asset["updated_at"] = now_iso()
            self.metrics["assignments_created"] += 1
            self.persist()
        self.audit("assignment.create", actor, "assignment", assignment_id, assignment)
        self.emit_event("assignment.created", "notice", "assignment", assignment_id, {"assignment": assignment, "asset": asset, "person": person})
        self.emit_event("asset.assigned", "notice", "asset", asset_id, {"assignment_id": assignment_id, "person_id": person_id})
        self.publish_state("assignments", assignment_id, assignment)
        self.publish_state("assets", asset_id, asset)
        return deepcopy(assignment)

    def return_asset(self, assignment_id: str, payload: dict[str, Any], actor: str = "openlab") -> dict[str, Any]:
        with self.lock:
            assignment = self.assignments.get(assignment_id)
            if not assignment:
                raise KeyError(assignment_id)
            if assignment.get("status") != ACTIVE_ASSIGNMENT:
                raise ValueError("assignment is not active")
            assignment["status"] = "returned"
            assignment["returned_at"] = now_iso()
            assignment["returned_by"] = actor
            assignment["return_note"] = compact(payload.get("return_note"), 1000)
            asset = self.assets.get(assignment["asset_id"])
            if asset:
                asset["current_assignment_id"] = None
                requested_status = str(payload.get("asset_status", "in_stock"))
                asset["status"] = requested_status if requested_status in ASSET_STATUSES else "in_stock"
                asset["location"] = compact(payload.get("location", asset.get("location")), 160)
                asset["updated_at"] = now_iso()
            self.metrics["assignments_returned"] += 1
            self.persist()
        self.audit("assignment.return", actor, "assignment", assignment_id, assignment)
        self.emit_event("assignment.returned", "notice", "assignment", assignment_id, {"assignment": assignment})
        if asset:
            self.emit_event("asset.returned", "notice", "asset", asset["asset_id"], {"assignment_id": assignment_id, "status": asset["status"]})
            self.publish_state("assets", asset["asset_id"], asset)
        self.publish_state("assignments", assignment_id, assignment)
        return deepcopy(assignment)

    def create_maintenance(self, payload: dict[str, Any], actor: str = "openlab") -> dict[str, Any]:
        asset_id = safe_id(payload.get("asset_id"))
        with self.lock:
            asset = self.assets.get(asset_id)
            if not asset:
                raise ValueError("unknown asset_id")
            record_id = str(uuid.uuid4())
            status = str(payload.get("status", "planned"))
            if status not in MAINTENANCE_STATUSES:
                raise ValueError("invalid maintenance status")
            record = {
                "record_id": record_id, "asset_id": asset_id,
                "kind": compact(payload.get("kind", "inspection"), 80),
                "status": status, "title": compact(payload.get("title", "Wartung"), 200),
                "due_at": payload.get("due_at"), "started_at": payload.get("started_at"),
                "completed_at": payload.get("completed_at"),
                "provider": compact(payload.get("provider"), 160),
                "notes": compact(payload.get("notes"), 2000),
                "result": compact(payload.get("result"), 2000),
                "task_id": payload.get("task_id"),
                "created_at": now_iso(), "updated_at": now_iso(),
            }
            self.maintenance[record_id] = record
            if status in {"planned", "in_progress"} and bool_value(payload.get("take_asset_out_of_service", False)):
                asset["status"] = "maintenance" if status == "planned" else "repair"
                asset["updated_at"] = now_iso()
            self.metrics["maintenance_created"] += 1
            self.persist()
        self.audit("maintenance.create", actor, "maintenance", record_id, record)
        event = "maintenance.started" if status == "in_progress" else "maintenance.created"
        self.emit_event(event, "notice", "maintenance", record_id, {"record": record, "asset_id": asset_id})
        self.publish_state("maintenance", record_id, record)
        self.publish_state("assets", asset_id, asset)
        return deepcopy(record)

    def update_maintenance(self, record_id: str, payload: dict[str, Any], actor: str = "openlab") -> dict[str, Any]:
        with self.lock:
            record = self.maintenance.get(record_id)
            if not record:
                raise KeyError(record_id)
            old_status = record.get("status")
            status = str(payload.get("status", old_status))
            if status not in MAINTENANCE_STATUSES:
                raise ValueError("invalid maintenance status")
            for key, limit in (("kind", 80), ("title", 200), ("provider", 160), ("notes", 2000), ("result", 2000)):
                if key in payload:
                    record[key] = compact(payload.get(key), limit)
            for key in ("due_at", "started_at", "completed_at", "task_id"):
                if key in payload:
                    record[key] = payload.get(key)
            record["status"] = status
            if status == "in_progress" and not record.get("started_at"):
                record["started_at"] = now_iso()
            if status == "completed" and not record.get("completed_at"):
                record["completed_at"] = now_iso()
            record["updated_at"] = now_iso()
            asset = self.assets.get(record["asset_id"])
            if asset and status == "completed" and bool_value(payload.get("return_to_stock", True), True):
                if not asset.get("current_assignment_id"):
                    asset["status"] = "in_stock"
                    asset["updated_at"] = now_iso()
            if status == "completed" and old_status != "completed":
                self.metrics["maintenance_completed"] += 1
            self.persist()
        self.audit("maintenance.update", actor, "maintenance", record_id, {"status": status, "payload": payload})
        event = {"in_progress": "maintenance.started", "completed": "maintenance.completed", "cancelled": "maintenance.cancelled"}.get(status, "maintenance.updated")
        self.emit_event(event, "notice", "maintenance", record_id, {"record": record})
        self.publish_state("maintenance", record_id, record)
        if asset:
            self.publish_state("assets", asset["asset_id"], asset)
        return deepcopy(record)

    def fetch_json(self, name: str, base_url: str, path: str) -> Any:
        started = time.monotonic()
        try:
            with urlopen(base_url.rstrip("/") + path, timeout=3) as response:
                data = json.loads(response.read())
            self.upstream_health[name] = {"healthy": True, "last_error": None, "latency_ms": int((time.monotonic() - started) * 1000), "checked_at": now_iso()}
            return data
        except (OSError, ValueError, HTTPError, URLError) as error:
            self.upstream_health[name] = {"healthy": False, "last_error": compact(error, 300), "latency_ms": None, "checked_at": now_iso()}
            raise

    def reconcile(self, actor: str = "system") -> dict[str, Any]:
        cfg = self.config.upstreams
        fetched: dict[str, Any] = {}
        failures: dict[str, str] = {}
        sources = [
            ("subscriber_profiles", "subscriber_core", "/api/v1/subscribers"),
            ("observed_subscribers", "subscriber_core", "/api/v1/observed"),
            ("mobility_subscribers", "mobility_core", "/api/v1/subscribers"),
        ]
        for target, section, path in sources:
            section_cfg = cfg.get(section, {}) if isinstance(cfg.get(section), dict) else {}
            if not section_cfg.get("enabled", True):
                continue
            base_url = str(section_cfg.get("base_url", ""))
            try:
                fetched[target] = self.fetch_json(section, base_url, path)
            except Exception as error:
                failures[target] = compact(error, 300)
        with self.lock:
            for key, value in fetched.items():
                self.external[key] = value if isinstance(value, list) else []
            self.external["last_sync_at"] = now_iso()
            profiles = {int_value(item.get("issi")): item for item in self.external.get("subscriber_profiles", []) if isinstance(item, dict)}
            observed = {int_value(item.get("issi")): item for item in self.external.get("observed_subscribers", []) if isinstance(item, dict)}
            mobility = {int_value(item.get("issi")): item for item in self.external.get("mobility_subscribers", []) if isinstance(item, dict)}
            linked = 0
            for asset in self.assets.values():
                issi = int_value(asset.get("issi"), 0)
                snapshot = {
                    "subscriber_profile": profiles.get(issi),
                    "observed": observed.get(issi),
                    "mobility": mobility.get(issi),
                    "refreshed_at": now_iso(),
                } if issi else {}
                asset["network_snapshot"] = snapshot
                if issi and any(snapshot.values()):
                    linked += 1
            self.metrics["upstream_syncs"] += 1
            if failures:
                self.metrics["upstream_sync_failures"] += 1
            self.persist()
        self.audit("upstream.reconcile", actor, "service", "asset-management", {"fetched": list(fetched), "failures": failures, "linked_assets": linked})
        self.emit_event("asset.reconciled", "warning" if failures else "info", "service", "asset-management", {"linked_assets": linked, "failures": failures})
        for asset in self.assets.values():
            self.publish_state("assets", asset["asset_id"], asset)
        return {"ok": not failures, "linked_assets": linked, "fetched": {key: len(value) for key, value in fetched.items() if isinstance(value, list)}, "failures": failures}

    def create_task_for_maintenance(self, asset_id: str, payload: dict[str, Any], actor: str = "openlab") -> dict[str, Any]:
        cfg = self.config.upstreams.get("task_workflow", {})
        if not cfg.get("enabled", True):
            raise ValueError("task_workflow disabled")
        asset = self.assets.get(asset_id)
        if not asset:
            raise KeyError(asset_id)
        body = {
            "template_id": "maintenance_ack",
            "title": compact(payload.get("title") or f"Wartung {asset.get('inventory_id')}", 200),
            "description": compact(payload.get("description") or payload.get("work") or "Wartung durchführen", 1000),
            "priority": int_value(payload.get("priority"), 4),
            "assigned_gssi": int_value(payload.get("assigned_gssi"), int_value(cfg.get("default_gssi"), 15201)),
            "form_data": {"asset": asset.get("inventory_id"), "work": compact(payload.get("work"), 500), "result": ""},
            "notify": bool_value(payload.get("notify", True), True),
        }
        request = Request(str(cfg.get("base_url", "")).rstrip("/") + "/api/v1/tasks", data=json.dumps(body).encode(), method="POST", headers={"Content-Type": "application/json"})
        try:
            with urlopen(request, timeout=5) as response:
                task = json.loads(response.read())
        except (OSError, ValueError, HTTPError, URLError) as error:
            raise ValueError(f"task workflow request failed: {compact(error, 300)}") from error
        record = self.create_maintenance({
            "asset_id": asset_id, "kind": payload.get("kind", "maintenance"),
            "title": body["title"], "due_at": payload.get("due_at"),
            "notes": body["description"], "task_id": task.get("task_id"),
        }, actor)
        return {"task": task, "maintenance": record}

    def status(self) -> dict[str, Any]:
        active_assignments = sum(1 for x in self.assignments.values() if x.get("status") == ACTIVE_ASSIGNMENT)
        due = 0
        now = datetime.now(timezone.utc)
        for record in self.maintenance.values():
            due_at = parse_time(record.get("due_at"))
            if record.get("status") == "planned" and due_at and due_at <= now:
                due += 1
        return {
            "service": "netcore-asset-management", "phase": 10, "mode": "open_lab",
            "started_at": self.started_at, "assets_total": len(self.assets),
            "persons_total": len(self.persons), "assignments_total": len(self.assignments),
            "active_assignments": active_assignments, "maintenance_total": len(self.maintenance),
            "maintenance_due": due, "mqtt_connected": self.mqtt_connected,
            "mqtt_last_error": self.mqtt_last_error, "upstreams": self.upstream_health,
            "external_last_sync_at": self.external.get("last_sync_at"), "metrics": self.metrics,
            "warning": "OPEN LAB: no login, no token, no TLS. RUI/RUA metadata only; no PINs are stored and no network login is executed.",
        }

    def loop(self) -> None:
        interval = max(10, int_value(self.config.management.get("upstream_sync_interval_secs"), 60))
        last_sync = 0.0
        due_emitted: set[str] = set()
        while not self.stop_event.wait(2):
            if time.monotonic() - last_sync >= interval:
                try:
                    self.reconcile("scheduler")
                except Exception:
                    pass
                last_sync = time.monotonic()
            now = datetime.now(timezone.utc)
            with self.lock:
                records = list(self.maintenance.values())
            for record in records:
                due_at = parse_time(record.get("due_at"))
                if record.get("status") == "planned" and due_at and due_at <= now and record["record_id"] not in due_emitted:
                    due_emitted.add(record["record_id"])
                    self.emit_event("maintenance.due", "warning", "maintenance", record["record_id"], {"record": record})


def html_page(app: AssetManagement) -> str:
    return r'''<!doctype html><html lang="de"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>NetCore Asset Management</title><style>
:root{font-family:Inter,system-ui,sans-serif;color-scheme:dark;background:#0b1020;color:#e8edf7}body{margin:0;background:#0b1020}.lab{background:#8a4b00;padding:10px 16px;font-weight:700;text-align:center}header{display:flex;justify-content:space-between;align-items:center;padding:18px 24px;background:#111936;border-bottom:1px solid #28345f}h1{margin:0;font-size:1.4rem}.muted{color:#9ca9c7}.wrap{padding:18px;display:grid;gap:16px}.cards{display:grid;grid-template-columns:repeat(auto-fit,minmax(140px,1fr));gap:10px}.card,.panel{background:#121a33;border:1px solid #27345f;border-radius:12px;padding:14px}.value{font-size:1.55rem;font-weight:800}.toolbar{display:flex;gap:8px;flex-wrap:wrap;margin:10px 0}button,.btn,input,select,textarea{background:#1b2546;color:#eef3ff;border:1px solid #3a4b7d;border-radius:8px;padding:9px}button,.btn{cursor:pointer;text-decoration:none}.primary{background:#315fc9}.danger{background:#7d2936}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(240px,1fr));gap:12px}.tablewrap{overflow:auto;max-height:430px}table{width:100%;border-collapse:collapse;font-size:.9rem}th,td{padding:8px;border-bottom:1px solid #28345f;text-align:left;vertical-align:top}th{position:sticky;top:0;background:#18213e}.ok{color:#6ee7a0}.bad{color:#ff8c9b}.tag{display:inline-block;border:1px solid #4c5d8e;border-radius:999px;padding:2px 7px;margin:1px;font-size:.75rem}form{display:grid;gap:8px}label{display:grid;gap:4px;font-size:.85rem}dialog{background:#121a33;color:#eef3ff;border:1px solid #455b96;border-radius:12px;max-width:760px;width:90%}pre{white-space:pre-wrap;word-break:break-word}small{color:#9ca9c7}</style></head><body>
<div class="lab">⚠ OPEN LAB – keine Anmeldung, keine Tokens, kein TLS. RUI/RUA nur als Metadaten; PINs werden niemals gespeichert.</div><header><div><h1>NetCore Asset Management</h1><div class="muted">Assets, Geräte, Personen, Ausgaben und Wartung</div></div><div id="health">…</div></header><main class="wrap"><section class="cards" id="cards"></section>
<section class="panel"><div class="toolbar"><button class="primary" onclick="openAsset()">Asset anlegen</button><button onclick="openPerson()">Person anlegen</button><button onclick="openAssign()">Gerät ausgeben</button><button onclick="openMaint()">Wartung planen</button><button onclick="reconcile()">Netzbestand abgleichen</button><a class="btn" href="/api/v1/export.json">JSON Export</a><a class="btn" href="/api/v1/export/assets.csv">Assets CSV</a></div><input id="filter" placeholder="Asset, Seriennummer, ISSI, Person…" oninput="render()"></section>
<section class="panel"><h2>Assets</h2><div class="tablewrap"><table><thead><tr><th>Inventar</th><th>Typ / Modell</th><th>Netz</th><th>Status</th><th>Zuordnung</th><th>Aktionen</th></tr></thead><tbody id="assetRows"></tbody></table></div></section>
<section class="grid"><section class="panel"><h2>Personen</h2><div class="tablewrap"><table><thead><tr><th>Name</th><th>Rolle</th><th>RUI</th><th>Status</th><th></th></tr></thead><tbody id="personRows"></tbody></table></div></section><section class="panel"><h2>Aktive Ausgaben</h2><div class="tablewrap"><table><thead><tr><th>Asset</th><th>Person</th><th>seit</th><th></th></tr></thead><tbody id="assignmentRows"></tbody></table></div></section></section>
<section class="grid"><section class="panel"><h2>Wartung</h2><div class="tablewrap"><table><thead><tr><th>Asset</th><th>Arbeit</th><th>Status</th><th>Termin</th><th></th></tr></thead><tbody id="maintenanceRows"></tbody></table></div></section><section class="panel"><h2>Upstream-Abgleich</h2><pre id="upstreams"></pre></section></section>
<section class="panel"><h2>Letzte Ereignisse</h2><pre id="events"></pre></section></main>
<dialog id="assetDlg"><h2 id="assetTitle">Asset</h2><form id="assetForm"><input name="asset_id" placeholder="Asset-ID (z.B. hrt-001)" required><input name="inventory_id" placeholder="Inventarnummer"><select name="kind"><option value="tetra_radio">TETRA-Funkgerät</option><option value="tbs">Basisstation</option><option value="rack">Rack</option><option value="server">Server</option><option value="rf_component">HF-Komponente</option><option value="power">Energie/USV</option><option value="gateway">Gateway</option><option value="accessory">Zubehör</option><option value="generic">Sonstiges</option></select><div class="grid"><input name="manufacturer" placeholder="Hersteller"><input name="model" placeholder="Modell"><input name="serial_number" placeholder="Seriennummer"><input name="firmware_version" placeholder="Firmware"><input name="codeplug_version" placeholder="Codeplug-Version"><input name="issi" type="number" placeholder="ISSI"><input name="device_tei" type="number" placeholder="TEI"><input name="location" placeholder="Standort"></div><select name="status"><option>in_stock</option><option>assigned</option><option>maintenance</option><option>repair</option><option>retired</option><option>lost</option></select><textarea name="notes" placeholder="Notizen"></textarea><div class="toolbar"><button class="primary">Speichern</button><button type="button" onclick="assetDlg.close()">Abbrechen</button></div></form></dialog>
<dialog id="personDlg"><h2 id="personTitle">Person</h2><form id="personForm"><input name="person_id" placeholder="Person-ID" required><div class="grid"><input name="username" placeholder="Benutzername"><input name="display_name" placeholder="Anzeigename" required><input name="organization" placeholder="Organisation"><input name="role" placeholder="Rolle"><input name="email" placeholder="E-Mail"><input name="phone" placeholder="Telefon"><input name="rui_username" placeholder="RUI Benutzername"><input name="rui_issi" type="number" placeholder="RUI ISSI"></div><label><input name="active" type="checkbox" checked> aktiv</label><textarea name="notes" placeholder="Notizen"></textarea><small>Es wird absichtlich kein PIN gespeichert.</small><div class="toolbar"><button class="primary">Speichern</button><button type="button" onclick="personDlg.close()">Abbrechen</button></div></form></dialog>
<dialog id="assignDlg"><h2>Asset ausgeben</h2><form id="assignForm"><select name="asset_id" id="assignAsset"></select><select name="person_id" id="assignPerson"></select><input name="expected_return_at" type="datetime-local"><textarea name="issue_note" placeholder="Ausgabehinweis"></textarea><div class="toolbar"><button class="primary">Ausgeben</button><button type="button" onclick="assignDlg.close()">Abbrechen</button></div></form></dialog>
<dialog id="maintDlg"><h2>Wartung planen</h2><form id="maintForm"><select name="asset_id" id="maintAsset"></select><div class="grid"><input name="kind" value="inspection" placeholder="Art"><input name="title" value="Wartung" placeholder="Titel"><input name="due_at" type="datetime-local"><input name="provider" placeholder="Dienstleister"></div><textarea name="notes" placeholder="Arbeiten / Hinweise"></textarea><label><input name="take_asset_out_of_service" type="checkbox"> Asset außer Betrieb setzen</label><div class="toolbar"><button class="primary">Planen</button><button type="button" onclick="maintDlg.close()">Abbrechen</button></div></form></dialog>
<script>
let state={assets:[],persons:[],assignments:[],maintenance:[],events:[],status:{}};const $=id=>document.getElementById(id);async function api(path,opt){const r=await fetch(path,opt);if(!r.ok){let e={};try{e=await r.json()}catch{}throw new Error(e.error||r.statusText)}return r.status===204?null:r.json()}function esc(x){return String(x??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]))}function dt(x){return x?new Date(x).toLocaleString():'–'}function personName(id){const p=state.persons.find(x=>x.person_id===id);return p?.display_name||id||'–'}function assetName(id){const a=state.assets.find(x=>x.asset_id===id);return a?.inventory_id||id||'–'}
async function refresh(){try{const [status,assets,persons,assignments,maintenance,events]=await Promise.all([api('/api/v1/status'),api('/api/v1/assets'),api('/api/v1/persons'),api('/api/v1/assignments'),api('/api/v1/maintenance'),api('/api/v1/events?limit=20')]);state={status,assets,persons,assignments,maintenance,events};$('health').innerHTML='<span class="'+(status.mqtt_connected?'ok':'bad')+'">● '+(status.mqtt_connected?'ONLINE':'DEGRADED')+'</span>';render()}catch(e){$('health').innerHTML='<span class="bad">● OFFLINE</span>';console.error(e)}}
function render(){const q=$('filter').value.toLowerCase();$('cards').innerHTML=[['Assets',state.status.assets_total],['Personen',state.status.persons_total],['ausgegeben',state.status.active_assignments],['Wartungen',state.status.maintenance_total],['fällig',state.status.maintenance_due],['MQTT',state.status.mqtt_connected?'online':'offline']].map(x=>`<div class="card"><div class="muted">${x[0]}</div><div class="value">${x[1]}</div></div>`).join('');$('assetRows').innerHTML=state.assets.filter(a=>JSON.stringify(a).toLowerCase().includes(q)).map(a=>{const as=state.assignments.find(x=>x.assignment_id===a.current_assignment_id);const snap=a.network_snapshot||{};return `<tr><td><b>${esc(a.inventory_id)}</b><br><small>${esc(a.asset_id)} · ${esc(a.serial_number)}</small></td><td>${esc(a.kind)}<br>${esc(a.manufacturer)} ${esc(a.model)}</td><td>ISSI ${a.issi||'–'}<br><small>${snap.mobility?.serving_node||snap.observed?.node_id||'keine Route'}</small></td><td>${esc(a.status)}</td><td>${as?esc(personName(as.person_id)):'–'}</td><td><button onclick="editAsset('${esc(a.asset_id)}')">Edit</button>${as?` <button onclick="returnAsset('${as.assignment_id}')">Rückgabe</button>`:''}</td></tr>`}).join('');$('personRows').innerHTML=state.persons.filter(p=>JSON.stringify(p).toLowerCase().includes(q)).map(p=>`<tr><td><b>${esc(p.display_name)}</b><br><small>${esc(p.username)}</small></td><td>${esc(p.organization)}<br>${esc(p.role)}</td><td>${esc(p.rui_username||'–')} / ${p.rui_issi||'–'}</td><td>${p.active?'<span class="ok">aktiv</span>':'<span class="bad">inaktiv</span>'}</td><td><button onclick="editPerson('${esc(p.person_id)}')">Edit</button></td></tr>`).join('');$('assignmentRows').innerHTML=state.assignments.filter(x=>x.status==='active').map(x=>`<tr><td>${esc(assetName(x.asset_id))}</td><td>${esc(personName(x.person_id))}</td><td>${dt(x.issued_at)}</td><td><button onclick="returnAsset('${x.assignment_id}')">Rückgabe</button></td></tr>`).join('');$('maintenanceRows').innerHTML=state.maintenance.map(x=>`<tr><td>${esc(assetName(x.asset_id))}</td><td>${esc(x.title)}<br><small>${esc(x.kind)}</small></td><td>${esc(x.status)}</td><td>${dt(x.due_at)}</td><td>${x.status!=='completed'?`<button onclick="completeMaint('${x.record_id}')">Fertig</button>`:''}</td></tr>`).join('');$('events').textContent=state.events.map(e=>`${e.timestamp} ${e.event_type} ${e.subject?.id||''}`).join('\n');$('upstreams').textContent=JSON.stringify({last_sync_at:state.status.external_last_sync_at,upstreams:state.status.upstreams},null,2);$('assignAsset').innerHTML=state.assets.filter(a=>!a.current_assignment_id&&a.status==='in_stock').map(a=>`<option value="${esc(a.asset_id)}">${esc(a.inventory_id)} (${esc(a.model)})</option>`).join('');$('assignPerson').innerHTML=state.persons.filter(p=>p.active).map(p=>`<option value="${esc(p.person_id)}">${esc(p.display_name)}</option>`).join('');$('maintAsset').innerHTML=state.assets.map(a=>`<option value="${esc(a.asset_id)}">${esc(a.inventory_id)}</option>`).join('')}
function formObj(form){const f=new FormData(form),o={};for(const [k,v] of f.entries())o[k]=v;for(const el of form.querySelectorAll('input[type=checkbox]'))o[el.name]=el.checked;for(const k of ['issi','device_tei','rui_issi'])if(o[k]==='')o[k]=null;return o}function openAsset(){assetForm.reset();assetForm.dataset.id='';assetForm.elements.asset_id.disabled=false;assetDlg.showModal()}function editAsset(id){const a=state.assets.find(x=>x.asset_id===id);openAsset();assetForm.dataset.id=id;for(const [k,v] of Object.entries(a))if(assetForm.elements[k])assetForm.elements[k].value=v??'';assetForm.elements.asset_id.disabled=true}assetForm.onsubmit=async e=>{e.preventDefault();const id=e.target.dataset.id,p=formObj(e.target);await api(id?`/api/v1/assets/${id}`:'/api/v1/assets',{method:id?'PUT':'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(p)});assetDlg.close();refresh()};function openPerson(){personForm.reset();personForm.dataset.id='';personForm.elements.active.checked=true;personForm.elements.person_id.disabled=false;personDlg.showModal()}function editPerson(id){const p=state.persons.find(x=>x.person_id===id);openPerson();personForm.dataset.id=id;for(const [k,v] of Object.entries(p))if(personForm.elements[k])personForm.elements[k].type==='checkbox'?personForm.elements[k].checked=!!v:personForm.elements[k].value=v??'';personForm.elements.person_id.disabled=true}personForm.onsubmit=async e=>{e.preventDefault();const id=e.target.dataset.id,p=formObj(e.target);await api(id?`/api/v1/persons/${id}`:'/api/v1/persons',{method:id?'PUT':'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(p)});personDlg.close();refresh()};function openAssign(){render();assignDlg.showModal()}assignForm.onsubmit=async e=>{e.preventDefault();await api('/api/v1/assignments',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(formObj(e.target))});assignDlg.close();refresh()};async function returnAsset(id){const note=prompt('Rückgabehinweis','')||'';await api(`/api/v1/assignments/${id}/return`,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({return_note:note})});refresh()}function openMaint(){render();maintDlg.showModal()}maintForm.onsubmit=async e=>{e.preventDefault();await api('/api/v1/maintenance',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(formObj(e.target))});maintDlg.close();refresh()};async function completeMaint(id){const result=prompt('Ergebnis','erledigt')||'';await api(`/api/v1/maintenance/${id}`,{method:'PUT',headers:{'Content-Type':'application/json'},body:JSON.stringify({status:'completed',result})});refresh()}async function reconcile(){await api('/api/v1/reconcile',{method:'POST'});refresh()}refresh();setInterval(refresh,10000)
</script></body></html>'''


class Handler(BaseHTTPRequestHandler):
    server_version = "NetCoreAssetManagement/1.0"

    @property
    def app(self) -> AssetManagement:
        return self.server.app  # type: ignore[attr-defined]

    def log_message(self, fmt: str, *args: Any) -> None:
        print(f"{self.address_string()} - {fmt % args}")

    def json_body(self) -> dict[str, Any]:
        length = int(self.headers.get("Content-Length", "0") or 0)
        if length > 2_000_000:
            raise ValueError("request too large")
        raw = self.rfile.read(length) if length else b"{}"
        value = json.loads(raw or b"{}")
        if not isinstance(value, dict):
            raise ValueError("JSON object required")
        return value

    def send_json(self, status: int, value: Any) -> None:
        data = json.dumps(value, ensure_ascii=False, indent=2).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(data)

    def send_text(self, status: int, content_type: str, value: str) -> None:
        data = value.encode()
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(data)

    def error(self, status: int, message: str) -> None:
        self.send_json(status, {"error": message})

    def actor(self) -> str:
        return compact(self.headers.get("X-NetCore-Actor", "openlab"), 120) or "openlab"

    def do_GET(self) -> None:
        parsed = urlparse(self.path)
        path = parsed.path.rstrip("/") or "/"
        query = parse_qs(parsed.query)
        try:
            if path == "/":
                self.send_text(200, "text/html; charset=utf-8", html_page(self.app)); return
            if path in {"/health/live", "/health/ready"}:
                status = self.app.status(); code = 200 if path.endswith("live") or (status["mqtt_connected"] and not any(not x.get("healthy", False) for x in status["upstreams"].values())) else 503
                self.send_json(code, status); return
            if path == "/api/v1/status": self.send_json(200, self.app.status()); return
            if path == "/api/v1/assets":
                items = list(self.app.assets.values()); q = compact(query.get("q", [""])[0], 100).lower(); kind = query.get("kind", [""])[0]; status = query.get("status", [""])[0]
                if q: items = [x for x in items if q in json.dumps(x, ensure_ascii=False).lower()]
                if kind: items = [x for x in items if x.get("kind") == kind]
                if status: items = [x for x in items if x.get("status") == status]
                self.send_json(200, sorted(items, key=lambda x: str(x.get("inventory_id")))); return
            if path.startswith("/api/v1/assets/"):
                asset_id = path.split("/")[4]; item = self.app.assets.get(asset_id)
                if not item: self.error(404, "asset not found")
                else: self.send_json(200, item)
                return
            if path == "/api/v1/persons": self.send_json(200, sorted(self.app.persons.values(), key=lambda x: str(x.get("display_name")))); return
            if path.startswith("/api/v1/persons/"):
                person_id = path.split("/")[4]; item = self.app.persons.get(person_id)
                if not item: self.error(404, "person not found")
                else: self.send_json(200, item)
                return
            if path == "/api/v1/assignments": self.send_json(200, sorted(self.app.assignments.values(), key=lambda x: str(x.get("issued_at")), reverse=True)); return
            if path == "/api/v1/maintenance": self.send_json(200, sorted(self.app.maintenance.values(), key=lambda x: str(x.get("created_at")), reverse=True)); return
            if path == "/api/v1/events":
                limit = min(1000, max(1, int_value(query.get("limit", [100])[0], 100)))
                self.send_json(200, list(self.app.events)[-limit:]); return
            if path == "/api/v1/upstreams": self.send_json(200, {"health": self.app.upstream_health, "snapshot": self.app.external}); return
            if path == "/api/v1/export.json":
                self.send_json(200, {"schema": STATE_SCHEMA, "exported_at": now_iso(), "assets": self.app.assets, "persons": self.app.persons, "assignments": self.app.assignments, "maintenance": self.app.maintenance}); return
            if path == "/api/v1/export/assets.csv":
                output = io.StringIO(); writer = csv.writer(output); writer.writerow(["asset_id","inventory_id","kind","status","manufacturer","model","serial_number","firmware_version","codeplug_version","issi","device_tei","location"])
                for a in self.app.assets.values(): writer.writerow([a.get(k, "") for k in ["asset_id","inventory_id","kind","status","manufacturer","model","serial_number","firmware_version","codeplug_version","issi","device_tei","location"]])
                self.send_text(200, "text/csv; charset=utf-8", output.getvalue()); return
            if path == "/metrics":
                s=self.app.status(); lines=["# TYPE netcore_asset_management_assets gauge",f"netcore_asset_management_assets {s['assets_total']}","# TYPE netcore_asset_management_persons gauge",f"netcore_asset_management_persons {s['persons_total']}","# TYPE netcore_asset_management_active_assignments gauge",f"netcore_asset_management_active_assignments {s['active_assignments']}","# TYPE netcore_asset_management_maintenance_due gauge",f"netcore_asset_management_maintenance_due {s['maintenance_due']}","# TYPE netcore_asset_management_mqtt_connected gauge",f"netcore_asset_management_mqtt_connected {1 if s['mqtt_connected'] else 0}"]
                self.send_text(200,"text/plain; version=0.0.4; charset=utf-8","\n".join(lines)+"\n"); return
            if path == "/openapi.json":
                self.send_json(200, {"openapi":"3.0.3","info":{"title":"NetCore Asset Management","version":"1.0.0","description":"OPEN LAB asset, device, person, assignment and maintenance API"},"paths":{p:{} for p in ["/api/v1/status","/api/v1/assets","/api/v1/assets/{asset_id}","/api/v1/persons","/api/v1/persons/{person_id}","/api/v1/assignments","/api/v1/assignments/{assignment_id}/return","/api/v1/maintenance","/api/v1/maintenance/{record_id}","/api/v1/reconcile","/api/v1/export.json","/health/live","/health/ready","/metrics"]}}); return
            self.error(404, "not found")
        except Exception as error:
            self.error(500, compact(error, 500))

    def do_POST(self) -> None:
        path = urlparse(self.path).path.rstrip("/")
        try:
            payload = self.json_body(); actor = self.actor()
            if path == "/api/v1/assets": self.send_json(201, self.app.create_asset(payload, actor)); return
            if path == "/api/v1/persons": self.send_json(201, self.app.create_person(payload, actor)); return
            if path == "/api/v1/assignments": self.send_json(201, self.app.assign(payload, actor)); return
            if path.startswith("/api/v1/assignments/") and path.endswith("/return"):
                assignment_id = path.split("/")[4]; self.send_json(200, self.app.return_asset(assignment_id, payload, actor)); return
            if path == "/api/v1/maintenance": self.send_json(201, self.app.create_maintenance(payload, actor)); return
            if path.startswith("/api/v1/assets/") and path.endswith("/maintenance-task"):
                asset_id = path.split("/")[4]; self.send_json(201, self.app.create_task_for_maintenance(asset_id, payload, actor)); return
            if path == "/api/v1/reconcile": self.send_json(200, self.app.reconcile(actor)); return
            if path == "/api/v1/import":
                replace = bool_value(payload.get("replace", False)); data = payload.get("data", payload)
                if not isinstance(data, dict): raise ValueError("import data must be object")
                with self.app.lock:
                    for name in ("assets","persons","assignments","maintenance"):
                        incoming = data.get(name, {})
                        if isinstance(incoming, list): incoming = {str(x.get(name[:-1]+"_id") or x.get("asset_id") or x.get("person_id") or x.get("assignment_id") or x.get("record_id")):x for x in incoming if isinstance(x,dict)}
                        if isinstance(incoming, dict):
                            if replace: setattr(self.app,name,incoming)
                            else: getattr(self.app,name).update(incoming)
                    self.app.persist()
                self.app.audit("import", actor, "service", "asset-management", {"replace":replace})
                self.send_json(200,{"ok":True,"replace":replace}); return
            self.error(404, "not found")
        except KeyError as error: self.error(404, f"not found: {error.args[0]}")
        except (ValueError, TypeError, json.JSONDecodeError) as error: self.error(400, compact(error,500))
        except Exception as error: self.error(500, compact(error,500))

    def do_PUT(self) -> None:
        path = urlparse(self.path).path.rstrip("/")
        try:
            payload = self.json_body(); actor = self.actor()
            if path.startswith("/api/v1/assets/"):
                self.send_json(200, self.app.update_asset(path.split("/")[4], payload, actor)); return
            if path.startswith("/api/v1/persons/"):
                self.send_json(200, self.app.update_person(path.split("/")[4], payload, actor)); return
            if path.startswith("/api/v1/maintenance/"):
                self.send_json(200, self.app.update_maintenance(path.split("/")[4], payload, actor)); return
            self.error(404,"not found")
        except KeyError as error: self.error(404, f"not found: {error.args[0]}")
        except (ValueError, TypeError, json.JSONDecodeError) as error: self.error(400, compact(error,500))
        except Exception as error: self.error(500, compact(error,500))

    def do_DELETE(self) -> None:
        path = urlparse(self.path).path.rstrip("/")
        try:
            actor = self.actor()
            if path.startswith("/api/v1/assets/"):
                self.app.delete_asset(path.split("/")[4], actor); self.send_json(200,{"ok":True}); return
            if path.startswith("/api/v1/persons/"):
                self.app.delete_person(path.split("/")[4], actor); self.send_json(200,{"ok":True}); return
            self.error(404,"not found")
        except KeyError as error: self.error(404, f"not found: {error.args[0]}")
        except ValueError as error: self.error(409, compact(error,500))
        except Exception as error: self.error(500, compact(error,500))


class Server(ThreadingHTTPServer):
    daemon_threads = True
    allow_reuse_address = True


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", default="/etc/netcore/asset-management.toml")
    args = parser.parse_args()
    with open(args.config, "rb") as handle:
        config = Config(tomllib.load(handle))
    app = AssetManagement(config)
    server = Server(config.bind, Handler)
    server.app = app  # type: ignore[attr-defined]
    worker = threading.Thread(target=app.loop, daemon=True)
    worker.start()
    def stop(*_: Any) -> None:
        app.stop_event.set()
        threading.Thread(target=server.shutdown, daemon=True).start()
    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    print(f"netcore-asset-management OPEN LAB listening on {config.bind[0]}:{config.bind[1]}", flush=True)
    try:
        server.serve_forever(poll_interval=0.5)
    finally:
        app.stop_event.set(); app.persist(); server.server_close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
